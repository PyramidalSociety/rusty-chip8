use std::time::{Duration, Instant};

use rodio::{DeviceSinkBuilder, MixerDeviceSink, Player, Source, source::SquareWave};

const MIN_BEEP_DURATION: Duration = Duration::from_millis(75);

pub(super) struct Beeper {
    _handle: MixerDeviceSink,
    player: Player,
    sound_start: Option<Instant>,
}

impl Beeper {
    pub(super) fn new() -> Self {
        let handle = DeviceSinkBuilder::open_default_sink().expect("Error opening default sink");

        let player = Player::connect_new(&handle.mixer());

        let source = SquareWave::new(440.0).repeat_infinite();
        player.append(source);
        player.set_volume(0.0);

        Self {
            _handle: handle,
            player,
            sound_start: None,
        }
    }

    pub(super) fn play(&mut self) {
        if self.sound_start.is_none() {
            self.sound_start = Some(Instant::now());
            self.player.set_volume(0.5);
        }
    }

    pub(super) fn pause(&mut self) {
        if let Some(start_time) = self.sound_start {
            if start_time.elapsed() >= MIN_BEEP_DURATION {
                self.player.set_volume(0.0);
                self.sound_start = None;
            }
        }
    }
}
