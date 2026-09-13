//! The title opens onto the same headquarters the player will manage.
use super::*;

pub fn draw_title(g: &Game) -> Option<UiAction> {
    let w = screen_width();
    let h = screen_height();
    let art = g
        .assets
        .get_texture("building")
        .expect("Required headquarters title artwork");
    draw_texture_ex(
        art,
        0.,
        0.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(w, h)),
            ..Default::default()
        },
    );
    draw_rectangle(0., 0., w, h, Color::new(0.015, 0.025, 0.035, 0.38));
    let width = (w - 28.).min(470.);
    let height = (h - 28.).min(490.);
    let x = (w - width) / 2.;
    let y = (h - height) / 2.;
    panel(Rect::new(x, y, width, height), PANEL);
    let short = h < 500.;
    headquarters::crest(vec2(x + width / 2., y + 35.), 19.);
    label(
        g.guild.text.get("title.name"),
        Rect::new(x + 14., y + 64., width - 28., 42.),
        29.,
        INK,
    );
    label(
        g.guild.text.get("title.tagline"),
        Rect::new(x + 14., y + 108., width - 28., 30.),
        18.,
        GOLD,
    );
    if !short {
        paragraph(
            g.guild.text.get("title.description"),
            Rect::new(x + 24., y + 158., width - 48., 85.),
            20.,
            MUTED,
        );
    }
    let controls = y + if short { 152. } else { 260. };
    if g.confirm_new {
        paragraph(
            g.guild.text.get("title.confirm_new"),
            Rect::new(x + 24., controls, width - 48., 46.),
            17.,
            GOLD,
        );
        if button(
            Rect::new(x + 24., controls + 52., width - 48., 46.),
            g.guild.text.get("title.start_new"),
            true,
        ) {
            return Some(UiAction::Start);
        }
        if button(
            Rect::new(x + 24., controls + 106., width - 48., 46.),
            g.guild.text.get("title.keep"),
            false,
        ) {
            return Some(UiAction::Cancel);
        }
    } else {
        if headquarters::primary(
            Rect::new(x + 24., controls, width - 48., 48.),
            g.guild.text.get("title.new"),
            true,
        ) {
            return Some(UiAction::Start);
        }
        if g.has_save
            && button(
                Rect::new(x + 24., controls + 56., width - 48., 46.),
                g.guild.text.get("title.continue"),
                false,
            )
        {
            return Some(UiAction::Continue);
        }
        #[cfg(target_os = "windows")]
        if button(
            Rect::new(x + 24., controls + 110., width - 48., 44.),
            g.guild.text.get("title.exit"),
            false,
        ) {
            return Some(UiAction::Exit);
        }
    }
    if !g.notice.is_empty() {
        paragraph(
            &g.notice,
            Rect::new(x + 20., y + height - 45., width - 40., 38.),
            15.,
            GOLD,
        );
    }
    None
}
