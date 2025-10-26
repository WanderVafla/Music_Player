use std::{fs::{self, File}, io::{BufReader, Write}, path::PathBuf, time::Duration, usize};

use eframe::{egui::{self, Button, CentralPanel, ColorImage, CornerRadius, Layout, TextureHandle, TextureOptions}, epaint::tessellator::Path, glow::{MAX_DUAL_SOURCE_DRAW_BUFFERS, PRIMITIVES_SUBMITTED}};

use lofty::{file::{AudioFile, TaggedFileExt}, probe, read_from_path, tag::ItemKey};
use rodio::{queue, Decoder, OutputStream, OutputStreamBuilder, Sink};
use serde::de::value::Error;

use clap::{builder::Str, Parser};

use serde::{Deserialize, Serialize};

use serde_json::from_str;

use rand::seq::{index, SliceRandom};
use rand::rng;

fn main() -> Result<(), eframe::Error> {
    let mut options = eframe::NativeOptions::default(); // создаём по умолчанию

    // Настраиваем размер окна через viewport
    options.viewport = egui::ViewportBuilder::default()
        .with_inner_size([800.0, 600.0]);
    eframe::run_native(
        "Моё первое окно",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),
    )
}

struct AlbumsList {
    album: String,
    album_artist: String,
}
struct SongData {
    id: usize,
    title: String,
    artist: String,
    album: String,
    durration: Duration,
    path: PathBuf,
}
struct MyApp {
    stream: rodio::OutputStream,
    sink: rodio::Sink,

    current_index: usize,
    random_index: usize,
    playlist: Vec<SongData>,
    order_song: Vec<usize>,
    albums_list: Vec<AlbumsList>,
    initialized: bool,

    current_durration: Duration,
    playing: bool,

    loopped: bool,
    random: bool,
    volume: f32,
    show_panel: bool,
}   

impl MyApp {
    fn new() -> Self {
        let stream = OutputStreamBuilder::open_default_stream().unwrap();
        let sink = rodio::Sink::connect_new(stream.mixer());

        let current_index = 0;
        let random_index = 0;
        let playlist = vec![];
        let order_song = vec![];
        let albums_list = vec![];

        let initialized: bool = false;

        let current_durration = sink.get_pos();
        let playing = false;

        let loopped = false;
        let random = false;
        let volume = 1.0;


        Self { stream, sink, current_index, random_index, playlist, albums_list, initialized, current_durration, playing, order_song, loopped, random, volume, show_panel: true }
   }

    fn load_song_queue(&mut self) {
        let file = PathBuf::from(r"C:\Users\User\Music\music");
        let mut id: usize = 0;
        for song in fs::read_dir(file).unwrap() {
            let song_entry = song.unwrap();
            let path = song_entry.path();

            let tagged_file_result = read_from_path(&path);
            if let Ok(tagged_file) = tagged_file_result {
                if let Some(tag) = tagged_file.primary_tag() {
                    let title: String = tag.get_string(&ItemKey::TrackTitle).unwrap_or("Unknow Title").to_string();                        
                    let album: String = tag.get_string(&ItemKey::AlbumTitle).unwrap_or("Unknow Title").to_string();  
                    let album_artist: String = tag.get_string(&ItemKey::AlbumArtist).unwrap_or("Unknow Title").to_string();                      
                    let artist: String = tag.get_string(&ItemKey::TrackArtist).unwrap_or("Unknow Title").to_string();

                    let durration = tagged_file.properties().duration();

            let song_data = SongData {
                id,
                title,
                artist,
                album: album.clone(),
                durration,
                path: path.clone()
            };
            self.playlist.push(song_data);

            let album_list = AlbumsList {
                album,
                album_artist
            };
            self.albums_list.push(album_list);
            }

        }
        println!("id load: {}", id);
            id = id + 1;
    }
    println!("album len: {}", self.albums_list.len());
   
    }

