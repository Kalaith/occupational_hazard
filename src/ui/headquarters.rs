//! The headquarters stays full size. Focused work opens a floating guild window.
use super::*;
use crate::headquarters::{activity, Activity, Room, Sheet};
mod chrome;
mod commissions;
mod management;
mod planning;
mod quest;
mod returns;
mod scene;

pub fn draw(g: &Game) -> Option<UiAction> {
    let w = screen_width();
    let h = screen_height();
    let phone = w < 650.;
    let short = h < 500.;
    let has_window = g.hq.sheet != Sheet::None;
    let stage = Rect::new(0., 0., w, h);
    let scene_action = scene::draw(g, stage, !has_window);
    let mut action = if has_window { None } else { scene_action };
    if has_window {
        // Opening work never compresses or rearranges the headquarters behind it.
        draw_rectangle(0., 0., w, h, Color::new(0.015, 0.025, 0.035, 0.48));
        let wide = matches!(g.hq.sheet, Sheet::Jobs | Sheet::Commissions) && w >= 900. && h >= 700.;
        let ww = if wide { 940.0_f32 } else { 580.0_f32 }.min(w - 24.);
        let wh = (h - if short { 16. } else { 80. }).min(620.);
        let r = Rect::new((w - ww) / 2., (h - wh) / 2., ww, wh);
        chrome::window(r);
        let planning = g.hq.sheet == Sheet::Jobs && g.hq.journey.is_none();
        if button(
            Rect::new(r.x + 20., r.y + 14., 104., 44.),
            if planning { "< Offers" } else { "< Guild" },
            false,
        ) {
            action = Some(if planning {
                UiAction::CommissionList
            } else {
                UiAction::Overview
            });
        }
        let title = match g.hq.sheet {
            Sheet::Commissions => "COMMISSION BOARD",
            Sheet::Jobs => "GUILD COMMISSION",
            Sheet::Returns => "EXPEDITION JOURNAL",
            Sheet::Career => "GUILD REGISTER",
            Sheet::Facility(_) => "HEADQUARTERS IMPROVEMENT",
            Sheet::None => "",
        };
        if ww > 500. {
            label(
                title,
                Rect::new(r.x + 140., r.y + 18., ww - 280., 36.),
                15.,
                GOLD,
            );
        }
        let area = Rect::new(r.x + 24., r.y + 74., ww - 48., wh - 98.);
        let inner = match g.hq.sheet {
            Sheet::Commissions => commissions::draw(g, area),
            Sheet::Jobs if wide => quest::draw(g, area),
            Sheet::Jobs => planning::draw(g, area),
            Sheet::Returns => returns::draw(g, area),
            Sheet::Career => management::career(g, area),
            Sheet::Facility(room) => management::facility(g, area, room),
            Sheet::None => None,
        };
        return inner.or(action);
    }
    if let Some(next) = chrome::hud(g) {
        action = Some(next);
    }
    if phone || short {
        for (i, room) in Room::ALL.iter().enumerate() {
            let bw = (w - 24.) / 3.;
            if button(
                Rect::new(
                    12. + (i % 3) as f32 * bw,
                    68. + (i / 3) as f32 * 48.,
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
    if g.lesson().is_some() {
        let r = Rect::new(
            16.,
            if phone || short { 174. } else { 74. },
            (w - 32.).min(440.),
            116.,
        );
        if activated(r) {
            action = None;
        }
        if let Some(next) = help::draw_guidance(g, r) {
            action = Some(next);
        }
    } else if !g.notice.is_empty() {
        let r = Rect::new(
            16.,
            if phone || short { 174. } else { 74. },
            (w - 32.).min(440.),
            60.,
        );
        panel(r, PANEL);
        if activated(r) {
            action = None;
        }
        text(
            &g.notice,
            Rect::new(r.x + 12., r.y + 9., r.w - 24., 42.),
            16.,
            INK,
        );
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
