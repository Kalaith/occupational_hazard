//! Objectives and the saved head-office review, with touch completion controls.
use super::*;

pub fn draw_month(g: &Game) -> Option<UiAction> {
    let w = (screen_width() - 24.0).min(680.0);
    let h = (screen_height() - 24.0).min(660.0);
    let x = (screen_width() - w) / 2.0;
    let y = (screen_height() - h) / 2.0;
    panel(Rect::new(x, y, w, h), PANEL);
    let review = g.guild.month.review.as_ref();
    label(
        if review.is_some() {
            "HEAD-OFFICE REVIEW"
        } else {
            "FIRST MONTH / DAY 30"
        },
        Rect::new(x + 12.0, y + 10.0, w - 24.0, 36.0),
        26.0,
        GOLD,
    );
    let counts = review
        .map(|r| crate::review::ObjectiveCounts {
            certifications: r.certifications,
            commissions: r.commissions,
        })
        .unwrap_or_else(|| g.guild.objective_counts(&g.contracts));
    let services = review
        .map(|r| r.service_returns)
        .unwrap_or(g.guild.board.service_credit.len());
    let target = review
        .map(|r| r.service_target)
        .unwrap_or(crate::review::SERVICE_QUOTA);
    paragraph(
        &format!("Bronze adventurers: {}/1\nSuccessful Bronze commission: {}/1\nService jobs: {services}/{target} / {} days left", counts.certifications, counts.commissions, 30u32.saturating_sub(g.guild.day)),
        Rect::new(x + 16.0, y + 55.0, w - 32.0, 82.0),
        19.0,
        WHITE,
    );
    let body = if let Some(r) = review {
        let result = if r.passed() {
            "TARGETS MET. Head office renews its confidence."
        } else {
            "TARGETS MISSED. The branch needed all targets by the cutoff. Continue to develop the guild or restart."
        };
        let careers = r
            .careers
            .iter()
            .map(|a| {
                format!(
                    "{} / {} / {} XP / {} successes",
                    a.name,
                    a.rank(),
                    a.xp,
                    a.successes
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        format!("{result}\n\nClosing treasury: {}g (opened with 80g; net {:+}g). Renown: {}.\n\n{careers}", r.gold, i64::from(r.gold) - 80, r.reputation)
    } else {
        "Choose one adventurer to earn Bronze: that person needs 60 XP, 3 successes and a solo Lantern Road Trial pass. In Staff, select them and tap APPROVE BRONZE. Rank and XP belong to each person.\n\nComplete A Bridge Worth Keeping with a Bronze leader and 6 different service jobs. Each service job counts once; daily cellar work earns gold and XP only.\n\nDay 30 returns and rewards count before the review. Later returns only count in sandbox. Tap BACK TO HEADQUARTERS to begin.".into()
    };
    paragraph(
        &body,
        Rect::new(x + 16.0, y + 147.0, w - 32.0, h - 325.0),
        19.0,
        MUTED,
    );
    if review.is_some() {
        if button(
            Rect::new(x + 16.0, y + h - 166.0, w - 32.0, 44.0),
            "CONTINUE SANDBOX",
            true,
        ) {
            return Some(UiAction::Sandbox);
        }
        if button(
            Rect::new(x + 16.0, y + h - 114.0, w - 32.0, 44.0),
            "RESTART",
            false,
        ) {
            return Some(UiAction::Restart);
        }
    }
    if button(
        Rect::new(x + 16.0, y + h - 62.0, w - 32.0, 44.0),
        if g.guild.review_pending() {
            "MENU / SAVE"
        } else {
            "BACK TO HEADQUARTERS"
        },
        false,
    ) {
        return Some(if g.guild.review_pending() {
            UiAction::Settings
        } else {
            UiAction::CloseMonth
        });
    }
    None
}
