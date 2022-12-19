use crate::types::U8;
use std::collections::BTreeMap;
use textparse::{Parse, ParseError, ParseResult, Parser, Span};

#[derive(Debug, Default, Clone)]
pub struct Macros {
    //pub volumes: BTreeMap<MacroNumber, VolumeMacro>,
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
