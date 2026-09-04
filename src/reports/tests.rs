use crate::{contracts, simulation::Guild};

#[test]
fn simultaneous_returns_keep_both_rewards_and_independent_unread_reports() {
    let qs = contracts::load().unwrap();
    let mut g = Guild::new();
    g.dispatch(0, &[0], &qs).unwrap();
    g.dispatch(1, &[2], &qs).unwrap();
    g.next_day(&qs);
    assert_eq!(g.gold, 134);
    assert_eq!(g.roster[0].xp, 20);
    assert_eq!(g.roster[2].xp, 22);
    assert_eq!(g.completed[0], 1);
    assert_eq!(g.completed[1], 1);
    assert_eq!(g.unread_reports(), 2);
    assert!(g.reports.iter().any(|r| r.title.contains(&qs[0].title)));
    assert!(g.reports.iter().any(|r| r.title.contains(&qs[1].title)));
    assert!(g.read_report(0));
    let json = serde_json::to_string(&g).unwrap();
    let mut restored: Guild =
        macroquad_toolkit::data_loader::parse_json_labeled("two returns", &json).unwrap();
    restored.validate(&qs).unwrap();
    assert_eq!(restored.unread_reports(), 1);
    assert!(restored.read_report(1));
    assert_eq!(restored.unread_reports(), 0);
    restored.next_day(&qs);
    assert_eq!(restored.gold, 134);
}

#[test]
fn later_return_does_not_hide_an_unread_earlier_return() {
    let qs = contracts::load().unwrap();
    let mut g = Guild::new();
    g.dispatch(0, &[0], &qs).unwrap();
    g.dispatch(2, &[1], &qs).unwrap();
    g.next_day(&qs);
    assert_eq!(g.unread_reports(), 1);
    g.next_day(&qs);
    assert_eq!(g.unread_reports(), 2);
    assert_eq!(g.gold, 152);
}
