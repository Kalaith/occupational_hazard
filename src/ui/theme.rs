//! Guild-specific leather, brass and stationery, drawn with toolkit surfaces.
use super::*;

const BRASS: Color = Color::new(0.48, 0.34, 0.18, 1.);
pub const PAPER_INK: Color = Color::new(0.23, 0.16, 0.10, 1.);

pub fn panel(r: Rect, color: Color) {
    draw_surface(r, &SurfaceStyle::new(color).with_border(1., BRASS));
    // Quiet tooling across the leather; deterministic and asset-free.
    for row in 0..(r.h / 8.) as usize {
        let y = r.y + 3. + row as f32 * 8.;
        draw_line(
            r.x + 2.,
            y,
            r.right() - 2.,
            y,
            1.,
            Color::new(0.8, 0.65, 0.4, 0.025),
        );
    }
    draw_line(
        r.x + 2.,
        r.y + 2.,
        r.right() - 2.,
        r.y + 2.,
        1.,
        Color::new(0.9, 0.73, 0.44, 0.2),
    );
    if r.h >= 120. && r.w >= 160. {
        draw_rectangle_lines(r.x + 6., r.y + 6., r.w - 12., r.h - 12., 1., BRASS);
        for (x, dx) in [(r.x + 6., 1.), (r.right() - 6., -1.)] {
            for (y, dy) in [(r.y + 6., 1.), (r.bottom() - 6., -1.)] {
                draw_triangle(
                    vec2(x, y),
                    vec2(x + dx * 17., y),
                    vec2(x, y + dy * 17.),
                    BRASS,
                );
                rivet(vec2(x + dx * 4., y + dy * 4.));
            }
        }
    }
}

fn rivet(p: Vec2) {
    draw_circle(p.x, p.y + 1., 2.5, Color::new(0.04, 0.03, 0.02, 1.));
    draw_circle(p.x, p.y, 1.7, GOLD);
    draw_line(p.x - 1., p.y, p.x + 1., p.y, 1., BRASS);
}

pub fn button(r: Rect, title: &str, selected: bool, primary: bool, enabled: bool) -> bool {
    let pointer = macroquad_toolkit::ui::Pointer::read(|p| p);
    let pressed = enabled && pointer.pressing(r);
    let hover = enabled && pointer.hovering_over(r);
    let color = if !enabled {
        Color::new(0.16, 0.15, 0.13, 1.)
    } else if primary {
        if hover {
            Color::new(0.53, 0.20, 0.13, 1.)
        } else {
            Color::new(0.38, 0.12, 0.085, 1.)
        }
    } else if selected {
        Color::new(0.25, 0.34, 0.27, 1.)
    } else if hover {
        Color::new(0.29, 0.24, 0.16, 1.)
    } else {
        Color::new(0.20, 0.16, 0.115, 1.)
    };
    draw_rectangle(r.x, r.y + 3., r.w, r.h, Color::new(0., 0., 0., 0.4));
    let face = Rect::new(r.x, r.y + if pressed { 2. } else { 0. }, r.w, r.h - 2.);
    panel(face, color);
    let edge = if enabled { GOLD } else { BRASS };
    draw_rectangle_lines(
        face.x + 3.,
        face.y + 3.,
        face.w - 6.,
        face.h - 6.,
        1.,
        Color::new(edge.r, edge.g, edge.b, 0.45),
    );
    if selected {
        draw_rectangle(face.x + 5., face.bottom() - 5., face.w - 10., 2., GOLD);
    }
    // Fittings stay outside the label's reserved area, even on narrow controls.
    if face.w >= 100. {
        for x in [face.x + 10., face.right() - 10.] {
            if primary {
                draw_poly(x, face.y + face.h / 2., 4, 4., 0., edge);
            } else {
                rivet(vec2(x, face.y + face.h / 2.));
            }
        }
    }
    let padding = if face.w >= 100. { 32. } else { 14. };
    let base = if primary { 20. } else { 19. };
    let measured = measure_text_size(title, TextStyle::new(base, INK)).width;
    let size = base * ((face.w - padding) / measured.max(1.)).min(1.);
    let text_rect = Rect::new(face.x + padding / 2., face.y, face.w - padding, face.h);
    label(
        title,
        Rect::new(text_rect.x, text_rect.y + 1., text_rect.w, text_rect.h),
        size,
        Color::new(0., 0., 0., 0.65),
    );
    label(
        title,
        text_rect,
        size,
        if !enabled {
            MUTED
        } else if selected {
            GOLD
        } else {
            INK
        },
    );
    enabled && pointer.released_on(r)
}

pub fn parchment(r: Rect) {
    draw_rectangle(r.x + 2., r.y + 3., r.w, r.h, Color::new(0., 0., 0., 0.3));
    draw_surface(
        r,
        &SurfaceStyle::new(Color::new(0.77, 0.68, 0.50, 1.)).with_border(1., BRASS),
    );
    draw_rectangle_lines(
        r.x + 4.,
        r.y + 4.,
        r.w - 8.,
        r.h - 8.,
        1.,
        Color::new(0.41, 0.29, 0.15, 0.35),
    );
    for i in 0..(r.h / 5.) as usize {
        let y = r.y + 2. + i as f32 * 5.;
        draw_line(
            r.x + 5.,
            y,
            r.right() - 5.,
            y,
            1.,
            Color::new(0.35, 0.22, 0.10, 0.035),
        );
    }
}

pub fn progress(r: Rect, fraction: f32) {
    draw_rectangle(r.x, r.y, r.w, r.h, Color::new(0.36, 0.28, 0.17, 0.3));
    draw_rectangle(
        r.x,
        r.y,
        r.w * fraction.clamp(0., 1.),
        r.h,
        Color::new(0.24, 0.34, 0.25, 1.),
    );
    draw_rectangle_lines(r.x, r.y, r.w, r.h, 1., Color::new(0.35, 0.25, 0.14, 0.6));
}
