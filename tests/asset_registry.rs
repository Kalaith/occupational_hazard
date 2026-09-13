//! Registered runtime assets and the typed texture manifest stay in agreement.

use occupational_hazard::data::TextureManifest;
use serde::Deserialize;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
struct AssetRegistry {
    version: u32,
    assets: Vec<String>,
}

#[test]
fn asset_registry_contains_external_texture_manifest_paths() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let registry_json = fs::read_to_string(root.join("asset_registry.json"))
        .expect("asset_registry.json must be readable");
    let registry: AssetRegistry =
        macroquad_toolkit::data_loader::parse_json_labeled("asset_registry.json", &registry_json)
            .expect("asset registry must be valid JSON");
    assert_eq!(registry.version, 1);

    let registered: BTreeSet<&str> = registry.assets.iter().map(String::as_str).collect();

    let manifest_json = fs::read_to_string(root.join("assets/data/texture_manifest.json"))
        .expect("texture manifest must be readable");
    let manifest =
        TextureManifest::from_json(&manifest_json).expect("texture manifest must validate");
    manifest
        .validate_paths(root)
        .expect("texture manifest asset references must exist");
    let expected: BTreeSet<&str> = manifest
        .entries
        .iter()
        .map(|entry| entry.path.as_str())
        .collect();

    let missing: Vec<&str> = expected.difference(&registered).copied().collect();
    assert!(
        missing.is_empty(),
        "texture manifest paths missing from asset registry: {missing:?}"
    );
    for relative in registered {
        assert!(
            root.join(relative).is_file(),
            "registered runtime asset is missing: {relative}"
        );
    }
}
