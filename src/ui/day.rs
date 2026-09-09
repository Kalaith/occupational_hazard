//! Explicit choice before spending a day with fully rested staff unassigned.
use super::*;

pub fn draw_warning(g: &Game) -> Option<UiAction> {
    let w = (screen_width() - 24.0).min(500.0);
    let h = 360.0;
    let x = (screen_width() - w) / 2.0;
    let y = (screen_height() - h).max(0.0) / 2.0;
    panel(Rect::new(x, y, w, h), PANEL);
    label(
        "RESTED AND UNASSIGNED",
        Rect::new(x + 12.0, y + 14.0, w - 24.0, 36.0),
        25.0,
        GOLD,
    );
    paragraph(
        &format!("{}\n\nThese adventurers are ready for a mission. Advance to day {} anyway? Expeditions will continue and other staff will recover.", g.guild.rested_idle().join(", "), g.guild.day + 1),
        Rect::new(x + 16.0, y + 62.0, w - 32.0, 170.0),
        20.0,
        WHITE,
    );
    if button(
        Rect::new(x + 16.0, y + h - 112.0, w - 32.0, 44.0),
        "BACK TO HEADQUARTERS",
        true,
    ) {
        return Some(UiAction::CancelDay);
    }
    if button(
        Rect::new(x + 16.0, y + h - 60.0, w - 32.0, 44.0),
        "ADVANCE DAY",
        false,
    ) {
        return Some(UiAction::ConfirmDay);
    }
    None
}
