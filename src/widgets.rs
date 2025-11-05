use clap::builder::Str;
use eframe::egui::{self, *};
#[derive(Clone)]

pub struct ItemSong {
    pub id: usize,
    pub title: String,
    pub artist: String,
    pub action: Option<ItemSongAction>,
    pub is_playing: bool,
    pub selected: bool,
}
impl ItemSong {
    pub fn new(id: impl Into<usize>, title: impl Into<String>, artist: impl Into<String>) -> Self {
        Self { 
            id: id.into(),
            title: title.into(),
            artist: artist.into(),
            action: None,
            is_playing: false,
            selected: false
            }
    }
    pub fn set_playing(&mut self, status: bool) {
        self.is_playing = status;
    }
    pub fn set_select(&mut self, status: bool) {
        self.selected = status;
    }
}
#[derive(Clone)]
pub enum ItemSongAction {
    Play,
    MoveToTitle,
    MoveToArtist,
}

impl egui::Widget for &mut ItemSong {
    fn ui(self, ui: &mut Ui) -> Response {        
        let desired_size = vec2(ui.available_width(), 36.0);
        let (rect, _response) = ui.allocate_exact_size(desired_size, Sense::hover());

        let visuals = &ui.visuals().widgets;
        let bg_color = visuals.inactive.bg_fill;

        ui.painter().rect_filled(rect, 6.0, bg_color);

        let button_rect = Rect::from_min_size(rect.min, vec2(36.0, rect.height()));
    
        let text_area = Rect::from_min_max(pos2(button_rect.max.x + 8.0, rect.min.y), rect.max);
        
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

        ui.painter().rect_filled(button_rect, 4.0, Color32::GRAY);
        if self.is_playing == false {
            ui.painter().text(
                button_rect.center(),
                Align2::CENTER_CENTER,
                "▶",
                FontId::proportional(18.0),
                Color32::WHITE,
            );
        } else {
            ui.painter().text(
                button_rect.center(),
                Align2::CENTER_CENTER,
                "⏸",
                FontId::proportional(18.0),
                Color32::WHITE,
            );
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