use std::time::{Duration, Instant};

use rodio::{DeviceSinkBuilder, MixerDeviceSink, Player, Source, source::SquareWave};

const MIN_BEEP_DURATION: Duration = Duration::from_millis(85);

pub(super) struct Beeper {
    audio: Option<(MixerDeviceSink, Player)>,
    sound_start: Option<Instant>,
}

impl Beeper {
    pub(super) fn new() -> Self {
        let audio = DeviceSinkBuilder::open_default_sink()
            .map(|handle| {
                let player = Player::connect_new(&handle.mixer());

                let source = SquareWave::new(440.0).repeat_infinite();
                player.append(source);
                player.set_volume(0.0);
                (handle, player)
            })
            .ok();

        if audio.is_none() {
            eprintln!("Warning: no audio device available, running without sound");
        }

        Self {
            audio,
            sound_start: None,
        }
    }

    pub(super) fn play(&mut self) {
        let Some((_, player)) = &self.audio else {
            return;
        };

        if self.sound_start.is_none() {
            self.sound_start = Some(Instant::now());
            player.set_volume(0.5);
        }
    }

    pub(super) fn pause(&mut self) {
        let Some((_, player)) = &self.audio else {
            return;
        };

        if let Some(start_time) = self.sound_start {
            if start_time.elapsed() >= MIN_BEEP_DURATION {
                player.set_volume(0.0);
                self.sound_start = None;
            }
        }
    }
}
