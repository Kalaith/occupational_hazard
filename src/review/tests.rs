use super::*;

fn reload(g: &Guild) -> Guild {
    let json = serde_json::to_string(g).unwrap();
    let loaded: Guild =
        macroquad_toolkit::data_loader::parse_json_labeled("review test", &json).unwrap();
    loaded.validate(&crate::contracts::load().unwrap()).unwrap();
    loaded
}

#[test]
fn fresh_guild_can_pass_with_cutoff_return_then_reload_and_continue() {
    let qs = crate::contracts::load().unwrap();
    let mut g = Guild::new();
    for quest in [0, 6, 7] {
        g.dispatch(quest, &[0], &qs).unwrap();
        for _ in 0..3 {
            g.next_day(&qs);
        }
    }
    g.dispatch(4, &[0], &qs).unwrap();
    g.next_day(&qs);
    g.next_day(&qs);
    g.promote(0).unwrap();
    assert!(g.month.review.is_none(), "promotion is intermediate");
    while g.day < 27 {
        g.next_day(&qs);
    }
    g.dispatch(5, &[0, 1, 2], &qs).unwrap();
    g = reload(&g);
    for _ in 0..3 {
        g.next_day(&qs);
    }
    let r = g.month.review.as_ref().unwrap();
    assert!(r.passed());
    assert_eq!(r.gold, 362);
    assert_eq!(r.careers[0].successes, 5);
    let snapshot = serde_json::to_string(r).unwrap();
    g = reload(&g);
    g.next_day(&qs);
    assert_eq!(g.day, 30);
    assert!(g.dispatch(0, &[0], &qs).is_err());
    assert!(g
        .purchase(crate::services::Purchase::Infirmary, &qs)
        .is_err());
    g.continue_sandbox();
    g = reload(&g);
    g.next_day(&qs);
    assert_eq!(g.day, 31);
    assert_eq!(
        serde_json::to_string(g.month.review.as_ref().unwrap()).unwrap(),
        snapshot
    );
}

#[test]
fn late_work_and_missed_targets_do_not_softlock_or_rewrite_review() {
    let qs = crate::contracts::load().unwrap();
    let mut g = Guild::new();
    while g.day < 29 {
        g.next_day(&qs);
    }
    assert!(g.cutoff_notice(2).contains("too late"));
    assert!(!g.cutoff_notice(1).contains("too late"));
    g.dispatch(2, &[1], &qs).unwrap();
    g.next_day(&qs);
    assert!(!g.month.review.as_ref().unwrap().passed());
    assert_eq!(g.gold, 80);
    g = reload(&g);
    g.continue_sandbox();
    g.next_day(&qs);
    assert_eq!(g.gold, 128);
    assert_eq!(g.month.review.as_ref().unwrap().gold, 80);
    g.next_day(&qs);
    assert_eq!(g.gold, 128);
    let fresh = Guild::new();
    assert_eq!(fresh.day, 1);
    assert!(!fresh.month.sandbox);
    assert!(fresh.month.review.is_none());
}

#[test]
fn older_ledgers_retain_progress_and_receive_a_review_when_loaded() {
    let qs = crate::contracts::load().unwrap();
    let mut value = serde_json::to_value(Guild::new()).unwrap();
    value.as_object_mut().unwrap().remove("month");
    value["day"] = 45.into();
    let mut g: Guild =
        macroquad_toolkit::data_loader::parse_json_labeled("legacy", &value.to_string()).unwrap();
    g.validate(&qs).unwrap();
    g.finish_review(&qs);
    assert_eq!(g.month.review.as_ref().unwrap().day, 45);
    assert_eq!(reload(&g).gold, 80);
}

#[test]
fn invalid_review_state_is_rejected() {
    let mut g = Guild::new();
    g.month.sandbox = true;
    assert!(g.validate_month().is_err());
}
