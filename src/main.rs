use std::{fs::{self, File}, io::{BufReader, Write}, path::PathBuf, ptr::null, time::Duration, usize};

use eframe::{egui::{self, Button, CentralPanel, ColorImage, CornerRadius, Image, Layout, TextureHandle, TextureOptions, Vec2}, epaint::tessellator::Path, glow::{MAX_DUAL_SOURCE_DRAW_BUFFERS, PRIMITIVES_SUBMITTED}};

use image::{ImageFormat, ImageReader};
use lofty::{file::{AudioFile, TaggedFileExt}, picture::{MimeType, PictureType}, probe, read_from_path, tag::ItemKey};
use rodio::{queue, Decoder, OutputStream, OutputStreamBuilder, Sink};

use serde_json::from_str;

use rand::seq::{SliceRandom};
use rand::rng;

mod widgets;
use widgets::{ItemSong, ItemSongGrid, ItemSongAction, Playback}; 

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
    durration: Duration,
    path: PathBuf,
    texture: Option<egui::TextureHandle>,
}
struct MyApp {
    stream: rodio::OutputStream,
    sink: rodio::Sink,

    current_index: usize,
    random_index: usize,
    playlist: Vec<SongData>,
    order_song: Vec<usize>,
    initialized: bool,

    current_durration: Duration,
    playing: bool,

    loopped: bool,
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

        let current_durration = sink.get_pos();
        let playing = false;

        let loopped = false;
        let random = false;
        let volume = 4.0;

        let tracks = vec![];

        Self { stream, sink, current_index, random_index, playlist, initialized, current_durration, playing, order_song, loopped, random, volume, show_playlist: true, show_queue: false, show_current_data: true, tracks }
   }

    fn load_track_list(&mut self) {
        for (index, item) in self.playlist.iter().enumerate() {
            // ItemSong::new(item.id, &item.title, &item.artist);
            self.tracks.push(ItemSong::new(index, &item.title, &item.artist, item.texture.clone()));

        }
    }

    fn load_path(&self) -> Vec<PathBuf> {
        let files = PathBuf::from(r"C:\\Users\\User\\Music\\music");
        let mut paths: Vec<PathBuf> = vec![];
        for song in fs::read_dir(files).unwrap() {
            let song_entry = song.unwrap();
            let path = song_entry.path();
            paths.push(path);
        }
        return paths;
    }

    fn load_song_queue(&mut self, ctx: &egui::Context) {
        for song in self.load_path().clone() {

            let tagged_file_result = read_from_path(&song);
            if let Ok(tagged_file) = tagged_file_result {
                if let Some(tag) = tagged_file.primary_tag() {
                    let title: String = tag.get_string(&ItemKey::TrackTitle).unwrap_or("Unknow Title").to_string();                        
                    let album: String = tag.get_string(&ItemKey::AlbumTitle).unwrap_or("Unknow Album").to_string();  
                    let album_artist: String = tag.get_string(&ItemKey::AlbumArtist).unwrap_or("Unknow AlbumArtist").to_string();                      
                    let artist: String = tag.get_string(&ItemKey::TrackArtist).unwrap_or("Unknow Artist").to_string();

                    let durration = tagged_file.properties().duration();

                    let pic = tag.pictures().iter()
                    .find(|p| p.pic_type() == PictureType::CoverFront);

                    // let texture = if let Some(pic) = tag.pictures().iter()
                    // .find(|p| p.pic_type() == PictureType::CoverFront) {

                        
                    //     let bytes = pic.data();
                        
                    //     let mime = pic.mime_type();
                    //     let format = match pic.mime_type() {
                    //         Some(MimeType::Jpeg) => ImageFormat::Jpeg,
                    //         Some(MimeType::Png)  => ImageFormat::Png,
                    //         _ => ImageFormat::Png,
                    //     };
                        
                    //     println!("{:?}, {:?}", mime, format);
                        
                    //     if let Ok(img) = image::load_from_memory(bytes) {
                    //         println!("ok");
                    //         let img = img.to_rgba8();
                    //         let size = [img.width() as usize, img.height() as usize];
                    //         let pixels = img.into_raw();

                    //         Some(ctx.load_texture(
                    //         "cover",
                    //         egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
                    //         Default::default(),
                    //     ))

                    //     } else {
                    //         None
                    //     }
                    // } else {
                    //     None
                    // };

                        

                    let song_data = SongData {
                        title,
                        artist,
                        album: album.clone(),
                        album_artist,
                        durration,
                        path: song.clone(),
                        texture: None,
                    };
                    self.playlist.push(song_data);
                }
            }
        }
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
        ctx.request_repaint();
        if self.initialized == false {
            self.load_song_queue(ctx);
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
                        self.show_current_data = !self.show_current_data;
                    }
                });
            });

            egui::ScrollArea::vertical().show(ui, |ui| {
                if self.show_playlist {
                        for track in &mut self.tracks {
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

                            ui.add(track);

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

                    // for (index, song_data_item) in self.playlist.iter_mut().enumerate() {
                    //     ui.horizontal(|ui| { 
                    //         if ui.add_sized([64.0, 64.0], egui::Button::new("song").corner_radius(10)).clicked() {
                    //             clicked_index = Some(index);
                    //         }
                    //         ui.vertical(|ui| {
                    //             ui.label(&song_data_item.title);
                    //             ui.label(&song_data_item.artist);
                    //         });
                    //     });
                    // }
                }
                
                if self.show_queue {
                    if self.order_song.len() > 0 {
                        for index_song in self.order_song.clone() {
                                let data = &self.playlist[index_song];
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

                if let Some(tex) = &self.playlist[self.current_index].texture {
                    
                    let width = 200.0;
                    let height = 200.0; // высота автоматически
                    ui.add(
                        egui::Image::new(tex)
                        .fit_to_exact_size(egui::Vec2::new(width, height))
                    );
            }

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
            } else {
                // ui.add(&mut ItemSongGrid::new(self.current_index, "title", "aritst"));
            }

            egui::TopBottomPanel::bottom("PlaybackPanel").show(ctx, |ui| {
                ui.centered_and_justified(|ui| { 
                        ui.horizontal(|ui| {
                            // ui.add(Playback::new());

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

