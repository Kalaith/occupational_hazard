use super::*;

#[cfg(test)]
mod tests;

/// Image-space room geometry is projected with the same mapping as every target.
pub fn project(stage: Rect, view: Rect, world: Rect) -> Rect {
    Rect::new(
        stage.x + (world.x - view.x) * stage.w / view.w,
        stage.y + (world.y - view.y) * stage.h / view.h,
        world.w * stage.w / view.w,
        world.h * stage.h / view.h,
    )
}

pub fn draw(g: &Game, stage: Rect, interactive: bool) -> Option<UiAction> {
    let phone = screen_width() < 650.;
    let mut view = Rect::new(0., 0., 1536., 900.);
    let short = screen_height() < 500.;
    if phone || short {
        let (cx, _) = g.hq.focus.unwrap_or(Room::Common).center();
        view.x = (cx - 310.).clamp(0., 916.);
        view.w = 620.;
        if short {
            view.y = if matches!(g.hq.focus, Some(Room::Recovery | Room::Records)) {
                0.
            } else {
                410.
            };
            view.h = 490.;
        }
    }
    let art = g
        .assets
        .get_texture("building")
        .expect("Required headquarters building");
    draw_texture_ex(
        art,
        stage.x,
        stage.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(stage.size()),
            source: Some(view),
            ..Default::default()
        },
    );
    let mut action = None;
    if interactive {
        for room in Room::ALL {
            let (cx, cy) = room.center();
            let world = Rect::new(
                cx - if matches!(room, Room::Gate | Room::Training) {
                    85.
                } else {
                    195.
                },
                cy - 120.,
                if matches!(room, Room::Gate | Room::Training) {
                    170.
                } else {
                    390.
                },
                250.,
            );
            let target = project(stage, view, world);
            if let Some(hit) = target.intersect(stage) {
                if activated(hit) {
                    action = Some(UiAction::Room(room));
                }
            }
        }
    }
    for room in Room::ALL {
        let (cx, _) = room.center();
        let y = match room {
            Room::Recovery | Room::Records => 138.,
            Room::Training => 470.,
            _ => 450.,
        };
        let r = project(stage, view, Rect::new(cx - 78., y, 156., 45.));
        if r.x >= stage.x
            && r.right() <= stage.right()
            && r.y >= stage.y
            && r.bottom() <= stage.bottom()
            && !((phone || short) && r.y < stage.y + 100.)
        {
            let target = Rect::new(r.x, r.y, r.w.max(90.), 44.);
            panel(Rect::new(target.x, target.y + 7., target.w, 30.), PANEL);
            label(room.name(), target, 15., GOLD);
            if interactive && !g.hq.dragged && activated(target) {
                action = Some(UiAction::Room(room));
            }
        }
    }
    for (enabled, source, world) in [
        (
            g.guild.services.training_yard,
            Rect::new(0., 0., 768., 512.),
            Rect::new(1260., 570., 270., 210.),
        ),
        (
            g.guild.services.infirmary,
            Rect::new(768., 0., 768., 512.),
            Rect::new(95., 290., 130., 110.),
        ),
    ] {
        if enabled {
            let dest = project(stage, view, world);
            if dest.x >= stage.x && dest.right() <= stage.right() {
                draw_texture_ex(
                    g.assets
                        .get_texture("facilities")
                        .expect("Required facility atlas"),
                    dest.x,
                    dest.y,
                    WHITE,
                    DrawTextureParams {
                        source: Some(source),
                        dest_size: Some(dest.size()),
                        ..Default::default()
                    },
                );
            }
        }
    }
    for (id, _) in g.guild.roster.iter().enumerate() {
        if g.hq
            .transition
            .as_ref()
            .is_some_and(|t| t.arriving && t.people.contains(&id))
        {
            continue;
        }
        let state = activity(&g.guild, id);
        let Some(room) = state.room() else { continue };
        let slot = id % 6;
        let (x, y) = match room {
            Room::Recovery => (
                190. + (slot % 3) as f32 * 145.,
                402. - (slot / 3) as f32 * 25.,
            ),
            Room::Training => (
                1320. + (slot % 3) as f32 * 70.,
                795. - (slot / 3) as f32 * 45.,
            ),
            _ => (
                250. + (slot % 3) as f32 * 115.,
                821. - (slot / 3) as f32 * 50.,
            ),
        };
        let resting = matches!(state, Activity::Resting | Activity::Recovering);
        let feet = project(stage, view, Rect::new(x, y, 0., 0.));
        if feet.x < stage.x + 20.
            || feet.x > stage.right() - 20.
            || feet.y < stage.y + 110.
            || feet.y > stage.bottom()
        {
            continue;
        }
        let height = (stage.h * if resting { 0.22 } else { 0.27 }).max(92.);
        if g.hq.sheet == Sheet::Jobs && g.party.contains(&id) {
            draw_ellipse_lines(feet.x, feet.y, height * 0.26, 7., 0., 2., GOLD);
        }
        if state == Activity::Training {
            active_figure(g, id, true, vec2(feet.x, feet.y), height, WHITE);
        } else {
            figure(g, id, resting, vec2(feet.x, feet.y), height, WHITE);
        }
        let rw = if phone { 112. } else { 118. };
        let tag_y = if resting {
            feet.y + 2.
        } else {
            feet.y - height - 31.
        };
        let tag = Rect::new(
            (feet.x - rw * 0.5).clamp(stage.x, (stage.right() - rw).max(stage.x)),
            tag_y,
            rw,
            28.,
        );
        if !short {
            panel(tag, PANEL);
            label(
                &format!("{} · {}", first(g, id), state.label()),
                tag,
                14.,
                if state == Activity::Ready {
                    Color::new(0.68, 0.82, 0.58, 1.)
                } else {
                    GOLD
                },
            );
        }
        let target = Rect::new(feet.x - 26., feet.y - height - 31., 52., height + 36.);
        if interactive && !g.hq.dragged && (activated(target) || activated(tag)) {
            action = Some(if g.hq.sheet == Sheet::Jobs {
                UiAction::Party(id)
            } else {
                UiAction::Dossier(id)
            });
        }
    }
    let elowen = project(stage, view, Rect::new(832., 816., 0., 0.));
    if elowen.x > stage.x && elowen.x < stage.right() {
        figure(g, 3, false, vec2(elowen.x, elowen.y), stage.h * 0.25, WHITE);
    }
    if let Some(t) = &g.hq.transition {
        let p = project(stage, view, Rect::new(1130., 820., 0., 0.));
        for (slot, &id) in t.people.iter().enumerate() {
            let progress = (t.elapsed / 1.2).clamp(0., 1.);
            let x =
                p.x + (if t.arriving { 1. - progress } else { progress }) * 70. - slot as f32 * 30.;
            if x >= stage.x && x < stage.right() {
                active_figure(
                    g,
                    id,
                    false,
                    vec2(x, p.y),
                    stage.h * 0.25,
                    Color::new(
                        1.,
                        1.,
                        1.,
                        if t.arriving { 1. } else { 1. - progress * 0.8 },
                    ),
                );
            }
        }
        text(
            if t.arriving {
                "PARTY RETURNED"
            } else {
                "PARTY DEPARTING"
            },
            Rect::new(p.x - 85., p.y - 110., 170., 30.),
            17.,
            GOLD,
        );
        if button(
            Rect::new(stage.x + 14., stage.bottom() - 52., 132., 44.),
            "Skip motion",
            false,
        ) {
            action = Some(UiAction::SkipMotion);
        }
    }
    action
}

