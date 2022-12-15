use std::ops::{Add, Mul};

use textparse::{
    components::{Char, Either, Maybe, While},
    Parse, ParseError, ParseResult, Parser, Position, Span,
};

#[derive(Debug, Clone, Copy)]
pub struct Sample(f32);

impl Sample {
    pub const MAX: Self = Sample(1.0);
    pub const MIN: Self = Sample(-1.0);
    pub const ZERO: Self = Sample(0.0);

    pub const fn new(v: f32) -> Self {
        Self(v)
    }

    pub fn get(self) -> f32 {
        self.0.max(Self::MIN.0).min(Self::MAX.0)
    }

    pub fn to_i16(self) -> i16 {
        let v = self.get();
        if v < 0.0 {
            (v.abs() * i16::MIN as f32) as i16
        } else {
            (v * i16::MAX as f32) as i16
        }
    }
}

impl Add<Sample> for Sample {
    type Output = Self;

    fn add(self, rhs: Sample) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Mul<f32> for Sample {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Timbre(u8);

impl Timbre {
    pub const DUTY_CYCLE_12: Self = Self(0);
    pub const DUTY_CYCLE_25: Self = Self(1);
    pub const DUTY_CYCLE_50: Self = Self(2);
    pub const DUTY_CYCLE_75: Self = Self(3);

    pub const fn new(n: u8) -> Self {
        Self(n)
    }

    pub const fn get(self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Letter {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl Letter {
    fn next(self) -> Self {
        match self {
            Letter::C => Letter::D,
            Letter::D => Letter::E,
            Letter::E => Letter::F,
            Letter::F => Letter::G,
            Letter::G => Letter::A,
            Letter::A => Letter::B,
            Letter::B => Letter::C,
        }
    }

    fn prev(self) -> Self {
        match self {
            Letter::C => Letter::B,
            Letter::D => Letter::C,
            Letter::E => Letter::D,
            Letter::F => Letter::E,
            Letter::G => Letter::F,
            Letter::A => Letter::G,
            Letter::B => Letter::A,
        }
    }

    fn from_char(c: char) -> Option<Self> {
        Some(match c {
            'c' => Self::C,
            'd' => Self::D,
            'e' => Self::E,
            'f' => Self::F,
            'g' => Self::G,
            'a' => Self::A,
            'b' => Self::B,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Span)]
pub struct Note {
    start: Position,
    letter: Letter,
    accidentals: i8,
    end: Position,
}

impl Note {
    pub const fn letter(self) -> Letter {
        self.letter
    }

    pub const fn accidentals(self) -> i8 {
        self.accidentals
    }

    pub fn normalize(mut self) -> (Letter, bool) {
        loop {
            if matches!(
                self.letter,
                Letter::C | Letter::D | Letter::F | Letter::G | Letter::A
            ) && self.accidentals >= 2
            {
                self.letter = self.letter.next();
                self.accidentals -= 2;
            } else if matches!(self.letter, Letter::E | Letter::B) && self.accidentals >= 1 {
                self.letter = self.letter.next();
                self.accidentals -= 1;
            } else if matches!(
                self.letter,
                Letter::D | Letter::E | Letter::G | Letter::A | Letter::B
            ) && self.accidentals <= -2
            {
                self.letter = self.letter.prev();
                self.accidentals += 2;
            } else if matches!(self.letter, Letter::C | Letter::F) && self.accidentals <= -1 {
                self.letter = self.letter.prev();
                self.accidentals += 1;
            }
            break;
        }

        if self.accidentals < 0 {
            self.letter = self.letter.prev();
            self.accidentals = 1;
        }

        assert!(self.accidentals == 0 || self.accidentals == 1);
        (self.letter, self.accidentals == 1)
    }
}

impl Parse for Note {
    fn parse(parser: &mut Parser) -> ParseResult<Self> {
        let start = parser.current_position();
        let letter = parser
            .read_char()
            .and_then(Letter::from_char)
            .ok_or(ParseError)?;
        let mut accidentals = 0;
        while let Ok(x) = parser.parse::<Either<Char<'+'>, Char<'-'>>>() {
            if matches!(x, Either::A(_)) {
                accidentals = (accidentals + 1) % 12;
            } else {
                accidentals = (accidentals - 1) % 12;
            }
        }
        let end = parser.current_position();
        Ok(Self {
            start,
            letter,
            accidentals,
            end,
        })
    }
}

#[derive(Debug, Clone, Copy, Span, Parse)]
pub struct DefaultNoteDuration(NonZeroU8);

impl DefaultNoteDuration {
    pub const fn get(self) -> u8 {
        self.0.get()
    }
}

impl Default for DefaultNoteDuration {
    fn default() -> Self {
        Self(NonZeroU8(U8::new(4)))
    }
}

#[derive(Debug, Clone, Copy, Span)]
pub struct NoteDuration {
    num: Maybe<NonZeroU8>,
    dots: While<Char<'.'>>,
}

impl NoteDuration {
    pub fn get(self) -> Option<u8> {
        self.num.get().map(|n| n.get())
    }

    pub fn dots(self) -> usize {
        self.dots.utf8_len()
    }
}

impl Parse for NoteDuration {
    fn parse(parser: &mut Parser) -> ParseResult<Self> {
        let start = parser.current_position();
        let num = parser.parse()?;
        let dots = parser.parse()?;
        Ok(Self { num, dots })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Span)]
struct NonZeroU8(U8);

impl NonZeroU8 {
    const fn get(self) -> u8 {
        self.0.value
    }
}

impl Parse for NonZeroU8 {
    fn parse(parser: &mut Parser) -> ParseResult<Self> {
        let n = parser.parse::<U8>()?;
        if n.value > 0 {
            Ok(Self(n))
        } else {
            Err(ParseError)
        }
    }

    fn name() -> Option<fn() -> String> {
        Some(|| "an integer between 1 and 255".to_owned())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Span)]
struct U8 {
    start: Position,
    value: u8,
    end: Position,
}

impl U8 {
    fn new(value: u8) -> Self {
        Self {
            start: Position::new(0),
            value,
            end: Position::new(0),
        }
    }
}

impl Parse for U8 {
    fn parse(parser: &mut Parser) -> ParseResult<Self> {
        let start = parser.current_position();
        let mut value = 0;
        while let Ok(d) = parser.parse::<Digit>() {
            value = d.value.checked_add(value).ok_or(ParseError)?;
        }
        let end = parser.current_position();
        if start == end {
            Err(ParseError)
        } else {
            Ok(Self { start, value, end })
        }
    }

    fn name() -> Option<fn() -> String> {
        Some(|| "an integer between 0 and 255".to_owned())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Span)]
struct Digit {
    start: Position,
    value: u8,
    end: Position,
}

impl Parse for Digit {
    fn parse(parser: &mut Parser) -> ParseResult<Self> {
        let c = parser.peek_char().ok_or(ParseError)?;
        if matches!(c, '0'..='9') {
            let (start, end) = parser.consume_chars(1);
            let value = c.to_digit(10).expect("unreachable") as u8;
            Ok(Self { start, value, end })
        } else {
            Err(ParseError)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Octave(u8);

impl Octave {
    pub const MIN: Self = Self(2);
    pub const MAX: Self = Self(7);

    pub fn new(octave: u8) -> Option<Self> {
        if (Self::MIN.0..=Self::MAX.0).contains(&octave) {
            Some(Self(octave))
        } else {
            None
        }
    }

    pub const fn get(self) -> u8 {
        self.0
    }
}

impl Default for Octave {
    fn default() -> Self {
        Self(4)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Detune(i8);

impl Detune {
    pub const fn new(detune: i8) -> Self {
        Self(detune)
    }

    pub const fn get(self) -> i8 {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tempo(NonZeroU8);

impl Tempo {
    pub const fn get(self) -> u8 {
        self.0.get()
    }
}

impl Default for Tempo {
    fn default() -> Self {
        Self(NonZeroU8(U8::new(120)))
    }
}
