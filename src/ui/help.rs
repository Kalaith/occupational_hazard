use super::*;
use crate::tutorial::LESSONS;

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
