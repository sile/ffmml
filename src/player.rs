use crate::{
    channel::{Channel, ChannelName},
    clocks::Clocks,
    commands::{Command, NoteCommand, OctaveCommand, VolumeCommand},
    oscillators::Oscillator,
    types::{Detune, Note, Octave, Sample, Volume},
    Music,
};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct MusicPlayer {
    channels: BTreeMap<ChannelName, ChannelPlayer>,
}

impl MusicPlayer {
    pub(crate) fn new(music: Music, sample_rate: u16) -> Self {
        let channels = music
            .into_channels()
            .into_iter()
            .map(|(name, channel)| (name, ChannelPlayer::new(channel, sample_rate)))
            .collect();
        Self { channels }
    }

    // TODO: seek, position, duration
}

impl Iterator for MusicPlayer {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        let mut sample = None;
        for x in self.channels.values_mut().flat_map(|c| c.next()) {
            sample = Some(sample.unwrap_or(Sample::ZERO) + x);
        }
        sample
    }
}

#[derive(Debug)]
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
            last_error: None,
        }
    }

    fn sample(&mut self) -> Sample {
        self.clocks.tick_sample_clock();
        let sample = self.oscillator.sample(self.clocks.sample_rate());
        sample * self.volume.as_ratio()
    }

    fn handle_note_command(&mut self, command: NoteCommand) -> Result<(), PlayMusicError> {
        self.oscillator
            .set_frequency(command.note(), self.octave, self.detune);
        self.clocks.tick_note_clock(command.note_duration());
        self.clocks.set_frame_clock(self.clocks.sample_clock());
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
}

impl Iterator for ChannelPlayer {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        while self.last_error.is_none() {
            if self.clocks.sample_clock() < self.clocks.note_clock() {
                return Some(self.sample());
            }

            let Some(command) = self.commands.get(self.command_index).cloned() else {
                return None;
            };
            self.command_index += 1;

            let result = match command {
                Command::Note(c) => self.handle_note_command(c),
                Command::Volume(c) => self.handle_volume_command(c),
                Command::Octave(c) => self.handle_octave_command(c),
            };
            if let Err(e) = result {
                self.last_error = Some(e);
            }
        }
        None
    }
}
