//! Guild facilities and paid expedition preparation.
use crate::simulation::Guild;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Services {
    pub infirmary: bool,
    pub training_yard: bool,
    pub scouted: Vec<usize>,
}

#[derive(Clone, Copy)]
pub enum Purchase {
    Infirmary,
    TrainingYard,
    Scout(usize),
}

impl Guild {
    pub fn purchase(
        &mut self,
        purchase: Purchase,
        contracts: &[crate::contracts::Contract],
    ) -> Result<String, String> {
        if self.review_pending() {
            return Err(self.text.get("service.review_pending").into());
        }
        let (cost, message) = match purchase {
            Purchase::Infirmary => {
                if self.services.infirmary {
                    return Err(self.text.get("service.infirmary_exists").into());
                }
                (
                    self.config.services.infirmary_cost,
                    self.text.get("service.infirmary_open").to_string(),
                )
            }
            Purchase::TrainingYard => {
                if self.services.training_yard {
                    return Err(self.text.get("service.training_exists").into());
                }
                (
                    self.config.services.training_yard_cost,
                    self.text.format(
                        "service.training_open",
                        &[
                            (
                                "xp_per_day",
                                self.config.services.training_xp_per_day.to_string(),
                            ),
                            (
                                "xp_cap",
                                self.config.progression.training_xp_cap.to_string(),
                            ),
                        ],
                    ),
                )
            }
            Purchase::Scout(id) => {
                if !self.contract_open(id, contracts) {
                    return Err(self.text.get("service.choose_contract").into());
                }
                let q = contracts
                    .get(id)
                    .ok_or_else(|| self.text.get("service.invalid_contract").to_string())?;
                if q.promotion {
                    return Err(self.text.get("service.trial_no_scout").into());
                }
                if self.services.scouted.contains(&id) {
                    return Err(self.text.get("service.scout_exists").into());
                }
                if self.expeditions.iter().any(|e| e.contract == id) {
                    return Err(self.text.get("service.expedition_exists").into());
                }
                (
                    self.config.services.scout_cost,
                    self.text.get("service.scout_prepared").to_string(),
                )
            }
        };
        if self.gold < cost {
            return Err(self
                .text
                .format("service.requires_gold", &[("cost", cost.to_string())]));
        }
        self.gold -= cost;
        match purchase {
            Purchase::Infirmary => self.services.infirmary = true,
            Purchase::TrainingYard => self.services.training_yard = true,
            Purchase::Scout(id) => self.services.scouted.push(id),
        }
        Ok(message)
    }
}
