//! Shipped balance configuration loads through the toolkit and rejects invalid edits.

use occupational_hazard::data::GameConfig;

#[test]
fn shipped_configuration_has_the_authored_start_and_thresholds() {
    let config = GameConfig::load().expect("shipped game config should validate");
    assert_eq!(config.roster.len(), 3);
    assert_eq!(config.starting.gold, 80);
    assert_eq!(config.progression.trial_xp, 60);
    assert_eq!(config.review.service_quota, 6);
}

#[test]
fn configuration_validation_rejects_duplicate_roster_identity() {
    let mut config = GameConfig::load().expect("shipped game config should validate");
    config.roster[1].name = config.roster[0].name.clone();
    let error = config
        .validate()
        .expect_err("duplicate names must be rejected");
    assert!(error.contains("roster"));
}
