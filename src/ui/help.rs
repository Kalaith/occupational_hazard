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
        Lesson::Welcome => "By day 30: promote one Bronze, complete a Bronze commission and 6 distinct service jobs. Tap Review for details.",
        Lesson::Selection if g.hq.sheet != crate::headquarters::Sheet::Jobs => "Tap the Assignments room or Jobs to inspect work. Tap a person or Staff to inspect their career.",
        Lesson::Selection => "Tap a portrait card to add that person. A copper ring marks selection. Tap Details to browse jobs or scout.",
        Lesson::Dispatch if g.hq.sheet != crate::headquarters::Sheet::Jobs => "Tap Jobs to review your selected party and send the expedition.",
        Lesson::Dispatch => "Check readiness and return day, then tap DISPATCH PARTY. Tap Details for the deadline and preparation.",
        Lesson::Time => "Tap ADVANCE DAY to advance journeys. People at home recover while others travel.",
        Lesson::Reports => "Tap Returns to read every return. Each expedition has its own report and automatic rewards.",
        Lesson::Recovery => "Leave tired or injured people at home and tap ADVANCE DAY. Typical fatigue clears in two days.",
        Lesson::Trial => "Tap Staff, select the rested candidate and tap PREPARE SOLO TRIAL. In Jobs, check readiness and tap DISPATCH PARTY.",
        Lesson::Promotion => "Tap Staff, select the passed candidate, then APPROVE BRONZE to promote that person.",
    }
}

pub fn is_target(g: &Game, text: &str) -> bool {
    match g.lesson() {
        Some(Lesson::Welcome) => text.starts_with("Review"),
        Some(Lesson::Selection) => text == "Jobs" || text.contains("Mira"),
        Some(Lesson::Dispatch) => text == "Jobs" || text == "DISPATCH PARTY",
        Some(Lesson::Time | Lesson::Recovery) => text == "ADVANCE DAY",
        Some(Lesson::Reports) => text.starts_with("Returns"),
        Some(Lesson::Trial) => text == "Staff",
        Some(Lesson::Promotion) => text == "Staff" || text == "APPROVE BRONZE",
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
        "BACK TO HEADQUARTERS",
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
