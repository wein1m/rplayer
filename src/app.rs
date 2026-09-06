use std::path::PathBuf;

use crate::audio::MusicPlayer;
use crate::scanner::scanner::TrackAudio;
use crate::tui::TUI;
use crate::scanner;

pub struct App {
    pub music_player: MusicPlayer,
    music_path: PathBuf,
    pub songs: Vec<TrackAudio>,
    current_song: Option<usize>,
}

impl App {
    pub fn new(music_path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        Self::validate_path(&music_path)?;

        Ok(Self {
            music_player: MusicPlayer::new()?,
            music_path,
            songs: Vec::new(),
            current_song: Some(0)
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

    pub fn get_current_song(&self) -> Option<&TrackAudio>{
        self.current_song
            .and_then(|i| self.songs.get(i))
    }

    fn select_song(&mut self, id: usize) {
        self.current_song = Some(id)
    }

    pub fn next_song(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.select_song(
            self.current_song.unwrap() + 1
        );

        self.update_song()?;

        Ok(())
    }

    fn update_song(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.songs = scanner::scan_music(&self.music_path)?;

        let song_path = self.get_current_song()
            .map(|song| &song.path)
            .unwrap();

        self.music_player.play_file(song_path.as_str())?;

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.update_song()?;

        ratatui::run(|term| 
            TUI::new(self).run(term)
        )?;


        Ok(())
    }
}
