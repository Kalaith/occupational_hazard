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
can now begin. Responsive captures and actual published-browser input remain open.

The first capture attempt caught an invalid dated-offer fixture on day 12. The
fixture now uses day 9, when the expedition is legally dispatchable. No gameplay
availability rule was relaxed for the screenshot. A later pass moves recovery
person labels below their feet to avoid colliding with room labels.

The old comparison images remain available in git history at b1b3b1e and earlier.
The former UI used three broad tabs, parchment contracts, text-row party selection
and a permanent Elowen guidance strip. The new composition changes all four.
