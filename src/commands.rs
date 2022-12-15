use crate::types::{
    DefaultNoteDuration, Detune, Digit, NonZeroU4, NonZeroU8, Note, NoteDuration, Octave, Tempo,
    Timbre, Volume,
};
use textparse::{
    components::{Char, Either, Not, StartsWith, StaticStr},
    Parse, Span,
};

#[derive(Debug, Clone, Span, Parse)]
#[parse(name = "command")]
pub enum Command {
    Note(NoteCommand),
    Volume(VolumeCommand),
    VolumeUp(VolumeUpCommand),
    VolumeDown(VolumeDownCommand),
    Octave(OctaveCommand),
    OctaveUp(OctaveUpCommand),
    OctaveDown(OctaveDownCommand),
    Detune(DetuneCommand),
    Timbre(TimbreCommand),
    DefaultNoteDuration(DefaultNoteDurationCommand),
    Tempo(TempoCommand),
    DataSkip(DataSkipCommand),
    TrackLoop(TrackLoopCommand),
    RepeatStart(RepeatStartCommand),
    RepeatEnd(RepeatEndCommand),
    TupletStart(TupletStartCommand),
    TupletEnd(TupletEndCommand),
    RestSign(RestSignCommand),
    Wait(WaitCommand),
    Tie(TieCommand),
    Slur(SlurCommand),
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
pub struct VolumeUpCommand {
    _prefix0: Char<'v'>,
    _prefix1: Char<'+'>,
    count: Either<NonZeroU4, Not<Digit>>,
}

impl VolumeUpCommand {
    pub fn count(&self) -> u8 {
        match self.count {
            Either::A(x) => x.get(),
            Either::B(_) => 1,
        }
    }
}

#[derive(Debug, Clone, Span, Parse)]
pub struct VolumeDownCommand {
    _prefix0: Char<'v'>,
    _prefix1: Char<'-'>,
    count: Either<NonZeroU4, Not<Digit>>,
}

impl VolumeDownCommand {
    pub fn count(&self) -> u8 {
        match self.count {
            Either::A(x) => x.get(),
            Either::B(_) => 1,
        }
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
pub struct OctaveUpCommand {
    _prefix: Char<'>'>,
}

#[derive(Debug, Clone, Span, Parse)]
pub struct OctaveDownCommand {
    _prefix: Char<'<'>,
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

#[derive(Debug, Clone, Span, Parse)]
pub struct DataSkipCommand {
    _prefix: Char<'!'>,
}

#[derive(Debug, Clone, Span, Parse)]
pub struct TrackLoopCommand {
    _prefix: Char<'L'>,
}

#[derive(Debug, Clone, Span, Parse)]
pub struct RepeatStartCommand {
    _prefix: Char<'['>,
}

#[derive(Debug, Clone, Span, Parse)]
pub struct RepeatEndCommand {
    _prefix: Char<']'>,
    count: NonZeroU8,
}

impl RepeatEndCommand {
    pub fn count(self) -> usize {
        usize::from(self.count.get())
    }
}

#[derive(Debug, Clone, Span, Parse)]
pub struct TupletStartCommand {
    _prefix: Char<'{'>,
}

#[derive(Debug, Clone, Span, Parse)]
pub struct TupletEndCommand {
    _prefix: Char<'}'>,
    duration: NoteDuration,
}

impl TupletEndCommand {
    pub fn note_duration(&self) -> NoteDuration {
        self.duration
    }
}

#[derive(Debug, Clone, Span, Parse)]
pub struct RestSignCommand {
    _prefix: Char<'r'>,
    duration: NoteDuration,
}

impl RestSignCommand {
    pub fn note_duration(&self) -> NoteDuration {
        self.duration
    }
}

#[derive(Debug, Clone, Span, Parse)]
pub struct WaitCommand {
    _prefix: Char<'w'>,
    duration: NoteDuration,
}

impl WaitCommand {
    pub fn note_duration(&self) -> NoteDuration {
        self.duration
    }
}

#[derive(Debug, Clone, Span, Parse)]
pub struct TieCommand {
    _prefix: Char<'^'>,
    duration: NoteDuration,
}

impl TieCommand {
    pub fn note_duration(&self) -> NoteDuration {
        self.duration
    }
}

#[derive(Debug, Clone, Span, Parse)]
pub struct SlurCommand {
    _prefix: Char<'&'>,
}
