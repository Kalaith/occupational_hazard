# Playable demo roadmap

Status: Phase 1 complete, 2026-09-04. The unfamiliar player reached the ending
and reported about five minutes of gameplay. Follow-up warning and personal-rank
clarity fixes are verified. Phase 2 implementation and technical checks are complete; player observation
of its assignment tradeoffs remains open. Phases 3-6 remain proposed work; the full demo's
30-45 minute pacing target is not met by this introductory slice.
See [Phase 1 evidence](phase1_verification.md).

## Demo promise

Run a small guild through its first head-office review. Learn the desk, develop
rookies, recruit help, choose which requests to prioritise, and decide whether
the next expedition is worth the risk. Finish with a review of the people and
decisions that shaped the branch, then restart or continue in a clearly labelled
sandbox.

Target a first session of **30â€“45 minutes across one 30-day review period**.
These are initial pacing targets to validate with players, not established
balance. Keep one branch, Iron and Bronze ranks, the three existing classes,
and a roster capped at six. The GDD's 10â€“15 adventurers, additional ranks and
broader simulation remain longer-term ambitions.

The demo succeeds when players can explain a difficult assignment decision,
remember an adventurer's career, and finish or recover without outside guidance.
More content alone will not establish those outcomes.

## Starting point

The README, implemented sections of the GDD and source currently establish:

- Three persistent Iron recruits, six repeatable authored contracts, class and
  fatigue effects, deterministic expedition results, injury and recovery.
- Explicit Bronze approval after 60 XP, three successes and the solo Lantern
  Road Trial; a subsequent North Bridge commission and slice celebration.
- Contract earnings, a reputation counter, an infirmary, a training yard and
  consumable route preparation. Personality labels are currently flavour.
- Touch/click controls, compact layouts and native/browser autosaves.

The existing verification records simulation tests and captured layouts, but
explicitly does not establish live browser input coverage. Recruitment, changing
contract supply, deadlines, review objectives, consequential reputation and
character history are still demo work.

## Phase 1 â€” Give the first month a beginning and an ending

**Player outcome:** a new player knows what the branch is working toward and can
complete a coherent session with the existing roster and contracts.

- [x] Add a first-month objective panel and a head-office review on day 30. Start
  with a provisional target of one Bronze certification and one successful
  Bronze commission; tune this against actual playthrough length.
  Observed completion is about five minutes. Retain one individually earned
  Bronze certification and one commission as the introductory milestone: the
  final mission was understood and completed. Raising this count alone would
  repeat the same training loop. Meaningful assignment decisions and longer
  pacing remain work for the later demo phases, not extra waiting in Phase 1.
- [x] Introduce the desk through short contextual prompts covering selection,
  DISPATCH, NEXT DAY, return reports, recovery and APPROVE BRONZE. Every prompt
  names its visible control. Allow tutorial skip and later help access.
- [x] Keep the first promotion celebration as an intermediate achievement. The
  final review summarises objective results, careers and finances, with visible
  restart and continue actions. A missed target produces an understandable
  unsuccessful review, not an unexplained blocked game.
- [x] Define day-boundary ordering: resolve returning expeditions and their rewards
  before evaluating the review. Explain the cutoff before the player dispatches
  work that cannot return in time. Review completion and continuation must save.

**Technical gate verified:** fresh-guild success and missed-target paths,
review and sandbox reloads, restart confirmation, browser click controls,
Iron-to-Bronze progression, late returns, and 360x640 layouts.

**Observation gate satisfied:** after the first session's blockers were fixed,
the player reached the ending. Follow-up feedback prompted a rested-idle warning
and explicit per-adventurer rank wording. Services remain optional; no purchase
is needed to pass. The five-minute observation is approximate, not a stopwatch
measurement or proof of broad balance. The later demo still needs substantial
playable decisions to reach its 30-45 minute target.

