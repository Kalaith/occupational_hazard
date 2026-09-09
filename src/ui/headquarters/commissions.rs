//! Compare every current offer before entering party planning.
use super::*;

pub fn draw(g: &Game, r: Rect) -> Option<UiAction> {
    let ids = g.guild.open_contracts(&g.contracts);
    let compact = r.w < 600.;
    text(
        "COMPARE COMMISSIONS",
        Rect::new(r.x, r.y, r.w, 30.),
        24.,
        INK,
    );
    if ids.is_empty() {
        text(
            "No postings today. Tap < Guild, then ADVANCE DAY.",
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
            "Standing trial".into()
        } else {
            format!(
                "Expires day {}",
                q.offer(g.guild.day).expect("open posting").expires
            )
        };
        text(
            &format!("{}g  ·  {} days  ·  {}", q.gold, q.days, expiry),
            Rect::new(row.x + 12., row.y + 36., row.w - 24., 25.),
            18.,
            GOLD,
        );
        let danger = format!("Danger: {}", q.danger);
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
            (false, "< Previous offers", r.x),
            (true, "More offers >", r.x + half + 12.),
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
