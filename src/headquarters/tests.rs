use super::*;
use crate::services::Purchase;

#[test]
fn rapid_day_inputs_do_not_consume_two_days() {
    let mut hq = Headquarters::default();
    let qs = crate::contracts::load().unwrap();
    let mut guild = Guild::new();
    for _ in 0..2 {
        if hq.begin_day_change() {
            guild.next_day(&qs);
        }
    }
    assert_eq!(guild.day, 2);
    hq.open(Sheet::Returns);
    hq.tick(0.1);
    assert!(!hq.begin_day_change());
    hq.tick(0.2);
    assert!(hq.begin_day_change());
    guild.next_day(&qs);
    assert_eq!(guild.day, 3);
}

#[test]
fn placement_precedence_tracks_dispatch_rest_and_purchase() {
    let qs = crate::contracts::load().unwrap();
    let mut g = Guild::new();
    assert_eq!(activity(&g, 0), Activity::Ready);
    g.gold = 500;
    g.purchase(Purchase::TrainingYard, &qs).unwrap();
    assert_eq!(activity(&g, 0), Activity::Training);
    g.roster[0].fatigue = 2;
    assert_eq!(activity(&g, 0), Activity::Resting);
    g.roster[0].injury = 1;
    assert_eq!(activity(&g, 0), Activity::Recovering);
    g.roster[0].injury = 0;
    g.dispatch(0, &[0], &qs).unwrap();
    assert_eq!(activity(&g, 0), Activity::Away);
    assert_eq!(activity(&g, 0).room(), None);
    g.next_day(&qs);
    assert_eq!(activity(&g, 0), Activity::Resting);
    g.roster[0].fatigue = 0;
    g.roster[0].xp = 60;
    assert_eq!(activity(&g, 0), Activity::Ready);
    g.roster[0].xp = 20;
    g.roster[0].bronze = true;
    assert_eq!(activity(&g, 0), Activity::Ready);
}

#[test]
fn two_journeys_reload_and_interruption_do_not_duplicate_rewards() {
    let qs = crate::contracts::load().unwrap();
    let mut g = Guild::new();
    g.dispatch(0, &[0], &qs).unwrap();
    g.dispatch(2, &[1], &qs).unwrap();
    assert!(g.dispatch(0, &[0], &qs).is_err());
    let mut hq = Headquarters {
        transition: Some(Transition {
            people: vec![0],
            arriving: false,
            elapsed: 0.,
        }),
        ..Default::default()
    };
    hq.tick(2.);
    assert!(hq.transition.is_none());
    assert_eq!(g.day, 1);
    assert_eq!(g.gold, 80);
    g.next_day(&qs);
    assert_eq!(g.expeditions.len(), 1);
    let paid = g.gold;
    let json = serde_json::to_string(&g).unwrap();
    let mut loaded: Guild =
        macroquad_toolkit::data_loader::parse_json_labeled("headquarters reload", &json).unwrap();
    loaded.validate(&qs).unwrap();
    loaded.read_report(0);
    loaded.read_report(0);
    assert_eq!(loaded.gold, paid);
    assert_eq!(activity(&loaded, 1), Activity::Away);
    loaded.next_day(&qs);
    assert!(loaded.expeditions.is_empty());
    assert_eq!(loaded.reports.len(), 2);
    let paid = loaded.gold;
    loaded.read_report(0);
    assert_eq!(loaded.gold, paid);
    assert_eq!(loaded.unread_reports(), 0);
}

#[test]
fn old_save_reconstructs_facilities_partial_career_and_unread_return() {
    let qs = crate::contracts::load().unwrap();
    let mut g = Guild::new();
    g.gold = 500;
    g.purchase(Purchase::Infirmary, &qs).unwrap();
    g.purchase(Purchase::TrainingYard, &qs).unwrap();
    g.roster[2].xp = 60;
    g.roster[2].successes = 3;
    g.roster[2].trial_passed = true;
    g.dispatch(0, &[0], &qs).unwrap();
    g.next_day(&qs);
    let value = serde_json::to_value(&g).unwrap();
    assert!(
        value.get("hq").is_none(),
        "presentation must not alter the guild schema"
    );
    let restored: Guild =
        macroquad_toolkit::data_loader::parse_json_labeled("pre-cutaway guild", &value.to_string())
            .unwrap();
    restored.validate(&qs).unwrap();
    assert!(restored.services.infirmary && restored.services.training_yard);
    assert!(restored.roster[2].trial_passed);
    assert_eq!(restored.unread_reports(), 1);
    assert_eq!(activity(&restored, 0), Activity::Resting);
    assert_eq!(activity(&restored, 1), Activity::Training);
}

#[test]
fn closing_planning_and_reduced_motion_only_change_presentation() {
    let g = Guild::new();
    let before = serde_json::to_string(&g).unwrap();
    let mut hq = Headquarters::default();
    hq.open(Sheet::Jobs);
    hq.open(Sheet::None);
    hq.reduced_motion = true;
    hq.transition = Some(Transition {
        people: vec![2],
        arriving: true,
        elapsed: 0.,
    });
    hq.tick(0.);
    assert!(hq.transition.is_none());
    assert_eq!(before, serde_json::to_string(&g).unwrap());
}
