//! Compact navigation and decision pages, with room artwork secondary to reading.
use super::*;

pub fn rooms(g: &Game, r: Rect) -> Option<UiAction> {
    let columns = if r.h < 360. { 2 } else { 1 };
    let row = (r.h / (6 / columns) as f32).min(72.);
    let width = (r.w - 8. * (columns - 1) as f32) / columns as f32;
    for (i, room) in Room::ALL.iter().enumerate() {
        if button(
            Rect::new(
                r.x + (i % columns) as f32 * (width + 8.),
                r.y + (i / columns) as f32 * row,
                width,
                row - 6.,
            ),
            room.name(&g.guild.text),
            false,
        ) {
            return Some(UiAction::Room(*room));
        }
    }
    None
}

pub fn overview(g: &Game, y: f32) -> Option<UiAction> {
    let w = screen_width();
    let summary = Rect::new(12., y, w - 24., 48.);
    panel(summary, PANEL);
    let ready = g
        .guild
        .roster
        .iter()
        .enumerate()
        .filter(|(id, _)| matches!(activity(&g.guild, *id), Activity::Ready))
        .count();
    let away = g
        .guild
        .expeditions
        .iter()
        .map(|e| e.party.len())
        .sum::<usize>();
    let recovering = g
        .guild
        .roster
        .iter()
        .filter(|a| a.injury > 0 || a.fatigue > 0)
        .count();
    text(
        &g.guild.text.format(
            if g.hq.staff_open {
                "ui.staff_open"
            } else {
                "ui.staff_summary"
            },
            &[
                ("count", g.guild.roster.len().to_string()),
                ("ready", ready.to_string()),
                ("away", away.to_string()),
                ("recovering", recovering.to_string()),
            ],
        ),
        Rect::new(summary.x + 12., summary.y + 4., summary.w - 24., 40.),
        17.,
        GOLD,
    );
    if activated(summary) {
        return Some(UiAction::ToggleStaff);
    }
    if !g.hq.staff_open {
        return if button(
            Rect::new(12., y + 56., w - 24., 48.),
            g.guild.text.get("ui.compare_phone"),
            false,
        ) {
            Some(UiAction::CommissionList)
        } else {
            None
        };
    }
    let cards_y = y + 56.;
    let available_bottom = if g.lesson().is_some() {
        screen_height() - 146.
    } else {
        screen_height() - 214.
    };
    let row_h = ((available_bottom - cards_y) / 3.).clamp(44., 62.);
    for id in 0..g.guild.roster.len() {
        let r = Rect::new(
            12.,
            cards_y + id as f32 * row_h,
            w - 24.,
            (row_h - 5.).max(44.),
        );
        panel(r, PANEL);
        let size = (r.h - 12.).min(46.);
        portrait(g, key(id), Rect::new(r.x + 6., r.y + 6., size, size));
        text(
            &g.guild.text.format(
                "ui.member_status",
                &[
                    ("name", first(g, id).to_string()),
                    (
                        "activity",
                        activity(&g.guild, id).label(&g.guild.text).to_string(),
                    ),
                ],
            ),
            Rect::new(r.x + 64., r.y + 3., r.w - 72., 22.),
            20.,
            INK,
        );
        text(
            &g.guild.text.format(
                "ui.member_fatigue",
                &[
                    ("class", g.guild.roster[id].class.clone()),
                    ("fatigue", g.guild.roster[id].fatigue.to_string()),
                    ("max_fatigue", g.guild.config.caps.max_fatigue.to_string()),
                ],
            ),
            Rect::new(r.x + 64., r.y + 25., r.w - 72., 20.),
            17.,
            MUTED,
        );
        if activated(r) {
            return Some(UiAction::Dossier(id));
        }
    }
    if button(
        Rect::new(12., cards_y + row_h * 3. + 3., w - 24., 48.),
        g.guild.text.get("ui.compare_phone"),
        false,
    ) {
        return Some(UiAction::CommissionList);
    }
    None
}

pub fn planning(g: &Game, r: Rect) -> Option<UiAction> {
    let id =
        g.hq.journey
            .map_or(g.selected, |e| g.guild.expeditions[e].contract);
    let q = &g.contracts[id];
    if g.hq.page == 3 {
        return terms(g, r, id);
    }
    let party = g.hq.page == 1;
    let short = r.h < 420.;
    if button(
        Rect::new(r.right() - 112., r.y - 54., 112., 44.),
        if party {
            g.guild.text.get("ui.contract")
        } else {
            g.guild.text.get("ui.party")
        },
        false,
    ) {
        return Some(UiAction::SheetPage(usize::from(!party)));
    }
    text(&q.title, Rect::new(r.x, r.y, r.w, 40.), 24., INK);
    if short && g.hq.page == 2 {
        return short_readiness(g, r, id);
    }
    if !party {
        return contract_page(g, r, id, short);
    }
    party_page(g, r, id, short)
}

