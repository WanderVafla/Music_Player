use std::{fs::{self, File}, io::{BufReader, Write}, path::PathBuf, ptr::null, rc::Rc, time::Duration, usize};
use eframe::egui::{self, Button, CentralPanel, ColorImage, CornerRadius, Image, Layout, ScrollArea, Slider, TextureHandle, TextureOptions, Ui, Vec2, Widget};
use lofty::{file::{AudioFile, TaggedFileExt}, picture::{MimeType, PictureType}, probe, read_from_path, tag::ItemKey};
use rodio::{queue, Decoder, OutputStream, OutputStreamBuilder, Sink};
use serde_json::from_str;
use rfd::FileDialog;

mod widgets;
use widgets::{ItemSong}; 

mod json_manager;
use json_manager::add_song_to_json;

mod player;
use player::Player;
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

struct MyApp {
    player: Player,

    initialized: bool,
    show_playlist: bool,
    show_queue: bool,
    show_current_data: bool,
    tracks: Vec<ItemSong>,
}   

impl MyApp {
    fn new() -> Self {
        Self {  
            player: Player::new(),
            initialized: false,
            show_playlist: true,
            show_queue: false,
            show_current_data: true,
            tracks: vec![] 
        }
    }

    fn load_track_list(&mut self) {
        for (index, item) in self.player.playlist.iter().enumerate() {
            let mut track = ItemSong::new(index, &item.title, &item.artist, item.cover_data.clone());
            self.tracks.push(track);
        }
    }

    

