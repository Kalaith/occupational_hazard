//! Deterministic dispatch, recovery, reports and professional certification.
use crate::{
    contracts::Contract,
    data::{GameConfig, TextCatalog},
};
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
    pub fn eligible(&self, config: &GameConfig) -> bool {
        !self.bronze
            && self.xp >= config.progression.trial_xp
            && self.successes >= config.progression.trial_successes
    }

    pub fn rank<'a>(&self, text: &'a TextCatalog) -> &'a str {
        if self.bronze {
            text.get("rank.bronze")
        } else {
            text.get("rank.iron")
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Expedition {
    #[serde(default)]
    pub instance: String,
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
    #[serde(skip)]
    pub config: GameConfig,
    #[serde(skip)]
    pub text: TextCatalog,
    #[serde(default)]
    pub board: crate::board::Board,
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
        Self::with_config(GameConfig::default())
    }

    pub fn with_config(config: GameConfig) -> Self {
        let starting_day = config.starting.day;
        let starting_gold = config.starting.gold;
        let starting_reputation = config.starting.reputation;
        let roster = config
            .roster
            .iter()
            .map(|entry| Adventurer {
                name: entry.name.clone(),
                class: entry.class.clone(),
                trait_name: entry.trait_name.clone(),
                bronze: false,
                xp: 0,
                successes: 0,
                fatigue: 0,
                injury: 0,
                trial_passed: false,
            })
            .collect();
        Self {
            config,
            text: TextCatalog::default(),
            board: crate::board::Board::default(),
            tutorial: crate::tutorial::Tutorial::default(),
            month: crate::review::Month::default(),
            services: crate::services::Services::default(),
            day: starting_day,
            gold: starting_gold,
            reputation: starting_reputation,
            roster,
            expeditions: vec![],
            reports: vec![],
            completed: vec![0; crate::contracts::CONTRACT_COUNT],
            victory_seen: false,
        }
    }

    /// Fully recovered adventurers who would spend the next day at the guild.
    pub fn rested_idle(&self) -> Vec<&str> {
        self.roster
            .iter()
            .enumerate()
            .filter(|(id, a)| !self.busy(*id) && a.fatigue == 0 && a.injury == 0)
            .map(|(_, a)| a.name.as_str())
            .collect()
    }

    pub fn validate(&self, contracts: &[Contract]) -> Result<(), String> {
        self.validate_month()?;
        self.validate_board(contracts)?;
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
        if self.roster.len() != self.config.roster.len()
            || self.completed.len() != contracts.len()
            || self.day == 0
            || self.day > self.config.caps.max_day
            || self.gold > self.config.caps.max_gold
            || self.reputation > self.config.caps.max_reputation
            || self.reports.len() > self.config.review.max_saved_reports
        {
            return Err("This ledger has unsupported or invalid guild records.".into());
        }
        let mut assigned = vec![];
        let mut quests = vec![];
        for e in &self.expeditions {
            if e.contract >= contracts.len()
                || e.returns <= self.day
                || e.returns > self.day + self.config.expedition.max_expedition_days
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
        if self.roster.iter().any(|a| {
            a.xp > self.config.caps.max_xp
                || a.successes > self.config.caps.max_successes
                || a.fatigue > self.config.caps.max_fatigue
                || a.injury > self.config.caps.max_injury
        }) {
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
                total += self.config.expedition.base_strength
                    + (a.xp / self.config.expedition.xp_strength_step)
                        .min(self.config.expedition.xp_strength_cap) as i32
                    + if a.bronze {
                        self.config.expedition.bronze_strength_bonus
                    } else {
                        0
                    };
                total += if q.specialty == a.class || q.specialty == "Any" {
                    self.config.expedition.specialty_bonus
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
            return Some(self.text.get("error.review_pending").into());
        }
        let Some(q) = qs.get(id) else {
            return Some(self.text.get("error.select_contract").into());
        };
        if !self.contract_open(id, qs) {
            return Some(self.text.get("error.offer_unavailable").into());
        }
        if self.expeditions.iter().any(|e| e.contract == id) {
            return Some(self.text.get("error.expedition_exists").into());
        }
        if party.is_empty() {
            return Some(self.text.get("error.select_party").into());
        }
        let mut seen = vec![];
        for &a in party {
            if a >= self.roster.len() || seen.contains(&a) {
                return Some(self.text.get("error.invalid_party").into());
            }
            seen.push(a);
            if self.busy(a) {
                return Some(self.text.get("error.adventurer_away").into());
            }
            if self.roster[a].injury > 0 {
                return Some(self.text.get("error.adventurer_injured").into());
            }
        }
        if q.promotion
            && (party.len() != 1
                || !self.roster[party[0]].eligible(&self.config)
                || self.roster[party[0]].trial_passed)
        {
            return Some(self.text.format(
                "error.trial_candidate",
                &[
                    ("trial_xp", self.config.progression.trial_xp.to_string()),
                    (
                        "trial_successes",
                        self.config.progression.trial_successes.to_string(),
                    ),
                ],
            ));
        }
        if q.bronze && !party.iter().any(|&a| self.roster[a].bronze) {
            return Some(self.text.get("error.bronze_leader").into());
        }
        None
    }

    pub fn dispatch(&mut self, id: usize, party: &[usize], qs: &[Contract]) -> Result<(), String> {
        if let Some(reason) = self.dispatch_problem(id, party, qs) {
            return Err(reason);
        }
        if self.board.definition_order.is_empty() {
            self.board.definition_order = qs.iter().map(|q| q.id.clone()).collect();
        }
        let instance = if qs[id].promotion {
            format!("{}@{}-{}", qs[id].id, self.day, party[0])
        } else {
            qs[id]
                .offer(self.day)
                .ok_or_else(|| self.text.get("error.offer_expired").to_string())?
                .id
        };
        self.board.accepted.insert(instance.clone());
        self.expeditions.push(Expedition {
            instance,
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
                self.config.services.scout_strength_bonus
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
                    && a.xp < self.config.progression.training_xp_cap
                    && a.fatigue == 0
                    && a.injury == 0
                {
                    a.xp = (a.xp + self.config.services.training_xp_per_day)
                        .min(self.config.progression.training_xp_cap);
                }
                a.fatigue = a
                    .fatigue
                    .saturating_sub(self.config.services.home_fatigue_recovery);
                a.injury = a.injury.saturating_sub(if self.services.infirmary {
                    self.config.services.infirmary_injury_recovery
                } else {
                    self.config.services.basic_injury_recovery
                });
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
            let close_call = e.strength < q.difficulty + self.config.expedition.close_call_margin;
            let names = e
                .party
                .iter()
                .map(|&id| self.roster[id].name.clone())
                .collect::<Vec<_>>()
                .join(" & ");
            let has_healer = e.party.iter().any(|&id| self.roster[id].class == "Healer");
            for &id in &e.party {
                let a = &mut self.roster[id];
                a.fatigue = (a.fatigue + self.config.expedition.mission_fatigue)
                    .min(self.config.caps.max_fatigue);
                a.xp += if success {
                    q.xp
                } else {
                    self.config.expedition.failed_xp
                };
                if success {
                    a.successes += 1;
                }
                if q.promotion && success {
                    a.trial_passed = true;
                }
                if !success || (close_call && !has_healer) {
                    a.injury = if has_healer {
                        self.config.expedition.healer_injury
                    } else {
                        self.config.expedition.failure_injury
                    };
                }
            }
            if success {
                self.gold += q.gold;
                self.reputation += if q.promotion {
                    self.config.expedition.promotion_reputation
                } else {
                    self.config.expedition.standard_reputation
                };
                self.completed[e.contract] += 1;
                if q.service && self.day <= self.config.review.cutoff_day {
                    self.board.service_credit.insert(q.id.clone());
                }
            }
            let result = if success {
                self.text.get("ui.success")
            } else {
                self.text.get("ui.retreat")
            };
            let resolution = if success {
                q.report.clone()
            } else {
                self.text.get("report.failure_body").to_string()
            };
            let recovery = if !success || (close_call && !has_healer) {
                self.text.get("report.medical_leave")
            } else {
                self.text.get("report.safe_return")
            };
            self.reports.insert(
                0,
                Report {
                    read: false,
                    title: self.text.format(
                        "report.title",
                        &[
                            ("day", self.day.to_string()),
                            ("result", result.to_string()),
                            ("contract", q.title.clone()),
                        ],
                    ),
                    body: format!("{names}. {resolution} {recovery}"),
                    reward: self.text.format(
                        "report.reward",
                        &[
                            ("gold", if success { q.gold } else { 0 }.to_string()),
                            (
                                "xp",
                                if success {
                                    q.xp
                                } else {
                                    self.config.expedition.failed_xp
                                }
                                .to_string(),
                            ),
                            (
                                "fatigue",
                                self.config.expedition.mission_fatigue.to_string(),
                            ),
                        ],
                    ),
                },
            );
        }
        self.expeditions = remaining;
        self.reports.truncate(self.config.review.max_saved_reports);
        // Returns, XP and payments on the cutoff day count before the review closes.
        self.finish_review(qs);
    }

    pub fn promote(&mut self, id: usize) -> Result<(), String> {
        if self.review_pending() {
            return Err(self.text.get("error.review_pending").into());
        }
        let busy = self.busy(id);
        let a = self
            .roster
            .get_mut(id)
            .ok_or_else(|| self.text.get("error.select_adventurer").to_string())?;
        if busy || !a.eligible(&self.config) || !a.trial_passed {
            return Err(self.text.get("error.candidate_unready").into());
        }
        let name = a.name.clone();
        a.bronze = true;
        self.reports.insert(
            0,
            Report {
                read: false,
                title: self
                    .text
                    .format("report.promotion_title", &[("name", name)]),
                body: self.text.get("report.promotion_body").into(),
                reward: self.text.get("report.promotion_reward").into(),
            },
        );
        self.reports.truncate(self.config.review.max_saved_reports);
        Ok(())
    }
}

impl Default for Guild {
    fn default() -> Self {
        Self::new()
    }
}
