//! Contract review, party selection and the day ledger.
use super::*;

pub fn draw_desk(g: &Game) -> Option<UiAction> {
    let w = screen_width();
    let h = screen_height();
    let compact = w < 760.0;
    let mut action = None;
    label(
        &format!("DAY {} / {}g", g.guild.day, g.guild.gold),
        Rect::new(12.0, 8.0, w - 232.0, 40.0),
        if compact { 19.0 } else { 26.0 },
        GOLD,
    );
    let (certs, commissions) = g.guild.objective_counts(&g.contracts);
    let objective = if g.guild.month.sandbox {
        "SANDBOX".into()
    } else {
        format!(
            "D30: {}/2",
            usize::from(certs > 0) + usize::from(commissions > 0)
        )
    };
    if guided_button(g, Rect::new(w - 212.0, 8.0, 114.0, 42.0), &objective, false) {
        action = Some(UiAction::Month);
    }
    if guided_button(g, Rect::new(w - 90.0, 8.0, 78.0, 42.0), "MENU", false) {
        action = Some(UiAction::Settings);
    }
    let tabs = ["CONTRACTS", "ADVENTURERS", "REPORTS"];
    for (i, text) in tabs.iter().enumerate() {
        let tw = (w - 32.0) / 3.0;
        if guided_button(
            g,
            Rect::new(12.0 + i as f32 * (tw + 4.0), 58.0, tw, 44.0),
            text,
            g.tab == i,
        ) {
            action = Some(UiAction::Tab(i));
        }
    }
    let guidance = g.lesson().is_some();
    let top = if guidance { 238.0 } else { 184.0 };
    if guidance {
        if let Some(next) = help::draw_guidance(g, Rect::new(12.0, 110.0, w - 24.0, 116.0)) {
            action = Some(next);
        }
    } else {
        portrait(g, "elowen", Rect::new(12.0, 112.0, 60.0, 60.0));
        paragraph(
            if g.notice.is_empty() {
                g.hint()
            } else {
                &g.notice
            },
            Rect::new(84.0, 113.0, w - 100.0, 58.0),
            18.0,
            MUTED,
        );
    }
    let area = Rect::new(12.0, top, w - 24.0, h - top - 70.0);
    let inner = match g.tab {
        0 => contracts(g, area),
        1 => dossier::draw_dossier(g, area),
        3 => services::draw_services(g, area),
        _ => reports(g, area),
    };
    if inner.is_some() {
        action = inner;
    }
    let travelling = g.guild.expeditions.len();
    if guided_button(
        g,
        Rect::new(12.0, h - 58.0, w - 180.0, 44.0),
        &format!("GUILD SERVICES / {travelling} away"),
        g.tab == 3,
    ) {
        action = Some(UiAction::Tab(3));
    }
    if guided_button(
        g,
        Rect::new(w - 162.0, h - 58.0, 150.0, 46.0),
        "NEXT DAY",
        true,
    ) {
        action = Some(UiAction::NextDay);
    }
    action
}

