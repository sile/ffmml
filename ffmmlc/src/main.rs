use clap::Parser;
use std::{
    io::Read,
    path::{Path, PathBuf},
    time::Duration,
};

/// FFMML compiler.
#[derive(Debug, Parser)]
#[command(version)]
struct Args {
    /// Input file path.
    #[clap(default_value = "-")]
    input_file: PathBuf,

    /// Output WAV file path.
    #[clap(short, long)]
    output_file: Option<PathBuf>,

    /// Sample rate.
    #[clap(long, default_value_t = 48000)]
    sample_rate: u16,

    /// Max duration (seconds).
    #[clap(long, default_value_t = 60)]
    duration: u16,
}

impl Args {
    fn read_input_file(&self) -> Result<String, String> {
        let mut mml = String::new();
        if self.input_file == Path::new("-") {
            std::io::stdin()
                .read_to_string(&mut mml)
                .map_err(|e| format!("failed to read MML text from STDIN ({e})"))?;
        } else {
            let mut file = std::fs::File::open(&self.input_file).map_err(|e| {
                format!(
                    "failed to open file: {} ({e})",
                    self.input_file.to_string_lossy()
                )
            })?;
            file.read_to_string(&mut mml).map_err(|e| {
                format!(
                    "failed to read MML text from {} ({e})",
                    self.input_file.to_string_lossy()
                )
            })?;
        }
        Ok(mml)
    }

    fn input_file_path(&self) -> PathBuf {
        if self.input_file == Path::new("-") {
            PathBuf::from("<STDIN>")
        } else {
            self.input_file.clone()
        }
    }

    fn create_output_writer(&self) -> Result<Box<dyn 'static + std::io::Write>, String> {
        let path = self.output_file_path();
        if path == Path::new("<STDOUT>") {
            Ok(Box::new(std::io::BufWriter::new(std::io::stdout())))
        } else {
            let file = std::fs::File::create(&path).map_err(|e| {
                format!(
                    "failed to create output file {} ({e})",
                    path.to_string_lossy()
                )
            })?;
            Ok(Box::new(std::io::BufWriter::new(file)))
        }
    }

    fn output_file_path(&self) -> PathBuf {
        if let Some(path) = &self.output_file {
            if path == Path::new("-") {
                PathBuf::from("<STDOUT>")
            } else {
                path.clone()
            }
        } else if self.input_file == Path::new("-") {
            PathBuf::from("<STDOUT>")
        } else {
            let mut path = self.input_file.clone();
            path.set_extension("wav");
            path
        }
    }
}

fn main() {
    let args = Args::parse();

    let result: Result<(), String> = (|| {
        // Read input.
        let mml = args.read_input_file()?;

        // Parse text.
        let music: ffmml::Music = mml
            .parse::<ffmml::Music>()
            .map_err(|e| e.file_path(args.input_file_path()).to_string())?;

        // Generate audio data.
        let wav = ffmml::wav::Wav::with_options(
            &music,
            ffmml::wav::WavOptions {
                sample_rate: args.sample_rate,
                max_duration: Duration::from_secs(u64::from(args.duration)),
            },
        )
        .map_err(|e| e.text(&mml).file_path(args.input_file_path()).to_string())?;

        // Write output.
        wav.to_writer(args.create_output_writer()?).map_err(|e| {
            format!(
                "failed to write WAV file to {} ({e})",
                args.output_file_path().to_string_lossy()
            )
        })?;

        Ok(())
    })();
    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
