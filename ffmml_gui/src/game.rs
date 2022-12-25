use ffmml::MusicPlayer;
use pagurus::{
    audio::AudioData,
    event::Event,
    failure::{Failure, OrFail},
    timeout::TimeoutTag,
    Game, Result, System,
};
use std::{str::FromStr, time::Duration};

const MML: &str = include_str!("../../examples/music01.mml");

pagurus::export_wasm_functions!(FfmmlGame);

#[derive(Debug, Default)]
pub struct FfmmlGame {
    audio_data: AudioData,
    player: Option<MusicPlayer>,
    start_time: Duration,
}

impl FfmmlGame {
    fn play_audio_frame<S: System>(&mut self, system: &mut S) -> Result<()> {
        if let Some(player) = &mut self.player {
            for i in 0..self.audio_data.samples().len() {
                self.audio_data
                    .write_sample(i, player.next().map_or(0.0, |s| s.get()));
            }
            system.audio_enqueue(self.audio_data.as_ref());
            if !player.is_eos() {
                if self.start_time == Duration::default() {
                    self.start_time = system.clock_game_time();
                }
                let elapsed = system.clock_game_time() - self.start_time;
                let wait = player.elapsed().saturating_sub(elapsed);
                system.clock_set_timeout(TimeoutTag::new(0), wait);
            }
        }
        if self.player.as_ref().map_or(false, |p| p.is_eos()) {
            self.player = None;
        }
        Ok(())
    }
}

const SAMPLE_RATE: u16 = 48000;
const DATA_SAMPLES: usize = 960; // 20 ms

impl<S: System + 'static> Game<S> for FfmmlGame {
    fn initialize(&mut self, system: &mut S) -> Result<()> {
        let music =
            ffmml::Music::from_str(MML).map_err(|e| Failure::new().message(e.to_string()))?;
        self.player = Some(music.play(SAMPLE_RATE));
        self.audio_data = AudioData::new(system.audio_init(SAMPLE_RATE, DATA_SAMPLES));

        log::info!("initialized");
        Ok(())
    }

    fn handle_event(&mut self, system: &mut S, event: Event) -> Result<bool> {
        if let Event::Timeout(_) = event {
            self.play_audio_frame(system).or_fail()?;
        }
        Ok(true)
    }

    fn command(&mut self, system: &mut S, _name: &str, _data: &[u8]) -> Result<()> {
        self.play_audio_frame(system).or_fail()?;
        Ok(())
    }
}
