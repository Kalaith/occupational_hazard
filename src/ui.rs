//! Responsive UI drawn directly in window coordinates, without letterboxing.

use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;

mod title;
pub use title::draw_title;

pub const BACKGROUND: Color = Color::new(0.075, 0.085, 0.09, 1.0);
const GOLD: Color = Color::new(0.78, 0.64, 0.38, 1.0);

pub enum UiAction {
    Start,
    Settings,
    CloseSettings,
    Title,
}

pub fn draw_game(settings_open: bool) -> Option<UiAction> {
    let cog = Rect::new(screen_width() - 68.0, 12.0, 56.0, 56.0);
    if settings_open {
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::new(0.0, 0.0, 0.0, 0.5),
        );
        let width = (screen_width() - 32.0).clamp(180.0, 360.0);
        let rect = Rect::new(
            (screen_width() - width) / 2.0,
            (screen_height() - 260.0) / 2.0,
            width,
            260.0,
        );
        draw_surface(rect, &SurfaceStyle::new(BACKGROUND).with_border(1.0, GOLD));
        label(
            "Settings",
            Rect::new(rect.x + 16.0, rect.y + 18.0, width - 32.0, 44.0),
            28.0,
            GOLD,
        );
        if button(
            Rect::new(rect.x + 20.0, rect.y + 86.0, width - 40.0, 52.0),
            "Return to Title",
        ) {
            return Some(UiAction::Title);
        }
        if button(
            Rect::new(rect.x + 20.0, rect.y + 160.0, width - 40.0, 52.0),
            "Close",
        ) {
            return Some(UiAction::CloseSettings);
        }
        return None;
    }
    draw_cog(cog);
    if activated(cog) {
        Some(UiAction::Settings)
    } else {
        None
    }
}

fn draw_cog(rect: Rect) {
    let center = vec2(rect.x + rect.w / 2.0, rect.y + rect.h / 2.0);
    let color = if rect.contains(Vec2::from(mouse_position())) {
        WHITE
    } else {
        GOLD
    };
    // Eight teeth joined to an annular hub; no texture or font glyph required.
    for tooth in 0..8 {
        let angle = tooth as f32 * std::f32::consts::TAU / 8.0;
        let direction = vec2(angle.cos(), angle.sin());
        let tangent = vec2(-direction.y, direction.x);
        let inner = center + direction * 13.0;
        let outer = center + direction * 21.0;
        let a = inner - tangent * 4.0;
        let b = inner + tangent * 4.0;
        let c = outer + tangent * 4.0;
        let d = outer - tangent * 4.0;
        draw_triangle(a, b, c, color);
        draw_triangle(a, c, d, color);
    }
    draw_circle(center.x, center.y, 16.0, color);
    draw_circle(center.x, center.y, 7.0, BACKGROUND);
}

fn activated(rect: Rect) -> bool {
    rect.contains(Vec2::from(mouse_position())) && is_mouse_button_released(MouseButton::Left)
}

fn label(text: &str, rect: Rect, size: f32, color: Color) {
    draw_text_centered_in_box(text, rect.x, rect.y, rect.w, rect.h, size, color);
}

fn button(rect: Rect, text: &str) -> bool {
    let hover = rect.contains(Vec2::from(mouse_position()));
    draw_surface(
        rect,
        &SurfaceStyle::new(if hover {
            Color::new(0.19, 0.21, 0.22, 1.0)
        } else {
            Color::new(0.12, 0.14, 0.15, 1.0)
        })
        .with_border(1.0, GOLD),
    );
    label(text, rect, 20.0, WHITE);
    activated(rect)
}
