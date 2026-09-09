use super::*;

pub fn window(r: Rect) {
    draw_rectangle(r.x + 8., r.y + 10., r.w, r.h, Color::new(0., 0., 0., 0.5));
    panel(r, Color::new(0.075, 0.095, 0.105, 1.));
    draw_rectangle_lines(
        r.x + 5.,
        r.y + 5.,
        r.w - 10.,
        r.h - 10.,
        1.,
        Color::new(0.34, 0.29, 0.21, 1.),
    );
    draw_line(r.x + 18., r.y + 64., r.right() - 18., r.y + 64., 1., GOLD);
    for (x, y) in [
        (r.x, r.y),
        (r.right(), r.y),
        (r.x, r.bottom()),
        (r.right(), r.bottom()),
    ] {
        draw_circle(x, y, 4., GOLD);
    }
}

pub fn hud(g: &Game) -> Option<UiAction> {
    let w = screen_width();
    let h = screen_height();
    let phone = w < 650.;
    let mut action = None;
    let resources = Rect::new(
        12.,
        12.,
        if phone { (w - 216.).min(160.) } else { 208. },
        44.,
    );
    panel(resources, PANEL);
    label(
        &format!("DAY {}     /     {}g", g.guild.day, g.guild.gold),
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
    if guided_button(
        g,
        review,
        &if g.guild.month.sandbox {
            "Sandbox".into()
        } else if phone {
            "Review".into()
        } else {
            format!("Review in {} days", 30u32.saturating_sub(g.guild.day))
        },
        false,
    ) {
        action = Some(UiAction::Month);
    }
    if button(Rect::new(w - 96., 12., 84., 44.), "Menu", false) {
        action = Some(UiAction::Settings);
    }
    let advance = Rect::new(
        w - if phone { 174. } else { 238. },
        h - 66.,
        if phone { 162. } else { 222. },
        50.,
    );
    if primary(advance, "ADVANCE DAY", true) {
        action = Some(UiAction::NextDay);
    }
    if help::is_target(g, "ADVANCE DAY") {
        draw_rectangle_lines(advance.x, advance.y, advance.w, advance.h, 3., GOLD);
    }
    if let Some((id, e)) = g
        .guild
        .expeditions
        .iter()
        .enumerate()
        .min_by_key(|(_, e)| e.returns)
    {
        let r = Rect::new(12., h - 78., (w - advance.w - 48.).min(390.), 62.);
        panel(r, PANEL);
        let name = first(g, e.party[0]);
        text(
            &format!("{} · ON THE ROAD\nReturns day {}", name, e.returns),
            Rect::new(r.x + 12., r.y + 9., r.w - 24., 44.),
            16.,
            INK,
        );
        if activated(r) {
            action = Some(UiAction::Journey(id));
        }
    } else if g.guild.unread_reports() > 0 {
        if button(
            Rect::new(12., h - 66., (w - advance.w - 48.).min(252.), 50.),
            &format!("Returns · {} unread", g.guild.unread_reports()),
            false,
        ) {
            action = Some(UiAction::Tab(2));
        }
    }
    action
}
