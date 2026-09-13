//! Pointer gesture ownership, kept separate from rendering and state actions.

use super::Game;
use macroquad_toolkit::ui::Pointer;

pub(super) fn capture(game: &mut Game) {
    let pointer = Pointer::read(|p| p);
    if pointer.down {
        let start = *game.hq.pointer_start.get_or_insert(pointer.position);
        if start.distance(pointer.position) > 10. {
            game.hq.dragged = true;
        }
    }
    if pointer.released
        && game
            .hq
            .pointer_start
            .is_some_and(|start| start.distance(pointer.position) > 10.)
    {
        game.hq.dragged = true;
    }
    if !pointer.down && !pointer.released {
        game.hq.pointer_start = None;
        game.hq.dragged = false;
    }
}
