use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand, ValueEnum};
use rumble_ai_practices_api::{serve, serve_with_store};
use rumble_ai_practices_audit::audit_corpus;
use rumble_ai_practices_content::{validate_activities, validate_content};
use rumble_ai_practices_domain::{ActivityId, PublicationStatus, QuestionId};
use rumble_ai_practices_session::{
    ActivityAttempt, ActivityAttemptStatus, SessionFixture, run_activity, run_fixture,
};
use sqlx::postgres::PgPoolOptions;
use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Debug, Parser)]
#[command(name = "rumble-ai-practices")]
#[command(about = "Validate, audit, and run rumble-ai-practices content")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Validate a question corpus and fail on blocking findings.
    ValidateCorpus {
        #[arg(long, default_value = "content/questions")]
        content: PathBuf,
        #[arg(long, default_value = "content/media")]
        media: PathBuf,
    },
    /// Validate governed learning activities and fail on invalid drafts or blockers.
    ValidateActivities {
        #[arg(long, default_value = "content/activities")]
        activities: PathBuf,
    },
    /// Run one activity as an explicit local preview; never awards success automatically.
    RunActivity {
        #[arg(long)]
        id: String,
        #[arg(long, default_value = "content/activities")]
        activities: PathBuf,
        #[arg(long, value_enum)]
        status: ActivityRunStatus,
        #[arg(long = "evidence-ref")]
        evidence_refs: Vec<String>,
        #[arg(long)]
        stop_reason: Option<String>,
        #[arg(long, default_value_t = false)]
        allow_draft_preview: bool,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Audit a question corpus and optionally write the report as JSON.
    AuditCorpus {
        #[arg(long, default_value = "content/questions")]
        content: PathBuf,
        #[arg(long, default_value = "content/media")]
        media: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Inspect one question by id.
    InspectQuestion {
        #[arg(long)]
        id: String,
        #[arg(long, default_value = "content/questions")]
        content: PathBuf,
        #[arg(long, default_value = "content/media")]
        media: PathBuf,
    },
    /// Run a deterministic session fixture against a corpus.
    RunSession {
        #[arg(long)]
        fixture: PathBuf,
        #[arg(long, default_value = "content/questions")]
        content: PathBuf,
        #[arg(long, default_value = "content/media")]
        media: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Serve the single-origin deployable: API + static web bundle.
    Serve {
        #[arg(long, default_value = "content/questions")]
        content: PathBuf,
        #[arg(long, default_value = "content/media")]
        media: PathBuf,
        #[arg(long, default_value = "127.0.0.1:3000")]
        bind: SocketAddr,
        #[arg(
            long,
            default_value = "target/dx/rumble-ai-practices-web-app/release/web/public"
        )]
        web_root: PathBuf,
    },
}

/// Outcome submitted by the local activity preview.
#[derive(Debug, Clone, Copy, ValueEnum)]
enum ActivityRunStatus {
    EvidenceSubmitted,
    Stopped,
}

impl From<ActivityRunStatus> for ActivityAttemptStatus {
    fn from(value: ActivityRunStatus) -> Self {
        match value {
            ActivityRunStatus::EvidenceSubmitted => Self::EvidenceSubmitted,
            ActivityRunStatus::Stopped => Self::Stopped,
        }
    }
}

/// Cohort backend mode: where to persist anonymous session outcomes.
#[derive(Debug)]
enum CohortBackend {
    Postgres(String),
    InMemory,
}

