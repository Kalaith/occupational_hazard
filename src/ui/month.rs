//! Objectives and the saved head-office review, with touch completion controls.
use super::*;

pub fn draw_month(g: &Game) -> Option<UiAction> {
    let w = (screen_width() - 24.0).min(680.0);
    let h = (screen_height() - 24.0).min(660.0);
    let x = (screen_width() - w) / 2.0;
    let y = (screen_height() - h) / 2.0;
    panel(Rect::new(x, y, w, h), PANEL);
    let review = g.guild.month.review.as_ref();
    let heading = if review.is_some() {
        g.guild.text.get("ui.review_heading").to_string()
    } else {
        g.guild.text.format(
            "ui.review_pending_heading",
            &[("day", g.guild.config.review.cutoff_day.to_string())],
        )
    };
    let top_action = Rect::new(x + w - 164., y + 10., 148., 36.);
    label(
        &heading,
        Rect::new(x + 12.0, y + 10.0, w - 184.0, 36.0),
        26.0,
        GOLD,
    );
    if review.is_some()
        && button(
            top_action,
            if g.review_details {
                g.guild.text.get("ui.review_result")
            } else {
                g.guild.text.get("ui.review_details")
            },
            false,
        )
    {
        return Some(UiAction::ReviewDetails);
    }
    if g.guild.review_pending() && button(top_action, g.guild.text.get("ui.menu_save"), false) {
        return Some(UiAction::Settings);
    }
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
        .unwrap_or(g.guild.config.review.service_quota);
    paragraph(
        &g.guild.text.format(
            "ui.review_targets",
            &[
                ("certifications", counts.certifications.to_string()),
                ("commissions", counts.commissions.to_string()),
                ("services", services.to_string()),
                ("target", target.to_string()),
                (
                    "days",
                    g.guild
                        .config
                        .review
                        .cutoff_day
                        .saturating_sub(g.guild.day)
                        .to_string(),
                ),
            ],
        ),
        Rect::new(x + 16.0, y + 55.0, w - 32.0, 82.0),
        19.0,
        WHITE,
    );
    let body = if let Some(r) = review {
        let result = if r.passed() {
            g.guild.text.get("ui.review_passed").to_string()
        } else {
            g.guild.text.get("ui.review_failed").to_string()
        };
        let careers = r
            .careers
            .iter()
            .map(|a| {
                g.guild.text.format(
                    "ui.review_careers",
                    &[
                        ("name", a.name.clone()),
                        ("rank", a.rank(&g.guild.text).to_string()),
                        ("xp", a.xp.to_string()),
                        ("successes", a.successes.to_string()),
                    ],
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        if g.review_details {
            format!(
                "{}\n\n{careers}",
                g.guild.text.format(
                    "ui.review_closing",
                    &[
                        ("gold", r.gold.to_string()),
                        ("starting_gold", g.guild.config.starting.gold.to_string()),
                        (
                            "net",
                            format!(
                                "{:+}",
                                i64::from(r.gold) - i64::from(g.guild.config.starting.gold)
                            ),
                        ),
                        ("reputation", r.reputation.to_string()),
                    ],
                )
            )
        } else {
            format!("{result}\n\n{}", g.guild.text.get("ui.review_details_hint"))
        }
    } else {
        g.guild.text.format(
            "ui.review_open_body",
            &[
                ("trial_xp", g.guild.config.progression.trial_xp.to_string()),
                (
                    "trial_successes",
                    g.guild.config.progression.trial_successes.to_string(),
                ),
                (
                    "service_quota",
                    g.guild.config.review.service_quota.to_string(),
                ),
                ("cutoff_day", g.guild.config.review.cutoff_day.to_string()),
            ],
        )
    };
    paragraph(
        &body,
        Rect::new(
            x + 16.0,
            y + 147.0,
            w - 32.0,
            if g.review_details {
                h - 220.0
            } else {
                h - 325.0
            },
        ),
        19.0,
        MUTED,
    );
    if review.is_some() && g.review_details {
        if button(
            Rect::new(x + 16.0, y + h - 62.0, w - 32.0, 44.0),
            g.guild.text.get("ui.review_result"),
            false,
        ) {
            return Some(UiAction::ReviewDetails);
        }
        return None;
    }
    if review.is_some() {
        let action_shift = if g.guild.review_pending() { 52. } else { 0. };
        if button(
            Rect::new(x + 16.0, y + h - 166.0 + action_shift, w - 32.0, 44.0),
            g.guild.text.get("ui.continue_sandbox"),
            true,
        ) {
            return Some(UiAction::Sandbox);
        }
        if button(
            Rect::new(x + 16.0, y + h - 114.0 + action_shift, w - 32.0, 44.0),
            g.guild.text.get("ui.restart"),
            false,
        ) {
            return Some(UiAction::Restart);
        }
    }
    if !g.guild.review_pending()
        && button(
            Rect::new(x + 16.0, y + h - 62.0, w - 32.0, 44.0),
            g.guild.text.get("ui.back_headquarters"),
            false,
        )
    {
        return Some(UiAction::CloseMonth);
    }
    None
}
