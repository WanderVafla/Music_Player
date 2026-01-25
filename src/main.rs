use std::{fs::{self, File}, io::{BufReader, Write}, path::PathBuf, ptr::null, time::Duration, usize};
use eframe::{egui::{self, Button, CentralPanel, ColorImage, CornerRadius, Image, Layout, ScrollArea, TextureHandle, TextureOptions, Ui, Vec2, Slider}, epaint::tessellator::Path, glow::{MAX_DUAL_SOURCE_DRAW_BUFFERS, PRIMITIVES_SUBMITTED}};
use image::{ImageFormat, ImageReader};
use lofty::{file::{AudioFile, TaggedFileExt}, picture::{MimeType, PictureType}, probe, read_from_path, tag::ItemKey};
use rodio::{queue, Decoder, OutputStream, OutputStreamBuilder, Sink};
use serde_json::from_str;
use rand::seq::{SliceRandom};
use rand::rng;
mod widgets;
use widgets::{ItemSong, ItemSongGrid, ItemSongAction, Playback}; 
use rfd::FileDialog;

mod json_manager;
use json_manager::add_song_to_json;
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
struct SongData {
    title: String,
    artist: String,
    album: String,
    album_artist: String,
    duration: Duration,
    path: PathBuf,
    cover_data: Option<Vec<u8>>,
    cover_texture: Option<TextureHandle>,
    cover_texture_converted: bool,
}
struct MyApp {
    stream: rodio::OutputStream,
    sink: rodio::Sink,

    current_index: usize,
    random_index: usize,
    playlist: Vec<SongData>,
    order_song: Vec<usize>,
    initialized: bool,

    current_duration: Duration,
    playing: bool,

    looped: bool,
    random: bool,
    volume: f32,

    show_playlist: bool,
    show_queue: bool,
    show_current_data: bool,

    tracks: Vec<ItemSong>,
    
}   

impl MyApp {
    fn new() -> Self {
        let stream = OutputStreamBuilder::open_default_stream().unwrap();
        let sink = rodio::Sink::connect_new(stream.mixer());

        let current_index = 0;
        let random_index = 0;
        let playlist = vec![];
        let order_song = vec![];

        let initialized: bool = false;

        let current_duration = sink.get_pos();
        let playing = false;

        let looped = false;
        let random = false;
        let volume = 4.0;

        let tracks = vec![];

        Self { stream, sink, current_index, random_index, playlist, initialized, current_duration, playing, order_song, looped, random, volume, show_playlist: true, show_queue: false, show_current_data: true, tracks }
   }

    fn load_track_list(&mut self) {
        for (index, item) in self.playlist.iter().enumerate() {
            // ItemSong::new(item.id, &item.title, &item.artist);
            self.tracks.push(ItemSong::new(
                index,
                &item.title,
                &item.artist,
                item.cover_data.clone(),
                ));

        }
    }

    fn load_path(&self) -> Vec<PathBuf> {
        let files = json_manager::read_paths();
        return files;
    }

