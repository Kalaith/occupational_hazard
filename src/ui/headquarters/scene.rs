//! Projected headquarters art, people, and touch targets.
use super::*;

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
    let short = screen_height() < 500.;
    let view = scene_view(g, stage, phone, short);
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
    let mut action = room_targets(g, stage, view, interactive, phone, short);
    if interactive && !phone && !short && open_jobs_badge(g, stage, view) {
        action = Some(UiAction::Room(Room::Assignments));
    }
    draw_facilities(g, stage, view);
    if let Some(next) = draw_people(g, stage, view, phone, short, interactive) {
        action = Some(next);
    }
    draw_clerk(g, stage, view);
    draw_foregrounds(art, stage, view);
    if let Some(next) = draw_transition(g, stage, view) {
        action = Some(next);
    }
    action
}

fn scene_view(g: &Game, stage: Rect, phone: bool, short: bool) -> Rect {
    let mut view = Rect::new(0., 0., 1536., 900.);
    if phone && g.hq.focus.is_none() {
        view.h = (1536. * stage.h / stage.w).min(1024.);
        view.y = (900. - view.h).max(0.);
    }
    if (phone && g.hq.focus.is_some()) || short {
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
    view
}

fn room_targets(
    g: &Game,
    stage: Rect,
    view: Rect,
    interactive: bool,
    phone: bool,
    short: bool,
) -> Option<UiAction> {
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
    for room in Room::ALL
        .into_iter()
        .filter(|_| !phone || g.hq.focus.is_some())
    {
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
            label(room.name(&g.guild.text), target, 15., GOLD);
            if interactive && help::is_target(g, room.name(&g.guild.text)) {
                draw_rectangle_lines(target.x, target.y + 7., target.w, 30., 2., GOLD);
            }
            if interactive && !g.hq.dragged && activated(target) {
                action = Some(UiAction::Room(room));
            }
        }
    }
    action
}

fn open_jobs_badge(g: &Game, stage: Rect, view: Rect) -> bool {
    let posting = project(stage, view, Rect::new(780., 568., 0., 0.));
    let badge = Rect::new(posting.x - 65., posting.y, 130., 44.);
    panel(badge, PANEL);
    label(
        &g.guild.text.format(
            "ui.open_jobs",
            &[(
                "count",
                g.guild.open_contracts(&g.contracts).len().to_string(),
            )],
        ),
        badge,
        16.,
        GOLD,
    );
    activated(badge)
}

fn draw_facilities(g: &Game, stage: Rect, view: Rect) {
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
}

fn draw_people(
    g: &Game,
    stage: Rect,
    view: Rect,
    phone: bool,
    short: bool,
    interactive: bool,
) -> Option<UiAction> {
    let mut action = None;
    for (id, _) in g.guild.roster.iter().enumerate() {
        if g.hq
            .transition
            .as_ref()
            .is_some_and(|t| t.arriving && t.people.contains(&id))
        {
            continue;
        }
        if let Some(next) = draw_person(g, stage, view, id, phone, short, interactive) {
            action = Some(next);
        }
    }
    action
}

fn draw_person(
    g: &Game,
    stage: Rect,
    view: Rect,
    id: usize,
    phone: bool,
    short: bool,
    interactive: bool,
) -> Option<UiAction> {
    let state = activity(&g.guild, id);
    let room = state.room()?;
    let (x, y) = person_slot(room, id);
    let resting = matches!(state, Activity::Resting | Activity::Recovering);
    let feet_rect = project(stage, view, Rect::new(x, y, 0., 0.));
    if feet_rect.x < stage.x + 20.
        || feet_rect.x > stage.right() - 20.
        || feet_rect.y < stage.y + 110.
        || feet_rect.y > stage.bottom()
    {
        return None;
    }
    let feet = vec2(feet_rect.x, feet_rect.y);
    let height =
        (stage.h * if resting { 0.15 } else { 0.19 }).max(if phone && g.hq.focus.is_none() {
            28.
        } else {
            82.
        });
    draw_ellipse(
        feet.x,
        feet.y - 2.,
        height * 0.23,
        5.,
        0.,
        Color::new(0., 0., 0., 0.32),
    );
    if g.hq.sheet == Sheet::Jobs && g.party.contains(&id) {
        draw_ellipse_lines(feet.x, feet.y, height * 0.26, 7., 0., 2., GOLD);
    }
    let layout = PersonLayout {
        stage,
        view,
        id,
        room,
        state,
        resting,
        feet,
        height,
        phone,
        short,
    };
    draw_person_figure(g, &layout);
    let tag = person_tag(g, &layout);
    let target = Rect::new(feet.x - 26., feet.y - height - 31., 52., height + 36.);
    if interactive && !g.hq.dragged && (activated(target) || activated(tag)) {
        return Some(if g.hq.sheet == Sheet::Jobs {
            UiAction::Party(layout.id)
        } else {
            UiAction::Dossier(layout.id)
        });
    }
    None
}

fn person_slot(room: Room, id: usize) -> (f32, f32) {
    let slot = id % 6;
    match room {
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
    }
}

struct PersonLayout {
    stage: Rect,
    view: Rect,
    id: usize,
    room: Room,
    state: Activity,
    resting: bool,
    feet: Vec2,
    height: f32,
    phone: bool,
    short: bool,
}

