# Headquarters feedback revision

## Contract comparison and return visibility

Assignments now opens a paged commission board. Each offer displays its title,
reward, duration, danger and actual acceptance expiry together. Selecting a row
opens planning; < Offers returns to the comparison without losing the party.
The journey card and unread-return control are independent. The journey names
the contract, client/destination, all travellers and return day.

Visual evidence: ui_commissions.png and ui_pending_returns.png (1280x720).
The latter uses an actual completed cellar expedition followed by a beekeeper
expedition: one unread return and Tomas travelling are visible simultaneously.
Further feedback work follows in separate commits.

Validation: no-argument publish.ps1 passed for Windows, WebGL, assets and Preview.

## Readiness, phone hierarchy and integrated art

Member rows now show class match (+3), fatigue penalty and actual individual
strength from Guild::strength. Total readiness uses Guild::prepared_strength;
scouting adds +2 once to the whole party. Advice identifies an available member's
extra contribution or the strength restored by full rest. Travel inspection uses
the expedition's stored strength rather than recalculating a new party. Checkmarks
mark selection. Only Dispatch is a filled copper action; scouting is secondary.

Phones use Overview and a compact Rooms selector, with readable roster status
cards and a direct Compare commissions control. Contract prose and party decisions
occupy separate full-height pages. Short landscape uses a third readiness page.
The artwork viewport crops rather than stretching when decision space is needed.

Twelve authored destination vignettes map by stable contract ID, including a well,
cellar, marsh, apiary, lantern road, bridge, shutters, quarry, boundary stones,
shepherd pasture, millhouse and watchtower. Resting characters now recline inside
complete beds, with pillows, blankets and overlapping footboards. Floor beams and
the desk apron draw in front of occupants; room light tones match the environment.

Screenshots include ui_phone_commissions, ui_phone_planning, ui_phone_party_readiness,
ui_phone_pending_returns, ui_landscape_party_readiness, ui_landscape_dispatch_readiness,
ui_gameplay, ui_planning and ui_facilities. Phone checks cover 390x844 and 390x667;
landscape is 844x390. Existing tests: 34 passed. Strict Clippy: passed.

Published-browser verification used http://127.0.0.1/games/occupational_hazard/:
Continue, Assignments, commission comparison, well offer, Mira selection, phone
Choose Party, Offers, Guild and Rooms all worked through visible pointer targets.
The well illustration, selected checkmark, contribution 16 versus difficulty 12,
secondary scouting and dominant Dispatch were inspected at 390x844. The desktop
preview was restored afterward. Saved day 16 and 222 gold were unchanged; no
expedition was dispatched and no scouting was purchased during this check.

Final no-argument publish.ps1 passed Windows and WebGL builds, all 12 registered
assets, Preview deployment and catalog update. Full-month playthrough was not
repeated: verification concentrated on the revised visual decisions and controls.
