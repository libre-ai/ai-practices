//! Anonymous cohort aggregation (ADR 0006).
//!
//! Pure functions — no storage, no clock — so the k-anonymity and retention
//! properties are exhaustively testable. A distribution for an axis is only
//! exposed when the cohort meets the minimum size `k`; below it, nothing partial
//! is ever returned. The scoring stays here (server engine), never in the UI
//! (ADR 0003).

use rumble_ai_practices_domain::{
    AxisLevel, DistributionBucket, DistributionPosition, PracticeLevel, RiskAxis,
};

/// Default minimum cohort size for k-anonymity (ADR 0006). A distribution is
/// withheld below this many sessions on an axis.
pub const DEFAULT_MIN_COHORT: usize = 5;

/// Default retention window in days for anonymous sessions (ADR 0006).
pub const DEFAULT_RETENTION_DAYS: i64 = 90;

const SECONDS_PER_DAY: i64 = 86_400;

/// The ordered practice bands a session can occupy on an axis.
const BANDS: [PracticeLevel; 4] = [
    PracticeLevel::Discovery,
    PracticeLevel::GuidedPractice,
    PracticeLevel::CarefulAutonomy,
    PracticeLevel::Reference,
];

fn band_label(level: PracticeLevel) -> &'static str {
    match level {
        PracticeLevel::Discovery => "découverte",
        PracticeLevel::GuidedPractice => "pratique guidée",
        PracticeLevel::CarefulAutonomy => "autonomie prudente",
        PracticeLevel::Reference => "référence",
    }
}

/// French label for a risk axis (the cohort context line).
fn axis_label(axis: RiskAxis) -> &'static str {
    match axis {
        RiskAxis::DataConfidentiality => "Confidentialité",
        RiskAxis::SourceVerification => "Sources",
        RiskAxis::Hallucination => "Fiabilité",
        RiskAxis::BiasFairness => "Biais & équité",
        RiskAxis::Security => "Sécurité",
        RiskAxis::PrivacyRgpd => "RGPD",
        RiskAxis::HumanResponsibility => "Responsabilité",
        RiskAxis::PromptPractice => "Sécurité du prompt",
        RiskAxis::BusinessContext => "Contexte métier",
        RiskAxis::MediaSynthetic => "Médias synthétiques",
    }
}

fn level_for_axis(levels: &[AxisLevel], axis: RiskAxis) -> Option<PracticeLevel> {
    levels.iter().find(|l| l.axis == axis).map(|l| l.level)
}

/// Compute the k-anonymous distribution for one axis.
///
/// `cohort` is every completed (anonymous) session's axis levels; `target` is
/// the session being situated. If fewer than `k` cohort members have answered
/// this axis, the distribution is withheld — `min_cohort_size_met: false`, no
/// buckets, no user bucket — so no partial aggregate is ever exposed (ADR 0006).
pub fn distribution_for_axis(
    cohort: &[Vec<AxisLevel>],
    target: &[AxisLevel],
    axis: RiskAxis,
    k: usize,
) -> DistributionPosition {
    let members: Vec<PracticeLevel> = cohort
        .iter()
        .filter_map(|levels| level_for_axis(levels, axis))
        .collect();

    let cohort_label = axis_label(axis).to_string();

    if members.len() < k {
        return DistributionPosition {
            cohort_label,
            min_cohort_size_met: false,
            buckets: Vec::new(),
            user_bucket: None,
        };
    }

    let total = members.len() as f64;
    let buckets = BANDS
        .iter()
        .map(|band| {
            let count = members.iter().filter(|level| *level == band).count();
            DistributionBucket {
                label: band_label(*band).to_string(),
                percent: (count as f64 / total) * 100.0,
            }
        })
        .collect();

    DistributionPosition {
        cohort_label,
        min_cohort_size_met: true,
        buckets,
        user_bucket: level_for_axis(target, axis)
            .map(band_label)
            .map(String::from),
    }
}

