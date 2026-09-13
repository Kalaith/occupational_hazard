//! Expedition list and report detail surfaces.
use super::*;

pub fn draw(g: &Game, r: Rect) -> Option<UiAction> {
    if g.report_detail && !g.guild.reports.is_empty() {
        return detail(g, r);
    }
    list(g, r)
}

fn list(g: &Game, r: Rect) -> Option<UiAction> {
    text(
        g.guild.text.get("ui.journeys_returns"),
        Rect::new(r.x, r.y, r.w, 34.),
        24.,
        INK,
    );
    let mut y = r.y + 46.;
    for (id, e) in g.guild.expeditions.iter().enumerate() {
        let names = e
            .party
            .iter()
            .map(|&id| first(g, id))
            .collect::<Vec<_>>()
            .join(" + ");
        if button(
            Rect::new(r.x, y, r.w, 52.),
            &g.guild.text.format(
                "ui.expedition_entry",
                &[
                    ("people", names),
                    ("contract", g.contracts[e.contract].title.clone()),
                    ("day", e.returns.to_string()),
                ],
            ),
            false,
        ) {
            return Some(UiAction::Journey(id));
        }
        y += 60.;
    }
    text(
        &g.guild.text.format(
            "ui.unread_reports",
            &[("count", g.guild.unread_reports().to_string())],
        ),
        Rect::new(r.x, y, r.w, 28.),
        18.,
        GOLD,
    );
    y += 34.;
    let rows = ((r.bottom() - y - 52.) / 58.).floor().max(1.) as usize;
    let pages = g.guild.reports.len().div_ceil(rows).max(1);
    let page = g.report_page.min(pages - 1);
    for (i, (id, report)) in g
        .guild
        .reports
        .iter()
        .enumerate()
        .skip(page * rows)
        .take(rows)
        .enumerate()
    {
        if button(
            Rect::new(r.x, y + i as f32 * 58., r.w, 50.),
            &g.guild.text.format(
                "ui.report_entry",
                &[
                    (
                        "status",
                        if report.read {
                            g.guild.text.get("ui.report_read").to_string()
                        } else {
                            g.guild.text.get("ui.report_new").to_string()
                        },
                    ),
                    ("title", report.title.clone()),
                ],
            ),
            !report.read,
        ) {
            return Some(UiAction::Report(id));
        }
    }
    if g.guild.reports.is_empty() {
        text(
            g.guild.text.get("ui.no_returns"),
            Rect::new(r.x, y, r.w, r.bottom() - y),
            20.,
            MUTED,
        );
    }
    if pages > 1
        && button(
            Rect::new(r.x, r.bottom() - 46., r.w, 44.),
            &g.guild.text.format(
                "ui.more_reports",
                &[
                    ("page", (page + 1).to_string()),
                    ("pages", pages.to_string()),
                ],
            ),
            false,
        )
    {
        return Some(UiAction::ReportPage((page + 1) % pages));
    }
    None
}

fn detail(g: &Game, r: Rect) -> Option<UiAction> {
    if r.h < 420. {
        compact_detail(g, r)
    } else {
        wide_detail(g, r)
    }
}

fn compact_detail(g: &Game, r: Rect) -> Option<UiAction> {
    let report = &g.guild.reports[g.report.min(g.guild.reports.len() - 1)];
    text(
        if g.hq.page == 0 {
            g.guild.text.get("ui.party_returned")
        } else {
            g.guild.text.get("ui.return_account")
        },
        Rect::new(r.x, r.y, r.w, 30.),
        23.,
        INK,
    );
    text(
        if g.hq.page == 0 {
            &report.body
        } else {
            &report.reward
        },
        Rect::new(r.x, r.y + 40., r.w, r.h - 100.),
        18.,
        INK,
    );
    let half = (r.w - 8.) / 2.;
    if button(
        Rect::new(r.x, r.bottom() - 46., half, 44.),
        if g.hq.page == 0 {
            g.guild.text.get("ui.rewards")
        } else {
            g.guild.text.get("ui.people")
        },
        false,
    ) {
        return Some(UiAction::SheetPage(usize::from(g.hq.page == 0)));
    }
    if primary(
        Rect::new(r.x + half + 8., r.bottom() - 46., half, 44.),
        g.guild.text.get("ui.acknowledge"),
        true,
    ) {
        return Some(UiAction::ReportList);
    }
    None
}

fn wide_detail(g: &Game, r: Rect) -> Option<UiAction> {
    let report = &g.guild.reports[g.report.min(g.guild.reports.len() - 1)];
    let people: Vec<usize> = g
        .guild
        .roster
        .iter()
        .enumerate()
        .filter(|(_, a)| report.body.contains(&a.name) || report.title.contains(&a.name))
        .map(|(id, _)| id)
        .collect();
    let title = report.title.split(" / ").last().unwrap_or(&report.title);
    text(
        g.guild.text.get("ui.party_returned"),
        Rect::new(r.x, r.y, r.w, 36.),
        27.,
        INK,
    );
    text(title, Rect::new(r.x, r.y + 40., r.w, 40.), 20., MUTED);
    let portrait_size = (r.w * 0.34).min(r.h * 0.25);
    let y = r.y + 92.;
    for (i, &id) in people.iter().enumerate() {
        let size = if people.len() > 1 {
            (r.w / 3. - 8.).min(portrait_size)
        } else {
            portrait_size
        };
        portrait(
            g,
            key(id),
            Rect::new(r.x + i as f32 * (size + 8.), y, size, size),
        );
    }
    if people.len() <= 1 {
        let x = r.x + portrait_size + 18.;
        text(
            people
                .first()
                .map_or(g.guild.text.get("ui.guild_record"), |&id| {
                    g.guild.roster[id].name.as_str()
                }),
            Rect::new(x, y + 12., r.right() - x, 40.),
            21.,
            INK,
        );
        text(
            if report.title.contains(g.guild.text.get("ui.success")) {
                g.guild.text.get("ui.success")
            } else if report.title.contains(g.guild.text.get("ui.retreat")) {
                g.guild.text.get("ui.retreat")
            } else {
                g.guild.text.get("ui.certified")
            },
            Rect::new(x, y + 60., r.right() - x, 35.),
            24.,
            GOLD,
        );
    }
    let body_y = y + portrait_size + 20.;
    rule(r, body_y);
    text(
        if report.body.contains(g.guild.text.get("activity.medical")) {
            g.guild.text.get("ui.medical_recovering")
        } else {
            g.guild.text.get("ui.rest_recommended")
        },
        Rect::new(r.x, body_y + 10., r.w, 26.),
        17.,
        GOLD,
    );
    text(
        &report.body,
        Rect::new(
            r.x,
            body_y + 42.,
            r.w,
            (r.bottom() - body_y - 187.).max(40.),
        ),
        18.,
        INK,
    );
    rule(r, r.bottom() - 132.);
    text(
        &report.reward,
        Rect::new(r.x, r.bottom() - 116., r.w, 58.),
        20.,
        GOLD,
    );
    if primary(
        Rect::new(r.x, r.bottom() - 48., r.w, 46.),
        g.guild.text.get("ui.acknowledge_all"),
        true,
    ) {
        return Some(UiAction::ReportList);
    }
    None
}
