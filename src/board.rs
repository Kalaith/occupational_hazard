//! Dated offers and stable save identities; UI indices are disposable lookup caches.
use crate::{contracts::Contract, data::TextCatalog, simulation::Guild};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const LEGACY_IDS: [&str; 12] = [
    "cellar",
    "medicine",
    "beekeeper",
    "well",
    "lantern-trial",
    "north-bridge",
    "shutters",
    "quarry",
    "boundary",
    "shepherd",
    "fever",
    "bandages",
];

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Board {
    /// Names the definition behind every numeric cache in this ledger.
    pub definition_order: Vec<String>,
    pub accepted: BTreeSet<String>,
    /// Distinct successful service definitions, not repeatable-job grinding.
    pub service_credit: BTreeSet<String>,
}

pub struct Offer {
    pub id: String,
    pub arrives: u32,
    pub expires: u32,
}

impl Contract {
    pub fn offer(&self, day: u32) -> Option<Offer> {
        if day < self.arrival {
            return None;
        }
        let arrives = self.arrival + ((day - self.arrival) / self.interval) * self.interval;
        let expires = arrives + self.window - 1;
        (day <= expires).then(|| Offer {
            id: format!("{}@{arrives}", self.id),
            arrives,
            expires,
        })
    }
}

impl Guild {
    pub fn contract_open(&self, id: usize, qs: &[Contract]) -> bool {
        let Some(q) = qs.get(id) else {
            return false;
        };
        if self.expeditions.iter().any(|e| e.contract == id) {
            return false;
        }
        if q.promotion {
            return self.roster.iter().any(|a| !a.bronze && !a.trial_passed);
        }
        q.offer(self.day)
            .is_some_and(|o| !self.board.accepted.contains(&o.id))
    }

    pub fn open_contracts(&self, qs: &[Contract]) -> Vec<usize> {
        (0..qs.len())
            .filter(|&id| self.contract_open(id, qs))
            .collect()
    }

    pub fn offer_notice(&self, q: &Contract, text: &TextCatalog) -> String {
        let acceptance = if q.promotion {
            text.get("contract.standing_trial").to_string()
        } else if let Some(o) = q.offer(self.day) {
            text.format(
                "contract.accept_days",
                &[
                    ("arrives", o.arrives.to_string()),
                    ("expires", o.expires.to_string()),
                ],
            )
        } else {
            text.get("contract.expired").into()
        };
        let credit = if q.service {
            if self.board.service_credit.contains(&q.id) {
                text.get("contract.service_credited")
            } else {
                text.get("contract.service_new")
            }
        } else {
            text.get("contract.no_service")
        };
        text.format(
            "contract.offer_notice",
            &[
                ("acceptance", acceptance),
                ("credit", credit.to_string()),
                ("cutoff", self.cutoff_notice(q.days)),
            ],
        )
    }

    /// Resolve legacy numeric slots once, then resolve all caches by saved names.
    /// Never infer a changed content order from the current array position.
    pub fn migrate_board(&mut self, qs: &[Contract]) -> Result<(), String> {
        let legacy = self.board.definition_order.is_empty();
        let order: Vec<String> = if legacy {
            if ![6, 12].contains(&self.completed.len()) {
                return Err(self.text.get("error.unsupported_legacy_records").into());
            }
            LEGACY_IDS[..self.completed.len()]
                .iter()
                .map(|s| s.to_string())
                .collect()
        } else {
            self.board.definition_order.clone()
        };
        if order.len() != self.completed.len()
            || order.iter().collect::<BTreeSet<_>>().len() != order.len()
        {
            return Err(self.text.get("error.invalid_saved_contracts").into());
        }
        let indices: Vec<usize> = order
            .iter()
            .map(|key| {
                qs.iter().position(|q| &q.id == key).ok_or_else(|| {
                    self.text
                        .format("error.unknown_saved_contract", &[("key", key.clone())])
                })
            })
            .collect::<Result<_, _>>()?;
        let mut completed = vec![0; qs.len()];
        for (old, &new) in indices.iter().enumerate() {
            completed[new] = self.completed[old];
        }
        for e in &mut self.expeditions {
            e.contract = *indices
                .get(e.contract)
                .ok_or_else(|| self.text.get("error.invalid_saved_expedition").to_string())?;
            if legacy {
                e.instance = format!("{}@legacy-{}", qs[e.contract].id, e.returns);
                self.board.accepted.insert(e.instance.clone());
                if let Some(o) = qs[e.contract].offer(self.day) {
                    self.board.accepted.insert(o.id);
                }
            }
        }
        for id in &mut self.services.scouted {
            *id = *indices
                .get(*id)
                .ok_or_else(|| self.text.get("error.invalid_saved_scout").to_string())?;
        }
        if legacy {
            for (id, q) in qs.iter().enumerate() {
                if completed[id] > 0 {
                    if q.service {
                        self.board.service_credit.insert(q.id.clone());
                    }
                    if !q.promotion {
                        if let Some(o) = q.offer(self.day) {
                            self.board.accepted.insert(o.id);
                        }
                    }
                }
            }
        }
        self.completed = completed;
        self.board.definition_order = qs.iter().map(|q| q.id.clone()).collect();
        Ok(())
    }

    pub fn validate_board(&self, qs: &[Contract]) -> Result<(), String> {
        if !self.board.definition_order.is_empty()
            && self.board.definition_order != qs.iter().map(|q| q.id.clone()).collect::<Vec<_>>()
        {
            return Err(self.text.get("error.contract_migration_required").into());
        }
        if self
            .board
            .service_credit
            .iter()
            .any(|key| !qs.iter().any(|q| &q.id == key && q.service))
        {
            return Err(self.text.get("error.invalid_service_quota_records").into());
        }
        let mut instances = BTreeSet::new();
        for e in &self.expeditions {
            if !self.board.definition_order.is_empty()
                && (!instances.insert(&e.instance)
                    || !self.board.accepted.contains(&e.instance)
                    || !qs
                        .get(e.contract)
                        .is_some_and(|q| e.instance.starts_with(&format!("{}@", q.id))))
            {
                return Err(self
                    .text
                    .get("error.invalid_dated_expedition_records")
                    .into());
            }
        }
        Ok(())
    }
}
