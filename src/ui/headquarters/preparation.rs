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
        return "Away · cannot join".into();
    }
    if a.injury > 0 {
        return format!("Medical leave · {} days", a.injury);
    }
    let q = &g.contracts[quest];
    let suitable = q.specialty == a.class || q.specialty == "Any";
    format!(
        "{} {} · Fatigue -{}\nContribution {}",
        a.class,
        if suitable { "fit +3" } else { "support +0" },
        a.fatigue,
        g.guild.strength(q, &[id])
    )
}

pub fn summary(g: &Game, id: usize) -> String {
    let q = &g.contracts[id];
    if let Some(journey) = g.hq.journey {
        let e = &g.guild.expeditions[journey];
        return format!("On the road · Strength {} / difficulty {}\nAccepted preparation is fixed. Returns day {}.",e.strength,q.difficulty,e.returns);
    }
    let power = g.guild.prepared_strength(id, q, &g.party);
    let scout = power - g.guild.strength(q, &g.party);
    let verdict = if power >= q.difficulty + 2 {
        "Well prepared"
    } else if power >= q.difficulty {
        "Close call"
    } else {
        "Outmatched"
    };
    format!(
        "{} · Strength {} / difficulty {}\nScouting +{} to party · {} specialist +3 each",
        verdict, power, q.difficulty, scout, q.specialty
    )
}

pub fn advice(g: &Game, id: usize) -> String {
    if g.hq.journey.is_some() {
        return "This party is already travelling; its preparation cannot change.".into();
    }
    if let Some(reason) = g.guild.dispatch_problem(id, &g.party, &g.contracts) {
        return reason;
    }
    let rest: u32 = g.party.iter().map(|&i| g.guild.roster[i].fatigue).sum();
    let q = &g.contracts[id];
    if q.promotion {
        return "Solo trial: one eligible adventurer, fully rested. No helpers or scouting.".into();
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
        return format!(
            "Add {}: {:+} strength. Full party rest restores +{}.",
            first(g, id),
            power,
            rest
        );
    }
    format!(
        "Full party rest restores +{} strength. {}",
        rest,
        if g.guild.services.scouted.contains(&id) {
            "Scouting is already included."
        } else if q.promotion {
            "The trial must be unaided."
        } else {
            "Scouting adds +2 once."
        }
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
            &format!("{} · {}", first(g, id), g.guild.roster[id].rank()),
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
    let enabled = !q.promotion && !scouted && g.guild.gold >= 20 && g.hq.journey.is_none();
    let title = if q.promotion {
        "Unaided trial · no scouts"
    } else if scouted {
        "Scouted · +2 party strength"
    } else if g.guild.gold < 20 {
        "Scout +2 · needs 20g"
    } else {
        "Scout +2 party strength · 20g"
    };
    panel(r, PANEL);
    label(title, r, 17., if enabled { INK } else { MUTED });
    if enabled && activated(r) {
        Some(UiAction::Purchase(crate::services::Purchase::Scout(id)))
    } else {
        None
    }
}
