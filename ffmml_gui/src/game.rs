use ffmml::{Music, MusicPlayer};
use pagurus::{
    audio::AudioData,
    event::Event,
    failure::{Failure, OrFail},
    timeout::{TimeoutId, TimeoutTag},
    Game, Result, System,
};
use std::time::Duration;

#[cfg(target_arch = "wasm32")]
pagurus::export_wasm_functions!(FfmmlGame);

#[derive(Debug, Default)]
pub struct FfmmlGame {
    audio_data: AudioData,
    mml: String,
    music: Option<Music>,
    player: Option<MusicPlayer>,
    play_start_time: Duration,
    timeout: Option<TimeoutId>,
}

impl FfmmlGame {
    fn play_audio_data<S: System>(&mut self, system: &mut S) -> Result<()> {
        let Some(player) = &mut self.player else {
            return Ok(());
        };

        for i in 0..self.audio_data.samples().len() {
            self.audio_data
                .write_sample(i, player.next().unwrap_or_default().get());
        }
        system.audio_enqueue(self.audio_data.as_ref());
        if player.is_eos() {
            if let Some(e) = player.take_last_error() {
                return Err(e.text(&self.mml).file_path("TEXTAREA")).or_fail();
            }
            self.player = None;
            return Ok(());
        }

        let elapsed = system.clock_game_time() - self.play_start_time;
        let wait = player.elapsed().saturating_sub(elapsed);
        self.timeout = Some(system.clock_set_timeout(TimeoutTag::new(0), wait));
        Ok(())
    }
}

const SAMPLE_RATE: u16 = 48000;
const AUDIO_DATA_SAMPLES: usize = 960; // 20 ms

impl<S: System + 'static> Game<S> for FfmmlGame {
    fn initialize(&mut self, system: &mut S) -> Result<()> {
        self.audio_data = AudioData::new(system.audio_init(SAMPLE_RATE, AUDIO_DATA_SAMPLES));
        Ok(())
    }

    fn handle_event(&mut self, system: &mut S, event: Event) -> Result<bool> {
        if let Event::Timeout(t) = event {
            if Some(t.id) == self.timeout {
                self.play_audio_data(system).or_fail()?;
            }
        }
        Ok(true)
    }

    fn command(&mut self, system: &mut S, name: &str, data: &[u8]) -> Result<()> {
        match name {
            "parseScript" => {
                self.mml = std::str::from_utf8(data).or_fail()?.to_owned();
                self.music = Some(
                    self.mml
                        .parse::<Music>()
                        .map_err(|e| e.file_path("TEXTAREA"))
                        .or_fail()?,
                );
            }
            "playAudio" => {
                let music = self.music.as_ref().or_fail()?;
                self.player = Some(music.play(SAMPLE_RATE));
                self.play_start_time = system.clock_game_time();
                self.play_audio_data(system).or_fail()?;
            }
            _ => {
                return Err(Failure::new().message(format!("unknown command: {name:?}")));
            }
        }
        Ok(())
    }

    fn query(&mut self, _system: &mut S, name: &str) -> Result<Vec<u8>> {
        match name {
            "exportWav" => {
                let music = self.music.as_ref().or_fail()?;
                let wav = ffmml::wav::Wav::new(music).or_fail()?;
                let mut buf = Vec::new();
                wav.to_writer(&mut buf).or_fail()?;
                Ok(buf)
            }
            _ => Err(Failure::new().message(format!("unknown query: {name:?}"))),
        }
    }
}
