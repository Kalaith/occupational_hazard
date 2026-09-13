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
            a.rank(),
            a.class,
            activity(&g.guild, id).label()
        ),
        Rect::new(r.x + size + 14., r.y + 105., r.w - size - 14., 58.),
        18.,
        GOLD,
    );
    if short && g.hq.page == 0 {
        if primary(
            Rect::new(r.x, r.bottom() - 48., r.w, 46.),
            "CAREER & PROMOTION",
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
    let stats = format!("{}  Experience: {} / {} XP\n{}  Successful contracts: {} / {}\n{}  Unaided assessment\nFatigue {}/{} · Medical leave {} day(s)", if a.xp >= trial_xp { "+" } else { "-" }, a.xp, trial_xp, if a.successes >= trial_successes { "+" } else { "-" }, a.successes, trial_successes, if a.trial_passed { "+ Passed" } else { "- Required" }, a.fatigue, max_fatigue, a.injury);
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
        text(
            if a.bronze {
                "BRONZE CERTIFIED\nThis adventurer can lead Bronze commissions."
            } else if a.trial_passed {
                "Assessment passed. Sign the licence below when the candidate is home."
            } else if a.eligible(&g.guild.config) {
                "Eligible for the solo trial. Rest first, then send this candidate unaided."
            } else {
                "Build this person's experience and successful-contract record. The trial unlocks at 60 XP and 3 successes."
            },
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
                "Latest personal report",
                false,
            ) {
                action = Some(UiAction::Report(report));
            }
        }
    }
    if a.trial_passed && !a.bronze {
        if primary(
            Rect::new(r.x, r.bottom() - 50., r.w, 48.),
            "APPROVE BRONZE",
            !g.guild.busy(id),
        ) {
            action = Some(UiAction::Promote(id));
        }
    } else if !a.bronze
        && primary(
            Rect::new(r.x, r.bottom() - 50., r.w, 48.),
            "PREPARE SOLO TRIAL",
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
            "RECOVERY ROOM"
        } else {
            "TRAINING YARD"
        },
        Rect::new(r.x, r.y, r.w, 40.),
        26.,
        INK,
    );
    let short = r.h < 400.;
    let body = if purchased && recovery {
        "Infirmary open. Medical leave recovers twice as quickly. Basic fatigue recovery continues each day at home. Beds have no capacity limit."
    } else if purchased {
        "Training yard open. Fully rested Iron staff at home gain 5 XP per day, up to 60 XP. Training never awards successful-contract credit."
    } else if short && recovery {
        "Basic rest is free. An infirmary doubles medical-leave recovery. It adds equipment, with no bed limit."
    } else if short {
        "Equip the yard: rested Iron staff gain 5 XP/day, capped at 60 XP. No successful-contract credit."
    } else if recovery {
        "Basic rest is always available. People at home recover fatigue and medical leave when you advance a day.\n\nThe infirmary adds medical equipment and doubles medical-leave recovery. Beds are scenery, with no capacity limit."
    } else {
        "The unequipped courtyard grants no training.\n\nEquip the yard for fully rested Iron recruits to gain 5 XP per day at home, up to 60 XP. Training never awards successful-contract credit."
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
            &format!(
                "Here now: {}",
                if people.is_empty() {
                    "No staff"
                } else {
                    &people
                }
            ),
            Rect::new(r.x, r.bottom() - 134., r.w, 65.),
            18.,
            MUTED,
        );
    }
    if primary(
        Rect::new(r.x, r.bottom() - 50., r.w, 48.),
        &if purchased {
            "FACILITY OPEN".into()
        } else {
            format!(
                "{} · {}g",
                if g.guild.gold >= cost {
                    "PURCHASE"
                } else {
                    "NEED"
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
