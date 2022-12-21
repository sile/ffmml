use ffmmlc::wav::Wav;
use std::io::Read;

fn main() {
    let result: Result<(), String> = (|| {
        let mut mml = String::new();
        std::io::stdin()
            .read_to_string(&mut mml)
            .map_err(|e| format!("failed to read MML text from STDIN ({e})"))?;
        let music: ffmml::Music = mml.parse::<ffmml::Music>().map_err(|e| e.to_string())?;
        let mut player = music.play(44100);
        let audio_data = (&mut player).map(|x| x.to_i16()).collect::<Vec<_>>();
        if let Some(e) = player.last_error() {
            return Err(e.to_string(&mml, Some("<STDIN>")));
        }
        Wav::new(44100, audio_data)
            .to_writer(std::io::stdout())
            .map_err(|e| format!("failed to write WAVE file to STDOUT ({e})"))?;
        Ok(())
    })();
    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
