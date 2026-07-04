use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use rumble_ai_practices_api::serve;
use rumble_ai_practices_audit::audit_corpus;
use rumble_ai_practices_content::validate_content;
use rumble_ai_practices_domain::QuestionId;
use rumble_ai_practices_session::{SessionFixture, run_fixture};
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
    serve(bind, loaded.questions, web_root.to_path_buf())
        .await
        .context("API server failed")
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
