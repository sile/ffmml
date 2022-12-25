use clap::Parser;
use ffmmlc::wav::Wav;
use std::{
    io::Read,
    path::{Path, PathBuf},
};

#[derive(Debug, Parser)]
struct Args {
    #[clap(default_value = "-")]
    input_file: PathBuf,

    #[clap(short, long)]
    output_file: Option<PathBuf>,

    #[clap(long, default_value_t = 44100)]
    sample_rate: u16,
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
        let mut player = music.play(args.sample_rate);
        let audio_data = (&mut player).map(|x| x.to_i16()).collect::<Vec<_>>();
        if let Some(e) = player.last_error() {
            return Err(e.to_string(&mml, Some(&args.input_file_path().to_string_lossy())));
        }

        // Write output.
        Wav::new(u32::from(args.sample_rate), audio_data)
            .to_writer(args.create_output_writer()?)
            .map_err(|e| {
                format!(
                    "failed to write WAVE file to {} ({e})",
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
