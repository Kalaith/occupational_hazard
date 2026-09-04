//! Authored contracts for the first guild licence.
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Contract {
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
    if contracts.len() != 6 || contracts.iter().filter(|q| q.promotion).count() != 1 {
        return Err("The Iron licence needs six contracts and one promotion assessment.".into());
    }
    if contracts
        .iter()
        .any(|q| q.days == 0 || q.days > 3 || q.difficulty < 1 || q.xp == 0)
    {
        return Err("Contract duration, difficulty or experience is invalid.".into());
    }
    Ok(contracts)
}
