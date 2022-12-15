//use rumml2wav::wav::Wav;
use std::io::Read;

fn main() {
    let result: Result<(), String> = (|| {
        let mut mml = String::new();
        std::io::stdin()
            .read_to_string(&mut mml)
            .map_err(|e| format!("failed to read MML text from STDIN ({e})"))?;
        let _music: ffmml::Music = mml.parse::<ffmml::Music>().map_err(|e| e.to_string())?;
        // let audio_data = music.play(44100).map(|x| x.to_i16()).collect::<Vec<_>>();
        // Wav::new(44100, audio_data)
        //     .to_writer(std::io::stdout())
        //     .map_err(|e| format!("failed to write WAVE file to STDOUT ({e})"))?;
        Ok(())
    })();
    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
