//! Persistence for the anonymous cohort (ADR 0006).
//!
//! The k-anonymity and retention *logic* lives in `session::cohort` (pure and
//! exhaustively tested). This crate only stores and fetches — with compile-time
//! checked SQL — and records access to the cohort API as audit events (never
//! user data). The schema holds **no nominative field**: a session is an opaque
//! id and its per-axis practice levels.

use rumble_ai_practices_domain::{AxisLevel, DistributionPosition, PracticeLevel, RiskAxis};
use rumble_ai_practices_session::cohort;
use sqlx::PgPool;

/// Embedded, versioned migrations (run via `MIGRATOR.run(&pool)` or `sqlx::test`).
pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),
}

/// One session's per-axis outcome, as persisted.
#[derive(Debug, Clone, PartialEq)]
pub struct AxisOutcome {
    pub axis: RiskAxis,
    pub level: PracticeLevel,
    pub score: f64,
}

fn axis_to_db(axis: RiskAxis) -> &'static str {
    match axis {
        RiskAxis::DataConfidentiality => "data_confidentiality",
        RiskAxis::SourceVerification => "source_verification",
        RiskAxis::Hallucination => "hallucination",
        RiskAxis::BiasFairness => "bias_fairness",
        RiskAxis::Security => "security",
        RiskAxis::PrivacyRgpd => "privacy_rgpd",
        RiskAxis::HumanResponsibility => "human_responsibility",
        RiskAxis::PromptPractice => "prompt_practice",
        RiskAxis::BusinessContext => "business_context",
        RiskAxis::MediaSynthetic => "media_synthetic",
    }
}

fn level_to_db(level: PracticeLevel) -> &'static str {
    match level {
        PracticeLevel::Discovery => "discovery",
        PracticeLevel::GuidedPractice => "guided_practice",
        PracticeLevel::CarefulAutonomy => "careful_autonomy",
        PracticeLevel::Reference => "reference",
    }
}

fn level_from_db(raw: &str) -> Option<PracticeLevel> {
    match raw {
        "discovery" => Some(PracticeLevel::Discovery),
        "guided_practice" => Some(PracticeLevel::GuidedPractice),
        "careful_autonomy" => Some(PracticeLevel::CarefulAutonomy),
        "reference" => Some(PracticeLevel::Reference),
        _ => None,
    }
}

