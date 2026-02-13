use std::{fs::{self, File}, io::{BufReader, Write}, path::PathBuf, ptr::null, time::Duration, usize};
use eframe::egui::TextureHandle;
use lofty::{file::{AudioFile, TaggedFileExt}, picture::{MimeType, PictureType}, probe, read_from_path, tag::ItemKey};
use rodio::{queue, Decoder, OutputStream, OutputStreamBuilder, Sink};
use rand::{rng, seq::SliceRandom};

use crate::json_manager::read_paths;

use crate::json_manager;

pub struct SongData {
    pub title: String,
    pub artist: String,
    pub album: String,
    album_artist: String,
    pub(crate) duration: Duration,
    path: PathBuf,
    pub cover_data: Option<Vec<u8>>,
    pub cover_texture: Option<TextureHandle>,
    pub cover_texture_converted: bool,
}

// #[derive(Clone)]
pub struct Player {
    stream: rodio::OutputStream,
    pub sink: rodio::Sink,

    pub current_index: usize,
    random_index: usize,
    pub playlist: Vec<SongData>,
    pub order_song: Vec<usize>,

    pub current_duration: Duration,
    pub playing: bool,

    pub looped: bool,
    pub random: bool,
    pub volume: f32,
}

impl Player {
    pub fn new() -> Self {
        let stream = OutputStreamBuilder::open_default_stream().unwrap();
        let sink = rodio::Sink::connect_new(stream.mixer());
    
        let current_index = 0;
        let random_index = 0;
        let playlist = vec![];
        let order_song = vec![];
    
        let current_duration = sink.get_pos();
        let playing = false;
    
        let looped = false;
        let random = false;
        let volume = 4.0;

        Self { stream, sink, current_index, random_index, playlist, order_song, current_duration, playing, looped, random, volume }
    }

    fn load_path(&self) -> Vec<PathBuf> {
        let files = json_manager::read_paths();
        return files;
    }

    pub fn load_song_queue(&mut self) {
        for song in self.load_path().clone() {

            let tagged_file_result = read_from_path(&song);
            if let Ok(tagged_file) = tagged_file_result {
                if let Some(tag) = tagged_file.primary_tag() {
                    let title: String = tag.get_string(&ItemKey::TrackTitle).unwrap_or("Unknown Title").to_string();                        
                    let album: String = tag.get_string(&ItemKey::AlbumTitle).unwrap_or("Unknown Album").to_string();  
                    let album_artist: String = tag.get_string(&ItemKey::AlbumArtist).unwrap_or("Unknown AlbumArtist").to_string();                      
                    let artist: String = tag.get_string(&ItemKey::TrackArtist).unwrap_or("Unknown Artist").to_string();

                    let duration = tagged_file.properties().duration();

                    let cover_data = tag.pictures().iter()
                        .find(|p| p.pic_type() == PictureType::CoverFront)
                        .map(|p| p.data().to_vec());

                    let song_data = SongData {
                        title,
                        artist,
                        album: album.clone(),
                        album_artist,
                        duration,
                        path: song.clone(),
                        cover_data,
                        cover_texture: None,
                        cover_texture_converted: false
                    };
                    self.playlist.push(song_data);
                }
            }
        }
    }

    pub fn do_order_song(&mut self) {
        if self.random == true {
            self.random_index = 0;

            let mut rng = rand::rng();

            self.order_song = (0..self.playlist.len()).collect();
            println!("{:?}", self.order_song);

            self.order_song.shuffle(&mut rng);

            if let Some(b) = self.order_song.iter().position(|&x| x == self.current_index) {
                self.order_song.swap( 0, b);
            }
            println!("random: {:?}", self.order_song);

        } else {
            self.order_song.clear();
        }
    }

    pub fn play_current(&mut self) {
        let path = &self.playlist[self.current_index].path;
        let file = File::open(path).expect("Errore to open music file");
    
        let source = Decoder::new(BufReader::new(file)).expect("Errore with a decoder");

        self.sink.stop();
        self.sink.append(source);
        self.sink.play();
                    
        self.playing = true;
    }

    pub fn playback_music(&mut self) {
        if self.sink.is_paused() == true {
            self.sink.play();
            self.playing = true;
        } else if self.sink.is_paused() == false {
            self.sink.pause();
            self.playing = false;
        }
    }

    pub fn prev_song(&mut self) {
        if self.random == true {
            if self.random_index != 0 {
                self.random_index = self.random_index - 1;
    
                let take_index: usize = self.order_song[self.random_index];
    
                self.current_index = take_index;
                println!("current index: {}", self.current_index);
                self.play_current();
                println!("random play")
            } else {
                self.play_current();
            }
        } else {
            if self.current_index == 0 {
                self.play_current();
                println!("Current index: {}", self.current_index);
            } else {
                self.current_index = self.current_index - 1;
                self.play_current();
                println!("Current index: {}", self.current_index);
            } 
        }
    }

    pub fn next_song(&mut self) {
        if self.random == true {
            if self.random_index != self.order_song.len() - 1 {
                self.random_index = self.random_index + 1;
    
                let take_index: usize = self.order_song[self.random_index];
    
                self.current_index = take_index;
                println!("current index: {}", self.current_index);
                self.play_current();
                print!("random play")
            } else {
                self.play_current();
            }
        } else {
            if self.current_index == self.playlist.len() - 1 {
                self.play_current();
                println!("Current index: {}", self.current_index);
            } else {
                self.current_index = self.current_index + 1;
                self.play_current();
                println!("Current index: {}", self.current_index);
            }
        }
    }

}