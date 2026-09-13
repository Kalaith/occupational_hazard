//! Saved contextual guidance; HELP can always read the complete desk handbook.
use crate::{data::TextCatalog, simulation::Guild};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Tutorial {
    pub seen: u16,
    pub skipped: bool,
    pub dispatched: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lesson {
    Welcome,
    Selection,
    Dispatch,
    Time,
    Reports,
    Recovery,
    Promotion,
    Trial,
}

pub const LESSONS: [Lesson; 8] = [
    Lesson::Welcome,
    Lesson::Selection,
    Lesson::Dispatch,
    Lesson::Time,
    Lesson::Reports,
    Lesson::Recovery,
    Lesson::Trial,
    Lesson::Promotion,
];

impl Lesson {
    pub fn title(self, text: &TextCatalog) -> &str {
        match self {
            Self::Welcome => text.get("lesson.welcome_title"),
            Self::Selection => text.get("lesson.selection_title"),
            Self::Dispatch => text.get("lesson.dispatch_title"),
            Self::Time => text.get("lesson.time_title"),
            Self::Reports => text.get("lesson.reports_title"),
            Self::Recovery => text.get("lesson.recovery_title"),
            Self::Promotion => text.get("lesson.promotion_title"),
            Self::Trial => text.get("lesson.trial_title"),
        }
    }

    pub fn text(self, guild: &Guild) -> String {
        let text = &guild.text;
        match self {
            Self::Welcome => text.format(
                "lesson.welcome_text",
                &[
                    ("cutoff_day", guild.config.review.cutoff_day.to_string()),
                    (
                        "service_quota",
                        guild.config.review.service_quota.to_string(),
                    ),
                ],
            ),
            Self::Selection => text.get("lesson.selection_text").into(),
            Self::Dispatch => text.get("lesson.dispatch_text").into(),
            Self::Time => text.get("lesson.time_text").into(),
            Self::Reports => text.get("lesson.reports_text").into(),
            Self::Recovery => text.get("lesson.recovery_text").into(),
            Self::Promotion => text.format(
                "lesson.promotion_text",
                &[
                    ("trial_xp", guild.config.progression.trial_xp.to_string()),
                    (
                        "trial_successes",
                        guild.config.progression.trial_successes.to_string(),
                    ),
                ],
            ),
            Self::Trial => text.get("lesson.trial_text").into(),
        }
    }
}

impl Tutorial {
    pub fn acknowledge(&mut self, lesson: Lesson) {
        self.seen |= 1 << lesson as u16;
    }
    pub fn has_seen(&self, lesson: Lesson) -> bool {
        !self.unseen(lesson)
    }

    fn unseen(&self, lesson: Lesson) -> bool {
        self.seen & (1 << lesson as u16) == 0
    }
}

impl Guild {
    pub fn lesson(&self, party_selected: bool) -> Option<Lesson> {
        if self.tutorial.skipped || self.review_pending() || self.month.sandbox {
            return None;
        }
        let candidates = [
            (Lesson::Welcome, true),
            (Lesson::Reports, !self.reports.is_empty()),
            (Lesson::Recovery, !self.reports.is_empty()),
            (
                Lesson::Promotion,
                self.roster.iter().any(|a| a.trial_passed && !a.bronze),
            ),
            (
                Lesson::Trial,
                self.roster
                    .iter()
                    .any(|a| a.eligible(&self.config) && !a.trial_passed),
            ),
            (
                Lesson::Time,
                self.tutorial.dispatched && !self.expeditions.is_empty(),
            ),
            (Lesson::Dispatch, party_selected),
            (
                Lesson::Selection,
                !self.tutorial.dispatched && !party_selected,
            ),
        ];
        candidates
            .into_iter()
            .find(|(l, ready)| *ready && self.tutorial.unseen(*l))
            .map(|(l, _)| l)
    }
}
