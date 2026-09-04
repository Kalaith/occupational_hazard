//! Desk surfaces and input targets. Rendering returns one action per frame.
use crate::game::Game;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;
mod desk;
mod dossier;
mod services;
mod title;

pub const BACKGROUND: Color = Color::new(0.055, 0.095, 0.10, 1.0);
const PANEL: Color = Color::new(0.09, 0.15, 0.16, 1.0);
const GOLD: Color = Color::new(0.83, 0.66, 0.39, 1.0);
const INK: Color = Color::new(0.20, 0.19, 0.15, 1.0);
const PAPER: Color = Color::new(0.87, 0.82, 0.69, 1.0);
const MUTED: Color = Color::new(0.66, 0.73, 0.70, 1.0);

pub enum UiAction {
    Purchase(crate::services::Purchase),
    ChooseParty(bool),
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
    Dossier(usize),
    Report(usize),
    Promote(usize),
    CloseVictory,
    Save,
}

pub fn draw(game: &Game) -> Option<UiAction> {
    clear_background(BACKGROUND);
    if game.in_title {
        return title::draw_title(game);
    }
    if game.settings_open {
        return menu(game);
    }
    if game.victory {
        return dossier::victory(game);
    }
    desk::draw_desk(game)
}

fn menu(game: &Game) -> Option<UiAction> {
    let w = (screen_width() - 32.0).min(440.0);
    let x = (screen_width() - w) / 2.0;
    let y = (screen_height() - 360.0).max(0.0) / 2.0;
    panel(Rect::new(x, y, w, 360.0), PANEL);
    label(
        "THE GUILD LEDGER",
        Rect::new(x, y + 14.0, w, 44.0),
        26.0,
        GOLD,
    );
    paragraph(
        "Progress saves after dispatch, day changes, promotions and purchases.",
        Rect::new(x + 20.0, y + 65.0, w - 40.0, 50.0),
        18.0,
        MUTED,
    );
    for (i, (text, action)) in [
        ("SAVE", UiAction::Save),
        ("RETURN TO TITLE", UiAction::Title),
        ("CLOSE", UiAction::CloseSettings),
    ]
    .into_iter()
    .enumerate()
    {
        if button(
            Rect::new(x + 20.0, y + 125.0 + i as f32 * 60.0, w - 40.0, 48.0),
            text,
            false,
        ) {
            return Some(action);
        }
    }
    paragraph(
        &game.notice,
        Rect::new(20.0, screen_height() - 65.0, screen_width() - 40.0, 60.0),
        16.0,
        GOLD,
    );
    None
}

fn activated(rect: Rect) -> bool {
    macroquad_toolkit::ui::Pointer::read(|p| p).released_on(rect)
}

fn panel(rect: Rect, color: Color) {
    draw_surface(
        rect,
        &SurfaceStyle::new(color).with_border(1.0, Color::new(0.44, 0.39, 0.27, 1.0)),
    );
}

fn label(text: &str, rect: Rect, size: f32, color: Color) {
    draw_text_centered_in_box(text, rect.x, rect.y, rect.w, rect.h, size, color);
}

fn paragraph(text: &str, rect: Rect, size: f32, color: Color) {
    draw_text_block(text, rect.x, rect.y, rect.w, rect.h, size, 3.0, color);
}
fn button(rect: Rect, text: &str, selected: bool) -> bool {
    let hover = macroquad_toolkit::ui::Pointer::read(|p| p).hovering_over(rect);
    panel(
        rect,
        if selected {
            Color::new(0.27, 0.36, 0.31, 1.0)
        } else if hover {
            Color::new(0.18, 0.25, 0.24, 1.0)
        } else {
            PANEL
        },
    );
    let measured = measure_text_size(text, TextStyle::new(19.0, WHITE)).width;
    let size = (19.0 * ((rect.w - 14.0) / measured.max(1.0)).min(1.0)).max(12.0);
    label(
        text,
        Rect::new(rect.x + 5.0, rect.y, rect.w - 10.0, rect.h),
        size,
        if selected { GOLD } else { WHITE },
    );
    activated(rect)
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
