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
    if g.hq.page == 3 {
        return phone::terms(g, r, id);
    }
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
    let picture = Rect::new(left.x, left.y + 64., left.w, 155.);
    phone::destination(g, id, picture);
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
        let expiry = if q.promotion {
            "Standing trial".into()
        } else {
            format!(
                "Accept by day {}",
                q.offer(g.guild.day).map_or(g.guild.day, |o| o.expires)
            )
        };
        text(
            &expiry,
            Rect::new(left.x, left.bottom() - 162., left.w, 26.),
            18.,
            GOLD,
        );
        if button(
            Rect::new(left.x, left.bottom() - 46., left.w, 44.),
            "Acceptance & service terms",
            false,
        ) {
            action = Some(UiAction::SheetPage(3));
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
    if let Some(next) =
        preparation::members(g, Rect::new(right.x, right.y + 40., right.w, 255.), id)
    {
        action = Some(next);
    }
    let problem = g.guild.dispatch_problem(id, &g.party, &g.contracts);
    text(
        &preparation::summary(g, id),
        Rect::new(right.x, right.y + 300., right.w, 55.),
        18.,
        GOLD,
    );
    {
        let reason = preparation::advice(g, id);
        text(
            &reason,
            Rect::new(right.x, right.y + 354., right.w, 38.),
            17.,
            INK,
        );
    }
    action = preparation::scout(
        g,
        Rect::new(right.x, right.bottom() - 122., right.w, 44.),
        id,
    )
    .or(action);
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
