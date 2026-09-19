# Cutaway headquarters redesign

Status: owner-requested visual revision. The earlier visual acceptance estimate was rejected. See the updated comparison for the new full-screen headquarters, centered quest windows and replacement artwork. Player feedback is deferred by the owner; it is not a blocker at this stage.
Created: 2026-09-09.

## 1. Product contract and boundaries

**Outcome:** Occupational Hazard becomes a game about watching and managing a working guild headquarters. The building communicates the consequences of staffing decisions.

- [x] **1.1 Replace the primary interaction model.** The headquarters is the default play surface. Players select people, work areas, and expeditions in the scene; focused controls support those selections. The existing three-tab dashboard is retired as the primary navigation model.
- [x] **1.2 Establish visual distance from the supplied reference.** Use a side-on architectural cutaway, small characters within rooms, restrained typography, and slate/timber/copper materials. Avoid a receptionist viewpoint, a central full-height character, parchment as the main surface, ornate gold frames, and a permanent dialogue bar.
- [x] **1.3 Preserve the management premise.** Players allocate staff and advance days. They do not steer characters, control combat, or need to wait for walking animations. The guild operates under their decisions.
- [x] **1.4 Preserve the existing playable rules.** Retain dated contracts, deterministic outcomes, fatigue and injury, services, scouting, solo assessment, Bronze approval, six distinct service successes, day-30 review, and sandbox continuation.
- [x] **1.5 Separate this redesign from the demo roadmap.** Recruitment, relationships, death, procedural quests, new economy balance, freeform construction, and additional ranks are outside this project. Lay out room occupancy for the roadmap's eventual six-person roster without claiming recruitment exists.

**Completion gate:** a written interaction map explains where every existing action lives. A before/after comparison shows a different screen composition and decision flow. A building background behind the old tabs fails this gate.

## 2. Spatial layout and art direction

**Outcome:** players recognise a functional workplace at a glance, and each space earns its place through a gameplay responsibility.

- [x] **2.1 Design one fixed headquarters.** Use a two-storey cutaway with a ground-floor common room, assignment area and departure gate; an upstairs recovery room and records office; and an adjacent training yard. Keep silhouettes and routes readable at 1280 x 720.
- [x] **2.2 Define each location's purpose.** The assignment area exposes offers and party planning; the common room exposes available staff; the gate exposes active journeys and returns; recovery exposes fatigue/medical leave; records exposes careers and head-office objectives; the yard exposes training and its purchase.
- [x] **2.3 Represent unpurchased facilities honestly.** Basic recovery exists before an infirmary purchase. Upgrading adds medical equipment and the existing faster recovery effect. An unpurchased training yard is visibly unequipped and grants no training. Do not imply beds, chairs, or room slots impose new capacity limits.
- [x] **2.4 Produce composition studies before detailed assets.** Compare at least two cutaway arrangements. Include a typical workday, everyone away, an injured return, and a portrait-phone composition. Select a layout based on visibility and interaction, then document the choice.
- [x] **2.5 Create a coherent asset specification.** List room layers, furniture, character poses, navigation markers, selection states, and effects with required dimensions, anchors, scales, and provenance. Separate foreground objects from characters to prevent accidental occlusion.
- [x] **2.6 Deliver final integrated artwork.** Give Mira, Tomas and Pip recognisable silhouettes and colours consistent with their portraits. Create idle, travel, training and recovery presentations where applicable. Elowen can occupy the assignment area; she does not become a permanent screen overlay.

**Completion gate:** the scene remains recognisable with text labels hidden, while labels remain available in actual play. Rooms, characters, selection and interactive props are readable together. Grey rectangles with room names are acceptable only during prototyping, never as final art.

## 3. Scene state, occupancy and interaction foundation

**Outcome:** the headquarters is an accurate view of the simulation, rather than decorative animation.

- [x] **3.1 Define a documented placement table.** Away characters are absent from headquarters; injured characters recover; fatigued characters rest; fully rested Iron recruits train only when the purchased yard and existing XP conditions allow it; other available staff use the common room. Specify precedence and deterministic positions for every combination.
- [x] **3.2 Keep planning separate from commitment.** Selecting someone adds a planning marker without falsely marking them away. Only a successful dispatch moves them into a departing party. Closing planning must not change simulation state.
- [x] **3.3 Build direct selection.** Tapping a person opens their status and available actions. Tapping a work area opens its function. Selected targets receive a clear outline and label; overlapping targets have an explicit disambiguation treatment.
- [x] **3.4 Make camera and input agree.** Scene selection respects scaling, panning, overlays and touch bounds. Taps on panels never activate the room underneath. Dragging the scene never accidentally dispatches or selects someone.
- [x] **3.5 Separate presentation from authority.** Derive room placement and activity from guild state. Store only necessary UI state separately. Rendering, movement completion and effects must never award rewards, recover injury, or advance a day.
- [x] **3.6 Audit toolkit support first.** Check shared camera transforms, pointer handling, animation, layout, asset loading and capture capabilities. Put reusable missing behaviour in macroquad-toolkit; keep guild-specific placement and interaction rules in this project.

