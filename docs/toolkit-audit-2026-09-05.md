# Toolkit audit — 5 September 2026

The review listed no migration finding. Current production source inspection
confirms existing toolkit adoption:

- `src/contracts.rs` loads the authored catalogue through toolkit `include_json!`.
  Contract identity, arrival schedule and licence validation remain local rules.
- `src/game.rs` uses AssetManager for portraits and toolkit save-slot existence,
  loading and saving. Board migration and guild validation remain game-owned.
- `src/ui.rs` uses shared Pointer hit testing, surfaces, centered text, measured
  text and text blocks. Its small button helper applies the guild's colors and
  readable font floor; it contains no wrapping or truncation loop.
- `src/main.rs` uses the toolkit capture lifecycle. Simulation, board scheduling,
  services, review and tutorial modules implement deterministic guild rules.

No local generic JSON loader, storage backend, random generator, audio bank,
particle engine or free-camera controller needs replacing. The game does not
require those last four systems. Adding unused toolkit features would not
improve adoption.

Validation: 28 checks, formatting, strict all-target/all-feature Clippy and
Rust source-size limits passed. Default `publish.ps1` passed Windows/WebGL
release builds, packaging with five assets, Preview deployment and Project
Roost tracking. No compiler, test or publisher warnings were reported.
