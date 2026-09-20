//! Desk surfaces and input targets. Rendering returns one action per frame.
use crate::game::Game;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;
mod day;
mod dossier;
pub mod headquarters;
mod help;
mod month;
mod theme;
mod title;

pub const BACKGROUND: Color = Color::new(0.055, 0.095, 0.10, 1.0);
const PANEL: Color = Color::new(0.12, 0.105, 0.085, 0.99);
const GOLD: Color = Color::new(0.83, 0.66, 0.39, 1.0);
const INK: Color = Color::new(0.90, 0.89, 0.85, 1.0);
const MUTED: Color = Color::new(0.66, 0.73, 0.70, 1.0);

pub enum UiAction {
    CommissionList,
    Rooms,
    CommissionPage(usize),
    Overview,
    ToggleStaff,
    ToggleReadinessDetails,
    Room(crate::headquarters::Room),
    PrepareTrial(usize),
    SheetPage(usize),
    Journey(usize),
    JourneyPage(usize),
    ReturnSection(usize),
    ReducedMotion,
    TextSize,
    SkipMotion,
    ReportList,
    ReportPage(usize),
    Help(usize),
    CloseHelp,
    LessonDone(crate::tutorial::Lesson),
    SkipTutorial,
    Month,
    CloseMonth,
    Sandbox,
    Restart,
    ReviewDetails,
    Purchase(crate::services::Purchase),
    Start,
    Continue,
    Cancel,
    #[cfg(target_os = "windows")]
    Exit,
    Settings,
    CloseSettings,
    Title,
    Tab(usize),
    Quest(usize),
    Party(usize),
    Dispatch,
    NextDay,
    ConfirmDay,
    CancelDay,
    Dossier(usize),
    Report(usize),
    Promote(usize),
    CloseVictory,
    Save,
    DismissFeedback,
}

pub fn draw(game: &Game) -> Option<UiAction> {
    clear_background(BACKGROUND);
    if game.in_title {
        return title::draw_title(game);
    }
    if game.settings_open {
        return menu(game);
    }
    if game.help_page.is_some() {
        return help::draw_help(game);
    }
    if game.guild.review_pending() || game.month_open {
        return month::draw_month(game);
    }
    if game.victory {
        return dossier::victory(game);
    }
    if game.confirm_day {
        return day::draw_warning(game);
    }
    headquarters::draw(game)
}

fn menu(game: &Game) -> Option<UiAction> {
    let copy = &game.guild.text;
    let w = (screen_width() - 32.0).min(440.0);
    let x = (screen_width() - w) / 2.0;
    let height = (screen_height() - 24.).min(490.);
    let y = (screen_height() - height) / 2.0;
    panel(Rect::new(x, y, w, height), PANEL);
    label(
        copy.get("menu.heading"),
        Rect::new(x, y + 14.0, w, 44.0),
        26.0,
        GOLD,
    );
    if height > 420. {
        paragraph(
            copy.get("menu.save_note"),
            Rect::new(x + 20.0, y + 65.0, w - 40.0, 50.0),
            18.0,
            MUTED,
        );
    }
    for (i, (text, action)) in [
        (copy.get("menu.save"), UiAction::Save),
        (copy.get("menu.help"), UiAction::Help(0)),
        (copy.get("menu.return"), UiAction::Title),
        (
            if game.hq.reduced_motion {
                copy.get("menu.motion_reduced")
            } else {
                copy.get("menu.motion_full")
            },
            UiAction::ReducedMotion,
        ),
        (copy.get("menu.close"), UiAction::CloseSettings),
        (
            if game.hq.large_text {
                copy.get("menu.text_large")
            } else {
                copy.get("menu.text_standard")
            },
            UiAction::TextSize,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        if button(
            if height < 450. {
                Rect::new(
                    x + 20. + (i % 2) as f32 * ((w - 32.) / 2.),
                    y + 104. + (i / 2) as f32 * 54.,
                    (w - 48.) / 2.,
                    48.,
                )
            } else {
                Rect::new(x + 20., y + 152. + i as f32 * 52., w - 40., 46.)
            },
            text,
            false,
        ) {
            return Some(action);
        }
    }
    if !game.feedback.message.is_empty() {
        paragraph(
            &game.feedback.message,
            Rect::new(20.0, screen_height() - 65.0, screen_width() - 40.0, 60.0),
            16.0,
            GOLD,
        );
    }
    None
}

fn activated(rect: Rect) -> bool {
    macroquad_toolkit::ui::Pointer::read(|p| p).released_on(rect)
}

fn panel(rect: Rect, color: Color) {
    theme::panel(rect, color);
}

fn label(text: &str, rect: Rect, size: f32, color: Color) {
    draw_text_centered_in_box(text, rect.x, rect.y, rect.w, rect.h, size, color);
}

fn paragraph(text: &str, rect: Rect, size: f32, color: Color) {
    draw_text_block(text, rect.x, rect.y, rect.w, rect.h, size, 3.0, color);
}
fn button(rect: Rect, text: &str, selected: bool) -> bool {
    theme::button(rect, text, selected, false, true)
}

fn portrait(game: &Game, key: &str, rect: Rect) {
    if let Some(texture) = game.assets.get_texture(key) {
        draw_texture_ex(
            texture,
            rect.x,
            rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(rect.w, rect.h)),
                ..Default::default()
            },
        );
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, GOLD);
    }
}

fn guided_button(g: &Game, rect: Rect, text: &str, selected: bool) -> bool {
    let clicked = button(rect, text, selected);
    if help::is_target(g, text) {
        draw_rectangle_lines(
            rect.x + 2.0,
            rect.y + 2.0,
            rect.w - 4.0,
            rect.h - 4.0,
            3.0,
            GOLD,
        );
    }
    clicked
}