fn draw_person_figure(g: &Game, layout: &PersonLayout) {
    let room_light = Color::new(0.80, 0.75, 0.65, 1.);
    if layout.resting {
        let bed = project(
            layout.stage,
            layout.view,
            Rect::new([178., 337., 493.][layout.id % 3] - 76., 254., 152., 148.),
        );
        draw_texture_ex(
            g.assets.get_texture("rest_beds").expect("occupied bed"),
            bed.x,
            bed.y,
            room_light,
            DrawTextureParams {
                source: Some(Rect::new(layout.id as f32 * 512. + 14., 30., 484., 464.)),
                dest_size: Some(bed.size()),
                ..Default::default()
            },
        );
    } else if layout.state == Activity::Training {
        active_figure(g, layout.id, true, layout.feet, layout.height, room_light);
    } else {
        figure(g, layout.id, false, layout.feet, layout.height, room_light);
    }
}

fn person_tag(g: &Game, layout: &PersonLayout) -> Rect {
    let spacing = match layout.room {
        Room::Recovery => 145.,
        Room::Training => 70.,
        _ => 115.,
    } * layout.stage.w
        / layout.view.w;
    let rw = spacing.clamp(44., 118.);
    let tag_y = if layout.resting {
        layout.feet.y - layout.height - 47.
    } else {
        layout.feet.y - layout.height - 31.
    };
    let tag = Rect::new(
        (layout.feet.x - rw * 0.5).clamp(
            layout.stage.x,
            (layout.stage.right() - rw).max(layout.stage.x),
        ),
        tag_y,
        rw,
        28.,
    );
    if !layout.short && (!layout.phone || g.hq.focus.is_some()) {
        panel(tag, PANEL);
        let caption = if rw < 100. {
            first(g, layout.id).to_string()
        } else {
            g.guild.text.format(
                "ui.person_caption",
                &[
                    ("name", first(g, layout.id).to_string()),
                    ("activity", layout.state.label(&g.guild.text).to_string()),
                ],
            )
        };
        label(
            &caption,
            if rw < 100. {
                Rect::new(tag.x, tag.y, tag.w, 14.)
            } else {
                tag
            },
            if rw < 100. { 12. } else { 14. },
            if layout.state == Activity::Ready {
                Color::new(0.68, 0.82, 0.58, 1.)
            } else {
                GOLD
            },
        );
        if rw < 100. {
            label(
                if layout.state == Activity::Recovering {
                    g.guild.text.get("ui.injured")
                } else {
                    layout.state.label(&g.guild.text)
                },
                Rect::new(tag.x, tag.y + 14., tag.w, 14.),
                12.,
                MUTED,
            );
        }
    }
    tag
}

fn draw_foregrounds(art: &macroquad::texture::Texture2D, stage: Rect, view: Rect) {
    // Restore the floor beam and desk apron after occupants are drawn.
    for world in [
        Rect::new(86., 402., 976., 22.),
        Rect::new(684., 743., 211., 55.),
    ] {
        let dest = project(stage, view, world);
        draw_texture_ex(
            art,
            dest.x,
            dest.y,
            WHITE,
            DrawTextureParams {
                source: Some(world),
                dest_size: Some(dest.size()),
                ..Default::default()
            },
        );
    }
}

fn draw_transition(g: &Game, stage: Rect, view: Rect) -> Option<UiAction> {
    let Some(t) = &g.hq.transition else {
        return None;
    };
    let mut action = None;
    let p = project(stage, view, Rect::new(1130., 820., 0., 0.));
    for (slot, &id) in t.people.iter().enumerate() {
        let progress = (t.elapsed / 1.2).clamp(0., 1.);
        let x = p.x + (if t.arriving { 1. - progress } else { progress }) * 70. - slot as f32 * 30.;
        if x >= stage.x && x < stage.right() {
            active_figure(
                g,
                id,
                false,
                vec2(x, p.y),
                stage.h * 0.19,
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
            g.guild.text.get("ui.party_returned")
        } else {
            g.guild.text.get("ui.party_departing")
        },
        Rect::new(p.x - 85., p.y - 110., 170., 30.),
        17.,
        GOLD,
    );
    if button(
        Rect::new(stage.x + 14., stage.bottom() - 130., 132., 44.),
        g.guild.text.get("ui.skip_motion"),
        false,
    ) {
        action = Some(UiAction::SkipMotion);
    }
    action
}

fn draw_clerk(g: &Game, stage: Rect, view: Rect) {
    let elowen = project(stage, view, Rect::new(832., 816., 0., 0.));
    if elowen.x > stage.x && elowen.x < stage.right() {
        figure(
            g,
            3,
            false,
            vec2(elowen.x, elowen.y),
            stage.h * 0.18,
            Color::new(0.80, 0.75, 0.65, 1.),
        );
    }
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
    // Crops retain each complete working pose and its contact with the floor.
    let source = if resting {
        match id {
            0 => Rect::new(40., 550., 330., 420.),
            1 => Rect::new(404., 550., 338., 420.),
            2 => Rect::new(800., 550., 318., 420.),
            _ => Rect::new(1180., 512., 340., 480.),
        }
    } else {
        Rect::new(id as f32 * 384. + 28., 12., 338., 502.)
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
