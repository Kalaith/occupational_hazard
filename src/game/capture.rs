//! Build deterministic presentation fixtures without touching player saves.

use super::Game;
use crate::{
    headquarters::{Room, Sheet, Transition},
    services::Purchase,
};

pub(super) fn begin(game: &mut Game, scene_name: &str) {
    let scene_id = normalized_scene(scene_name);
    reset(game, scene_id);
    setup_review(game, scene_id);
    setup_character(game, scene_id);
    setup_reports(game, scene_id);
    setup_special_tabs(game, scene_id);
    setup_base_sheet(game);
    setup_planning(game, scene_id);
    setup_gameplay(game, scene_id);
    setup_scene_overrides(game, scene_id);
}

fn normalized_scene(scene_name: &str) -> &str {
    scene_name
        .strip_prefix("phone_")
        .or_else(|| scene_name.strip_prefix("landscape_"))
        .or_else(|| scene_name.strip_prefix("tablet_"))
        .unwrap_or(scene_name)
}

fn reset(game: &mut Game, scene_id: &str) {
    game.hq = Default::default();
    macroquad_toolkit::ui::set_ui_text_scale(1.0);
    game.capture = true;
    game.guild = crate::simulation::Guild::new();
    game.guild.tutorial.skipped = scene_id != "tutorial";
    game.help_page = None;
    game.report_detail = false;
    game.report_page = 0;
    game.board_page = 0;
    game.party = vec![0];
    game.has_save = false;
    game.in_title = scene_id == "title";
    game.settings_open = scene_id == "settings";
    game.tab = 0;
    game.victory = false;
    game.month_open = scene_id == "objectives";
    game.confirm_day = scene_id == "idle_warning";
    game.notice.clear();
    game.pending = None;
}

fn setup_review(game: &mut Game, scene_id: &str) {
    if !matches!(scene_id, "review" | "review_missed") {
        return;
    }
    if scene_id == "review" {
        game.guild.roster[0].bronze = true;
        game.guild.roster[0].xp = 135;
        game.guild.roster[0].successes = 5;
        game.guild.completed[5] = 1;
        game.guild.board.service_credit = game
            .contracts
            .iter()
            .filter(|contract| contract.service)
            .take(6)
            .map(|contract| contract.id.clone())
            .collect();
        game.guild.gold = 352;
    }
    game.guild.day = 30;
    game.guild.finish_review(&game.contracts);
}

fn setup_character(game: &mut Game, scene_id: &str) {
    game.dossier = match scene_id {
        "tomas" => 1,
        "pip" => 2,
        _ => 0,
    };
    if matches!(scene_id, "tomas" | "pip") {
        game.tab = 1;
    }
    game.selected = if scene_id == "mobile_contract" { 4 } else { 0 };
    if matches!(scene_id, "promotion" | "mobile_promotion") {
        let adventurer = &mut game.guild.roster[0];
        adventurer.xp = 90;
        adventurer.successes = 4;
        adventurer.trial_passed = true;
        game.tab = 1;
    }
}

fn setup_reports(game: &mut Game, scene_id: &str) {
    if !matches!(scene_id, "report" | "reports") {
        return;
    }
    game.guild.dispatch(0, &[0], &game.contracts).unwrap();
    game.guild.dispatch(1, &[2], &game.contracts).unwrap();
    game.report_detail = scene_id == "report";
    game.guild.next_day(&game.contracts);
    game.tab = 2;
}

fn setup_special_tabs(game: &mut Game, scene_id: &str) {
    if scene_id == "victory" {
        game.victory = true;
    }
    if matches!(scene_id, "services" | "mobile_services") {
        game.tab = 3;
        game.guild.gold = 180;
        game.selected = 3;
    }
}

fn setup_base_sheet(game: &mut Game) {
    game.hq.sheet = match game.tab {
        1 => Sheet::Career,
        2 => Sheet::Returns,
        3 => Sheet::Facility(Room::Recovery),
        _ => Sheet::None,
    };
}