/// Whether an anonymous session has passed its retention window and must be
/// purged (ADR 0006). Pure: the current time and the window are passed in.
pub fn is_expired(completed_at_epoch_secs: i64, now_epoch_secs: i64, retention_days: i64) -> bool {
    let age = now_epoch_secs.saturating_sub(completed_at_epoch_secs);
    age > retention_days.saturating_mul(SECONDS_PER_DAY)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lvl(axis: RiskAxis, level: PracticeLevel) -> AxisLevel {
        AxisLevel {
            axis,
            level,
            score: 0.0,
        }
    }

    fn member(level: PracticeLevel) -> Vec<AxisLevel> {
        vec![lvl(RiskAxis::SourceVerification, level)]
    }

    #[test]
    fn below_k_withholds_the_distribution() {
        let cohort: Vec<Vec<AxisLevel>> = (0..DEFAULT_MIN_COHORT - 1)
            .map(|_| member(PracticeLevel::CarefulAutonomy))
            .collect();
        let target = member(PracticeLevel::CarefulAutonomy);
        let d = distribution_for_axis(
            &cohort,
            &target,
            RiskAxis::SourceVerification,
            DEFAULT_MIN_COHORT,
        );
        assert!(!d.min_cohort_size_met);
        assert!(d.buckets.is_empty(), "no partial aggregate below k");
        assert_eq!(d.user_bucket, None, "own band withheld below k");
    }

    #[test]
    fn at_k_exposes_the_distribution_summing_to_100() {
        let cohort: Vec<Vec<AxisLevel>> = (0..DEFAULT_MIN_COHORT)
            .map(|_| member(PracticeLevel::CarefulAutonomy))
            .collect();
        let target = member(PracticeLevel::Reference);
        let d = distribution_for_axis(
            &cohort,
            &target,
            RiskAxis::SourceVerification,
            DEFAULT_MIN_COHORT,
        );
        assert!(d.min_cohort_size_met);
        assert_eq!(d.buckets.len(), 4, "one bucket per band");
        let sum: f64 = d.buckets.iter().map(|b| b.percent).sum();
        assert!(
            (sum - 100.0).abs() < 1e-9,
            "percentages sum to 100, got {sum}"
        );
        assert_eq!(d.user_bucket.as_deref(), Some("référence"));
    }

    #[test]
    fn distribution_reflects_band_proportions() {
        // 3 at CarefulAutonomy, 2 at Discovery -> 60% / 40%
        let mut cohort = vec![member(PracticeLevel::CarefulAutonomy); 3];
        cohort.extend(vec![member(PracticeLevel::Discovery); 2]);
        let target = member(PracticeLevel::Discovery);
        let d = distribution_for_axis(&cohort, &target, RiskAxis::SourceVerification, 5);
        let get = |label: &str| d.buckets.iter().find(|b| b.label == label).unwrap().percent;
        assert!((get("autonomie prudente") - 60.0).abs() < 1e-9);
        assert!((get("découverte") - 40.0).abs() < 1e-9);
        assert!((get("référence") - 0.0).abs() < 1e-9);
    }

    #[test]
    fn deletion_preserves_k_anonymity() {
        // ADR 0006 mandated test: anonymity holds under deletion.
        let full: Vec<Vec<AxisLevel>> = (0..DEFAULT_MIN_COHORT + 1)
            .map(|_| member(PracticeLevel::CarefulAutonomy))
            .collect();
        let target = member(PracticeLevel::CarefulAutonomy);
        let axis = RiskAxis::SourceVerification;

        // k+1 -> exposed
        assert!(
            distribution_for_axis(&full, &target, axis, DEFAULT_MIN_COHORT).min_cohort_size_met
        );
        // delete one -> exactly k -> still exposed
        let at_k = &full[..DEFAULT_MIN_COHORT];
        assert!(distribution_for_axis(at_k, &target, axis, DEFAULT_MIN_COHORT).min_cohort_size_met);
        // delete another -> k-1 -> withheld, nothing leaks
        let below = &full[..DEFAULT_MIN_COHORT - 1];
        let d = distribution_for_axis(below, &target, axis, DEFAULT_MIN_COHORT);
        assert!(!d.min_cohort_size_met);
        assert!(d.buckets.is_empty());
    }

    #[test]
    fn an_unanswered_axis_is_withheld() {
        // cohort answered a different axis -> zero members on the target axis
        let cohort =
            vec![vec![lvl(RiskAxis::Security, PracticeLevel::Reference)]; DEFAULT_MIN_COHORT + 3];
        let target = member(PracticeLevel::Discovery);
        let d = distribution_for_axis(
            &cohort,
            &target,
            RiskAxis::SourceVerification,
            DEFAULT_MIN_COHORT,
        );
        assert!(!d.min_cohort_size_met);
        assert!(d.buckets.is_empty());
    }

    #[test]
    fn retention_expires_only_past_the_window() {
        let now = 1_000 * SECONDS_PER_DAY;
        // completed today -> not expired
        assert!(!is_expired(now, now, DEFAULT_RETENTION_DAYS));
        // completed exactly at the window edge -> not expired (strictly greater)
        let at_edge = now - DEFAULT_RETENTION_DAYS * SECONDS_PER_DAY;
        assert!(!is_expired(at_edge, now, DEFAULT_RETENTION_DAYS));
        // one second past the window -> expired
        assert!(is_expired(at_edge - 1, now, DEFAULT_RETENTION_DAYS));
    }
}