    fn load_song_queue(&mut self) {
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

    fn do_order_song(&mut self) {
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

    fn add_new_song(&mut self) {
        if let Some(files) = FileDialog::new()
            .add_filter("audio", &["mp3", "flac", "wav"])
            .set_directory("/")
            .pick_files() {
                json_manager::add_song_to_json(files);
                // self.load_song_queue();
                self.playlist.clear();
                self.load_song_queue();
                self.load_track_list();

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

    fn playback_music(&mut self) {
        if self.sink.is_paused() == true {
            self.sink.play();
            self.playing = true;
        } else if self.sink.is_paused() == false {
            self.sink.pause();
            self.playing = false;
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
        ctx.request_repaint();
        if self.initialized == false {
            self.load_song_queue();
            self.load_track_list();
            self.do_order_song();
            self.initialized = true;
        } 
        let mut clicked_index: Option<usize> = None;

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
                                let track = &mut self.tracks[i];
                                let action_before = track.action.take();
                                
                                let track_id = track.id;
                                if self.current_index == track_id {
                                    track.set_select(true);
                                    if self.playing {
                                        track.set_playing(true);
                                    } else {
                                        track.set_playing(false);
                                    }
                                } else {
                                    track.set_select(false);
                                    track.set_playing(false);
                                }
                                ui.horizontal(|ui| {
                                    ui.add(track);
                                });
                                
                                if let Some(action) = action_before {
                                    match action {
                                        ItemSongAction::Play => {
                                            clicked_index = Some(track_id);
                                        }
                                        ItemSongAction::MoveToTitle => {
                                            println!("Move to title")
                                        }
                                        ItemSongAction::MoveToArtist => {
                                            println!("Move to artist")
                                        }
                                    }
                                }
                            }
                        }

                        // if !self.show_queue {
                        //     for i in self.order_song {
                        //         let track = &mut self.tracks[i];
                        //         let action_before = track.action.take();
                                
                        //         let track_id = track.id;
                        //         if self.current_index == track_id {
                        //             track.set_select(true);
                        //             if self.playing {
                        //                 track.set_playing(true);
                        //             } else {
                        //                 track.set_playing(false);
                        //             }
                        //         } else {
                        //             track.set_select(false);
                        //             track.set_playing(false);
                        //         }
                        //         ui.horizontal(|ui| {
                        //             ui.add(track);
                        //         });
                        //     }
                        // }
                            
                    
                    if !self.show_queue {
                        if self.order_song.len() > 0 {
                            for index_song in self.order_song.clone() {
                                let data = &self.tracks[index_song];
                                ui.horizontal(|ui| { 
                                        if ui.add_sized([64.0, 64.0], egui::Button::new("song").corner_radius(10)).clicked() {
                                            clicked_index = Some(index_song);
                                        }
                                        ui.vertical(|ui| {
                                            if ui.label(data.title.clone()).clicked() {
                                                println!("{}", data.title.clone())
                                            };
                                            ui.label(data.artist.clone());
                                        });
                                    });
        
                                }
                        } else {
                            ui.label("No song");
                        }
                    }
                });

            });
        });

        if let Some(index) = clicked_index {
            if index != self.current_index {
                self.current_index = index;
                println!("current song: {}", self.current_index);
                self.play_current();
                self.do_order_song(); 
            } else {
                self.playback_music();
            }
            }
            
            egui::CentralPanel::default().show(ctx, |ui|{
                if self.show_current_data {
                    if !self.playlist.is_empty() {  
                        
                        ui.centered_and_justified(|ui| {
                            ui.vertical_centered_justified(|ui| {

                                let current_song = &mut self.playlist[self.current_index];
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
                            
                                        
                            ui.vertical_centered(|ui| {
                                ui.label(self.playlist[self.current_index].title.clone());
                                ui.label(self.playlist[self.current_index].artist.clone());
                            });
                            ui.vertical_centered_justified(|ui| {

                                ui.horizontal_centered(|ui| {
                                    let max_duration = self.playlist[self.current_index].duration.as_secs_f32();
                                    self.current_duration = self.sink.get_pos();
                                    
                                    ui.with_layout(Layout::centered_and_justified(egui::Direction::TopDown) ,|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(self.current_duration.as_secs().to_string());
                                            if ui.add(egui::Slider::new(&mut self.current_duration.as_secs_f32(),
                                        0.0..=max_duration).show_value(false)).changed() {
                                            println!("duration: {}", self.current_duration.as_secs());
                                        }
                                        ui.label(max_duration.to_string());
                                    });
                                });
                                if self.playing == true {
                                    if self.sink.empty() == true {
                                        if self.looped == true {
                                            self.play_current();
                                        } else {
                                            self.next_song();
                                        }
                                        println!("song is finished");
                                    }
                                }
                            });
                            });
                        });
                        });
                    }
                }
            });
            
            egui::TopBottomPanel::bottom("PlaybackPanel").show(ctx, |ui| {
                ui.centered_and_justified(|ui| { 
                    ui.horizontal(|ui| {
                            // ui.add(Playback::new());

                            if ui.add_sized([16.0, 16.0], egui::Checkbox::new(&mut self.looped, "")).changed() {
                                println!("loop: {}", self.looped);
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
                            }});
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
    }
}
