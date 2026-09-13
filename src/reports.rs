//! Persistent inbox state; reading a report never reapplies expedition rewards.
use crate::simulation::Guild;

impl Guild {
    pub fn unread_reports(&self) -> usize {
        self.reports.iter().filter(|r| !r.read).count()
    }

    pub fn read_report(&mut self, index: usize) -> bool {
        if let Some(report) = self.reports.get_mut(index) {
            report.read = true;
            true
        } else {
            false
        }
    }
}
