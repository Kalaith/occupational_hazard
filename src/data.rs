//! Typed, validated balance data shared by simulation, review, and services.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::Path;

pub const CONFIG_PATH: &str = "assets/data/game_config.json";
pub const TEXT_PATH: &str = "assets/data/text.json";
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

pub const REQUIRED_TEXT_KEYS: &[&str] = &[
    "title.name",
    "title.tagline",
    "title.description",
    "title.confirm_new",
    "title.start_new",
    "title.keep",
    "title.new",
    "title.continue",
    "title.exit",
    "menu.heading",
    "menu.save_note",
    "menu.save",
    "menu.help",
    "menu.return",
    "menu.motion_reduced",
    "menu.motion_full",
    "menu.close",
    "menu.text_large",
    "menu.text_standard",
    "room.common",
    "room.assignments",
    "room.recovery",
    "room.records",
    "room.departure",
    "room.training",
    "activity.away",
    "activity.medical",
    "activity.resting",
    "activity.training",
    "activity.ready",
    "rank.bronze",
    "rank.iron",
    "ui.overview",
    "ui.rooms",
    "ui.back_offers",
    "ui.back_guild",
    "ui.headquarters_rooms",
    "ui.commission_board",
    "ui.guild_commission",
    "ui.expedition_journal",
    "ui.guild_register",
    "ui.headquarters_improvement",
    "ui.advance_day",
    "ui.day_gold",
    "ui.sandbox",
    "ui.review",
    "ui.review_in",
    "ui.menu",
    "ui.new_day",
    "ui.dispatched",
    "ui.return_reports",
    "ui.return_unread",
    "ui.journey_card",
    "ui.open_jobs",
    "ui.person_caption",
    "ui.injured",
    "ui.party_departing",
    "ui.skip_motion",
    "ui.day_warning_heading",
    "ui.day_warning_body",
    "ui.back_headquarters",
    "ui.compare_commissions",
    "ui.no_postings",
    "ui.expires_day",
    "ui.standing_trial",
    "ui.previous_offers",
    "ui.more_offers",
    "ui.danger",
    "ui.commission_stats",
    "ui.member_status",
    "ui.member_fatigue",
    "ui.phone_contract_stats",
    "ui.return_timing",
    "ui.member_contribution",
    "ui.fatigue_loss",
    "ui.contributes",
    "ui.career_heading",
    "ui.career_identity",
    "ui.purchase_cost",
    "ui.quest_stats",
    "ui.offer_terms",
    "ui.career_stats",
    "ui.mark_passed",
    "ui.mark_required",
    "ui.career_bronze",
    "ui.career_passed",
    "ui.career_eligible",
    "ui.career_build",
    "ui.latest_report",
    "ui.approve_bronze",
    "ui.prepare_trial",
    "ui.recovery_room",
    "ui.training_yard",
    "ui.facility_infirmary_open",
    "ui.facility_training_open",
    "ui.facility_recovery_short",
    "ui.facility_training_short",
    "ui.facility_recovery_long",
    "ui.facility_training_long",
    "ui.here_now",
    "ui.no_staff",
    "ui.facility_open",
    "ui.purchase",
    "ui.need",
    "ui.compare_phone",
    "ui.party",
    "ui.contract",
    "ui.dispatch_party",
    "ui.acceptance_terms",
    "ui.solo_trial",
    "ui.bronze",
    "ui.iron",
    "ui.accept_by_day",
    "ui.before_review",
    "ui.after_review",
    "ui.choose_party",
    "ui.readiness_dispatch",
    "ui.back_contract",
    "ui.unaided_trial",
    "ui.scouted",
    "ui.scout_needs",
    "ui.scout_strength",
    "ui.away_cannot_join",
    "ui.medical_days",
    "ui.fit_strength",
    "ui.support_strength",
    "ui.well_prepared",
    "ui.close_call",
    "ui.outmatched",
    "ui.on_the_road",
    "ui.on_the_road_label",
    "ui.travelling_party",
    "ui.choose_your_party",
    "ui.return_day",
    "ui.preparation_fixed",
    "ui.preparation_unchanged",
    "ui.add_member",
    "ui.member_detail",
    "ui.preparation_summary",
    "ui.trial_readiness",
    "ui.full_party_rest",
    "ui.scouting_included",
    "ui.trial_unaided",
    "ui.scouting_adds",
    "ui.journeys_returns",
    "ui.unread_reports",
    "ui.expedition_entry",
    "ui.report_entry",
    "ui.report_read",
    "ui.report_new",
    "ui.no_returns",
    "ui.more_reports",
    "ui.party_returned",
    "ui.return_account",
    "ui.rewards",
    "ui.people",
    "ui.acknowledge",
    "ui.guild_record",
    "ui.success",
    "ui.retreat",
    "ui.certified",
    "ui.medical_recovering",
    "ui.rest_recommended",
    "ui.acknowledge_all",
    "ui.review_heading",
    "ui.review_pending_heading",
    "ui.review_targets",
    "ui.review_passed",
    "ui.review_failed",
    "ui.review_careers",
    "ui.review_open_body",
    "ui.continue_sandbox",
    "ui.restart",
    "ui.menu_save",
    "ui.dossier_heading",
    "ui.dossier_body",
    "help.next_tip",
    "help.skip",
    "help.next",
    "help.welcome",
    "help.selection_rooms",
    "help.selection_jobs",
    "help.dispatch_rooms",
    "help.dispatch_jobs",
    "help.time",
    "help.reports",
    "help.recovery",
    "help.trial",
    "help.promotion",
    "lesson.welcome_title",
    "lesson.selection_title",
    "lesson.dispatch_title",
    "lesson.time_title",
    "lesson.reports_title",
    "lesson.recovery_title",
    "lesson.promotion_title",
    "lesson.trial_title",
    "lesson.welcome_text",
    "lesson.selection_text",
    "lesson.dispatch_text",
    "lesson.time_text",
    "lesson.reports_text",
    "lesson.recovery_text",
    "lesson.promotion_text",
    "lesson.trial_text",
    "action.cannot_open_ledger",
    "action.preferences_save_failed",
    "action.ledger_save_failed",
    "error.review_pending",
    "error.select_contract",
    "error.select_adventurer",
    "error.offer_unavailable",
    "error.expedition_exists",
    "error.select_party",
    "error.invalid_party",
    "error.adventurer_away",
    "error.adventurer_injured",
    "error.trial_candidate",
    "error.bronze_leader",
    "error.offer_expired",
    "error.candidate_unready",
    "service.infirmary_open",
    "service.training_open",
    "service.scout_prepared",
    "service.requires_gold",
    "service.review_pending",
    "service.infirmary_exists",
    "service.training_exists",
    "service.choose_contract",
    "service.invalid_contract",
    "service.trial_no_scout",
    "service.scout_exists",
    "service.expedition_exists",
    "report.failure_body",
    "report.medical_leave",
    "report.safe_return",
    "report.title",
    "report.reward",
    "report.promotion_title",
    "report.promotion_body",
    "report.promotion_reward",
    "review.sandbox_returns",
    "review.late_returns",
    "review.credited_returns",
    "contract.standing_trial",
    "contract.accept_days",
    "contract.expired",
    "contract.service_credited",
    "contract.service_new",
    "contract.no_service",
    "contract.offer_notice",
];

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextCatalog {
    pub strings: std::collections::BTreeMap<String, String>,
}

