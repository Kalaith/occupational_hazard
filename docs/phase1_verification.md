# Phase 1 verification

The initial checks below describe the pre-feedback build. The later feedback
and fix section supersedes its tutorial, report navigation and repeated-job path.

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
successes. The successful review screenshot is `docs/verification/ui_review.png`.

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

Evidence lives directly in docs/verification/: the current phone review,
missed-review, tutorial and warning captures, with deterministic title,
objectives, promotion and review captures. The catalog thumbnail was refreshed
from the new title screen.
## First unfamiliar-player feedback and fixes

Feedback supplied by the user after the session:

- Tutorial took full focus; the player wanted a subwindow and a highlighted next control.
- Sending adventurers on two missions appeared to show only one result.
- Completed quests should leave the board.
- The player stopped before finishing because of those issues.
- They saw no reason to spend 100g on services given smaller quest rewards,
  although no service was required before they stopped.
- They estimated the roughly five-mission offering looked closer to five
  minutes than thirty. This was an estimate, not a timed completed session.

Corrections are committed separately:

- 8d6fd5d: nonmodal tutorial panel, outlined next controls, automatic step
  acknowledgements and optional NEXT TIP / SKIP TUTORIAL.
- 58dd9fc: an inbox entry for every report, saved read status and an unread
  badge. Simulation regressions establish both simultaneous and staggered
  missions already pay and report independently; visibility was the observed problem.
- eced808: completed ordinary jobs leave the board and cannot dispatch or
  consume scouting again. Failed jobs remain retryable. Six distinct Iron
  jobs preserve three-job promotion routes for every class. Original six-slot
  saves migrate without losing expedition, scouting or completed-job data.
  The trial remains an appointment for candidates who have not passed it.

Twenty-three Rust tests (21 unit, two integration checks), clippy with warnings
as errors and the required no-argument publisher pass after these fixes.
Publication built Windows and WebGL and deployed to the configured preview.
The source-size integration gate passes for every Rust file.

Actual browser clicks at 360x640 verified the revised flow: CHOOSE PARTY,
Mira selection and DISPATCH work while the tutorial remains open. Its next
control changes from D30 to DISPATCH and NEXT DAY through normal play.
Two simultaneous one-day assignments to Mira and Pip returned two visible
inbox entries and raised the treasury from 80g to 134g. Opening each report
cleared its own unread marker. The board advanced past both completed jobs,
and backwards navigation wrapped to a new unfinished job rather than either
completed one. Reload retained day 2, 134g, zero unread reports and removal
of both completed quests. The previous six-job test ledger also loaded with
its day-33/152g state intact and completed jobs absent from navigation.

Screenshots: ui_phone_tutorial.png and ui_phone_pending_returns.png;
updated native ui_tutorial.png, ui_reports.png, ui_report.png and ui_gameplay.png.
No physical touchscreen-device coverage is claimed.

## Completed unfamiliar-player follow-up and pacing decision

The user subsequently reported reaching the ending, with about five minutes
of gameplay. The final mission's requirements were clear. Rank was not clear
as an individual adventurer's progression; additional services still did not
seem necessary. They requested a warning when rested adventurers are left idle
before NEXT DAY. This is a reported approximate duration, not a timestamped
measurement; review result, career dates and service purchases were not supplied.

The human completion observation satisfies Phase 1's unfamiliar-player gate.
The rank/objective text now explicitly assigns XP, successes and promotion to
one person. Service titles identify optional purchases and the infirmary states
its improvement over normal recovery. The warning is verified below.

Keep one individual Bronze certification and one successful Bronze commission
as the introductory target. The observed final mission worked; multiplying
promotion requirements would repeat that loop without establishing management
depth. The current slice is about five minutes by this observation. The full
roadmap's 30-45 minute target remains unmet and must be tested through later
assignment and roster decisions. Do not inflate playtime with waits or require
services merely to spend surplus gold. Services' strategic value remains a
balance question for those later phases; current prices are unchanged.

## Rested-staff warning verification

The follow-up player requested a warning before NEXT DAY leaves rested staff
unassigned. The warning lists only fully rested, uninjured people at home;
BACK TO DESK preserves the day and ADVANCE DAY deliberately proceeds.
A regression covers a fresh roster, mixed availability, recovery and everyone
away. At 360x640, actual browser clicks on the published build showed Tomas
alone in the warning on day 2, cancellation retained day 2/134g, and explicit
confirmation advanced to day 3/134g. See ui_phone_idle_warning.png.
All 24 Rust tests (22 unit and two integration) and the required no-argument
publish.ps1 passed. Windows and WebGL packages deployed to the WSL preview.


## Follow-up wording validation

The final build passed the no-argument publisher and clippy with warnings as
errors. Published-browser clicks at 360x640 verified the complete objective
explanation, the individual Iron record, and all three optional-service rows.
Updated ui_objectives.png and ui_phone_services.png now come from this browser
check, replacing captures of those same screens. These checks verify layout
and controls, not a second human comprehension study or physical touch hardware.
