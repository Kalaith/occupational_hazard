# Occupational Hazard

A playable fantasy guild-management vertical slice based on [gdd.md](gdd.md).

The [playable demo roadmap](docs/playable_demo_plan.md) sets out six gated phases
from this slice to a complete first-month guild-management demo.

## The first Bronze licence

Open NEW GUILD, review a contract, select adventurers and tap DISPATCH. NEXT DAY
advances expeditions and lets people at the guild recover. Returning reports
record rewards, experience, fatigue and medical leave. Contracts can be repeated.

Every recruit starts at Iron. Earn 60 XP and three successful contracts to qualify
for The Lantern Road Trial. Send one rested candidate alone, wait two days for the
assessment, then open ADVENTURERS and tap APPROVE BRONZE. Promotion unlocks the
North Bridge commission and a completion celebration; you can keep playing.

The roster has Mira (fighter), Tomas (ranger) and Pip (healer). Class matches,
experience, rank and fatigue affect expedition outcomes. Healers reduce injuries.
Danger and qualitative readiness replace numerical success probabilities.

Six authored contracts cover extermination, medicine delivery, search and rescue,
an uncertain investigation, a promotion trial, and a Bronze defence commission.
This slice uses deterministic resolution and recoverable injuries. Personality
labels are character flavour here; recruitment, death, quotas, procedural quests
and relationship simulation remain beyond this milestone.

## Controls and persistence

GUILD SERVICES spends contract earnings on permanent facilities and preparation:
the 100g infirmary clears two medical-leave days per day at home, and the 140g
training yard grants fully rested Iron recruits 5 XP daily up to 60 XP. Training
does not grant successful contracts. For 20g, scouts prepare the selected route,
giving its next dispatched party a small capability advantage. Scouting is
consumed only on dispatch and is forbidden for the unaided promotion trial.
Readiness assessments include purchased scouting. Existing ledgers load with
facilities unpurchased; new purchases persist in the autosave.

All actions have visible tap/click targets. Smaller windows use CHOOSE PARTY and
BACK to separate contract reading from dispatch. MENU offers SAVE and RETURN TO
TITLE. Starting a new guild asks before replacing an existing ledger.

The toolkit stores an autosave after dispatch, day progression, promotion and purchases.
CONTINUE restores it, including expeditions in progress. Native saves use the
application data directory; browsers use local storage. Save errors appear in the
receptionist's notice. Progress is per browser/device, without cloud sync.

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