impl TextCatalog {
    pub fn load() -> Result<Self, String> {
        Self::from_json(macroquad_toolkit::include_json_str!(
            "../assets/data/text.json"
        ))
    }

    pub fn from_json(json: &str) -> Result<Self, String> {
        let strings = macroquad_toolkit::data_loader::parse_json_labeled(TEXT_PATH, json)?;
        let catalog = Self { strings };
        catalog.validate()?;
        Ok(catalog)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self
            .strings
            .iter()
            .any(|(key, value)| key.trim().is_empty() || value.trim().is_empty())
        {
            return Err(format!(
                "{TEXT_PATH}: text keys and values must be non-empty."
            ));
        }
        if let Some(key) = REQUIRED_TEXT_KEYS
            .iter()
            .find(|key| !self.strings.contains_key(**key))
        {
            return Err(format!("{TEXT_PATH}: missing required text key '{key}'."));
        }
        Ok(())
    }

    pub fn get(&self, key: &str) -> &str {
        self.strings
            .get(key)
            .unwrap_or_else(|| panic!("{TEXT_PATH}: missing text key '{key}'"))
    }

    pub fn format(&self, key: &str, values: &[(&str, String)]) -> String {
        let mut value = self.get(key).to_string();
        for (name, replacement) in values {
            value = value.replace(&format!("{{{name}}}"), replacement);
        }
        value
    }
}

impl Default for TextCatalog {
    fn default() -> Self {
        Self::load().expect("Required occupational hazard text catalog")
    }
}

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
