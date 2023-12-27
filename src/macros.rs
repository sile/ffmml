use crate::{
    comment::CommentsOrWhitespaces,
    types::{Int, NoteEnvelope, PitchEnvelope, Timbres, Vibrato, VolumeEnvelope},
};
use std::collections::BTreeMap;
use textparse::{
    components::{Char, Empty, Str},
    Parse, Parser, Span,
};

#[derive(Debug, Default, Clone)]
pub struct Macros {
    pub volumes: BTreeMap<MacroNumber, VolumeMacro>,
    pub timbres: BTreeMap<MacroNumber, TimbreMacro>,
    pub pitches: BTreeMap<MacroNumber, PitchMacro>,
    pub arpeggios: BTreeMap<MacroNumber, ArpeggioMacro>,
    pub vibratos: BTreeMap<MacroNumber, VibratoMacro>,
}

impl Macros {
    pub fn parse(&mut self, parser: &mut Parser) -> Option<()> {
        while parser.peek_char() == Some('@') {
            if let Some(m) = parser.parse::<VolumeMacro>() {
                self.volumes.insert(m.number(), m);
            } else if let Some(m) = parser.parse::<TimbreMacro>() {
                self.timbres.insert(m.number(), m);
            } else if let Some(m) = parser.parse::<PitchMacro>() {
                self.pitches.insert(m.number(), m);
            } else if let Some(m) = parser.parse::<ArpeggioMacro>() {
                self.arpeggios.insert(m.number(), m);
            } else if let Some(m) = parser.parse::<VibratoMacro>() {
                self.vibratos.insert(m.number(), m);
            } else {
                return None;
            }
            let _: CommentsOrWhitespaces = parser.parse()?;
        }
        Some(())
    }
}

#[derive(Debug, Clone, Copy, Span, Parse)]
pub struct MacroNumber(Int<0, 127>);

impl PartialEq for MacroNumber {
    fn eq(&self, other: &Self) -> bool {
        self.0.get() == other.0.get()
    }
}

impl Eq for MacroNumber {}

impl PartialOrd for MacroNumber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MacroNumber {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.get().cmp(&other.0.get())
    }
}

#[derive(Debug, Clone, Span, Parse)]
struct MacroKey<Prefix> {
    _at: Char<'@'>,
    _prefix: Prefix,
    number: MacroNumber,
    _space0: CommentsOrWhitespaces,
    _equal: Char<'='>,
    _space1: CommentsOrWhitespaces,
}

#[derive(Debug, Clone, Span, Parse)]
pub struct VolumeMacro {
    key: MacroKey<Char<'v'>>,
    envelope: VolumeEnvelope,
}

impl VolumeMacro {
    pub fn number(&self) -> MacroNumber {
        self.key.number
    }

    pub fn envelope(&self) -> &VolumeEnvelope {
        &self.envelope
    }
}

#[derive(Debug, Clone, Span, Parse)]
pub struct PitchMacro {
    key: MacroKey<Str<'E', 'P'>>,
    envelope: PitchEnvelope,
}

impl PitchMacro {
    pub fn number(&self) -> MacroNumber {
        self.key.number
    }

    pub fn envelope(&self) -> &PitchEnvelope {
        &self.envelope
    }
}

#[derive(Debug, Clone, Span, Parse)]
pub struct ArpeggioMacro {
    key: MacroKey<Str<'E', 'N'>>,
    envelope: NoteEnvelope,
}

impl ArpeggioMacro {
    pub fn number(&self) -> MacroNumber {
        self.key.number
    }

    pub fn envelope(&self) -> &NoteEnvelope {
        &self.envelope
    }
}

#[derive(Debug, Clone, Span, Parse)]
pub struct TimbreMacro {
    key: MacroKey<Empty>,
    timbres: Timbres,
}

impl TimbreMacro {
    pub fn number(&self) -> MacroNumber {
        self.key.number
    }

    pub fn timbres(&self) -> &Timbres {
        &self.timbres
    }
}

#[derive(Debug, Clone, Span, Parse)]
pub struct VibratoMacro {
    key: MacroKey<Str<'M', 'P'>>,
    vibrato: Vibrato,
}

impl VibratoMacro {
    pub fn number(&self) -> MacroNumber {
        self.key.number
    }

    pub fn vibrato(&self) -> &Vibrato {
        &self.vibrato
    }
}
