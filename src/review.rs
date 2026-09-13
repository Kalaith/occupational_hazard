//! First-month objective accounting and immutable review snapshots.
use crate::{
    contracts::Contract,
    simulation::{Adventurer, Guild},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObjectiveCounts {
    pub certifications: usize,
    pub commissions: u32,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Month {
    pub review: Option<Review>,
    pub sandbox: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Review {
    #[serde(default)]
    pub service_returns: usize,
    #[serde(default)]
    pub service_target: usize,
    pub day: u32,
    pub certifications: usize,
    pub commissions: u32,
    pub gold: u32,
    pub reputation: u32,
    pub careers: Vec<Adventurer>,
}

impl Review {
    pub fn passed(&self) -> bool {
        self.certifications > 0
            && self.commissions > 0
            && self.service_returns >= self.service_target
    }
}

impl Guild {
    pub fn review_pending(&self) -> bool {
        self.month.review.is_some() && !self.month.sandbox
    }

    pub fn objective_counts(&self, qs: &[Contract]) -> ObjectiveCounts {
        ObjectiveCounts {
            certifications: self.roster.iter().filter(|a| a.bronze).count(),
            commissions: qs
                .iter()
                .zip(&self.completed)
                .filter(|(q, _)| q.bronze)
                .map(|(_, n)| n)
                .sum(),
        }
    }

    pub fn finish_review(&mut self, qs: &[Contract]) {
        if self.day < self.config.review.cutoff_day || self.month.review.is_some() {
            return;
        }
        let counts = self.objective_counts(qs);
        self.month.review = Some(Review {
            service_returns: self.board.service_credit.len(),
            service_target: self.config.review.service_quota,
            day: self.day,
            certifications: counts.certifications,
            commissions: counts.commissions,
            gold: self.gold,
            reputation: self.reputation,
            careers: self.roster.clone(),
        });
    }

    pub fn continue_sandbox(&mut self) {
        if self.month.review.is_some() {
            self.month.sandbox = true;
        }
    }

    pub fn cutoff_notice(&self, days: u32) -> String {
        let returns = self.day + days;
        if self.month.sandbox {
            self.text
                .format("review.sandbox_returns", &[("day", returns.to_string())])
        } else if returns > self.config.review.cutoff_day {
            self.text.format(
                "review.late_returns",
                &[
                    ("day", returns.to_string()),
                    ("cutoff_day", self.config.review.cutoff_day.to_string()),
                ],
            )
        } else {
            self.text.format(
                "review.credited_returns",
                &[
                    ("day", returns.to_string()),
                    ("cutoff_day", self.config.review.cutoff_day.to_string()),
                ],
            )
        }
    }

    pub fn validate_month(&self) -> Result<(), String> {
        if self.month.sandbox && self.month.review.is_none() {
            return Err("Sandbox ledger is missing its review.".into());
        }
        if let Some(r) = &self.month.review {
            if r.day < self.config.review.cutoff_day
                || r.day > self.day
                || r.careers.len() != self.config.roster.len()
                || r.certifications != r.careers.iter().filter(|a| a.bronze).count()
                || (!self.month.sandbox && r.day != self.day)
            {
                return Err("This ledger has an invalid first-month review.".into());
            }
        }
        Ok(())
    }
}
