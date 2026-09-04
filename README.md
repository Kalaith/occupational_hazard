# Occupational Hazard

A fantasy guild management game. The complete design is in [gdd.md](gdd.md).

## Current foundation

The title screen opens a clean game workspace with one procedurally drawn settings cog in the top-right corner. Settings provides Close and Return to Title. The grid, actions, energy, camera demo, sample persistence, and their data have been removed. Guild gameplay is not implemented yet.

The UI uses the full current window dimensions without a fixed aspect ratio or letterboxing. Native builds open fullscreen. Browser builds opt into the shared publisher's `layout: viewport` mode so the canvas fills the available browser viewport without surrounding page headings or panels.

## Development

Keep this folder, `macroquad-toolkit`, and `rust_management` as siblings under `RustGames`. Install Rust and the `wasm32-unknown-unknown` target.

```powershell
cargo run -p occupational_hazard
cargo fmt -p occupational_hazard -- --check
cargo test -p occupational_hazard
cargo clippy -p occupational_hazard --all-targets --all-features -- -D warnings
.\scripts\capture_ui.ps1
.\publish.ps1
```

`src/game.rs` owns screen transitions; `src/ui.rs` draws the workspace, procedural cog, and settings; `src/ui/title.rs` draws the responsive title. `asset_registry.json` and `assets/data/texture_manifest.json` remain empty until assets are introduced. Capture scenes are title, gameplay, and settings. The publisher generates the browser page from `game_page.json` and builds Windows/WebGL preview packages. The itch.io target remains unconfigured.

Read `AGENTS.md` and the shared development documents before adding systems. The next gameplay milestone is the guild roster, contracts, and party assignment described in the GDD.
