use crate::app::App;
use crate::tui::ui;
use std::io::Result;
use std::time::{Duration, Instant};

use crossterm::event::{self, KeyCode};
use ratatui::widgets::TableState;
use ratatui::DefaultTerminal;


// #[derive(Debug, Default)]
// pub struct Song {
//     artist: String,
//     title: String,
//     duration: String
// }

pub struct TUI<'a> {
    pub state: TableState,
    pub app: &'a mut App,
    pub progress: u64,
    last_seek: Option<Instant>,
}

impl<'a> TUI<'a> {
    pub fn new(app: &'a mut App) -> Self {
        // let songs: Vec<TrackAudio> = vec![
        //     TrackAudio { artist: "RADWIMPS".into(), title: "Track One".into(), duration: "3:12".into() },
        //     TrackAudio { artist: "Dehumanizing Itatrain Worship".into(), title: "Track Two".into(), duration: "4:05".into() },
        //     TrackAudio { artist: "Awairo".into(), title: "Track Three".into(), duration: "2:59".into() },
        // ];

        Self{
            state: TableState::default().with_selected(0),
            app,
            progress: 0,
            last_seek: Some(Instant::now()),
        }
    }


    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_secs(1);
        let mut last_tick = Instant::now();

        loop {
            terminal.draw(|frame| ui::render(&mut self, frame))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if !event::poll(timeout)? {
                self.on_tick();
                last_tick = Instant::now();
                continue
            }
            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),

                    KeyCode::Char('l') => self.seek_forward(10),
                    KeyCode::Char('h') => self.seek_backward(10),

                    KeyCode::Char(' ') | KeyCode::Pause => self.app.music_player.pause(),

                    KeyCode::Char('L') => self.next_row(),
                    KeyCode::Char('H') => self.prev_row(),
                    _ => {}
                } 
            }
        }
    }

    fn next_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.app.songs.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));

        self.app.next_song().unwrap();
        self.progress = 0;
    }

    fn prev_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    i
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));

        self.app.prev_song(&mut self.progress).unwrap();
        self.progress = 0;
    }

    fn seek_forward(&mut self, sec: u64) {
        if self
            .last_seek
            .is_some_and(|last| last.elapsed() < Duration::from_millis(300))
        {
            return;
        };

        self.progress = (self.progress + sec).min(self.max_duration());
        self.app.music_player.seek(self.progress);

        self.last_seek = Some(Instant::now());
    }

    fn seek_backward(&mut self, sec: u64) {
        if self
            .last_seek
            .is_some_and(|last| last.elapsed() < Duration::from_millis(300))
        {
            return;
        };

        self.progress = self.progress.saturating_sub(sec);
        self.app.music_player.seek(self.progress);

        self.last_seek = Some(Instant::now());
    }

    fn on_tick(&mut self) {
        if !self.app.music_player.is_paused() {
            self.progress += 1;
        }

        if self.progress >= self.max_duration() {
            self.next_row();
        }
    }

    pub fn max_duration(&self) -> u64 {
        self.app.get_current_song()
            .map(|song| song.track_duration)
            .unwrap_or(0)
    }

}
