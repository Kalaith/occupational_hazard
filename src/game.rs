//! Runtime ownership and action handling.
use crate::{
    contracts::{self, Contract},
    simulation::Guild,
    ui::{self, UiAction},
};
use macroquad::prelude::*;
use macroquad_toolkit::{assets::AssetManager, persistence};

pub struct Game {
    #[cfg(target_os = "windows")]
    pub exit_requested: bool,
    pub in_title: bool,
    pub settings_open: bool,
    pub guild: Guild,
    pub contracts: Vec<Contract>,
    pub assets: AssetManager,
    pub selected: usize,
    pub choosing_party: bool,
    pub party: Vec<usize>,
    pub tab: usize,
    pub dossier: usize,
    pub report: usize,
    pub notice: String,
    pub has_save: bool,
    pub confirm_new: bool,
    pub victory: bool,
    capture: bool,
    pending: Option<UiAction>,
}

impl Game {
    pub async fn new() -> Self {
        let mut assets = AssetManager::new();
        assets.set_default_filter(FilterMode::Linear);
        for (key, path) in [
            ("mira", "assets/portraits/mira.png"),
            ("elowen", "assets/portraits/elowen.png"),
        ] {
            assets
                .load_texture(key, path)
                .await
                .unwrap_or_else(|e| panic!("Required portrait {path}: {e}"));
        }
        Self {
            #[cfg(target_os = "windows")]
            exit_requested: false,
            in_title: true,
            settings_open: false,
            guild: Guild::new(),
            contracts: contracts::load().expect("Required guild contracts"),
            assets,
            selected: 0,
            choosing_party: false,
            party: vec![],
            tab: 0,
            dossier: 0,
            report: 0,
            notice: String::new(),
            has_save: persistence::slot_exists("occupational_hazard", "guild"),
            confirm_new: false,
            victory: false,
            capture: false,
            pending: None,
        }
    }

    pub fn update(&mut self, _dt: f32) {
        if !self.in_title && is_key_pressed(KeyCode::Escape) {
            self.settings_open = !self.settings_open;
        }
        let Some(action) = self.pending.take() else {
            return;
        };
        match action {
            UiAction::ChooseParty(value) => self.choosing_party = value,
            #[cfg(target_os = "windows")]
            UiAction::Exit => self.exit_requested = true,
            UiAction::Start => {
                if self.has_save && !self.confirm_new {
                    self.confirm_new = true;
                    return;
                }
                self.guild = Guild::new();
                self.victory = false;
                self.choosing_party = false;
                self.dossier = 0;
                self.report = 0;
                self.party.clear();
                self.selected = 0;
                self.tab = 0;
                self.in_title = false;
                self.confirm_new = false;
                self.notice.clear();
                self.save();
            }
            UiAction::Cancel => self.confirm_new = false,
            UiAction::Continue => {
                match persistence::load_from_slot::<Guild>("occupational_hazard", "guild") {
                    Ok(guild) => match guild.validate(&self.contracts) {
                        Ok(()) => {
                            self.guild = guild;
                            self.in_title = false;
                            self.notice.clear();
                        }
                        Err(e) => self.notice = format!("Cannot open ledger: {e}"),
                    },
                    Err(e) => self.notice = format!("Cannot open ledger: {e}"),
                }
            }
            UiAction::Settings => self.settings_open = true,
            UiAction::CloseSettings => self.settings_open = false,
            UiAction::Title => {
                self.in_title = true;
                self.settings_open = false;
            }
            UiAction::Tab(tab) => self.tab = tab,
            UiAction::Quest(id) => {
                self.selected = id;
                self.notice.clear();
            }
            UiAction::Party(id) => {
                if self.party.contains(&id) {
                    self.party.retain(|&a| a != id);
                } else if !self.guild.busy(id) && self.guild.roster[id].injury == 0 {
                    self.party.push(id);
                }
                self.notice.clear();
            }
            UiAction::Dispatch => {
                match self
                    .guild
                    .dispatch(self.selected, &self.party, &self.contracts)
                {
                    Ok(()) => {
                        self.notice = "Dispatched. Tap NEXT DAY to advance their journey.".into();
                        self.party.clear();
                        self.save();
                    }
                    Err(e) => self.notice = e,
                }
            }
            UiAction::NextDay => {
                let returning = self
                    .guild
                    .expeditions
                    .iter()
                    .any(|e| e.returns == self.guild.day + 1);
                self.guild.next_day(&self.contracts);
                self.notice = if returning {
                    "Adventurers returned. Their reports are on your desk."
                } else {
                    "A new day. Adventurers at the guild have rested."
                }
                .into();
                if returning {
                    self.tab = 2;
                    self.report = 0;
                }
                self.save();
            }
            UiAction::Dossier(id) => self.dossier = id,
            UiAction::Report(id) => self.report = id,
            UiAction::Promote(id) => match self.guild.promote(id) {
                Ok(()) => {
                    self.victory = !self.guild.victory_seen;
                    self.guild.victory_seen = true;
                    self.notice =
                        "Bronze certification signed. North Bridge is now available.".into();
                    self.save();
                }
                Err(e) => self.notice = e,
            },
            UiAction::CloseVictory => self.victory = false,
            UiAction::Save => self.save(),
        }
    }

    fn save(&mut self) {
        if self.capture {
            return;
        }
        match persistence::save_to_slot("occupational_hazard", "guild", &self.guild) {
            Ok(()) => {
                self.has_save = true;
            }
            Err(e) => {
                self.notice =
                    format!("Ledger could not be saved: {e}. Tap MENU then SAVE to retry.")
            }
        }
    }

    pub fn draw(&mut self) {
        self.pending = ui::draw(self);
    }

    pub fn hint(&self) -> &str {
        if !self.guild.expeditions.is_empty() {
            return "The road takes time. Tap NEXT DAY to advance expeditions; resting staff recover.";
        }
        if self
            .guild
            .roster
            .iter()
            .any(|a| a.trial_passed && !a.bronze)
        {
            return "A recommendation has arrived! Tap ADVENTURERS, select the candidate, then APPROVE BRONZE.";
        }
        if self.guild.roster.iter().any(|a| a.bronze) {
            return "Your first Bronze licence! Try A Bridge Worth Keeping, or develop another Iron recruit.";
        }
        if self.guild.roster.iter().any(|a| a.eligible()) {
            return "A candidate is ready. Select The Lantern Road Trial and that candidate alone. Rest them first.";
        }
        "Tap a contract, then adventurers, then DISPATCH. Earn 60 XP and 3 successes to qualify for the Bronze trial."
    }

    pub fn begin_capture_scene(&mut self, scene: &str) {
        self.capture = true;
        self.guild = Guild::new();
        self.party = vec![0];
        self.has_save = false;
        self.in_title = scene == "title";
        self.settings_open = scene == "settings";
        self.tab = 0;
        self.victory = false;
        self.notice.clear();
        self.pending = None;
        self.selected = if scene == "mobile_contract" { 4 } else { 0 };
        self.choosing_party = scene == "mobile_party";
        if scene == "promotion" || scene == "mobile_promotion" {
            let a = &mut self.guild.roster[0];
            a.xp = 90;
            a.successes = 4;
            a.trial_passed = true;
            self.tab = 1;
        }
        if scene == "report" {
            self.guild.dispatch(0, &[0], &self.contracts).unwrap();
            self.guild.next_day(&self.contracts);
            self.tab = 2;
        }
        if scene == "victory" {
            self.victory = true;
        }
    }
}