/// Persist one anonymous session and its per-axis levels (transactional).
pub async fn insert_session(
    pool: &PgPool,
    session_id: &str,
    completed_at: i64,
    created_at: i64,
    axes: &[AxisOutcome],
) -> Result<(), StoreError> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "insert into anonymous_sessions (session_id, completed_at, created_at) values ($1, $2, $3)",
        session_id,
        completed_at,
        created_at,
    )
    .execute(&mut *tx)
    .await?;
    for outcome in axes {
        sqlx::query!(
            "insert into anonymous_session_axes (session_id, axis, level, score) values ($1, $2, $3, $4)",
            session_id,
            axis_to_db(outcome.axis),
            level_to_db(outcome.level),
            outcome.score,
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    Ok(())
}

/// Every cohort member's practice level on one axis.
pub async fn cohort_levels(
    pool: &PgPool,
    axis: RiskAxis,
) -> Result<Vec<PracticeLevel>, StoreError> {
    let rows = sqlx::query_scalar!(
        "select level from anonymous_session_axes where axis = $1",
        axis_to_db(axis),
    )
    .fetch_all(pool)
    .await?;
    Ok(rows.iter().filter_map(|raw| level_from_db(raw)).collect())
}

/// Purge sessions past the retention window; returns the number deleted. The
/// cutoff mirrors `cohort::is_expired` (strictly older than the window).
pub async fn purge_expired(
    pool: &PgPool,
    now: i64,
    retention_days: i64,
) -> Result<u64, StoreError> {
    let cutoff = now.saturating_sub(retention_days.saturating_mul(86_400));
    let result = sqlx::query!(
        "delete from anonymous_sessions where completed_at < $1",
        cutoff,
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}

/// Compute the k-anonymous distribution for an axis from the stored cohort, and
/// record the access as an audit event (event only, no user data).
pub async fn distribution(
    pool: &PgPool,
    axis: RiskAxis,
    target_level: Option<PracticeLevel>,
    k: usize,
    now: i64,
) -> Result<DistributionPosition, StoreError> {
    let levels = cohort_levels(pool, axis).await?;
    let cohort: Vec<Vec<AxisLevel>> = levels
        .iter()
        .map(|level| {
            vec![AxisLevel {
                axis,
                level: *level,
                score: 0.0,
            }]
        })
        .collect();
    let target: Vec<AxisLevel> = target_level
        .map(|level| {
            vec![AxisLevel {
                axis,
                level,
                score: 0.0,
            }]
        })
        .unwrap_or_default();

    let position = cohort::distribution_for_axis(&cohort, &target, axis, k);

    sqlx::query!(
        "insert into cohort_access_audit (accessed_at, axis, cohort_size, threshold_met) values ($1, $2, $3, $4)",
        now,
        axis_to_db(axis),
        levels.len() as i32,
        position.min_cohort_size_met,
    )
    .execute(pool)
    .await?;

    Ok(position)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rumble_ai_practices_session::cohort::{DEFAULT_MIN_COHORT, DEFAULT_RETENTION_DAYS};

    fn outcome(axis: RiskAxis, level: PracticeLevel) -> AxisOutcome {
        AxisOutcome {
            axis,
            level,
            score: 1.0,
        }
    }

    async fn seed(
        pool: &PgPool,
        axis: RiskAxis,
        level: PracticeLevel,
        n: usize,
        completed_at: i64,
    ) {
        for i in 0..n {
            let id = format!("s-{level:?}-{completed_at}-{i}");
            insert_session(
                pool,
                &id,
                completed_at,
                completed_at,
                &[outcome(axis, level)],
            )
            .await
            .unwrap();
        }
    }

    #[sqlx::test]
    async fn insert_then_cohort_levels_roundtrips(pool: PgPool) {
        let axis = RiskAxis::SourceVerification;
        seed(&pool, axis, PracticeLevel::CarefulAutonomy, 3, 1000).await;
        let levels = cohort_levels(&pool, axis).await.unwrap();
        assert_eq!(levels.len(), 3);
        assert!(levels.iter().all(|l| *l == PracticeLevel::CarefulAutonomy));
    }

    #[sqlx::test]
    async fn distribution_withholds_below_k_and_audits(pool: PgPool) {
        let axis = RiskAxis::SourceVerification;
        seed(
            &pool,
            axis,
            PracticeLevel::CarefulAutonomy,
            DEFAULT_MIN_COHORT - 1,
            1000,
        )
        .await;
        let d = distribution(
            &pool,
            axis,
            Some(PracticeLevel::CarefulAutonomy),
            DEFAULT_MIN_COHORT,
            2000,
        )
        .await
        .unwrap();
        assert!(!d.min_cohort_size_met);
        assert!(d.buckets.is_empty());
        // the access was audited, with the withheld flag
        let (size, met): (i32, bool) = sqlx::query_as(
            "select cohort_size, threshold_met from cohort_access_audit order by id desc limit 1",
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(size, (DEFAULT_MIN_COHORT - 1) as i32);
        assert!(!met);
    }

    #[sqlx::test]
    async fn distribution_exposes_at_k(pool: PgPool) {
        let axis = RiskAxis::SourceVerification;
        seed(
            &pool,
            axis,
            PracticeLevel::CarefulAutonomy,
            DEFAULT_MIN_COHORT,
            1000,
        )
        .await;
        let d = distribution(
            &pool,
            axis,
            Some(PracticeLevel::CarefulAutonomy),
            DEFAULT_MIN_COHORT,
            2000,
        )
        .await
        .unwrap();
        assert!(d.min_cohort_size_met);
        let sum: f64 = d.buckets.iter().map(|b| b.percent).sum();
        assert!((sum - 100.0).abs() < 1e-9);
        assert_eq!(d.user_bucket.as_deref(), Some("autonomie prudente"));
    }

    #[sqlx::test]
    async fn purge_removes_only_expired(pool: PgPool) {
        let axis = RiskAxis::Security;
        let now = 1_000_000i64;
        let window = DEFAULT_RETENTION_DAYS * 86_400;
        // old (expired) + recent (kept)
        seed(&pool, axis, PracticeLevel::Discovery, 2, now - window - 10).await;
        seed(&pool, axis, PracticeLevel::Reference, 3, now).await;
        let deleted = purge_expired(&pool, now, DEFAULT_RETENTION_DAYS)
            .await
            .unwrap();
        assert_eq!(deleted, 2);
        let remaining = cohort_levels(&pool, axis).await.unwrap();
        assert_eq!(remaining.len(), 3, "only the recent sessions remain");
    }

    #[sqlx::test]
    async fn schema_holds_no_nominative_column(pool: PgPool) {
        // structural anonymity check (ADR 0006): no column looks like PII
        let cols: Vec<String> = sqlx::query_scalar(
            "select column_name from information_schema.columns \
             where table_schema = 'public' and table_name like 'anonymous_%'",
        )
        .fetch_all(&pool)
        .await
        .unwrap();
        for c in &cols {
            let lc = c.to_lowercase();
            assert!(
                !["email", "name", "nom", "prenom", "user", "phone", "ip"]
                    .iter()
                    .any(|pii| lc.contains(pii)),
                "unexpected PII-looking column: {c}"
            );
        }
    }
}
