use crate::{
    channel::ChannelNames,
    comment::{Comment, MaybeComment},
    types::OscillatorKind,
};
use std::marker::PhantomData;
use textparse::{
    components::{Char, NonEmpty, OneOfThree, StartsWith, StaticStr, While},
    Parse, ParseError, ParseResult, Parser, Position, Span,
};

#[derive(Debug, Clone, Span, Parse)]
pub enum Definition {
    Title(Title),
    Composer(Composer),
    Programer(Programer),
    Channel(Channel),
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
pub struct Channel {
    start: Position,
    channel_names: ChannelNames,
    oscillator_kind: OscillatorKind,
    end: Position,
}

impl Channel {
    pub fn channel_names(&self) -> &ChannelNames {
        &self.channel_names
    }

    pub fn oscillator_kind(&self) -> OscillatorKind {
        self.oscillator_kind
    }
}

impl Parse for Channel {
    fn parse(parser: &mut Parser) -> ParseResult<Self> {
        let start = parser.current_position();
        let _: Char<'#'> = parser.parse()?;
        let _: StartsWith<ChannelStr> = parser.parse()?;
        let _: NonEmpty<While<SpaceOrTabOrComment>> = parser.parse()?;
        let channel_names = parser.parse()?;
        let _: NonEmpty<While<SpaceOrTabOrComment>> = parser.parse()?;
        let oscillator_kind = parser.parse()?;
        let end = parser.current_position();

        Ok(Self {
            start,
            channel_names,
            oscillator_kind,
            end,
        })
    }

    fn name() -> Option<fn() -> String> {
        Some(|| "#CHANNEL".to_owned())
    }
}

#[derive(Debug, Clone)]
struct ChannelStr;

impl StaticStr for ChannelStr {
    fn static_str() -> &'static str {
        "CHANNEL"
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
        if value.is_empty() {
            return Err(ParseError);
        }
        Ok(Self {
            start,
            label: PhantomData,
            value,
            end,
        })
    }
}

type SpaceOrTabOrComment = OneOfThree<Char<' ', false>, Char<'\t', false>, Comment>;
