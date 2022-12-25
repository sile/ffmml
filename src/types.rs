use std::ops::{Add, Div, Mul};
use textparse::{
    components::{Char, Digit, Either, Maybe, While},
    Parse, Parser, Position, Span,
};

use crate::{comment::CommentsOrWhitespaces, traits::NthFrameItem};

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

impl Div<f32> for Sample {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / rhs)
    }
}

#[derive(Debug, Default, Clone, Copy, Span, Parse)]
pub struct Timbre(Int<0, 3>);

impl Timbre {
    pub const DUTY_CYCLE_12: u8 = 0;
    pub const DUTY_CYCLE_25: u8 = 1;
    pub const DUTY_CYCLE_50: u8 = 2;
    pub const DUTY_CYCLE_75: u8 = 3;

    pub const NOISE_NORMAL: u8 = 0;
    pub const NOISE_LOOPED: u8 = 1;

    pub const fn get(self) -> u8 {
        self.0.get() as u8
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

    pub fn apply_note_number_delta(mut self, delta: i8) -> (Self, i8) {
        let mut octave_delta = delta / 12;
        let delta = delta % 12;

        let old_offset = self.offset_from_c();
        self.accidentals = self.accidentals.saturating_add(delta);
        let (letter, has_sharp) = self.normalize();
        self.letter = letter;
        self.accidentals = has_sharp as i8;
        let new_offset = self.offset_from_c();

        if delta > 0 && new_offset < old_offset {
            octave_delta += 1;
        } else if delta < 0 && new_offset > old_offset {
            octave_delta -= 1;
        }

        (self, octave_delta)
    }

    pub fn offset_from_a(self) -> usize {
        match self.normalize() {
            (Letter::A, false) => 0,
            (Letter::A, true) => 1,
            (Letter::B, _) => 2,
            (Letter::C, false) => 3,
            (Letter::C, true) => 4,
            (Letter::D, false) => 5,
            (Letter::D, true) => 6,
            (Letter::E, _) => 7,
            (Letter::F, false) => 8,
            (Letter::F, true) => 9,
            (Letter::G, false) => 10,
            (Letter::G, true) => 11,
        }
    }

    pub fn offset_from_c(self) -> usize {
        let offset = self.offset_from_a();
        if let Some(n) = offset.checked_sub(3) {
            n
        } else {
            offset + 9
        }
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
            } else {
                break;
            }
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
    fn parse(parser: &mut Parser) -> Option<Self> {
        let start = parser.current_position();
        let letter = parser.read_char().and_then(Letter::from_char)?;
        let mut accidentals = 0;
        while let Some(x) = parser.parse::<Either<Char<'+'>, Char<'-'>>>() {
            if matches!(x, Either::A(_)) {
                accidentals = (accidentals + 1) % 12;
            } else {
                accidentals = (accidentals - 1) % 12;
            }
        }
        let end = parser.current_position();
        Some(Self {
            start,
            letter,
            accidentals,
            end,
        })
    }
}

#[derive(Debug, Clone, Copy, Span, Parse)]
pub struct DefaultNoteDuration(Int<1, 255>);

impl DefaultNoteDuration {
    pub const fn get(self) -> u8 {
        self.0.get() as u8
    }
}

impl Default for DefaultNoteDuration {
    fn default() -> Self {
        Self(Int::new(4))
    }
}

#[derive(Debug, Clone, Copy, Span)]
pub struct NoteDuration {
    num: Maybe<Int<1, 255>>,
    dots: While<Char<'.'>>,
}

impl NoteDuration {
    pub fn get(self) -> Option<u8> {
        self.num.get().map(|n| n.get() as u8)
    }

    pub fn dots(self) -> usize {
        self.dots.len()
    }
}

impl Parse for NoteDuration {
    fn parse(parser: &mut Parser) -> Option<Self> {
        let num = parser.parse()?;
        let dots = parser.parse()?;
        Some(Self { num, dots })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Span)]
pub struct Int<const MIN: i32, const MAX: i32> {
    start: Position,
    value: i32,
    end: Position,
}

impl<const MIN: i32, const MAX: i32> Int<MIN, MAX> {
    pub const fn new(value: i32) -> Self {
        Self {
            start: Position::new(0),
            value,
            end: Position::new(0),
        }
    }

