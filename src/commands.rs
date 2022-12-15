use crate::types::{Note, NoteDuration};
use textparse::{components::Maybe, Parse, Span};

#[derive(Debug, Clone, Span, Parse)]
#[parse(name = "command")]
pub enum Command {
    Note(NoteCommand),
}

#[derive(Debug, Clone, Span, Parse)]
pub struct NoteCommand {
    note: Note,
    duration: Maybe<NoteDuration>,
}

impl NoteCommand {
    pub fn note(&self) -> &Note {
        &self.note
    }

    pub fn duration(&self) -> Option<&NoteDuration> {
        self.duration.get()
    }
}
