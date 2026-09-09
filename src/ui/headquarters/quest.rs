//! A commission is an open journal, not a permanent sidebar.
use super::*;

pub fn draw(g: &Game, r: Rect) -> Option<UiAction> {
    if g.guild.open_contracts(&g.contracts).is_empty() && g.hq.journey.is_none() {
        text(
            "No commissions posted today. Return to the guild and tap ADVANCE DAY.",
            r,
            24.,
            INK,
        );
        return None;
    }
    let journey = g.hq.journey.and_then(|id| g.guild.expeditions.get(id));
    let id = journey.map_or(g.selected, |e| e.contract);
    let q = &g.contracts[id];
    let left = Rect::new(r.x, r.y, r.w * 0.49, r.h);
    let right = Rect::new(left.right() + 30., r.y, r.right() - left.right() - 30., r.h);
    draw_line(
        left.right() + 15.,
        r.y,
        left.right() + 15.,
        r.bottom(),
        1.,
        Color::new(0.32, 0.30, 0.25, 1.),
    );
    let mut action = None;
    text(
        &q.title.to_uppercase(),
        Rect::new(left.x, left.y, left.w, 58.),
        28.,
        INK,
    );
    let art = g
        .assets
        .get_texture("route")
        .expect("Required route illustration");
    let picture = Rect::new(left.x, left.y + 64., left.w, 155.);
    draw_texture_ex(
        art,
        picture.x,
        picture.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(picture.size()),
            source: Some(Rect::new(0., 210., 1536., 700.)),
            ..Default::default()
        },
    );
    draw_rectangle_lines(picture.x, picture.y, picture.w, picture.h, 1., GOLD);
    text(
        &q.client,
        Rect::new(left.x, picture.bottom() + 12., left.w, 25.),
        16.,
        GOLD,
    );
    text(
        &q.brief,
        Rect::new(left.x, picture.bottom() + 45., left.w, 84.),
        19.,
        INK,
    );
    text(
        &format!(
            "{}    /    {} days    /    {}g",
            if q.promotion {
                "SOLO TRIAL"
            } else if q.bronze {
                "BRONZE"
            } else {
                "IRON"
            },
            q.days,
            q.gold
        ),
        Rect::new(left.x, left.bottom() - 124., left.w, 28.),
        20.,
        GOLD,
    );
    text(
        &format!("Danger: {}", q.danger),
        Rect::new(left.x, left.bottom() - 87., left.w, 30.),
        18.,
        INK,
    );
    if journey.is_none() {
        let half = (left.w - 12.) / 2.;
        for (forward, name, x) in [
            (false, "< Previous job", left.x),
            (true, "Next job >", left.x + half + 12.),
        ] {
            if button(Rect::new(x, left.bottom() - 46., half, 44.), name, false) {
                action = Some(UiAction::Quest(g.guild.adjacent_contract(
                    id,
                    forward,
                    &g.contracts,
                )));
            }
        }
    }
    text(
        if journey.is_some() {
            "THE TRAVELLING PARTY"
        } else {
            "CHOOSE YOUR PARTY"
        },
        Rect::new(right.x, right.y + 4., right.w, 32.),
        19.,
        GOLD,
    );
    if let Some(next) = planning::cards(g, Rect::new(right.x, right.y + 47., right.w, 170.)) {
        action = Some(next);
    }
    let problem = g.guild.dispatch_problem(id, &g.party, &g.contracts);
    text(
        if journey.is_some() {
            "Accepted commission. These adventurers are on the road."
        } else {
            problem
                .as_deref()
                .unwrap_or_else(|| planning::readiness(g, id))
        },
        Rect::new(right.x, right.y + 218., right.w, 50.),
        18.,
        INK,
    );
    text(
        &g.guild.offer_notice(q),
        Rect::new(right.x, right.y + 276., right.w, 57.),
        16.,
        MUTED,
    );
    let scouted = g.guild.services.scouted.contains(&id);
    if journey.is_none() {
        let can_scout = !q.promotion && !scouted && g.guild.gold >= 20;
        if primary(
            Rect::new(right.x, right.bottom() - 145., right.w, 44.),
            if q.promotion {
                "Unaided trial · no scouting"
            } else if scouted {
                "Route scouted for next dispatch"
            } else if g.guild.gold < 20 {
                "Scouting needs 20g"
            } else {
                "Scout this route · 20g"
            },
            can_scout,
        ) {
            action = Some(UiAction::Purchase(crate::services::Purchase::Scout(id)));
        }
        text(
            "Scouting applies to this route's next dispatch only.",
            Rect::new(right.x, right.bottom() - 96., right.w, 24.),
            14.,
            MUTED,
        );
    }
    let day = journey.map_or(g.guild.day + q.days, |e| e.returns);
    text(
        &format!(
            "Return · day {}     {}",
            day,
            if day <= 30 {
                "Before review"
            } else {
                "After review"
            }
        ),
        Rect::new(right.x, right.bottom() - 70., right.w, 24.),
        16.,
        GOLD,
    );
    if journey.is_some() {
        label(
            "ON THE ROAD",
            Rect::new(right.x, right.bottom() - 43., right.w, 43.),
            22.,
            GOLD,
        );
    } else if primary(
        Rect::new(right.x, right.bottom() - 46., right.w, 46.),
        "DISPATCH PARTY",
        problem.is_none(),
    ) {
        action = Some(UiAction::Dispatch);
    }
    action
}
