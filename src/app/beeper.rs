use rodio::{DeviceSinkBuilder, MixerDeviceSink, Player, Source, source::SquareWave};

pub(super) struct Beeper {
    _handle: MixerDeviceSink,
    player: Player,
}

impl Beeper {
    pub(super) fn new() -> Self {
        let handle = DeviceSinkBuilder::open_default_sink().expect("Error opening default sink");

        let player = Player::connect_new(&handle.mixer());

        let source = SquareWave::new(440.0).repeat_infinite();
        player.append(source);

        Self {
            _handle: handle,
            player,
        }
    }

    pub(super) fn play(&self) {
        self.player.play();
    }

    pub(super) fn pause(&self) {
        self.player.pause();
    }
}
