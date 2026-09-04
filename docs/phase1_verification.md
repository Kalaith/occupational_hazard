# Phase 1 verification

## First-month review

2026-09-04: `cargo test -p occupational_hazard` passed 15 tests, including
the established class promotion paths and four new review regressions. A fresh
guild earns Bronze and a commission returning exactly on day 30; its rewards
and career advances are included before review. Failed and late-work paths
reload, continue, and pay once without changing the frozen review. Legacy
saves without month state load and receive a review if past the cutoff.

No-argument `./publish.ps1` passed Windows and WebGL release builds, packaging
and deployment to the configured WSL preview. Review and objective screenshots
at 360x640 were visually checked in `docs/verification/`.

## Contextual onboarding

Seventeen tests pass (15 unit and two integration checks); clippy with warnings
as errors passes. Browser clicks exercised selection, dispatch, NEXT DAY,
reports, recovery, approval and the intermediate celebration. This fresh guild
certified Mira on day 12 and returned a Bronze commission on day 17, then
received its successful review on day 30 with 352g and Mira at 130 XP/five
successes. The successful review screenshot is `browser_review.png`.

Testing prompted separate trial and approval guidance and fixed consecutive
lessons so BACK TO DESK actually returns to the desk. Compact tutorial and
review native captures are legible. The first browser run exposed 30-day caching of the old shared stylesheet.
The shared publisher now versions the CSS URL by content hash. Its itch
rewrite retains local asset paths with that version, and viewport pages hide
external widget launchers which could cover bottom-row game controls.
The publisher regression script and the game's no-argument publisher both
pass after this fix (rust_management commit 73614a3).

## Published browser completion checks

Actual browser clicks (not injected game state) on the configured preview verified:

- Successful review reloaded at 360x640, retaining 352g, Mira's Bronze rank,
  130 XP and five successes. CONTINUE SANDBOX advanced to day 31 and survived reload.
- SANDBOX reopened the frozen review. RESTART offered replacement confirmation;
  KEEP MY GUILD cancelled it, and START NEW GUILD reset to day 1/80g with tutorial.
- SKIP TUTORIAL persisted across reload, while MENU > HELP exposed all eight
  legible compact pages. CHOOSE PARTY, adventurer selection and DISPATCH worked.
- Reload during a day-1 expedition retained the travelling party and paid 24g
  once on day 2. Advancing to day 29 left the balance at 104g.
- The two-day Beekeeper assignment displayed a day-31/too-late warning before
  dispatch. Day 30 produced an understandable failed review at 104g. Reload
  retained that result and the outstanding expedition.
- Continuing the failed review paid the late expedition once on day 31,
  reaching 152g. Two recovery days left the balance unchanged and cleared
  Tomas's fatigue and medical leave. The frozen review remains at 104g.

The browser canvas was measured at 360x640 after the stylesheet fix; desktop
captures use 1280x720. These are mouse-click tests at desktop and compact
viewports, not a claim of physical touchscreen-device coverage. Native capture
checks are layout verification, not an independent native interactive playthrough.

Evidence lives directly in docs/verification/: browser_review_mobile.png,
browser_review_missed_mobile.png, browser_cutoff_mobile.png and
browser_help_mobile.png, with deterministic title, objectives, promotion and
review captures. The catalog thumbnail was refreshed from the new title screen.
## Remaining human gate

The roadmap's unfamiliar-player observation and measured session-duration
tuning require a human participant and have not been performed. A repeatable
protocol is in phase1_playtest.md. The one-certification/one-commission target
remains provisional. The deterministic route reaches both by day 17, but an
agent's tool-driven interaction time is not valid evidence for a new player's
30–45 minute session. Phase 2 remains gated on that observation.
