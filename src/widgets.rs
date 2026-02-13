use std::rc::Rc;

use eframe::egui::{self, *};
use lofty::picture::MimeType;
use rodio::source::Function;

// use crate::player::Player;

#[derive(Clone)]
pub enum ItemSongAction {
    Play,
    MoveToTitle,
    MoveToArtist,
}

// pub struct ItemSongGrid {
//     pub id: usize,
//     pub title: String,
//     pub artist: String,
//     pub action: Option<ItemSongAction>,
//     pub is_playing: bool,
//     pub selected: bool,
// }

// impl ItemSongGrid {
//     pub fn new(id: impl Into<usize>, title: impl Into<String>, artist: impl Into<String>, image: impl Into<TextureHandle>) -> Self {
//         Self { 
//             id: id.into(),
//             title: title.into(),
//             artist: artist.into(),
//             action: None,
//             is_playing: false,
//             selected: false,
//             }
//     }
//     pub fn set_playing(&mut self, status: bool) {
//         self.is_playing = status;
//     }
//     pub fn set_select(&mut self, status: bool) {
//         self.selected = status;
//     }
// }

// impl egui::Widget for &mut ItemSongGrid {
//     fn ui(self, ui: &mut Ui) -> Response {
//         let desired_size = vec2(95.0, 150.0);
//         let (rect, _response) = ui.allocate_exact_size(desired_size, Sense::click());
        
//         let button_rect = Rect::from_min_size(rect.min, vec2(rect.width(), 95.0));

        
//         let title_rect = Rect::from_min_max(pos2(button_rect.min.x, button_rect.max.y), rect.max / 2.0);
//         let artist_rect = Rect::from_min_max(pos2(title_rect.min.x, title_rect.max.y), rect.max);

//         ui.painter().rect_filled(rect, 6.0, Color32::GRAY);
//         ui.painter().rect_filled(button_rect, 4.0, Color32::BROWN);

//         ui.painter().text(
//             title_rect.min + vec2(0.0, 0.0),
//             Align2::LEFT_TOP,
//             &self.title,
//             FontId::proportional(16.0),
//             Color32::WHITE,
//             );

//         ui.painter().text(
//             artist_rect.min + vec2(0.0, 0.0),
//             Align2::LEFT_TOP,
//             &self.artist,
//             FontId::proportional(14.0),
//             Color32::WHITE,
//             );

//         _response
//     }
// }


pub struct ItemSong {
    pub id: usize,
    pub title: String,
    pub artist: String,
    pub action: Option<ItemSongAction>,
    pub is_playing: bool,
    pub selected: bool,

    pub texture: Option<egui::TextureHandle>,
    pub cover_data: Option<Vec<u8>>,
    pub cover_loaded: bool,

    // pub on_play: Rc<dyn FnMut(usize)>
}
impl ItemSong {
    pub fn new(
        id: impl Into<usize>,
        title: impl Into<String>,
        artist: impl Into<String>,
        cover_data: Option<Vec<u8>>,
        // on_play: Rc<dyn FnMut(usize)>,
    ) -> Self {

        Self { 
            id: id.into(),
            title: title.into(),
            artist: artist.into(),
            action: None,
            is_playing: false,
            selected: false,
            
            texture: None,
            cover_data,
            cover_loaded: false,

            // on_play,
            }
    }
    pub fn set_playing(&mut self, status: bool) {
        self.is_playing = status;
    }
    pub fn set_select(&mut self, status: bool) {
        self.selected = status;
    }
}


