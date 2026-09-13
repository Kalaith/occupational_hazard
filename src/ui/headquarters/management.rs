use super::*;

pub fn career(g: &Game, r: Rect) -> Option<UiAction> {
    let id = g.dossier;
    let a = &g.guild.roster[id];
    let mut action = None;
    let cw = (r.w - 16.) / 3.;
    for member in 0..g.guild.roster.len() {
        if button(
            Rect::new(r.x + member as f32 * (cw + 8.), r.y, cw, 44.),
            first(g, member),
            member == id,
        ) {
            action = Some(UiAction::Dossier(member));
        }
    }
    let short = r.h < 400.;
    let size = if short { 64. } else { 112. };
    let trial_xp = g.guild.config.progression.trial_xp;
    let trial_successes = g.guild.config.progression.trial_successes;
    let max_fatigue = g.guild.config.caps.max_fatigue;
    portrait(g, key(id), Rect::new(r.x, r.y + 58., size, size));
    text(
        &a.name,
        Rect::new(r.x + size + 14., r.y + 62., r.w - size - 14., 42.),
        23.,
        INK,
    );
    text(
        &format!(
            "{} · {}\n{}",
            a.rank(&g.guild.text),
            a.class,
            activity(&g.guild, id).label(&g.guild.text)
        ),
        Rect::new(r.x + size + 14., r.y + 105., r.w - size - 14., 58.),
        18.,
        GOLD,
    );
    if short && g.hq.page == 0 {
        if primary(
            Rect::new(r.x, r.bottom() - 48., r.w, 46.),
            g.guild.text.get("ui.career_heading"),
            true,
        ) {
            return Some(UiAction::SheetPage(1));
        }
        return action;
    }
    let y = if short { r.y + 52. } else { r.y + 188. };
    if short {
        panel(Rect::new(r.x, y, r.w, r.h - 52.), PANEL);
    }
    let stats = g.guild.text.format(
        "ui.career_stats",
        &[
            ("xp_mark", if a.xp >= trial_xp { "+" } else { "-" }.into()),
            ("xp", a.xp.to_string()),
            ("trial_xp", trial_xp.to_string()),
            (
                "success_mark",
                if a.successes >= trial_successes {
                    "+".into()
                } else {
                    "-".into()
                },
            ),
            ("successes", a.successes.to_string()),
            ("trial_successes", trial_successes.to_string()),
            (
                "trial_mark",
                if a.trial_passed {
                    g.guild.text.get("ui.mark_passed").to_string()
                } else {
                    g.guild.text.get("ui.mark_required").to_string()
                },
            ),
            ("fatigue", a.fatigue.to_string()),
            ("max_fatigue", max_fatigue.to_string()),
            ("injury", a.injury.to_string()),
        ],
    );
    if short {
        text(&stats, Rect::new(r.x, y, r.w, 104.), 19., INK);
    } else {
        theme::parchment(Rect::new(r.x, y, r.w, 116.));
        text(
            &stats,
            Rect::new(r.x + 12., y + 7., r.w - 24., 94.),
            18.,
            theme::PAPER_INK,
        );
        let track_width = (r.w - 36.) / 2.;
        theme::progress(
            Rect::new(r.x + 12., y + 102., track_width, 5.),
            a.xp as f32 / trial_xp as f32,
        );
        theme::progress(
            Rect::new(r.x + 24. + track_width, y + 102., track_width, 5.),
            a.successes as f32 / trial_successes as f32,
        );
    }
    if !short {
        rule(r, y + 128.);
        let career_note = if a.bronze {
            g.guild.text.get("ui.career_bronze").to_string()
        } else if a.trial_passed {
            g.guild.text.get("ui.career_passed").to_string()
        } else if a.eligible(&g.guild.config) {
            g.guild.text.get("ui.career_eligible").to_string()
        } else {
            g.guild.text.format(
                "ui.career_build",
                &[
                    ("trial_xp", trial_xp.to_string()),
                    ("trial_successes", trial_successes.to_string()),
                ],
            )
        };
        text(
            &career_note,
            Rect::new(r.x, y + 143., r.w, (r.bottom() - y - 249.).max(42.)),
            18.,
            GOLD,
        );
        if let Some(report) = g
            .guild
            .reports
            .iter()
            .position(|report| report.body.contains(&a.name) || report.title.contains(&a.name))
        {
            if button(
                Rect::new(r.x, r.bottom() - 104., r.w, 44.),
                g.guild.text.get("ui.latest_report"),
                false,
            ) {
                action = Some(UiAction::Report(report));
            }
        }
    }
    if a.trial_passed && !a.bronze {
        if primary(
            Rect::new(r.x, r.bottom() - 50., r.w, 48.),
            g.guild.text.get("ui.approve_bronze"),
            !g.guild.busy(id),
        ) {
            action = Some(UiAction::Promote(id));
        }
    } else if !a.bronze
        && primary(
            Rect::new(r.x, r.bottom() - 50., r.w, 48.),
            g.guild.text.get("ui.prepare_trial"),
            true,
        )
    {
        action = Some(UiAction::PrepareTrial(id));
    }
    action
}

