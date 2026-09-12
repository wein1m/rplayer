use crate::app::App;
use crate::tui::Colors;
use std::io::Result;
use std::time::{Duration, Instant};

use crossterm::event::{self, KeyCode};
use ratatui::layout::{Alignment, Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, BorderType, Borders, Cell, Gauge, Padding, Paragraph, Row, Table, TableState};
use ratatui::{DefaultTerminal, Frame};


// #[derive(Debug, Default)]
// pub struct Song {
//     artist: String,
//     title: String,
//     duration: String
// }

pub struct TUI<'a> {
    state: TableState,
    app: &'a mut App,
    progress: u64,
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
            self.progress = 0;
        }
    }

    fn max_duration(&self) -> u64 {
        self.app.get_current_song()
            .map(|song| song.track_duration)
            .unwrap_or(0)
    }

    fn format_time(secs: u64) -> String {
        let min = secs / 60;
        let sec = secs % 60;

        format!("{:02}:{:02}", min, sec)
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_secs(1);
        let mut last_tick = Instant::now();

        loop {
            terminal.draw(|frame| self.render(frame))?;

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

    fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::vertical([
            Constraint::Fill(2), 
            Constraint::Length(5)
        ]);
        let rects = frame.area().layout_vec(&layout);

        self.render_table(frame, rects[0]);
        self.render_player(frame, rects[1]);
    }

    fn render_player(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(Colors::BORDER))
            .padding(Padding::horizontal(1));
        
        frame.render_widget(&block, area);

        let inner = block.inner(area);

        let [top, bottom] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1)
        ])
        .areas(inner);

        let [title, controls, duration] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .flex(Flex::SpaceBetween)
        .areas(top);

        let track_title = self.app.get_current_song()
            .map(|song| song.track_title.as_str())
            .unwrap_or("");

        let controls_icon = if self.app.music_player.is_paused() {
            "󰒮 󰐊 󰒭"
        } else {
            "󰒮 󰏤 󰒭"
        };

        frame.render_widget(
            Paragraph::new(track_title).alignment(Alignment::Left), 
            title
        );
        frame.render_widget(
            Paragraph::new(controls_icon).alignment(Alignment::Center),
            controls
        );

        frame.render_widget(
            Paragraph::new(format!(
            "{} / {}",
            Self::format_time(self.progress),
            Self::format_time(self.max_duration()),
        )
        ).alignment(Alignment::Right)
        ,duration);

        let ratio = self.progress as f64 / self.max_duration() as f64;

        let progress_bar = Gauge::default()
            .label("")
            .ratio(ratio)
            .gauge_style(
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Black)
            );

        frame.render_widget(progress_bar, bottom); 
    }


    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let row_style = Style::default().fg(Colors::TEXT);
        let selected_row_style = Style::default()
            .bg(Colors::BG_HOVER)
            .fg(Colors::TEXT_HOVER);

        let header = ["Artist", "Title", "Duration"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .bottom_margin(1)
        ;

        let block = Block::default()
            .title_alignment(Alignment::Center)
            .title("  Song List  ").bold()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(Colors::BORDER))
            .padding(Padding::uniform(1))
        ;

        let rows = self.app.songs.iter().map(|song| {
            Row::new([
                Cell::from(song.track_artist.as_str()),
                Cell::from(song.track_title.as_str()),
                Cell::from(Self::format_time(song.track_duration))
            ])
            .style(row_style)
        });


        let t = Table::new(
            rows, 
            [           
                Constraint::Percentage(30),
                Constraint::Percentage(50),
                Constraint::Percentage(20),
            ],
        )
        .header(header)
        .block(block)
        .row_highlight_style(selected_row_style);

        frame.render_stateful_widget(t, area, &mut self.state);
    }
}
