use crate::{
    clocks::Clock,
    types::{Detune, Letter, Note, Octave, Sample, Timbre},
};

const MASTER_CLOCK_HZ: f32 = 21477272.7272;
const SYSTEM_CLOCK_HZ: f32 = MASTER_CLOCK_HZ / 12.0;

#[derive(Debug, Clone)]
pub enum Oscillator {
    PulseWave(PulseWave),
    TriangleWave(TriangleWave),
    // Noise,
}

impl Oscillator {
    pub fn pulse_wave() -> Self {
        Self::PulseWave(PulseWave::new())
    }

    pub fn triangle_wave() -> Self {
        Self::TriangleWave(TriangleWave::new())
    }

    pub fn sample(&mut self, sample_rate: u16, lfo: Option<&mut PitchLfo>) -> Sample {
        match self {
            Oscillator::PulseWave(o) => o.sample(sample_rate, lfo),
            Oscillator::TriangleWave(o) => o.sample(sample_rate, lfo),
        }
    }

    pub fn mute(&mut self, mute: bool) {
        match self {
            Oscillator::PulseWave(o) => o.mute(mute),
            Oscillator::TriangleWave(o) => o.mute(mute),
        }
    }

    pub fn set_frequency(&mut self, note: Note, octave: Octave, detune: Detune) {
        match self {
            Oscillator::PulseWave(o) => o.set_frequency(note, octave, detune),
            Oscillator::TriangleWave(o) => o.set_frequency(note, octave, detune),
        }
    }

    pub fn sweep_frequency(&mut self, depth: i8) {
        match self {
            Oscillator::PulseWave(o) => o.sweep_frequency(depth),
            Oscillator::TriangleWave(o) => o.sweep_frequency(depth),
        }
    }

