//! Saved tutorial progression and help coverage regressions.

use occupational_hazard::simulation::Guild;
use occupational_hazard::tutorial::*;

#[test]
fn guidance_follows_selection_dispatch_return_and_promotion() {
    let mut g = Guild::new();
    let qs = occupational_hazard::contracts::load().unwrap();
    assert_eq!(g.lesson(false), Some(Lesson::Welcome));
    g.tutorial.acknowledge(Lesson::Welcome);
    assert_eq!(g.lesson(false), Some(Lesson::Selection));
    g.tutorial.acknowledge(Lesson::Selection);
    assert_eq!(g.lesson(false), None);
    assert_eq!(g.lesson(true), Some(Lesson::Dispatch));
    g.tutorial.acknowledge(Lesson::Dispatch);
    g.dispatch(0, &[0], &qs).unwrap();
    g.tutorial.dispatched = true;
    assert_eq!(g.lesson(false), Some(Lesson::Time));
    g.tutorial.acknowledge(Lesson::Time);
    g.next_day(&qs);
    assert_eq!(g.lesson(false), Some(Lesson::Reports));
    g.tutorial.acknowledge(Lesson::Reports);
    assert_eq!(g.lesson(false), Some(Lesson::Recovery));
    g.tutorial.acknowledge(Lesson::Recovery);
    assert_eq!(g.lesson(false), None);
    g.roster[0].xp = 60;
    g.roster[0].successes = 3;
    assert_eq!(g.lesson(false), Some(Lesson::Trial));
    g.tutorial.acknowledge(Lesson::Trial);
    g.roster[0].trial_passed = true;
    assert_eq!(g.lesson(false), Some(Lesson::Promotion));
}

#[test]
fn skip_and_acknowledgements_survive_reload_without_hiding_help() {
    let mut g = Guild::new();
    g.tutorial.acknowledge(Lesson::Welcome);
    g.tutorial.skipped = true;
    let json = serde_json::to_string(&g).unwrap();
    let loaded: Guild =
        macroquad_toolkit::data_loader::parse_json_labeled("tutorial", &json).unwrap();
    assert_eq!(loaded.lesson(true), None);
    assert!(loaded.tutorial.has_seen(Lesson::Welcome));
    assert_eq!(LESSONS.len(), 8);
    for lesson in LESSONS {
        assert!(lesson.text(&loaded).contains("BACK TO HEADQUARTERS"));
    }
}
