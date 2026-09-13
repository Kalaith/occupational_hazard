//! Apply UI intents to the owned game state and persist durable changes.

use super::Game;
use crate::{
    headquarters::{Room, Sheet, Transition},
    tutorial::Lesson,
    ui::UiAction,
};
use macroquad::prelude::{is_key_pressed, KeyCode};
use macroquad_toolkit::persistence;

pub(super) fn update(game: &mut Game, dt: f32) {
    game.hq.tick(dt);
    if !game.in_title && is_key_pressed(KeyCode::Escape) {
        game.settings_open = !game.settings_open;
    }
    let Some(action) = game.pending.take() else {
        return;
    };
    if apply_action(game, action) {
        normalize_selected(game);
    }
}

fn apply_action(game: &mut Game, action: UiAction) -> bool {
    match action {
        UiAction::Rooms => game.hq.open(Sheet::Rooms),
        UiAction::CommissionList => game.hq.open(Sheet::Commissions),
        UiAction::CommissionPage(page) => game.board_page = page,
        UiAction::Overview => {
            game.hq.open(Sheet::None);
            game.hq.focus = None;
        }
        UiAction::PrepareTrial(id) => prepare_trial(game, id),
        UiAction::Room(room) => open_room(game, room),
        UiAction::SheetPage(page) => game.hq.page = page,
        UiAction::Journey(id) => {
            game.hq.open(Sheet::Jobs);
            game.hq.journey = Some(id);
        }
        UiAction::ReducedMotion => toggle_motion(game),
        UiAction::TextSize => toggle_text_size(game),
        UiAction::SkipMotion => game.hq.transition = None,
        UiAction::ReportList => game.report_detail = false,
        UiAction::ReportPage(page) => game.report_page = page,
        UiAction::Help(page) => {
            game.help_page = Some(page);
            game.settings_open = false;
        }
        UiAction::CloseHelp => game.help_page = None,
        UiAction::LessonDone(lesson) => {
            game.guild.tutorial.acknowledge(lesson);
            game.save();
        }
        UiAction::SkipTutorial => {
            game.guild.tutorial.skipped = true;
            game.save();
        }
        UiAction::Month => {
            game.guild.tutorial.acknowledge(Lesson::Welcome);
            game.month_open = true;
            game.save();
        }
        UiAction::CloseMonth => game.month_open = false,
        UiAction::Sandbox => {
            game.guild.continue_sandbox();
            game.month_open = false;
            game.save();
        }
        UiAction::Restart => {
            game.in_title = true;
            game.confirm_new = true;
            game.month_open = false;
        }
        UiAction::Purchase(purchase) => apply_purchase(game, purchase),
        #[cfg(target_os = "windows")]
        UiAction::Exit => game.exit_requested = true,
        UiAction::Start => return start_new_guild(game),
        UiAction::Cancel => game.confirm_new = false,
        UiAction::Continue => continue_guild(game),
        UiAction::Settings => game.settings_open = true,
        UiAction::CloseSettings => game.settings_open = false,
        UiAction::Title => {
            game.in_title = true;
            game.confirm_day = false;
            game.settings_open = false;
        }
        UiAction::Tab(tab) => open_tab(game, tab),
        UiAction::Quest(id) => select_quest(game, id),
        UiAction::Party(id) => toggle_party_member(game, id),
        UiAction::Dispatch => dispatch(game),
        UiAction::NextDay => next_day(game),
        UiAction::ConfirmDay => confirm_day(game),
        UiAction::CancelDay => game.confirm_day = false,
        UiAction::Dossier(id) => {
            game.dossier = id;
            game.hq.open(Sheet::Career);
        }
        UiAction::Report(id) => open_report(game, id),
        UiAction::Promote(id) => promote(game, id),
        UiAction::CloseVictory => game.victory = false,
        UiAction::Save => game.save(),
    }
    true
}

fn prepare_trial(game: &mut Game, id: usize) {
    if let Some(quest) = game.contracts.iter().position(|q| q.promotion) {
        game.selected = quest;
        game.party = vec![id];
        game.hq.open(Sheet::Jobs);
    }
}

