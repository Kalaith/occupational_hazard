//! Adventurer records and the promotion signature.
use super::*;

pub fn draw_dossier(g: &Game, r: Rect) -> Option<UiAction> {
    let a = &g.guild.roster[g.dossier];
    let mut action = None;
    let tw = (r.w - 12.0) / 3.0;
    for (i, member) in g.guild.roster.iter().enumerate() {
        if button(
            Rect::new(r.x + i as f32 * (tw + 6.0), r.y, tw, 44.0),
            member.name.split(' ').next().unwrap_or("Member"),
            g.dossier == i,
        ) {
            action = Some(UiAction::Dossier(i));
        }
    }
    let wide = r.w > 700.0;
    let size = if wide {
        (r.h - 60.0).min(r.w * 0.35)
    } else {
        64.0
    };
    let art = Rect::new(r.x, r.y + 56.0, size, size);
    portrait(g, ["mira", "tomas", "pip"][g.dossier], art);
    let x = if wide { art.x + size + 24.0 } else { r.x };
    let y = if wide { r.y + 56.0 } else { r.y + 132.0 };
    let width = r.x + r.w - x;
    if !wide {
        label(
            &a.name,
            Rect::new(art.x + size + 12.0, art.y, r.w - size - 12.0, 42.0),
            26.0,
            WHITE,
        );
        label(
            &format!("{} / {}", a.rank(), a.class),
            Rect::new(art.x + size + 12.0, art.y + 44.0, r.w - size - 12.0, 30.0),
            20.0,
            GOLD,
        );
    } else {
        label(&a.name, Rect::new(x, y, width, 44.0), 32.0, WHITE);
    }
    let offset = if wide { 54.0 } else { 0.0 };
    paragraph(&format!("{} / {} / {}\nExperience: {} / 60 XP\nSuccessful contracts: {} / 3\nFatigue: {} / 6  |  Medical leave: {} day(s)",
        a.rank(), a.class, a.trait_name, a.xp, a.successes, a.fatigue, a.injury),
        Rect::new(x, y + offset, width, if wide { 126.0 } else { 84.0 }), 22.0, MUTED);
    let text = if a.bronze {
        "BRONZE CERTIFIED. Lead a party to North Bridge, or help your fellow recruits grow."
    } else if a.trial_passed {
        "ASSESSMENT PASSED. Examiner Vale recommends promotion. Your signature is the final step."
    } else if a.eligible() {
        "ELIGIBLE. Rest, then select The Lantern Road Trial in CONTRACTS. Dispatch this candidate alone."
    } else {
        "IRON LICENCE. Earn 60 XP and three successes, pass The Lantern Road Trial, then approve promotion here."
    };
    paragraph(
        text,
        Rect::new(
            x,
            y + offset + if wide { 138.0 } else { 96.0 },
            width,
            if wide { 78.0 } else { (r.h - 284.0).max(42.0) },
        ),
        20.0,
        GOLD,
    );
    if a.trial_passed
        && !a.bronze
        && button(
            Rect::new(x, r.y + r.h - 48.0, width, 44.0),
            "APPROVE BRONZE",
            true,
        )
    {
        action = Some(UiAction::Promote(g.dossier));
    }
    action
}

pub fn victory(g: &Game) -> Option<UiAction> {
    let w = screen_width();
    let h = screen_height();
    let width = (w - 32.0).min(650.0);
    let x = (w - width) / 2.0;
    let y = (h - 540.0).max(0.0) / 2.0;
    panel(Rect::new(x, y, width, 540.0), PANEL);
    portrait(
        g,
        "elowen",
        Rect::new(w / 2.0 - 72.0, y + 20.0, 144.0, 144.0),
    );
    label(
        "BRONZE, DULY EARNED",
        Rect::new(x + 12.0, y + 182.0, width - 24.0, 50.0),
        34.0,
        GOLD,
    );
    paragraph("The first licence is signed. You sent a recruit into the world and helped them come home a professional. Elowen files the form with a rare, satisfied smile.",
        Rect::new(x + 24.0, y + 250.0, width - 48.0, 118.0), 23.0, WHITE);
    paragraph("First target achieved. Complete A Bridge Worth Keeping by day 30 for the head-office review. Tap BACK TO THE DESK to continue.",
        Rect::new(x + 24.0, y + 380.0, width - 48.0, 70.0), 20.0, MUTED);
    if button(
        Rect::new(x + 24.0, y + 470.0, width - 48.0, 48.0),
        "BACK TO THE DESK",
        true,
    ) {
        return Some(UiAction::CloseVictory);
    }
    None
}
