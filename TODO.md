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
observations. No new browser, hardware-touch, native capture, test or publish
pass was performed for this documentation-only audit.

Preserve what works: the desktop cutaway fills the screen; focused work uses
centered windows; Dispatch and Advance Day already have primary styling; save
management is behind Menu; reports are retrievable; tutorial acknowledgements
and skip state are saved. This is already a game-specific composition, not an
unmodified template demonstration. Do not restore the old dashboard or remove
essential room, recovery, warning or utility entry points.

Verification baseline for the tasks: 1280×720 desktop, 1024×768 tablet,
390×844 portrait and 844×390 short landscape, measured as actual game canvas
sizes. These are documented capture sizes, not a declared minimum support
contract. Task UI-01 must declare that contract in README/GDD. Check normal and
larger text, native and embedded browser rendering, and touch-only paths without
keyboard or hover. Replace equivalent captures directly in `docs/verification/`.
After each meaningful implementation change, run `./publish.ps1` without
parameters and report the result; retain relevant regression coverage under
`tests/`, UI actions/state ownership, toolkit helpers and the 800-line limit.

### Confirmed findings and implementation tasks

- [ ] **UI-01 — Recompose the headquarters around the current day's decision.**
  **Screen/files:** Overview and room focus; `src/ui/headquarters.rs::draw`,
  `headquarters/phone.rs::overview`, `headquarters/chrome.rs::header` and
  `journey_card`, `headquarters/scene.rs::{scene_view,project,room_targets,person_tag}`;
  `README.md`/`gdd.md` for the screen brief. Paths under `headquarters/` here and
  below mean `src/ui/headquarters/`.
  **Observed:** The saved 390×844 overview gives the building about 257 pixels
  of height (roughly 30%), surrounded by two navigation rows, three permanent
  roster cards, commissions, journey and day controls. Current code confirms
  the `h - 560` stage budget and fixed three-row overview. Normal play has too
  many peer regions, and the small world is difficult to inspect. Desktop
  Review and Menu are adjacent, identically styled controls, mixing a gameplay
  objective with utilities. README/GDD lack the new seven-part screen brief and
  explicit normal/minimum canvas declaration.
  **Change:** First record current decision, dominant focus, primary action,
  supporting/deferred facts, camera and touch feedback for overview, planning
  and returns. Make the cutaway/selected room the dominant overview area; move
  the full roster/fatigue list to a visibly labelled Staff disclosure and keep
  only actionable availability/recovery status beside people. Remove the
  redundant Overview button while already at overview. Combine assignment and
  return status into one compact supporting area; keep Advance Day reachable
  and identify the next useful action without automatically advancing time.
  Relocate Review beside the day/deadline state and spatially separate Menu.
  Preserve discoverable room/staff alternatives for targets too small in art.
  Reframe narrow and short views around selectable occupants instead of
  stretching the image: `project` currently scales x/y independently and
  `scene_view` uses fixed 620×490 short-screen crops. Reuse toolkit transform
  helpers, retaining one mapping for art and hit targets.
  **Acceptance:** At normal play the player sees one dominant workplace and
  at most two supporting attention regions. Available work, urgent returns,
  recovery and advancing the day are easy to find; utilities are visually
  separate. People remain recognizable and selectable at declared minimum
  sizes; removing cards does not leave unused space or hidden controls.
  **Verify:** Baseline sizes, all-ready/everyone-away/injured/pending-return
  states; tap each room and person, open/close work, resize and rotate, and
  confirm picking still matches art. Check display scaling and browser canvas
  dimensions, not only native screenshot dimensions.

- [ ] **UI-02 — Reserve space for contextual guidance and carry it into the relevant work window.**
  **Depends on:** UI-01 composition. **Screen/files:** First-use headquarters
  and party selection; `src/ui/headquarters.rs::draw`, `src/ui/help.rs`,
  `headquarters/phone.rs`, `headquarters/quest.rs`, `src/tutorial.rs` and help
  copy in `assets/`.
  **Observed:** `ui_phone_tutorial.png` shows the welcome panel covering Pip's
  row and the commission entry. Code places guidance at `h - 282` without
  reserving overview space and suppresses clicks under it. The early return
  from the open-sheet branch also bypasses guidance entirely, although
  `help::tip` has selection/dispatch text specifically for that sheet.
  **Change:** Reserve or reflow a concise hint beside the actual next control,
  including in planning; do not overlay interactive roster/action space.
  Name the visible phone control (currently Compare commissions) rather than
  requiring an unseen Assignments label. Highlight the matching portrait or
  Dispatch control in the focused window. Keep Next Tip/Skip and reopenable
  Menu > Help, and remove each hint after its acknowledgement.
  **Acceptance:** A new player can follow every lesson through dispatch and
  return without first dismissing guidance to reach the instructed control.
  Completed lessons do not remain as permanent explanatory panels.
  **Verify:** New guild and resumed partial tutorial at all baseline sizes,
  especially 390×844 and 844×390 with larger text; tap Review, commissions,
  portraits, Dispatch, Advance Day, returns, trial and approval. Verify Skip,
  reload and Help without keyboard input or accidental actions beneath hints.

