use crate::{
    channel::{Channel, ChannelName},
    clocks::Clocks,
    commands::{
        ArpeggioCommand, Command, DataSkipCommand, DefaultNoteDurationCommand, DetuneCommand,
        NoteCommand, OctaveCommand, OctaveDownCommand, OctaveUpCommand, PitchEnvelopeCommand,
        PitchSweepCommand, QuantizeCommand, QuantizeFrameCommand, RepeatEndCommand,
        RepeatStartCommand, RestSignCommand, SlurCommand, TempoCommand, TieCommand, TimbreCommand,
        TimbresCommand, TrackLoopCommand, TupletEndCommand, TupletStartCommand, VibratoCommand,
        VolumeCommand, VolumeDownCommand, VolumeEnvelopeCommand, VolumeUpCommand, WaitCommand,
    },
    macros::Macros,
    oscillators::{Oscillator, PitchLfo},
    traits::NthFrameItem,
    types::{
        Detune, Note, NoteEnvelope, Octave, PitchEnvelope, PitchSweep, Sample, Timbres, Volume,
        VolumeEnvelope,
    },
    Music,
};
use std::{collections::BTreeMap, sync::Arc, time::Duration};
use textparse::{Position, Span};

#[derive(Debug)]
pub struct MusicPlayer {
    channels: BTreeMap<ChannelName, ChannelPlayer>,
    last_error: Option<PlayMusicError>,
}

impl MusicPlayer {
    // TODO: volume arg
    pub(crate) fn new(music: Music, sample_rate: u16) -> Self {
        let macros = music.macros();
        let channels = music
            .into_channels()
            .into_iter()
            .map(|(name, channel)| {
                (
                    name,
                    ChannelPlayer::new(channel, macros.clone(), sample_rate),
                )
            })
            .collect();
        Self {
            channels,
            last_error: None,
        }
    }

    fn take_last_error(&mut self) -> Option<PlayMusicError> {
        for (name, channel) in &mut self.channels {
            if let Some(mut e) = channel.last_error.take() {
                e.channel = *name;
                return Some(e);
            }
        }
        None
    }

    pub fn last_error(&self) -> Option<&PlayMusicError> {
        self.last_error.as_ref()
    }

    pub fn current_position(&self) -> Duration {
        for c in self.channels.values() {
            if !c.is_eos() {
                return c.clocks.sample_clock().now();
            }
        }
        Duration::from_secs(0)
    }

    pub fn is_eos(&self) -> bool {
        self.channels.values().all(|c| c.is_eos())
    }
}

impl Iterator for MusicPlayer {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        if self.last_error.is_some() {
            return None;
        }

        let n = self.channels.len() as f32;
        let mut sample = None;
        for x in self.channels.values_mut().flat_map(|c| c.next()) {
            sample = Some(sample.unwrap_or(Sample::ZERO) + x / n);
        }

