use super::*;

pub fn window(r: Rect) {
    draw_rectangle(r.x + 8., r.y + 10., r.w, r.h, Color::new(0., 0., 0., 0.5));
    panel(r, PANEL);
    draw_rectangle(
        r.x + 8.,
        r.y + 8.,
        r.w - 16.,
        55.,
        Color::new(0.27, 0.19, 0.11, 0.5),
    );
    draw_rectangle_lines(
        r.x + 5.,
        r.y + 5.,
        r.w - 10.,
        r.h - 10.,
        1.,
        Color::new(0.34, 0.29, 0.21, 1.),
    );
    rule(Rect::new(r.x + 18., r.y, r.w - 36., r.h), r.y + 64.);
    if r.w > 500. {
        crest(vec2(r.right() - 44., r.y + 36.), 17.);
    }
}

pub fn hud(g: &Game) -> Option<UiAction> {
    let w = screen_width();
    let h = screen_height();
    let phone = w < 650.;
    let stack = w < 900.;
    let mut action = None;
    let resources = Rect::new(
        12.,
        12.,
        if phone { (w - 216.).min(160.) } else { 208. },
        44.,
    );
    panel(resources, PANEL);
    label(
        &g.guild.text.format(
            "ui.day_gold",
            &[
                ("day", g.guild.day.to_string()),
                ("gold", g.guild.gold.to_string()),
            ],
        ),
        resources,
        18.,
        GOLD,
    );
    let review = Rect::new(
        w - if phone { 196. } else { 274. },
        12.,
        if phone { 90. } else { 166. },
        44.,
    );
    let review_label = if g.guild.month.sandbox {
        g.guild.text.get("ui.sandbox").to_string()
    } else if phone {
        g.guild.text.get("ui.review").to_string()
    } else {
        g.guild.text.format(
            "ui.review_in",
            &[(
                "days",
                g.guild
                    .config
                    .review
                    .cutoff_day
                    .saturating_sub(g.guild.day)
                    .to_string(),
            )],
        )
    };
    if guided_button(g, review, &review_label, false) {
        action = Some(UiAction::Month);
    }
    if button(
        Rect::new(w - 96., 12., 84., 44.),
        g.guild.text.get("ui.menu"),
        false,
    ) {
        action = Some(UiAction::Settings);
    }
    let advance = Rect::new(
        w - if phone { 174. } else { 238. },
        h - 66.,
        if phone { 162. } else { 222. },
        50.,
    );
    if primary(advance, g.guild.text.get("ui.advance_day"), true) {
        action = Some(UiAction::NextDay);
    }
    if help::is_target(g, g.guild.text.get("ui.advance_day")) {
        draw_rectangle_lines(advance.x, advance.y, advance.w, advance.h, 3., GOLD);
    }
    if let Some((id, e)) = g
        .guild
        .expeditions
        .iter()
        .enumerate()
        .min_by_key(|(_, e)| e.returns)
    {
        let r = Rect::new(
            12.,
            h - if stack { 156. } else { 82. },
            if stack { w - 24. } else { (w - 580.).min(600.) },
            if stack { 76. } else { 66. },
        );
        panel(r, PANEL);
        let names = e
            .party
            .iter()
            .map(|&id| first(g, id))
            .collect::<Vec<_>>()
            .join(" + ");
        text(
            &g.guild.text.format(
                "ui.journey_card",
                &[
                    ("contract", g.contracts[e.contract].title.clone()),
                    ("client", g.contracts[e.contract].client.clone()),
                    ("people", names),
                    ("day", e.returns.to_string()),
                ],
            ),
            Rect::new(r.x + 12., r.y + 7., r.w - 24., r.h - 14.),
            16.,
            INK,
        );
        if activated(r) {
            action = Some(UiAction::Journey(id));
        }
    }
    let returns_x = if !stack && !g.guild.expeditions.is_empty() {
        24. + (w - 580.).min(600.)
    } else {
        12.
    };
    if g.guild.unread_reports() > 0
        && button(
            Rect::new(
                returns_x,
                h - 66.,
                (w - advance.w - returns_x - 36.).min(252.),
                50.,
            ),
            &g.guild.text.format(
                "ui.return_unread",
                &[("count", g.guild.unread_reports().to_string())],
            ),
            false,
        )
    {
        action = Some(UiAction::Tab(2));
    }
    action
}
