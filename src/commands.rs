use crate::types::{
    DefaultNoteDuration, Detune, Note, NoteDuration, Octave, Tempo, Timbre, Volume,
};
use textparse::{
    components::{Char, Either, StartsWith, StaticStr},
    Parse, Span,
};

#[derive(Debug, Clone, Span, Parse)]
#[parse(name = "command")]
pub enum Command {
    Note(NoteCommand),
    Volume(VolumeCommand),
    Octave(OctaveCommand),
    Detune(DetuneCommand),
    Timbre(TimbreCommand),
    DefaultNoteDuration(DefaultNoteDurationCommand),
    Tempo(TempoCommand),
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
pub struct DefaultNoteDurationCommand {
    _prefix: Char<'l'>,
    duration: DefaultNoteDuration,
}

impl DefaultNoteDurationCommand {
    pub fn default_note_duration(&self) -> DefaultNoteDuration {
        self.duration
    }
}

#[derive(Debug, Clone, Span, Parse)]
pub struct TempoCommand {
    _prefix: Char<'t'>,
    tempo: Tempo,
}

impl TempoCommand {
    pub fn tempo(&self) -> Tempo {
        self.tempo
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

#[derive(Debug, Clone, Span, Parse)]
pub struct DetuneCommand {
    _prefix: Char<'D'>,
    detune: Either<StartsWith<N255>, Detune>,
}

impl DetuneCommand {
    pub fn detune(&self) -> Detune {
        match self.detune {
            Either::A(_) => Detune::default(),
            Either::B(d) => d,
        }
    }
}

#[derive(Debug)]
struct N255;

impl StaticStr for N255 {
    fn static_str() -> &'static str {
        "255"
    }
}

#[derive(Debug, Clone, Span, Parse)]
pub struct TimbreCommand {
    _prefix: Char<'@'>,
    timbre: Timbre,
}

impl TimbreCommand {
    pub fn timbre(&self) -> Timbre {
        self.timbre
    }
}
