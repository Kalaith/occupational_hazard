use super::*;

pub fn draw(g: &Game, r: Rect) -> Option<UiAction> {
    let open = g.guild.open_contracts(&g.contracts);
    if open.is_empty() && g.hq.journey.is_none() {
        text(
            "No offers today.\n\nTap ADVANCE DAY for new postings and returning staff.",
            r,
            22.,
            INK,
        );
        return None;
    }
    let quest =
        g.hq.journey
            .and_then(|id| g.guild.expeditions.get(id))
            .map_or(g.selected, |e| e.contract);
    let q = &g.contracts[quest];
    let mut action = None;
    let short = r.h < 420.;
    let detail = g.hq.page == 1;
    // The sheet has explicit pages on short screens; controls never shrink away.
    let header_y = r.y - 54.;
    if button(
        Rect::new(r.right() - 112., header_y, 112., 44.),
        if detail { "Party" } else { "Details" },
        detail,
    ) {
        action = Some(UiAction::SheetPage(usize::from(!detail)));
    }
    if detail {
        return details(g, r, quest).or(action);
    }
    text(
        &q.title.to_uppercase(),
        Rect::new(r.x, r.y, r.w, 42.),
        25.,
        INK,
    );
    if short {
        text(
            &format!(
                "{} · {} days · {}g · {}",
                if q.bronze { "BRONZE" } else { "IRON" },
                q.days,
                q.gold,
                q.danger
            ),
            Rect::new(r.x, r.y + 42., r.w, 24.),
            16.,
            GOLD,
        );
        let card_y = r.y + 64.;
        if let Some(next) = cards(g, Rect::new(r.x, card_y, r.w, 80.)) {
            action = Some(next);
        }
        if primary(
            Rect::new(r.x, r.bottom() - 46., r.w, 44.),
            "REVIEW DISPATCH",
            true,
        ) {
            action = Some(UiAction::SheetPage(2));
        }
        if g.hq.page == 2 {
            return confirmation(g, r, quest);
        }
        return action;
    }
    let image_h = (r.h * 0.17).min(110.);
    let image_rect = Rect::new(r.x, r.y + 42., r.w, image_h);
    let route = g
        .assets
        .get_texture("route")
        .expect("Required route illustration");
    draw_texture_ex(
        route,
        image_rect.x,
        image_rect.y,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(0., 300., 1536., 540.)),
            dest_size: Some(image_rect.size()),
            ..Default::default()
        },
    );
    draw_rectangle_lines(
        image_rect.x,
        image_rect.y,
        image_rect.w,
        image_rect.h,
        1.,
        GOLD,
    );
    let y = image_rect.bottom() + 10.;
    text(&q.brief, Rect::new(r.x, y, r.w, 34.), 17., INK);
    text(
        &format!(
            "{}     /     {} days     /     {}g",
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
        Rect::new(r.x, y + 38., r.w, 26.),
        18.,
        INK,
    );
    text(
        &format!("!  Danger: {}", q.danger),
        Rect::new(r.x, y + 67., r.w, 24.),
        18.,
        GOLD,
    );
    rule(r, y + 95.);
    let card_h = (r.h * 0.24).min(150.);
    if let Some(next) = cards(g, Rect::new(r.x, y + 108., r.w, card_h)) {
        action = Some(next);
    }
    let note_y = y + 112. + card_h;
    let problem = g.guild.dispatch_problem(quest, &g.party, &g.contracts);
    text(
        if g.hq.journey.is_some() {
            "Accepted assignment. This party is on the road."
        } else {
            problem.as_deref().unwrap_or_else(|| readiness(g, quest))
        },
        Rect::new(r.x, note_y, r.w, (r.bottom() - 82. - note_y).max(30.)),
        16.,
        GOLD,
    );
    text(
        &format!(
            "Returns day {}  ·  {}",
            g.hq.journey
                .map_or(g.guild.day + q.days, |id| g.guild.expeditions[id].returns),
            if g.guild.day + q.days <= 30 {
                "Before review"
            } else {
                "After review"
            }
        ),
        Rect::new(r.x, r.bottom() - 78., r.w, 24.),
        16.,
        MUTED,
    );
    if let Some(id) = g.hq.journey {
        let e = &g.guild.expeditions[id];
        text(
            &format!("ON THE ROAD · Returns day {}", e.returns),
            Rect::new(r.x, r.bottom() - 49., r.w, 44.),
            19.,
            GOLD,
        );
    } else if primary(
        Rect::new(r.x, r.bottom() - 49., r.w, 48.),
        "DISPATCH PARTY",
        problem.is_none(),
    ) {
        action = Some(UiAction::Dispatch);
    }
    action
}

