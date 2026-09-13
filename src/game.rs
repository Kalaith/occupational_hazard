//! Runtime ownership and coordination between input, state, and presentation.

mod actions;
mod capture;

use crate::{
    contracts::{self, Contract},
    simulation::Guild,
    ui::{self, UiAction},
};
use macroquad::prelude::*;
use macroquad_toolkit::{assets::AssetManager, persistence};

pub struct Game {
    pub hq: crate::headquarters::Headquarters,
    #[cfg(target_os = "windows")]
    pub exit_requested: bool,
    pub in_title: bool,
    pub settings_open: bool,
    pub guild: Guild,
    pub contracts: Vec<Contract>,
    pub assets: AssetManager,
    pub selected: usize,
    pub party: Vec<usize>,
    pub tab: usize,
    pub dossier: usize,
    pub report: usize,
    pub report_detail: bool,
    pub report_page: usize,
    pub board_page: usize,
    pub notice: String,
    pub has_save: bool,
    pub confirm_new: bool,
    pub victory: bool,
    pub month_open: bool,
    pub confirm_day: bool,
    pub help_page: Option<usize>,
    capture: bool,
    pending: Option<UiAction>,
}

impl Game {
    pub async fn new() -> Self {
        let mut assets = AssetManager::new();
        assets.set_default_filter(FilterMode::Linear);
        for (key, path) in [
            ("building", "assets/headquarters/building-v2.png"),
            ("route", "assets/headquarters/route.png"),
            ("destinations", "assets/headquarters/destinations.png"),
            ("mira", "assets/portraits/mira.png"),
            ("tomas", "assets/portraits/tomas.png"),
            ("pip", "assets/portraits/pip.png"),
            ("elowen", "assets/portraits/elowen.png"),
        ] {
            assets
                .load_texture(key, path)
                .await
                .unwrap_or_else(|e| panic!("Required portrait {path}: {e}"));
        }
        assets
            .load_texture_keyed(
                "people",
                "assets/headquarters/people-v3-keyed.png",
                [255, 0, 255],
                90,
                75,
            )
            .await
            .expect("Required headquarters people atlas");
        assets
            .load_texture_keyed(
                "facilities",
                "assets/headquarters/facilities-keyed.png",
                [255, 0, 255],
                90,
                75,
            )
            .await
            .expect("Required headquarters facility atlas");
        assets
            .load_texture_keyed(
                "activity",
                "assets/headquarters/activity-v3-keyed.png",
                [255, 0, 255],
                90,
                75,
            )
            .await
            .expect("Required activity atlas");
        assets
            .load_texture_keyed(
                "rest_beds",
                "assets/headquarters/rest-beds-keyed.png",
                [255, 0, 255],
                90,
                75,
            )
            .await
            .expect("Required occupied beds");
        let preferences =
            persistence::load_from_slot::<(bool, bool)>("occupational_hazard", "preferences")
                .unwrap_or_default();
        macroquad_toolkit::ui::set_ui_text_scale(if preferences.1 { 1.15 } else { 1.0 });
        Self {
            hq: crate::headquarters::Headquarters {
                reduced_motion: preferences.0,
                large_text: preferences.1,
                ..Default::default()
            },
            #[cfg(target_os = "windows")]
            exit_requested: false,
            in_title: true,
            settings_open: false,
            guild: Guild::new(),
            contracts: contracts::load().expect("Required guild contracts"),
            assets,
            selected: 0,
            party: vec![],
            tab: 0,
            dossier: 0,
            report: 0,
            report_detail: false,
            report_page: 0,
            board_page: 0,
            notice: String::new(),
            has_save: persistence::slot_exists("occupational_hazard", "guild"),
            confirm_new: false,
            victory: false,
            month_open: false,
            confirm_day: false,
            help_page: None,
            capture: false,
            pending: None,
        }
    }

    pub fn update(&mut self, dt: f32) {
        actions::update(self, dt);
    }

    pub fn draw(&mut self) {
        let pointer = macroquad_toolkit::ui::Pointer::read(|p| p);
        if pointer.down {
            let start = *self.hq.pointer_start.get_or_insert(pointer.position);
            if start.distance(pointer.position) > 10. {
                self.hq.dragged = true;
            }
        }
        if pointer.released
            && self
                .hq
                .pointer_start
                .is_some_and(|start| start.distance(pointer.position) > 10.)
        {
            self.hq.dragged = true;
        }
        self.pending = ui::draw(self);
        if self.hq.dragged {
            self.pending = None;
        }
        if !pointer.down {
            self.hq.pointer_start = None;
            self.hq.dragged = false;
        }
    }

    pub fn lesson(&self) -> Option<crate::tutorial::Lesson> {
        self.guild.lesson(!self.party.is_empty())
    }

    pub fn begin_capture_scene(&mut self, scene_name: &str) {
        capture::begin(self, scene_name);
    }
}
