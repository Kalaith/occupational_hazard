//! Facility, training, and scouting purchase regressions.

use occupational_hazard::{contracts, services::Purchase, simulation::Guild};

#[test]
fn purchases_charge_once_and_reject_unaffordable_orders() {
    let qs = contracts::load().unwrap();
    let mut g = Guild::new();
    assert!(g.purchase(Purchase::Infirmary, &qs).is_err());
    assert_eq!(g.gold, 80);
    assert!(!g.services.infirmary);
    g.gold = 300;
    g.purchase(Purchase::Infirmary, &qs).unwrap();
    assert_eq!(g.gold, 200);
    assert!(g.purchase(Purchase::Infirmary, &qs).is_err());
    assert_eq!(g.gold, 200);
    g.purchase(Purchase::TrainingYard, &qs).unwrap();
    assert_eq!(g.gold, 60);
}

#[test]
fn infirmary_and_training_apply_only_to_eligible_staff_at_home() {
    let qs = contracts::load().unwrap();
    let mut g = Guild::new();
    g.services.infirmary = true;
    g.services.training_yard = true;
    g.roster[0].injury = 2;
    g.roster[1].fatigue = 2;
    g.dispatch(1, &[2], &qs).unwrap();
    g.next_day(&qs);
    assert_eq!(g.roster[0].injury, 0);
    assert_eq!(g.roster[0].xp, 0);
    assert_eq!(g.roster[1].xp, 0);
    assert_eq!(g.roster[2].xp, qs[1].xp);
    g.next_day(&qs);
    assert_eq!(g.roster[0].xp, 5);
    assert_eq!(g.roster[1].xp, 5);
    g.roster[0].xp = 59;
    g.roster[1].xp = 90;
    g.next_day(&qs);
    assert_eq!(g.roster[0].xp, 60);
    assert_eq!(g.roster[1].xp, 90);
    assert!(!g.roster[0].eligible(&g.config));
    assert!(g.dispatch(4, &[0], &qs).is_err());
}

#[test]
fn scouting_is_consumed_once_by_its_route_and_preserves_trial_rules() {
    let qs = contracts::load().unwrap();
    let mut g = Guild::new();
    assert!(g.purchase(Purchase::Scout(4), &qs).is_err());
    assert!(g.purchase(Purchase::Scout(99), &qs).is_err());
    g.purchase(Purchase::Scout(2), &qs).unwrap();
    assert_eq!(g.gold, 60);
    assert!(g.purchase(Purchase::Scout(2), &qs).is_err());
    assert!(g.dispatch(2, &[], &qs).is_err());
    assert!(g.services.scouted.contains(&2));
    let base = g.strength(&qs[2], &[1]);
    g.dispatch(0, &[0], &qs).unwrap();
    assert!(g.services.scouted.contains(&2));
    g.dispatch(2, &[1], &qs).unwrap();
    assert_eq!(
        g.expeditions[1].strength,
        base + g.config.services.scout_strength_bonus
    );
    assert!(g.services.scouted.is_empty());
    assert!(g.purchase(Purchase::Scout(2), &qs).is_err());
    g.next_day(&qs);
    g.next_day(&qs);
    assert_eq!(g.roster[1].injury, 0);
    assert_eq!(
        g.prepared_strength(2, &qs[2], &[1]),
        g.strength(&qs[2], &[1])
    );
}

#[test]
fn existing_saves_load_without_services_and_new_saves_preserve_purchases() {
    let qs = contracts::load().unwrap();
    let mut g = Guild::new();
    let mut old = serde_json::to_value(&g).unwrap();
    old.as_object_mut().unwrap().remove("services");
    let loaded: Guild = macroquad_toolkit::data_loader::parse_json(&old.to_string()).unwrap();
    loaded.validate(&qs).unwrap();
    assert!(!loaded.services.infirmary);
    assert!(loaded.services.scouted.is_empty());
    g.services.infirmary = true;
    g.purchase(Purchase::Scout(3), &qs).unwrap();
    let mut loaded: Guild =
        macroquad_toolkit::data_loader::parse_json(&serde_json::to_string(&g).unwrap()).unwrap();
    loaded.validate(&qs).unwrap();
    assert!(loaded.services.infirmary);
    assert_eq!(loaded.services.scouted, vec![3]);
    loaded.services.scouted.push(3);
    assert!(loaded.validate(&qs).is_err());
}
