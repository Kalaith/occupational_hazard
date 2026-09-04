use super::*;
use crate::tutorial::Lesson;
use crate::tutorial::LESSONS;

/// Guidance occupies its own desk panel; every game control remains interactive.
pub fn draw_guidance(g: &Game, r: Rect) -> Option<UiAction> {
    let lesson = g.lesson()?;
    panel(r, PANEL);
    paragraph(
        tip(g, lesson),
        Rect::new(r.x + 10.0, r.y + 6.0, r.w - 20.0, 58.0),
        18.0,
        GOLD,
    );
    let width = (r.w - 30.0) / 2.0;
    if button(
        Rect::new(r.x + 10.0, r.y + 66.0, width, 44.0),
        "NEXT TIP",
        false,
    ) {
        return Some(UiAction::LessonDone(lesson));
    }
    if button(
        Rect::new(r.x + width + 20.0, r.y + 66.0, width, 44.0),
        "SKIP TUTORIAL",
        false,
    ) {
        return Some(UiAction::SkipTutorial);
    }
    None
}

fn tip(g: &Game, lesson: Lesson) -> &'static str {
    match lesson {
        Lesson::Welcome => "By day 30: promote one adventurer to Bronze; complete a Bronze commission. Tap D30 for details.",
        Lesson::Selection if g.tab != 0 => "Tap CONTRACTS to choose an assignment. The outlined controls show the next step.",
        Lesson::Selection if g.choosing_party => "Tap Mira Ashford for the cellar job. Gold names join your party; tap again to remove.",
        Lesson::Selection => "Browse contracts. Tap CHOOSE PARTY, then an adventurer; gold names join the party.",
        Lesson::Dispatch if g.tab != 0 => "Tap CONTRACTS to review your selected party and send the expedition.",
        Lesson::Dispatch => "Check readiness, then tap DISPATCH. Every control remains available while this tip is open.",
        Lesson::Time => "Tap NEXT DAY to advance journeys. People at home recover while others travel.",
        Lesson::Reports => "Tap REPORTS to read every return. Each expedition has its own report and automatic rewards.",
        Lesson::Recovery => "Leave tired or injured people at home and tap NEXT DAY. Typical fatigue clears in two days.",
        Lesson::Trial => "Rest the eligible candidate. In CONTRACTS select The Lantern Road Trial; DISPATCH that person alone.",
        Lesson::Promotion => "Tap ADVENTURERS, select the passed candidate, then APPROVE BRONZE to promote that person.",
    }
}

pub fn is_target(g: &Game, text: &str) -> bool {
    match g.lesson() {
        Some(Lesson::Welcome) => text.starts_with("D30:"),
        Some(Lesson::Selection) if g.tab != 0 => text == "CONTRACTS",
        Some(Lesson::Selection) => text == "CHOOSE PARTY" || text.contains("Mira Ashford"),
        Some(Lesson::Dispatch) if g.tab != 0 => text == "CONTRACTS",
        Some(Lesson::Dispatch) => text == "DISPATCH" || text == "CHOOSE PARTY",
        Some(Lesson::Time | Lesson::Recovery) => text == "NEXT DAY",
        Some(Lesson::Reports) => text.starts_with("REPORTS"),
        Some(Lesson::Trial) => text == "CONTRACTS" || text.contains("The Lantern Road Trial"),
        Some(Lesson::Promotion) => text == "ADVENTURERS" || text == "APPROVE BRONZE",
        _ => false,
    }
}

pub fn draw_help(g: &Game) -> Option<UiAction> {
    let lesson = if let Some(page) = g.help_page {
        LESSONS[page % LESSONS.len()]
    } else {
        g.lesson()?
    };
    let w = (screen_width() - 24.0).min(620.0);
    let h = (screen_height() - 24.0).min(570.0);
    let x = (screen_width() - w) / 2.0;
    let y = (screen_height() - h) / 2.0;
    panel(Rect::new(x, y, w, h), PANEL);
    label(
        lesson.title(),
        Rect::new(x + 12.0, y + 12.0, w - 24.0, 44.0),
        24.0,
        GOLD,
    );
    paragraph(
        lesson.text(screen_width() < 874.0 || screen_height() < 674.0),
        Rect::new(x + 20.0, y + 78.0, w - 40.0, h - 222.0),
        22.0,
        WHITE,
    );
    if let Some(page) = g.help_page {
        if button(
            Rect::new(x + 20.0, y + h - 124.0, w - 40.0, 46.0),
            &format!("NEXT HELP / {}/{}", page + 1, LESSONS.len()),
            false,
        ) {
            return Some(UiAction::Help((page + 1) % LESSONS.len()));
        }
    } else if button(
        Rect::new(x + 20.0, y + h - 124.0, w - 40.0, 46.0),
        "SKIP TUTORIAL",
        false,
    ) {
        return Some(UiAction::SkipTutorial);
    }
    if button(
        Rect::new(x + 20.0, y + h - 66.0, w - 40.0, 46.0),
        "BACK TO DESK",
        true,
    ) {
        return Some(if g.help_page.is_some() {
            UiAction::CloseHelp
        } else {
            UiAction::LessonDone(lesson)
        });
    }
    None
}
