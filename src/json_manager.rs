use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Song {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_artist: String,
    pub durration: f32, 
    pub path: String,
}

pub fn save_songs_to_json(songs: &Vec<Song>) {
    let json = serde_json::to_string_pretty(songs).unwrap();
    let mut file = File::create("SongsList.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
}