fn contracts(g: &Game, area: Rect) -> Option<UiAction> {
    if area.w < 850.0 || area.h < 420.0 {
        return compact_contract(g, area);
    }
    let mut action = None;
    let wide = area.w >= 850.0;
    let list_w = if wide { area.w * 0.26 } else { 0.0 };
    if wide {
        label(
            "CONTRACT REGISTER",
            Rect::new(area.x, area.y, list_w - 12.0, 34.0),
            20.0,
            GOLD,
        );
        for (i, q) in g.contracts.iter().enumerate() {
            let y = area.y + 45.0 + i as f32 * 57.0;
            let tag = if q.promotion {
                "TRIAL"
            } else if q.bronze {
                "BRONZE"
            } else {
                "IRON"
            };
            if guided_button(
                g,
                Rect::new(area.x, y, list_w - 12.0, 49.0),
                &format!("{} / {}", tag, q.title),
                g.selected == i,
            ) {
                action = Some(UiAction::Quest(i));
            }
        }
    }
    let r = Rect::new(area.x + list_w, area.y, area.w - list_w, area.h);
    panel(r, PAPER);
    let q = &g.contracts[g.selected];
    let pad = if wide { 22.0 } else { 12.0 };
    let x = r.x + pad;
    let width = r.w - pad * 2.0;
    if !wide {
        if guided_button(g, Rect::new(x, r.y + 8.0, 50.0, 44.0), "<", false) {
            action = Some(UiAction::Quest(
                (g.selected + g.contracts.len() - 1) % g.contracts.len(),
            ));
        }
        if guided_button(
            g,
            Rect::new(x + width - 50.0, r.y + 8.0, 50.0, 44.0),
            ">",
            false,
        ) {
            action = Some(UiAction::Quest((g.selected + 1) % g.contracts.len()));
        }
    }
    label(
        &q.title,
        Rect::new(
            x + if wide { 0.0 } else { 56.0 },
            r.y + 9.0,
            width - if wide { 0.0 } else { 112.0 },
            40.0,
        ),
        28.0,
        INK,
    );
    label(
        &format!(
            "{}  /  {} day(s)  /  {}g  /  {} XP",
            if q.promotion {
                "TRIAL"
            } else if q.bronze {
                "BRONZE"
            } else {
                "IRON"
            },
            q.days,
            q.gold,
            q.xp
        ),
        Rect::new(x, r.y + 54.0, width, 24.0),
        18.0,
        INK,
    );
    let brief_height = (r.h - 348.0).clamp(40.0, 130.0);
    paragraph(
        &format!("{}\n{}", q.client, q.brief),
        Rect::new(x, r.y + 84.0, width, brief_height),
        20.0,
        INK,
    );
    let base = r.y + 86.0 + brief_height;
    label(
        &format!(
            "Danger: {}{}",
            q.danger,
            if g.guild.services.scouted.contains(&g.selected) {
                " / SCOUTED"
            } else {
                ""
            }
        ),
        Rect::new(x, base, width, 24.0),
        18.0,
        INK,
    );
    let party_y = base + 30.0;
    let row_h = 42.0;
    for (i, a) in g.guild.roster.iter().enumerate() {
        let status = if g.guild.busy(i) {
            "AWAY".into()
        } else if a.injury > 0 {
            format!("REST {}d", a.injury)
        } else {
            format!("fatigue {}/6", a.fatigue)
        };
        let text = format!(
            "{} {} / {} / {}",
            if g.party.contains(&i) { "+" } else { " " },
            a.name,
            a.class,
            status
        );
        if guided_button(
            g,
            Rect::new(x, party_y + i as f32 * (row_h + 4.0), width, row_h),
            &text,
            g.party.contains(&i),
        ) {
            action = Some(UiAction::Party(i));
        }
    }
    let cutoff = g.guild.cutoff_notice(q.days);
    let problem = g.guild.dispatch_problem(g.selected, &g.party, &g.contracts);
    let assessment = if g.party.is_empty() {
        "Select your party above."
    } else if g.guild.prepared_strength(g.selected, q, &g.party) >= q.difficulty + 2 {
        "Well prepared. Rest still matters."
    } else if g.guild.prepared_strength(g.selected, q, &g.party) >= q.difficulty {
        "A close call is likely. Consider support or rest."
    } else {
        "Outmatched. Bring support or rest first."
    };
    let bottom = r.y + r.h;
    // The compact screen keeps the exact dispatch requirement visible above its button.
    paragraph(
        if g.guild.day + q.days > 30 && !g.guild.month.sandbox {
            &cutoff
        } else {
            problem.as_deref().unwrap_or(assessment)
        },
        Rect::new(x, bottom - 81.0, width, 32.0),
        16.0,
        INK,
    );
    if guided_button(
        g,
        Rect::new(x, bottom - 46.0, width, 40.0),
        "DISPATCH",
        problem.is_none(),
    ) {
        action = Some(UiAction::Dispatch);
    }
    action
}

