//! Compact party cards used while planning a commission.
use super::*;

pub(super) fn cards(g: &Game, r: Rect) -> Option<UiAction> {
    let cw = (r.w - 20.) / 3.;
    let quest =
        g.hq.journey
            .map_or(g.selected, |e| g.guild.expeditions[e].contract);
    let q = &g.contracts[quest];
    for (id, a) in g.guild.roster.iter().enumerate() {
        let x = r.x + id as f32 * (cw + 10.);
        let pic = Rect::new(x, r.y, 64., 64.);
        portrait(g, key(id), pic);
        let blocked = matches!(
            activity(&g.guild, id),
            Activity::Away | Activity::Recovering
        );
        if blocked {
            draw_rectangle(pic.x, pic.y, pic.w, pic.h, Color::new(0., 0., 0., 0.5));
        }
        if preparation::selected(g, id) {
            preparation::checkmark(Rect::new(pic.right() - 24., pic.bottom() - 24., 24., 24.));
        }
        text(first(g, id), Rect::new(x, r.y + 68., cw, 23.), 20., INK);
        text(
            &g.guild.text.format(
                "ui.member_contribution",
                &[
                    ("class", a.class.clone()),
                    (
                        "contribution",
                        if q.specialty == a.class || q.specialty == "Any" {
                            format!("+{}", g.guild.config.expedition.specialty_bonus)
                        } else {
                            "+0".into()
                        },
                    ),
                ],
            ),
            Rect::new(x, r.y + 94., cw, 23.),
            17.,
            GOLD,
        );
        text(
            &g.guild
                .text
                .format("ui.fatigue_loss", &[("fatigue", a.fatigue.to_string())]),
            Rect::new(x, r.y + 119., cw, 23.),
            17.,
            MUTED,
        );
        text(
            &if blocked {
                activity(&g.guild, id).label(&g.guild.text).to_string()
            } else {
                g.guild.text.format(
                    "ui.contributes",
                    &[("strength", g.guild.strength(q, &[id]).to_string())],
                )
            },
            Rect::new(x, r.y + 143., cw, 23.),
            17.,
            MUTED,
        );
        if !blocked && g.hq.journey.is_none() && activated(Rect::new(x, r.y, cw, r.h)) {
            return Some(UiAction::Party(id));
        }
    }
    None
}