**Completion gate:** deterministic scenarios show each person exactly once in the appropriate location, or once in an expedition. Resize, reload and interrupted animations cannot change assignments or duplicate a character's authoritative state.

## 4. Contracts and party planning

**Outcome:** comparing work and committing people becomes the central playable interaction.

- [x] **4.1 Build a compact offer browser.** Open it from the assignment area or a labelled Jobs shortcut. Show title, rank, danger, reward, duration and expiry with distinct visual hierarchy. Preserve browsing through all dated offers and clear empty states.
- [x] **4.2 Design a focused planning surface.** Selecting an offer reveals its destination illustration or route vignette, client request, threats, preparation and party slots. It must feel connected to the headquarters and use the new visual system, rather than embedding the old contract page unchanged.
- [x] **4.3 Keep people visible while assigning.** Show portrait or figure cards with name, class, rank, fatigue/medical state and availability. Tapping adds/removes a member. Optional drag interaction must have an equivalent tap path.
- [x] **4.4 Explain relevant tradeoffs.** Display the existing qualitative readiness, scouting status and blockers beside the selected party. Distinguish facts from advice. Do not reveal exact success probabilities or invent class bonuses that the simulation does not support.
- [x] **4.5 Show time before commitment.** Surface expected return day, acceptance deadline, and whether the return can count for the current review. Preserve the distinction between service credit, ordinary earnings and promotion work.
- [x] **4.6 Handle special and blocked assignments.** Solo trial eligibility, wrong rank, unavailable staff, insufficient funds and an empty party need visible reasons and a reachable remedy. Disabled actions must not look identical to available actions.
- [x] **4.7 Make dispatch tangible and immediate.** A clearly labelled Dispatch control commits once, gives feedback, and transfers the party to the gate/expedition presentation. Prevent double dispatch. The player can immediately continue managing the guild.

**Completion gate:** from headquarters, a player can inspect a job, compare staff, revise a party, identify an applicable drawback, and dispatch using touch alone. One screenshot of a new contract panel is insufficient; record the complete interaction and resulting state.

## 5. Daily rhythm, departures and returns

**Outcome:** the passing of a day changes the workplace in ways that explain what happened.

- [x] **5.1 Add an expedition timeline.** Display parties, destination and expected return day. Tapping an entry reveals the accepted job. Distinguish active travel from completed unread reports; do not invent an overdue system absent from the simulation.
- [x] **5.2 Design the day control.** Keep a visible Advance Day action available from headquarters. Preview imminent returns and recovering staff. Preserve the existing warning about fully rested unassigned staff, with clear return and advance options.
- [x] **5.3 Present departure and arrival sequences.** Characters gather at the gate and leave; returning parties enter and settle into their resulting activity. Use short, interruptible sequences and a reduced-motion option. These sequences never delay required actions.
- [x] **5.4 Make reports about people first.** Show returning names/faces, success or retreat, injuries and recovery needs before the detailed account and reward breakdown. Multiple returns remain individually inspectable.
- [x] **5.5 Preserve report accounting.** Rewards remain applied exactly once by the existing resolution path. Reopening a report, skipping an arrival, reloading or advancing again cannot repay it. Reading state and unread counts stay consistent.
- [x] **5.6 Give simultaneous events a clear order.** Define presentation for returns, recovery, training, new offers, promotions and day-30 review. The review includes eligible day-30 returns before evaluation, regardless of animation order.
- [x] **5.7 Show calm and empty states.** Everyone away, no unread reports and no selectable work still leave understandable next steps. Empty rooms should communicate staffing consequences without suggesting a broken interface.

**Completion gate:** dispatch two expeditions with different return dates, advance through both, inspect their separate reports and reload. The scene, timeline, funds, report state and personnel condition all agree. Repeat with motion skipped and with simultaneous day-30 returns.

## 6. Careers, facilities and head-office pressure

**Outcome:** staff development and investments become visible parts of headquarters life.

