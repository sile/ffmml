use crate::{
    channel::ChannelNames,
    comment::{Comment, MaybeComment},
    types::OscillatorKind,
};
use std::marker::PhantomData;
use textparse::{
    components::{Char, NonEmpty, OneOfThree, Str, While},
    Parse, Parser, Position, Span,
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
pub struct Title(DefineString<Str<'T', 'I', 'T', 'L', 'E'>>);

impl Title {
    pub fn get(&self) -> &str {
        &self.0.value
    }
}

#[derive(Debug, Clone, Span, Parse)]
#[parse(name = "#COMPOSER")]
pub struct Composer(DefineString<Str<'C', 'O', 'M', 'P', 'O', 'S', 'E', 'R'>>);

impl Composer {
    pub fn get(&self) -> &str {
        &self.0.value
    }
}

#[derive(Debug, Clone, Span, Parse)]
#[parse(name = "#PROGRAMER")]
pub struct Programer(DefineString<Str<'P', 'R', 'O', 'G', 'R', 'A', 'M', 'E', 'R'>>);

impl Programer {
    pub fn get(&self) -> &str {
        &self.0.value
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
    fn parse(parser: &mut Parser) -> Option<Self> {
        let start = parser.current_position();
        let _: Str<'#', 'C', 'A', 'N', 'N', 'E', 'L'> = parser.parse()?;
        let _: NonEmpty<While<SpaceOrTabOrComment>> = parser.parse()?;
        let channel_names = parser.parse()?;
        let _: NonEmpty<While<SpaceOrTabOrComment>> = parser.parse()?;
        let oscillator_kind = parser.parse()?;
        let end = parser.current_position();

        Some(Self {
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

#[derive(Debug, Clone, Span)]
struct DefineString<T> {
    start: Position,
    label: PhantomData<T>,
    value: String,
    end: Position,
}

impl<T: Parse> Parse for DefineString<T> {
    fn parse(parser: &mut Parser) -> Option<Self> {
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
            return None;
        }
        Some(Self {
            start,
            label: PhantomData,
            value,
            end,
        })
    }
}

type SpaceOrTabOrComment = OneOfThree<Char<' ', false>, Char<'\t', false>, Comment>;