    fn do_order_song(&mut self) {
        if self.random == true {
            self.random_index = 0;

            let mut rng = rng();

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

    fn play_current(&mut self) {
        let path = &self.playlist[self.current_index].path;
        let file = File::open(path).expect("Errore to open music file");
    
        let source = Decoder::new(BufReader::new(file)).expect("Errore with a decoder");

        self.sink.stop();
        self.sink.append(source);
        self.sink.play();

        self.playing = true;
    }

    fn playback_music(&self) {
        if self.sink.is_paused() == true {
            self.sink.play();
        } else {
            self.sink.pause();
        }
    }

    fn prev_song(&mut self) {
        if self.random == true {
            if self.random_index != 0 {
                self.random_index = self.random_index - 1;
    
                let take_index: usize = self.order_song[self.random_index];
    
                self.current_index = take_index;
                println!("current index: {}", self.current_index);
                self.play_current();
                print!("random play")
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
    fn next_song(&mut self) {
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

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.initialized == false {
            self.load_song_queue();
            self.do_order_song();
            self.initialized = true;
        } 
        let mut clicked_index: Option<usize> = None;

        egui::SidePanel::left("Playlist").resizable(false)
        .min_width(250.0)
        .default_width(250.0)
        .show(ctx,|ui| {

            egui::TopBottomPanel::top("sort").show_inside(ui, |ui| {
                if ui.button("").clicked() {

                }
            });

            egui::ScrollArea::vertical().show(ui, |ui| {
                for (index, song_data_item) in self.playlist.iter_mut().enumerate() {
                    ui.horizontal(|ui| { 
                        if ui.add_sized([64.0, 64.0], egui::Button::new("song").corner_radius(10)).clicked() {
                            clicked_index = Some(index);
                        }
                        ui.vertical(|ui| {
                            ui.label(&song_data_item.title);
                            ui.label(&song_data_item.artist);
                        });
                    });
                }
            });
        });

        if let Some(index) = clicked_index {
            self.current_index = index;
            println!("current song: {}", self.current_index);
            self.play_current();
            self.do_order_song(); 
            }

        egui::CentralPanel::default().show(ctx, |ui|{
            ui.vertical(|ui| {
                ui.label(self.playlist[self.current_index].title.clone());
                ui.label(self.playlist[self.current_index].artist.clone());
            });

            ui.horizontal(|ui| {
                let max_durration = self.playlist[self.current_index].durration.as_secs_f32();
                self.current_durration = self.sink.get_pos();

                ui.with_layout(Layout::centered_and_justified(egui::Direction::TopDown) ,|ui| {
                    ui.horizontal(|ui| {
                        ui.label(self.current_durration.as_secs().to_string());
                        if ui.add(egui::Slider::new(&mut self.current_durration.as_secs_f32(),
                        0.0..=max_durration).show_value(false)).changed() {
                            println!("durration: {}", self.current_durration.as_secs());
                        }
                        ui.label(max_durration.to_string());
                    });
                });
                if self.playing == true {
                    if self.sink.empty() == true {
                        if self.loopped == true {
                            self.play_current();
                        } else {
                            self.next_song();
                        }
                        println!("song is finished");
                    }
                }
            });

            egui::TopBottomPanel::bottom("PlaybackPanel").show(ctx, |ui| {
                ui.centered_and_justified(|ui| { 
                        ui.horizontal(|ui| {
                            if ui.add_sized([16.0, 16.0], egui::Checkbox::new(&mut self.loopped, "")).changed() {
                                println!("loop: {}", self.loopped);
                            }
                            
                            ui.horizontal(|ui| {
                                if ui.button("prev").clicked() {
                                    self.prev_song();
                                }
                                if ui.button("play").clicked() {
                                    self.playback_music();
                                }
                                if ui.button("next").clicked() {
                                    self.next_song();
                                }
                                
                                if ui.add_sized([16.0, 16.0], egui::Checkbox::new(&mut self.random, ""))
                            .changed() {
                                self.do_order_song();
                            }

                            });
                            ui.horizontal(|ui| {
                                if ui.button("").clicked() {
                                    self.volume = 0.0;
                                    self.sink.set_volume(self.volume);
                                }
                                if ui.add(egui::Slider::new(&mut self.volume, 0.0..=10.0)
                                .show_value(false))
                                .changed() {
                                    self.sink.set_volume(self.volume);
                                    println!("volume: {}", self.volume);
                                }
                            });
                        });
                });
        });
    });

       
    }
}

