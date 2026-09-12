use ratatui::layout::{Alignment, Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, BorderType, Borders, Cell, Gauge, Padding, Paragraph, Row, Table};
use ratatui::Frame;

use crate::tui::Colors;
use super::app::TUI;

pub fn render(tui: &mut TUI, frame: &mut Frame) {
    let layout = Layout::vertical([
        Constraint::Fill(2), 
        Constraint::Length(5)
    ]);
    let rects = frame.area().layout_vec(&layout);

    render_table(tui, frame, rects[0]);
    render_player(tui, frame, rects[1]);
}

fn render_player(tui: &mut TUI, frame: &mut Frame, area: Rect) {
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

    let track_title = tui.app.get_current_song()
        .map(|song| song.track_title.as_str())
        .unwrap_or("");

    let controls_icon = if tui.app.music_player.is_paused() {
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
        format_time(tui.progress),
        format_time(tui.max_duration()),
    )
    ).alignment(Alignment::Right)
    ,duration);

    let ratio = tui.progress as f64 / tui.max_duration() as f64;

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


fn render_table(tui: &mut TUI, frame: &mut Frame, area: Rect) {
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

    let rows = tui.app.songs.iter().map(|song| {
        Row::new([
            Cell::from(song.track_artist.as_str()),
            Cell::from(song.track_title.as_str()),
            Cell::from(format_time(song.track_duration))
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

    frame.render_stateful_widget(t, area, &mut tui.state);
}

pub fn format_time(secs: u64) -> String {
    let min = secs / 60;
    let sec = secs % 60;

    format!("{:02}:{:02}", min, sec)
}
