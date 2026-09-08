# Headquarters design contract

## Composition studies and choice (milestone A)

Study A places recovery and records upstairs, common room and assignments below,
with a ground-floor gate and adjacent ground-level yard. Study B puts the yard
above the gate, as in mockup 02. A is selected: it preserves the broad composition
of mockup 01 and makes the bare courtyard and purchased equipment visible without
suggesting that the guild can construct a second floor. Planning and returns use
a slate right-hand sheet occupying roughly 30–35% of desktop width, as in 02/03.

```
A — selected                       B — rejected
RECOVERY | RECORDS | dusk sky       RECOVERY | RECORDS | TRAINING
COMMON   | JOBS    | GATE | YARD    COMMON   | JOBS    | GATE
journey / Jobs / Staff / Returns / Advance Day
```

Typical workday: ready figures downstairs, resting figures upstairs, the next
expedition named in the footer. Everyone away: empty rooms, occupied timeline,
visible Advance Day. Injured return: recovery figures upstairs, people-first
report beside the building. Portrait phone: a cropped room at readable scale,
labelled room controls and Overview; a focused full-width sheet for planning.
Landscape phone: a split view with paged sheet content, never miniature controls.

## Existing action map

| Existing action | Headquarters entry and focused control |
| --- | --- |
| Browse dated offers / pages | Assignments room or Jobs, previous/next offer |
| Inspect job, danger, expiry, reward | Planning sheet with route illustration |
| Add/remove staff, dispatch | Portrait cards, Dispatch Party |
| Scout | Planning preparation, Scout Route / Purchased |
| Inspect careers, trial, Approve Bronze | Person or Staff, career sheet |
| Infirmary / training purchase | Recovery / Training room, facility sheet |
| Active expedition / return reports | Gate, Returns and next journey footer |
| Advance / cancel idle warning | Advance Day, return or advance confirmation |
| Review objectives / results / sandbox | Review reminder, records, review sheet |
| Tutorial, help, save, title, restart | Contextual guidance and Menu |

## Authority and placement

Simulation, board, services, reports and review remain authoritative. Precedence:
away > medical leave > fatigue > rested Iron below 60 XP with purchased training
yard > common room. Roster index assigns a deterministic slot, with six visual
positions reserved per room, without adding recruitment or capacity mechanics.
Planning rings never change this placement. Successful dispatch alone removes
people from home. Presentation transitions are bounded and cannot award rewards.
Day order remains recovery/training, returns and rewards, then review evaluation;
new offers are derived from the resulting day. No animation controls these steps.

## Asset and ownership specification

| Layer | Asset / size | Anchor / use |
| --- | --- | --- |
| Building | headquarters/building.png, 1536×1024 | Full frame; room geometry uses image coordinates |
| People | headquarters/people-keyed.png, 1536×1024 | Explicit atlas crops; feet anchor; standing/resting poses |
| Portraits | Existing four 1254×1254 portraits | Square cards and career/report faces |
| Facilities | Separate generated equipment sprites | Recovery foreground / courtyard; only after purchase |
| Route | Generated landscape vignette | Cover crop within planning sheet |
| Selection | Code-drawn copper rings and name plaques | Above art, below controls |

Built-in imagegen generated the building and people from the supplied mockups
and existing portraits. The generator did not deliver genuine alpha; use a
shared toolkit chroma-key loading facility for the magenta atlas. Keep source
assets unchanged. Furniture in the base painting sits behind runtime figures;
upgrade props and name plaques render separately. Required art loads once through
AssetManager with source-labelled errors. JSON remains in toolkit data_loader.

Toolkit audit: CameraTransform handles projected positions and inverse selection;
Pointer handles mouse/touch and DPI; existing clip, capture and asset utilities
cover rendering and evidence. Guild-specific placement and panel state belong
in headquarters modules. Chroma-key decoding is reusable toolkit work.

## Evidence policy

The requested 70% similarity gate is a visual estimate against the three mockups,
covering composition, architectural art, palette, character scale, and panels.
Record actual captures and differences; do not claim a numerical image metric.
Broader regression testing is deferred until this visual gate, at the user's
request. Required no-argument publishing follows meaningful runtime milestones.
Three unfamiliar-player sessions and physical touch-device performance require
real participants/hardware; automated checks cannot stand in for those findings.

Commit vocabulary: headquarters = presentation; guild ledger = saved state;
assignment = dispatch; return = resolution. Plain-terms tags explain each change.
