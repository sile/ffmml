use crate::{
    macros::MacroNumber,
    types::{
        DefaultNoteDuration, Detune, Int, Note, NoteDuration, Octave, PitchSweep, Quantize,
        QuantizeFrame, Tempo, Timbre, Volume,
    },
};
use textparse::{
    components::{Char, Digit, Either, NonEmpty, Not, Str},
    Parse, Span,
};

#[derive(Debug, Clone, Span, Parse)]
#[parse(name = "command")]
pub enum Command {
    Note(NoteCommand),
    Arpeggio(ArpeggioCommand),
    Volume(VolumeCommand),
    VolumeUp(VolumeUpCommand),
    VolumeDown(VolumeDownCommand),
    VolumeEnvelope(VolumeEnvelopeCommand),
    Octave(OctaveCommand),
    OctaveUp(OctaveUpCommand),
    OctaveDown(OctaveDownCommand),
    Detune(DetuneCommand),
    PitchEnvelope(PitchEnvelopeCommand),
    PitchSweep(PitchSweepCommand),
    Vibrato(VibratoCommand),
    Timbre(TimbreCommand),
    Timbres(TimbresCommand),
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
    Quantize(QuantizeCommand),
    QuantizeFrame(QuantizeFrameCommand),
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
pub struct ArpeggioCommand {
    _prefix: Str<'E', 'N'>,
    macro_number: Either<MacroNumber, Off>,
}

impl ArpeggioCommand {
    pub fn macro_number(&self) -> Option<MacroNumber> {
        if let Either::A(n) = self.macro_number {
            Some(n)
        } else {
            None
        }
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
pub struct VolumeEnvelopeCommand {
    _prefix: Str<'@', 'v'>,
    macro_number: MacroNumber,
}

impl VolumeEnvelopeCommand {
    pub fn macro_number(&self) -> MacroNumber {
        self.macro_number
    }
}

#[derive(Debug, Clone, Span, Parse)]
pub struct VolumeUpCommand {
    _prefix: Str<'v', '+'>,
    count: Either<Int<1, 15>, Not<Digit>>,
}

impl VolumeUpCommand {
    pub fn count(&self) -> u8 {
        match self.count {
            Either::A(x) => x.get() as u8,
            Either::B(_) => 1,
        }
    }
}

#[derive(Debug, Clone, Span, Parse)]
pub struct VolumeDownCommand {
    _prefix: Str<'v', '-'>,
    count: Either<Int<1, 15>, Not<Digit>>,
}

impl VolumeDownCommand {
    pub fn count(&self) -> u8 {
        match self.count {
            Either::A(x) => x.get() as u8,
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
    detune: Either<N255, Detune>,
}

impl DetuneCommand {
    pub fn detune(&self) -> Detune {
        match self.detune {
            Either::A(_) => Detune::default(),
            Either::B(d) => d,
        }
    }
}

#[derive(Debug, Clone, Span, Parse)]
pub struct PitchEnvelopeCommand {
    _prefix: Str<'E', 'P'>,
    macro_number: Either<MacroNumber, Off>,
}

impl PitchEnvelopeCommand {
    pub fn macro_number(&self) -> Option<MacroNumber> {
        if let Either::A(n) = self.macro_number {
            Some(n)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Span, Parse)]
pub struct PitchSweepCommand {
    _prefix: Char<'s'>,
    sweep: PitchSweep,
}

impl PitchSweepCommand {
    pub fn sweep(&self) -> PitchSweep {
        self.sweep
    }
}

#[derive(Debug, Clone, Span, Parse)]
pub struct VibratoCommand {
    _prefix: Str<'M', 'P'>,
    macro_number: Either<MacroNumber, Off>,
}

impl VibratoCommand {
    pub fn macro_number(&self) -> Option<MacroNumber> {
        if let Either::A(n) = self.macro_number {
            Some(n)
        } else {
            None
        }
    }
}

type N255 = Str<'2', '5', '5'>;

type Of = Str<'O', 'F'>;

type Off = Either<N255, Of>;

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
pub struct TimbresCommand {
    _prefix: Str<'@', '@'>,
    macro_number: MacroNumber,
}

impl TimbresCommand {
    pub fn macro_number(&self) -> MacroNumber {
        self.macro_number
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
    count: Int<1, 255>,
}

impl RepeatEndCommand {
    pub fn count(self) -> usize {
        self.count.get() as usize
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
    duration: NonEmpty<NoteDuration>,
}

impl TieCommand {
    pub fn note_duration(&self) -> NoteDuration {
        *self.duration.get()
    }
}

#[derive(Debug, Clone, Span, Parse)]
pub struct SlurCommand {
    _prefix: Char<'&'>,
}

#[derive(Debug, Clone, Span, Parse)]
pub struct QuantizeCommand {
    _prefix: Char<'q'>,
    quantize: Quantize,
}

impl QuantizeCommand {
    pub fn quantize(&self) -> Quantize {
        self.quantize
    }
}

#[derive(Debug, Clone, Span, Parse)]
pub struct QuantizeFrameCommand {
    _prefix: Str<'@', 'q'>,
    quantize: QuantizeFrame,
}

impl QuantizeFrameCommand {
    pub fn quantize_frame(&self) -> QuantizeFrame {
        self.quantize
    }
}
