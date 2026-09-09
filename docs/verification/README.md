# Verification record

The current cutaway implementation is documented in [cutaway_comparison.md](cutaway_comparison.md), with per-goal status in [the requirement ledger](../cutaway_requirement_ledger.md).

`ui_*.png` captures show the delivered UI. `ui_browser_landscape.png` is an actual published-browser capture; other `ui_` images use deterministic native fixtures. Phone is 390×844, landscape 844×390, tablet 1024×768 and unprefixed desktop captures 1280×720. Existing `ui_mobile_` paths were refreshed too.

Older `browser_*.png` and `phase2_*.png` files are historical evidence from the prior UI, not current cutaway verification. Their original observations remain in git history. The earlier clean-shell record is superseded: the game now loads compatible guild saves and provides a complete deterministic management loop.

Publishing builds Windows and WebGL, packages all registered assets and updates Preview plus its catalog. The latest no-argument run passed, including Project Roost tracking. See the comparison document for exact live actions, limitations and the outstanding unfamiliar-player protocol.
