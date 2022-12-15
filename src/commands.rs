use crate::types::{Note, NoteDuration, Octave, Volume};
use textparse::{components::Char, Parse, Span};

#[derive(Debug, Clone, Span, Parse)]
#[parse(name = "command")]
pub enum Command {
    Note(NoteCommand),
    Volume(VolumeCommand),
    Octave(OctaveCommand),
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

#[derive(Debug, Clone, Span, Parse)]
pub struct OctaveCommand {
    _prefix: Char<'o'>,
    octave: Octave,
}

impl OctaveCommand {
    pub fn octave(&self) -> Octave {
        self.octave
    }
}
