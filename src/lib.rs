//! TODO
#![warn(missing_docs)]

mod channel;
mod clocks;
mod commands;
mod comment;
mod definitions;
mod macros;
mod music;
mod oscillators;
mod player;
mod traits;
mod types;

pub use self::channel::ChannelName;
pub use self::music::{Music, ParseMusicError};
pub use self::player::{ChannelState, MusicPlayer, PlayMusicError};
