use std::marker::PhantomData;

use crate::comment::{Comment, MaybeComment};
use textparse::{
    components::{Char, NonEmpty, OneOfThree, StartsWith, StaticStr, While},
    Parse, ParseResult, Parser, Position, Span,
};

#[derive(Debug, Span)]
pub enum Definition {
    Title(Title),
    Composer(Composer),
    Programer(Programer),
}

#[derive(Debug, Clone, Span, Parse)]
#[parse(name = "#TITLE")]
pub struct Title(DefineString<StartsWith<TitleStr>>);

impl Title {
    pub fn get(&self) -> &str {
        &self.0.value
    }
}

#[derive(Debug, Clone)]
struct TitleStr;

impl StaticStr for TitleStr {
    fn static_str() -> &'static str {
        "TITLE"
    }
}

#[derive(Debug, Clone, Span, Parse)]
#[parse(name = "#COMPOSER")]
pub struct Composer(DefineString<StartsWith<ComposerStr>>);

impl Composer {
    pub fn get(&self) -> &str {
        &self.0.value
    }
}

#[derive(Debug, Clone)]
struct ComposerStr;

impl StaticStr for ComposerStr {
    fn static_str() -> &'static str {
        "COMPOSER"
    }
}

#[derive(Debug, Clone, Span, Parse)]
#[parse(name = "#PROGRAMER")]
pub struct Programer(DefineString<StartsWith<ProgramerStr>>);

impl Programer {
    pub fn get(&self) -> &str {
        &self.0.value
    }
}

#[derive(Debug, Clone)]
struct ProgramerStr;

impl StaticStr for ProgramerStr {
    fn static_str() -> &'static str {
        "PROGRAMER"
    }
}

#[derive(Debug, Clone, Span)]
struct DefineString<T> {
    start: Position,
    label: PhantomData<T>,
    value: String,
    end: Position,
}

impl<T: Parse> Parse for DefineString<T> {
    fn parse(parser: &mut Parser) -> ParseResult<Self> {
        let start = parser.current_position();
        let _: (Char<'#'>, T, NonEmpty<While<SpaceOrTabOrComment>>) = parser.parse()?;
        let mut end;
        let mut value = String::new();
        loop {
            let _: MaybeComment = parser.parse()?;

            end = parser.current_position();
            let c = parser.read_char().unwrap_or('\n');
            if c == '\n' {
                break;
            }
            value.push(c);
        }
        Ok(Self {
            start,
            label: PhantomData,
            value,
            end,
        })
    }
}

type SpaceOrTabOrComment = OneOfThree<Char<' '>, Char<'\t'>, Comment>;
