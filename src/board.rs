//! Completed requests leave the board; trial appointments belong to candidates.
use crate::{contracts::Contract, simulation::Guild};

impl Guild {
    pub fn contract_open(&self, id: usize, qs: &[Contract]) -> bool {
        let Some(q) = qs.get(id) else {
            return false;
        };
        if q.promotion {
            self.roster.iter().any(|a| !a.bronze && !a.trial_passed)
        } else {
            self.completed.get(id) == Some(&0)
        }
    }

    pub fn open_contracts(&self, qs: &[Contract]) -> Vec<usize> {
        (0..qs.len())
            .filter(|&id| self.contract_open(id, qs))
            .collect()
    }

    pub fn adjacent_contract(&self, current: usize, forward: bool, qs: &[Contract]) -> usize {
        let ids = self.open_contracts(qs);
        if ids.is_empty() {
            return current;
        }
        let index = ids.iter().position(|&id| id == current).unwrap_or(0);
        ids[(index + if forward { 1 } else { ids.len() - 1 }) % ids.len()]
    }

    /// The original six slots are append-only and retain their exact meanings.
    /// Outstanding missions, purchases, scouting and completion records stay intact.
    pub fn migrate_board(&mut self, qs: &[Contract]) {
        if self.completed.len() == 6 && qs.len() == crate::contracts::CONTRACT_COUNT {
            self.completed.resize(qs.len(), 0);
        }
    }
}

#[cfg(test)]
mod tests;
