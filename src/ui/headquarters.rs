//! The building is the primary navigation surface; sheets are focused tasks.
use super::*;
use crate::headquarters::{activity, Activity, Room, Sheet};
mod management;
mod planning;
mod returns;
mod scene;

pub fn draw(g: &Game) -> Option<UiAction> {
    let w = screen_width();
    let h = screen_height();
    let phone = w < 650.;
    let short = h < 500.;
    let top = if phone { 62. } else { 48. };
    let bottom = if phone {
        116.
    } else if short {
        66.
    } else {
        94.
    };
    let has_sheet = g.hq.sheet != Sheet::None;
    let sheet_w = if phone {
        w
    } else {
        (w * 0.34).max(380.).min(w * 0.52)
    };
    let stage = Rect::new(
        0.,
        top,
        if has_sheet && !phone { w - sheet_w } else { w },
        h - top - bottom,
    );
    let mut action = scene::draw(g, stage, !has_sheet || !phone);
    panel(Rect::new(0., 0., w, top), PANEL);
    if !phone {
        crest(vec2(26., 23.), 17.);
        text(
            "O C C U P A T I O N A L   H A Z A R D",
            Rect::new(56., 8., w * 0.43, 32.),
            19.,
            INK,
        );
    }
    let status_x = if phone { 12. } else { w * 0.49 };
    text(
        &format!("DAY {} / {}g", g.guild.day, g.guild.gold),
        Rect::new(status_x, 8., if phone { 205. } else { 180. }, 32.),
        if phone { 16. } else { 20. },
        GOLD,
    );
    if guided_button(
        g,
        Rect::new(
            w - if phone { 182. } else { 282. },
            4.,
            if phone { 86. } else { 188. },
            44.,
        ),
        &if g.guild.month.sandbox {
            "Sandbox".into()
        } else if phone {
            "Review".into()
        } else {
            format!("Review: {} days", 30u32.saturating_sub(g.guild.day))
        },
        false,
    ) {
        action = Some(UiAction::Month);
    }
    if button(Rect::new(w - 88., 4., 80., 44.), "Menu", false) {
        action = Some(UiAction::Settings);
    }
    if (phone && !has_sheet) || short {
        for (i, room) in Room::ALL.iter().enumerate() {
            let bw = (stage.w - 16.) / 3.;
            if button(
                Rect::new(
                    8. + (i % 3) as f32 * bw,
                    top + (i / 3) as f32 * 48.,
                    bw - 4.,
                    44.,
                ),
                room.name(),
                g.hq.focus == Some(*room),
            ) {
                action = Some(UiAction::Room(*room));
            }
        }
    }
    if has_sheet {
        let r = Rect::new(
            if phone { 0. } else { w - sheet_w },
            top,
            sheet_w,
            h - top - bottom,
        );
        panel(r, PANEL);
        if button(
            Rect::new(r.x + 16., r.y + 12., 100., 44.),
            "< Overview",
            false,
        ) {
            action = Some(UiAction::Overview);
        }
        let area = Rect::new(r.x + 18., r.y + 66., r.w - 36., r.h - 78.);
        let inner = match g.hq.sheet {
            Sheet::Jobs => planning::draw(g, area),
            Sheet::Returns => returns::draw(g, area),
            Sheet::Career => management::career(g, area),
            Sheet::Facility(room) => management::facility(g, area, room),
            Sheet::None => None,
        };
        if inner.is_some() {
            action = inner;
        }
    } else if let Some(lesson) = g.lesson() {
        let rw = (w - 32.).min(440.);
        let r = Rect::new(
            16.,
            if phone || short {
                top + 110.
            } else {
                top + 12.
            },
            rw.min(stage.w - 32.),
            116.,
        );
        if macroquad_toolkit::ui::Pointer::read(|p| p).released_on(r) {
            action = None;
        }
        if let Some(next) = help::draw_guidance(g, r) {
            action = Some(next);
        }
        let _ = lesson;
    } else if !g.notice.is_empty() {
        let r = Rect::new(16., top + 10., (stage.w - 32.).min(520.), 58.);
        panel(r, PANEL);
        if macroquad_toolkit::ui::Pointer::read(|p| p).released_on(r) {
            action = None;
        }
        text(
            &g.notice,
            Rect::new(r.x + 10., r.y + 8., r.w - 20., 45.),
            16.,
            INK,
        );
    }
    if let Some(next) = footer(g, Rect::new(0., h - bottom, w, bottom), phone) {
        action = Some(next);
    }
    action
}

