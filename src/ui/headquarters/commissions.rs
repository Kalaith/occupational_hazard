//! Compare every current offer before entering party planning.
use super::*;

pub fn draw(g: &Game, r: Rect) -> Option<UiAction> {
    let ids = g.guild.open_contracts(&g.contracts);
    let compact = r.w < 600.;
    text(
        g.guild.text.get("ui.compare_commissions"),
        Rect::new(r.x, r.y, r.w, 30.),
        24.,
        INK,
    );
    if ids.is_empty() {
        text(
            g.guild.text.get("ui.no_postings"),
            Rect::new(r.x, r.y + 48., r.w, 100.),
            21.,
            MUTED,
        );
        return None;
    }
    let row_h = if compact { 110. } else { 80. };
    let rows = ((r.h - 94.) / row_h).floor().max(1.) as usize;
    let pages = ids.len().div_ceil(rows);
    let page = g.board_page.min(pages - 1);
    for (slot, &id) in ids.iter().skip(page * rows).take(rows).enumerate() {
        let q = &g.contracts[id];
        let row = Rect::new(r.x, r.y + 42. + slot as f32 * row_h, r.w, row_h - 8.);
        panel(row, PANEL);
        text(
            &q.title,
            Rect::new(row.x + 12., row.y + 7., row.w - 24., 27.),
            21.,
            INK,
        );
        let expiry = if q.promotion {
            g.guild.text.get("ui.standing_trial").into()
        } else {
            g.guild.text.format(
                "ui.expires_day",
                &[(
                    "day",
                    q.offer(g.guild.day)
                        .expect("open posting")
                        .expires
                        .to_string(),
                )],
            )
        };
        text(
            &g.guild.text.format(
                "ui.commission_stats",
                &[
                    ("gold", q.gold.to_string()),
                    ("days", q.days.to_string()),
                    ("expiry", expiry),
                ],
            ),
            Rect::new(row.x + 12., row.y + 36., row.w - 24., 25.),
            18.,
            GOLD,
        );
        let danger = g
            .guild
            .text
            .format("ui.danger", &[("danger", q.danger.clone())]);
        text(
            &danger,
            if compact {
                Rect::new(row.x + 12., row.y + 65., row.w - 24., 30.)
            } else {
                Rect::new(row.x + row.w * 0.53, row.y + 36., row.w * 0.47 - 12., 30.)
            },
            18.,
            MUTED,
        );
        if activated(row) {
            return Some(UiAction::Quest(id));
        }
    }
    if pages > 1 {
        let half = (r.w - 12.) / 2.;
        for (next, title, x) in [
            (false, g.guild.text.get("ui.previous_offers"), r.x),
            (true, g.guild.text.get("ui.more_offers"), r.x + half + 12.),
        ] {
            if button(Rect::new(x, r.bottom() - 44., half, 44.), title, false) {
                return Some(UiAction::CommissionPage(
                    (page + if next { 1 } else { pages - 1 }) % pages,
                ));
            }
        }
    }
    None
}