fn short_readiness(g: &Game, r: Rect, id: usize) -> Option<UiAction> {
    let problem = g.guild.dispatch_problem(id, &g.party, &g.contracts);
    text(
        &preparation::summary(g, id),
        Rect::new(r.x, r.y + 46., r.w, 42.),
        19.,
        GOLD,
    );
    if button(
        Rect::new(r.x, r.y + 90., r.w, 34.),
        if g.hq.readiness_details {
            g.guild.text.get("ui.hide_readiness_details")
        } else {
            g.guild.text.get("ui.readiness_details")
        },
        false,
    ) {
        return Some(UiAction::ToggleReadinessDetails);
    }
    let q = &g.contracts[id];
    let timing = if g.guild.day + q.days <= g.guild.config.review.cutoff_day {
        g.guild.text.get("ui.before_review").to_string()
    } else {
        g.guild.text.get("ui.after_review").to_string()
    };
    text(
        &g.guild.text.format(
            "ui.decision_service",
            &[
                ("status", preparation::service_status(g, id)),
                ("day", (g.guild.day + q.days).to_string()),
                ("timing", timing),
            ],
        ),
        Rect::new(r.x, r.y + 126., r.w, 26.),
        15.,
        MUTED,
    );
    text(
        &preparation::advice(g, id),
        Rect::new(r.x, r.y + 153., r.w, 22.),
        15.,
        INK,
    );
    let action = preparation::scout(g, Rect::new(r.x, r.bottom() - 98., r.w, 44.), id);
    if primary(
        Rect::new(r.x, r.bottom() - 48., r.w, 48.),
        g.guild.text.get("ui.dispatch_party"),
        problem.is_none() && g.hq.journey.is_none(),
    ) {
        return Some(UiAction::Dispatch);
    }
    action
}

fn contract_page(g: &Game, r: Rect, id: usize, short: bool) -> Option<UiAction> {
    let q = &g.contracts[id];
    let image_h = if short { 0. } else { 140. };
    if !short {
        destination(g, id, Rect::new(r.x, r.y + 46., r.w, image_h));
    }
    let compact = short && r.h < 420.;
    let footer_top = r.bottom() - if compact { 166. } else { 226. };
    let body_y = r.y + if short { 42. } else { image_h + 60. };
    text(
        &q.brief,
        Rect::new(r.x, body_y, r.w, (footer_top - body_y - 8.).max(28.)),
        20.,
        INK,
    );
    if button(
        Rect::new(r.x, footer_top, r.w, if compact { 30. } else { 36. }),
        g.guild.text.get("ui.acceptance_terms"),
        false,
    ) {
        return Some(UiAction::SheetPage(3));
    }
    text(
        &g.guild.text.format(
            "ui.phone_contract_stats",
            &[
                ("gold", q.gold.to_string()),
                ("days", q.days.to_string()),
                (
                    "type",
                    if q.promotion {
                        g.guild.text.get("ui.solo_trial").to_string()
                    } else if q.bronze {
                        g.guild.text.get("ui.bronze").to_string()
                    } else {
                        g.guild.text.get("ui.iron").to_string()
                    },
                ),
                ("danger", q.danger.clone()),
            ],
        ),
        Rect::new(
            r.x,
            r.bottom() - if compact { 132. } else { 182. },
            r.w,
            if compact { 24. } else { 38. },
        ),
        if compact { 14. } else { 16. },
        GOLD,
    );
    text(
        &g.guild.text.format(
            "ui.decision_service",
            &[
                ("status", preparation::service_status(g, id)),
                ("day", (g.guild.day + q.days).to_string()),
                (
                    "timing",
                    if g.guild.day + q.days <= g.guild.config.review.cutoff_day {
                        g.guild.text.get("ui.before_review").to_string()
                    } else {
                        g.guild.text.get("ui.after_review").to_string()
                    },
                ),
            ],
        ),
        Rect::new(
            r.x,
            r.bottom() - if compact { 102. } else { 142. },
            r.w,
            if compact { 24. } else { 34. },
        ),
        if compact { 14. } else { 16. },
        MUTED,
    );
    let expiry = if q.promotion {
        g.guild.text.get("ui.standing_trial").into()
    } else {
        g.guild.text.format(
            "ui.accept_by_day",
            &[(
                "day",
                q.offer(g.guild.day)
                    .map_or(g.guild.day, |o| o.expires)
                    .to_string(),
            )],
        )
    };
    let return_day =
        g.hq.journey
            .map_or(g.guild.day + q.days, |e| g.guild.expeditions[e].returns);
    text(
        &g.guild.text.format(
            "ui.return_timing",
            &[
                ("expiry", expiry),
                ("return_day", return_day.to_string()),
                (
                    "timing",
                    if return_day <= g.guild.config.review.cutoff_day {
                        g.guild.text.get("ui.before_review").to_string()
                    } else {
                        g.guild.text.get("ui.after_review").to_string()
                    },
                ),
            ],
        ),
        Rect::new(
            r.x,
            r.bottom() - if compact { 72. } else { 104. },
            r.w,
            if compact { 24. } else { 38. },
        ),
        if compact { 14. } else { 16. },
        MUTED,
    );
    if primary(
        Rect::new(
            r.x,
            r.bottom() - if compact { 44. } else { 48. },
            r.w,
            if compact { 40. } else { 48. },
        ),
        g.guild.text.get("ui.choose_party"),
        true,
    ) {
        return Some(UiAction::SheetPage(1));
    }
    None
}

