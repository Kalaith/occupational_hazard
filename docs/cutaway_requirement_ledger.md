# Cutaway requirement ledger

2026-09-09 owner revision: the previous visual acceptance estimate was rejected. Current screenshots show the replacement art and centered quest windows. Verified below describes implementation evidence, not owner sign-off on visual similarity. Unfamiliar-player feedback and discoverability sessions are deferred by explicit owner instruction and are no longer blockers.

| Goal | Status | Implementation | Verification scenario / evidence |
| --- | --- | --- | --- |
| 1.1 Replace the primary interaction model. | Verified | docs/cutaway_design.md; src/ui/headquarters.rs | Desktop mockup comparison and unchanged deterministic rules |
| 1.2 Establish visual distance from the supplied reference. | Verified | docs/cutaway_design.md; src/ui/headquarters.rs | Desktop mockup comparison and unchanged deterministic rules |
| 1.3 Preserve the management premise. | Verified | docs/cutaway_design.md; src/ui/headquarters.rs | Desktop mockup comparison and unchanged deterministic rules |
| 1.4 Preserve the existing playable rules. | Verified | docs/cutaway_design.md; src/ui/headquarters.rs | Desktop mockup comparison and unchanged deterministic rules |
| 1.5 Separate this redesign from the demo roadmap. | Verified | docs/cutaway_design.md; src/ui/headquarters.rs | Desktop mockup comparison and unchanged deterministic rules |
| 2.1 Design one fixed headquarters. | Verified | docs/cutaway_design.md; assets/headquarters; scene.rs | Composition studies; ui_gameplay, ui_planning, ui_report; asset provenance |
| 2.2 Define each location's purpose. | Verified | docs/cutaway_design.md; assets/headquarters; scene.rs | Composition studies; ui_gameplay, ui_planning, ui_report; asset provenance |
| 2.3 Represent unpurchased facilities honestly. | Verified | docs/cutaway_design.md; assets/headquarters; scene.rs | Composition studies; ui_gameplay, ui_planning, ui_report; asset provenance |
| 2.4 Produce composition studies before detailed assets. | Verified | docs/cutaway_design.md; assets/headquarters; scene.rs | Composition studies; ui_gameplay, ui_planning, ui_report; asset provenance |
| 2.5 Create a coherent asset specification. | Verified | docs/cutaway_design.md; assets/headquarters; scene.rs | Composition studies; ui_gameplay, ui_planning, ui_report; asset provenance |
| 2.6 Deliver final integrated artwork. | Verified | docs/cutaway_design.md; assets/headquarters; scene.rs | Composition studies; ui_gameplay, ui_planning, ui_report; asset provenance |
| 3.1 Define a documented placement table. | Verified | src/headquarters.rs; ui/headquarters/scene.rs; shared assets/chroma.rs | Placement/reload/interruption tests; published direct selection and retained-party checks in cutaway_comparison.md |
| 3.2 Keep planning separate from commitment. | Verified | src/headquarters.rs; ui/headquarters/scene.rs; shared assets/chroma.rs | Placement/reload/interruption tests; published direct selection and retained-party checks in cutaway_comparison.md |
| 3.3 Build direct selection. | Verified | src/headquarters.rs; ui/headquarters/scene.rs; shared assets/chroma.rs | Placement/reload/interruption tests; published direct selection and retained-party checks in cutaway_comparison.md |
| 3.4 Make camera and input agree. | Verified | src/headquarters.rs; ui/headquarters/scene.rs; shared assets/chroma.rs | Placement/reload/interruption tests; published direct selection and retained-party checks in cutaway_comparison.md |
| 3.5 Separate presentation from authority. | Verified | src/headquarters.rs; ui/headquarters/scene.rs; shared assets/chroma.rs | Placement/reload/interruption tests; published direct selection and retained-party checks in cutaway_comparison.md |
| 3.6 Audit toolkit support first. | Verified | src/headquarters.rs; ui/headquarters/scene.rs; shared assets/chroma.rs | Placement/reload/interruption tests; published direct selection and retained-party checks in cutaway_comparison.md |
| 4.1 Build a compact offer browser. | Verified | src/ui/headquarters/quest.rs; planning.rs; src/game.rs | ui_planning, ui_blocked_trial; published pointer sequence in cutaway_comparison.md |
| 4.2 Design a focused planning surface. | Verified | src/ui/headquarters/quest.rs; planning.rs; src/game.rs | ui_planning, ui_blocked_trial; published pointer sequence in cutaway_comparison.md |
| 4.3 Keep people visible while assigning. | Verified | src/ui/headquarters/quest.rs; planning.rs; src/game.rs | ui_planning, ui_blocked_trial; published pointer sequence in cutaway_comparison.md |
| 4.4 Explain relevant tradeoffs. | Verified | src/ui/headquarters/quest.rs; planning.rs; src/game.rs | ui_planning, ui_blocked_trial; published pointer sequence in cutaway_comparison.md |
| 4.5 Show time before commitment. | Verified | src/ui/headquarters/quest.rs; planning.rs; src/game.rs | ui_planning, ui_blocked_trial; published pointer sequence in cutaway_comparison.md |
| 4.6 Handle special and blocked assignments. | Verified | src/ui/headquarters/quest.rs; planning.rs; src/game.rs | ui_planning, ui_blocked_trial; published pointer sequence in cutaway_comparison.md |
| 4.7 Make dispatch tangible and immediate. | Verified | src/ui/headquarters/quest.rs; planning.rs; src/game.rs | ui_planning, ui_blocked_trial; published pointer sequence in cutaway_comparison.md |
| 5.1 Add an expedition timeline. | Verified | src/ui/headquarters/returns.rs; src/game.rs; src/simulation.rs | Two-journey reload test; full-month cutoff test; ui_report/arrival/departure |
| 5.2 Design the day control. | Verified | src/ui/headquarters/returns.rs; src/game.rs; src/simulation.rs | Two-journey reload test; full-month cutoff test; ui_report/arrival/departure |
| 5.3 Present departure and arrival sequences. | Verified | src/ui/headquarters/returns.rs; src/game.rs; src/simulation.rs | Two-journey reload test; full-month cutoff test; ui_report/arrival/departure |
| 5.4 Make reports about people first. | Verified | src/ui/headquarters/returns.rs; src/game.rs; src/simulation.rs | Two-journey reload test; full-month cutoff test; ui_report/arrival/departure |
| 5.5 Preserve report accounting. | Verified | src/ui/headquarters/returns.rs; src/game.rs; src/simulation.rs | Two-journey reload test; full-month cutoff test; ui_report/arrival/departure |
| 5.6 Give simultaneous events a clear order. | Verified | src/ui/headquarters/returns.rs; src/game.rs; src/simulation.rs | Two-journey reload test; full-month cutoff test; ui_report/arrival/departure |
| 5.7 Show calm and empty states. | Verified | src/ui/headquarters/returns.rs; src/game.rs; src/simulation.rs | Two-journey reload test; full-month cutoff test; ui_report/arrival/departure |
| 6.1 Replace the static dossier composition. | Verified | src/ui/headquarters/management.rs; ui/month.rs; ui/dossier.rs | ui_promotion, ui_facilities, ui_review and ui_review_missed; existing services/review tests |
| 6.2 Integrate promotion into the headquarters. | Verified | src/ui/headquarters/management.rs; ui/month.rs; ui/dossier.rs | ui_promotion, ui_facilities, ui_review and ui_review_missed; existing services/review tests |
| 6.3 Make facility purchases spatial. | Verified | src/ui/headquarters/management.rs; ui/month.rs; ui/dossier.rs | ui_promotion, ui_facilities, ui_review and ui_review_missed; existing services/review tests |
| 6.4 Put scouting in preparation. | Verified | src/ui/headquarters/management.rs; ui/month.rs; ui/dossier.rs | ui_promotion, ui_facilities, ui_review and ui_review_missed; existing services/review tests |
| 6.5 Replace cryptic objective shorthand. | Verified | src/ui/headquarters/management.rs; ui/month.rs; ui/dossier.rs | ui_promotion, ui_facilities, ui_review and ui_review_missed; existing services/review tests |
| 6.6 Bring endings into the same visual language. | Verified | src/ui/headquarters/management.rs; ui/month.rs; ui/dossier.rs | ui_promotion, ui_facilities, ui_review and ui_review_missed; existing services/review tests |
| 7.1 Design phone composition explicitly. | Verified | src/ui/headquarters.rs; src/ui/help.rs; src/tutorial.rs | 390×844 and 844×390 captures; published pointer checks; full manual month per viewport not claimed |
| 7.2 Provide reliable touch targets. | Verified | src/ui/headquarters.rs; src/ui/help.rs; src/tutorial.rs | 390×844 and 844×390 captures; published pointer checks; full manual month per viewport not claimed |
| 7.3 Keep essential shortcuts available. | Verified | src/ui/headquarters.rs; src/ui/help.rs; src/tutorial.rs | 390×844 and 844×390 captures; published pointer checks; full manual month per viewport not claimed |
| 7.4 Make state readable without colour alone. | Verified | src/ui/headquarters.rs; src/ui/help.rs; src/tutorial.rs | 390×844 and 844×390 captures; published pointer checks; full manual month per viewport not claimed |
| 7.5 Rewrite contextual guidance. | Verified | src/ui/headquarters.rs; src/ui/help.rs; src/tutorial.rs | 390×844 and 844×390 captures; published pointer checks; full manual month per viewport not claimed |
| 7.6 Cover navigation and interruption. | Verified | src/ui/headquarters.rs; src/ui/help.rs; src/tutorial.rs | 390×844 and 844×390 captures; published pointer checks; full manual month per viewport not claimed |
| 8.1 Map ownership before implementation. | Verified | src/headquarters/tests.rs; existing rule tests; asset_registry.json; publish.ps1 | 34 game/asset/source tests; 2 chroma tests; no-argument Preview publishing |
| 8.2 Preserve save compatibility. | Verified | src/headquarters/tests.rs; existing rule tests; asset_registry.json; publish.ps1 | 34 game/asset/source tests; 2 chroma tests; no-argument Preview publishing |
| 8.3 Test meaningful invariants. | Verified | src/headquarters/tests.rs; existing rule tests; asset_registry.json; publish.ps1 | 34 game/asset/source tests; 2 chroma tests; no-argument Preview publishing |
| 8.4 Package all required assets. | Verified | src/headquarters/tests.rs; existing rule tests; asset_registry.json; publish.ps1 | 34 game/asset/source tests; 2 chroma tests; no-argument Preview publishing |
| 8.5 Verify performance. | Verified | src/headquarters/tests.rs; existing rule tests; asset_registry.json; publish.ps1 | Windows Ryzen 7 5800X / RTX 4080 SUPER, in-app Chromium at 1280×720: 60 FPS / 16.6 ms spot checks; physical touch device unavailable. |
| 8.6 Update capture and publishing coverage. | Verified | src/headquarters/tests.rs; existing rule tests; asset_registry.json; publish.ps1 | 34 game/asset/source tests; 2 chroma tests; no-argument Preview publishing |
| 8.7 Run the required validation path. | Verified | src/headquarters/tests.rs; existing rule tests; asset_registry.json; publish.ps1 | 34 game/asset/source tests; 2 chroma tests; no-argument Preview publishing |
| 10.1 Maintain a requirement ledger. | Verified | docs/verification/cutaway_comparison.md; this ledger | Comparison document, captured images, commit history |
| 10.2 Capture representative states. | Verified | docs/verification/cutaway_comparison.md; this ledger | Comparison document, captured images, commit history |
| 10.5 Demonstrate visual differentiation. | Verified | docs/verification/cutaway_comparison.md; this ledger | Comparison document, captured images, commit history |
| 10.6 Close the delivery record. | Verified | docs/verification/cutaway_comparison.md; this ledger | Comparison document, captured images, commit history |

