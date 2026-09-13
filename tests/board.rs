//! Contract board migration and dated-offer regression rules.

use occupational_hazard::{contracts, services::Purchase, simulation::Guild};

fn reload(g: &Guild, qs: &[contracts::Contract]) -> Guild {
    let mut loaded: Guild = macroquad_toolkit::data_loader::parse_json_labeled(
        "board test",
        &serde_json::to_string(g).unwrap(),
    )
    .unwrap();
    loaded.migrate_board(qs).unwrap();
    loaded.validate(qs).unwrap();
    loaded
}

#[test]
fn expiry_is_last_acceptance_day_and_rewards_arrive_once_after_expiry() {
    let qs = contracts::load().unwrap();
    let mut g = Guild::new();
    g.day = 3;
    g.dispatch(2, &[1], &qs).unwrap();
    assert_eq!(g.expeditions[0].instance, "beekeeper@1");
    assert!(g.dispatch(2, &[0], &qs).is_err());
    g = reload(&g, &qs);
    g.next_day(&qs);
    assert!(!g.contract_open(2, &qs));
    assert_eq!(g.gold, 80);
    g.next_day(&qs);
    assert_eq!(g.gold, 128);
    assert_eq!(g.board.service_credit.len(), 1);
    g = reload(&g, &qs);
    g.next_day(&qs);
    assert_eq!(g.gold, 128);
    while g.day < 8 {
        g.next_day(&qs);
    }
    assert!(g.contract_open(2, &qs));
    assert_eq!(qs[2].offer(g.day).unwrap().id, "beekeeper@8");
    g.dispatch(2, &[1], &qs).unwrap();
    g.next_day(&qs);
    g.next_day(&qs);
    assert_eq!(g.completed[2], 2);
    assert_eq!(
        g.board.service_credit.len(),
        1,
        "repeat does not fill quota"
    );
}

#[test]
fn six_job_ledger_migration_preserves_away_staff_and_existing_records() {
    let qs = contracts::load().unwrap();
    let mut g = Guild::new();
    g.purchase(Purchase::Scout(3), &qs).unwrap();
    g.dispatch(2, &[1], &qs).unwrap();
    g.completed[0] = 3;
    g.completed.truncate(6);
    g.board = Default::default(); // Actual slice saves had no board field.
    g.services.infirmary = true;
    let mut restored = reload(&g, &qs);
    assert_eq!(restored.completed.len(), 12);
    assert_eq!(restored.completed[0], 3);
    assert_eq!(restored.services.scouted, vec![3]);
    assert!(restored.services.infirmary);
    assert_eq!(restored.expeditions[0].contract, 2);
    assert_eq!(restored.gold, 60);
    restored.next_day(&qs);
    restored.next_day(&qs);
    assert_eq!(restored.gold, 108);
    restored = reload(&restored, &qs);
    restored.next_day(&qs);
    assert_eq!(restored.completed[2], 1);
    assert_eq!(restored.gold, 108);
}

#[test]
fn content_reordering_preserves_completion_scouting_and_away_job_identity() {
    let mut qs = contracts::load().unwrap();
    let mut g = Guild::new();
    g.dispatch(2, &[1], &qs).unwrap();
    g.purchase(Purchase::Scout(3), &qs).unwrap();
    g.completed[0] = 2;
    qs.reverse();
    g = reload(&g, &qs);
    assert_eq!(qs[g.expeditions[0].contract].id, "beekeeper");
    assert_eq!(qs[g.services.scouted[0]].id, "well");
    assert_eq!(g.completed[11], 2);
    g.next_day(&qs);
    g.next_day(&qs);
    assert_eq!(g.gold, 108);
    assert_eq!(g.board.service_credit.len(), 1);
    assert!(g.validate(&qs).is_ok());
}

#[test]
fn failed_job_recovery_is_free_and_trial_remains_available() {
    let qs = contracts::load().unwrap();
    let mut g = Guild::new();
    g.gold = 0;
    g.dispatch(3, &[0], &qs).unwrap();
    for _ in 0..4 {
        g.next_day(&qs);
    }
    assert!(g.roster.iter().all(|a| a.injury == 0));
    assert!(g.contract_open(0, &qs));
    g.dispatch(0, &[2], &qs).unwrap();
    g.next_day(&qs);
    assert_eq!(g.gold, 24);
    assert!(g.board.service_credit.is_empty());
    for day in 1..=60 {
        g.day = day;
        assert!(qs[0].offer(day).is_some());
        assert!(g.contract_open(4, &qs));
        assert!(g.open_contracts(&qs).len() <= 10);
    }
}

#[test]
fn unknown_identities_are_rejected_instead_of_resetting_progress() {
    let qs = contracts::load().unwrap();
    let mut g = Guild::new();
    g.migrate_board(&qs).unwrap();
    g.board.definition_order[0] = "missing-job".into();
    assert!(g.migrate_board(&qs).is_err());
}

#[test]
fn month_schedule_supports_three_competing_assignment_decisions() {
    let qs = contracts::load().unwrap();
    let mut g = Guild::new();
    // Day 1: supporting the 65g well blocks three separate development jobs.
    assert!(g.dispatch_problem(3, &[0, 2], &qs).is_none());
    assert!(g.strength(&qs[3], &[0, 2]) >= qs[3].difficulty);
    for (job, person) in [(0, 0), (1, 2), (2, 1)] {
        g.dispatch(job, &[person], &qs).unwrap();
    }
    while g.day < 4 {
        g.next_day(&qs);
    }
    // Day 4: Tomas needs recovery; send the others and let the well lapse.
    assert!(!g.contract_open(3, &qs));
    assert!(g.roster[1].injury > 0);
    g.dispatch(6, &[0], &qs).unwrap();
    g.dispatch(10, &[2], &qs).unwrap();
    g.next_day(&qs);
    g.dispatch(8, &[1], &qs).unwrap();
    while g.day < 7 {
        g.next_day(&qs);
    }
    g.dispatch(7, &[0], &qs).unwrap();
    g.dispatch(11, &[2], &qs).unwrap();
    while g.day < 10 {
        g.next_day(&qs);
    }
    // Day 10: a trial and shepherd compete with another last-day well posting.
    assert!(g.contract_open(3, &qs));
    assert!(g.dispatch_problem(3, &[0, 2], &qs).is_none());
    g.dispatch(4, &[0], &qs).unwrap();
    g.dispatch(9, &[1], &qs).unwrap();
    g = reload(&g, &qs);
    g.next_day(&qs);
    g.next_day(&qs);
    g.promote(0).unwrap();
    while g.day < 14 {
        g.next_day(&qs);
    }
    g.dispatch(5, &[0, 1, 2], &qs).unwrap();
    while g.day < 30 {
        g.next_day(&qs);
    }
    let review = g.month.review.as_ref().unwrap();
    assert!(review.passed());
    assert_eq!(review.service_returns, 8);
    assert_eq!(review.gold, 582);
}

#[test]
fn scouting_survives_expiry_until_the_next_posting() {
    let qs = contracts::load().unwrap();
    let mut g = Guild::new();
    g.purchase(Purchase::Scout(3), &qs).unwrap();
    g = reload(&g, &qs);
    while g.day < 8 {
        g.next_day(&qs);
    }
    g.dispatch(3, &[0, 2], &qs).unwrap();
    assert_eq!(
        g.expeditions[0].strength,
        g.strength(&qs[3], &[0, 2]) + g.config.services.scout_strength_bonus
    );
    assert!(g.services.scouted.is_empty());
}