/// Decide the cohort backend based on the DATABASE_URL environment variable.
/// Empty or whitespace-only strings are treated as unset (in-memory).
fn cohort_backend(db_url: Option<String>) -> CohortBackend {
    match db_url {
        Some(url) if !url.trim().is_empty() => CohortBackend::Postgres(url.trim().to_string()),
        _ => CohortBackend::InMemory,
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err:#}");
            ExitCode::from(1)
        }
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::ValidateCorpus { content, media } => validate_corpus_cmd(&content, &media),
        Command::ValidateActivities { activities } => validate_activities_cmd(&activities),
        Command::RunActivity {
            id,
            activities,
            status,
            evidence_refs,
            stop_reason,
            allow_draft_preview,
            out,
        } => run_activity_cmd(
            &activities,
            &id,
            status,
            evidence_refs,
            stop_reason,
            allow_draft_preview,
            out.as_deref(),
        ),
        Command::AuditCorpus {
            content,
            media,
            out,
        } => audit_corpus_cmd(&content, &media, out.as_deref()),
        Command::InspectQuestion { id, content, media } => {
            inspect_question_cmd(&content, &media, &id)
        }
        Command::RunSession {
            fixture,
            content,
            media,
            out,
        } => run_session_cmd(&content, &media, &fixture, out.as_deref()),
        Command::Serve {
            content,
            media,
            bind,
            web_root,
        } => serve_cmd(&content, &media, bind, &web_root).await,
    }
}

fn validate_corpus_cmd(content: &Path, media: &Path) -> Result<()> {
    let loaded = validate_content(content, media).context("failed to validate content")?;
    print_json(&loaded.report)?;
    if loaded.report.is_success() {
        Ok(())
    } else {
        bail!("corpus validation failed")
    }
}

fn validate_activities_cmd(activities: &Path) -> Result<()> {
    let loaded = validate_activities(activities).context("failed to validate activities")?;
    print_json(&loaded.report)?;
    if loaded.report.is_success() {
        Ok(())
    } else {
        bail!("activity validation failed")
    }
}

fn run_activity_cmd(
    activities: &Path,
    id: &str,
    status: ActivityRunStatus,
    evidence_refs: Vec<String>,
    stop_reason: Option<String>,
    allow_draft_preview: bool,
    out: Option<&Path>,
) -> Result<()> {
    let activity_id = ActivityId::parse(id.to_owned())?;
    let loaded = validate_activities(activities).context("failed to validate activities")?;
    if !loaded.report.is_success() {
        print_json(&loaded.report)?;
        bail!("refusing to run invalid activity content")
    }
    let activity = loaded
        .activities
        .iter()
        .find(|activity| activity.id == activity_id)
        .with_context(|| format!("activity `{id}` not found"))?;
    if activity.status != PublicationStatus::Approved && !allow_draft_preview {
        bail!("activity `{id}` is not approved; pass --allow-draft-preview only for local review")
    }
    let outcome = run_activity(
        activity,
        ActivityAttempt {
            activity_id,
            status: status.into(),
            evidence_refs,
            stop_reason,
        },
    )?;
    write_or_print(out, &outcome)
}

fn audit_corpus_cmd(content: &Path, media: &Path, out: Option<&Path>) -> Result<()> {
    let loaded = validate_content(content, media).context("failed to load content before audit")?;
    let report = audit_corpus(&loaded.questions);
    write_or_print(out, &report)?;
    if report.has_blockers() {
        bail!("audit found blocker findings")
    }
    Ok(())
}

fn inspect_question_cmd(content: &Path, media: &Path, id: &str) -> Result<()> {
    let question_id = QuestionId::parse(id.to_owned())?;
    let loaded =
        validate_content(content, media).context("failed to load content before inspection")?;
    let question = loaded
        .questions
        .iter()
        .find(|question| question.id == question_id)
        .with_context(|| format!("question `{id}` not found"))?;
    print_json(question)
}

fn run_session_cmd(content: &Path, media: &Path, fixture: &Path, out: Option<&Path>) -> Result<()> {
    let loaded =
        validate_content(content, media).context("failed to load content before session")?;
    let raw_fixture = fs::read_to_string(fixture)
        .with_context(|| format!("failed to read fixture `{}`", fixture.display()))?;
    let fixture: SessionFixture = serde_json::from_str(&raw_fixture)
        .with_context(|| format!("failed to parse fixture `{}`", fixture.display()))?;
    let summary =
        run_fixture(loaded.questions, fixture).context("failed to run session fixture")?;
    write_or_print(out, &summary)
}