**Exit gate:** a fresh guild can reach either review result, reload it, and use
the completion controls with clicks/taps alone. The established Iron-to-Bronze
path still works. Observe one unfamiliar player to find onboarding blockers
before adding more systems.

## Phase 2 â€” Make today's assignments compete

Depends on Phase 1. **Player outcome:** there are worthwhile reasons to split
the party, rest someone, or let a contract go.

- [x] Separate authored contract definitions from dated contract instances. Give
  definitions and instances stable IDs; stop tying saved progress and active
  expeditions to positions in a six-item array.
- [x] Add a bounded rotating board with authored arrival schedules, expiry dates
  and clear duration information. Begin with 10â€“12 authored definitions across
  the existing categories, including short safe jobs and overlapping demands.
- [x] Specify expiry as the last day to accept; accepted expeditions may return
  later. Review credit requires return by the review cutoff. Show both rules
  where they affect dispatch.
- [x] Keep the promotion assessment available to eligible candidates. Provide a
  low-risk earning route so a poor assignment does not make recovery impossible.
- [x] Introduce a simple service quota alongside certification. Count qualifying
  returns once and show remaining time and progress. Tune the quota only after
  the board schedule is playable; avoid rewards that make one repeatable job
  the best answer to every objective.
- [x] Preserve a slice-save migration path, including expeditions already away,
  scouting, purchases and completed-contract records. Route content loading
  through the toolkit's data loader with project-owned schema validation.

**Technical evidence:** [Phase 2 verification](phase2_verification.md). Two scripted
month routes pass; three specific tradeoffs are exercised. Player observation
of these choices and fresh-session duration are not yet established.

**Exit gate:** a complete month contains at least three observed decisions
between competing useful assignments. Verify expiry boundaries, one-time
rewards, board refresh after reload and a viable recovery route. Review whether
services versus recruitment savings will have room to matter in Phase 3.

## Phase 3 â€” Make the roster a management decision

Depends on Phase 2. **Player outcome:** hiring and developing another adventurer
changes which contracts the guild can handle.

- Add a small authored applicant pool and scheduled recruitment opportunities,
  growing the roster from three to at most six. Display class, trait, fee and
  available capacity before hiring; start recruits at Iron.
- Use stable adventurer IDs for expeditions and records. Replace the exact
  three-member save validation and any fixed-size roster presentation with
  bounded, scrollable or paginated views.
- Balance hiring against the existing 100g infirmary, 140g training yard and
  20g preparation. Retain their current prices as a baseline until a full-month
  economy pass supplies evidence for changes. Do not add wages or debt yet.
- Record compact dated career events: recruitment, assignments, injury,
  recovery and certification. Make these readable from each dossier.
- Give each existing personality one understandable effect, with report text
  explaining when it matters. Keep effects narrow and distinct from class
  bonuses; avoid a relationship graph or a general personality simulation.

**Exit gate:** hiring creates a useful new scheduling option, six-member party
selection works on compact screens, and saves retain identities and histories.
Compare a facilities-first run with a hiring-first run; both should be viable
without requiring identical outcomes or balances.

## Phase 4 â€” Make risk understandable and consequential

Depends on Phase 3. **Player outcome:** uncertainty creates hesitation, while
reports explain enough for the player to learn from the result.

- Add a small authored set of hidden complications with visible warning clues.
  Distinguish known facts, suspicions and unknowns in the contract brief.
- Expand scouting to reveal useful information as well as providing its
  existing preparation advantage. Show what is purchased and when it is used;
  keep the promotion trial unaided.
- Add partial success and longer but recoverable injuries where they create
  distinct scheduling consequences. Reports should connect outcomes to party
  suitability, fatigue, preparation and relevant traits without showing exact
  success percentages.
- Keep resolution deterministic where practical. If varied outcomes require
  randomness, use guild-owned seeded state and persist resolved dispatch facts
  so reloading cannot reroll a mission.
