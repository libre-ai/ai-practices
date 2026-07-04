//! WCAG 1.4.3 contrast gate for the verdict *ink* tones.
//!
//! The verdict text colors live in `assets/tokens.css` (`--color-<v>-ink`,
//! overridden per theme). CSS can't be asserted in Rust, so these hex values
//! MIRROR that file — keep the two in sync. Each ink must reach AA (≥ 4.5:1)
//! against its theme ground (`--color-surface`, the console/summary surface the
//! verdict text sits on).

/// sRGB relative luminance per WCAG 2.x.
fn luminance(hex: &str) -> f64 {
    let n = hex.trim_start_matches('#');
    let channel = |i: usize| u8::from_str_radix(&n[i..i + 2], 16).unwrap() as f64 / 255.0;
    let lin = |c: f64| {
        if c <= 0.039_28 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * lin(channel(0)) + 0.7152 * lin(channel(2)) + 0.0722 * lin(channel(4))
}

/// WCAG contrast ratio between two colors (≥ 1.0).
fn contrast(a: &str, b: &str) -> f64 {
    let (la, lb) = (luminance(a), luminance(b));
    let (hi, lo) = if la >= lb { (la, lb) } else { (lb, la) };
    (hi + 0.05) / (lo + 0.05)
}

#[test]
fn verdict_ink_meets_aa_on_both_theme_grounds() {
    // ground = --color-surface (dark / light), mirrored from tokens.css
    let dark_ground = "#1e222c";
    let light_ground = "#e2dbc9";

    // (verdict, ink, ground) — mirrors --color-<v>-ink for each theme
    let pairs = [
        ("juste/dark", "#6fc7ab", dark_ground),
        ("partiel/dark", "#e0bd6a", dark_ground),
        ("risque/dark", "#e08974", dark_ground),
        ("faux/dark", "#b193c4", dark_ground),
        ("juste/light", "#1a5c43", light_ground),
        ("partiel/light", "#5f480e", light_ground),
        ("risque/light", "#992c19", light_ground),
        ("faux/light", "#5a3f73", light_ground),
    ];

    let mut failures = Vec::new();
    for (name, ink, ground) in pairs {
        let ratio = contrast(ink, ground);
        println!("{name}: {ink} on {ground} = {ratio:.2}:1");
        if ratio < 4.5 {
            failures.push(format!("{name} = {ratio:.2}:1 (< 4.5 AA)"));
        }
    }
    assert!(failures.is_empty(), "verdict ink below AA: {failures:?}");
}