pub fn facility(g: &Game, r: Rect, room: Room) -> Option<UiAction> {
    let recovery = room == Room::Recovery;
    let purchased = if recovery {
        g.guild.services.infirmary
    } else {
        g.guild.services.training_yard
    };
    let cost = if recovery {
        g.guild.config.services.infirmary_cost
    } else {
        g.guild.config.services.training_yard_cost
    };
    text(
        if recovery {
            g.guild.text.get("ui.recovery_room")
        } else {
            g.guild.text.get("ui.training_yard")
        },
        Rect::new(r.x, r.y, r.w, 40.),
        26.,
        INK,
    );
    let short = r.h < 400.;
    let body = if purchased && recovery {
        g.guild.text.get("ui.facility_infirmary_open")
    } else if purchased {
        &g.guild.text.format(
            "ui.facility_training_open",
            &[
                (
                    "xp_per_day",
                    g.guild.config.services.training_xp_per_day.to_string(),
                ),
                (
                    "xp_cap",
                    g.guild.config.progression.training_xp_cap.to_string(),
                ),
            ],
        )
    } else if short && recovery {
        g.guild.text.get("ui.facility_recovery_short")
    } else if short {
        &g.guild.text.format(
            "ui.facility_training_short",
            &[
                (
                    "xp_per_day",
                    g.guild.config.services.training_xp_per_day.to_string(),
                ),
                (
                    "xp_cap",
                    g.guild.config.progression.training_xp_cap.to_string(),
                ),
            ],
        )
    } else if recovery {
        g.guild.text.get("ui.facility_recovery_long")
    } else {
        &g.guild.text.format(
            "ui.facility_training_long",
            &[
                (
                    "xp_per_day",
                    g.guild.config.services.training_xp_per_day.to_string(),
                ),
                (
                    "xp_cap",
                    g.guild.config.progression.training_xp_cap.to_string(),
                ),
            ],
        )
    };
    text(
        body,
        Rect::new(
            r.x,
            r.y + 50.,
            r.w,
            if short { r.h - 108. } else { r.h - 200. },
        ),
        if short { 18. } else { 21. },
        INK,
    );
    let people = g
        .guild
        .roster
        .iter()
        .enumerate()
        .filter(|(id, _)| activity(&g.guild, *id).room() == Some(room))
        .map(|(_, a)| a.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    if !short {
        text(
            &g.guild.text.format(
                "ui.here_now",
                &[(
                    "people",
                    if people.is_empty() {
                        g.guild.text.get("ui.no_staff").to_string()
                    } else {
                        people
                    },
                )],
            ),
            Rect::new(r.x, r.bottom() - 134., r.w, 65.),
            18.,
            MUTED,
        );
    }
    if primary(
        Rect::new(r.x, r.bottom() - 50., r.w, 48.),
        &if purchased {
            g.guild.text.get("ui.facility_open").into()
        } else {
            format!(
                "{} · {}g",
                if g.guild.gold >= cost {
                    g.guild.text.get("ui.purchase")
                } else {
                    g.guild.text.get("ui.need")
                },
                cost
            )
        },
        !purchased && g.guild.gold >= cost,
    ) {
        return Some(UiAction::Purchase(if recovery {
            crate::services::Purchase::Infirmary
        } else {
            crate::services::Purchase::TrainingYard
        }));
    }
    None
}
