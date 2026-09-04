# Phase 1 verification

## First-month review

2026-09-04: `cargo test -p occupational_hazard` passed 15 tests, including
the established class promotion paths and four new review regressions. A fresh
guild earns Bronze and a commission returning exactly on day 30; its rewards
and career advances are included before review. Failed and late-work paths
reload, continue, and pay once without changing the frozen review. Legacy
saves without month state load and receive a review if past the cutoff.

No-argument `./publish.ps1` passed Windows and WebGL release builds, packaging
and deployment to the configured WSL preview. Review and objective screenshots
at 360x640 were visually checked in `docs/verification/`.

Onboarding and actual browser input validation are pending the next change.
The roadmap's unfamiliar-player observation and measured session-duration
tuning require a human participant and have not been performed.
