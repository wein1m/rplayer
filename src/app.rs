use std::path::PathBuf;

use crate::audio::MusicPlayer;
use crate::scanner::scanner::TrackAudio;
use crate::tui::TUI;
use crate::scanner;
use crate::mpris::{Mpris, MprisCommand};

pub struct App {
    pub music_player: MusicPlayer,
    music_path: PathBuf,
    pub songs: Vec<TrackAudio>,
    current_song: Option<usize>,
    pub mpris: Mpris
}

impl App {
    pub fn new(music_path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        Self::validate_path(&music_path)?;

        Ok(Self {
            music_player: MusicPlayer::new()?,
            music_path,
            songs: Vec::new(),
            current_song: Some(0),
            mpris: Mpris::new()?,
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
        if self.current_song.unwrap() == self.songs.len() - 1 {
            self.select_song(0)
        } else {
            self.select_song(self.current_song.unwrap() + 1)
        }

        self.update_song()?;

        Ok(())
    }

    pub fn prev_song(&mut self, progress: &mut u64) -> Result<(), Box<dyn std::error::Error>> {
        if self.current_song.unwrap() == 0 {
            *progress = 0;
            self.music_player.seek(0);
            return Ok(())
        }

        self.select_song(
            self.current_song.unwrap() - 1);

        self.update_song()?;

        Ok(())
    }

    fn update_song(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.songs = scanner::scan_music(&self.music_path)?;

        if let Some(song) = self.get_current_song() {
            self.music_player.play_file(song.path.as_str())?;
            self.mpris.update_song(&song);
            self.mpris.set_playback_status(false);
        }

        Ok(())
    }

    pub fn toggle_pause(&mut self) {
        self.music_player.pause();
        self.mpris.set_playback_status(self.music_player.is_paused());
    }

    pub fn handle_mpris_commands(&mut self, progress: &mut u64) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(cmd) = self.mpris.poll_command() {
            match cmd {
                MprisCommand::Next => {
                    self.next_song()?;
                    *progress =0;
                }
                MprisCommand::Previous => {
                    self.prev_song(progress)?;
                }
                MprisCommand::PlayPause => {
                    self.toggle_pause();
                }
                MprisCommand::Play => {
                    if self.music_player.is_paused() {
                        self.toggle_pause();
                    }
                }
                MprisCommand::Pause => {
                    if !self.music_player.is_paused() {
                        self.toggle_pause();
                    }
                }
            }
        }
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
