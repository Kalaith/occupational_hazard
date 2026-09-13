//! The headquarters stays full size. Focused work opens a floating guild window.
use super::*;
use crate::headquarters::{activity, Activity, Room, Sheet};
mod chrome;
mod commissions;
mod management;
mod phone;
mod planning;
mod preparation;
mod quest;
mod returns;
pub mod scene;

pub fn draw(g: &Game) -> Option<UiAction> {
    let w = screen_width();
    let h = screen_height();
    let phone = w < 650.;
    let short = h < 500.;
    let has_window = g.hq.sheet != Sheet::None;
    let overview = phone && g.hq.focus.is_none();
    let stage = if overview {
        Rect::new(0., 124., w, (h - 560.).clamp(64., w * 0.66))
    } else {
        Rect::new(0., 0., w, h)
    };
    let scene_action = scene::draw(g, stage, !has_window);
    let mut action = if has_window { None } else { scene_action };
    if has_window {
        // Opening work never compresses or rearranges the headquarters behind it.
        draw_rectangle(0., 0., w, h, Color::new(0.015, 0.025, 0.035, 0.48));
        let wide = matches!(g.hq.sheet, Sheet::Jobs | Sheet::Commissions) && w >= 900. && h >= 700.;
        let ww = if wide { 940.0_f32 } else { 580.0_f32 }.min(w - 24.);
        let wh = (h - if short || phone { 16. } else { 80. }).min(if phone { 820. } else { 620. });
        let r = Rect::new((w - ww) / 2., (h - wh) / 2., ww, wh);
        chrome::window(r);
        let planning = g.hq.sheet == Sheet::Jobs && g.hq.journey.is_none();
        if button(
            Rect::new(r.x + 20., r.y + 14., 104., 44.),
            if planning {
                g.guild.text.get("ui.back_offers")
            } else {
                g.guild.text.get("ui.back_guild")
            },
            false,
        ) {
            action = Some(if planning {
                UiAction::CommissionList
            } else {
                UiAction::Overview
            });
        }
        let title = match g.hq.sheet {
            Sheet::Rooms => g.guild.text.get("ui.headquarters_rooms"),
            Sheet::Commissions => g.guild.text.get("ui.commission_board"),
            Sheet::Jobs => g.guild.text.get("ui.guild_commission"),
            Sheet::Returns => g.guild.text.get("ui.expedition_journal"),
            Sheet::Career => g.guild.text.get("ui.guild_register"),
            Sheet::Facility(_) => g.guild.text.get("ui.headquarters_improvement"),
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
            Sheet::Rooms => phone::rooms(g, area),
            Sheet::Commissions => commissions::draw(g, area),
            Sheet::Jobs if wide => quest::draw(g, area),
            Sheet::Jobs => phone::planning(g, area),
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
        if button(
            Rect::new(12., 68., (w - 32.) / 2., 44.),
            g.guild.text.get("ui.overview"),
            false,
        ) {
            action = Some(UiAction::Overview);
        }
        if button(
            Rect::new(w / 2. + 4., 68., (w - 32.) / 2., 44.),
            g.guild.text.get("ui.rooms"),
            false,
        ) {
            action = Some(UiAction::Rooms);
        }
        if overview {
            action = phone::overview(g, stage.bottom() + 14.).or(action);
        }
    }
    if g.lesson().is_some() {
        let r = Rect::new(
            16.,
            if phone || short { h - 282. } else { 74. },
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
            if phone || short { h - 226. } else { 74. },
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
        .unwrap_or(g.guild.text.get("ui.guild_record"))
}

pub fn primary(r: Rect, title: &str, enabled: bool) -> bool {
    theme::button(r, title, false, true, enabled)
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
    draw_line(r.x, y, r.x + r.w, y, 1., Color::new(0.45, 0.34, 0.21, 1.));
    draw_poly(r.x + r.w / 2., y, 4, 3., 0., GOLD);
}
