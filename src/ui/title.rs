//! The guild opens with the people whose careers the player will shape.
use super::*;

pub fn draw_title(game: &Game) -> Option<UiAction> {
    let w = screen_width();
    let h = screen_height();
    if w > 850.0 {
        let size = (w * 0.32).min(h * 0.73);
        portrait(game, "mira", Rect::new(0.0, h - size, size, size));
        portrait(game, "elowen", Rect::new(w - size, h - size, size, size));
    } else {
        let size = (h * 0.25).min(160.0);
        portrait(game, "mira", Rect::new(w / 2.0 - size, 16.0, size, size));
        portrait(game, "elowen", Rect::new(w / 2.0, 16.0, size, size));
    }
    let width = (w - 32.0).min(480.0);
    let x = (w - width) / 2.0;
    let y = if w > 850.0 {
        h * 0.13
    } else {
        (h * 0.25).min(160.0) + 28.0
    };
    panel(
        Rect::new(x, y, width, (h - y - 20.0).min(500.0)),
        BACKGROUND,
    );
    label(
        "ADVENTURERS' GUILD / FIRST MONTH",
        Rect::new(x + 8.0, y + 16.0, width - 16.0, 25.0),
        17.0,
        GOLD,
    );
    label(
        "Occupational Hazard",
        Rect::new(x + 8.0, y + 50.0, width - 16.0, 55.0),
        38.0,
        WHITE,
    );
    paragraph("Head office visits on day 30. Certify a Bronze adventurer and complete a Bronze commission before the review.",
        Rect::new(x + 24.0, y + 120.0, width - 48.0, 76.0), 21.0, MUTED);
    let bw = width - 48.0;
    if game.confirm_new {
        paragraph(
            "Open a new branch? This replaces your saved guild.",
            Rect::new(x + 24.0, y + 204.0, bw, 45.0),
            17.0,
            GOLD,
        );
        if button(
            Rect::new(x + 24.0, y + 255.0, bw, 48.0),
            "START NEW GUILD",
            false,
        ) {
            return Some(UiAction::Start);
        }
        if button(
            Rect::new(x + 24.0, y + 313.0, bw, 48.0),
            "KEEP MY GUILD",
            false,
        ) {
            return Some(UiAction::Cancel);
        }
    } else {
        if button(Rect::new(x + 24.0, y + 214.0, bw, 50.0), "NEW GUILD", true) {
            return Some(UiAction::Start);
        }
        if game.has_save && button(Rect::new(x + 24.0, y + 274.0, bw, 50.0), "CONTINUE", false) {
            return Some(UiAction::Continue);
        }
        #[cfg(target_os = "windows")]
        if button(Rect::new(x + 24.0, y + 334.0, bw, 46.0), "EXIT GAME", false) {
            return Some(UiAction::Exit);
        }
    }
    paragraph(
        &game.notice,
        Rect::new(x + 16.0, y + 390.0, width - 32.0, 78.0),
        16.0,
        GOLD,
    );
    None
}