    pub fn set_timbre(&mut self, timbre: Timbre) -> bool {
        match self {
            Oscillator::PulseWave(o) => o.set_timbre(timbre),
            Oscillator::TriangleWave(o) => o.set_timbre(timbre),
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

    fn sample(&mut self, sample_rate: u16, lfo: Option<&mut PitchLfo>) -> Sample {
        let frequency = if let Some(lfo) = lfo {
            let d = lfo.sample(sample_rate);
            register_to_frequency(frequency_to_register(self.frequency) - d)
        } else {
            self.frequency
        };
        self.phase += frequency / f32::from(sample_rate);
        self.phase -= self.phase.floor();
        if self.phase > self.duty_cycle {
            Sample::MAX
        } else {
            Sample::MIN
        }
    }

    fn mute(&mut self, mute: bool) {}

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

    fn sweep_frequency(&mut self, depth: i8) {
        let mut register = frequency_to_register(self.frequency);
        if depth >= 0 {
            register -= register / 2f32.powi(i32::from(depth));
        } else {
            register += register / 2f32.powi(i32::from(-depth));
        }
        self.frequency = register_to_frequency(register);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MuteState {
    Off { count: usize },
    On { count: usize },
}

#[derive(Debug, Clone)]
pub struct TriangleWave {
    frequency: f32,
    phase: f32,
    mute: MuteState,
}

impl TriangleWave {
    fn new() -> Self {
        Self {
            frequency: 0.0, // dummy initial value
            phase: 0.0,
            mute: MuteState::Off { count: 0 },
        }
    }

    fn mute(&mut self, mute: bool) {
        match self.mute {
            MuteState::Off { count } if mute => {
                self.mute = MuteState::On { count };
            }
            MuteState::On { count } if !mute => {
                self.mute = MuteState::Off { count };
            }
            _ => {}
        }
    }

    fn sample(&mut self, sample_rate: u16, lfo: Option<&mut PitchLfo>) -> Sample {
        const WAVEFORM: [f32; 32] = [
            1.0,
            0.8666667,
            0.73333335,
            0.6,
            0.46666667,
            0.33333334,
            0.2,
            0.06666667,
            -0.06666667,
            -0.2,
            -0.33333334,
            -0.46666667,
            -0.6,
            -0.73333335,
            -0.8666667,
            -1.0,
            -1.0,
            -0.8666667,
            -0.73333335,
            -0.6,
            -0.46666667,
            -0.33333334,
            -0.2,
            -0.06666667,
            0.06666667,
            0.2,
            0.33333334,
            0.46666667,
            0.6,
            0.73333335,
            0.8666667,
            1.0,
        ];
        const N: f32 = WAVEFORM.len() as f32;

        let frequency = if let Some(lfo) = lfo {
            let d = lfo.sample(sample_rate);
            register_to_frequency(frequency_to_register(self.frequency) - d)
        } else {
            self.frequency
        };

        const X: usize = 100;

        self.phase += frequency / f32::from(sample_rate);
        self.phase -= self.phase.floor();
        let i = (self.phase * N).floor() as usize;
        let s = Sample::new(WAVEFORM[i]);
        match self.mute {
            MuteState::On { count: 0 } => Sample::ZERO,
            MuteState::Off { count: X } => s,
            MuteState::On { count } => {
                self.mute = MuteState::On { count: count - 1 };
                s * (count as f32 / X as f32)
            }
            MuteState::Off { count } => {
                self.mute = MuteState::Off { count: count + 1 };
                s * (count as f32 / X as f32)
            }
        }
    }

    fn set_frequency(&mut self, note: Note, octave: Octave, detune: Detune) {
        let mut o = i32::from(octave.get());
        if !matches!(note.letter(), Letter::A | Letter::B) {
            o -= 1;
        }
        let ratio = FREQUENCY_RATIO_TABLE[note.offset_from_a()];
        let a = 27.5 * 2f32.powi(o - 1);
        self.frequency = a * ratio;
        if detune.get() != 0 {
            let d = f32::from(detune.get());
            self.frequency = register_to_frequency(frequency_to_register(self.frequency) - d);
        }
    }

    fn sweep_frequency(&mut self, depth: i8) {
        let mut register = frequency_to_register(self.frequency);
        if depth >= 0 {
            register -= register / 2f32.powi(i32::from(depth));
        } else {
            register += register / 2f32.powi(i32::from(-depth));
        }
        self.frequency = register_to_frequency(register);
    }

    fn set_timbre(&mut self, timbre: Timbre) -> bool {
        // TODO: Add a sentinel value indicating "unset"
        timbre.get() == 0
    }
}

#[derive(Debug)]
pub struct PitchLfo {
    now: Clock,
    start: Clock,
    sine_wave: SineWave,
    depth: u8,
}

impl PitchLfo {
    pub fn new(delay: u8, speed: u8, depth: u8) -> Self {
        let frequency = 20.0 / f32::from(speed);
        let mut start = Clock::default();
        start.tick(u64::from(delay), 60);
        Self {
            now: Clock::default(),
            start,
            sine_wave: SineWave::new(frequency),
            depth,
        }
    }

    pub fn sample(&mut self, sample_rate: u16) -> f32 {
        self.now.tick(1, u64::from(sample_rate));
        if self.now < self.start {
            0.0
        } else {
            f32::from(self.depth) * self.sine_wave.sample(sample_rate).get()
        }
    }

    pub fn reset_timer(&mut self) {
        self.now = Clock::default();
    }
}

#[derive(Debug)]
pub struct SineWave {
    frequency: f32,
    phase: f32,
}

impl SineWave {
    pub fn new(frequency: f32) -> Self {
        Self {
            frequency,
            phase: 0.0,
        }
    }

    pub fn sample(&mut self, sample_rate: u16) -> Sample {
        use std::f32::consts::PI;

        self.phase += self.frequency / f32::from(sample_rate);
        self.phase -= self.phase.floor();
        Sample::new((self.phase * 2.0 * PI).sin())
    }
}

fn frequency_to_register(frequency: f32) -> f32 {
    SYSTEM_CLOCK_HZ / frequency / 16.0
}

fn register_to_frequency(register: f32) -> f32 {
    SYSTEM_CLOCK_HZ / 16.0 / register
}