async fn serve_cmd(content: &Path, media: &Path, bind: SocketAddr, web_root: &Path) -> Result<()> {
    let loaded =
        validate_content(content, media).context("failed to validate content before serve")?;
    if !loaded.report.is_success() {
        print_json(&loaded.report)?;
        bail!("refusing to serve invalid content")
    }

    if !web_root.is_dir() {
        eprintln!(
            "error: web bundle directory not found: {}",
            web_root.display()
        );
        eprintln!("Build the Dioxus web bundle first:");
        eprintln!("  cargo install dioxus-cli --version 0.7.9 --locked");
        eprintln!("  dx build --platform web --release");
        bail!("web bundle not found at {}", web_root.display())
    }

    // The SPA fallback serves `index.html` for every client-side route, so a
    // present-but-incomplete bundle (dir exists, index missing) would start fine
    // and then 404 every navigation. Fail fast at startup instead.
    let index_html = web_root.join("index.html");
    if !index_html.is_file() {
        eprintln!(
            "error: web bundle is incomplete — missing {}",
            index_html.display()
        );
        eprintln!("Re-run: dx build --platform web --release");
        bail!(
            "web bundle incomplete: missing index.html at {}",
            index_html.display()
        )
    }

    eprintln!("serving rumble-ai-practices on http://{bind}");
    eprintln!("serving static bundle from {}", web_root.display());

    // Decide cohort backend: Postgres (with k-anonymous cohort) or in-memory.
    let db_url = std::env::var("DATABASE_URL").ok();
    match cohort_backend(db_url) {
        CohortBackend::Postgres(url) => {
            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(&url)
                .await
                .context("failed to connect to Postgres")?;

            // Run embedded migrations (idempotent).
            rumble_ai_practices_store::MIGRATOR
                .run(&pool)
                .await
                .context("failed to run database migrations")?;

            eprintln!("cohort backend: Postgres (k-anonymous cohort enabled)");
            serve_with_store(bind, loaded.questions, pool, web_root.to_path_buf())
                .await
                .context("API server failed")
        }
        CohortBackend::InMemory => {
            eprintln!(
                "cohort backend: in-memory (set DATABASE_URL to enable the k-anonymous cohort)"
            );
            serve(bind, loaded.questions, web_root.to_path_buf())
                .await
                .context("API server failed")
        }
    }
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

fn write_or_print<T: serde::Serialize>(out: Option<&Path>, value: &T) -> Result<()> {
    let json = serde_json::to_string_pretty(value)?;
    if let Some(path) = out {
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create `{}`", parent.display()))?;
        }
        fs::write(path, json).with_context(|| format!("failed to write `{}`", path.display()))?;
    } else {
        println!("{json}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cohort_backend_postgres() {
        let url = "postgres://user@localhost/db".to_string();
        match cohort_backend(Some(url)) {
            CohortBackend::Postgres(s) => assert!(s.contains("postgres")),
            CohortBackend::InMemory => panic!("expected Postgres"),
        }
    }

    #[test]
    fn test_cohort_backend_empty_string() {
        match cohort_backend(Some("".to_string())) {
            CohortBackend::InMemory => (),
            CohortBackend::Postgres(_) => panic!("expected InMemory for empty string"),
        }
    }

    #[test]
    fn test_cohort_backend_whitespace() {
        match cohort_backend(Some("  ".to_string())) {
            CohortBackend::InMemory => (),
            CohortBackend::Postgres(_) => panic!("expected InMemory for whitespace"),
        }
    }

    #[test]
    fn test_cohort_backend_unset() {
        match cohort_backend(None) {
            CohortBackend::InMemory => (),
            CohortBackend::Postgres(_) => panic!("expected InMemory for None"),
        }
    }
}