impl egui::Widget for &mut ItemSong {
    fn ui(self, ui: &mut Ui) -> Response {
        let width = 46.0;
        let desired_size = vec2(ui.available_width(), width);
        let (rect, _response) = ui.allocate_exact_size(desired_size, Sense::click());

        let button_rect = Rect::from_min_size(rect.min, vec2(width, rect.height()));
        
        let text_area = Rect::from_min_max(pos2(width + 15.0, rect.min.y), rect.max);
        
        let text_height = text_area.height() / 2.0;
        let title_rect = Rect::from_min_max(text_area.min, pos2(text_area.max.x, text_area.min.y + text_height));
        let artist_rect = Rect::from_min_max(pos2(text_area.min.x, text_area.min.y + text_height), text_area.max);
        
        let button_resp = ui.interact(button_rect, ui.id().with(&self.id), Sense::click());
        if button_resp.clicked() {
            self.action = Some(ItemSongAction::Play);
        }

        let id_title: String = self.title.clone() + &self.id.clone().to_string(); 
        let title_resp = ui.interact(title_rect, ui.id().with(id_title), Sense::click());
        if title_resp.clicked() {
            self.action = Some(ItemSongAction::MoveToTitle)
        };
        

        let id_artist: String = self.artist.clone() + &self.id.clone().to_string();
        let artist_resp = ui.interact(artist_rect, ui.id().with(id_artist), Sense::click());
        if artist_resp.clicked() {
            self.action = Some(ItemSongAction::MoveToArtist)
        }

        let visuals = &ui.visuals().widgets;
        let mut bg_color = visuals.inactive.bg_fill;

        if _response.hovered() {
            bg_color = visuals.hovered.bg_fill;
        } else if button_resp.hovered() {
            bg_color = visuals.hovered.bg_fill;
        } else if title_resp.hovered() {
            bg_color = visuals.hovered.bg_fill;
        } else if artist_resp.hovered() {
            bg_color = visuals.hovered.bg_fill;
        };

        ui.painter().rect_filled(rect, 6.0, bg_color);

        if self.texture.is_none() && self.cover_data.is_some() && !self.cover_loaded {
            if let Some(bytes) = &self.cover_data {
                if let Ok(mut tex) = image::load_from_memory(bytes) {
                    tex = tex.resize(128, 128, image::imageops::FilterType::Triangle);
                    let rgba = tex.to_rgba8();
                    let size = [rgba.width() as usize, rgba.height() as usize];
                    let pixels = rgba.into_raw();

                    let texture = ui.ctx().load_texture(
                        format!("cover_{}", self.id),
                        egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
                        egui::TextureOptions::default()
                    );
                    self.texture = Some(texture);
                }
            }
            self.cover_loaded = true;
            println!("{}, {}", self.id, self.cover_loaded);
        };

        if let Some(texture) = &self.texture {
            ui.put(
                button_rect,
                Image::new(texture).fit_to_exact_size(Vec2::new(button_rect.width(), button_rect.height()))
            );
        } else if self.cover_data.is_none() {
            ui.painter().rect_filled(button_rect, 8.0, Color32::DARK_GRAY);
            ui.painter().text(
                button_rect.center(),
                Align2::CENTER_CENTER,
                "🎵",
                FontId::proportional(30.0),
                Color32::LIGHT_GRAY,
            );
        } else {
            println!("loading");
            ui.painter().rect_filled(button_rect, 8.0, Color32::DARK_GRAY);
            ui.spinner();
        }

        if self.selected == false {
            ui.painter().text(
                title_rect.min + vec2(0.0, 2.0),
                Align2::LEFT_TOP,
                &self.title,
                FontId::proportional(16.0),
                Color32::WHITE,
            );
        } else {
            ui.painter().text(
            title_rect.min + vec2(0.0, 2.0),
            Align2::LEFT_TOP,
            &self.title,
            FontId::proportional(16.0),
            Color32::RED,
        );
        }

        ui.painter().text(
            artist_rect.min + vec2(0.0, 2.0),
            Align2::LEFT_TOP,
            &self.artist,
            FontId::proportional(14.0),
            Color32::GRAY,
        );
        
        
        _response
    }
    
}


pub struct Playback {
    pub is_playing: bool,
    pub is_looping: bool,
    pub is_shuffing: bool,
}

impl Playback {
    pub fn new() -> Self {
        Self {
            is_playing: false,
            is_looping: false,
            is_shuffing: false,
        }
    }
}

impl egui::Widget for Playback {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = vec2(ui.available_width(), 40.0);
        let (_rect, _response) = ui.allocate_exact_size(desired_size, Sense::hover());
        
        let visuals = &ui.visuals().widgets;
        let bg_color = visuals.inactive.bg_fill;

        let loopper_button_rect = Rect::from_min_size(_rect.min, vec2(45.0, _rect.height()));

