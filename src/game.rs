//! Window-sized game shell; gameplay systems will be added here.

use crate::ui::{self, UiAction};
use macroquad::prelude::*;

pub struct Game {
    #[cfg(target_os = "windows")]
    pub exit_requested: bool,
    in_title: bool,
    settings_open: bool,
}

impl Game {
    pub async fn new() -> Self {
        Self {
            #[cfg(target_os = "windows")]
            exit_requested: false,
            in_title: true,
            settings_open: false,
        }
    }

    pub fn update(&mut self, _dt: f32) {
        if !self.in_title && is_key_pressed(KeyCode::Escape) {
            self.settings_open = !self.settings_open;
        }
    }

    pub fn draw(&mut self) {
        clear_background(ui::BACKGROUND);
        let action = if self.in_title {
            ui::draw_title()
        } else {
            ui::draw_game(self.settings_open)
        };
        match action {
            #[cfg(target_os = "windows")]
            Some(UiAction::Exit) => self.exit_requested = true,
            Some(UiAction::Start) => self.in_title = false,
            Some(UiAction::Settings) => self.settings_open = true,
            Some(UiAction::CloseSettings) => self.settings_open = false,
            Some(UiAction::Title) => {
                self.in_title = true;
                self.settings_open = false;
            }
            None => {}
        }
    }

    pub fn begin_capture_scene(&mut self, scene: &str) {
        (self.in_title, self.settings_open) = match scene {
            "title" => (true, false),
            "gameplay" => (false, false),
            "settings" => (false, true),
            other => panic!("Unknown capture scene: {other}"),
        };
    }
}
