use std::{io::sink, option};
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStreamBuilder, Sink};

use eframe::egui;

// mod charge_songs;

fn main() -> Result<(), eframe::Error> {
    let option = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size([800.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Моё первое окно",
        option,
        Box::new(|_cc| Ok(Box::new(MyApp::new())))
    )
}

struct MyApp {
    stream: rodio::OutputStream,
    sink: Sink,
    volume: f32
}




impl MyApp {
    fn new() -> Self {
        let stream = OutputStreamBuilder::open_default_stream().unwrap();
        let sink = Sink::connect_new(stream.mixer());
        let volume = 50.0;
        Self { stream, sink, volume }
    }

    fn play_music(&self) {
        let path = r"C:\Users\User\Music\music\Gruppa Skryptonite - Снегопад.mp3";

        let file = File::open(path).expect("Не удалось открыть файл");
        let source = Decoder::new(BufReader::new(file)).expect("Не удалось декодировать");

        self.sink.stop();
        println!("song is stoped");
        self.sink.append(source);
        println!("song is appended");
        self.sink.play();
        println!("song is playing");
    }

    fn playback_music(&self) {
        if self.sink.is_paused() == true {
            self.sink.play();
        } else {
            self.sink.pause();
        }
    }

    fn back_music(&self) {
        println!("back")
    }

    fn next_music(&self) {
        self.sink.skip_one();
        println!("next")
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::left("song_list")
        .resizable(false)
        .min_width(250.0)
        .default_width(250.0)
        .show(ctx,|ui| {
            ui.horizontal(|ui| {
                if ui.button("button").clicked() {
                    self.play_music();
                }
    
                ui.vertical(|ui| {
                    ui.label("title");
                    ui.label("artist");
                });
            });
        });

        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Heading");
            egui::TopBottomPanel::bottom("playback").show(ctx, |ui| {
                
                
                ui.horizontal_centered(|ui| {
                    ui.horizontal(|ui| {
                        if ui.button("back").clicked() {
                            self.back_music();
                        }
                        
                        if ui.button("play").clicked() {
                            println!("play or pause clicked");
                            self.playback_music();
                        }
                        
                        if ui.button("next").clicked() {
                            self.next_music();
                        }
                    });

                    ui.add(egui::Slider::new(&mut self.volume, 0.0..=100.0)
                        .vertical()
                        .show_value(false)
                    );

                });
            });
        });
    }
}