fn setup_planning(game: &mut Game, scene_id: &str) {
    if matches!(
        scene_id,
        "planning" | "mobile_party" | "mobile_contract" | "party_readiness" | "dispatch_readiness"
    ) {
        game.hq.sheet = Sheet::Jobs;
    }
}

fn setup_gameplay(game: &mut Game, scene_id: &str) {
    if !matches!(
        scene_id,
        "gameplay"
            | "planning"
            | "recovery"
            | "commissions"
            | "party_readiness"
            | "dispatch_readiness"
    ) {
        return;
    }
    game.guild.day = 9;
    game.guild.gold = 182;
    game.guild.roster[2].fatigue = 3;
    game.guild.migrate_board(&game.contracts).unwrap();
    game.guild.dispatch(2, &[1], &game.contracts).unwrap();
    game.selected = 3;
    if matches!(
        scene_id,
        "planning" | "party_readiness" | "dispatch_readiness"
    ) {
        game.party = vec![0, 2];
    } else {
        game.party.clear();
    }
    if scene_id == "recovery" {
        game.guild.roster[2].injury = 2;
    }
}

fn setup_scene_overrides(game: &mut Game, scene_id: &str) {
    if scene_id == "party_readiness" {
        game.hq.page = 1;
    }
    if scene_id == "dispatch_readiness" {
        game.hq.page = 2;
    }
    if scene_id == "commissions" {
        game.hq.sheet = Sheet::Commissions;
    }
    setup_pending_returns(game, scene_id);
    setup_everyone_away(game, scene_id);
    setup_facilities(game, scene_id);
    setup_blocked_trial(game, scene_id);
    setup_return_scene(game, scene_id);
    setup_departure(game, scene_id);
}

fn setup_pending_returns(game: &mut Game, scene_id: &str) {
    if scene_id != "pending_returns" {
        return;
    }
    game.guild.dispatch(0, &[0], &game.contracts).unwrap();
    game.guild.next_day(&game.contracts);
    game.guild.dispatch(2, &[1], &game.contracts).unwrap();
    game.party.clear();
}

fn setup_everyone_away(game: &mut Game, scene_id: &str) {
    if scene_id != "everyone_away" {
        return;
    }
    game.guild.dispatch(0, &[0], &game.contracts).unwrap();
    game.guild.dispatch(1, &[2], &game.contracts).unwrap();
    game.guild.dispatch(2, &[1], &game.contracts).unwrap();
    game.party.clear();
}

fn setup_facilities(game: &mut Game, scene_id: &str) {
    if scene_id != "facilities" {
        return;
    }
    game.guild.gold = 300;
    game.guild
        .purchase(Purchase::Infirmary, &game.contracts)
        .unwrap();
    game.guild
        .purchase(Purchase::TrainingYard, &game.contracts)
        .unwrap();
    game.guild.roster[2].injury = 2;
}

fn setup_blocked_trial(game: &mut Game, scene_id: &str) {
    if scene_id == "blocked_trial" {
        game.selected = 4;
        game.party = vec![0];
        game.hq.sheet = Sheet::Jobs;
    }
}

fn setup_return_scene(game: &mut Game, scene_id: &str) {
    if !matches!(scene_id, "injured_return" | "arrival") {
        return;
    }
    game.guild.dispatch(3, &[2], &game.contracts).unwrap();
    game.guild.next_day(&game.contracts);
    game.guild.next_day(&game.contracts);
    game.report_detail = true;
    game.hq.sheet = Sheet::Returns;
    if scene_id == "arrival" {
        game.hq.transition = Some(Transition {
            people: vec![2],
            arriving: true,
            elapsed: 0.,
        });
    }
}

fn setup_departure(game: &mut Game, scene_id: &str) {
    if scene_id != "departure" {
        return;
    }
    game.guild.dispatch(0, &[0], &game.contracts).unwrap();
    game.hq.transition = Some(Transition {
        people: vec![0],
        arriving: false,
        elapsed: 0.,
    });
}
