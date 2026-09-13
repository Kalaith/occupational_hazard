//! Explain the same contributions used by the deterministic resolution rules.
use super::*;

pub fn selected(g: &Game, id: usize) -> bool {
    g.hq.journey.map_or_else(
        || g.party.contains(&id),
        |e| g.guild.expeditions[e].party.contains(&id),
    )
}

pub fn checkmark(r: Rect) {
    panel(r, GOLD);
    draw_line(
        r.x + 5.,
        r.y + r.h * 0.52,
        r.x + r.w * 0.43,
        r.bottom() - 6.,
        3.,
        BLACK,
    );
    draw_line(
        r.x + r.w * 0.43,
        r.bottom() - 6.,
        r.right() - 4.,
        r.y + 5.,
        3.,
        BLACK,
    );
}

pub fn member(g: &Game, id: usize, quest: usize) -> String {
    let a = &g.guild.roster[id];
    if g.guild.busy(id) {
        return g.guild.text.get("ui.away_cannot_join").into();
    }
    if a.injury > 0 {
        return g
            .guild
            .text
            .format("ui.medical_days", &[("days", a.injury.to_string())]);
    }
    let q = &g.contracts[quest];
    let specialty_bonus = g.guild.config.expedition.specialty_bonus;
    let suitable = q.specialty == a.class || q.specialty == "Any";
    g.guild.text.format(
        "ui.member_detail",
        &[
            ("class", a.class.clone()),
            (
                "contribution",
                if suitable {
                    g.guild
                        .text
                        .format("ui.fit_strength", &[("bonus", specialty_bonus.to_string())])
                } else {
                    g.guild.text.get("ui.support_strength").into()
                },
            ),
            ("fatigue", a.fatigue.to_string()),
            ("strength", g.guild.strength(q, &[id]).to_string()),
        ],
    )
}

pub fn summary(g: &Game, id: usize) -> String {
    let q = &g.contracts[id];
    if let Some(journey) = g.hq.journey {
        let e = &g.guild.expeditions[journey];
        return g.guild.text.format(
            "ui.on_the_road",
            &[
                ("strength", e.strength.to_string()),
                ("difficulty", q.difficulty.to_string()),
                ("day", e.returns.to_string()),
            ],
        );
    }
    let power = g.guild.prepared_strength(id, q, &g.party);
    let scout = power - g.guild.strength(q, &g.party);
    let verdict = if power >= q.difficulty + g.guild.config.expedition.close_call_margin {
        g.guild.text.get("ui.well_prepared")
    } else if power >= q.difficulty {
        g.guild.text.get("ui.close_call")
    } else {
        g.guild.text.get("ui.outmatched")
    };
    g.guild.text.format(
        "ui.preparation_summary",
        &[
            ("verdict", verdict.to_string()),
            ("power", power.to_string()),
            ("difficulty", q.difficulty.to_string()),
            ("scout", scout.to_string()),
            ("specialty", q.specialty.clone()),
            (
                "bonus",
                g.guild.config.expedition.specialty_bonus.to_string(),
            ),
        ],
    )
}

pub fn advice(g: &Game, id: usize) -> String {
    if g.hq.journey.is_some() {
        return g.guild.text.get("ui.preparation_unchanged").into();
    }
    if let Some(reason) = g.guild.dispatch_problem(id, &g.party, &g.contracts) {
        return reason;
    }
    let rest: u32 = g.party.iter().map(|&i| g.guild.roster[i].fatigue).sum();
    let q = &g.contracts[id];
    if q.promotion {
        return g.guild.text.get("ui.trial_readiness").into();
    }
    if let Some((id, power)) = g
        .guild
        .roster
        .iter()
        .enumerate()
        .filter(|(i, a)| !g.party.contains(i) && !g.guild.busy(*i) && a.injury == 0)
        .map(|(i, _)| (i, g.guild.strength(q, &[i])))
        .filter(|(_, power)| *power > 0)
        .max_by_key(|(_, p)| *p)
    {
        return g.guild.text.format(
            "ui.add_member",
            &[
                ("member", first(g, id).to_string()),
                ("strength", format!("{power:+}")),
                ("rest", rest.to_string()),
            ],
        );
    }
    g.guild.text.format(
        "ui.full_party_rest",
        &[
            ("rest", rest.to_string()),
            (
                "note",
                if g.guild.services.scouted.contains(&id) {
                    g.guild.text.get("ui.scouting_included").to_string()
                } else if q.promotion {
                    g.guild.text.get("ui.trial_unaided").to_string()
                } else {
                    g.guild.text.format(
                        "ui.scouting_adds",
                        &[(
                            "bonus",
                            g.guild.config.services.scout_strength_bonus.to_string(),
                        )],
                    )
                },
            ),
        ],
    )
}

pub fn members(g: &Game, r: Rect, quest: usize) -> Option<UiAction> {
    let row_h = (r.h / 3.).min(105.);
    for id in 0..g.guild.roster.len() {
        let row = Rect::new(r.x, r.y + id as f32 * row_h, r.w, row_h - 6.);
        panel(row, PANEL);
        let pic = Rect::new(
            row.x + 6.,
            row.y + 6.,
            (row.h - 12.).min(74.),
            (row.h - 12.).min(74.),
        );
        portrait(g, key(id), pic);
        if selected(g, id) {
            checkmark(Rect::new(pic.right() - 24., pic.bottom() - 24., 24., 24.));
            draw_rectangle_lines(row.x, row.y, row.w, row.h, 2., GOLD);
        }
        let x = pic.right() + 12.;
        text(
            &format!(
                "{} · {}",
                first(g, id),
                g.guild.roster[id].rank(&g.guild.text)
            ),
            Rect::new(x, row.y + 4., row.right() - x - 8., 24.),
            20.,
            INK,
        );
        text(
            &member(g, id, quest),
            Rect::new(x, row.y + 30., row.right() - x - 8., row.h - 32.),
            17.,
            MUTED,
        );
        if activated(row)
            && g.hq.journey.is_none()
            && !matches!(
                activity(&g.guild, id),
                Activity::Away | Activity::Recovering
            )
        {
            return Some(UiAction::Party(id));
        }
    }
    None
}

pub fn scout(g: &Game, r: Rect, id: usize) -> Option<UiAction> {
    let q = &g.contracts[id];
    let scouted = g.guild.services.scouted.contains(&id);
    let scout_cost = g.guild.config.services.scout_cost;
    let scout_bonus = g.guild.config.services.scout_strength_bonus;
    let enabled = !q.promotion && !scouted && g.guild.gold >= scout_cost && g.hq.journey.is_none();
    let title = if q.promotion {
        g.guild.text.get("ui.unaided_trial").into()
    } else if scouted {
        g.guild
            .text
            .format("ui.scouted", &[("bonus", scout_bonus.to_string())])
    } else if g.guild.gold < scout_cost {
        g.guild.text.format(
            "ui.scout_needs",
            &[
                ("bonus", scout_bonus.to_string()),
                ("cost", scout_cost.to_string()),
            ],
        )
    } else {
        g.guild.text.format(
            "ui.scout_strength",
            &[
                ("bonus", scout_bonus.to_string()),
                ("cost", scout_cost.to_string()),
            ],
        )
    };
    panel(r, PANEL);
    label(&title, r, 17., if enabled { INK } else { MUTED });
    if enabled && activated(r) {
        Some(UiAction::Purchase(crate::services::Purchase::Scout(id)))
    } else {
        None
    }
}
