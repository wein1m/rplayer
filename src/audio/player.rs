use std::{fs::File, time::Duration};
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

        self.player.pause();
        self.player.append(source);
        self.player.play();

        Ok(())
    }

    pub fn pause(&self) {
        if self.player.is_paused() {
            self.player.play();
        } else {
            self.player.pause();
        }
    }

    pub fn is_paused(&self) -> bool {
        self.player.is_paused()
    }

    pub fn seek(&self, sec: u64) {
        let time = Duration::from_secs(sec);

        match self.player.try_seek(time) {
            Ok(()) => {},
            Err(e) => eprintln!("Failed to seek: {e}")
        }
    }
}
