use byteorder::{LittleEndian, WriteBytesExt};
use std::io::Write;

#[derive(Debug)]
pub struct Wav {
    sample_rate: u32,
    samples: Vec<i16>,
}

impl Wav {
    pub const fn new(sample_rate: u32, samples: Vec<i16>) -> Self {
        Self {
            sample_rate,
            samples,
        }
    }

    fn chunk_len(&self) -> u32 {
        let mut written = WrittenBytes::default();
        self.write_fmt_chunk(&mut written).expect("unreachable");
        self.write_data_chunk(&mut written).expect("unreachable");
        written.len as u32
    }

    pub fn to_writer<W: Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(b"RIFF")?;
        writer.write_u32::<LittleEndian>(self.chunk_len())?;
        writer.write_all(b"WAVE")?;

        self.write_fmt_chunk(&mut writer)?;
        self.write_data_chunk(&mut writer)?;

        writer.flush()?;
        Ok(())
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
