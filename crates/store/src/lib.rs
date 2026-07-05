//! Persistence for the anonymous cohort (ADR 0006).
//!
//! The k-anonymity and retention *logic* lives in `session::cohort` (pure and
//! exhaustively tested). This crate only stores and fetches — with compile-time
//! checked SQL — and records access to the cohort API as audit events (never
//! user data). The schema holds **no nominative field**: a session is an opaque
//! id and its per-axis practice levels.

use rumble_ai_practices_domain::{
    AxisLevel, DistributionPosition, PracticeLevel, QuestionChoiceShare, QuestionDistribution,
    RiskAxis,
};
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
/// Idempotent on `session_id`: re-persisting the same completed session is a
/// no-op, so a replayed summary request never double-counts a learner.
pub async fn insert_session(
    pool: &PgPool,
    session_id: &str,
    completed_at: i64,
    created_at: i64,
    axes: &[AxisOutcome],
) -> Result<(), StoreError> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "insert into anonymous_sessions (session_id, completed_at, created_at) values ($1, $2, $3) \
         on conflict (session_id) do nothing",
        session_id,
        completed_at,
        created_at,
    )
    .execute(&mut *tx)
    .await?;
    for outcome in axes {
        sqlx::query!(
            "insert into anonymous_session_axes (session_id, axis, level, score) values ($1, $2, $3, $4) \
             on conflict (session_id, axis) do nothing",
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

/// Purge everything past the retention window — anonymous sessions and the
/// per-question tally — and return the total rows deleted. The cutoff mirrors
/// `cohort::is_expired` (strictly older than the window).
pub async fn purge_expired(
    pool: &PgPool,
    now: i64,
    retention_days: i64,
) -> Result<u64, StoreError> {
    let cutoff = now.saturating_sub(retention_days.saturating_mul(86_400));
    let sessions = sqlx::query!(
        "delete from anonymous_sessions where completed_at < $1",
        cutoff,
    )
    .execute(pool)
    .await?;
    // the per-question "% des autres" tally follows the same retention window
    let answers = sqlx::query!(
        "delete from anonymous_question_answers where created_at < $1",
        cutoff,
    )
    .execute(pool)
    .await?;
    Ok(sessions.rows_affected() + answers.rows_affected())
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

/// Record one client's answer to one question. Integrity-first: a client counts
/// once per question (primary key), and a re-judge overwrites its choice rather
/// than inflating the tally. `client_id` is the opaque `raip_cohort_id` —
/// pseudonymous, never nominative (ADR 0006), and this table is never joined to
/// the cohort tables.
pub async fn record_question_answer(
    pool: &PgPool,
    client_id: &str,
    question_id: &str,
    choice_id: &str,
    now: i64,
) -> Result<(), StoreError> {
    sqlx::query!(
        "insert into anonymous_question_answers (client_id, question_id, choice_id, created_at) \
         values ($1, $2, $3, $4) \
         on conflict (client_id, question_id) \
         do update set choice_id = excluded.choice_id, created_at = excluded.created_at",
        client_id,
        question_id,
        choice_id,
        now,
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// The k-anonymous distribution of answers for one question — the vitrine "% des
/// autres" — plus an access audit (event only, no user data). Below k the shares
/// are withheld so a small cohort can't be de-anonymised, mirroring `distribution`
/// for the cohort. `k` is the caller's threshold (`cohort::DEFAULT_MIN_COHORT`).
pub async fn question_distribution(
    pool: &PgPool,
    question_id: &str,
    k: usize,
    now: i64,
) -> Result<QuestionDistribution, StoreError> {
    let rows = sqlx::query!(
        "select choice_id, count(*) as \"count!\" from anonymous_question_answers \
         where question_id = $1 group by choice_id",
        question_id,
    )
    .fetch_all(pool)
    .await?;

    let total: i64 = rows.iter().map(|r| r.count).sum();
    let met = total as usize >= k;

    // withheld below k: reveal shares only when the cohort is large enough
    let shares = if met {
        rows.iter()
            .map(|r| QuestionChoiceShare {
                choice_id: r.choice_id.clone(),
                percent: (r.count as f64) * 100.0 / (total as f64),
            })
            .collect()
    } else {
        Vec::new()
    };

    sqlx::query!(
        "insert into question_access_audit (accessed_at, question_id, cohort_size, threshold_met) \
         values ($1, $2, $3, $4)",
        now,
        question_id,
        total as i32,
        met,
    )
    .execute(pool)
    .await?;

    Ok(QuestionDistribution {
        question_id: question_id.to_string(),
        min_cohort_size_met: met,
        total: total as u32,
        shares,
    })
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
    async fn question_distribution_withholds_below_k_and_audits(pool: PgPool) {
        // k-1 distinct clients answer the same question -> withheld
        for i in 0..(DEFAULT_MIN_COHORT - 1) {
            record_question_answer(&pool, &format!("c{i}"), "q-x", "a", 1000)
                .await
                .unwrap();
        }
        let d = question_distribution(&pool, "q-x", DEFAULT_MIN_COHORT, 2000)
            .await
            .unwrap();
        assert!(!d.min_cohort_size_met);
        assert!(d.shares.is_empty());
        assert_eq!(d.total, (DEFAULT_MIN_COHORT - 1) as u32);
        // the access was audited, with the withheld flag
        let (size, met): (i32, bool) = sqlx::query_as(
            "select cohort_size, threshold_met from question_access_audit order by id desc limit 1",
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(size, (DEFAULT_MIN_COHORT - 1) as i32);
        assert!(!met);
    }

    #[sqlx::test]
    async fn question_distribution_exposes_shares_at_k(pool: PgPool) {
        // k clients: k-1 pick "a", 1 picks "b"
        for i in 0..(DEFAULT_MIN_COHORT - 1) {
            record_question_answer(&pool, &format!("a{i}"), "q-y", "a", 1000)
                .await
                .unwrap();
        }
        record_question_answer(&pool, "b0", "q-y", "b", 1000)
            .await
            .unwrap();
        let d = question_distribution(&pool, "q-y", DEFAULT_MIN_COHORT, 2000)
            .await
            .unwrap();
        assert!(d.min_cohort_size_met);
        assert_eq!(d.total, DEFAULT_MIN_COHORT as u32);
        let sum: f64 = d.shares.iter().map(|s| s.percent).sum();
        assert!((sum - 100.0).abs() < 1e-9, "shares sum to 100%");
        let a = d.shares.iter().find(|s| s.choice_id == "a").unwrap();
        let expected = (DEFAULT_MIN_COHORT - 1) as f64 * 100.0 / DEFAULT_MIN_COHORT as f64;
        assert!((a.percent - expected).abs() < 1e-9);
    }

    #[sqlx::test]
    async fn re_judge_overwrites_choice_without_inflating(pool: PgPool) {
        // integrity: same client answers "a" then re-judges "b" — one row, choice "b"
        record_question_answer(&pool, "c", "q-z", "a", 1000)
            .await
            .unwrap();
        record_question_answer(&pool, "c", "q-z", "b", 1500)
            .await
            .unwrap();
        let (count, choice): (i64, String) = sqlx::query_as(
            "select count(*), max(choice_id) from anonymous_question_answers where question_id = 'q-z'",
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(count, 1, "one client counts once per question");
        assert_eq!(choice, "b", "the latest judgement wins");
    }

    #[sqlx::test]
    async fn purge_removes_expired_question_answers(pool: PgPool) {
        let now = 1_000_000i64;
        let window = DEFAULT_RETENTION_DAYS * 86_400;
        // one expired answer + one recent, on the same question
        record_question_answer(&pool, "old", "q", "a", now - window - 10)
            .await
            .unwrap();
        record_question_answer(&pool, "new", "q", "a", now)
            .await
            .unwrap();
        purge_expired(&pool, now, DEFAULT_RETENTION_DAYS)
            .await
            .unwrap();
        let remaining: i64 =
            sqlx::query_scalar("select count(*) from anonymous_question_answers")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(remaining, 1, "only the recent answer survives the purge");
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
