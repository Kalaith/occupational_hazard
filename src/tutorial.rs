//! Saved contextual guidance; HELP can always read the complete desk handbook.
use crate::simulation::Guild;
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
    pub fn title(self) -> &'static str {
        match self {
            Self::Welcome => "YOUR FIRST MONTH",
            Self::Selection => "SELECT A CONTRACT AND PARTY",
            Self::Dispatch => "SEND THE EXPEDITION",
            Self::Time => "ADVANCE THE JOURNEY",
            Self::Reports => "READ THE RETURN REPORT",
            Self::Recovery => "REST AND RECOVER",
            Self::Promotion => "SIGN THE BRONZE LICENCE",
            Self::Trial => "TAKE THE BRONZE TRIAL",
        }
    }

    pub fn text(self, _compact: bool) -> &'static str {
        match self {
            Self::Welcome => "Tap BACK TO HEADQUARTERS. Tap a room to open its work or a person to inspect their career. Review shows the day-30 targets: one Bronze certification, one Bronze commission and six distinct service successes.",
            Self::Selection => "Tap BACK TO HEADQUARTERS, then Jobs. Tap Details to browse with Previous job and Next job. Tap Party to return, then tap portrait cards to add or remove people. Copper rings mark selection; away or injured staff cannot join.",
            Self::Dispatch => "Tap BACK TO HEADQUARTERS, then Jobs. Check readiness, fatigue and return day. Details explains the acceptance deadline and offers Scout Route. Tap DISPATCH PARTY to commit. On short screens, first tap REVIEW DISPATCH.",
            Self::Time => "Tap BACK TO HEADQUARTERS, then ADVANCE DAY. Journeys advance immediately and people at home recover or train. The footer identifies the next return. Tap its entry to inspect the accepted assignment.",
            Self::Reports => "Tap BACK TO HEADQUARTERS, then Returns. Tap a NEW report to inspect its people, outcome and rewards. Tap ACKNOWLEDGE to return to the list. Rewards are applied once on return, never when reading.",
            Self::Recovery => "Tap BACK TO HEADQUARTERS. Resting and injured people appear upstairs in Recovery. Tap a person for their exact fatigue and medical leave. Leave them home and tap ADVANCE DAY. Tap Recovery for the optional infirmary upgrade.",
            Self::Promotion => "Tap BACK TO HEADQUARTERS, then Staff and the candidate. Earn 60 XP and three successes, pass the solo trial, then tap APPROVE BRONZE when home. Each person earns their own rank. A Bronze leader unlocks the Bronze commission.",
            Self::Trial => "Tap BACK TO HEADQUARTERS, then Staff, the candidate and PREPARE SOLO TRIAL. This selects that person alone. Rest until fatigue clears, check the blockers in Jobs, then tap DISPATCH PARTY. The assessment must be unaided.",
        }
    }
}

impl Tutorial {
    pub fn acknowledge(&mut self, lesson: Lesson) {
        self.seen |= 1 << lesson as u16;
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
                self.roster.iter().any(|a| a.eligible() && !a.trial_passed),
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

#[cfg(test)]
mod tests;
