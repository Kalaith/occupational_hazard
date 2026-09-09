//! A licence celebration remains usable on a short touch viewport.
use super::*;

pub fn victory(g: &Game) -> Option<UiAction> {
    let width = (screen_width() - 24.).min(600.);
    let height = (screen_height() - 24.).min(520.);
    let x = (screen_width() - width) / 2.;
    let y = (screen_height() - height) / 2.;
    panel(Rect::new(x, y, width, height), PANEL);
    let size = if height < 400. { 80. } else { 132. };
    portrait(
        g,
        "elowen",
        Rect::new(x + (width - size) / 2., y + 18., size, size),
    );
    label(
        "BRONZE, DULY EARNED",
        Rect::new(x + 16., y + size + 32., width - 32., 44.),
        28.,
        GOLD,
    );
    paragraph("The licence is signed. This adventurer can now lead a Bronze commission. Return by day 30 and complete six distinct service jobs for head office.", Rect::new(x + 24., y + size + 86., width - 48., height - size - 160.), 21., INK);
    if button(
        Rect::new(x + 24., y + height - 62., width - 48., 48.),
        "BACK TO HEADQUARTERS",
        true,
    ) {
        return Some(UiAction::CloseVictory);
    }
    None
}
