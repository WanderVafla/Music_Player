use std::{fs::{self, File}, io::{BufReader, Write}, path::PathBuf, time::Duration};

use eframe::{egui::{self, Button, CentralPanel, ColorImage, CornerRadius, TextureHandle, TextureOptions}, epaint::tessellator::Path, glow::{MAX_DUAL_SOURCE_DRAW_BUFFERS, PRIMITIVES_SUBMITTED}};

use lofty::{file::{AudioFile, TaggedFileExt}, probe, read_from_path, tag::ItemKey};
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use serde::de::value::Error;

use clap::{builder::Str, Parser};

use serde::{Deserialize, Serialize};

use serde_json::from_str;

use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::default::{get_codecs, get_probe};

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
#[derive(Deserialize, Serialize, Debug)]
struct Song_Data {
    title: String,
    artist: String,
    durration: Duration,
    path: PathBuf,
}
struct MyApp {
    stream: rodio::OutputStream,
    sink: rodio::Sink,

    current_index: usize,
    playlist: Vec<Song_Data>,
    initialized: bool,

    current_durration: Duration,
    playing: bool,

    loopped: bool,
    random: bool,
    volume: f32,
}   

impl MyApp {
    fn new() -> Self {
        let stream = OutputStreamBuilder::open_default_stream().unwrap();
        let sink = rodio::Sink::connect_new(stream.mixer());

        let current_index = 0;
        let playlist = vec![];
        let initialized: bool = false;

        let current_durration = sink.get_pos();
        let playing = false;

        let loopped = false;
        let random = false;
        let volume = 1.0;


        Self { stream, sink, current_index, playlist, initialized, current_durration, playing, loopped, random, volume }
   }

    fn load_song_queue(&mut self) {
        let file = PathBuf::from(r"C:\Users\User\Music\music");
        
        for song in fs::read_dir(file).unwrap() {
            let song_entry = song.unwrap();
            let path = song_entry.path();

            let tagged_file_result = read_from_path(&path);
            if let Ok(tagged_file) = tagged_file_result {
                if let Some(tag) = tagged_file.primary_tag() {
                    let title: String = tag.get_string(&ItemKey::TrackTitle).unwrap_or("Unknow Title").to_string();                        
                    // let album: String = tag.get_string(&ItemKey::AlbumTitle).unwrap_or("Unknow Title").to_string();                         
                    let artist: String = tag.get_string(&ItemKey::TrackArtist).unwrap_or("Unknow Title").to_string();

                    let durration = tagged_file.properties().duration();

            let song_data = Song_Data {
                title,
                artist,
                durration,
                path: path.clone()
            };
            self.playlist.push(song_data);
            }
        }
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
        if self.current_index == 0 {
            self.play_current();
            println!("Current index: {}", self.current_index);
        } else {
            self.current_index = self.current_index - 1;
            self.play_current();
            println!("Current index: {}", self.current_index);
        } 
        println!("prev!");
    }
    fn next_song(&mut self) {
        if self.current_index == self.playlist.len() {
            self.play_current();
            println!("Current index: {}", self.current_index);
        } else {
            self.current_index = self.current_index + 1;
            self.play_current();
            println!("Current index: {}", self.current_index);
        }
        println!("next!");
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.initialized == false {
            self.load_song_queue();
            self.initialized = true;
        } 
        let mut clicked_index: Option<usize> = None;

        egui::SidePanel::left("Playlist").resizable(false)
        .min_width(250.0)
        .default_width(250.0)
        .show(ctx,|ui| {

            if ui.add_sized([10.0, 10.0], egui::Button::new("")).clicked() {
                println!("PORNO!!!!")
            }

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
            }

        egui::CentralPanel::default().show(ctx, |ui|{
            ui.vertical(|ui| {
                ui.label(self.playlist[self.current_index].title.clone());
                ui.label(self.playlist[self.current_index].artist.clone());
            });

            ui.horizontal(|ui| {
                let max_durration = self.playlist[self.current_index].durration.as_secs_f32();
                self.current_durration = self.sink.get_pos();

                ui.horizontal(|ui| {
                    ui.label(self.current_durration.as_secs().to_string());
                    if ui.add(egui::Slider::new(&mut self.current_durration.as_secs_f32(),
                    0.0..=max_durration).show_value(false)).changed() {
                        println!("durration: {}", self.current_durration.as_secs());
                    }
                    ui.label(max_durration.to_string());
                });
                if self.playing == true {
                    if self.sink.empty() == true {
                        if self.loopped == true {
                            self.play_current();
                        } else {
                            self.current_index = self.current_index + 1;
                            self.play_current();
                        }
                        println!("song is finished");
                    }
                }
            });

            egui::TopBottomPanel::bottom("PlaybackPanel").show(ctx, |ui| {
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
                        println!("random: {}", self.random);
                        }

                    });
                    if ui.add(egui::Slider::new(&mut self.volume, 0.0..=10.0)
                        .vertical()
                        .show_value(false))
                        .changed() {
                            self.sink.set_volume(self.volume);
                            println!("volume: {}", self.volume);
                        }
                });
            
        });
    });

       
    }
}