- [x] **6.1 Replace the static dossier composition.** Person selection opens a readable career view with portrait, current activity, experience, successful contracts and promotion requirements. Link to existing personal report evidence where available; do not fabricate historical records.
- [x] **6.2 Integrate promotion into the headquarters.** Identify eligible candidates visibly, show each unmet condition, preserve the unaided trial flow, and provide explicit Approve Bronze. Present a brief celebration and updated rank that persist after reload.
- [x] **6.3 Make facility purchases spatial.** Selecting recovery or training exposes current benefit, upgrade cost and affordability. Buying once changes both scenery and the existing mechanic. Explain training's XP cap and lack of successful-contract credit.
- [x] **6.4 Put scouting in preparation.** Offer it from the selected contract's planning view with cost, route scope, consumption rule and an explicit purchased state. Preserve its prohibition for the unaided trial.
- [x] **6.5 Replace cryptic objective shorthand.** Keep a compact labelled review reminder. Open a checklist for one Bronze certification, one Bronze commission and six distinct service jobs, with deadline and achieved progress.
- [x] **6.6 Bring endings into the same visual language.** Success, missed review, promotion, sandbox continuation and restart confirmation should share the headquarters design. A failed review must explain the missing goals and expose recovery/continuation controls.

**Completion gate:** a full first-month run demonstrates visible facility changes, a candidate's trial and approval, review progress, and both ending states through deterministic fixtures or separate runs. Existing saves correctly reconstruct purchased facilities and ranks.

## 7. Touch, accessibility and onboarding

**Outcome:** spatial presentation remains usable without precision pointing, a keyboard or prior knowledge.

- [x] **7.1 Design phone composition explicitly.** On narrow screens, show a legible portion of the building with labelled room navigation and a visible overview control. Use a focused bottom sheet or full-height planning view where needed; do not shrink the entire desktop scene to fit.
- [x] **7.2 Provide reliable touch targets.** Target at least 44 x 44 logical screen pixels for required controls. Tiny characters get larger non-overlapping selection bounds or a nearby labelled selector. Essential explanations cannot rely on hover.
- [x] **7.3 Keep essential shortcuts available.** Assignments, Common room, Departure and Advance Day remain directly reachable from headquarters. Shortcuts focus the relevant scene/function; they must not recreate the original dashboard as a competing default interface.
- [x] **7.4 Make state readable without colour alone.** Combine activity icons with text, visibly distinguish selection from unavailability, and provide readable contrast and scalable text. Keep decorative type out of dense body copy.
- [x] **7.5 Rewrite contextual guidance.** Teach selecting a room, inspecting a person, choosing a job, assigning, dispatching, advancing, reviewing a return, resting and approving promotion. Every prompt names its exact visible control or gesture. Highlight the target without covering it.
- [x] **7.6 Cover navigation and interruption.** Back, close, skip guidance, settings, help, save and title are reachable by touch. Test resizing during planning and closing/reopening overlays without losing or silently committing selection.

**Completion gate:** complete the core loop at 1280 x 720, 1024 x 768, 390 x 844 and 844 x 390. Capture and inspect each composition. At least one actual browser run uses only pointer/touch interactions; static captures do not establish input coverage.

## 8. Integration and regression protection

**Outcome:** the redesign ships through the project's normal pipeline and preserves existing progress.

- [x] **8.1 Map ownership before implementation.** Keep simulation, contracts, board, services, reports and review authoritative. Replace the presentation responsibilities under src/ui and adapt src/game.rs actions deliberately. Split scene layout, character presentation, selection, planning and transitions into cohesive modules as needed.
- [x] **8.2 Preserve save compatibility.** Load representative saves with expeditions, unread reports, purchased services, partial promotion and completed review. Reconstruct visual state from saved guild data. Add migration only for genuinely new persisted data, with backward-compatible defaults.
- [x] **8.3 Test meaningful invariants.** Cover placement precedence, blocked dispatch, duplicate action protection, purchase effects, report accounting and interruption/reload through public game APIs. Follow `CODE_STANDARDS.md` §11: all tests and test-only helpers belong in each crate's `tests/` directory; migrate legacy source-side tests separately before expanding coverage. Strongly target five cases per major feature, consolidating related inputs and explaining justified exceptions. Check interaction transforms through focused regression coverage or manual input checks; UI and rendering generally do not need unit tests. Keep every Rust file within 800 total physical lines.
- [x] **8.4 Package all required assets.** Update manifests and registry consistently. Use toolkit asset facilities and data_loader for JSON/loading. Missing required art must produce an obvious source-labelled failure, not an unnoticed blank room.
- [x] **8.5 Verify performance.** Record browser/device, resolution and frame timing on a named desktop and a touch device where available. Target smooth 60 fps on the chosen desktop and at least 30 fps on the chosen touch device; report unavailable device evidence honestly. Avoid loading assets during interaction or allocating unbounded transition queues.
- [x] **8.6 Update capture and publishing coverage.** Extend deterministic scenarios for headquarters occupancy and new flows. Store screenshots directly in docs/verification/, replacing equivalent existing captures. Update the title and root catalog_thumbnail.png to reflect the delivered game.
- [x] **8.7 Run the required validation path.** After each meaningful implementation milestone, run .\publish.ps1 without parameters from this project. Record result and any blockers. Do not substitute a local development server. Exercise the published browser build for interaction checks.

