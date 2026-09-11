# TODO

Follow-up work identified by the updated `AGENTS.md` and `CODE_STANDARDS.md` review.

The current source gate passes: every Rust file is below the 800-line hard limit. The items below are standards-alignment and maintainability work, not current test failures.

## High Priority

- [ ] Migrate feature tests out of `src/**/tests.rs` into the crate-level `tests/` directory as a separate change, then remove every `#[cfg(test)] mod tests;` declaration from `src/`. Introduce `src/lib.rs` as the public test surface and have `main.rs` consume the library, keeping internals private unless an intentional public seam is needed. Affected legacy suites are under `board`, `headquarters`, `reports`, `review`, `services`, `simulation`, `tutorial`, and `ui/headquarters/scene`.
- [ ] Split `src/game.rs` before further feature growth. It is 713 lines, and `Game::update` is 288 lines while `Game::begin_capture_scene` is 176 lines. Move action handlers and deterministic capture-scene setup into cohesive child modules while keeping `Game` as the explicit state/action coordinator.
- [ ] Break up UI functions that exceed the 100-line function maximum: `headquarters::draw` (115 lines), `headquarters::management::career` (120), `headquarters::returns::detail` (131), `headquarters::phone::planning` (155), and `headquarters::scene::draw` (344). Preserve the existing `UiAction` intent boundary and keep rendering separate from mutation.
- [ ] Move balance, configuration, authored setup data, and player-facing copy out of Rust string/numeric literals. Add a typed `assets/data/game_config.json` loaded with `macroquad_toolkit::include_json!` and validate it in project code. Start with initial roster data, starting gold, promotion/review thresholds, service costs, training/recovery/scouting values, caps, room/status labels, tutorial/help text, and repeated report/error copy.

## Standards Cleanup

- [ ] Add a short `//!` module-purpose comment to runtime modules that currently lack one: `src/ui/headquarters/chrome.rs`, `management.rs`, `planning.rs`, `returns.rs`, `scene.rs`, and `src/ui/help.rs`. Test files can be handled by the test migration.
- [ ] Remove unjustified underscore-prefixed parameters: rename `Game::update(_dt)` to `dt`, remove the unused `Lesson::text(_compact)` parameter or implement compact text behavior, and remove the unused `phone::rooms(_g)` parameter and its call-site argument.
- [ ] Reconcile test counts after migration. `src/board/tests.rs` has seven cases and `src/simulation/tests.rs` has six, exceeding the updated target of no more than five cases per major feature. Consolidate related inputs with table-driven assertions where appropriate; if distinct regression coverage must remain, document why each case is necessary.
- [ ] Add focused tests for the new typed configuration and semantic validation, plus any public API seams created by the test migration. Keep those tests in `tests/` and use the toolkit parser/loading APIs for game data.

## Verification And Documentation

- [ ] Add a repeatable published-browser touch pass covering start/new-guild confirmation, every tutorial step, core dispatch/report/recovery flows, review completion or continuation, and restart recovery. Keep the evidence in `docs/verification/`, replacing captures of the same screen/state instead of adding duplicates.
- [ ] Align `GAME_DEVELOPMENT_GUIDE.md` with the updated data-loading guidance. Its project tree still calls the module `data/loader.rs` and its example uses the older loader shape; update the shared source document and redistribute it through the documented sync path so examples agree with `CODE_STANDARDS.md` §5.3.
- [ ] Review startup asset loading in `Game::new`: consume the typed texture manifest instead of duplicating asset paths in Rust, and make missing asset/config failures visible with source/path context during publishing and startup diagnostics.
