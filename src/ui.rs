use crate::{app::App, blocks, key_event};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{CrosstermBackend, Terminal},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{List, ListItem, Paragraph, Wrap},
};
use std::io;

pub fn run_ui(mut app: App) -> io::Result<()> {
    enable_raw_mode()?;

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(8),
                        Constraint::Percentage(if app.is_playing { 82 } else { 84 }),
                        Constraint::Percentage(if app.is_playing { 10 } else { 8 }),
                    ]
                    .as_ref(),
                )
                .split(frame.size());

            let middle_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(chunks[1]);

            let blocks = [
                blocks::search_block(app.active_block),
                blocks::artist_block(app.active_block),
                blocks::track_block(app.active_block),
                blocks::status_block(app.active_block),
            ];

            // render text inside the search_block
            let help_block = blocks[0].to_owned().title("Help");
            let help_items = Paragraph::new(Span::styled(
                "'q' - quit | 'p' - pause/play | 'l' - loop playing | '+' or '-' - change volume | 'esc' - stop playing",
                Style::default().fg(Color::Rgb(180, 190, 254)),
            ))
            .wrap(Wrap { trim: true })
            .add_modifier(Modifier::ITALIC)
            .block(help_block);

            // render artists in the artist_block
            let artist_block = blocks[1].to_owned().title("Artists");
            let artist_list = List::new(app.artists.iter().map(|i| {
                ListItem::new(i.clone()).style(Style::default().fg(Color::Rgb(137, 220, 235)))
            }))
            .block(artist_block)
            .highlight_style(Style::default().bg(Color::White).fg(Color::Black));

            if app.active_block == 1 {
                frame.render_stateful_widget(artist_list, middle_chunks[0], &mut app.artist_state);
            } else {
                frame.render_widget(artist_list, middle_chunks[0]);
            }
            let selected_artist = app
                .selected_artist
                .clone()
                .unwrap_or_else(|| app.artists[app.artist_state.selected().unwrap()].clone());

            // render songs in the track_block
            let track_block = blocks[2].to_owned().title(app.track_block_title.clone());
            let styled_tracks: Vec<ListItem> = app.songs[&selected_artist]
                .iter()
                .enumerate()
                .map(|(i, r)| {
                    let index = Span::styled(
                        format!("{}. ", i + 1),
                        Style::default().fg(Color::Rgb(198, 160, 246)),
                    );
                    let title =
                        Span::styled(format!("{}", r.0), Style::default().fg(Color::White).bold());
                    ListItem::new(Line::from(vec![index, title]))
                })
                .collect::<Vec<ListItem>>();

            let song_list = List::new(styled_tracks)
                .block(track_block)
                .highlight_style(Style::default().bg(Color::White).fg(Color::Black));

            if app.active_block == 2 {
                frame.render_stateful_widget(song_list, middle_chunks[1], &mut app.song_state);
            } else {
                frame.render_widget(song_list, middle_chunks[1]);
            }

            // render the status_block
            let status_block = blocks[3].to_owned().title("Status");
            let playing_styled = if let Some(selected_song) = &app.selected_song {
                Span::styled(
                    format!("Playing: {}", selected_song.0),
                    Style::default().fg(Color::Rgb(203, 166, 247)),
                )
            } else {
                Span::styled(
                    format!("Start playing something to see it here!",),
                    Style::default().fg(Color::Rgb(180, 190, 254)),
                )
            };
            let statusbar = Span::styled(
                format!("vol: {}  loop: {}", app.volume, app.looping.to_string()),
                Style::default().fg(Color::Green),
            )
            .to_right_aligned_line();
            let now_playing = List::new(vec![
                ListItem::new(playing_styled),
                ListItem::new(statusbar),
            ])
            .block(status_block);

            frame.render_widget(help_items, chunks[0]);
            frame.render_widget(now_playing, chunks[2]);
        })?;

        if !key_event::handle_key_event(&mut app)? {
            break;
        }
    }

    disable_raw_mode()?;
    Ok(())
}
