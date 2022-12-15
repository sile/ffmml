use crate::{
    channel::{Channel, ChannelName},
    commands::{Command, NoteCommand},
    types::{Detune, Octave, Sample},
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
pub struct PlayMusicError;

#[derive(Debug)]
struct ChannelPlayer {
    channel: Channel,
    sample_rate: u16,
    command_index: usize,
    octave: Octave,
    detune: Detune,
    last_error: Option<PlayMusicError>,
}

impl ChannelPlayer {
    fn new(channel: Channel, sample_rate: u16) -> Self {
        Self {
            channel,
            sample_rate,
            command_index: 0,
            octave: Octave::default(),
            detune: Detune::default(),
            last_error: None,
        }
    }

    fn handle_note_command(&mut self, command: NoteCommand) -> Result<(), PlayMusicError> {
        if !self
            .channel
            .oscillator
            .set_frequency(command.note(), self.octave, self.detune)
        {
            panic!("bug");
        }

        Ok(())
    }
}

impl Iterator for ChannelPlayer {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(command) = self.channel.commands.get(self.command_index).cloned() else {
            return None;
        };
        self.command_index += 1;

        let result = match command {
            Command::Note(c) => self.handle_note_command(c),
        };
        match result {
            Err(e) => {
                self.last_error = Some(e);
                None
            }
            Ok(_) => todo!(),
        }
    }
}
