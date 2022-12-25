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

pub use self::music::{Music, ParseMusicError};
pub use self::player::{MusicPlayer, PlayMusicError};
