# Cutaway visual comparison — 2026-09-09

First integrated captures at 1280×720: ui_gameplay.png, ui_planning.png,
ui_report.png. Compared directly with mockups 01, 02 and 03 respectively.

| Axis | Observed result |
| --- | --- |
| Composition | Building dominates; narrow header/footer; planning/report at right |
| Architecture | Two-storey timber cutaway, slate roof, warm rooms, cool castle vista |
| Navigation | Selectable room/person plaques plus Jobs, Staff, Returns, Advance Day |
| Character scale | Full-body sprites inside rooms; no permanent full-height portrait |
| Planning | Route vignette, three portrait cards, readiness, return date, copper dispatch |
| Return | Face/name and outcome precede report body and already-applied rewards |
| Differences | Existing character identities retained; simpler buttons; denser type; smaller cards |

Visual estimate: headquarters 75–80%, planning and returns 70–75% similar to
their respective mockups. This is a qualitative review, not an objective pixel
metric or unfamiliar-player result. The 70% gate is provisionally reached for
these three desktop compositions, so publishing and meaningful regression checks
began only after this gate. Final responsive captures and published-browser input
are recorded below.

The first capture attempt caught an invalid dated-offer fixture on day 12. The
fixture now uses day 9, when the expedition is legally dispatchable. No gameplay
availability rule was relaxed for the screenshot. A later pass moves recovery
person labels below their feet to avoid colliding with room labels.

The old comparison images remain available in git history at b1b3b1e and earlier.
The former UI used three broad tabs, parchment contracts, text-row party selection
and a permanent Elowen guidance strip. The new composition changes all four.

## Final published verification

The no-argument publish script passed on 2026-09-09 for Windows and WebGL,
including Preview deployment, all 10 registered assets, catalog thumbnail and
Project Roost tracking. Strict Clippy passed; 31 unit tests, one asset registry
test and one source-size test passed. Two shared chroma-key tests also passed
during the artwork milestone.

Live target: http://127.0.0.1/games/occupational_hazard/ in the Codex in-app
Chromium browser on this Windows host. No local development server was substituted.
The existing browser save continued at day 5, 182g, one unread report and tired
staff. Using only visible pointer targets, the agent:

- Selected Mira, closed planning, reopened it with selection retained, dispatched
  Cellar, Meet Sword, and observed her departure.
- Advanced to day 6, observed arrival and two unread reports, opened Mira's report,
  and verified 206g (+24g) and a single remaining unread report.
- Resized to 390×844, read the report, navigated to Training, purchased the yard
  for 140g, observed new dummies and saved from the visible Menu.
- Resized to 844×390, opened Jobs and Details, browsed a dated offer and scouted
  it for 20g. The control became Route scouted and gold became 46g.
- Reloaded the final published build, tapped Continue and verified day 6, 46g,
  the purchased yard and one unread report. The updated facility description
  correctly explained its active benefit.
- Resized to 1024×768, tapped Tomas's actual recovery-room sprite, inspected his
  career, and prepared his trial. Missing XP/successes were explained and dispatch
  remained disabled.

This is agent-operated pointer evidence, not an unfamiliar-player study or a
physical touchscreen result. Native deterministic captures cover 1280×720,
1024×768, 390×844 and 844×390. A complete month is covered by deterministic rule
tests; a complete independent manual month at every viewport is not claimed.

Live review caught overlapping character plaques, stale purchased-facility copy,
and missing job names in short Details. These are corrected. Short journey views
now remain explicitly read-only, and accepted return dates drive their review label.

## Capture scenarios

All paths below are directly in docs/verification. Prefixes phone_, tablet_ and
landscape_ after ui_ identify their respective target viewport sizes above.

