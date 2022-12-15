use crate::types::{Note, NoteDuration};
use textparse::{Parse, Span};

#[derive(Debug, Clone, Span, Parse)]
#[parse(name = "command")]
pub enum Command {
    Note(NoteCommand),
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