fn open_room(game: &mut Game, room: Room) {
    game.hq.focus = Some(room);
    game.hq.open(match room {
        Room::Common => Sheet::Career,
        Room::Assignments => Sheet::Commissions,
        Room::Gate => Sheet::Returns,
        Room::Recovery | Room::Training => Sheet::Facility(room),
        Room::Records => {
            game.month_open = true;
            Sheet::None
        }
    });
}

fn toggle_motion(game: &mut Game) {
    game.hq.reduced_motion = !game.hq.reduced_motion;
    game.hq.transition = None;
    game.save_preferences();
}

fn toggle_text_size(game: &mut Game) {
    game.hq.large_text = !game.hq.large_text;
    macroquad_toolkit::ui::set_ui_text_scale(if game.hq.large_text { 1.15 } else { 1.0 });
    game.save_preferences();
}

fn apply_purchase(game: &mut Game, purchase: crate::services::Purchase) {
    match game.guild.purchase(purchase, &game.contracts) {
        Ok(message) => {
            game.notice = message;
            game.save();
        }
        Err(error) => game.notice = error,
    }
}

fn start_new_guild(game: &mut Game) -> bool {
    if game.has_save && !game.confirm_new {
        game.confirm_new = true;
        return false;
    }
    game.confirm_day = false;
    game.guild = crate::simulation::Guild::new();
    game.hq = crate::headquarters::Headquarters {
        reduced_motion: game.hq.reduced_motion,
        large_text: game.hq.large_text,
        ..Default::default()
    };
    game.help_page = None;
    game.month_open = false;
    game.victory = false;
    game.dossier = 0;
    game.report = 0;
    game.report_detail = false;
    game.report_page = 0;
    game.party.clear();
    game.selected = 0;
    game.board_page = 0;
    game.tab = 0;
    game.in_title = false;
    game.confirm_new = false;
    game.notice.clear();
    game.save();
    true
}

fn continue_guild(game: &mut Game) {
    match persistence::load_from_slot::<crate::simulation::Guild>("occupational_hazard", "guild") {
        Ok(mut guild) => {
            let validation = guild
                .migrate_board(&game.contracts)
                .and_then(|()| guild.validate(&game.contracts));
            match validation {
                Ok(()) => {
                    game.confirm_day = false;
                    game.guild = guild;
                    game.guild.finish_review(&game.contracts);
                    game.month_open = false;
                    game.in_title = false;
                    game.notice.clear();
                    game.save();
                }
                Err(error) => {
                    game.notice = game
                        .guild
                        .text
                        .format("action.cannot_open_ledger", &[("error", error)])
                }
            }
        }
        Err(error) => {
            game.notice = game
                .guild
                .text
                .format("action.cannot_open_ledger", &[("error", error)])
        }
    }
}

fn open_tab(game: &mut Game, tab: usize) {
    game.hq.open(match tab {
        0 => Sheet::Commissions,
        1 => Sheet::Career,
        2 => Sheet::Returns,
        _ => Sheet::Facility(Room::Recovery),
    });
    game.tab = tab;
    if tab == 2 {
        game.report_detail = false;
        game.report_page = 0;
    }
    if tab == 2 && !game.guild.reports.is_empty() {
        game.guild.tutorial.acknowledge(Lesson::Reports);
        game.save();
    }
    game.notice.clear();
}

fn select_quest(game: &mut Game, id: usize) {
    let page = if game.hq.sheet == Sheet::Jobs {
        game.hq.page
    } else {
        0
    };
    game.hq.open(Sheet::Jobs);
    game.hq.page = page;
    game.selected = id;
    game.notice.clear();
}

fn toggle_party_member(game: &mut Game, id: usize) {
    if game.party.contains(&id) {
        game.party.retain(|&member| member != id);
    } else if !game.guild.busy(id) && game.guild.roster[id].injury == 0 {
        game.party.push(id);
        game.guild.tutorial.acknowledge(Lesson::Welcome);
        game.guild.tutorial.acknowledge(Lesson::Selection);
    }
    game.notice.clear();
}