fn footer(g: &Game, r: Rect, phone: bool) -> Option<UiAction> {
    panel(r, PANEL);
    let mut action = None;
    let travel_w = if phone { r.w - 16. } else { r.w * 0.29 };
    if let Some((id, e)) = g
        .guild
        .expeditions
        .iter()
        .enumerate()
        .min_by_key(|(_, e)| e.returns)
    {
        let a = e.party[0];
        let size = if phone || r.h < 80. { 38. } else { 58. };
        portrait(g, key(a), Rect::new(18., r.y + 13., size, size));
        text(
            &if r.h < 80. {
                format!("{} · Day {}", first(g, a), e.returns)
            } else {
                format!(
                    "{} · {}\nReturns day {}",
                    first(g, a),
                    g.contracts[e.contract].title,
                    e.returns
                )
            },
            Rect::new(
                size + 30.,
                r.y + 16.,
                travel_w - size - 42.,
                if r.h < 80. { 32. } else { 54. },
            ),
            if phone { 15. } else { 17. },
            INK,
        );
        if activated(Rect::new(0., r.y, travel_w, if phone { 54. } else { r.h })) {
            action = Some(UiAction::Journey(id));
        }
    } else {
        text(
            if r.h < 80. {
                "THE GUILD IS HOME"
            } else {
                "THE GUILD IS HOME\nTap Jobs to plan the next assignment."
            },
            Rect::new(
                20.,
                r.y + 16.,
                travel_w - 30.,
                if r.h < 80. { 32. } else { 54. },
            ),
            16.,
            MUTED,
        );
    }
    let start = if phone { 8. } else { travel_w + 4. };
    let y = r.y + if phone { 62. } else { (r.h - 48.) / 2. };
    let available = r.w - start - 12.;
    let bw = available * 0.205;
    for (i, (name, tab)) in [("Jobs", 0), ("Staff", 1), ("Returns", 2)]
        .iter()
        .enumerate()
    {
        let title = if *tab == 2 && g.guild.unread_reports() > 0 {
            format!("Returns ({})", g.guild.unread_reports())
        } else {
            name.to_string()
        };
        if guided_button(
            g,
            Rect::new(start + i as f32 * (bw + 5.), y, bw, 48.),
            &title,
            false,
        ) {
            action = Some(UiAction::Tab(*tab));
        }
    }
    let x = start + 3. * (bw + 5.);
    let advance = Rect::new(x, y, r.w - x - 12., 48.);
    if primary(advance, "ADVANCE DAY", true) {
        action = Some(UiAction::NextDay);
    }
    if help::is_target(g, "ADVANCE DAY") {
        draw_rectangle_lines(advance.x, advance.y, advance.w, advance.h, 3., GOLD);
    }
    action
}

pub fn text(value: &str, r: Rect, size: f32, color: Color) {
    paragraph(value, r, size, color);
}
pub fn key(id: usize) -> &'static str {
    ["mira", "tomas", "pip", "elowen"][id.min(3)]
}
pub fn first(g: &Game, id: usize) -> &str {
    g.guild.roster[id]
        .name
        .split_whitespace()
        .next()
        .unwrap_or("Member")
}

pub fn primary(r: Rect, title: &str, enabled: bool) -> bool {
    panel(
        r,
        if enabled {
            Color::new(0.58, 0.36, 0.14, 1.)
        } else {
            Color::new(0.13, 0.15, 0.16, 1.)
        },
    );
    label(
        title,
        r,
        if r.w < 150. { 15. } else { 20. },
        if enabled { WHITE } else { MUTED },
    );
    enabled && activated(r)
}

pub fn crest(p: Vec2, size: f32) {
    draw_line(p.x, p.y - size, p.x, p.y + size, 2., GOLD);
    for side in [-1., 1.] {
        for y in [-0.4, 0.1, 0.6] {
            draw_line(
                p.x,
                p.y + y * size,
                p.x + side * size * 0.5,
                p.y + (y - 0.35) * size,
                2.,
                GOLD,
            );
        }
    }
}

pub fn rule(r: Rect, y: f32) {
    draw_line(r.x, y, r.x + r.w, y, 1., Color::new(0.30, 0.31, 0.30, 1.));
}
