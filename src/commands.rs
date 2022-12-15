use crate::types::{Note, NoteDuration, Volume};
use textparse::{components::Char, Parse, Span};

#[derive(Debug, Clone, Span, Parse)]
#[parse(name = "command")]
pub enum Command {
    Note(NoteCommand),
    Volume(VolumeCommand),
}

#[derive(Debug, Clone, Span, Parse)]
pub struct NoteCommand {
    note: Note,
    duration: NoteDuration,
}

impl NoteCommand {
    pub fn note(&self) -> Note {
        self.note
    }

    pub fn note_duration(&self) -> NoteDuration {
        self.duration
    }
}

#[derive(Debug, Clone, Span, Parse)]
pub struct VolumeCommand {
    _prefix: Char<'v'>,
    volume: Volume,
}

impl VolumeCommand {
    pub fn volume(&self) -> Volume {
        self.volume
    }
}
