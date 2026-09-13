//! The shipped text catalog is complete, formatted, and rejects missing copy.

use occupational_hazard::data::TextCatalog;

#[test]
fn shipped_catalog_has_required_copy_and_formats_values() {
    let catalog = TextCatalog::load().expect("shipped text catalog must load");
    assert!(catalog.get("ui.dispatch_party").contains("DISPATCH"));
    assert_eq!(
        catalog.format("ui.review_in", &[("days", "7".into())]),
        "Review in 7 days"
    );
}

#[test]
fn catalog_validation_rejects_missing_required_copy() {
    let error = TextCatalog::from_json(r#"{"title.name":"Only one entry"}"#)
        .expect_err("an incomplete catalog must be rejected");
    assert!(error.contains("missing required text key"));
}