fn compact_contract(g: &Game, r: Rect) -> Option<UiAction> {
    panel(r, PAPER);
    let q = &g.contracts[g.selected];
    let x = r.x + 12.0;
    let w = r.w - 24.0;
    if g.choosing_party {
        label(&q.title, Rect::new(x, r.y + 8.0, w, 36.0), 24.0, INK);
        for (id, a) in g.guild.roster.iter().enumerate() {
            let status = if g.guild.busy(id) {
                "AWAY".into()
            } else if a.injury > 0 {
                format!("REST {}d", a.injury)
            } else {
                format!("Fatigue {}/6", a.fatigue)
            };
            if guided_button(
                g,
                Rect::new(x, r.y + 52.0 + id as f32 * 50.0, w, 44.0),
                &format!("{} / {} / {}", a.name, a.class, status),
                g.party.contains(&id),
            ) {
                return Some(UiAction::Party(id));
            }
        }
        let cutoff = g.guild.cutoff_notice(q.days);
        let problem = g.guild.dispatch_problem(g.selected, &g.party, &g.contracts);
        let assessment = if g.guild.prepared_strength(g.selected, q, &g.party) >= q.difficulty + 2 {
            "Well prepared. Tap DISPATCH to send this party."
        } else if g.guild.prepared_strength(g.selected, q, &g.party) >= q.difficulty {
            "A close call is likely. Consider support or rest before DISPATCH."
        } else {
            "Outmatched. Select more support, or tap NEXT DAY to rest."
        };
        paragraph(
            if g.guild.day + q.days > 30 && !g.guild.month.sandbox {
                &cutoff
            } else {
                problem.as_deref().unwrap_or(assessment)
            },
            Rect::new(x, r.y + 210.0, w, r.h - 272.0),
            20.0,
            INK,
        );
        if guided_button(
            g,
            Rect::new(x, r.y + r.h - 52.0, w * 0.4, 44.0),
            "BACK",
            false,
        ) {
            return Some(UiAction::ChooseParty(false));
        }
        if guided_button(
            g,
            Rect::new(x + w * 0.42, r.y + r.h - 52.0, w * 0.58, 44.0),
            "DISPATCH",
            problem.is_none(),
        ) {
            return Some(UiAction::Dispatch);
        }
    } else {
        if guided_button(g, Rect::new(x, r.y + 8.0, 48.0, 44.0), "<", false) {
            return Some(UiAction::Quest(
                (g.selected + g.contracts.len() - 1) % g.contracts.len(),
            ));
        }
        if guided_button(
            g,
            Rect::new(x + w - 48.0, r.y + 8.0, 48.0, 44.0),
            ">",
            false,
        ) {
            return Some(UiAction::Quest((g.selected + 1) % g.contracts.len()));
        }
        label(
            &q.title,
            Rect::new(x + 54.0, r.y + 8.0, w - 108.0, 44.0),
            23.0,
            INK,
        );
        label(
            &format!(
                "{} / {} day(s) / {}g / {} XP",
                if q.promotion {
                    "TRIAL"
                } else if q.bronze {
                    "BRONZE"
                } else {
                    "IRON"
                },
                q.days,
                q.gold,
                q.xp
            ),
            Rect::new(x, r.y + 62.0, w, 26.0),
            19.0,
            INK,
        );
        paragraph(
            &format!("{}\n{}", q.client, q.brief),
            Rect::new(x, r.y + 100.0, w, r.h - 220.0),
            22.0,
            INK,
        );
        paragraph(
            &format!(
                "Danger: {}{}",
                q.danger,
                if g.guild.services.scouted.contains(&g.selected) {
                    " / SCOUTED"
                } else {
                    ""
                }
            ),
            Rect::new(x, r.y + r.h - 109.0, w, 44.0),
            19.0,
            INK,
        );
        if guided_button(
            g,
            Rect::new(x, r.y + r.h - 52.0, w, 44.0),
            "CHOOSE PARTY",
            true,
        ) {
            return Some(UiAction::ChooseParty(true));
        }
    }
    None
}

fn reports(g: &Game, r: Rect) -> Option<UiAction> {
    panel(r, PAPER);
    let x = r.x + 18.0;
    let width = r.w - 36.0;
    if g.guild.reports.is_empty() {
        label(
            "NO RETURN REPORTS YET",
            Rect::new(x, r.y + 16.0, width, 38.0),
            24.0,
            INK,
        );
        paragraph("Dispatch an expedition from CONTRACTS, then tap NEXT DAY until it returns. Rewards and experience are recorded automatically.",
            Rect::new(x, r.y + 72.0, width, 100.0), 22.0, INK);
    } else {
        let index = g.report.min(g.guild.reports.len() - 1);
        let report = &g.guild.reports[index];
        label(
            &report.title,
            Rect::new(x, r.y + 12.0, width, 42.0),
            25.0,
            INK,
        );
        paragraph(
            &report.body,
            Rect::new(x, r.y + 65.0, width, r.h * 0.36),
            23.0,
            INK,
        );
        paragraph(
            &report.reward,
            Rect::new(x, r.y + r.h * 0.55, width, 50.0),
            21.0,
            INK,
        );
        let bw = (width - 12.0) / 2.0;
        if guided_button(
            g,
            Rect::new(x, r.y + r.h - 54.0, bw, 44.0),
            "NEWER REPORT",
            false,
        ) {
            return Some(UiAction::Report(index.saturating_sub(1)));
        }
        if guided_button(
            g,
            Rect::new(x + bw + 12.0, r.y + r.h - 54.0, bw, 44.0),
            "OLDER REPORT",
            false,
        ) {
            return Some(UiAction::Report((index + 1).min(g.guild.reports.len() - 1)));
        }
    }
    if !g.guild.expeditions.is_empty() {
        let text = g
            .guild
            .expeditions
            .iter()
            .map(|e| {
                format!(
                    "{}: returns day {}",
                    g.contracts[e.contract].title, e.returns
                )
            })
            .collect::<Vec<_>>()
            .join(" / ");
        paragraph(
            &text,
            Rect::new(x, r.y + r.h - 123.0, width, 60.0),
            18.0,
            INK,
        );
    }
    None
}