| Capture | Setup and expected visible outcome |
| --- | --- |
| ui_gameplay / ui_planning | Day 9; Mira ready, Pip tired, Tomas away; planning selects Mira and Pip for the well job |
| ui_everyone_away | Three separately accepted journeys; home adventurers absent; Elowen and next-return shortcut remain |
| ui_report / ui_reports | Two day-2 returns; Pip's medicine success, recovery advice and already-applied reward; separate inbox entries |
| ui_injured_return | Unsupported Pip retreats from the well; medical leave precedes reward information |
| ui_facilities | Both facilities purchased; new medical trolley and training dummies; Pip injured, Mira and Tomas training |
| ui_blocked_trial | New recruit lacks trial prerequisites; explicit blocker and disabled dispatch |
| ui_promotion | Fixture candidate has passed unaided trial; career requirements and Approve Bronze visible |
| ui_review / ui_review_missed | Day-30 met/missed objective fixtures; sandbox continuation and restart remain visible |
| ui_arrival / ui_departure | Brief travel poses at the gate; skip control; simulation has already applied the transition |
| ui_title / catalog_thumbnail | Delivered cutaway title artwork and start/continue controls |
| ui_browser_landscape | Actual published day-6 save, purchased training yard after reload at 844×390 |

## Open evidence

Desktop spot measurements using the shared toolkit frame-time overlay showed 60 FPS / 16.6 ms in headquarters and planning at 1280×720. Host: Windows, AMD Ryzen 7 5800X, NVIDIA RTX 4080 SUPER; browser: Codex in-app Chromium. See ui_browser_performance.png. This is a smoothed frame-time spot check, not a sustained benchmark. The F3 developer overlay was used only after the pointer-only gameplay checks. Physical touch
hardware was unavailable. The plan's three unfamiliar-player sessions and timed
discoverability observations remain open unless explicitly deferred by the owner.
For each participant record device, viewport, task duration, misclicks, uncoached
dispatch/return completion and their explanation of fatigue or return timing.
Targets are three completed loops and two correct tradeoff explanations.


## Follow-up accessibility audit

A code and capture audit found a 16-pixel overlap between phone Review and Menu.
Review now ends 8 pixels before Menu begins; both retain at least 44-pixel height.
The welcome and day-advance tutorial targets now receive visible outlines, and the
trial tip explicitly names Staff, Prepare Solo Trial and Dispatch Party. Compact
person plaques retain separate name and activity lines, including Injured, so the
absence of space no longer removes activity text. Updated phone tutorial and
injured-return captures were inspected. The ledger now incorporates the published
pointer observations already recorded above; its previous bulk status update had
not changed the corresponding table cells.

Remaining validation is deliberately narrower than the implemented feature list:
complete manual loops at every viewport, exhaustive navigation/guidance and motion
coverage, and the three unfamiliar-player sessions still require further evidence.

## Completed live loops and final input audit

The same published save completed one full assignment loop at each requested size:
390×844: Mira's day-7 dispatch and day-8 return; 844×390: rested Mira's day-10
unaided trial, day-12 success and explicit Bronze approval; 1024×768: Tomas and
Pip's day-12 dispatch and day-13 return; 1280×720: Mira's day-14 dispatch and
day-15 return. Each result was opened through the report list. Additional live
screenshots are ui_browser_phone, ui_browser_promotion, ui_browser_tablet and
ui_browser_desktop. The saved Bronze rank was inspected after a browser reload.

A drag across a selected portrait did not change the party. Direct sprite selection
matched the displayed person after resizing. Settings exposed and saved larger text
and reduced motion; larger-text Help remained readable. Reduced-motion dispatch and
return placed people immediately without travel animation. Single-input checks
confirmed that Skip motion changes no date and Advance Day consumes one day.

One rapid automated two-click batch had consumed an extra day; single separated
inputs did not reproduce it. Batched portrait clicks also differed from separate
clicks, so automation event timing may contribute. A 250ms guard now coalesces
rapid duplicate day-change inputs. This guard belongs to the game's presentation
of a discrete calendar decision; simulation next_day remains unchanged. A dedicated
regression test covers two immediate inputs, an overlay change and a later deliberate
advance. On the final published build, an explicit double click advanced day 15 to
16 exactly once and applied the 24g return reward once (198g to 222g).

Latest validation: 32 unit tests, one asset-registry test and one source-size test
passed (34 total), strict Clippy passed, and the no-argument publisher passed for
Windows, WebGL, Preview, asset packaging, catalog and tracking. Earlier counts above
refer to their recorded milestones. No physical touch or unfamiliar-player results
are claimed. All automated/native/manual-agent evidence is separate from those two
external requirements.
