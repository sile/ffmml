use crate::types::{DefaultNoteDuration, NoteDuration, Tempo};
use num::rational::Ratio;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Clock(Ratio<u64>);

impl Clock {
    fn tick(&mut self, numer: u64, denom: u64) {
        self.0 += Ratio::new_raw(numer, denom);
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
        }
    }

    pub fn sample_clock(&self) -> Clock {
        self.sample_clock
    }

    pub fn note_clock(&self) -> Clock {
        self.note_clock
    }

    pub fn frame_clock(&self) -> Clock {
        self.frame_clock
    }

    pub fn sample_rate(&self) -> u16 {
        self.sample_rate
    }

    pub fn tick_sample_clock(&mut self) {
        self.sample_clock.tick(1, u64::from(self.sample_rate));
    }

    pub fn tick_note_clock(&mut self, note_duration: NoteDuration) {
        let unit = u64::from(self.tempo.get()) * 4; // four-four time
        let mut duration = u64::from(
            note_duration
                .get()
                .unwrap_or_else(|| self.default_note_duration.get()),
        );
        for _ in 0..=note_duration.dots() {
            self.note_clock.tick(unit, duration);
            duration *= 2;

            // safe guard for overflow
            // (the threshold value has no special meaning other than a somewhat large value)
            if duration > 0xFFFF {
                break;
            }
        }
    }

    pub fn tick_frame_clock(&mut self) {
        self.frame_clock.tick(1, 10); // 100 ms
    }

    pub fn set_frame_clock(&mut self, clock: Clock) {
        self.frame_clock = clock;
    }

    pub fn set_tempo(&mut self, tempo: Tempo) {
        self.tempo = tempo;
    }

    pub fn set_default_note_duration(&mut self, default: DefaultNoteDuration) {
        self.default_note_duration = default;
    }
}