    pub const fn get(self) -> i32 {
        self.value
    }
}

impl<const MIN: i32, const MAX: i32> Parse for Int<MIN, MAX> {
    fn parse(parser: &mut Parser) -> Option<Self> {
        let start = parser.current_position();
        let mut value: i32 = 0;
        let minus = MIN < 0 && parser.parse::<Char<'-'>>().is_some();
        while let Some(d) = parser.parse::<Digit>() {
            let d = i32::from(d.get());
            value = value.checked_mul(10).and_then(|v| v.checked_add(d))?;
        }
        let end = parser.current_position();
        if minus {
            value = -value;
        }
        if start == end || !(MIN..=MAX).contains(&value) {
            None
        } else {
            Some(Self { start, value, end })
        }
    }

    fn name() -> Option<fn() -> String> {
        Some(|| format!("an integer between {MIN} and {MAX}"))
    }
}

#[derive(Debug, Clone, Copy, Span, Parse)]
pub struct Octave(Int<2, 7>);

impl Octave {
    pub const MIN: Self = Self(Int::new(2));
    pub const MAX: Self = Self(Int::new(7));

    pub const fn get(self) -> u8 {
        self.0.get() as u8
    }

    pub fn checked_add(self, n: u8) -> Option<Self> {
        if self.get().saturating_add(n) <= 7 {
            Some(Self(Int::new(i32::from(self.get() + n))))
        } else {
            None
        }
    }

    pub fn checked_sub(self, n: u8) -> Option<Self> {
        if n <= self.get() - 2 {
            Some(Self(Int::new(i32::from(self.get() - n))))
        } else {
            None
        }
    }
}

impl Default for Octave {
    fn default() -> Self {
        Self(Int::new(4))
    }
}

#[derive(Debug, Clone, Copy, Span, Parse)]
pub struct Quantize(Int<1, 8>);

impl Quantize {
    pub const fn get(self) -> u8 {
        self.0.get() as u8
    }
}

impl Default for Quantize {
    fn default() -> Self {
        Self(Int::new(8))
    }
}

#[derive(Debug, Default, Clone, Copy, Span, Parse)]
pub struct QuantizeFrame(Int<0, 255>);

impl QuantizeFrame {
    pub const fn get(self) -> u8 {
        self.0.get() as u8
    }
}

#[derive(Debug, Default, Clone, Copy, Span, Parse)]
pub struct Detune(Int<-128, 127>);

impl Detune {
    pub const fn get(self) -> i8 {
        self.0.get() as i8
    }
}

#[derive(Debug, Clone, Copy, Span, Parse)]
pub struct Tempo(Int<1, 255>);

impl Tempo {
    pub const fn get(self) -> u8 {
        self.0.get() as u8
    }
}

impl Default for Tempo {
    fn default() -> Self {
        Self(Int::new(120))
    }
}

#[derive(Debug, Clone, Copy, Span, Parse)]
pub struct Volume(Int<0, 15>);

impl Volume {
    pub const fn get(self) -> u8 {
        self.0.get() as u8
    }

    pub fn checked_add(self, n: u8) -> Option<Self> {
        if self.get().saturating_add(n) <= 15 {
            Some(Self(Int::new(i32::from(self.get() + n))))
        } else {
            None
        }
    }

    pub fn checked_sub(self, n: u8) -> Option<Self> {
        self.get()
            .checked_sub(n)
            .map(|n| Self(Int::new(i32::from(n))))
    }

    pub fn as_ratio(self) -> f32 {
        f32::from(self.get()) / 15.0
    }
}

impl Default for Volume {
    fn default() -> Self {
        Self(Int::new(15))
    }
}

#[derive(Debug, Clone, Span)]
struct LoopList<T> {
    start: Position,
    items: Vec<T>,
    loop_point: Option<usize>,
    end: Position,
}

impl<T> LoopList<T> {
    fn constant(item: T) -> Self {
        Self {
            start: Position::new(0),
            items: vec![item],
            loop_point: None,
            end: Position::new(0),
        }
    }
}

impl<T: Parse> Parse for LoopList<T> {
    fn parse(parser: &mut Parser) -> Option<Self> {
        let start = parser.current_position();
        let _: Char<'{'> = parser.parse()?;

        let mut loop_point = None;
        let mut items = Vec::new();
        loop {
            let _: CommentsOrWhitespaces = parser.parse()?;
            if parser.parse::<Char<'|'>>().is_some() {
                loop_point = Some(items.len());
                let _: CommentsOrWhitespaces = parser.parse()?;
            }

            items.push(parser.parse::<T>()?);

            let _: CommentsOrWhitespaces = parser.parse()?;
            if parser.parse::<Char<','>>().is_some() {
                continue;
            }

            if parser.parse::<Char<'}'>>().is_some() {
                break;
            }
        }
        let end = parser.current_position();
        Some(Self {
            start,
            items,
            loop_point,
            end,
        })
    }
}

impl<T: Copy> NthFrameItem for LoopList<T> {
    type Item = T;

