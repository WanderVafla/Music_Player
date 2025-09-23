use std::fs::{File, Metadata};
use std::io::Write;
use std::string::String;
use std::str::Bytes;
use std::{fs, path};
use std::path::{Path, PathBuf};

use lofty::file::TaggedFileExt;
use lofty::read_from_path;
use lofty::tag::ItemKey;

use serde::{Serialize, Deserialize};

pub fn charge_songs()  {
    let mut songs_paths = vec![];
    load_paths(&mut songs_paths);
    take_metadata_from_song(songs_paths);
    // save_to_json(metadata, filename);
}

pub fn load_paths(path_list: &mut Vec<PathBuf>) {

    let path_folder = Path::new("C:\\Users\\User\\Music\\music");

    for song in fs::read_dir(path_folder).expect("A folder did'n faund") {
        let song = song.expect("the file posibilite not for read");
        let path = song.path();
        
        if let Some(ext) = path.extension() {
            if ext == "mp3" {
                path_list.push(path);
            } else if ext == "flac" {
                path_list.push(path);
            } else if ext == "wav" {
                path_list.push(path);
            } else {
                println!("this file is not audio!");
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Song {
    title: String,
    artist: String,
    album: String,
    path: PathBuf,
}

pub fn take_metadata_from_song(path_list: Vec<PathBuf>) {

    let mut title: Option<String>;
    let mut artist: Option<String>;
    let mut album: Option<String>;
    let mut path: PathBuf;

    for path_song in path_list {

        let tagged_file_result = read_from_path(&path_song);
        if let Ok(tagged_file) = tagged_file_result {
            if let Some(tag) = tagged_file.primary_tag() {
                let title: String = tag.get_string(&ItemKey::TrackTitle).unwrap_or("Unknow Title").to_string();
                    // println!("Название: {}", title);
                
                let album: String = tag.get_string(&ItemKey::AlbumTitle).unwrap_or("Unknow Title").to_string(); 
                    // println!("Subtitle: {}", album);
                
                 let artist: String = tag.get_string(&ItemKey::TrackArtist).unwrap_or("Unknow Title").to_string();
                    // artist = artist_meta;
                    // println!("Artist: {}", artist);
                

        let song = Song {
            title,
            artist,
            album,
            path: path_song.clone(),
        };

        save_to_json(&song, "music_list.json");
        
        println!("{:?}", song);
            } else {
                println!("Failed to read tagged file");
            };
        };
    }
}

pub fn save_to_json(metadata: &Song, filename: &str) {
    let mut songs = if Path::new(filename).exists() {
        // Читаем существующий файл и парсим его
        let file_content = std::fs::read_to_string(filename).unwrap_or_else(|_| "[]".to_string());
        serde_json::from_str::<Vec<Song>>(&file_content).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    };

    songs.push(metadata.clone());

    let json = serde_json::to_string_pretty(&songs).unwrap();
    let mut file = File::create(filename).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
