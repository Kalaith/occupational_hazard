use super::*;

fn rest(g: &mut Guild, qs: &[Contract]) {
    while g.roster.iter().any(|a| a.fatigue > 0 || a.injury > 0) || !g.expeditions.is_empty() {
        g.next_day(qs);
    }
}

#[test]
fn iron_to_bronze_and_first_commission_are_playable() {
    let qs = crate::contracts::load().unwrap();
    let mut g = Guild::new();
    assert!(g.promote(0).is_err());
    assert!(g.dispatch(4, &[0], &qs).is_err());
    for _ in 0..3 {
        g.dispatch(0, &[0], &qs).unwrap();
        g.next_day(&qs);
        rest(&mut g, &qs);
    }
    assert!(g.roster[0].eligible());
    assert_eq!(g.roster[0].xp, 60);
    assert!(g.promote(0).is_err());
    assert!(g.dispatch(4, &[0, 1], &qs).is_err());
    g.dispatch(4, &[0], &qs).unwrap();
    g.next_day(&qs);
    assert!(!g.roster[0].trial_passed);
    assert!(g.promote(0).is_err());
    g.next_day(&qs);
    assert!(g.roster[0].trial_passed);
    assert!(!g.roster[0].bronze);
    g.promote(0).unwrap();
    assert!(g.roster[0].bronze);
    assert!(g.promote(0).is_err());
    rest(&mut g, &qs);
    g.dispatch(5, &[0, 1, 2], &qs).unwrap();
    for _ in 0..3 {
        g.next_day(&qs);
    }
    assert_eq!(g.completed[5], 1);
    assert_eq!(g.gold, 80 + 72 + 80 + 120);
}

#[test]
fn no_double_booking_or_early_rewards() {
    let qs = crate::contracts::load().unwrap();
    let mut g = Guild::new();
    assert!(g.dispatch(0, &[], &qs).is_err());
    assert!(g.dispatch(0, &[0, 0], &qs).is_err());
    assert!(g.dispatch(99, &[0], &qs).is_err());
    g.dispatch(2, &[1], &qs).unwrap();
    assert!(g.dispatch(0, &[1], &qs).is_err());
    assert!(g.dispatch(2, &[0], &qs).is_err());
    g.next_day(&qs);
    assert_eq!(g.gold, 80);
    g.next_day(&qs);
    assert_eq!(g.gold, 128);
    g.next_day(&qs);
    assert_eq!(g.gold, 128);
}

#[test]
fn poor_assignment_retreats_and_recovers_without_a_softlock() {
    let qs = crate::contracts::load().unwrap();
    let mut g = Guild::new();
    g.dispatch(3, &[2], &qs).unwrap();
    g.next_day(&qs);
    g.next_day(&qs);
    assert_eq!(g.completed[3], 0);
    assert_eq!(g.gold, 80);
    assert!(g.roster[2].injury > 0);
    assert!(g.dispatch(1, &[2], &qs).is_err());
    rest(&mut g, &qs);
    g.dispatch(1, &[2], &qs).unwrap();
    g.next_day(&qs);
    assert_eq!(g.completed[1], 1);
}

#[test]
fn save_round_trip_preserves_expedition_and_certification() {
    let qs = crate::contracts::load().unwrap();
    let mut g = Guild::new();
    g.roster[0].trial_passed = true;
    g.roster[0].xp = 90;
    g.roster[0].successes = 4;
    g.dispatch(2, &[1], &qs).unwrap();
    let json = serde_json::to_string(&g).unwrap();
    let mut loaded: Guild =
        macroquad_toolkit::data_loader::parse_json_labeled("save round trip", &json).unwrap();
    loaded.validate(&qs).unwrap();
    loaded.promote(0).unwrap();
    loaded.next_day(&qs);
    loaded.next_day(&qs);
    assert!(loaded.roster[0].bronze);
    assert_eq!(loaded.completed[2], 1);
    loaded.expeditions.push(Expedition {
        contract: 100,
        party: vec![0],
        returns: 9,
        strength: 0,
    });
    assert!(loaded.validate(&qs).is_err());
}

#[test]
fn each_class_can_earn_bronze_through_its_specialty() {
    let qs = crate::contracts::load().unwrap();
    for (candidate, quest) in [(0, 0), (1, 2), (2, 1)] {
        let mut g = Guild::new();
        for _ in 0..3 {
            g.dispatch(quest, &[candidate], &qs).unwrap();
            rest(&mut g, &qs);
        }
        g.dispatch(4, &[candidate], &qs).unwrap();
        rest(&mut g, &qs);
        g.promote(candidate).unwrap();
        assert!(g.roster[candidate].bronze);
    }
}
