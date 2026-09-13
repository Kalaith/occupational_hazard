# Phase 2 verification - 2026-09-04

Implementation complete; unfamiliar-player assignment observations and new session
length remain unmeasured. This does not establish 30-45 minute pacing or a green
light to expand Phase 3 without observing the new month.

## Implemented rules

Twelve authored definitions have stable names. Offers derive stable dated IDs
from arrival, interval and inclusive acceptance window. Local work rotates every
seven days; cellar work renews daily; the bridge accepts days 10-29 and recurs in
sandbox. The trial remains available to candidates. At most ten offers appear
under this authored schedule. Accepted offers leave immediately, including failed
attempts; another attempt requires a new posting. Accepted missions may return
after their acceptance window closes. Only returns through day 30 affect review.

Six different service definitions must succeed. A set of credited definition IDs
makes repeated postings ineligible for extra quota credit. They still pay their
normal rewards once. Daily cellar work costs nothing and is safe for every rested
class, but cannot satisfy the service quota. Scout preparation stays with its
named route until dispatch, including across expiry. Existing facilities keep
their 100g/140g prices and scouting costs 20g.

Save dictionaries name every index cache; loading remaps completions, scouts and
expeditions by definition ID rather than current content order. Dated acceptance
IDs survive reload. Original six-slot and twelve-slot saves migrate; unknown
identities fail visibly rather than resetting. Existing filed Phase 1 reviews
retain their original zero service target and historical result.

## Technical evidence

- `cargo test --quiet`: 28 tests passed (26 unit, two integration).
- `cargo clippy --all-targets -- -D warnings`: passed.
- No-argument `publish.ps1`: Windows and WebGL release builds, packaging and WSL
  preview deployment passed. This is the required validation path.
- Source-size integration gate covers every Rust file; all are below 800 lines.
- Regressions cover final-day acceptance and later return, no early or duplicate
  rewards, repeat service credit, board regeneration after reload, reordered
  definitions, legacy away missions/purchases/completions, unknown identities,
  scouting across expiry, zero-gold recovery, persistent assessment availability,
  old review preservation, success/missed reviews and sandbox continuation.

## Scripted month decisions and economy

The full-month schedule regression makes three explicit choices with normal
simulation actions (no invented XP, gold or ranks):

1. Day 1: send the three recruits on separate cellar/medicine/beekeeper work
   rather than committing two people to the 65g well. Tomas returns injured from
   his close call; the split develops all three people.
2. Day 4: rest injured Tomas, let the first well posting lapse, and send Mira/Pip
   on shutters/fever. Tomas recovers in time for Boundary Stones on day 5.
3. Day 10: choose Mira's trial and Tomas's shepherd request rather than putting
   Mira and Pip on the last-day well posting. Rest before the Bronze commission.

That route returns eight distinct services, certifies Mira, completes the bridge
and closes at 582g. The separate cutoff regression completes six services and
returns the bridge exactly on day 30, closing at 510g. Neither needs purchases.
At current prices, the 582g route leaves 342g after reserving both facilities,
or 582g for future hiring. This establishes room for a Phase 3 hiring decision,
not balanced recruitment: fees and the timing/value of hires are not implemented.
A player still needs to demonstrate that the tradeoffs are noticeable and useful;
scripted choices alone cannot establish engagement or session length.

## Published browser check

Actual clicks at 360x640 loaded the prior day-3, 134g ledger. Beekeeper showed
acceptance through day 3 and return on day 5. Tomas alone showed a close call;
adding Pip showed well prepared. Dispatch claimed the offer, and advancing to
day 4 removed the expired well while retaining the travelling expedition.
Reload/CONTINUE preserved day 4, 134g and one expedition away. Advancing through
the visible idle warning returned Beekeeper on day 5 and paid exactly 48g (182g).
The objective panel showed two distinct service credits, including the migrated
medicine success, and 25 days remaining. No keyboard controls were used.

Evidence: `verification/ui_phone_party_readiness.png`,
`verification/ui_phone_pending_returns.png`, `verification/ui_objectives.png`.
Browser clicks are not physical-touch testing or an unfamiliar-player study.

Desktop 1280x720 inspection also confirmed readable contract timing, roster and
dispatch controls (verification/ui_gameplay.png). The final title capture replaces
verification/ui_title.png and catalog_thumbnail.png.