fn dispatch(game: &mut Game) {
    match game
        .guild
        .dispatch(game.selected, &game.party, &game.contracts)
    {
        Ok(()) => {
            game.hq.transition = Some(Transition {
                people: game.party.clone(),
                arriving: false,
                elapsed: 0.0,
            });
            game.hq.open(Sheet::None);
            game.guild.tutorial.dispatched = true;
            game.guild.tutorial.acknowledge(Lesson::Dispatch);
            if game.contracts[game.selected].promotion {
                game.guild.tutorial.acknowledge(Lesson::Trial);
            }
            game.notice = game.guild.text.get("ui.dispatched").into();
            game.party.clear();
            game.save();
        }
        Err(error) => game.notice = error,
    }
}

fn next_day(game: &mut Game) {
    if game.guild.rested_idle().is_empty() {
        advance_day(game);
    } else {
        game.confirm_day = true;
    }
}

fn confirm_day(game: &mut Game) {
    if game.confirm_day {
        game.confirm_day = false;
        advance_day(game);
    }
}

fn open_report(game: &mut Game, id: usize) {
    game.hq.open(Sheet::Returns);
    if game.guild.read_report(id) {
        game.report = id;
        game.report_detail = true;
        game.guild.tutorial.acknowledge(Lesson::Reports);
        game.save();
    }
}

fn promote(game: &mut Game, id: usize) {
    match game.guild.promote(id) {
        Ok(()) => {
            game.guild.tutorial.acknowledge(Lesson::Promotion);
            game.victory = !game.guild.victory_seen;
            game.guild.victory_seen = true;
            game.notice = game.guild.text.get("action.promotion").into();
            game.save();
        }
        Err(error) => game.notice = error,
    }
}

fn advance_day(game: &mut Game) {
    if !game.hq.begin_day_change() {
        return;
    }
    game.guild.tutorial.acknowledge(Lesson::Time);
    if game
        .guild
        .roster
        .iter()
        .any(|adventurer| adventurer.fatigue > 0 || adventurer.injury > 0)
    {
        game.guild.tutorial.acknowledge(Lesson::Recovery);
    }
    let returning = game
        .guild
        .expeditions
        .iter()
        .filter(|expedition| expedition.returns == game.guild.day + 1)
        .count();
    let arrivals = game
        .guild
        .expeditions
        .iter()
        .filter(|expedition| expedition.returns == game.guild.day + 1)
        .flat_map(|expedition| expedition.party.iter().copied())
        .collect();
    game.guild.next_day(&game.contracts);
    game.notice = if returning > 0 {
        game.guild.text.format(
            "ui.return_reports",
            &[
                ("count", game.guild.unread_reports().to_string()),
                ("returning", returning.to_string()),
            ],
        )
    } else {
        game.guild.text.get("ui.new_day").into()
    };
    if returning > 0 {
        game.hq.open(Sheet::Returns);
        game.hq.transition = Some(Transition {
            people: arrivals,
            arriving: true,
            elapsed: 0.0,
        });
        game.tab = 2;
        game.report = 0;
        game.report_detail = false;
        game.report_page = 0;
    }
    game.save();
}

fn normalize_selected(game: &mut Game) {
    if !game.guild.contract_open(game.selected, &game.contracts) {
        if let Some(id) = game.guild.open_contracts(&game.contracts).first() {
            game.selected = *id;
        }
    }
}

impl Game {
    fn save_preferences(&mut self) {
        if !self.capture {
            if let Err(error) = persistence::save_to_slot(
                "occupational_hazard",
                "preferences",
                &(self.hq.reduced_motion, self.hq.large_text),
            ) {
                self.notice = self
                    .guild
                    .text
                    .format("action.preferences_save_failed", &[("error", error)]);
            }
        }
    }

    fn save(&mut self) {
        if self.capture {
            return;
        }
        if let Err(error) = self.guild.migrate_board(&self.contracts) {
            self.notice = self
                .guild
                .text
                .format("action.ledger_save_failed", &[("error", error)]);
            return;
        }
        match persistence::save_to_slot("occupational_hazard", "guild", &self.guild) {
            Ok(()) => self.has_save = true,
            Err(error) => {
                self.notice = self
                    .guild
                    .text
                    .format("action.ledger_save_retry", &[("error", error)])
            }
        }
    }
}
