//! Title screen for the development build.

use super::*;

pub fn draw_title(ui: &VirtualUi, save_exists: bool, session_started: bool) -> Vec<UiAction> {
    let mut actions = Vec::new();
    let mouse = ui.mouse_position();
    let gold = Color::new(0.78, 0.64, 0.38, 1.0);
    draw_surface(
        Rect::new(100.0, 64.0, 1080.0, 592.0),
        &SurfaceStyle::new(Color::new(0.09, 0.105, 0.11, 1.0))
            .with_border(2.0, gold)
            .with_inner_border(10.0, 1.0, Color::new(0.38, 0.34, 0.25, 1.0)),
    );
    for (text, y, size, color) in [
        ("ADVENTURERS' GUILD", 112.0, 20.0, gold),
        ("Occupational Hazard", 166.0, 54.0, dark::TEXT_BRIGHT),
        (
            "Every contract is someone's future.",
            238.0,
            24.0,
            dark::TEXT,
        ),
        ("DEVELOPMENT BUILD", 308.0, 17.0, gold),
        (
            "Explore the starter systems while the guild takes shape.",
            343.0,
            19.0,
            dark::TEXT_DIM,
        ),
    ] {
        draw_text_centered_in_box(text, 140.0, y, 1000.0, 58.0, size, color);
    }
    let entries = [
        (
            if session_started {
                "Resume Sandbox"
            } else {
                "Open Sandbox"
            },
            if session_started {
                UiAction::Resume
            } else {
                UiAction::NewGame
            },
            true,
        ),
        ("Load Saved Sandbox", UiAction::Load, save_exists),
    ];
    for (index, (label, action, enabled)) in entries.into_iter().enumerate() {
        if virtual_button(
            Rect::new(430.0, 425.0 + index as f32 * 66.0, 420.0, 52.0),
            label,
            enabled,
            ButtonTone::Secondary,
            mouse,
        ) {
            actions.push(action);
        }
    }
    draw_text_centered_in_box(
        "Guild management gameplay is not implemented yet.",
        140.0,
        576.0,
        1000.0,
        36.0,
        17.0,
        dark::TEXT_DIM,
    );
    actions
}
