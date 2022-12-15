use crate::{
    channel::{Channel, ChannelName},
    clocks::Clocks,
    commands::{
        Command, DataSkipCommand, DefaultNoteDurationCommand, DetuneCommand, NoteCommand,
        OctaveCommand, RepeatEndCommand, RepeatStartCommand, RestSignCommand, TempoCommand,
        TimbreCommand, TrackLoopCommand, VolumeCommand, WaitCommand,
    },
    oscillators::Oscillator,
    types::{Detune, Note, Octave, Sample, Volume},
    Music,
};
use std::{collections::BTreeMap, time::Duration};
use textparse::Span;

#[derive(Debug)]
pub struct MusicPlayer {
    channels: BTreeMap<ChannelName, ChannelPlayer>,
    last_error: Option<PlayMusicError>,
}

impl MusicPlayer {
    pub(crate) fn new(music: Music, sample_rate: u16) -> Self {
        let channels = music
            .into_channels()
            .into_iter()
            .map(|(name, channel)| (name, ChannelPlayer::new(channel, sample_rate)))
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

    pub fn position(&self) -> Duration {
        self.channels
            .values()
            .map(|c| c.clocks.sample_clock().now())
            .max()
            .unwrap_or_default()
    }

    // TODO: seek
}

impl Iterator for MusicPlayer {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        if self.last_error.is_some() {
            return None;
        }

        let mut sample = None;
        for x in self.channels.values_mut().flat_map(|c| c.next()) {
            sample = Some(sample.unwrap_or(Sample::ZERO) + x);
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
    pub command: Command,
    pub reason: String,
}

impl PlayMusicError {
    fn new(command: Command, reason: &str) -> Self {
        Self {
            channel: ChannelName::A, // dummy initial value.
            command,
            reason: reason.to_string(),
        }
    }

    pub fn to_string(&self, text: &str, filename: Option<&str>) -> String {
        let offset = self.command.start_position().get();
        let (line, column) = self.command.start_position().line_and_column(text);
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
    octave: Octave,
    detune: Detune,
    volume: Volume,
    clocks: Clocks,
    loop_point: Option<usize>,
    repeat_stack: Vec<Repeat>,
    note: Option<Note>,
    last_error: Option<PlayMusicError>,
}

impl ChannelPlayer {
    fn new(channel: Channel, sample_rate: u16) -> Self {
        Self {
            oscillator: channel.oscillator,
            commands: channel.commands,
            command_index: 0,
            octave: Octave::default(),
            detune: Detune::default(),
            volume: Volume::default(),
            clocks: Clocks::new(sample_rate),
            loop_point: None,
            repeat_stack: Vec::new(),
            note: None,
            last_error: None,
        }
    }

    fn sample(&mut self) -> Sample {
        self.clocks.tick_sample_clock();
        if self.note.is_none() {
            Sample::ZERO
        } else {
            let sample = self.oscillator.sample(self.clocks.sample_rate());
            sample * self.volume.as_ratio()
        }
    }

    fn handle_frame(&mut self) {}

    fn handle_note_command(&mut self, command: NoteCommand) -> Result<(), PlayMusicError> {
        self.oscillator
            .set_frequency(command.note(), self.octave, self.detune);
        self.clocks.tick_note_clock(command.note_duration());
        self.clocks.set_frame_clock(self.clocks.sample_clock());
        self.note = Some(command.note());
        Ok(())
    }

    fn handle_rest_sign_command(&mut self, command: RestSignCommand) -> Result<(), PlayMusicError> {
        self.clocks.tick_note_clock(command.note_duration());
        self.clocks.set_frame_clock(self.clocks.sample_clock());
        self.note = None;
        Ok(())
    }

    fn handle_wait_command(&mut self, command: WaitCommand) -> Result<(), PlayMusicError> {
        if let Some(note) = self.note {
            self.oscillator
                .set_frequency(note, self.octave, self.detune);
        }
        self.clocks.tick_note_clock(command.note_duration());
        Ok(())
    }

    fn handle_volume_command(&mut self, command: VolumeCommand) -> Result<(), PlayMusicError> {
        self.volume = command.volume();
        Ok(())
    }

    fn handle_octave_command(&mut self, command: OctaveCommand) -> Result<(), PlayMusicError> {
        self.octave = command.octave();
        Ok(())
    }

    fn handle_detune_command(&mut self, command: DetuneCommand) -> Result<(), PlayMusicError> {
        self.detune = command.detune();
        Ok(())
    }

    fn handle_timbre_command(&mut self, command: TimbreCommand) -> Result<(), PlayMusicError> {
        if self.oscillator.set_timbre(command.timbre()) {
            Ok(())
        } else {
            Err(PlayMusicError::new(
                Command::Timbre(command),
                "unsupported timbre value",
            ))
        }
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
                Command::RepeatEnd(_) => stack_size -= 1,
                _ => {}
            }
        }
        if stack_size > 0 {
            return Err(PlayMusicError::new(
                Command::RepeatStart(command),
                "no maching ']' command",
            ));
        }

        self.repeat_stack.push(Repeat::new(self.command_index));
        Ok(())
    }

    fn handle_repeat_end_command(
        &mut self,
        command: RepeatEndCommand,
    ) -> Result<(), PlayMusicError> {
        let Some(mut repeat) = self.repeat_stack.pop() else {
            return Err(PlayMusicError::new(
                Command::RepeatEnd(command),
                "no maching '[' command",
            ));
        };
        if repeat.count < command.count() {
            self.command_index = repeat.start_index;
            repeat.count += 1;
            self.repeat_stack.push(repeat);
        }
        Ok(())
    }
}

impl Iterator for ChannelPlayer {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        while self.last_error.is_none() {
            if self.clocks.tick_frame_clock_if_need() {
                self.handle_frame();
            }

            if self.clocks.sample_clock() < self.clocks.note_clock() {
                return Some(self.sample());
            }

            let Some(command) = self.commands.get(self.command_index).cloned() else {
                if let Some(i) = self.loop_point {
                    self.command_index = i;
                    continue;
                }
                return None;
            };
            self.command_index += 1;

            let result = match command {
                Command::Note(c) => self.handle_note_command(c),
                Command::Volume(c) => self.handle_volume_command(c),
                Command::Octave(c) => self.handle_octave_command(c),
                Command::Detune(c) => self.handle_detune_command(c),
                Command::Timbre(c) => self.handle_timbre_command(c),
                Command::DefaultNoteDuration(c) => self.handle_default_note_duration_command(c),
                Command::Tempo(c) => self.handle_tempo_command(c),
                Command::DataSkip(c) => self.handle_data_skip_command(c),
                Command::TrackLoop(c) => self.handle_track_loop_command(c),
                Command::RepeatStart(c) => self.handle_repeat_start_command(c),
                Command::RepeatEnd(c) => self.handle_repeat_end_command(c),
                Command::RestSign(c) => self.handle_rest_sign_command(c),
                Command::Wait(c) => self.handle_wait_command(c),
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
