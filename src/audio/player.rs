use std::fs::File;
use rodio::{DeviceSinkBuilder, Decoder, Player};

pub struct MusicPlayer {
    _stream: rodio::MixerDeviceSink,
    player: Player
}

impl MusicPlayer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let stream = DeviceSinkBuilder::open_default_sink()?;
        let player = Player::connect_new(&stream.mixer());

        Ok(Self {
            _stream: stream,
            player,
        })
    }

    pub fn play_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let source = Decoder::try_from(file)?;

        self.player.append(source);

        Ok(())
    }

    pub fn pause(&self) {
        self.player.pause();
    }

    pub fn resume(&self) {
        self.player.play();
    }

    pub fn sleep_until_end(&self) {
        self.player.sleep_until_end();
    }
}
