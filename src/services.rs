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
            return Err("Tap CONTINUE SANDBOX to resume the guild.".into());
        }
        let (cost, message) = match purchase {
            Purchase::Infirmary => {
                if self.services.infirmary {
                    return Err("The infirmary is already open.".into());
                }
                (
                    self.config.services.infirmary_cost,
                    "Infirmary opened. Medical leave now recovers twice as fast at the guild.",
                )
            }
            Purchase::TrainingYard => {
                if self.services.training_yard {
                    return Err("The training yard is already open.".into());
                }
                (self.config.services.training_yard_cost, "Training yard opened. Fully rested Iron recruits gain 5 XP per day at the guild, up to 60 XP.")
            }
            Purchase::Scout(id) => {
                if !self.contract_open(id, contracts) {
                    return Err("Choose an unfinished contract to scout.".into());
                }
                let q = contracts
                    .get(id)
                    .ok_or("Select a valid contract to scout.")?;
                if q.promotion {
                    return Err(
                        "The promotion trial must be completed unaided. Scouts cannot assist."
                            .into(),
                    );
                }
                if self.services.scouted.contains(&id) {
                    return Err("Scouts have already prepared this route.".into());
                }
                if self.expeditions.iter().any(|e| e.contract == id) {
                    return Err("That expedition has already left. Scout before dispatch.".into());
                }
                (self.config.services.scout_cost, "Scouts prepared the route. The next party on this route gets an advantage, even on a later posting.")
            }
        };
        if self.gold < cost {
            return Err(format!(
                "Requires {cost}g. Complete contracts to earn more."
            ));
        }
        self.gold -= cost;
        match purchase {
            Purchase::Infirmary => self.services.infirmary = true,
            Purchase::TrainingYard => self.services.training_yard = true,
            Purchase::Scout(id) => self.services.scouted.push(id),
        }
        Ok(message.into())
    }
}