fn cards(g: &Game, r: Rect) -> Option<UiAction> {
    let mut action = None;
    let cw = (r.w - 20.) / 3.;
    let size = cw.min(r.h - 42.).max(44.);
    for (id, a) in g.guild.roster.iter().enumerate() {
        let x = r.x + id as f32 * (cw + 10.);
        let rect = Rect::new(x, r.y, size, size);
        portrait(g, key(id), rect);
        if r.h > 84. {
            panel(Rect::new(rect.x, rect.y, rect.w, 20.), PANEL);
            label(&a.class, Rect::new(rect.x, rect.y, rect.w, 20.), 13., INK);
        }
        let state = activity(&g.guild, id);
        let blocked = matches!(state, Activity::Away | Activity::Recovering);
        if blocked {
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::new(0., 0., 0., 0.58));
        }
        let selected = if let Some(journey) = g.hq.journey {
            g.guild.expeditions[journey].party.contains(&id)
        } else {
            g.party.contains(&id)
        };
        if selected {
            draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 3., GOLD);
            panel(
                Rect::new(rect.right() - 24., rect.bottom() - 24., 24., 24.),
                GOLD,
            );
            label(
                "+",
                Rect::new(rect.right() - 24., rect.bottom() - 24., 24., 24.),
                22.,
                BLACK,
            );
        }
        text(
            first(g, id),
            Rect::new(x, rect.bottom() + 3., cw, 22.),
            19.,
            if blocked { MUTED } else { INK },
        );
        if r.h > 84. {
            text(
                &format!(
                    "{} · {}",
                    a.rank(),
                    if state == Activity::Resting {
                        format!("Tired {}/6", a.fatigue)
                    } else {
                        state.label().to_owned()
                    }
                ),
                Rect::new(x, rect.bottom() + 26., cw, 24.),
                13.,
                MUTED,
            );
        }
        if activated(Rect::new(x, r.y, cw, r.h.max(44.))) && !blocked && g.hq.journey.is_none() {
            action = Some(UiAction::Party(id));
        }
    }
    action
}

fn readiness(g: &Game, id: usize) -> &'static str {
    let q = &g.contracts[id];
    let power = g.guild.prepared_strength(id, q, &g.party);
    if power >= q.difficulty + 2 {
        "Well prepared. Fatigue still matters."
    } else if power >= q.difficulty {
        "Close call likely. Consider support or rest."
    } else {
        "Outmatched. Add support or rest before dispatch."
    }
}

fn details(g: &Game, r: Rect, id: usize) -> Option<UiAction> {
    let q = &g.contracts[id];
    let mut action = None;
    let half = (r.w - 8.) / 2.;
    if g.hq.journey.is_none() {
        for (forward, name, x) in [
            (false, "< Previous job", r.x),
            (true, "Next job >", r.x + half + 8.),
        ] {
            if button(Rect::new(x, r.y, half, 44.), name, false) {
                action = Some(UiAction::Quest(g.guild.adjacent_contract(
                    id,
                    forward,
                    &g.contracts,
                )));
            }
        }
    }
    let y = r.y + 56.;
    let scouted = g.guild.services.scouted.contains(&id);
    let content = if r.h < 420. {
        format!(
            "{}\n{}\nScout: 20g; route-specific, consumed on next dispatch. Trial must be unaided.",
            q.client,
            g.guild.offer_notice(q)
        )
    } else {
        format!("{}\n{}\n\n{}\n\n{}\n{}\n\nScouting costs 20g, applies to this route and is consumed by its next dispatch, even on a later posting. The solo trial must be unaided.", q.title, q.client, q.brief, g.guild.offer_notice(q), if q.promotion { "Promotion work: solo assessment; no service credit." } else if q.bronze { "Bronze commission: review objective and earnings." } else if q.service { "Distinct service job: counts once toward six successes by day 30." } else { "Ordinary earnings: no distinct service credit." })
    };
    text(&content, Rect::new(r.x, y, r.w, r.h - 114.), 18., INK);
    if primary(
        Rect::new(r.x, r.bottom() - 48., r.w, 46.),
        if scouted {
            "ROUTE SCOUTED"
        } else if q.promotion {
            "UNAIDED TRIAL · NO SCOUTS"
        } else if g.guild.gold < 20 {
            "SCOUT ROUTE · NEED 20g"
        } else {
            "SCOUT ROUTE · 20g"
        },
        !scouted && !q.promotion && g.guild.gold >= 20 && g.hq.journey.is_none(),
    ) {
        action = Some(UiAction::Purchase(crate::services::Purchase::Scout(id)));
    }
    action
}

fn confirmation(g: &Game, r: Rect, id: usize) -> Option<UiAction> {
    panel(r, PANEL);
    let q = &g.contracts[id];
    let problem = g.guild.dispatch_problem(id, &g.party, &g.contracts);
    text(
        &format!(
            "{}\nReturns day {}\n{}\n{}",
            q.title,
            g.guild.day + q.days,
            g.guild.offer_notice(q),
            problem.as_deref().unwrap_or_else(|| readiness(g, id))
        ),
        Rect::new(r.x, r.y + 4., r.w, r.h - 55.),
        19.,
        INK,
    );
    if primary(
        Rect::new(r.x, r.bottom() - 48., r.w, 46.),
        "DISPATCH PARTY",
        problem.is_none(),
    ) {
        return Some(UiAction::Dispatch);
    }
    None
}
