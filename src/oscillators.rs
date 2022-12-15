use crate::types::{Detune, Letter, Note, Octave, Sample, Timbre};

const MASTER_CLOCK_HZ: f32 = 21477272.7272;
const SYSTEM_CLOCK_HZ: f32 = MASTER_CLOCK_HZ / 12.0;

#[derive(Debug, Clone)]
pub enum Oscillator {
    PulseWave(PulseWave),
    // TriangleWave,
    // Noise,
}

impl Oscillator {
    pub fn pulse_wave() -> Self {
        Self::PulseWave(PulseWave::new())
    }

    pub fn sample(&mut self, sample_rate: u16) -> Sample {
        match self {
            Oscillator::PulseWave(o) => o.sample(sample_rate),
        }
    }

    pub fn set_frequency(&mut self, note: Note, octave: Octave, detune: Detune) {
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

const FREQUENCY_RATIO_TABLE: [f32; 12] = [
    1.000000, 1.059463, 1.122462, 1.189207, 1.259921, 1.334840, 1.414214, 1.498307, 1.587401,
    1.681793, 1.781797, 1.887749,
];

#[derive(Debug, Clone)]
pub struct PulseWave {
    frequency: f32,
    duty_cycle: f32,
    phase: f32,
}

impl PulseWave {
    fn new() -> Self {
        Self {
            frequency: 0.0, // dummy initial value
            duty_cycle: 0.125,
            phase: 0.0,
        }
    }

    fn sample(&mut self, sample_rate: u16) -> Sample {
        self.phase += self.frequency / f32::from(sample_rate);
        self.phase -= self.phase.floor();
        if self.phase > self.duty_cycle {
            Sample::MAX
        } else {
            Sample::MIN
        }
    }

    fn set_frequency(&mut self, note: Note, octave: Octave, detune: Detune) {
        let mut o = i32::from(octave.get());
        if !matches!(note.letter(), Letter::A | Letter::B) {
            o -= 1;
        }
        let ratio = FREQUENCY_RATIO_TABLE[note.offset_from_a()];
        let a = 27.5 * 2f32.powi(o);
        self.frequency = a * ratio;
        if detune.get() != 0 {
            let d = f32::from(detune.get());
            self.frequency = register_to_frequency(frequency_to_register(self.frequency) - d);
        }
    }

    fn set_timbre(&mut self, timbre: Timbre) -> bool {
        self.duty_cycle = match timbre.get() {
            Timbre::DUTY_CYCLE_12 => 0.125,
            Timbre::DUTY_CYCLE_25 => 0.250,
            Timbre::DUTY_CYCLE_50 => 0.500,
            Timbre::DUTY_CYCLE_75 => 0.750,
            _ => return false,
        };
        true
    }
}

fn frequency_to_register(frequency: f32) -> f32 {
    SYSTEM_CLOCK_HZ / frequency / 16.0
}

fn register_to_frequency(register: f32) -> f32 {
    SYSTEM_CLOCK_HZ / 16.0 / register
}
