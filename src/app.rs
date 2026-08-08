use crate::audio::MusicPlayer;

pub struct App {
    music_player: MusicPlayer
}

impl App {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            music_player: MusicPlayer::new()?,
        })
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.music_player.play_file("assets/example.mp3")?;
        self.music_player.sleep_until_end();

        Ok(())
    }
}
