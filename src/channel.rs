use crate::{commands::Command, oscillators::Oscillator};
use std::collections::BTreeMap;
use textparse::{Parse, ParseError, ParseResult, Parser, Position, Span};

#[derive(Debug, Clone)]
pub struct Channels(BTreeMap<ChannelName, Channel>);

impl Channels {
    pub fn new() -> Self {
        let channels = [
            (ChannelName::A, Channel::new(Oscillator::pulse_wave())),
            (ChannelName::B, Channel::new(Oscillator::pulse_wave())),
            // (
            //     ChannelName::C,
            //     Channel::new(Oscillator::triangle_wave(44100)),
            // ),
            // (ChannelName::D, Channel::new(Oscillator::noise(44100))),
        ]
        .into_iter()
        .collect();
        Self(channels)
    }

    pub fn parse(&mut self, parser: &mut Parser) -> ParseResult<()> {
        let channel_name = parser.parse::<ParsableChannelName>()?.name;
        while !parser.is_eos() {}
        todo!()
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

// TODO: ChannelNames
#[derive(Debug, Clone, Span)]
struct ParsableChannelName {
    start: Position,
    name: ChannelName,
    end: Position,
}

impl Parse for ParsableChannelName {
    fn parse(parser: &mut Parser) -> ParseResult<Self> {
        let start = parser.current_position();
        let name = match parser.read_char().ok_or(ParseError)? {
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
            _ => return Err(ParseError),
        };
        let end = parser.current_position();
        Ok(Self { start, name, end })
    }

    fn name() -> Option<fn() -> String> {
        Some(|| "a channel name".to_owned())
    }
}

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
