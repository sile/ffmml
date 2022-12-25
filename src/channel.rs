use crate::{
    commands::Command,
    comment::{Comment, CommentsOrWhitespaces},
    oscillators::Oscillator,
    ParseMusicError,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};
use textparse::{
    components::{Either, Whitespace},
    Parse, Parser, Position, Span,
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

    pub fn add_channel(&mut self, name: ChannelName, oscillator: Oscillator) {
        self.0.insert(name, Channel::new(oscillator));
    }

    pub fn parse(&mut self, parser: &mut Parser) -> Option<Result<(), ParseMusicError>> {
        let mut channels: BTreeMap<_, Vec<Command>> =
            self.0.keys().copied().map(|k| (k, Vec::new())).collect();
        while !parser.is_eos() {
            let names = parser.parse::<ChannelNames>()?;
            if let Some(i) = names.names.iter().position(|n| !channels.contains_key(n)) {
                let error_position = Position::new(names.start_position().get() + i);
                return Some(Err(ParseMusicError::new(
                    error_position,
                    "undefined channel",
                )));
            }
            let names = names.names;

            let _: Space = parser.parse()?;
            let _: CommentsOrWhitespaces = parser.parse()?;

            let mut has_space = false;
            while let Some(command) = parser.parse::<Command>() {
                for name in &names {
                    channels
                        .get_mut(name)
                        .expect("unreachable")
                        .push(command.clone());
                }

                let space: CommentsOrWhitespaces = parser.parse()?;
                has_space = !space.is_empty();
            }

            if !has_space && !parser.is_eos() {
                // Needs spaces before channel names.
                return None;
            }
        }
        for (key, channel) in &mut self.0 {
            channel.commands = Arc::new(channels.remove(&key).expect("unreachable"));
        }
        Some(Ok(()))
    }

    pub fn iter(&self) -> impl '_ + Iterator<Item = (ChannelName, Channel)> {
        self.0.iter().map(|(k, v)| (*k, v.clone()))
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
pub struct ChannelNames {
    start: Position,
    names: BTreeSet<ChannelName>,
    end: Position,
}

impl ChannelNames {
    pub fn names(&self) -> &BTreeSet<ChannelName> {
        &self.names
    }
}

impl Parse for ChannelNames {
    fn parse(parser: &mut Parser) -> Option<Self> {
        let start = parser.current_position();
        let mut names = BTreeSet::new();
        while let Some(name) = parser.peek_char().and_then(ChannelName::from_char) {
            names.insert(name);
            parser.read_char();
        }
        if names.is_empty() {
            return None;
        }
        let end = parser.current_position();
        Some(Self { start, names, end })
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
    pub commands: Arc<Vec<Command>>,
}

impl Channel {
    fn new(oscillator: Oscillator) -> Self {
        Self {
            oscillator,
            commands: Arc::new(Vec::new()),
        }
    }
}
