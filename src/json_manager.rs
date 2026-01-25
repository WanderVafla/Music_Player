
use lofty::file;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::any::Any;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default, Debug)]
struct PlaylistFIle {
    songs: Vec<PathBuf>
}


pub fn add_song_to_json(paths: Vec<PathBuf>) {
    let file_path = "SongsList.json";
    let playlistFile = PlaylistFIle { songs: paths.clone() };

    if PathBuf::from(file_path).exists() {
        let mut file = File::open(file_path).expect("did exists");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Not did try for read");

        if !content.trim().is_empty() {
            
            let list_paths = read_paths();
            let mut playlist_file: PlaylistFIle = serde_json::from_str(&content).unwrap();
            for check_path in paths {
                if list_paths.contains(&check_path) {
                    playlist_file.songs.push(check_path.clone());
                }
            }
            let json = serde_json::to_string_pretty(&playlistFile).unwrap();
            let mut file = File::create(file_path).unwrap();
            file.write_all(json.as_bytes()).unwrap();
            println!("{}", content);

        } else {
            let json = serde_json::to_string_pretty(&playlistFile).unwrap();
            let mut file = File::create(file_path).unwrap();
            file.write_all(json.as_bytes()).unwrap();

        }
    } else {

        let json = serde_json::to_string_pretty(&playlistFile).unwrap();
        let mut file = File::create(file_path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
        
    }
}

pub fn read_paths() -> Vec<PathBuf> {
    check_json_path();

    let file_path = PathBuf::from("SongsList.json");
    let mut list_songs_paths: Vec<PathBuf> = vec![];

    let content= fs::read_to_string(file_path).unwrap();
    let playlist_file: PlaylistFIle = serde_json::from_str(&content).unwrap();

    for song in playlist_file.songs {
        list_songs_paths.push(song);
    }

    return list_songs_paths;

}

fn check_json_path() {
    let file_path = PathBuf::from("SongsList.json");
    let data = serde_json::json!({
        "songs": []
    });

    if file_path.exists() {
        let mut file = File::open(file_path.clone()).expect("did exists");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Not did try for read");

        if !content.trim().is_empty() {

            match serde_json::from_str::<PlaylistFIle>(&content) {
                Ok(_) => {},
                Err(e) => {
                    create_struct(data, file_path);
                    println!("file dont have good struct!")
                }}

        } else {
            create_struct(data, file_path);
        }
    } else {
        create_struct(data, file_path);
    }

    fn create_struct(structure: Value, file_path: PathBuf) {
        let json = serde_json::to_string_pretty(&structure).unwrap();
        let mut file = File::create(file_path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

}