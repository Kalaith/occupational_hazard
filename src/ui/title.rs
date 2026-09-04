//! Opening title screen, sized to the current window.

use super::*;

pub fn draw_title() -> Option<UiAction> {
    let width = screen_width();
    let height = screen_height();
    let content_width = (width - 40.0).clamp(180.0, 900.0);
    let left = (width - content_width) / 2.0;
    let middle = height / 2.0;
    label(
        "ADVENTURERS' GUILD",
        Rect::new(left, middle - 130.0, content_width, 36.0),
        20.0,
        GOLD,
    );
    label(
        "Occupational Hazard",
        Rect::new(left, middle - 80.0, content_width, 70.0),
        (width / 20.0).clamp(24.0, 54.0),
        WHITE,
    );
    label(
        "Every contract is someone's future.",
        Rect::new(left, middle, content_width, 40.0),
        (width / 35.0).clamp(14.0, 24.0),
        Color::new(0.65, 0.67, 0.66, 1.0),
    );
    let button_width = content_width.min(320.0);
    if button(
        Rect::new(
            (width - button_width) / 2.0,
            middle + 84.0,
            button_width,
            56.0,
        ),
        "Start",
    ) {
        Some(UiAction::Start)
    } else {
        None
    }
}
