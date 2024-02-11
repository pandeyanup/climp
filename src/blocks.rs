use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
};

pub fn search_block<'a>(active_block: usize) -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(if active_block == 0 {
            Style::default().fg(Color::Rgb(166, 227, 161))
        } else {
            Style::default()
        })
}

pub fn artist_block<'a>(active_block: usize) -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(if active_block == 1 {
            Style::default().fg(Color::Rgb(243, 139, 168))
        } else {
            Style::default()
        })
}

pub fn track_block<'a>(active_block: usize) -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(if active_block == 2 {
            Style::default().fg(Color::Rgb(245, 224, 220))
        } else {
            Style::default()
        })
}

pub fn status_block<'a>(active_block: usize) -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(if active_block == 3 {
            Style::default().fg(Color::Rgb(203, 166, 247))
        } else {
            Style::default()
        })
}
