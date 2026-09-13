# Published input pass — 2026-09-14

The no-argument `publish.ps1` run built Windows and WebGL, packaged 14 assets,
deployed the Preview package and recorded the publish with Project Roost. The
standard Preview host was not listening during the check, so the deployed
Preview directory was served at
`http://127.0.0.1:8765/games/occupational_hazard/` for the browser pass. This
was the published WebGL package, not a source or development build.

## Live pointer path

The Codex in-app browser used pointer clicks and a drag on the 1280×720 game
canvas. The following states were visibly verified:

- new-guild confirmation with an existing save, including KEEP MY GUILD;
- contextual onboarding through welcome, selection, dispatch, time, reports
  and recovery, plus the complete handbook from 1/8 through 8/8;
- commission selection, adventurer selection, dispatch, return animation,
  unread report, report detail and acknowledgement;
- rested-and-unassigned warning, BACK TO HEADQUARTERS cancellation and explicit
  ADVANCE DAY confirmation;
- MENU > TEXT: LARGE, with the enlarged menu and handbook remaining legible;
- advancing the saved ledger to day 30, opening the failed HEAD-OFFICE REVIEW,
  CONTINUE SANDBOX, reopening the frozen review and opening RESTART confirmation;
- dragging across a pending restart control left the confirmation open, so the
  drag did not activate it.

No physical touchscreen device was available; browser pointer automation is
not a claim of hardware touch testing. The day-30 live path intentionally used
the saved 104g ledger and therefore displayed the failed review; the responsive
fixtures below cover both passed and missed review compositions.

## Current responsive fixtures

Deterministic captures from the current source were refreshed directly in this
directory:

- 390×844: [confirm](ui_phone_confirm_new.png), [tutorial](ui_phone_tutorial.png),
  [idle warning](ui_phone_idle_warning.png), [review](ui_phone_review.png),
  [missed review](ui_phone_review_missed.png), [objectives](ui_phone_objectives.png),
  [settings](ui_phone_settings.png) and [large text](ui_phone_large_text.png);
- 844×390: [confirm](ui_landscape_confirm_new.png), [tutorial](ui_landscape_tutorial.png),
  [idle warning](ui_landscape_idle_warning.png), [review](ui_landscape_review.png),
  [missed review](ui_landscape_review_missed.png), [objectives](ui_landscape_objectives.png),
  [dispatch readiness](ui_landscape_dispatch_readiness.png),
  [settings](ui_landscape_settings.png) and [large text](ui_landscape_large_text.png).

The responsive capture harness is deterministic and does not replace the live
pointer pass. Rust tests cover the state transitions and save/reload behavior;
the browser run covers the visible controls and release/drag interaction path.
