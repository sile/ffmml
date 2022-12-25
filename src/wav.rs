//! WAV: RIFF waveform Audio Format.
use crate::{Music, PlayMusicError};
use byteorder::{LittleEndian, WriteBytesExt};
use std::{io::Write, time::Duration};

/// [`Wav`] options.
#[derive(Debug, Clone)]
pub struct WavOptions {
    /// Sample rate.
    ///
    /// The default value is `48000`.
    pub sample_rate: u16,

    /// Maximum duration.
    ///
    /// If the length of the target music exceeds this limit,
    /// the exceeded part will exclude from the output WAV file.
    ///
    /// The default value is `Duration::from_secs(60)`.
    pub max_duration: Duration,
}

impl WavOptions {
    fn max_samples(&self) -> usize {
        (self.max_duration.as_secs_f32() * f32::from(self.sample_rate)).floor() as usize
    }
}

impl Default for WavOptions {
    fn default() -> Self {
        Self {
            sample_rate: 48000,
            max_duration: Duration::from_secs(60),
        }
    }
}

/// [`Wav`] provides a feature to export audio data as WAV format.
#[derive(Debug)]
pub struct Wav {
    sample_rate: u32,
    samples: Vec<i16>,
}

impl Wav {
    /// Makes a [`Wav`] instance with the default options.
    pub fn new(music: &Music) -> Result<Self, PlayMusicError> {
        Self::with_options(music, Default::default())
    }

    /// Makes a [`Wav`] instance.
    pub fn with_options(music: &Music, options: WavOptions) -> Result<Self, PlayMusicError> {
        let mut player = music.play(options.sample_rate);
        let samples = (&mut player)
            .take(options.max_samples())
            .map(|s| s.to_i16())
            .collect::<Vec<_>>();
        if let Some(e) = player.take_last_error() {
            return Err(e);
        }
        Ok(Self {
            sample_rate: u32::from(options.sample_rate),
            samples,
        })
    }

    /// Exports this music (audio samples) as WAV into the writer.
    pub fn to_writer<W: Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(b"RIFF")?;
        writer.write_u32::<LittleEndian>(self.chunk_len())?;
        writer.write_all(b"WAVE")?;

        self.write_fmt_chunk(&mut writer)?;
        self.write_data_chunk(&mut writer)?;

        writer.flush()?;
        Ok(())
    }

    fn chunk_len(&self) -> u32 {
        let mut written = WrittenBytes::default();
        self.write_fmt_chunk(&mut written).expect("unreachable");
        self.write_data_chunk(&mut written).expect("unreachable");
        written.len as u32
    }

    fn write_fmt_chunk<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(b"fmt ")?;
        writer.write_u32::<LittleEndian>(16)?; // Chunk Size
        writer.write_u16::<LittleEndian>(1)?; // Format: 1=Linear PCM
        writer.write_u16::<LittleEndian>(1)?; // Channels
        writer.write_u32::<LittleEndian>(self.sample_rate)?;
        writer.write_u32::<LittleEndian>(self.sample_rate * 2)?; // Bytes per Second
        writer.write_u16::<LittleEndian>(2)?; // Block Size
        writer.write_u16::<LittleEndian>(16)?; // Bits per Sample
        Ok(())
    }

    fn write_data_chunk<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(b"data")?;
        writer.write_u32::<LittleEndian>((self.samples.len() * 2) as u32)?;
        for &sample in &self.samples {
            writer.write_i16::<LittleEndian>(sample)?;
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
struct WrittenBytes {
    len: usize,
}

impl Write for WrittenBytes {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.len += buf.len();
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
