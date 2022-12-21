use crate::{
    commands::Command,
    comment::{Comment, CommentsOrWhitespaces},
    oscillators::Oscillator,
};
use std::collections::{BTreeMap, BTreeSet};
use textparse::{
    components::{Either, Whitespace},
    Parse, ParseError, ParseResult, Parser, Position, Span,
};

#[derive(Debug, Clone)]
pub struct Channels(BTreeMap<ChannelName, Channel>);

impl Channels {
    pub fn new() -> Self {
        let channels = [
            (ChannelName::A, Channel::new(Oscillator::pulse_wave())),
            (ChannelName::B, Channel::new(Oscillator::pulse_wave())),
            (ChannelName::C, Channel::new(Oscillator::triangle_wave())),
            (ChannelName::D, Channel::new(Oscillator::noise())),
        ]
        .into_iter()
        .collect();
        Self(channels)
    }

    pub fn parse(&mut self, parser: &mut Parser) -> ParseResult<()> {
        while !parser.is_eos() {
            let names = parser
                .parse::<ChannelNames>()?
                .check_if_defined(parser, &self)?
                .names;
            let _: Space = parser.parse()?;
            let _: CommentsOrWhitespaces = parser.parse()?;

            let mut has_space = false;
            while let Ok(command) = parser.parse::<Command>() {
                for name in &names {
                    self.0
                        .get_mut(name)
                        .expect("unreachable")
                        .commands
                        .push(command.clone());
                }

                let space: CommentsOrWhitespaces = parser.parse()?;
                has_space = !space.is_empty();
            }

            if !has_space && !parser.is_eos() {
                // Needs spaces before channel names.
                return Err(ParseError);
            }
        }
        Ok(())
    }

    pub fn into_iter(self) -> impl Iterator<Item = (ChannelName, Channel)> {
        self.0.into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChannelName {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl ChannelName {
    fn from_char(c: char) -> Option<Self> {
        Some(match c {
            'A' => ChannelName::A,
            'B' => ChannelName::B,
            'C' => ChannelName::C,
            'D' => ChannelName::D,
            'E' => ChannelName::E,
            'F' => ChannelName::F,
            'G' => ChannelName::G,
            'H' => ChannelName::H,
            'I' => ChannelName::I,
            'J' => ChannelName::J,
            'K' => ChannelName::K,
            'L' => ChannelName::L,
            'M' => ChannelName::M,
            'N' => ChannelName::N,
            'O' => ChannelName::O,
            'P' => ChannelName::P,
            'Q' => ChannelName::Q,
            'R' => ChannelName::R,
            'S' => ChannelName::S,
            'T' => ChannelName::T,
            'U' => ChannelName::U,
            'V' => ChannelName::V,
            'W' => ChannelName::W,
            'X' => ChannelName::X,
            'Y' => ChannelName::Y,
            'Z' => ChannelName::Z,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Span)]
struct ChannelNames {
    start: Position,
    names: BTreeSet<ChannelName>,
    end: Position,
}

impl ChannelNames {
    fn check_if_defined(mut self, parser: &mut Parser, channels: &Channels) -> ParseResult<Self> {
        if let Some(i) = self.names.iter().position(|n| !channels.0.contains_key(n)) {
            self.start = Position::new(self.start.get() + i);
            parser.rollback(self);
            Err(ParseError)
        } else {
            Ok(self)
        }
    }
}

impl Parse for ChannelNames {
    fn parse(parser: &mut Parser) -> ParseResult<Self> {
        let start = parser.current_position();
        let mut names = BTreeSet::new();
        while let Some(name) = parser.peek_char().and_then(ChannelName::from_char) {
            names.insert(name);
            parser.consume_chars(1);
        }
        if names.is_empty() {
            return Err(ParseError);
        }
        let end = parser.current_position();
        Ok(Self { start, names, end })
    }

    fn name() -> Option<fn() -> String> {
        Some(|| "channel name".to_owned())
    }
}

#[derive(Debug, Clone, Span, Parse)]
#[parse(name = "a space")]
struct Space(Either<Whitespace, Comment>);

#[derive(Debug, Clone)]
pub struct Channel {
    pub oscillator: Oscillator,
    pub commands: Vec<Command>,
}

impl Channel {
    fn new(oscillator: Oscillator) -> Self {
        Self {
            oscillator,
            commands: Vec::new(),
        }
    }
}
