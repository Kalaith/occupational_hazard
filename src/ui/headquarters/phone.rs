//! Compact navigation and decision pages, with room artwork secondary to reading.
use super::*;

pub fn rooms(r: Rect) -> Option<UiAction> {
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
            room.name(),
            false,
        ) {
            return Some(UiAction::Room(*room));
        }
    }
    None
}

pub fn overview(g: &Game, y: f32) -> Option<UiAction> {
    let w = screen_width();
    let row_h = ((screen_height() - y - 214.) / 3.).clamp(48., 65.);
    for id in 0..g.guild.roster.len() {
        let r = Rect::new(12., y + id as f32 * row_h, w - 24., (row_h - 5.).max(44.));
        panel(r, PANEL);
        let size = (r.h - 12.).min(46.);
        portrait(g, key(id), Rect::new(r.x + 6., r.y + 6., size, size));
        text(
            &format!("{} · {}", first(g, id), activity(&g.guild, id).label()),
            Rect::new(r.x + 64., r.y + 3., r.w - 72., 22.),
            20.,
            INK,
        );
        text(
            &format!(
                "{} · Fatigue {}/6",
                g.guild.roster[id].class, g.guild.roster[id].fatigue
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
        Rect::new(12., y + row_h * 3. + 3., w - 24., 48.),
        "Compare commissions >",
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
        if party { "Contract" } else { "Party >" },
        false,
    ) {
        return Some(UiAction::SheetPage(usize::from(!party)));
    }
    text(&q.title, Rect::new(r.x, r.y, r.w, 40.), 24., INK);
    if short && g.hq.page == 2 {
        let problem = g.guild.dispatch_problem(id, &g.party, &g.contracts);
        text(
            &preparation::summary(g, id),
            Rect::new(r.x, r.y + 46., r.w, 56.),
            19.,
            GOLD,
        );
        text(
            &preparation::advice(g, id),
            Rect::new(r.x, r.y + 105., r.w, 60.),
            18.,
            INK,
        );
        let action = preparation::scout(g, Rect::new(r.x, r.bottom() - 98., r.w, 44.), id);
        if primary(
            Rect::new(r.x, r.bottom() - 48., r.w, 48.),
            "DISPATCH PARTY",
            problem.is_none() && g.hq.journey.is_none(),
        ) {
            return Some(UiAction::Dispatch);
        }
        return action;
    }
    if !party {
        let image_h = if short { 0. } else { 140. };
        if !short {
            destination(g, id, Rect::new(r.x, r.y + 46., r.w, image_h));
        }
        let body_y = r.y + if short { 42. } else { image_h + 60. };
        text(
            &q.brief,
            Rect::new(r.x, body_y, r.w, r.bottom() - 154. - body_y),
            20.,
            INK,
        );
        if r.h > 620.
            && button(
                Rect::new(r.x, r.bottom() - 240., r.w, 44.),
                "Acceptance & service terms",
                false,
            )
        {
            return Some(UiAction::SheetPage(3));
        }
        text(
            &format!(
                "{}g · {} days · {}\nDanger: {}",
                q.gold,
                q.days,
                if q.promotion {
                    "Solo trial"
                } else if q.bronze {
                    "Bronze"
                } else {
                    "Iron"
                },
                q.danger
            ),
            Rect::new(r.x, r.bottom() - 150., r.w, 53.),
            18.,
            GOLD,
        );
        let expiry = if q.promotion {
            "Standing trial".into()
        } else {
            format!(
                "Accept by day {}",
                q.offer(g.guild.day).map_or(g.guild.day, |o| o.expires)
            )
        };
        let return_day =
            g.hq.journey
                .map_or(g.guild.day + q.days, |e| g.guild.expeditions[e].returns);
        text(
            &format!(
                "{}\nReturn day {} · {} review",
                expiry,
                return_day,
                if return_day <= 30 { "before" } else { "after" }
            ),
            Rect::new(r.x, r.bottom() - 95., r.w, 43.),
            18.,
            MUTED,
        );
        if primary(
            Rect::new(r.x, r.bottom() - 48., r.w, 48.),
            "CHOOSE PARTY >",
            true,
        ) {
            return Some(UiAction::SheetPage(1));
        }
        return None;
    }
    if short {
        let mut action = super::planning::cards(g, Rect::new(r.x, r.y + 42., r.w, 170.));
        if primary(
            Rect::new(r.x, r.bottom() - 44., r.w, 44.),
            "READINESS & DISPATCH >",
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
    let problem = g.guild.dispatch_problem(id, &g.party, &g.contracts);
    text(
        &preparation::advice(g, id),
        Rect::new(
            r.x,
            summary_y + 75.,
            r.w,
            (r.bottom() - 114. - summary_y - 75.).min(65.),
        ),
        18.,
        INK,
    );
    action = preparation::scout(g, Rect::new(r.x, r.bottom() - 104., r.w, 44.), id).or(action);
    if primary(
        Rect::new(r.x, r.bottom() - 52., r.w, 52.),
        "DISPATCH PARTY",
        problem.is_none() && g.hq.journey.is_none(),
    ) {
        action = Some(UiAction::Dispatch);
    }
    action
}

pub fn terms(g: &Game, r: Rect, id: usize) -> Option<UiAction> {
    text(
        &format!(
            "{}\n\n{}\n\n{}",
            g.contracts[id].title,
            g.contracts[id].client,
            g.guild.offer_notice(&g.contracts[id])
        ),
        Rect::new(r.x, r.y, r.w, r.h - 62.),
        21.,
        INK,
    );
    if button(
        Rect::new(r.x, r.bottom() - 48., r.w, 48.),
        "< Contract",
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
