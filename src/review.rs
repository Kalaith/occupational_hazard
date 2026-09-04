//! First-month objective accounting and immutable review snapshots.
use crate::{
    contracts::Contract,
    simulation::{Adventurer, Guild},
};
use serde::{Deserialize, Serialize};

pub const REVIEW_DAY: u32 = 30;
pub const SERVICE_QUOTA: usize = 6;

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

    pub fn objective_counts(&self, qs: &[Contract]) -> (usize, u32) {
        (
            self.roster.iter().filter(|a| a.bronze).count(),
            qs.iter()
                .zip(&self.completed)
                .filter(|(q, _)| q.bronze)
                .map(|(_, n)| n)
                .sum(),
        )
    }

    pub fn finish_review(&mut self, qs: &[Contract]) {
        if self.day < REVIEW_DAY || self.month.review.is_some() {
            return;
        }
        let (certifications, commissions) = self.objective_counts(qs);
        self.month.review = Some(Review {
            service_returns: self.board.service_credit.len(),
            service_target: SERVICE_QUOTA,
            day: self.day,
            certifications,
            commissions,
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
            format!("SANDBOX / Returns day {returns}. Review is already filed.")
        } else if returns > REVIEW_DAY {
            format!("Returns day {returns}: too late for day 30 review. Continue in sandbox to collect.")
        } else {
            format!("Returns day {returns}; credited before the day 30 review.")
        }
    }

    pub fn validate_month(&self) -> Result<(), String> {
        if self.month.sandbox && self.month.review.is_none() {
            return Err("Sandbox ledger is missing its review.".into());
        }
        if let Some(r) = &self.month.review {
            if r.day < REVIEW_DAY
                || r.day > self.day
                || r.careers.len() != 3
                || r.certifications != r.careers.iter().filter(|a| a.bronze).count()
                || (!self.month.sandbox && r.day != self.day)
            {
                return Err("This ledger has an invalid first-month review.".into());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
