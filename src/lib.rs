pub mod channel;
pub mod commands;
pub mod macros;
pub mod oscillators;
pub mod types;

mod comment;
mod definitions;
mod music;
mod player;

pub use self::music::{Music, ParseMusicError};
pub use self::player::{MusicPlayer, PlayMusicError};
