use crate::app::App;
use crossterm::event::{self, Event, KeyCode};
use std::io::{Result, Write};
use std::os::unix::net::UnixStream;

pub fn handle_key_event(app: &mut App) -> Result<bool> {
    if let Event::Key(event) = event::read()? {
        match event.code {
            KeyCode::Enter => {
                if app.active_block == 1 {
                    app.selected_artist =
                        Some(app.artists[app.artist_state.selected().unwrap()].clone());
                    app.active_block = 2;
                    app.navigating_song = true;
                    app.song_state.select(Some(0));
                    app.track_block_title = app.selected_artist.clone().unwrap();
                } else if app.active_block == 2 {
                    app.selected_song = Some(
                        app.songs[&app.selected_artist.clone().unwrap()]
                            [app.song_state.selected().unwrap()]
                        .clone(),
                    );
                    let (_, url) = app.selected_song.clone().unwrap();
                    if url.ends_with(".mp3") {
                        if let Some(mpv) = &mut app.mpv {
                            let _ = mpv.kill();
                        }
                        app.mpv = Some(
                            std::process::Command::new("mpv")
                                .arg(url)
                                .arg("--no-video")
                                .arg("--force-window=no")
                                .arg("{ 'command': [ 'seek', '0', 'absolute' ] }")
                                .arg("--input-ipc-server=/tmp/mpvsocket")
                                .stdout(std::process::Stdio::null())
                                .stderr(std::process::Stdio::null())
                                .spawn()
                                .expect("mpv failed to start"),
                        );
                        app.is_playing = true;
                    }
                }
            }
            KeyCode::Esc => {
                if let Some(mpv) = &mut app.mpv {
                    let _ = mpv.kill();
                }
                app.is_playing = false;
                app.selected_song = None;
            }
            KeyCode::Char('+') => {
                if app.is_playing && app.volume + 5 <= 100 {
                    let mut stream = UnixStream::connect("/tmp/mpvsocket").unwrap();
                    write!(stream, "{{\"command\":[\"add\", \"volume\", \"5\"]}}\n").unwrap();
                    app.volume += 5;
                }
            }
            KeyCode::Char('-') => {
                if app.is_playing && app.volume - 5 >= 0 {
                    let mut stream = UnixStream::connect("/tmp/mpvsocket").unwrap();
                    write!(stream, "{{\"command\":[\"add\", \"volume\", \"-5\"]}}\n").unwrap();
                    app.volume -= 5;
                }
            }
            KeyCode::Char('p') => {
                if app.is_playing {
                    let mut stream = UnixStream::connect("/tmp/mpvsocket").unwrap();
                    write!(stream, "{{\"command\":[\"cycle\", \"pause\"]}}\n").unwrap();
                    app.is_playing = true;
                }
            }
            KeyCode::Char('b') => {
                if app.active_block == 2 {
                    app.active_block = 1;
                    app.selected_artist = None;
                    app.track_block_title = "Tracks".to_string();
                }
            }
            KeyCode::Char('l') => {
                if app.is_playing {
                    let mut stream = UnixStream::connect("/tmp/mpvsocket").unwrap();
                    if app.looping {
                        write!(
                            stream,
                            "{{\"command\":[\"set_property\", \"loop-file\", \"no\"]}}\n"
                        )
                        .unwrap();
                        app.looping = false;
                    } else {
                        write!(
                            stream,
                            "{{\"command\":[\"set_property\", \"loop-file\", \"inf\"]}}\n"
                        )
                        .unwrap();
                        app.looping = true;
                    }
                }
            }
            KeyCode::Up => {
                if let Some(selected) = if app.active_block == 1 {
                    app.artist_state.selected()
                } else {
                    app.song_state.selected()
                } {
                    if selected > 0 {
                        if app.active_block == 1 {
                            app.artist_state.select(Some(selected - 1));
                        } else {
                            app.song_state.select(Some(selected - 1));
                        }
                    }
                }
            }
            KeyCode::Down => {
                if let Some(selected) = if app.active_block == 1 {
                    app.artist_state.selected()
                } else {
                    app.song_state.selected()
                } {
                    let len = if app.active_block == 1 {
                        app.artists.len()
                    } else {
                        app.songs[&app.selected_artist.clone().unwrap()].len()
                    };
                    if selected < len - 1 {
                        if app.active_block == 1 {
                            app.artist_state.select(Some(selected + 1));
                        } else {
                            app.song_state.select(Some(selected + 1));
                        }
                    }
                }
            }
            KeyCode::Char('q') => {
                if let Some(mpv) = &mut app.mpv {
                    let _ = mpv.kill();
                }
                app.is_playing = false;
                return Ok(false);
            }
            _ => {}
        }
    }
    Ok(true)
}