    fn add_new_song(&mut self) {
        if let Some(files) = FileDialog::new()
            .add_filter("audio", &["mp3", "flac", "wav"])
            .set_directory("/")
            .pick_files() {
                json_manager::add_song_to_json(files);
                // self.load_song_queue();
                self.player.playlist.clear();
                self.player.load_song_queue();
                self.load_track_list();
        }
            
    }

}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();

        if self.initialized == false {
            self.player.load_song_queue();
            self.load_track_list();
            self.player.do_order_song();
            self.initialized = true;
        }
        
        egui::SidePanel::left("Playlist").resizable(false)
        .min_width(250.0)
        .default_width(250.0)
        .show(ctx,|ui| {
            egui::TopBottomPanel::top("sort").show_inside(ui, |ui| { 
                ui.horizontal(|ui| {
                    if ui.button("queue").clicked() {
                        self.show_playlist = !self.show_playlist;
                        self.show_queue = !self.show_queue;
                        println!("show playlist: {}", self.show_playlist);
                        println!("show queue: {}", self.show_queue);
                    }
                    if ui.button("add").clicked() {
                        self.add_new_song();
                    }
                });
            });
            
            egui::ScrollArea::vertical().show(ui, |ui| {
                let row_height = 46.0;
                let num_items = self.tracks.len();
                
                ScrollArea::vertical().show_rows(ui,
                    row_height,
                    num_items,
                    |ui, row_range| {
                        if self.show_playlist {
                            for i in row_range {
                                ui.horizontal(|ui| {
                                    let track = &mut self.tracks[i];
                                    let index = track.id;

                                    if ui.add(track).clicked() {
                                        if index != self.player.current_index {
                                            self.player.current_index = index;
                                            println!("current song: {}", self.player.current_index);
                                            self.player.play_current();
                                            self.player.do_order_song(); 
                                        } else {
                                            self.player.playback_music();
                                        }
                                    }

                                });
                            }
                        }

                        if self.show_queue {
                             for i in self.player.order_song.clone() {
                                ui.horizontal(|ui| {
                                    let track = &mut self.tracks[i];
                                    let index = track.id;

                                    if ui.add(track).clicked() {
                                        if index != self.player.current_index {
                                            self.player.current_index = index;
                                            println!("current song: {}", self.player.current_index);
                                            self.player.play_current();
                                            self.player.do_order_song(); 
                                        } else {
                                            self.player.playback_music();
                                        }
                                    }

                                });
                            }
                        }
                    });
                });
            });
            
            
            
            egui::CentralPanel::default().show(ctx, |ui|{
                if self.show_current_data {
                    if !self.player.playlist.is_empty() {  
                        
                        ui.centered_and_justified(|ui| {
                            ui.vertical_centered(|ui| {

                                
                                let current_song = &mut self.player.playlist[self.player.current_index];
                                let size_current_cover: f32 = 250.0;
                                
                                    if current_song.cover_texture_converted == false {
                                        if let Some(bytes) = &current_song.cover_data {
                                            if let Ok(tex) = image::load_from_memory(bytes) {
                                                let tex = tex.resize(500, 500, image::imageops::FilterType::Triangle);
                                            let rgba = tex.to_rgba8();
                                            let size = [rgba.width() as usize, rgba.height() as usize];
                                            let pixels = rgba.into_raw();
                                            
                                            let texture: TextureHandle = ui.ctx().load_texture(
                                            format!("cover"),
                                            egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
                                            egui::TextureOptions::default()
                                        );
                                        println!("{}", current_song.cover_texture_converted);
                                        current_song.cover_texture = Some(texture);
                                        current_song.cover_texture_converted = true;
                                    }
                                }
                            }
                            
                            match &current_song.cover_texture {
                                Some(cover_texture) => {
                                    ui.add(
                                            egui::Image::new(cover_texture)
                                            .fit_to_exact_size(egui::Vec2::new(size_current_cover, size_current_cover))
                                        );
                                    }
                                    None => {
                                        ui.add_sized([size_current_cover, size_current_cover], egui::Button::new("🎵").corner_radius(10));
                                    }
                                }
                                
                                
                                ui.heading(self.player.playlist[self.player.current_index].title.clone());
                                ui.label(self.player.playlist[self.player.current_index].artist.clone());
                                
                                let max_duration = self.player.playlist[self.player.current_index].duration.as_secs_f32();
                                self.player.current_duration = self.player.sink.get_pos();
                                
                                    ui.horizontal(|ui| {
                                        let seconds_current = self.player.current_duration.as_secs();
                                        let current_time = ui.label(format!("{:02}:{:02}", seconds_current / 60, seconds_current % 60));
                                        // let title = ui.label("00:00:00");
                                        ui.spacing_mut().slider_width = ui.available_width() - (current_time.rect.size().x + 10.0);
                                        ui.add(
                                            egui::Slider::new(&mut self.player.current_duration.as_secs_f32(),
                                            0.0..=max_duration)
                                            .show_value(false));
                                        let seconds_max = max_duration.trunc() as u32;   
                                        ui.label(format!("{:02}:{:02}", seconds_max / 60, seconds_max % 60));
                                        // ui.label("00:00:00");
                                    });
                                    
                                if self.player.playing == true {
                                    if self.player.sink.empty() == true {
                                        if self.player.looped == true {
                                            self.player.play_current();
                                        } else {
                                            self.player.next_song();
                                        }
                                        println!("song is finished");
                                    }
                                }
                            });
                        });
                    }
                }
            });
            
            egui::TopBottomPanel::bottom("PlaybackPanel").show(ctx, |ui| {
                    ui.allocate_ui(egui::vec2(ui.available_width(), 30.0), |ui| {

                        
                        // ui.add(Playback::new());
                        ui.horizontal(|ui| {

                        ui.horizontal(|ui| {
                            
                        if ui.add_sized([16.0, 16.0], egui::Checkbox::new(&mut self.player.looped, "")).changed() {
                            println!("loop: {}", self.player.looped);
                        }
                        
                        ui.horizontal(|ui| {
                            if ui.button("⏮").clicked() {
                                self.player.prev_song();
                                }
                                if self.player.playing {
                                    if ui.button("⏸").clicked() {
                                        self.player.playback_music();
                                    }
                                } else {
                                    if ui.button("▶").clicked() {
                                        self.player.playback_music();
                                    }
                                }
                                if ui.button("⏭").clicked() {
                                    self.player.next_song();
                                }
                            });

                            if ui.add_sized([16.0, 16.0], egui::Checkbox::new(&mut self.player.random, ""))
                            .changed() {
                                self.player.do_order_song();
                            }
                        });

                                if ui.button("").clicked() {
                                    self.player.volume = 0.0;
                                    self.player.sink.set_volume(self.player.volume);
                                }
                                if ui.add(egui::Slider::new(&mut self.player.volume, 0.0..=10.0)
                                .show_value(false))
                                .changed() {
                                    self.player.sink.set_volume(self.player.volume);
                                    println!("volume: {}", self.player.volume);
                                }
                            });
                            
                        });
            });
                 
    }
}