fn party_page(g: &Game, r: Rect, id: usize, short: bool) -> Option<UiAction> {
    let q = &g.contracts[id];
    if short {
        let mut action = super::planning::cards(g, Rect::new(r.x, r.y + 42., r.w, 170.));
        if primary(
            Rect::new(r.x, r.bottom() - 44., r.w, 44.),
            g.guild.text.get("ui.readiness_dispatch"),
            true,
        ) {
            action = Some(UiAction::SheetPage(2));
        }
        return action;
    }
    let member_h = (r.h - 300.).clamp(180., 300.);
    let summary_y = r.y + 58. + member_h;
    let mut action = preparation::members(g, Rect::new(r.x, r.y + 46., r.w, member_h), id);
    text(
        &preparation::summary(g, id),
        Rect::new(r.x, summary_y, r.w, 66.),
        19.,
        GOLD,
    );
    if button(
        Rect::new(r.x, summary_y + 70., r.w, 34.),
        if g.hq.readiness_details {
            g.guild.text.get("ui.hide_readiness_details")
        } else {
            g.guild.text.get("ui.readiness_details")
        },
        false,
    ) {
        return Some(UiAction::ToggleReadinessDetails);
    }
    let problem = g.guild.dispatch_problem(id, &g.party, &g.contracts);
    text(
        &preparation::advice(g, id),
        Rect::new(
            r.x,
            summary_y + 110.,
            r.w,
            (r.bottom() - 150. - summary_y - 110.).min(65.),
        ),
        18.,
        INK,
    );
    text(
        &g.guild.text.format(
            "ui.decision_service",
            &[
                ("status", preparation::service_status(g, id)),
                ("day", (g.guild.day + q.days).to_string()),
                (
                    "timing",
                    if g.guild.day + q.days <= g.guild.config.review.cutoff_day {
                        g.guild.text.get("ui.before_review").to_string()
                    } else {
                        g.guild.text.get("ui.after_review").to_string()
                    },
                ),
            ],
        ),
        Rect::new(r.x, r.bottom() - 184., r.w, 28.),
        15.,
        GOLD,
    );
    action = preparation::scout(g, Rect::new(r.x, r.bottom() - 104., r.w, 44.), id).or(action);
    if primary(
        Rect::new(r.x, r.bottom() - 52., r.w, 52.),
        g.guild.text.get("ui.dispatch_party"),
        problem.is_none() && g.hq.journey.is_none(),
    ) {
        action = Some(UiAction::Dispatch);
    }
    action
}

pub fn terms(g: &Game, r: Rect, id: usize) -> Option<UiAction> {
    text(
        &g.guild.text.format(
            "ui.offer_terms",
            &[
                ("title", g.contracts[id].title.clone()),
                ("client", g.contracts[id].client.clone()),
                (
                    "notice",
                    g.guild.offer_notice(&g.contracts[id], &g.guild.text),
                ),
            ],
        ),
        Rect::new(r.x, r.y, r.w, r.h - 62.),
        21.,
        INK,
    );
    if button(
        Rect::new(r.x, r.bottom() - 48., r.w, 48.),
        g.guild.text.get("ui.back_contract"),
        false,
    ) {
        Some(UiAction::SheetPage(0))
    } else {
        None
    }
}

pub fn destination(g: &Game, id: usize, r: Rect) {
    let cells = [
        "cellar",
        "medicine",
        "beekeeper",
        "well",
        "lantern-trial",
        "north-bridge",
        "shutters",
        "quarry",
        "boundary",
        "shepherd",
        "fever",
        "bandages",
    ];
    let cell = cells
        .iter()
        .position(|key| *key == g.contracts[id].id)
        .expect("Authored destination art");
    draw_texture_ex(
        g.assets.get_texture("destinations").expect("destination"),
        r.x,
        r.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(r.size()),
            source: Some(Rect::new(
                (cell % 3) as f32 * 512. + 3.,
                (cell / 3) as f32 * 256. + 3.,
                506.,
                250.,
            )),
            ..Default::default()
        },
    );
}
