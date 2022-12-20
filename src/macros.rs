use crate::{
    comment::CommentsOrWhitespaces,
    types::{PitchEnvelope, Timbres, VolumeEnvelope, U8},
};
use std::collections::BTreeMap;
use textparse::{
    components::{Char, Empty},
    Parse, ParseError, ParseResult, Parser, Span,
};

#[derive(Debug, Default, Clone)]
pub struct Macros {
    pub volumes: BTreeMap<MacroNumber, VolumeMacro>,
    pub timbres: BTreeMap<MacroNumber, TimbreMacro>,
    pub pitches: BTreeMap<MacroNumber, PitchMacro>,
}

impl Macros {
    pub fn parse(&mut self, parser: &mut Parser) -> ParseResult<()> {
        while parser.peek_char() == Some('@') {
            if let Ok(m) = parser.parse::<VolumeMacro>() {
                self.volumes.insert(m.number(), m);
            } else if let Ok(m) = parser.parse::<TimbreMacro>() {
                self.timbres.insert(m.number(), m);
            } else if let Ok(m) = parser.parse::<PitchMacro>() {
                self.pitches.insert(m.number(), m);
            } else {
                return Err(ParseError);
            }
            let _: CommentsOrWhitespaces = parser.parse()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Span)]
pub struct MacroNumber(U8);

impl Parse for MacroNumber {
    fn parse(parser: &mut Parser) -> ParseResult<Self> {
        let n: U8 = parser.parse()?;
        if n.get() < 128 {
            Ok(Self(n))
        } else {
            Err(ParseError)
        }
    }

    fn name() -> Option<fn() -> String> {
        Some(|| "an integer between 0 and 127".to_owned())
    }
}

impl PartialEq for MacroNumber {
    fn eq(&self, other: &Self) -> bool {
        self.0.get() == other.0.get()
    }
}

impl Eq for MacroNumber {}

impl PartialOrd for MacroNumber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.get().partial_cmp(&other.0.get())
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
    key: MacroKey<(Char<'E'>, Char<'P'>)>,
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
