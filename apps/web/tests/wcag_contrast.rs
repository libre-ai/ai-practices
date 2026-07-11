//! WCAG evidence comes from the canonical Portal Forge report, never mirrored
//! product-local color literals.

use serde_json::Value;
use std::{fs, path::PathBuf};

#[test]
fn libre_ia_contrast_report_passes() {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/libre-ia/contrast-report.json");
    let report: Value = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    assert_eq!(
        report["format"].as_str(),
        Some("portal.contrast_report.v0.1")
    );
    let checks = report["checks"].as_array().expect("contrast checks");
    assert!(checks.len() >= 8);
    assert!(
        checks
            .iter()
            .all(|check| check["passes_wcag_aa"].as_bool() == Some(true))
    );
}

#[test]
fn design_manifest_pins_v2() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/libre-ia/manifest.json");
    let manifest: Value = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    assert_eq!(manifest["version"].as_str(), Some("2.0.0"));
}

#[test]
fn pwa_assets_resolve_from_the_bundled_shell() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    assert_eq!(
        fs::read(root.join("assets/sw.js")).unwrap(),
        fs::read(root.join("public/sw.js")).unwrap()
    );

    let manifest: Value =
        serde_json::from_str(&fs::read_to_string(root.join("assets/manifest.json")).unwrap())
            .unwrap();
    assert_eq!(manifest["start_url"].as_str(), Some("../"));
    assert_eq!(manifest["scope"].as_str(), Some("../"));
    assert!(root.join("assets/media/rumble_asset_0304.webp").is_file());
}
