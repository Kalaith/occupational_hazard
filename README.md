# Occupational Hazard

A playable fantasy guild-management vertical slice based on [gdd.md](gdd.md).

The [playable demo roadmap](docs/playable_demo_plan.md) sets out six gated phases
from this slice to a complete first-month guild-management demo. Phase 1 is
complete: after initial blockers were fixed, the unfamiliar player reached the
ending and reported about five minutes of gameplay. This introductory slice
does not meet the full roadmap's 30-45 minute target. Phase 2 now adds dated
rotating offers and a six-job service quota; see [verification](docs/phase2_verification.md).
Fresh-player pacing and assignment-choice observation remain to be collected.

## The first Bronze licence

Open NEW GUILD, tap Jobs, select portrait cards and tap DISPATCH PARTY. ADVANCE DAY
advances expeditions and lets people at the guild recover. Fully rested,
unassigned staff trigger a warning: BACK TO HEADQUARTERS preserves the day, while
ADVANCE DAY confirms it. Returning reports
record rewards, experience, fatigue and medical leave in a separate inbox entry
for every expedition. Returns shows the unread count; reading never pays twice.
Accepted offers leave the board. Later postings are new requests, even after a
success or retreat. Expiry is the last day to accept; accepted work can return later.

Every recruit starts at Iron. Earn 60 XP and three successful contracts to qualify
for The Lantern Road Trial. Send one rested candidate alone, wait two days for the
assessment, then open Staff and tap APPROVE BRONZE. Promotion unlocks the
North Bridge commission and an intermediate promotion celebration.

The first month ends with a day-30 head-office review: certify one Bronze
adventurer, complete one Bronze commission, and succeed at six different service
jobs. Repeat successes earn rewards but each service definition counts only once. Tap the Review control
for details. Returns and rewards on day 30 count before evaluation; later
returns wait for sandbox continuation. The review saves its career, objective
and treasury snapshot. CONTINUE SANDBOX resumes play; RESTART opens a
confirmation before replacing the guild. Older slice saves beyond day 30
receive a review of their existing progress on load.

The roster has Mira (fighter), Tomas (ranger) and Pip (healer). Class matches,
experience, rank and fatigue affect expedition outcomes. Healers reduce injuries.
Danger and qualitative readiness replace numerical success probabilities.

Twelve authored contracts provide ten distinct Iron jobs, a candidate-specific
promotion assessment and a Bronze commission. Each class has three suitable
Iron jobs, so promotion does not require repeating completed requests. The
assessment remains available for other candidates until everyone has passed.
Tap Details in Jobs, then Previous job or Next job to browse dated offers.
The board contains at most ten offers with this schedule. Local work rotates weekly;
the bridge accepts on days 10-29 and recurs in sandbox. Cellar work renews daily,
costs nothing to accept, and is safe for any rested recruit. It earns gold/XP but
no service credit. Offer briefs show acceptance dates, return day and review cutoff.
Original six- and twelve-contract saves retain missions, purchases and completions.
Stable definition names resolve saved index caches even when content is reordered;
accepted offers have dated IDs, and scouting carries to the next posting of its route.
Already-filed Phase 1 reviews keep their original targets and result.
This slice uses deterministic resolution and recoverable injuries. Personality
labels are character flavour here; recruitment, death, procedural quests
and relationship simulation remain beyond this milestone.

## Controls and persistence

Services are optional; the review can be passed without purchasing any.
Tap Recovery or Training to purchase facilities; open Jobs > Details for scouting:
the 100g infirmary clears two medical-leave days per day at home, and the 140g
training yard grants fully rested Iron recruits 5 XP daily up to 60 XP. Training
does not grant successful contracts. For 20g, scouts prepare the selected route,
giving its next dispatched party a small capability advantage. Scouting is
consumed only on dispatch and is forbidden for the unaided promotion trial.
Readiness assessments include purchased scouting. Existing ledgers load with
facilities unpurchased; new purchases persist in the autosave.

New guilds receive contextual lessons for selection, DISPATCH PARTY, ADVANCE DAY,
reports, recovery, the solo trial and APPROVE BRONZE. Guidance stays in a small
headquarters panel while the outlined controls remain usable. Following a step advances
it automatically; NEXT TIP advances manually and SKIP TUTORIAL is saved.
MENU > HELP opens the full handbook.

All actions have visible tap/click targets. Phones use a focused planning sheet;
short landscape screens add REVIEW DISPATCH. MENU offers SAVE and RETURN TO
TITLE. Starting a new guild asks before replacing an existing ledger.

The toolkit stores an autosave after dispatch, day progression, promotion and purchases.
CONTINUE restores it, including expeditions in progress. Native saves use the
application data directory; browsers use local storage. Save errors appear in the
headquarters notice. Progress is per browser/device, without cloud sync.

## Development and validation

Keep this folder, macroquad-toolkit and rust_management as siblings. Use the
project's required publishing path:

```powershell
.\publish.ps1
```

Targeted checks are `cargo test -p occupational_hazard` and
`cargo clippy -p occupational_hazard --all-targets -- -D warnings`.
`./scripts/capture_ui.ps1` records deterministic screens in docs/verification.
Captures do not write the player's save.

`src/simulation.rs` owns gameplay, `src/contracts.rs` loads typed authored data
through the toolkit, `src/game.rs` handles actions and persistence, and `src/ui/`
contains responsive views. Artwork is packaged from assets/portraits; generation
prompts and provenance are in [docs/artwork.md](docs/artwork.md).


## Cutaway headquarters redesign

The primary play surface is now a two-storey headquarters with generated room,
character, travel, training and facility artwork. Tap people for careers and rooms
for their work. Jobs, Staff, Returns and Advance Day remain visible shortcuts.
Planning is separate from dispatch. Away staff leave the building; medical leave
and fatigue move people upstairs; equipped training appears in the courtyard.
Menu includes saved reduced-motion and larger-text preferences.

See [design and action map](docs/cutaway_design.md),
[visual comparisons](docs/verification/cutaway_comparison.md), and the
[requirement ledger](docs/cutaway_requirement_ledger.md). The redesign retains the
existing deterministic first-month rules and guild save schema. Recruitment,
relationships, death and procedural contracts remain roadmap work.

Developer frame timing: F3 toggles the shared toolkit overlay. This diagnostic is optional; gameplay remains fully available through visible pointer controls. See docs/verification/cutaway_comparison.md for the measured host and limitations.
