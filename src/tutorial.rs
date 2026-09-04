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

    pub fn text(self, compact: bool) -> &'static str {
        match self {
            Self::Welcome => "By day 30, certify one Bronze adventurer and complete A Bridge Worth Keeping. Day 30 returns count before head office reviews the branch.\n\nTap BACK TO DESK, then the D30 objective control to see your targets. MENU opens HELP whenever you need it.",
            Self::Selection if compact => "Tap BACK TO DESK. Tap < or > to browse contracts, then CHOOSE PARTY. Tap adventurer names to select them. Selected names turn gold.\n\nStart with Mira Ashford on Cellar, Meet Sword. Matching classes help; injured or away staff cannot join.",
            Self::Selection => "Tap BACK TO DESK. Tap a contract in CONTRACT REGISTER, then tap adventurer names to select them. Selected names turn gold.\n\nStart with Mira Ashford on Cellar, Meet Sword. Matching classes help; injured or away staff cannot join.",
            Self::Dispatch => "Your party is selected. Check the readiness message, then tap BACK TO DESK and DISPATCH.\n\nDispatch sends the selected people away for the contract's listed duration. Work returning after day 30 cannot improve the review; it can finish in sandbox.",
            Self::Time => "The expedition is travelling. Tap BACK TO DESK, then NEXT DAY to advance one day. Repeat until the party returns.\n\nPeople staying at the guild rest while others travel. Returning expeditions receive their rewards before the day-30 review.",
            Self::Reports => "An expedition returned. Tap BACK TO DESK, then REPORTS to read what happened. Tap NEWER REPORT or OLDER REPORT to browse.\n\nGold, experience and successful-contract credit are applied automatically. The report explains any medical leave.",
            Self::Recovery => "Tap BACK TO DESK, then ADVENTURERS to inspect fatigue and medical leave. Leave tired or injured staff at home and tap NEXT DAY to recover.\n\nRest two days after a typical return. An injured adventurer cannot dispatch. GUILD SERVICES offers an infirmary once you can afford it.",
            Self::Promotion => "Earn 60 XP and 3 successes, then pass The Lantern Road Trial with that candidate alone.\n\nTap BACK TO DESK, then ADVENTURERS, select the candidate and tap APPROVE BRONZE. Approval unlocks A Bridge Worth Keeping. Send a Bronze leader with support and return by day 30.",
            Self::Trial => "A candidate has 60 XP and 3 successes. Rest them at home with NEXT DAY until fatigue clears.\n\nTap BACK TO DESK, then CONTRACTS. Select The Lantern Road Trial, choose that candidate alone and tap DISPATCH. Tap NEXT DAY twice for the assessment. On compact screens, tap CHOOSE PARTY to select the candidate.",
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