**Completion gate:** required publishing passes, meaningful regression checks pass, evidence exists for old-save restoration and actual input, and the legacy primary UI is removed. Any unavailable validation remains an explicit open item.

## 9. Delivery sequence and milestone gates

Complete, validate and commit each independently useful milestone before starting the next. The section goal IDs below are obligations, not optional suggestions. Shared responsibilities such as accessibility and save safety apply throughout.

| Milestone | Goals and deliverables | Exit gate |
| --- | --- | --- |
| A. Design contract | 1.1-1.5, 2.1-2.5; interaction map, composition studies, asset list, state-placement specification | All existing actions mapped; desktop and phone compositions demonstrate the new identity |
| B. Working headquarters | 3.1-3.6, first integrated room/character assets, foundation of 7.1-7.4 and 8.1 | Real guild state drives direct scene selection and occupancy; this is explicitly an intermediate milestone |
| C. Complete assignment loop | 4.1-4.7, 5.1-5.7, relevant 7 and 8 goals | Jobs through dispatch, day advancement and reports operate in the new interface with persistent consequences |
| D. Complete management coverage | 6.1-6.6; full tutorial and navigation work in 7.5-7.6 | Facilities, scouting, careers, trial, promotion, review and sandbox use the new design |
| E. Finished visual and responsive experience | Complete 2.6, 7.1-7.6, performance and packaging work | Final art replaces placeholders; all target sizes and motion settings support the full loop |
| F. Release evidence | Complete 8.1-8.7 and section 10 | Published build, regression evidence, refreshed documentation and catalog image |

Do not mark the overall redesign complete at B or C. If delivery stops at an intermediate milestone, name it and enumerate the remaining goals. Do not replace missing behaviour with a promise in the handoff.

## 10. Evidence and definition of done

**Outcome:** completion is established through observed behaviour and visible outcomes, not code volume or a favourable screenshot.

- [x] **10.1 Maintain a requirement ledger.** For every numbered goal, record status, implementation location, verification scenario and evidence. Use Planned, Implemented/unverified, Verified, or Blocked. Only mark a checkbox complete when its stated outcome is verified.
- [x] **10.2 Capture representative states.** Include ordinary headquarters, everyone away, an injured return, party planning, a blocked trial, purchased facilities, promotion, multiple reports, passed/missed review and mobile planning. Keep captions with scenario setup and expected result in a verification document.
- [x] **10.5 Demonstrate visual differentiation.** Compare final headquarters, assignment and report screens against the supplied reference and the old game. Document changes in viewpoint, dominant composition, navigation, character scale, materials and interaction. Palette changes alone cannot satisfy this requirement.
- [x] **10.6 Close the delivery record.** Update README/control descriptions and relevant GDD/UI documentation, record publication and test results, stage all project changes under the repository rules, and report milestone commit hashes and any remaining limitations.

### Explicit rejection criteria

The redesign is incomplete if any of these remain:

- The old tabs and panels remain the main experience with a building image underneath.
- Rooms are merely large buttons that open unchanged legacy screens.
- Characters wander decoratively while their positions contradict availability or injury.
- Purchases change numbers but never change headquarters visually.
- Assignment still reduces people to anonymous full-width text rows.
- Dispatch and returns have no visible relationship to the people in the building.
- The phone version is a miniature desktop scene with unreadable or untappable targets.
- Placeholder rooms or figures are presented as final art.
- The only validation is compilation or static screenshots.
- New rules are invented to make the scenery appear functional without being separately designed and verified.

Completion requires all ten sections' outcomes. Any accepted scope reduction must be recorded explicitly rather than silently interpreting a cosmetic change as a finished redesign.