        if let Some(e) = self.take_last_error() {
            self.last_error = Some(e);
            None
        } else {
            sample
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayMusicError {
    pub channel: ChannelName,
    pub span: std::ops::Range<Position>,
    pub reason: String,
}

impl PlayMusicError {
    fn new(span: impl Span, reason: &str) -> Self {
        Self {
            channel: ChannelName::A, // dummy initial value.
            span: span.start_position()..span.end_position(),
            reason: reason.to_string(),
        }
    }

    pub fn to_string(&self, text: &str, filename: Option<&str>) -> String {
        let offset = self.span.start_position().get();
        let (line, column) = self.span.start_position().line_and_column(text);
        let mut s = format!("{} on channel {:?}\n", self.reason, self.channel);
        s += &format!("  --> {}:{line}:{column}\n", filename.unwrap_or("<SCRIPT>"));

        let line_len = format!("{line}").len();
        s += &format!("{:line_len$} |\n", ' ');
        s += &format!(
            "{line} | {}\n",
            text[offset + 1 - column..]
                .lines()
                .next()
                .expect("unreachable")
        );
        s += &format!("{:line_len$} | {:>column$} {}\n", ' ', '^', self.reason);
        s
    }
}

#[derive(Debug)]
struct ChannelPlayer {
    oscillator: Oscillator,
    commands: Vec<Command>,
    command_index: usize,
    macros: Arc<Macros>,
    octave: Octave,
    detune: PitchEnvelope,
    volume: VolumeEnvelope,
    timbre: Option<Timbres>,
    clocks: Clocks,
    arpeggio: Option<NoteEnvelope>,
    loop_point: Option<usize>,
    repeat_stack: Vec<Repeat>,
    note: Option<Note>,
    pitch_lfo: Option<PitchLfo>,
    pitch_sweep: Option<PitchSweep>,
    last_error: Option<PlayMusicError>,
    eos: bool,
}

impl ChannelPlayer {
    fn new(channel: Channel, macros: Arc<Macros>, sample_rate: u16) -> Self {
        Self {
            oscillator: channel.oscillator,
            commands: channel.commands,
            command_index: 0,
            macros,
            octave: Octave::default(),
            detune: PitchEnvelope::constant(Detune::default()),
            volume: VolumeEnvelope::constant(Volume::default()),
            timbre: None,
            clocks: Clocks::new(sample_rate),
            loop_point: None,
            arpeggio: None,
            repeat_stack: Vec::new(),
            note: None,
            pitch_lfo: None,
            pitch_sweep: None,
            last_error: None,
            eos: false,
        }
    }

    fn is_eos(&self) -> bool {
        return self.eos;
    }

    fn sample(&mut self) -> Sample {
        self.clocks.tick_sample_clock();
        let sample = self
            .oscillator
            .sample(self.clocks.sample_rate(), self.pitch_lfo.as_mut());
        if !(self.clocks.sample_clock() < self.clocks.quantize_clock()) {
            self.oscillator.mute(true);
        }
        let volume = self.volume.nth_frame_item(self.clocks.frame_index());
        sample * volume.as_ratio()
    }

    fn handle_frame(&mut self) -> Result<(), PlayMusicError> {
        if let Some(timbre) = self
            .timbre
            .as_ref()
            .map(|t| t.nth_frame_item(self.clocks.frame_index()))
        {
            if !self.oscillator.set_timbre(timbre) {
                return Err(PlayMusicError::new(timbre, "unsupported timbre value"));
            }
        }
        self.update_frequency()?;
        Ok(())
    }

    fn update_frequency(&mut self) -> Result<(), PlayMusicError> {
        let Some(mut note) = self.note else {
            return Ok(());
        };

        let frame_index = self.clocks.frame_index();
        let detune = self.detune.nth_frame_item(frame_index);
        let mut octave = self.octave;

        if let Some(arpeggio) = &self.arpeggio {
            let result = note.apply_note_number_delta(arpeggio.nth_frame_item(frame_index));
            note = result.0;
            if result.1 < 0 {
                octave = octave
                    .checked_sub(result.1.abs() as u8)
                    .ok_or_else(|| PlayMusicError::new(note, "octave underflow"))?;
            } else {
                octave = octave
                    .checked_add(result.1 as u8)
                    .ok_or_else(|| PlayMusicError::new(note, "octave overflow"))?;
            };
        };

        if let Some((Some(speed), Some(depth))) = self.pitch_sweep.map(|s| (s.speed(), s.depth())) {
            if frame_index == 0 {
                self.oscillator.set_frequency(note, octave, detune);
            }
            if frame_index % usize::from(speed) == 0 {
                self.oscillator.sweep_frequency(depth);
            }
        } else {
            self.oscillator.set_frequency(note, octave, detune);
        }
        Ok(())
    }

    fn handle_note_command(&mut self, command: NoteCommand) -> Result<(), PlayMusicError> {
        self.note = Some(command.note());
        self.update_frequency()?;
        self.clocks.tick_note_clock(command.note_duration());
        self.clocks.reset_frame_clock(self.clocks.sample_clock());
        self.handle_frame()?;
        if let Some(lfo) = &mut self.pitch_lfo {
            lfo.reset_timer();
        }
        self.oscillator.mute(false);
        Ok(())
    }

    fn handle_arpeggio_command(&mut self, command: ArpeggioCommand) -> Result<(), PlayMusicError> {
        if let Some(n) = command.macro_number() {
            self.arpeggio = Some(
                self.macros
                    .arpeggios
                    .get(&n)
                    .ok_or_else(|| PlayMusicError::new(command, "undefined macro number"))?
                    .envelope()
                    .clone(),
            );
        } else {
            self.arpeggio = None;
        }
        Ok(())
    }

    fn handle_rest_sign_command(&mut self, command: RestSignCommand) -> Result<(), PlayMusicError> {
        self.clocks.tick_note_clock(command.note_duration());
        self.clocks.reset_frame_clock(self.clocks.sample_clock());
        self.note = None;
        self.handle_frame()?;
        self.oscillator.mute(true);
        Ok(())
    }

    fn handle_wait_command(&mut self, command: WaitCommand) -> Result<(), PlayMusicError> {
        self.update_frequency()?;
        self.clocks.tick_note_clock(command.note_duration());
        Ok(())
    }

    fn handle_tie_command(&mut self, command: TieCommand) -> Result<(), PlayMusicError> {
        if !matches!(
            self.commands[self.command_index.saturating_sub(2)],
            Command::Note(_)
        ) {
            return Err(PlayMusicError::new(
                command,
                "'^' must follow a note command",
            ));
        }

        self.clocks.tick_note_clock(command.note_duration());
        Ok(())
    }

    fn handle_slur_command(&mut self, command: SlurCommand) -> Result<(), PlayMusicError> {
        let Command::Note(before) = &self.commands[self.command_index.saturating_sub(2)] else {
            return Err(PlayMusicError::new(
                command,
                "'&' must follow a note command",
            ));
        };

        let Some(Command::Note(after)) = self.commands.get(self.command_index) else {
            return Err(PlayMusicError::new(
                command,
                "mssing a note command after '&'",
            ));
        };
        self.command_index += 1;

        if before.note().normalize() != after.note().normalize() {
            return Err(PlayMusicError::new(
                command,
                "'&' cannot combine different notes",
            ));
        }

        self.clocks.tick_note_clock(after.note_duration());
        Ok(())
    }

    fn handle_volume_command(&mut self, command: VolumeCommand) -> Result<(), PlayMusicError> {
        self.volume = VolumeEnvelope::constant(command.volume());
        Ok(())
    }

    fn handle_volume_envelope_command(
        &mut self,
        command: VolumeEnvelopeCommand,
    ) -> Result<(), PlayMusicError> {
        self.volume = self
            .macros
            .volumes
            .get(&command.macro_number())
            .ok_or_else(|| PlayMusicError::new(command, "undefined macro number"))?
            .envelope()
            .clone();
        Ok(())
    }

    fn handle_volume_up_command(&mut self, command: VolumeUpCommand) -> Result<(), PlayMusicError> {
        if !self.volume.is_constant() {
            return Err(PlayMusicError::new(
                command,
                "cannot be used with volume envelope",
            ));
        }
        let v = self
            .volume
            .nth_frame_item(self.clocks.frame_index())
            .checked_add(command.count())
            .ok_or_else(|| PlayMusicError::new(command, "volume overflow"))?;
        self.volume = VolumeEnvelope::constant(v);
        Ok(())
    }

    fn handle_volume_down_command(
        &mut self,
        command: VolumeDownCommand,
    ) -> Result<(), PlayMusicError> {
        if !self.volume.is_constant() {
            return Err(PlayMusicError::new(
                Command::VolumeDown(command),
                "cannot be used with volume envelope",
            ));
        }
        let v = self
            .volume
            .nth_frame_item(self.clocks.frame_index())
            .checked_sub(command.count())
            .ok_or_else(|| PlayMusicError::new(command, "volume underflow"))?;
        self.volume = VolumeEnvelope::constant(v);
        Ok(())
    }

    fn handle_octave_command(&mut self, command: OctaveCommand) -> Result<(), PlayMusicError> {
        self.octave = command.octave();
        Ok(())
    }

    fn handle_octave_up_command(&mut self, command: OctaveUpCommand) -> Result<(), PlayMusicError> {
        self.octave = self
            .octave
            .checked_add(1)
            .ok_or_else(|| PlayMusicError::new(command, "octave oveflow"))?;
        Ok(())
    }

    fn handle_octave_down_command(
        &mut self,
        command: OctaveDownCommand,
    ) -> Result<(), PlayMusicError> {
        self.octave = self
            .octave
            .checked_sub(1)
            .ok_or_else(|| PlayMusicError::new(command, "octave underflow"))?;
        Ok(())
    }

    fn handle_detune_command(&mut self, command: DetuneCommand) -> Result<(), PlayMusicError> {
        self.detune = PitchEnvelope::constant(command.detune());
        Ok(())
    }

    fn handle_pitch_envelope_command(
        &mut self,
        command: PitchEnvelopeCommand,
    ) -> Result<(), PlayMusicError> {
        if let Some(n) = command.macro_number() {
            self.detune = self
                .macros
                .pitches
                .get(&n)
                .ok_or_else(|| PlayMusicError::new(command, "undefined macro number"))?
                .envelope()
                .clone();
        } else {
            self.detune = PitchEnvelope::constant(Detune::default());
        }
        Ok(())
    }

    fn handle_pitch_sweep_command(
        &mut self,
        command: PitchSweepCommand,
    ) -> Result<(), PlayMusicError> {
        self.pitch_sweep = Some(command.sweep());
        Ok(())
    }

    fn handle_vibrato_command(&mut self, command: VibratoCommand) -> Result<(), PlayMusicError> {
        if let Some(n) = command.macro_number() {
            let vibrato = self
                .macros
                .vibratos
                .get(&n)
                .ok_or_else(|| PlayMusicError::new(command, "undefined macro number"))?
                .vibrato();
            self.pitch_lfo = Some(PitchLfo::new(
                vibrato.delay(),
                vibrato.speed(),
                vibrato.depth(),
            ));
        } else {
            self.pitch_lfo = None;
        }
        Ok(())
    }

    fn handle_timbre_command(&mut self, command: TimbreCommand) -> Result<(), PlayMusicError> {
        self.timbre = Some(Timbres::constant(command.timbre()));
        Ok(())
    }

    fn handle_timbres_command(&mut self, command: TimbresCommand) -> Result<(), PlayMusicError> {
        self.timbre = Some(
            self.macros
                .timbres
                .get(&command.macro_number())
                .ok_or_else(|| PlayMusicError::new(command, "undefined macro number"))?
                .timbres()
                .clone(),
        );
        Ok(())
    }

    fn handle_default_note_duration_command(
        &mut self,
        command: DefaultNoteDurationCommand,
    ) -> Result<(), PlayMusicError> {
        self.clocks
            .set_default_note_duration(command.default_note_duration());
        Ok(())
    }

    fn handle_tempo_command(&mut self, command: TempoCommand) -> Result<(), PlayMusicError> {
        self.clocks.set_tempo(command.tempo());
        Ok(())
    }

    fn handle_data_skip_command(
        &mut self,
        _command: DataSkipCommand,
    ) -> Result<(), PlayMusicError> {
        self.command_index = self.commands.len();
        Ok(())
    }

    fn handle_track_loop_command(
        &mut self,
        _command: TrackLoopCommand,
    ) -> Result<(), PlayMusicError> {
        self.loop_point = Some(self.command_index);
        Ok(())
    }

    fn handle_repeat_start_command(
        &mut self,
        command: RepeatStartCommand,
    ) -> Result<(), PlayMusicError> {
        let mut stack_size: isize = 1;
        for command in &self.commands[self.command_index..] {
            match command {
                Command::RepeatStart(_) => stack_size += 1,
                Command::RepeatEnd(_) => {
                    stack_size -= 1;
                    if stack_size == 0 {
                        break;
                    }
                }
                Command::DataSkip(_) => break,
                _ => {}
            }
        }
        if stack_size > 0 {
            return Err(PlayMusicError::new(command, "no maching ']'"));
        }

        self.repeat_stack.push(Repeat::new(self.command_index));
        Ok(())
    }

    fn handle_repeat_end_command(
        &mut self,
        command: RepeatEndCommand,
    ) -> Result<(), PlayMusicError> {
        let Some(mut repeat) = self.repeat_stack.pop() else {
            return Err(PlayMusicError::new(command, "no maching '['"));
        };
        if repeat.count < command.count() {
            self.command_index = repeat.start_index;
            repeat.count += 1;
            self.repeat_stack.push(repeat);
        }
        Ok(())
    }

    fn handle_tuplet_start_command(
        &mut self,
        command: TupletStartCommand,
    ) -> Result<(), PlayMusicError> {
        let mut note_count = 0;
        for c in &self.commands[self.command_index..] {
            match c {
                Command::TupletStart(_) => {
                    return Err(PlayMusicError::new(command, "nested tuplet"));
                }
                Command::TupletEnd(c) => {
                    self.clocks.set_tuplet(note_count, c.note_duration());
                    return Ok(());
                }
                Command::DataSkip(_) | Command::RepeatStart(_) | Command::RepeatEnd(_) => break,
                Command::Note(_)
                | Command::RestSign(_)
                | Command::Wait(_)
                | Command::Tie(_)
                | Command::Slur(_) => {
                    note_count += 1;
                }
                _ => {}
            }
        }
        Err(PlayMusicError::new(command, "no maching '}'"))
    }

    fn handle_tuplet_end_command(
        &mut self,
        command: TupletEndCommand,
    ) -> Result<(), PlayMusicError> {
        for c in self.commands[..self.command_index - 1].iter().rev() {
            match c {
                Command::TupletStart(_) => {
                    return Ok(());
                }
                Command::TupletEnd(_) => {
                    break;
                }
                _ => {}
            }
        }
        Err(PlayMusicError::new(command, "no maching '{'"))
    }

    fn handle_quantize_command(&mut self, command: QuantizeCommand) -> Result<(), PlayMusicError> {
        self.clocks.set_quantize(command.quantize());
        Ok(())
    }

    fn handle_quantize_frame_command(
        &mut self,
        command: QuantizeFrameCommand,
    ) -> Result<(), PlayMusicError> {
        self.clocks.set_quantize_frame(command.quantize_frame());
        Ok(())
    }
}

impl Iterator for ChannelPlayer {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        if self.eos {
            return None;
        }

        while self.last_error.is_none() {
            if self.clocks.tick_frame_clock_if_need() {
                self.last_error = self.handle_frame().err();
                if self.last_error.is_some() {
                    self.eos = true;
                    return None;
                }
            }

            if self.clocks.sample_clock() < self.clocks.note_clock() {
                return Some(self.sample());
            }

            let Some(command) = self.commands.get(self.command_index).cloned() else {
                if let Some(i) = self.loop_point {
                    self.command_index = i;
                    continue;
                }
                self.eos = true;
                return None;
            };
            self.command_index += 1;

            let result = match command {
                Command::Note(c) => self.handle_note_command(c),
                Command::Arpeggio(c) => self.handle_arpeggio_command(c),
                Command::Volume(c) => self.handle_volume_command(c),
                Command::VolumeUp(c) => self.handle_volume_up_command(c),
                Command::VolumeDown(c) => self.handle_volume_down_command(c),
                Command::VolumeEnvelope(c) => self.handle_volume_envelope_command(c),
                Command::Octave(c) => self.handle_octave_command(c),
                Command::OctaveUp(c) => self.handle_octave_up_command(c),
                Command::OctaveDown(c) => self.handle_octave_down_command(c),
                Command::Detune(c) => self.handle_detune_command(c),
                Command::PitchEnvelope(c) => self.handle_pitch_envelope_command(c),
                Command::PitchSweep(c) => self.handle_pitch_sweep_command(c),
                Command::Vibrato(c) => self.handle_vibrato_command(c),
                Command::Timbre(c) => self.handle_timbre_command(c),
                Command::Timbres(c) => self.handle_timbres_command(c),
                Command::DefaultNoteDuration(c) => self.handle_default_note_duration_command(c),
                Command::Tempo(c) => self.handle_tempo_command(c),
                Command::DataSkip(c) => self.handle_data_skip_command(c),
                Command::TrackLoop(c) => self.handle_track_loop_command(c),
                Command::RepeatStart(c) => self.handle_repeat_start_command(c),
                Command::RepeatEnd(c) => self.handle_repeat_end_command(c),
                Command::TupletStart(c) => self.handle_tuplet_start_command(c),
                Command::TupletEnd(c) => self.handle_tuplet_end_command(c),
                Command::RestSign(c) => self.handle_rest_sign_command(c),
                Command::Wait(c) => self.handle_wait_command(c),
                Command::Tie(c) => self.handle_tie_command(c),
                Command::Slur(c) => self.handle_slur_command(c),
                Command::Quantize(c) => self.handle_quantize_command(c),
                Command::QuantizeFrame(c) => self.handle_quantize_frame_command(c),
            };
            if let Err(e) = result {
                self.last_error = Some(e);
            }
        }
        None
    }
}

#[derive(Debug)]
struct Repeat {
    start_index: usize,
    count: usize,
}

impl Repeat {
    fn new(start_index: usize) -> Self {
        Self {
            start_index,
            count: 1,
        }
    }
}
