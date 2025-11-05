use std::{fs::File, io::BufReader, time::Duration, vec};

use rodio::{Decoder, OutputStreamBuilder};

pub struct Playback {
    pub stream: rodio::OutputStream,
    pub sink: rodio::Sink,
    pub current_index: usize,
    pub current_durration: Duration,
    pub playing: bool,
    pub random_index: usize,
    pub playlist: Vec<SongData>,
    pub order_song: Vec<usize>,
}

impl Playback {
    fn new() -> Self {
        let stream = OutputStreamBuilder::open_default_stream().unwrap();
        let sink = rodio::Sink::connect_new(stream.mixer());
        let current_durration = sink.get_pos();

        
        Self {
            stream,
            sink,
            current_index: 0,
            current_durration,
            playing: false,
            random_index: 0,
            playlist: vec![],
            order_song: vec![],
        }
    }

    fn play_current(&mut self, playlist: Vec<_>) {
        let path = &playlist[self.current_index].path;
        let file = File::open(path).expect("Errore to open music file");
    
        let source = Decoder::new(BufReader::new(file)).expect("Errore with a decoder");

        self.sink.stop();
        self.sink.append(source);
        self.sink.play();

        self.playing = true;
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