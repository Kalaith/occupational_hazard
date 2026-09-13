//! Typed, validated balance data shared by simulation, review, and services.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::Path;

pub const CONFIG_PATH: &str = "assets/data/game_config.json";
pub const TEXTURE_MANIFEST_PATH: &str = "assets/data/texture_manifest.json";
pub const REQUIRED_TEXTURE_KEYS: [&str; 11] = [
    "mira",
    "elowen",
    "tomas",
    "pip",
    "building",
    "people",
    "route",
    "facilities",
    "activity",
    "destinations",
    "rest_beds",
];

#[derive(Clone, Debug, Deserialize)]
pub struct TextureManifest {
    pub entries: Vec<TextureEntry>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TextureEntry {
    pub key: String,
    pub path: String,
    #[serde(default)]
    pub chroma_key: Option<ChromaKeyConfig>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ChromaKeyConfig {
    pub color: [u8; 3],
    pub tolerance: u8,
    pub feather: u8,
}

impl TextureManifest {
    pub fn load() -> Result<Self, String> {
        Self::from_json(macroquad_toolkit::include_json_str!(
            "../assets/data/texture_manifest.json"
        ))
    }

    pub fn from_json(json: &str) -> Result<Self, String> {
        let entries: Vec<TextureEntry> =
            macroquad_toolkit::data_loader::parse_json_labeled(TEXTURE_MANIFEST_PATH, json)?;
        let manifest = Self { entries };
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn validate(&self) -> Result<(), String> {
        let mut keys = BTreeSet::new();
        let mut paths = BTreeSet::new();
        for entry in &self.entries {
            if entry.key.trim().is_empty()
                || entry.path.trim().is_empty()
                || !entry.path.starts_with("assets/")
                || !keys.insert(entry.key.clone())
                || !paths.insert(entry.path.clone())
            {
                return Err(format!(
                    "{TEXTURE_MANIFEST_PATH}: texture keys and asset references must be unique, non-empty assets paths."
                ));
            }
        }
        for required in REQUIRED_TEXTURE_KEYS {
            if !keys.contains(required) {
                return Err(format!(
                    "{TEXTURE_MANIFEST_PATH}: missing required texture key '{required}'."
                ));
            }
        }
        if self.entries.len() != REQUIRED_TEXTURE_KEYS.len() {
            return Err(format!(
                "{TEXTURE_MANIFEST_PATH}: unexpected texture key; expected {} entries.",
                REQUIRED_TEXTURE_KEYS.len()
            ));
        }
        Ok(())
    }

    pub fn validate_paths(&self, root: &Path) -> Result<(), String> {
        for entry in &self.entries {
            if !root.join(&entry.path).is_file() {
                return Err(format!(
                    "{TEXTURE_MANIFEST_PATH}: asset '{}' for key '{}' is missing.",
                    entry.path, entry.key
                ));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameConfig {
    pub starting: StartingResources,
    pub roster: Vec<RosterEntry>,
    pub progression: ProgressionConfig,
    pub review: ReviewConfig,
    pub services: ServicesConfig,
    pub expedition: ExpeditionConfig,
    pub caps: CapsConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StartingResources {
    pub day: u32,
    pub gold: u32,
    pub reputation: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RosterEntry {
    pub name: String,
    pub class: String,
    pub trait_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProgressionConfig {
    pub trial_xp: u32,
    pub trial_successes: u32,
    pub training_xp_cap: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReviewConfig {
    pub cutoff_day: u32,
    pub service_quota: usize,
    pub max_saved_reports: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServicesConfig {
    pub infirmary_cost: u32,
    pub training_yard_cost: u32,
    pub scout_cost: u32,
    pub scout_strength_bonus: i32,
    pub training_xp_per_day: u32,
    pub home_fatigue_recovery: u32,
    pub basic_injury_recovery: u32,
    pub infirmary_injury_recovery: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExpeditionConfig {
    pub base_strength: i32,
    pub xp_strength_step: u32,
    pub xp_strength_cap: u32,
    pub bronze_strength_bonus: i32,
    pub specialty_bonus: i32,
    pub mission_fatigue: u32,
    pub failed_xp: u32,
    pub standard_reputation: u32,
    pub promotion_reputation: u32,
    pub close_call_margin: i32,
    pub failure_injury: u32,
    pub healer_injury: u32,
    pub max_expedition_days: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CapsConfig {
    pub max_day: u32,
    pub max_gold: u32,
    pub max_reputation: u32,
    pub max_xp: u32,
    pub max_successes: u32,
    pub max_fatigue: u32,
    pub max_injury: u32,
}

impl GameConfig {
    pub fn load() -> Result<Self, String> {
        Self::from_json(macroquad_toolkit::include_json_str!(
            "../assets/data/game_config.json"
        ))
    }

    pub fn from_json(json: &str) -> Result<Self, String> {
        let config: Self = macroquad_toolkit::data_loader::parse_json_labeled(CONFIG_PATH, json)?;
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.roster.is_empty() {
            return Err("assets/data/game_config.json: roster must not be empty.".into());
        }
        let mut names = BTreeSet::new();
        for entry in &self.roster {
            if entry.name.trim().is_empty()
                || entry.class.trim().is_empty()
                || entry.trait_name.trim().is_empty()
                || !names.insert(&entry.name)
            {
                return Err(
                    "assets/data/game_config.json: roster names and fields must be unique and non-empty."
                        .into(),
                );
            }
        }
        if self.starting.day == 0
            || self.progression.trial_xp == 0
            || self.progression.trial_successes == 0
            || self.progression.training_xp_cap < self.progression.trial_xp
            || self.review.cutoff_day == 0
            || self.review.service_quota == 0
            || self.review.max_saved_reports == 0
        {
            return Err(
                "assets/data/game_config.json: starting and progression thresholds are invalid."
                    .into(),
            );
        }
        if self.services.scout_strength_bonus < 0
            || self.services.training_xp_per_day == 0
            || self.services.home_fatigue_recovery == 0
            || self.services.basic_injury_recovery == 0
            || self.services.infirmary_injury_recovery < self.services.basic_injury_recovery
        {
            return Err(
                "assets/data/game_config.json: service recovery values are invalid.".into(),
            );
        }
        if self.expedition.base_strength <= 0
            || self.expedition.xp_strength_step == 0
            || self.expedition.xp_strength_cap == 0
            || self.expedition.specialty_bonus < 0
            || self.expedition.mission_fatigue == 0
            || self.expedition.failed_xp == 0
            || self.expedition.standard_reputation == 0
            || self.expedition.promotion_reputation == 0
            || self.expedition.close_call_margin < 0
            || self.expedition.failure_injury == 0
            || self.expedition.healer_injury == 0
            || self.expedition.max_expedition_days == 0
        {
            return Err(
                "assets/data/game_config.json: expedition resolution values are invalid.".into(),
            );
        }
        if self.caps.max_day == 0
            || self.caps.max_gold == 0
            || self.caps.max_reputation == 0
            || self.caps.max_xp == 0
            || self.caps.max_successes == 0
            || self.caps.max_fatigue == 0
            || self.caps.max_injury == 0
        {
            return Err("assets/data/game_config.json: state caps must be positive.".into());
        }
        if self.expedition.failure_injury > self.caps.max_injury
            || self.expedition.healer_injury > self.caps.max_injury
        {
            return Err(
                "assets/data/game_config.json: injury outcomes exceed the injury cap.".into(),
            );
        }
        Ok(())
    }
}

impl Default for GameConfig {
    fn default() -> Self {
        Self::load().expect("Required occupational hazard game configuration")
    }
}
