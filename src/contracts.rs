//! Authored contracts for the first guild licence.
use serde::Deserialize;

pub const CONTRACT_COUNT: usize = 12;

#[derive(Clone, Deserialize)]
pub struct Contract {
    pub id: String,
    pub arrival: u32,
    pub window: u32,
    pub interval: u32,
    pub service: bool,
    pub title: String,
    pub client: String,
    pub brief: String,
    pub danger: String,
    pub specialty: String,
    pub difficulty: i32,
    pub days: u32,
    pub gold: u32,
    pub xp: u32,
    pub report: String,
    pub promotion: bool,
    pub bronze: bool,
}

pub fn load() -> Result<Vec<Contract>, String> {
    let contracts: Vec<Contract> =
        macroquad_toolkit::include_json!("../assets/data/contracts.json")?;
    let mut ids = std::collections::BTreeSet::new();
    for q in &contracts {
        if q.id.is_empty()
            || !ids.insert(&q.id)
            || q.arrival == 0
            || q.interval == 0
            || q.window == 0
            || q.window > q.interval
            || (q.service && (q.promotion || q.bronze))
        {
            return Err("assets/data/contracts.json: invalid identity or arrival schedule.".into());
        }
    }
    if contracts.len() != CONTRACT_COUNT || contracts.iter().filter(|q| q.promotion).count() != 1 {
        return Err("The Iron licence needs twelve contracts and one promotion assessment.".into());
    }
    if contracts
        .iter()
        .any(|q| q.days == 0 || q.days > 3 || q.difficulty < 1 || q.xp == 0)
    {
        return Err("Contract duration, difficulty or experience is invalid.".into());
    }
    Ok(contracts)
}
