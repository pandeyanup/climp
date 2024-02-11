use id3::{Tag, TagLike};
use ratatui::widgets::ListState;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub struct App {
    pub active_block: usize,
    pub artists: Vec<String>,
    pub songs: HashMap<String, Vec<(String, String)>>,
    pub artist_state: ListState,
    pub song_state: ListState,
    pub selected_artist: Option<String>,
    pub selected_song: Option<(String, String)>,
    pub navigating_song: bool,
    pub track_block_title: String,
    pub is_playing: bool,
    pub mpv: Option<std::process::Child>,
    pub volume: i32,
    pub looping: bool,
}

impl App {
    pub fn new(dir: &str) -> App {
        let mut artist_state = ListState::default();
        artist_state.select(Some(0));

        let song_state = ListState::default();

        let mut songs = HashMap::new();
        let mut artists = HashSet::new();

        App::read_dir(dir, &mut artists, &mut songs);

        let mut artists_vec: Vec<String> = artists.into_iter().collect();
        artists_vec.sort();

        for song_list in songs.values_mut() {
            song_list.sort();
        }

        App {
            active_block: 1, // artist block
            artists: artists_vec,
            songs,
            artist_state,
            song_state,
            selected_artist: None,
            selected_song: None,
            navigating_song: false,
            track_block_title: "Tracks".to_string(),
            is_playing: false,
            mpv: None,
            volume: 100,
            looping: false,
        }
    }

    fn read_dir(
        dir: &str,
        artists: &mut HashSet<String>,
        songs: &mut HashMap<String, Vec<(String, String)>>,
    ) {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                App::read_dir(path.to_str().unwrap(), artists, songs);
            } else if path.is_file() && path.extension().unwrap_or_default() == "mp3" {
                let tag = Tag::read_from_path(&path).unwrap();
                if let (Some(artist), Some(title)) = (tag.artist(), tag.title()) {
                    artists.insert(artist.to_string());
                    songs
                        .entry(artist.to_string())
                        .or_insert_with(Vec::new)
                        .push((title.to_string(), path.to_str().unwrap().to_string()));
                }
            }
        }
    }
}
