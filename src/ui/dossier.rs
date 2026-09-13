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
        g.guild.text.get("ui.dossier_heading"),
        Rect::new(x + 16., y + size + 32., width - 32., 44.),
        28.,
        GOLD,
    );
    paragraph(
        &g.guild.text.format(
            "ui.dossier_body",
            &[
                ("cutoff_day", g.guild.config.review.cutoff_day.to_string()),
                (
                    "service_quota",
                    g.guild.config.review.service_quota.to_string(),
                ),
            ],
        ),
        Rect::new(x + 24., y + size + 86., width - 48., height - size - 160.),
        21.,
        INK,
    );
    if button(
        Rect::new(x + 24., y + height - 62., width - 48., 48.),
        g.guild.text.get("ui.back_headquarters"),
        true,
    ) {
        return Some(UiAction::CloseVictory);
    }
    None
}
