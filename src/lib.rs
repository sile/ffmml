pub mod channel;
pub mod commands;
pub mod macros;
pub mod oscillators;
pub mod types;

mod definitions;
mod music;

pub use self::music::{Music, ParseMusicError};