- [ ] **UI-03 — Keep essential commission terms and timing beside the final dispatch decision.**
  **Screen/files:** Offer comparison, contract, party and short readiness pages;
  `headquarters/commissions.rs::draw`, `phone.rs::{contract_page,party_page,short_readiness,terms}`,
  `quest.rs::draw_party`, `src/board.rs::{offer_notice,cutoff_notice}`.
  **Observed:** Code shows Acceptance & service terms only when compact
  content height exceeds 620. At 844×390 and the 1024×768 narrow sheet it is
  unavailable. The terms contain whether a job grants new service credit,
  repeats existing credit or grants none. Offer rows do not show that distinction.
  The saved phone-party and landscape-readiness captures, corroborated by code,
  omit return date/review cutoff at Dispatch, unlike desktop. Players must
  remember facts from another page and can select work that cannot meet quota.
  **Change:** Put new/already-earned/no service credit in the offer comparison
  and selected job's decision summary. Provide a visible terms disclosure at
  every supported size. Keep selected party, danger/readiness, return day and
  an explicit after-review warning with Dispatch on compact layouts; show
  blocking reasons there too. Preserve reward, acceptance expiry and scouting
  cost/consumption/unaided-trial restrictions without turning the final page
  into another full contract brief. Do not invent a dispatch fee.
  **Acceptance:** Immediately before dispatch, a touch player knows who is
  leaving, the material risk, when they return and whether success helps the
  current quota. No necessary fact is unreachable because of screen height.
  **Verify:** Baseline sizes; free cellar, new service, already-credited service,
  solo trial, Bronze commission, unaffordable scouting and days 28–30. Tap
  offers → contract → terms → party → readiness → back/edit → dispatch; confirm
  selection persists and deadlines update after time advances.

- [ ] **UI-04 — Reflow the review so the outcome stays readable and recovery choices stay separate from utilities.**
  **Screen/files:** Objectives, passed/missed day-30 review and sandbox review;
  `src/ui/month.rs::draw_month` and review copy in `assets/`.
  **Observed:** `ui_landscape_review_missed.png` renders the explanation as a
  tiny line while three full-width actions dominate. Code reserves `h - 325`
  for the entire result, treasury and careers: only 41 pixels at 844×390.
  Continue Sandbox, Restart and Menu/Save occupy one undifferentiated action
  stack, contrary to gameplay/navigation separation.
  **Change:** Lead with readable outcome and missed/completed targets. Put
  treasury/career details behind a visible Details page or deliberate scroll
  area on short screens; reserve enough space for the explanation before
  laying out controls. Give continuation clear primary treatment, keep Restart
  secondary with its existing replacement confirmation, and move Menu/Save to
  separate quiet chrome. Apply the same hierarchy to pending objectives.
  **Acceptance:** At the minimum height a player can read why the review
  passed/failed, inspect the saved details, and choose continuation or restart
  without mistaking save/menu controls for gameplay decisions.
  **Verify:** Passed/missed/pending/frozen sandbox review at all baseline sizes,
  normal/large text and large treasury values; tap Details/back, Continue,
  Restart/Keep My Guild, Menu/Save and return. Preserve the frozen review data.

- [ ] **UI-05 — Contain active journeys and report history within the journal's available space.**
  **Screen/files:** Dense expedition journal; `headquarters/returns.rs::list`,
  `src/game/capture.rs`, report fixtures under `tests/` if state logic changes.
  **Observed (code-confirmed geometry; no dense live capture):** Every active
  expedition consumes 60 pixels before reports are laid out, then report rows
  are forced to at least one with `.max(1.)`. At 844×390 the content height is
  276; three supported solo expeditions consume the space needed for report
  history and its footer. A guild with prior reports and all three people away
  can place a report below the window or across pagination. Single-line journey
  and report button labels also rely on width fitting for long titles.
  **Change:** Make active journeys and report history share a bounded paged or
  scrollable list, or expose clearly labelled separate views with unread count.
  Reserve footer/navigation space before allocating rows; never force a row
  into insufficient space. Use readable multi-line entries for name, job and
  return day instead of shrinking one long button label.
  **Acceptance:** Every supported active journey and old/unread report can be
  reached and read without overlap, clipped controls or tiny entry labels;
  unread returns remain easy to locate and acknowledgement never repays rewards.
  **Verify:** First reproduce with three solo expeditions and existing report
  history, then many reports and long authored titles at all baseline sizes.
  Tap every entry, page both collections, inspect/reopen a report and return to
  headquarters using only visible controls.