    fn nth_frame_item(&self, frame_index: usize) -> Self::Item {
        if let Some(item) = self.items.get(frame_index).copied() {
            item
        } else {
            let loop_point = self.loop_point.unwrap_or_else(|| self.items.len() - 1);
            let i = frame_index - self.items.len();
            let i = (i % (self.items.len() - loop_point)) + loop_point;
            self.items[i]
        }
    }
}

#[derive(Debug, Clone, Span, Parse)]
pub struct VolumeEnvelope {
    envelope: LoopList<Volume>,
}

impl VolumeEnvelope {
    pub fn constant(volume: Volume) -> Self {
        Self {
            envelope: LoopList::constant(volume),
        }
    }

    pub fn is_constant(&self) -> bool {
        self.envelope.items.len() == 1
    }
}

impl NthFrameItem for VolumeEnvelope {
    type Item = Volume;

    fn nth_frame_item(&self, frame_index: usize) -> Self::Item {
        self.envelope.nth_frame_item(frame_index)
    }
}

#[derive(Debug, Clone, Span, Parse)]
pub struct PitchEnvelope {
    envelope: LoopList<Detune>,
}

impl PitchEnvelope {
    pub fn constant(detune: Detune) -> Self {
        Self {
            envelope: LoopList::constant(detune),
        }
    }
}

impl NthFrameItem for PitchEnvelope {
    type Item = Detune;

    fn nth_frame_item(&self, frame_index: usize) -> Self::Item {
        self.envelope.nth_frame_item(frame_index)
    }
}

#[derive(Debug, Clone, Span, Parse)]
pub struct NoteEnvelope {
    envelope: LoopList<Int<-128, 127>>,
}

impl NthFrameItem for NoteEnvelope {
    type Item = i8;

    fn nth_frame_item(&self, frame_index: usize) -> Self::Item {
        let mut v: i8 = 0;
        for i in 0..frame_index {
            if i < self.envelope.items.len() || self.envelope.loop_point.is_some() {
                v = v.saturating_add(self.envelope.nth_frame_item(frame_index).get() as i8);
            }
        }
        v
    }
}

#[derive(Debug, Clone, Span, Parse)]
pub struct Timbres {
    list: LoopList<Timbre>,
}

impl Timbres {
    pub fn constant(timbre: Timbre) -> Self {
        Self {
            list: LoopList::constant(timbre),
        }
    }
}

impl NthFrameItem for Timbres {
    type Item = Timbre;

    fn nth_frame_item(&self, frame_index: usize) -> Self::Item {
        self.list.nth_frame_item(frame_index)
    }
}

#[derive(Debug, Clone, Span, Parse)]
pub struct Vibrato {
    _open: Char<'{'>,
    _space0: CommentsOrWhitespaces,
    delay: Int<0, 255>,
    _space1: CommentsOrWhitespaces,
    speed: Int<1, 255>,
    _space2: CommentsOrWhitespaces,
    depth: Int<0, 255>,
    _space3: CommentsOrWhitespaces,
    _close: Char<'}'>,
}

impl Vibrato {
    pub fn delay(&self) -> u8 {
        self.delay.get() as u8
    }

    pub fn speed(&self) -> u8 {
        self.speed.get() as u8
    }

    pub fn depth(&self) -> u8 {
        self.depth.get() as u8
    }
}

#[derive(Debug, Clone, Copy, Span, Parse)]
pub struct PitchSweep {
    speed: Int<0, 15>,
    _space0: CommentsOrWhitespaces,
    _comma: Char<','>,
    _space1: CommentsOrWhitespaces,
    depth: Int<0, 15>,
}

impl PitchSweep {
    pub fn speed(self) -> Option<u8> {
        let v = self.speed.get() as u8;
        if v < 8 {
            None
        } else {
            Some(v - 7)
        }
    }

    pub fn depth(self) -> Option<i8> {
        let v = self.depth.get() as i8;
        if 8 < v {
            Some(v - 8)
        } else if 0 < v && v < 8 {
            Some(-v)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, Span, Parse)]
pub struct OscillatorKind(Int<0, 2>);

impl OscillatorKind {
    pub const PULSE_WAVE: u8 = 0;
    pub const TRIANGLE_WAVE: u8 = 1;
    pub const NOISE: u8 = 2;

    pub const fn get(self) -> u8 {
        self.0.get() as u8
    }
}
