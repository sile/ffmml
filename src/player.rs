use crate::{
    channel::{Channel, ChannelName},
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

#[derive(Debug)]
pub struct PlayMusicError;

#[derive(Debug)]
struct ChannelPlayer {
    channel: Channel,
    sample_rate: u16,
}

impl ChannelPlayer {
    fn new(channel: Channel, sample_rate: u16) -> Self {
        Self {
            channel,
            sample_rate,
        }
    }
}
