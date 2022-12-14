use std::ops::{Add, Mul};

#[derive(Debug, Default, Clone)]
pub struct Credits {
    pub title: Option<String>,
    pub composer: Option<String>,
    pub programer: Option<String>,
}

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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Note {
    letter: Letter,
    accidentals: i8,
}

impl Note {
    pub const C: Self = Note::new(Letter::C);
    pub const D: Self = Note::new(Letter::D);
    pub const E: Self = Note::new(Letter::E);
    pub const F: Self = Note::new(Letter::F);
    pub const G: Self = Note::new(Letter::G);
    pub const A: Self = Note::new(Letter::A);
    pub const B: Self = Note::new(Letter::B);

    pub const fn new(letter: Letter) -> Self {
        Self {
            letter,
            accidentals: 0,
        }
    }

    pub const fn with_accidentals(letter: Letter, accidentals: i8) -> Self {
        Self {
            letter,
            accidentals: accidentals % 12,
        }
    }

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

    // TODO: private
    pub fn add_sharp(&mut self) {
        self.accidentals = (self.accidentals + 1) % 12;
    }

    pub fn add_flat(&mut self) {
        self.accidentals = (self.accidentals - 1) % 12;
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