        let playback_area = Rect::from_min_max(pos2(loopper_button_rect.max.x, _rect.min.y), pos2(-45.0 + _rect.max.x, _rect.max.y));
        let with_playback_element: f32 = playback_area.width() / 3.0;

        let prev_button_rect = Rect::from_min_size(playback_area.min, vec2(with_playback_element, _rect.height()));
        // let play_button_rect = Rect::from_min_size(prev_button_rect.min, vec2(with_playback_element, _rect.height()));
        // let next_button_rect = Rect::from_min_size(pos2(playback_area.max.x, playback_area.min.y), vec2(with_playback_element, _rect.height()));

        let shuffer_button_rect = Rect::from_min_size(pos2(playback_area.max.x, _rect.min.y), vec2(45.0, _rect.height()));

        
        ui.painter().rect_filled(_rect, 6, bg_color);
        ui.painter().rect_filled(playback_area, 0, Color32::BLUE);
        ui.painter().text(
            loopper_button_rect.center(),
            Align2::CENTER_CENTER,
            "🔁",
            FontId::proportional(18.0),
            Color32::WHITE);


        ui.painter().text(
            prev_button_rect.center(),
            Align2::CENTER_CENTER,
            "⏮",
            FontId::proportional(18.0),
            Color32::WHITE);

        // ui.painter().text(
        //     play_button_rect.center(),
        //     Align2::CENTER_CENTER,
        //     "▶",
        //     FontId::proportional(18.0),
        //     Color32::WHITE);

        // ui.painter().text(
        //     next_button_rect.center(),
        //     Align2::CENTER_CENTER,
        //     "⏭",
        //     FontId::proportional(18.0),
        //     Color32::WHITE);


        ui.painter().text(
            shuffer_button_rect.center(),
            Align2::CENTER_CENTER,
            "🔀",
            FontId::proportional(18.0),
            Color32::WHITE);
        

        _response
    }

    
}

pub struct Current_song_data {
    pub cover_texture: Option<TextureHandle>,
    pub size_current_cover: f32,
    pub title: String,
    pub artist: String,
}

impl Current_song_data {
    pub fn new(
        title: impl Into<String>,
        artist: impl Into<String>,
        ) -> Self {
        Self {
            cover_texture: None,
            size_current_cover: 250.0,
            title: title.into(),
            artist: artist.into(),
        }
    }
}

impl egui::Widget for Current_song_data {
    fn ui(self, ui: &mut Ui) -> Response {

        let desired_size = vec2(ui.available_width(), ui.available_width());
        let (rect, _response) = ui.allocate_exact_size(desired_size, Sense::click());
        
        let visuals = &ui.visuals().widgets;
        let mut bg_color = visuals.inactive.bg_fill;

        let size_current_cover: f32 = 250.0;

        let cover_rect = Rect::from_center_size(rect.center(), vec2(size_current_cover, size_current_cover));


        // match &self.cover_texture {
        //     Some(cover_texture) => {
        //         ui.add(
        //             egui::Image::new(cover_texture)
        //             .fit_to_exact_size(egui::Vec2::new(size_current_cover, size_current_cover))
        //             );
        //         }
        //     None => {
        //         ui.add_sized([size_current_cover, size_current_cover], egui::Button::new("🎵").corner_radius(10));
        //         }
        //     }

        // ui.add_sized([size_current_cover, size_current_cover], egui::Button::new("🎵").corner_radius(10));
        // ui.vertical(|ui| {
        //     ui.label("TItle");
        //     ui.label("Artist");
        // });

        // ui.with_layout(Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
        //     ui.horizontal(|ui| {
        //         ui.label("00:00");
        //         if ui.add(egui::Slider::new(&mut 1.0,
        //             0.0..=10.0).show_value(false)).changed() {
        //             }
        //         ui.label("00:00");
        //     });
        // });
        ui.painter().rect_filled(rect, 6, bg_color);
        ui.painter().rect_filled(cover_rect, 8.0, Color32::DARK_GRAY);
        ui.painter().text(
            cover_rect.center(),
            Align2::CENTER_CENTER,
            "🎵",
            FontId::proportional(30.0),
            Color32::LIGHT_GRAY,
            );
        _response
    }
}