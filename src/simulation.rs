//! Deterministic dispatch, recovery, reports and professional certification.
use crate::contracts::Contract;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Adventurer {
    pub name: String,
    pub class: String,
    pub trait_name: String,
    pub bronze: bool,
    pub xp: u32,
    pub successes: u32,
    pub fatigue: u32,
    pub injury: u32,
    pub trial_passed: bool,
}

impl Adventurer {
    pub fn eligible(&self) -> bool {
        !self.bronze && self.xp >= 60 && self.successes >= 3
    }

    pub fn rank(&self) -> &str {
        if self.bronze {
            "BRONZE"
        } else {
            "IRON"
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Expedition {
    pub contract: usize,
    pub party: Vec<usize>,
    pub returns: u32,
    pub strength: i32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Report {
    #[serde(default)]
    pub read: bool,
    pub title: String,
    pub body: String,
    pub reward: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Guild {
    #[serde(default)]
    pub tutorial: crate::tutorial::Tutorial,
    #[serde(default)]
    pub month: crate::review::Month,
    #[serde(default)]
    pub services: crate::services::Services,
    pub day: u32,
    pub gold: u32,
    pub reputation: u32,
    pub roster: Vec<Adventurer>,
    pub expeditions: Vec<Expedition>,
    pub reports: Vec<Report>,
    pub completed: Vec<u32>,
    pub victory_seen: bool,
}

impl Guild {
    pub fn new() -> Self {
        let roster = [
            ("Mira Ashford", "Fighter", "Protective"),
            ("Tomas Reed", "Ranger", "Cautious"),
            ("Pip Fenwick", "Healer", "Dependable"),
        ]
        .into_iter()
        .map(|(name, class, trait_name)| Adventurer {
            name: name.into(),
            class: class.into(),
            trait_name: trait_name.into(),
            bronze: false,
            xp: 0,
            successes: 0,
            fatigue: 0,
            injury: 0,
            trial_passed: false,
        })
        .collect();
        Self {
            tutorial: crate::tutorial::Tutorial::default(),
            month: crate::review::Month::default(),
            services: crate::services::Services::default(),
            day: 1,
            gold: 80,
            reputation: 0,
            roster,
            expeditions: vec![],
            reports: vec![],
            completed: vec![0; crate::contracts::CONTRACT_COUNT],
            victory_seen: false,
        }
    }

    pub fn validate(&self, contracts: &[Contract]) -> Result<(), String> {
        self.validate_month()?;
        let mut scouted = self.services.scouted.clone();
        scouted.sort_unstable();
        scouted.dedup();
        if scouted.len() != self.services.scouted.len()
            || scouted
                .iter()
                .any(|&id| id >= contracts.len() || contracts[id].promotion)
        {
            return Err("This ledger has invalid scouting records.".into());
        }
        if self.roster.len() != 3
            || self.completed.len() != contracts.len()
            || self.day == 0
            || self.day > 1_000_000
            || self.gold > 100_000_000
            || self.reputation > 1_000_000
            || self.reports.len() > 30
        {
            return Err("This ledger has unsupported or invalid guild records.".into());
        }
        let mut assigned = vec![];
        let mut quests = vec![];
        for e in &self.expeditions {
            if e.contract >= contracts.len()
                || e.returns <= self.day
                || e.returns > self.day + 3
                || e.party.is_empty()
                || quests.contains(&e.contract)
            {
                return Err("This ledger has an invalid expedition.".into());
            }
            quests.push(e.contract);
            for &id in &e.party {
                if id >= self.roster.len() || assigned.contains(&id) {
                    return Err("This ledger assigns an adventurer more than once.".into());
                }
                assigned.push(id);
            }
        }
        if self
            .roster
            .iter()
            .any(|a| a.xp > 1_000_000 || a.successes > 1_000_000 || a.fatigue > 6 || a.injury > 2)
        {
            return Err("This ledger has invalid adventurer records.".into());
        }
        Ok(())
    }

    pub fn busy(&self, id: usize) -> bool {
        self.expeditions.iter().any(|e| e.party.contains(&id))
    }

    pub fn strength(&self, q: &Contract, party: &[usize]) -> i32 {
        let mut total = 0;
        for &id in party {
            if let Some(a) = self.roster.get(id) {
                total += 5 + (a.xp / 20).min(5) as i32 + if a.bronze { 4 } else { 0 };
                total += if q.specialty == a.class || q.specialty == "Any" {
                    3
                } else {
                    0
                };
                total -= a.fatigue as i32;
            }
        }
        total
    }

    pub fn dispatch_problem(&self, id: usize, party: &[usize], qs: &[Contract]) -> Option<String> {
        if self.review_pending() {
            return Some("Read the review, then tap CONTINUE SANDBOX or RESTART.".into());
        }
        let Some(q) = qs.get(id) else {
            return Some("Select a contract.".into());
        };
        if !self.contract_open(id, qs) {
            return Some("This request is complete. Choose another contract.".into());
        }
        if self.expeditions.iter().any(|e| e.contract == id) {
            return Some("This contract already has an expedition.".into());
        }
        if party.is_empty() {
            return Some("Tap an adventurer to select a party.".into());
        }
        let mut seen = vec![];
        for &a in party {
            if a >= self.roster.len() || seen.contains(&a) {
                return Some("Invalid party.".into());
            }
            seen.push(a);
            if self.busy(a) {
                return Some("A selected adventurer is away.".into());
            }
            if self.roster[a].injury > 0 {
                return Some("Let injured adventurers recover: tap NEXT DAY.".into());
            }
        }
        if q.promotion
            && (party.len() != 1
                || !self.roster[party[0]].eligible()
                || self.roster[party[0]].trial_passed)
        {
            return Some(
                "Select one Iron candidate with 60 XP and 3 successes who still needs the trial."
                    .into(),
            );
        }
        if q.bronze && !party.iter().any(|&a| self.roster[a].bronze) {
            return Some("A Bronze adventurer must lead this contract.".into());
        }
        None
    }

    pub fn dispatch(&mut self, id: usize, party: &[usize], qs: &[Contract]) -> Result<(), String> {
        if let Some(reason) = self.dispatch_problem(id, party, qs) {
            return Err(reason);
        }
        self.expeditions.push(Expedition {
            contract: id,
            party: party.to_vec(),
            returns: self.day + qs[id].days,
            strength: self.prepared_strength(id, &qs[id], party),
        });
        self.services.scouted.retain(|&q| q != id);
        Ok(())
    }

    pub fn prepared_strength(&self, id: usize, q: &Contract, party: &[usize]) -> i32 {
        self.strength(q, party)
            + if self.services.scouted.contains(&id) {
                2
            } else {
                0
            }
    }

    pub fn next_day(&mut self, qs: &[Contract]) {
        if self.review_pending() {
            return;
        }
        // Recover only people who stayed at the guild, never people still travelling.
        for id in 0..self.roster.len() {
            if !self.busy(id) {
                let a = &mut self.roster[id];
                if self.services.training_yard
                    && !a.bronze
                    && a.xp < 60
                    && a.fatigue == 0
                    && a.injury == 0
                {
                    a.xp = (a.xp + 5).min(60);
                }
                a.fatigue = a.fatigue.saturating_sub(2);
                a.injury = a
                    .injury
                    .saturating_sub(if self.services.infirmary { 2 } else { 1 });
            }
        }
        self.day += 1;
        let mut remaining = vec![];
        for e in std::mem::take(&mut self.expeditions) {
            if e.returns > self.day {
                remaining.push(e);
                continue;
            }
            let q = &qs[e.contract];
            let success = e.strength >= q.difficulty;
            let close_call = e.strength < q.difficulty + 2;
            let names = e
                .party
                .iter()
                .map(|&id| self.roster[id].name.clone())
                .collect::<Vec<_>>()
                .join(" & ");
            let has_healer = e.party.iter().any(|&id| self.roster[id].class == "Healer");
            for &id in &e.party {
                let a = &mut self.roster[id];
                a.fatigue = (a.fatigue + 3).min(6);
                a.xp += if success { q.xp } else { 5 };
                if success {
                    a.successes += 1;
                }
                if q.promotion && success {
                    a.trial_passed = true;
                }
                if !success || (close_call && !has_healer) {
                    a.injury = if has_healer { 1 } else { 2 };
                }
            }
            if success {
                self.gold += q.gold;
                self.reputation += if q.promotion { 5 } else { 2 };
                self.completed[e.contract] += 1;
            }
            self.reports.insert(0, Report { read: false,
                title: format!("Day {} / {} / {}", self.day, if success { "SUCCESS" } else { "RETREAT" }, q.title),
                body: format!("{names}. {} {}", if success { &q.report } else {
                    "The party could not safely finish the job. Everyone returned; rest, bring support and try again."
                }, if !success || (close_call && !has_healer) { "Medical leave required. Tap NEXT DAY to recover." } else { "Everyone returned safely, but needs rest." }),
                reward: format!("Guild +{}g / Each adventurer +{} XP / Fatigue +3", if success { q.gold } else { 0 }, if success { q.xp } else { 5 }),
            });
        }
        self.expeditions = remaining;
        self.reports.truncate(30);
        // Returns, XP and payments on the cutoff day count before the review closes.
        self.finish_review(qs);
    }

    pub fn promote(&mut self, id: usize) -> Result<(), String> {
        if self.review_pending() {
            return Err("Tap CONTINUE SANDBOX to resume the guild.".into());
        }
        let busy = self.busy(id);
        let a = self.roster.get_mut(id).ok_or("Select an adventurer.")?;
        if busy || !a.eligible() || !a.trial_passed {
            return Err("The candidate must return with a passed assessment.".into());
        }
        a.bronze = true;
        self.reports.insert(0, Report { read: false, title: format!("{} / BRONZE CERTIFIED", a.name),
            body: "You sign the promotion form. An Iron recruit becomes a trusted Bronze adventurer. The North Bridge commission is now open.".into(),
            reward: "Bronze licence / stronger expedition capability / new commission".into() });
        self.reports.truncate(30);
        Ok(())
    }
}

#[cfg(test)]
mod tests;