- [ ] **UI-06 — Separate short-lived event feedback from durable state and recoverable errors.**
  **Screen/files:** Dispatch/day/purchase/promotion feedback and save failures;
  `src/game.rs::Game::notice`, `src/game/actions.rs`, `src/ui/headquarters.rs::draw`,
  `src/ui.rs::menu` and `src/ui/title.rs`.
  **Observed (source-confirmed, not timed live):** A single `notice: String`
  holds success events and errors. Dispatch/new-day messages have no lifetime
  or dismiss control, so they persist until selected actions replace/clear them.
  Tutorial guidance takes precedence and open work windows bypass the notice
  drawing path, potentially hiding a purchase/autosave failure where it occurs.
  This both duplicates current state and weakens critical feedback.
  **Change:** Use state-owned feedback with explicit severity and lifetime.
  Expire or dismiss ordinary success feedback after a readable interval; keep
  day, gold, journeys and unread reports as the durable state. Show actionable
  errors in the active screen above tutorial priority, with readable details
  and a visible retry/dismiss route; unrelated party selection must not silently
  erase an unresolved persistence error. Preserve retrievable results in reports.
  **Acceptance:** Calm headquarters does not retain yesterday's event prose.
  Save/purchase failures are visible when they happen and remain recoverable;
  results still make sense after a toast/animation ends or motion is reduced.
  **Verify:** Dispatch, advance without returns, purchase, promotion, multiple
  simultaneous returns and controlled save failures, with tutorial on/off and
  a work window open. Wait for ordinary feedback expiry, reopen history, retry
  saving, and verify all baseline sizes without hiding controls.

- [ ] **UI-07 — Give party readiness a concise default and disclose the calculation on demand.**
  **Depends on:** UI-03 to protect essential warnings/terms while simplifying.
  **Screen/files:** Desktop/phone planning and short readiness;
  `headquarters/preparation.rs::{member,summary,advice,scout}`,
  `planning.rs::cards`, `phone.rs`, `quest.rs`, relevant text catalog entries.
  **Observed:** Saved planning captures and current functions show class bonus,
  fatigue subtraction and contribution on every person, then strength versus
  difficulty, scouting bonus and specialty again in the summary, followed by
  numeric rest/scout advice and the scout purchase. All arrive at first use.
  Exact arithmetic competes with the people/risk choice and repeats facts.
  **Change:** Default cards to identity, availability, relevant class fit and
  fatigue/injury; lead the party summary with qualitative readiness and the
  actionable reason or remedy. Move the full contribution/difficulty breakdown
  behind a labelled Readiness details disclosure. Explain scouting once beside
  its priced control, exposing the optional explanation when requested/relevant.
  Preserve numerical transparency through that disclosure, all disabled reasons,
  trial restrictions, healer relevance and a clear selected-party indicator.
  **Acceptance:** The player can compare people and risk at a glance, then
  inspect the exact deterministic factors by tap. No balance or availability
  rule changes, hidden required cost, or invented research/unlock system.
  **Verify:** First commission, selected/unselected staff, fatigue, injury,
  everyone away, outmatched/close-call/well-prepared parties, scouted route and
  unaided trial across baseline sizes; open/close details and change the party
  without losing the Dispatch control or hiding UI-03's warnings.

- [ ] **UI-08 — Reduce redundant framing after the screen hierarchy is settled.**
  **Depends on:** UI-01–UI-07. **Screen/files:** Headquarters labels and focused
  work windows; `src/ui/theme.rs::{panel,button,parchment}`,
  `headquarters/chrome.rs::window`, `scene.rs::{room_targets,open_jobs_badge,person_tag}`,
  `commissions.rs::draw` and `quest.rs`.
  **Observed:** Current theme adds outlines to every panel and inner borders/
  corner fittings on large ones; window adds more framing and a rule; buttons
  add another inner outline. `ui_planning.png` demonstrates nested decoration.
  The commission board repeats its purpose in a chrome title and COMPARE
  COMMISSIONS heading; Assignments has separate room and open-job plaques.
  These peer accents make secondary surfaces compete with selection/actions.
  **Change:** Keep one meaningful window boundary, use spacing/tonal grouping
  inside it, and reserve strong outlines for selection, focus and urgent state.
  Remove the duplicate board heading; combine assignment label/count into one
  discoverable target. Quiet ordinary room/name plaques without deleting labels
  or relying on color alone; retain the guild's brass/leather visual identity.
  Reclaim space for readable content rather than merely removing decoration.
  **Acceptance:** In overview, no more than 2–3 regions demand strong attention;
  in planning the people/commission comparison and Dispatch lead. Selection,
  disabled controls, urgent warnings and touch affordances remain unambiguous.
  **Verify:** Before/after normal and minimum-size captures of overview,
  commissions, planning and returns, including dense selections and large text.
  Tap quieted controls and verify focus/selected/disabled states remain legible.

### Further inspection and acceptance gates (not additional proven defects)

- [ ] **UI-09 — Refresh the visual evidence and verify remaining responsive/input risks.**
  **Screen/files:** `scripts/capture_ui.ps1`, `src/game/capture.rs`,
  `src/ui/{day,dossier,title,theme}.rs`, `headquarters/{management,scene,returns}.rs`,
  `src/game/input.rs`, `docs/verification/README.md` and existing captures.
  **Gap:** Saved evidence mixes revisions and the reviewed large-text capture
  only demonstrates the menu. Current camera/hit-target behavior, dense career/
  facility/report content, long labels and browser scaling have not been
  exercised in this audit. Prior browser records are historical evidence, not
  a new pass. No physical touchscreen verification is claimed.
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
