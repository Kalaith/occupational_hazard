//! A commission is an open journal, not a permanent sidebar.
use super::*;

pub fn draw(g: &Game, r: Rect) -> Option<UiAction> {
    if g.guild.open_contracts(&g.contracts).is_empty() && g.hq.journey.is_none() {
        text(g.guild.text.get("ui.no_postings"), r, 24., INK);
        return None;
    }
    let journey = g.hq.journey.and_then(|id| g.guild.expeditions.get(id));
    let id = journey.map_or(g.selected, |e| e.contract);
    if g.hq.page == 3 {
        return phone::terms(g, r, id);
    }
    let mut action = draw_contract(g, r, id, journey.is_some());
    action = draw_party(g, r, id, journey).or(action);
    action
}

fn draw_contract(g: &Game, r: Rect, id: usize, travelling: bool) -> Option<UiAction> {
    let q = &g.contracts[id];
    let left = Rect::new(r.x, r.y, r.w * 0.49, r.h);
    let mut action = None;
    draw_line(
        left.right() + 15.,
        r.y,
        left.right() + 15.,
        r.bottom(),
        1.,
        Color::new(0.32, 0.30, 0.25, 1.),
    );
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
        &g.guild.text.format(
            "ui.quest_stats",
            &[
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
                ("days", q.days.to_string()),
                ("gold", q.gold.to_string()),
            ],
        ),
        Rect::new(left.x, left.bottom() - 124., left.w, 28.),
        20.,
        GOLD,
    );
    text(
        &g.guild
            .text
            .format("ui.danger", &[("danger", q.danger.clone())]),
        Rect::new(left.x, left.bottom() - 87., left.w, 30.),
        18.,
        INK,
    );
    if !travelling {
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
        text(
            &expiry,
            Rect::new(left.x, left.bottom() - 162., left.w, 26.),
            18.,
            GOLD,
        );
        if button(
            Rect::new(left.x, left.bottom() - 46., left.w, 44.),
            g.guild.text.get("ui.acceptance_terms"),
            false,
        ) {
            action = Some(UiAction::SheetPage(3));
        }
    }
    action
}

fn draw_party(
    g: &Game,
    r: Rect,
    id: usize,
    journey: Option<&crate::simulation::Expedition>,
) -> Option<UiAction> {
    let q = &g.contracts[id];
    let left = Rect::new(r.x, r.y, r.w * 0.49, r.h);
    let right = Rect::new(left.right() + 30., r.y, r.right() - left.right() - 30., r.h);
    let mut action = None;
    text(
        if journey.is_some() {
            g.guild.text.get("ui.travelling_party")
        } else {
            g.guild.text.get("ui.choose_your_party")
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
        &g.guild.text.format(
            "ui.return_day",
            &[
                ("day", day.to_string()),
                (
                    "timing",
                    if day <= g.guild.config.review.cutoff_day {
                        g.guild.text.get("ui.before_review").to_string()
                    } else {
                        g.guild.text.get("ui.after_review").to_string()
                    },
                ),
            ],
        ),
        Rect::new(right.x, right.bottom() - 70., right.w, 24.),
        16.,
        GOLD,
    );
    if journey.is_some() {
        label(
            g.guild.text.get("ui.on_the_road_label"),
            Rect::new(right.x, right.bottom() - 43., right.w, 43.),
            22.,
            GOLD,
        );
    } else if primary(
        Rect::new(right.x, right.bottom() - 46., right.w, 46.),
        g.guild.text.get("ui.dispatch_party"),
        problem.is_none(),
    ) {
        action = Some(UiAction::Dispatch);
    }
    action
}
