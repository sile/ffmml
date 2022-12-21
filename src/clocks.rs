use std::time::Duration;

use crate::types::{DefaultNoteDuration, NoteDuration, Quantize, QuantizeFrame, Tempo};
use num::rational::Ratio;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Clock(Ratio<u64>);

impl Clock {
    pub fn tick(&mut self, numer: u64, denom: u64) {
        self.0 += Ratio::new(numer, denom);
    }

    fn reverse_tick(&mut self, numer: u64, denom: u64) {
        self.0 -= Ratio::new(numer, denom);
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
    quantize_clock: Clock,
    sample_rate: u16,
    tempo: Tempo,
    quantize: QuantizeMode,
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
            quantize_clock: Clock::default(),
            sample_rate,
            tempo: Tempo::default(),
            quantize: QuantizeMode::default(),
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

    pub fn quantize_clock(&self) -> Clock {
        self.quantize_clock
    }

    pub fn sample_rate(&self) -> u16 {
        self.sample_rate
    }

    pub fn tick_sample_clock(&mut self) {
        self.sample_clock.tick(1, u64::from(self.sample_rate));
    }

    pub fn tick_note_clock(&mut self, note_duration: NoteDuration) {
        self.quantize_clock = self.note_clock;

        if let Some(tuplet) = &mut self.tuplet {
            tuplet.remainings -= 1;
            let is_last = tuplet.remainings == 0;
            let numer = *tuplet.duration.numer();
            let denom = *tuplet.duration.denom();
            self.note_clock.tick(numer, denom);
            self.tick_quantize_clock(numer, denom);
            if is_last {
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
            self.tick_quantize_clock(numer, denom);
            denom *= 2;
        }
    }

    fn tick_quantize_clock(&mut self, numer: u64, denom: u64) {
        match self.quantize {
            QuantizeMode::None => {
                self.quantize_clock.tick(numer, denom);
            }
            QuantizeMode::Normal(q) => {
                self.quantize_clock
                    .tick(numer * u64::from(q.get()), denom * 8);
            }
            QuantizeMode::Frame(q) => {
                self.quantize_clock.tick(numer, denom);
                self.quantize_clock.reverse_tick(u64::from(q.get()), 60);
            }
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

    pub fn set_quantize(&mut self, quantize: Quantize) {
        self.quantize = QuantizeMode::Normal(quantize);
    }

    pub fn set_quantize_frame(&mut self, quantize_frame: QuantizeFrame) {
        self.quantize = QuantizeMode::Frame(quantize_frame);
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

#[derive(Debug, Default)]
enum QuantizeMode {
    #[default]
    None,
    Normal(Quantize),
    Frame(QuantizeFrame),
}
