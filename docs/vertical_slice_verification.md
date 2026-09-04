# Vertical slice verification - 2026-09-04

- Required `./publish.ps1` with no parameters: PASS. Windows and WebGL release
  builds packaged with both portraits and deployed to the configured preview.
- `cargo test -p occupational_hazard`: PASS, seven tests including the source-size
  and asset registry gates. Gameplay tests cover all three candidates reaching
  Bronze, the subsequent Bronze commission, booking restrictions, no early or
  duplicate rewards, retreat/recovery, and save serialization/validation.
- `cargo clippy -p occupational_hazard --all-targets -- -D warnings`: PASS.
- `git diff --check`: PASS.
- Deterministic rendering inspected at 1280x720: title, contract desk, dossier,
  return report, and Bronze celebration. Menu also captured.
- Deterministic rendering inspected at 360x640: full promotion contract brief,
  party selection, and promotion approval. The toolkit handles touch input.
- Screenshot files are directly in docs/verification. catalog_thumbnail.png is
  the current title capture. No live browser input automation was performed.

The end-to-end gameplay verification is driven by the actual Guild simulation,
including travel days and explicit promotion. Capture scenes use isolated seeded
presentation states and never write the player's autosave.

## Guild services extension - 2026-09-04

Eleven tests pass, including old-save loading, purchase affordability and
idempotence, scouting consumption, training eligibility, and infirmary recovery.
The original Iron-to-Bronze progression tests still pass. Clippy passes with
warnings denied. Services screens were inspected at 1280x720 and 360x640;
existing desktop and compact captures were refreshed.