- Give reputation one bounded consequence, such as access to a special client
  commission, and explain the requirement. Avoid multiple reputation axes.

**Exit gate:** a player can identify a clue before dispatch and explain a setback
afterwards. Equivalent saved states reproduce outcomes, and an injured roster
can recover to useful play. Do not introduce permanent death for this demo:
prove attachment and readable risk first, then evaluate mortality separately.

## Phase 5 â€” Tune the complete month and finish presentation

Depends on Phase 4. **Player outcome:** the whole session has variety, legible
consequences and a satisfying ending without excessive repeated clicks.

- Play through cautious, expansion-focused and aggressive approaches. Tune
  contract arrivals, quotas, rewards, recovery and hiring costs together.
- Ensure the first few days teach the loop, the middle creates competing
  demands, and the final days make review timing matter. Remove empty-day
  busywork without skipping reports or concealing relevant events.
- Finalise authored briefs, applicant identities and report variants within the
  agreed scope. Show meaningful career moments in the review summary.
- Audit text wrapping, contrast, scrolling, selected-party visibility and
  disabled-action explanations at 1280x720 and 360x640. Add restrained feedback
  and audio only where they clarify actions; provide visible audio controls if
  audio ships.
- Explain save location/limitations, handle save errors visibly, and confirm
  replacement of an existing guild. Keep the catalog thumbnail current.

**Exit gate:** the three play styles can reach a review without a soft lock;
at least one plausible recovery run remains viable after an early failure.
Measure session duration and repeated actions before declaring pacing complete.

## Phase 6 â€” Validate and release the demo

Depends on Phase 5. **Player outcome:** the published build works for someone
who has never seen the project.

- Run the required `./publish.ps1` with no parameters from the project directory.
  Record its result and any environment blocker; do not substitute a local
  development server for this gate.
- Exercise the published browser build using actual clicks/taps: new guild,
  tutorial, dispatch, reports, services, hiring, promotion, review, restart and
  continue. Check desktop and a touch device, plus reload during an expedition.
- Verify older slice saves, current saves and invalid-save recovery; no silent
  resets or duplicate payouts. Check packaged assets and native build behavior.
- Run focused simulation/save regression tests and clippy. Store tests in child
  files and keep every Rust source file at or below 800 physical lines.
- Observe at least three unfamiliar players. Target all three completing the
  first dispatch unaided and at least two finishing a review and describing one
  meaningful roster tradeoff. Treat this small sample as a usability gate, not
  proof of broad appeal. Fix blockers and repeat affected checks.
- Save verification evidence directly in `docs/verification/`, replacing captures
  of the same state. Update README and the implemented GDD sections to match
  what actually ships, including any deferred features.

**Release gate:** required publishing passes; there are no known progression,
save-loss or touch-control blockers; observed sessions broadly meet the pacing
target; and the demo offers a clear ending and another playable run.

## Scope and execution rules

Defer procedural generation, permanent death/missing-person systems, additional
ranks/classes, complex relationships, mentorship, equipment economies, monthly
campaign escalation, cloud saves and world exploration. Authored variation is
enough to test the demo's decisions. Revisit the GDD's broader prototype scope
after feedback demonstrates that this smaller loop is engaging.

Implement phases in order, with each phase leaving a playable build. Treat the
exit gates as go/no-go decisions before expanding scope. Split independently
useful changes into separate validated commits. Run the required publisher after
meaningful changes, and check whether shared runtime/input/rendering needs belong
in `macroquad-toolkit` before creating local alternatives.

If the schedule tightens, reduce contract/report variety and optional feedback
first. Preserve the complete session, competing assignments, recruitment,
recoverable consequences, persistence and touch controls. Do not add a new system
to compensate for a failed pacing or comprehension gate.

**Next gate:** observe a Phase 2 month and three assignment tradeoffs before
expanding into Phase 3. Use the five-minute Phase 1 baseline; do not claim the
full demo duration or force purchased services into the review.
