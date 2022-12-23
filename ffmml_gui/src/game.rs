use byteorder::{BigEndian, WriteBytesExt};
use ffmml::MusicPlayer;
use pagurus::{
    audio::AudioData,
    event::Event,
    failure::{Failure, OrFail},
    ActionId, Game, Result, System,
};
use pagurus_game_std::logger::Logger;
use std::{str::FromStr, time::Duration};

const MML: &str = include_str!("../../examples/music01.mml");

pagurus_game_std::export_wasm_functions!(FfmmlGame);

#[derive(Debug, Default)]
pub struct FfmmlGame {
    player: Option<MusicPlayer>,
    start_time: Duration,
    timeout: Option<ActionId>,
}

impl FfmmlGame {
    fn play_audio_frame<S: System>(&mut self, system: &mut S) -> Result<()> {
        let mut eos = false;
        if let Some(player) = &mut self.player {
            let now = player.current_position();
            let mut data = Vec::new();
            let frame_size = Duration::from_millis(20);
            while player.current_position() - now < frame_size {
                let Some(sample) = player.next() else {
                    eos = true;
                    break;
                };
                (&mut data)
                    .write_i16::<BigEndian>(sample.to_i16())
                    .or_fail()?;
            }
            let size = system.audio_enqueue(AudioData::new(&data).or_fail()?);
            (size == data.len() / 2).or_fail()?;
            if !eos {
                if self.timeout.is_none() {
                    self.start_time = system.clock_game_time();
                }
                let elapsed = system.clock_game_time() - self.start_time;
                let wait = player.current_position().saturating_sub(elapsed);
                self.timeout = Some(system.clock_set_timeout(wait));
            }
        }
        if eos {
            self.player = None;
        }
        Ok(())
    }
}

impl<S: System + 'static> Game<S> for FfmmlGame {
    fn initialize(&mut self, _system: &mut S) -> Result<()> {
        Logger::<S>::init(log::Level::Debug).or_fail()?;
        let music =
            ffmml::Music::from_str(MML).map_err(|e| Failure::new().message(e.to_string()))?;
        self.player = Some(music.play(AudioData::SAMPLE_RATE as u16));

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
