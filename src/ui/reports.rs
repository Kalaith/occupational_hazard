//! A visible inbox for every expedition, including simultaneous returns.
use super::*;

pub fn draw_reports(g: &Game, r: Rect) -> Option<UiAction> {
    panel(r, PAPER);
    let x = r.x + 12.0;
    let w = r.w - 24.0;
    if g.report_detail && !g.guild.reports.is_empty() {
        return detail(g, r);
    }
    label(
        &format!(
            "{} REPORTS / {} UNREAD",
            g.guild.reports.len(),
            g.guild.unread_reports()
        ),
        Rect::new(x, r.y + 8.0, w, 32.0),
        23.0,
        INK,
    );
    let rows = ((r.h - 120.0) / 60.0).floor().max(1.0) as usize;
    let pages = g.guild.reports.len().div_ceil(rows).max(1);
    let page = g.report_page.min(pages - 1);
    for (row, (id, report)) in g
        .guild
        .reports
        .iter()
        .enumerate()
        .skip(page * rows)
        .take(rows)
        .enumerate()
    {
        if button(
            Rect::new(x, r.y + 48.0 + row as f32 * 60.0, w, 54.0),
            &format!(
                "{} {}",
                if report.read { "READ /" } else { "NEW /" },
                report.title
            ),
            !report.read,
        ) {
            return Some(UiAction::Report(id));
        }
    }
    if g.guild.reports.is_empty() {
        paragraph("No returns yet. DISPATCH a party from CONTRACTS, then tap NEXT DAY. Each returning expedition gets its own report here.",
            Rect::new(x, r.y + 55.0, w, r.h - 130.0), 21.0, INK);
    }
    label(
        &format!(
            "Page {}/{} / {} expeditions away",
            page + 1,
            pages,
            g.guild.expeditions.len()
        ),
        Rect::new(x, r.y + r.h - 74.0, w, 22.0),
        17.0,
        INK,
    );
    if pages > 1 {
        if button(
            Rect::new(x, r.y + r.h - 48.0, w * 0.48, 42.0),
            "NEWER",
            false,
        ) {
            return Some(UiAction::ReportPage(page.saturating_sub(1)));
        }
        if button(
            Rect::new(x + w * 0.52, r.y + r.h - 48.0, w * 0.48, 42.0),
            "OLDER",
            false,
        ) {
            return Some(UiAction::ReportPage((page + 1).min(pages - 1)));
        }
    }
    None
}

fn detail(g: &Game, r: Rect) -> Option<UiAction> {
    let index = g.report.min(g.guild.reports.len() - 1);
    let report = &g.guild.reports[index];
    let x = r.x + 14.0;
    let w = r.w - 28.0;
    label(&report.title, Rect::new(x, r.y + 8.0, w, 44.0), 23.0, INK);
    paragraph(
        &report.body,
        Rect::new(x, r.y + 62.0, w, r.h - 190.0),
        22.0,
        INK,
    );
    paragraph(
        &report.reward,
        Rect::new(x, r.y + r.h - 120.0, w, 54.0),
        20.0,
        INK,
    );
    if button(
        Rect::new(x, r.y + r.h - 52.0, w, 44.0),
        &format!("ALL REPORTS / {} unread", g.guild.unread_reports()),
        false,
    ) {
        return Some(UiAction::ReportList);
    }
    None
}