fn active_figure(g: &Game, id: usize, training: bool, feet: Vec2, height: f32, tint: Color) {
    let source = Rect::new(
        id as f32 * 512.,
        if training { 512. } else { 0. },
        512.,
        512.,
    );
    draw_texture_ex(
        g.assets
            .get_texture("activity")
            .expect("Required activity atlas"),
        feet.x - height * 0.5,
        feet.y - height,
        tint,
        DrawTextureParams {
            source: Some(source),
            dest_size: Some(vec2(height, height)),
            ..Default::default()
        },
    );
}

pub fn figure(g: &Game, id: usize, resting: bool, feet: Vec2, height: f32, tint: Color) {
    // Explicit crops accommodate the artist's hand-spaced atlas, including bows.
    let source = if resting {
        match id {
            0 => Rect::new(24., 544., 350., 446.),
            1 => Rect::new(350., 544., 435., 446.),
            2 => Rect::new(800., 544., 350., 452.),
            _ => Rect::new(1200., 530., 280., 490.),
        }
    } else {
        Rect::new(id as f32 * 384. + 35., 0., 340., 535.)
    };
    let width = height * source.w / source.h;
    draw_texture_ex(
        g.assets
            .get_texture("people")
            .expect("Required people atlas"),
        feet.x - width * 0.5,
        feet.y - height,
        tint,
        DrawTextureParams {
            source: Some(source),
            dest_size: Some(vec2(width, height)),
            ..Default::default()
        },
    );
}
