//! Tutorial prompts and the complete help handbook.
use super::*;
use crate::tutorial::Lesson;
use crate::tutorial::LESSONS;
use crate::ui::headquarters::first;

/// Guidance occupies its own desk panel; every game control remains interactive.
pub fn draw_guidance(g: &Game, r: Rect) -> Option<UiAction> {
    let lesson = g.lesson()?;
    panel(r, PANEL);
    paragraph(
        &tip(g, lesson),
        Rect::new(r.x + 10.0, r.y + 6.0, r.w - 20.0, 58.0),
        18.0,
        GOLD,
    );
    let width = (r.w - 30.0) / 2.0;
    if button(
        Rect::new(r.x + 10.0, r.y + 66.0, width, 44.0),
        g.guild.text.get("help.next_tip"),
        false,
    ) {
        return Some(UiAction::LessonDone(lesson));
    }
    if button(
        Rect::new(r.x + width + 20.0, r.y + 66.0, width, 44.0),
        g.guild.text.get("help.skip"),
        false,
    ) {
        return Some(UiAction::SkipTutorial);
    }
    None
}

fn tip(g: &Game, lesson: Lesson) -> String {
    match lesson {
        Lesson::Welcome => g.guild.text.format(
            "help.welcome",
            &[
                ("cutoff_day", g.guild.config.review.cutoff_day.to_string()),
                (
                    "service_quota",
                    g.guild.config.review.service_quota.to_string(),
                ),
            ],
        ),
        Lesson::Selection if g.hq.sheet != crate::headquarters::Sheet::Jobs => {
            g.guild.text.get("help.selection_rooms").into()
        }
        Lesson::Selection => g.guild.text.get("help.selection_jobs").into(),
        Lesson::Dispatch if g.hq.sheet != crate::headquarters::Sheet::Jobs => {
            g.guild.text.get("help.dispatch_rooms").into()
        }
        Lesson::Dispatch => g.guild.text.get("help.dispatch_jobs").into(),
        Lesson::Time => g.guild.text.get("help.time").into(),
        Lesson::Reports => g.guild.text.get("help.reports").into(),
        Lesson::Recovery => g.guild.text.get("help.recovery").into(),
        Lesson::Trial => g.guild.text.get("help.trial").into(),
        Lesson::Promotion => g.guild.text.get("help.promotion").into(),
    }
}

pub fn is_target(g: &Game, text: &str) -> bool {
    match g.lesson() {
        Some(Lesson::Welcome) => text.starts_with(g.guild.text.get("ui.review")),
        Some(Lesson::Selection) => {
            text == g.guild.text.get("room.assignments") || text.contains(first(g, 0))
        }
        Some(Lesson::Dispatch) => {
            text == g.guild.text.get("room.assignments")
                || text == g.guild.text.get("ui.dispatch_party")
        }
        Some(Lesson::Time | Lesson::Recovery) => text == g.guild.text.get("ui.advance_day"),
        Some(Lesson::Reports) => {
            text == g.guild.text.get("room.departure")
                || text.starts_with(g.guild.text.get("ui.return_unread"))
        }
        Some(Lesson::Trial) => text == g.guild.text.get("room.common"),
        Some(Lesson::Promotion) => {
            text == g.guild.text.get("room.common") || text == g.guild.text.get("ui.approve_bronze")
        }
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
        lesson.title(&g.guild.text),
        Rect::new(x + 12.0, y + 12.0, w - 24.0, 44.0),
        24.0,
        GOLD,
    );
    paragraph(
        &lesson.text(&g.guild),
        Rect::new(x + 20.0, y + 78.0, w - 40.0, h - 222.0),
        22.0,
        WHITE,
    );
    if let Some(page) = g.help_page {
        if button(
            Rect::new(x + 20.0, y + h - 124.0, w - 40.0, 46.0),
            &g.guild.text.format(
                "help.next",
                &[
                    ("page", (page + 1).to_string()),
                    ("pages", LESSONS.len().to_string()),
                ],
            ),
            false,
        ) {
            return Some(UiAction::Help((page + 1) % LESSONS.len()));
        }
    } else if button(
        Rect::new(x + 20.0, y + h - 124.0, w - 40.0, 46.0),
        g.guild.text.get("help.skip"),
        false,
    ) {
        return Some(UiAction::SkipTutorial);
    }
    if button(
        Rect::new(x + 20.0, y + h - 66.0, w - 40.0, 46.0),
        g.guild.text.get("ui.back_headquarters"),
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
