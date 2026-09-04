# Occupational Hazard

A fantasy guild management simulation about assessing contracts, assigning adventurers, and living with the consequences. The complete design is in [gdd.md](gdd.md), preserved exactly from the supplied document.

## Development status

Initial scaffold copied from `rust_management/template`. The current build runs the template grid/action demonstration with Occupational Hazard branding. Guild gameplay is not implemented yet.

Includes toolkit UI, embedded JSON data, assets, save/load, notifications, event handling, grid helpers, camera controls, tests, screenshot capture, GitHub CI, and Windows/WebGL packaging.

## Setup

Keep `occupational_hazard`, `macroquad-toolkit`, and `rust_management` as sibling folders under `RustGames`. Install Rust and the `wasm32-unknown-unknown` target. The parent Cargo workspace automatically includes this game.

Run from this folder:

```powershell
cargo run -p occupational_hazard
cargo fmt -p occupational_hazard -- --check
cargo test -p occupational_hazard
cargo clippy -p occupational_hazard --all-targets --all-features -- -D warnings
.\publish.ps1
.\scripts\capture_ui.ps1
```

The default publisher builds Windows and WebGL and deploys locally for preview. `game_page.json` supplies the generated browser page. `itch.json` retains a placeholder until an itch.io destination is assigned.

## Layout

- `src/main.rs`: runtime entry and `OCCUPATIONAL_HAZARD_CAPTURE_*` hooks.
- `src/game.rs`: input, events, and lifecycle.
- `src/state.rs`: serializable starter state and updates.
- `src/data.rs`, `assets/data/`: configuration and embedded data.
- `src/ui.rs`: rendering and controls.
- `tests/`: asset integrity and source size checks.
- `docs/verification/`: development captures.

Read `AGENTS.md`, `CODE_STANDARDS.md`, `MACROQUAD_TOOLKIT.md`, and `GAME_DEVELOPMENT_GUIDE.md` before implementing game systems.

## First gameplay milestone

Replace sample grid/actions with a reception desk, persistent roster, contracts, party assignment, and a deterministic day/expedition loop. Then add injuries, rewards, promotions, quotas, recruitment, and save/load for that state, following the GDD prototype scope.

The first design question is: **Is deciding who to send on a quest interesting?**
