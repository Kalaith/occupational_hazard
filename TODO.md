# Occupational Hazard TODO

## UI_STYLE review — 2026-09-20

Audit and planning only; no UI implementation was changed. This file was empty
at the start, so there were no existing tasks or completion records to merge.
Order below follows player impact and dependencies; finish composition before
decorative changes. References are project-relative unless stated otherwise.

### Evidence and scope

Read `AGENTS.md`, `UI_STYLE.md`, `CODE_STANDARDS.md`,
`GAME_DEVELOPMENT_GUIDE.md`, `README.md`, `gdd.md`, and the cutaway design and
verification records. No project `PROJECT_AGENTS.md` was found. Inspected the
screen dispatcher, action flow, headquarters projection/input, planning,
commissions, careers/facilities, reports, review, menu, title and tutorial code.
The implemented three-person, first-month slice is the scope; the GDD's future
recruitment, deaths and procedural contracts are not missing-UI findings.

Inspected existing captures directly in `docs/verification/`: `ui_gameplay.png`,
`ui_phone_gameplay.png`, `ui_pending_returns.png`, `ui_phone_pending_returns.png`,
`ui_planning.png`, `ui_landscape_planning.png`, `ui_phone_party_readiness.png`,
`ui_landscape_dispatch_readiness.png`, `ui_commissions.png`, `ui_phone_report.png`,
`ui_phone_tutorial.png`, `ui_review_missed.png`,
`ui_landscape_review_missed.png`, `ui_phone_large_text.png`, `ui_services.png`
and `ui_tablet_gameplay.png`.

These are saved evidence, not a fresh run. Some have old colors/chrome;
`ui_services.png` and `ui_tablet_gameplay.png` visibly retain the superseded
title/tab/side-panel layout. Do not implement fixes to those obsolete surfaces.
The comparison record explicitly distinguishes older captures from refreshed
ones. Findings below identify source confirmation separately from screenshot
observations. At audit start, no new browser, hardware-touch, native capture,
test or publish pass had been performed.

Preserve what works: the desktop cutaway fills the screen; focused work uses
centered windows; Dispatch and Advance Day already have primary styling; save
management is behind Menu; reports are retrievable; tutorial acknowledgements
and skip state are saved. This is already a game-specific composition, not an
unmodified template demonstration. Do not restore the old dashboard or remove
essential room, recovery, warning or utility entry points.

Declared verification baseline for the tasks: 1280×720 desktop, 1024×768 tablet,
390×844 portrait and 844×390 short landscape, measured as actual game canvas
sizes. Task UI-01 recorded this support contract in README/GDD. Check normal and
larger text, native and embedded browser rendering, and touch-only paths without
keyboard or hover. Replace equivalent captures directly in `docs/verification/`.
After each meaningful implementation change, run `./publish.ps1` without
parameters and report the result; retain relevant regression coverage under
`tests/`, UI actions/state ownership, toolkit helpers and the 800-line limit.

### Confirmed findings and implementation tasks

Implementation record — 2026-09-20

UI-01 through UI-08 were completed and verified in the refreshed native captures. The completed task text was removed after implementation, while the original audit and verification record above is preserved.

### Further inspection and acceptance gates (not additional proven defects)

- [ ] **UI-09 — Refresh the visual evidence and verify remaining responsive/input risks.**
  **Screen/files:** `scripts/capture_ui.ps1`, `src/game/capture.rs`,
  `src/ui/{day,dossier,title,theme}.rs`, `headquarters/{management,scene,returns}.rs`,
  `src/game/input.rs`, `docs/verification/README.md` and existing captures.
  **Gap:** Refreshed native captures now cover the declared sizes, including
  dense journeys and reports, but browser canvas scaling, hardware touch, and
  several long-label or large-text states remain unverified. The no-argument
  publisher is blocked by the shared workspace's missing
  `D:\WebHatchery\RustGames\_tiny_necromancer_check2` member. Prior browser
  records are historical evidence, not a new pass. No physical touchscreen
  verification is claimed.
  **Action:** After composition changes, refresh the supported existing scenes
  from the exact validated revision, recording revision, canvas size, text
  setting and fixture state. Cover all baseline sizes and newly declared minima;
  probe immediately around width 650/900 and height 500/700 breakpoints. Check
  full-size text/targets in the actual embedded canvas, normal and larger text,
  long names/titles, large gold/XP values, selected careers, purchased facilities,
  multi-person reports, title replacement confirmation and idle-day warning.
  Inspect whether fitting in `theme::button` or toolkit text helpers defeats
  readability; reflow only where observed, considering shared toolkit changes
  before introducing local generic alternatives. Add specific follow-up tasks
  only for reproduced failures, with state and evidence.
  **Acceptance:** Evidence clearly separates refreshed captures, historical
  evidence and untested cases. Required controls remain readable, at least
  44×44 logical pixels for touch targets, correctly mapped after resize/display
  scaling, and operable without hover/keyboard. No claim of hardware touch or
  uncoached usability is made from pointer automation.
  **Verify:** Visible-control path: New/Continue → tutorial/help → assignments
  → selection/scouting → Dispatch → Advance/cancel idle warning → returns and
  acknowledgement → recovery/trial/promotion → review → sandbox/restart cancel
  → Menu/save/title/reload. Exercise drag cancellation and reduced motion;
  check one intentional tap advances one day. Preserve existing regression
  coverage and publish with no arguments after implementation; document any
  host/device limitation. Keep previously deferred unfamiliar-player studies
  separate from this implementation acceptance gate.
