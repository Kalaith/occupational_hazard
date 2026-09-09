use super::*;

#[test]
fn room_bounds_and_art_share_the_same_projection_at_every_target_size() {
    for (width, height) in [(1280., 720.), (1024., 768.), (390., 844.), (844., 390.)] {
        let stage = Rect::new(0., 62., width, height - 178.);
        let view = if width < 650. {
            Rect::new(0., 0., 620., 900.)
        } else {
            Rect::new(0., 0., 1536., 900.)
        };
        let room = Rect::new(140., 450., 390., 250.);
        let projected = project(stage, view, room);
        let center = project(
            stage,
            view,
            Rect::new(room.x + room.w / 2., room.y + room.h / 2., 0., 0.),
        );
        assert!(projected.contains(center.point()));
        assert!(!projected.contains(vec2(projected.right() + 1., projected.bottom() + 1.)));
        let full = project(stage, view, view);
        assert_eq!(full, stage);
    }
}
