use std::path::PathBuf;

use crate::audio::MusicPlayer;
use crate::scanner::scanner::TrackAudio;
use crate::tui::TUI;
use crate::scanner;

pub struct App {
    music_player: MusicPlayer,
    music_path: PathBuf,
    songs: Vec<TrackAudio>
}

impl App {
    pub fn new(music_path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        Self::validate_path(&music_path)?;

        Ok(Self {
            music_player: MusicPlayer::new()?,
            music_path,
            songs: Vec::new()
        })
    }

    fn validate_path(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        if !path.exists() {
            return Err(format!("Path is not exist: {}", path.display()).into());
        }

        if !path.is_dir() {
            return Err(format!("Path is not a directory: {}", path.display()).into());
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.songs = scanner::scan_music(&self.music_path)?;

        ratatui::run(|term| 
            TUI::new(&self.songs).run(term)
        );

        // self.music_player.play_file("assets/example.mp3")?;
        // self.music_player.sleep_until_end();

        Ok(())
    }
}
