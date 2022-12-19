use std::time::Duration;

use crate::types::{DefaultNoteDuration, NoteDuration, Tempo};
use num::rational::Ratio;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Clock(Ratio<u64>);

impl Clock {
    fn tick(&mut self, numer: u64, denom: u64) {
        self.0 += Ratio::new(numer, denom);
    }

    pub fn now(self) -> Duration {
        if let Ok(denom) = u32::try_from(*self.0.denom()) {
            Duration::from_secs(*self.0.numer()) / denom
        } else {
            Duration::from_secs_f64(*self.0.numer() as f64 / *self.0.denom() as f64)
        }
    }
}

#[derive(Debug)]
pub struct Clocks {
    sample_clock: Clock,
    note_clock: Clock,
    frame_clock: Clock,
    sample_rate: u16,
    tempo: Tempo,
    default_note_duration: DefaultNoteDuration,
    tuplet: Option<Tuplet>,
    frame_index: usize,
}

impl Clocks {
    pub fn new(sample_rate: u16) -> Self {
        Self {
            sample_clock: Clock::default(),
            note_clock: Clock::default(),
            frame_clock: Clock::default(),
            sample_rate,
            tempo: Tempo::default(),
            default_note_duration: DefaultNoteDuration::default(),
            tuplet: None,
            frame_index: 0,
        }
    }

    pub fn sample_clock(&self) -> Clock {
        self.sample_clock
    }

    pub fn note_clock(&self) -> Clock {
        self.note_clock
    }

    pub fn sample_rate(&self) -> u16 {
        self.sample_rate
    }

    pub fn tick_sample_clock(&mut self) {
        self.sample_clock.tick(1, u64::from(self.sample_rate));
    }

    pub fn tick_note_clock(&mut self, note_duration: NoteDuration) {
        if let Some(tuplet) = &mut self.tuplet {
            tuplet.remainings -= 1;
            self.note_clock
                .tick(*tuplet.duration.numer(), *tuplet.duration.denom());
            if tuplet.remainings == 0 {
                self.tuplet = None;
            }
            return;
        }

        let duration = note_duration
            .get()
            .unwrap_or_else(|| self.default_note_duration.get());

        let numer = 60 /* a minute */ * 4 /* four-four- time*/;
        let mut denom = u64::from(self.tempo.get()) * u64::from(duration);
        for _ in 0..=std::cmp::min(note_duration.dots(), 16) {
            self.note_clock.tick(numer, denom);
            denom *= 2;
        }
    }

    pub fn tick_frame_clock_if_need(&mut self) -> bool {
        let mut next_frame = self.frame_clock;
        next_frame.tick(1, 60);

        if self.sample_clock < next_frame {
            false
        } else {
            self.frame_clock = next_frame;
            self.frame_index += 1;
            true
        }
    }

    pub fn reset_frame_clock(&mut self, clock: Clock) {
        self.frame_index = 0;
        self.frame_clock = clock;
    }

    pub fn frame_index(&self) -> usize {
        self.frame_index
    }

    pub fn set_tempo(&mut self, tempo: Tempo) {
        self.tempo = tempo;
    }

    pub fn set_default_note_duration(&mut self, default: DefaultNoteDuration) {
        self.default_note_duration = default;
    }

    pub fn set_tuplet(&mut self, note_count: usize, note_duration: NoteDuration) {
        if note_count == 0 {
            return;
        }

        let clock = std::mem::take(&mut self.note_clock);
        self.tick_note_clock(note_duration);
        let duration = std::mem::replace(&mut self.note_clock, clock).0;
        self.tuplet = Some(Tuplet {
            remainings: note_count,
            duration: duration / note_count as u64,
        });
    }
}

#[derive(Debug)]
struct Tuplet {
    remainings: usize,
    duration: Ratio<u64>,
}
