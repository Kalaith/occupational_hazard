use crate::{contracts, services::Purchase, simulation::Guild};

#[test]
fn completed_jobs_leave_board_but_failed_jobs_remain() {
    let qs = contracts::load().unwrap();
    let mut g = Guild::new();
    g.dispatch(0, &[0], &qs).unwrap();
    g.dispatch(3, &[2], &qs).unwrap();
    g.next_day(&qs);
    g.next_day(&qs);
    assert!(!g.open_contracts(&qs).contains(&0));
    assert!(g.open_contracts(&qs).contains(&3));
    assert!(g.dispatch(0, &[1], &qs).is_err());
    assert!(g.purchase(Purchase::Scout(0), &qs).is_err());
    assert_ne!(g.adjacent_contract(11, true, &qs), 0);
    let json = serde_json::to_string(&g).unwrap();
    let restored: Guild =
        macroquad_toolkit::data_loader::parse_json_labeled("completed board", &json).unwrap();
    assert!(!restored.contract_open(0, &qs));
    assert!(restored.contract_open(3, &qs));
}

#[test]
fn six_job_ledger_migration_preserves_away_staff_and_existing_records() {
    let qs = contracts::load().unwrap();
    let mut g = Guild::new();
    g.purchase(Purchase::Scout(3), &qs).unwrap();
    g.dispatch(2, &[1], &qs).unwrap();
    g.completed[0] = 3;
    g.completed.truncate(6);
    let json = serde_json::to_string(&g).unwrap();
    let mut restored: Guild =
        macroquad_toolkit::data_loader::parse_json_labeled("six jobs", &json).unwrap();
    restored.migrate_board(&qs);
    restored.validate(&qs).unwrap();
    assert_eq!(restored.completed.len(), 12);
    assert_eq!(restored.completed[0], 3);
    assert_eq!(restored.services.scouted, vec![3]);
    assert_eq!(restored.expeditions[0].contract, 2);
    assert_eq!(restored.gold, 60);
    restored.next_day(&qs);
    restored.next_day(&qs);
    assert_eq!(restored.gold, 108);
    assert!(!restored.contract_open(2, &qs));
    restored.migrate_board(&qs);
    assert_eq!(restored.completed[2], 1);
}

#[test]
fn trial_remains_an_appointment_for_other_candidates_then_board_can_empty() {
    let qs = contracts::load().unwrap();
    let mut g = Guild::new();
    g.completed.fill(1);
    g.roster[0].trial_passed = true;
    assert_eq!(g.open_contracts(&qs), vec![4]);
    for a in &mut g.roster {
        a.trial_passed = true;
    }
    assert!(g.open_contracts(&qs).is_empty());
    assert_eq!(g.adjacent_contract(4, true, &qs), 4);
}

#[test]
fn original_contract_slots_remain_compatible_with_existing_saves() {
    let qs = contracts::load().unwrap();
    let original = [
        "Cellar, Meet Sword",
        "Medicine Before Sundown",
        "The Missing Beekeeper",
        "Something in the Well",
        "The Lantern Road Trial",
        "A Bridge Worth Keeping",
    ];
    for (id, title) in original.iter().enumerate() {
        assert_eq!(&qs[id].title, title);
    }
}
