use crate::types::{Detune, Note, Octave, Sample, Timbre};

#[derive(Debug, Clone)]
pub enum Oscillator {
    PulseWave(PulseWave),
    // TriangleWave,
    // Noise,
}

impl Oscillator {
    pub fn pulse_wave(sample_rate: u16) -> Self {
        Self::PulseWave(PulseWave::new(sample_rate))
    }

    pub fn sample(&mut self) -> Sample {
        match self {
            Oscillator::PulseWave(o) => o.sample(),
        }
    }

    pub fn set_frequency(&mut self, note: Note, octave: Octave, detune: Detune) -> bool {
        match self {
            Oscillator::PulseWave(o) => o.set_frequency(note, octave, detune),
        }
    }

    pub fn set_timbre(&mut self, timbre: Timbre) -> bool {
        match self {
            Oscillator::PulseWave(o) => o.set_timbre(timbre),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PulseWave {
    sample_rate: f32,
    frequency: f32,
    duty_cycle: f32,
    phase: f32,
}

impl PulseWave {
    fn new(sample_rate: u16) -> Self {
        Self {
            frequency: 0.0, // dummy initial value
            duty_cycle: 0.5,
            sample_rate: f32::from(sample_rate),
            phase: 0.0,
        }
    }

    fn sample(&mut self) -> Sample {
        self.phase += self.frequency / self.sample_rate;
        self.phase -= self.phase.floor();
        if self.phase > self.duty_cycle {
            Sample::MAX
        } else {
            Sample::MIN
        }
    }

    fn set_frequency(&mut self, note: Note, octave: Octave, detune: Detune) -> bool {
        // TODO: handle detune
        // https://wikiwiki.jp/mck/%E5%91%A8%E6%B3%A2%E6%95%B0%E3%81%A8%E3%83%AC%E3%82%B8%E3%82%B9%E3%82%BF%E3%81%AE%E9%96%A2%E4%BF%82
        todo!()
    }

    fn set_timbre(&mut self, timbre: Timbre) -> bool {
        self.duty_cycle = match timbre {
            Timbre::DUTY_CYCLE_12 => 0.125,
            Timbre::DUTY_CYCLE_25 => 0.250,
            Timbre::DUTY_CYCLE_50 => 0.500,
            Timbre::DUTY_CYCLE_75 => 0.750,
            _ => return false,
        };
        true
    }
}
