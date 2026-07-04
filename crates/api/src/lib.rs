//! Thin Axum API adapter for the validated Rust core.

use axum::extract::{DefaultBodyLimit, Path, State};
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderName, HeaderValue, Request, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use rumble_ai_practices_domain::{
    AnswerEvaluation, AxisLevel, DistributionPosition, EvaluationLevel, FeedbackCard, Interaction,
    Question, QuestionId, ScenarioContext, SessionSummary, SourceRef,
};
use rumble_ai_practices_session::cohort::{DEFAULT_MIN_COHORT, DEFAULT_RETENTION_DAYS};
use rumble_ai_practices_session::{
    SessionError, SessionState, complete_session, start_session, submit_answer,
};
use rumble_ai_practices_store::{self as store, AxisOutcome};
use rumble_ai_practices_web::render_app_html;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub service: String,
}

pub fn health_status() -> HealthStatus {
    HealthStatus {
        status: "ok".into(),
        service: "rumble-ai-practices".into(),
    }
}

const MAX_IN_MEMORY_SESSIONS: usize = 1024;
const SESSION_TTL: Duration = Duration::from_secs(60 * 60);

#[derive(Clone)]
pub struct ApiState {
    questions: Arc<Vec<Question>>,
    sessions: Arc<RwLock<BTreeMap<String, StoredSession>>>,
    next_session: Arc<AtomicU64>,
    // Two-tier storage (ADR 0006): live sessions stay in memory (ephemeral,
    // per-user, no cross-session value — data minimisation); only the completed
    // ANONYMOUS result lands in Postgres for the k-anonymous cohort. `None`
    // keeps the API fully in-memory (used by the unit tests).
    store: Option<PgPool>,
}

#[derive(Debug, Clone)]
struct StoredSession {
    state: SessionState,
    created_at: Instant,
    /// Opaque, random id used when the completed result is persisted anonymously
    /// — never the enumerable in-memory session id (ADR 0006).
    anon_id: String,
}

impl ApiState {
    pub fn new(questions: Vec<Question>) -> Self {
        Self {
            questions: Arc::new(questions),
            sessions: Arc::new(RwLock::new(BTreeMap::new())),
            next_session: Arc::new(AtomicU64::new(1)),
            store: None,
        }
    }

    /// Same as `new`, wired to a cohort store (anonymous results + distribution).
    pub fn with_store(questions: Vec<Question>, pool: PgPool) -> Self {
        Self {
            store: Some(pool),
            ..Self::new(questions)
        }
    }

    fn allocate_session_id(&self) -> String {
        let id = self.next_session.fetch_add(1, Ordering::Relaxed);
        format!("session-{id}")
    }
}

/// Current wall-clock time in epoch seconds (for persistence + retention).
fn now_epoch_secs() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

pub async fn serve(addr: SocketAddr, questions: Vec<Question>) -> std::io::Result<()> {
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router(questions)).await
}

/// Serve with a cohort store: persists completed anonymous results and runs the
/// retention purge on a schedule (ADR 0006).
pub async fn serve_with_store(
    addr: SocketAddr,
    questions: Vec<Question>,
    pool: PgPool,
) -> std::io::Result<()> {
    spawn_retention(pool.clone(), 6 * 60 * 60);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(
        listener,
        router_with_state(ApiState::with_store(questions, pool)),
    )
    .await
}

/// Background retention purge (ADR 0006): drops anonymous sessions past the
/// retention window every `interval_secs`.
fn spawn_retention(pool: PgPool, interval_secs: u64) {
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(Duration::from_secs(interval_secs));
        loop {
            ticker.tick().await;
            let _ = store::purge_expired(&pool, now_epoch_secs(), DEFAULT_RETENTION_DAYS).await;
        }
    });
}

pub fn router(questions: Vec<Question>) -> Router {
    router_with_state(ApiState::new(questions))
}

pub fn router_with_state(state: ApiState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/app.js", get(app_js))
        .route("/assets/icon.svg", get(icon_svg))
        .route("/manifest.webmanifest", get(manifest))
        .route("/sw.js", get(service_worker))
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/v1/catalog", get(catalog))
        .route("/v1/sessions", post(create_session))
        .route("/v1/sessions/{session_id}/next", get(next_question))
        .route(
            "/v1/sessions/{session_id}/answers",
            post(submit_session_answer),
        )
        .route("/v1/sessions/{session_id}/summary", get(session_summary))
        // offline-first: the client runs the parcours locally and posts only its
        // anonymous result here to be situated against the cohort (ADR 0006).
        .route("/v1/cohort", post(cohort_position))
        .layer(DefaultBodyLimit::max(16 * 1024))
        .layer(middleware::from_fn(security_headers))
        .with_state(state)
}

fn cleanup_sessions(sessions: &mut BTreeMap<String, StoredSession>) {
    let now = Instant::now();
    sessions.retain(|_, session| now.duration_since(session.created_at) <= SESSION_TTL);
}

fn enforce_session_limit(sessions: &mut BTreeMap<String, StoredSession>) {
    while sessions.len() > MAX_IN_MEMORY_SESSIONS {
        let Some(oldest_key) = sessions
            .iter()
            .min_by_key(|(_, session)| session.created_at)
            .map(|(key, _)| key.clone())
        else {
            break;
        };
        sessions.remove(&oldest_key);
    }
}

async fn security_headers(request: Request<axum::body::Body>, next: Next) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert(
        HeaderName::from_static("content-security-policy"),
        HeaderValue::from_static("default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data:; connect-src 'self'; object-src 'none'; base-uri 'self'; frame-ancestors 'none'"),
    );
    headers.insert(
        HeaderName::from_static("x-content-type-options"),
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        HeaderName::from_static("referrer-policy"),
        HeaderValue::from_static("no-referrer"),
    );
    headers.insert(
        HeaderName::from_static("permissions-policy"),
        HeaderValue::from_static("camera=(), microphone=(), geolocation=(), payment=()"),
    );
    headers.insert(
        HeaderName::from_static("x-frame-options"),
        HeaderValue::from_static("DENY"),
    );
    response
}

async fn index() -> Html<String> {
    Html(index_html())
}

fn index_html() -> String {
    format!(
        r##"<!doctype html>
<html lang="fr">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <meta name="theme-color" content="#111827">
  <link rel="manifest" href="/manifest.webmanifest">
  <link rel="icon" href="/assets/icon.svg" type="image/svg+xml">
  <title>Rumble AI Practices</title>
</head>
<body>
  <div id="app">{}</div>
  <script src="/app.js" defer></script>
</body>
</html>"##,
        render_app_html()
    )
}

async fn app_js() -> impl IntoResponse {
    ([(CONTENT_TYPE, "text/javascript; charset=utf-8")], APP_JS)
}

const APP_JS: &str = r#"if ('serviceWorker' in navigator) {
  window.addEventListener('load', () => {
    navigator.serviceWorker.register('/sw.js').catch(() => undefined);
  });
}"#;

async fn manifest() -> impl IntoResponse {
    ([(CONTENT_TYPE, "application/manifest+json")], MANIFEST)
}

const MANIFEST: &str = r##"{
  "name": "Rumble AI Practices",
  "short_name": "AI Practices",
  "description": "Diagnostic pédagogique souverain des pratiques IA.",
  "start_url": "/",
  "scope": "/",
  "display": "standalone",
  "background_color": "#ffffff",
  "theme_color": "#111827",
  "icons": [
    {
      "src": "/assets/icon.svg",
      "sizes": "any",
      "type": "image/svg+xml",
      "purpose": "any maskable"
    }
  ]
}"##;

async fn service_worker() -> impl IntoResponse {
    (
        [(CONTENT_TYPE, "text/javascript; charset=utf-8")],
        SERVICE_WORKER,
    )
}

const SERVICE_WORKER: &str = r#"const CACHE = 'rumble-ai-practices-shell-v1';
const ASSETS = ['/', '/app.js', '/manifest.webmanifest', '/assets/icon.svg'];
self.addEventListener('install', event => {
  event.waitUntil(caches.open(CACHE).then(cache => cache.addAll(ASSETS)));
});
self.addEventListener('activate', event => {
  event.waitUntil(self.clients.claim());
});
self.addEventListener('fetch', event => {
  const url = new URL(event.request.url);
  if (event.request.method !== 'GET' || url.pathname.startsWith('/v1/')) return;
  event.respondWith(caches.match(event.request).then(cached => cached || fetch(event.request)));
});"#;

async fn icon_svg() -> impl IntoResponse {
    ([(CONTENT_TYPE, "image/svg+xml; charset=utf-8")], ICON_SVG)
}

const ICON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 128 128" role="img" aria-label="Rumble AI Practices">
  <rect width="128" height="128" rx="28" fill="#111827"/>
  <path d="M30 78c12-28 24-42 36-42 14 0 22 11 32 35" fill="none" stroke="#93c5fd" stroke-width="10" stroke-linecap="round"/>
  <circle cx="46" cy="84" r="8" fill="#f9fafb"/>
  <circle cx="82" cy="84" r="8" fill="#f9fafb"/>
</svg>"##;

async fn healthz() -> Json<ApiEnvelope<HealthStatus>> {
    Json(ApiEnvelope::new(health_status()))
}

async fn readyz(
    State(state): State<ApiState>,
) -> Result<Json<ApiEnvelope<ReadinessStatus>>, ApiError> {
    if state.questions.is_empty() {
        return Err(ApiError::unavailable("no questions loaded"));
    }

    Ok(Json(ApiEnvelope::new(ReadinessStatus {
        status: "ready".into(),
        question_count: state.questions.len(),
    })))
}

async fn catalog(State(state): State<ApiState>) -> Json<ApiEnvelope<CatalogResponse>> {
    Json(ApiEnvelope::new(CatalogResponse {
        question_count: state.questions.len(),
        tracks: vec![TrackSummary {
            id: "ai-practices-basics".into(),
            label: "Pratiques IA — fondamentaux".into(),
            question_count: state.questions.len(),
        }],
    }))
}

async fn create_session(
    State(state): State<ApiState>,
    Json(request): Json<CreateSessionRequest>,
) -> Result<Json<ApiEnvelope<CreateSessionResponse>>, ApiError> {
    if request.mode.as_deref() != Some("anonymous") && request.mode.is_some() {
        return Err(ApiError::bad_request(
            "only anonymous mode is supported in the MVP",
        ));
    }

    let session_id = state.allocate_session_id();
    let session = start_session(session_id.clone(), state.questions.as_ref().clone())?;
    let mut sessions = state.sessions.write().await;
    cleanup_sessions(&mut sessions);
    sessions.insert(
        session_id.clone(),
        StoredSession {
            state: session,
            created_at: Instant::now(),
            anon_id: Uuid::new_v4().to_string(),
        },
    );
    enforce_session_limit(&mut sessions);

    Ok(Json(ApiEnvelope::new(CreateSessionResponse {
        session_id,
        track_id: request
            .track_id
            .unwrap_or_else(|| "ai-practices-basics".into()),
    })))
}

async fn next_question(
    State(state): State<ApiState>,
    Path(session_id): Path<String>,
) -> Result<Json<ApiEnvelope<Option<PublicQuestion>>>, ApiError> {
    let sessions = state.sessions.read().await;
    let session = sessions
        .get(&session_id)
        .ok_or_else(|| ApiError::not_found("session not found"))?;

    let next = session
        .state
        .questions
        .iter()
        .find(|question| !session.state.answers.contains_key(question.id.as_str()))
        .map(PublicQuestion::from);

    Ok(Json(ApiEnvelope::new(next)))
}

async fn submit_session_answer(
    State(state): State<ApiState>,
    Path(session_id): Path<String>,
    Json(request): Json<SubmitAnswerRequest>,
) -> Result<Json<ApiEnvelope<PublicAnswerFeedback>>, ApiError> {
    let mut sessions = state.sessions.write().await;
    cleanup_sessions(&mut sessions);
    let session = sessions
        .get_mut(&session_id)
        .ok_or_else(|| ApiError::not_found("session not found"))?;
    let question_id = QuestionId::parse(request.question_id)
        .map_err(|err| ApiError::bad_request(err.to_string()))?;
    let evaluation = submit_answer(&mut session.state, &question_id, request.choice_ids)?;
    Ok(Json(ApiEnvelope::new(PublicAnswerFeedback::from(
        evaluation,
    ))))
}

async fn session_summary(
    State(state): State<ApiState>,
    Path(session_id): Path<String>,
) -> Result<Json<ApiEnvelope<SessionSummary>>, ApiError> {
    let sessions = state.sessions.read().await;
    let session = sessions
        .get(&session_id)
        .ok_or_else(|| ApiError::not_found("session not found"))?;

    let mut summary = complete_session(&session.state);
    let complete = session.state.answers.len() == session.state.questions.len();

    // Only a fully-answered run is persisted to the anonymous cohort and situated
    // against it (a partial run is neither counted nor compared).
    if complete && let Some(pool) = &state.store {
        summary.private_distributions =
            enrich_cohort(pool, &session.anon_id, &summary.axis_levels).await?;
    }

    Ok(Json(ApiEnvelope::new(summary)))
}

/// Persist the completed anonymous result (idempotently, opaque id) and read
/// back the k-anonymous distribution for each answered axis (ADR 0006). The
/// user is persisted first, so they are counted in the cohort they see.
async fn enrich_cohort(
    pool: &PgPool,
    anon_id: &str,
    axis_levels: &[AxisLevel],
) -> Result<Vec<DistributionPosition>, ApiError> {
    let now = now_epoch_secs();
    let outcomes: Vec<AxisOutcome> = axis_levels
        .iter()
        .map(|al| AxisOutcome {
            axis: al.axis,
            level: al.level,
            score: al.score,
        })
        .collect();
    store::insert_session(pool, anon_id, now, now, &outcomes)
        .await
        .map_err(ApiError::internal)?;

    let mut positions = Vec::with_capacity(axis_levels.len());
    for al in axis_levels {
        let position = store::distribution(pool, al.axis, Some(al.level), DEFAULT_MIN_COHORT, now)
            .await
            .map_err(ApiError::internal)?;
        positions.push(position);
    }
    Ok(positions)
}

/// Offline-first cohort submission: the client sends its locally-computed
/// anonymous result (per-axis levels) and gets the k-anonymous distribution
/// back. `client_id` is an optional client-generated opaque idempotency key so a
/// network retry does not double-count the learner; without one the server mints
/// a fresh opaque id. Never carries a nominative field (ADR 0006).
#[derive(Debug, Clone, Deserialize)]
pub struct CohortRequest {
    #[serde(default)]
    pub client_id: Option<String>,
    pub axis_levels: Vec<AxisLevel>,
}

async fn cohort_position(
    State(state): State<ApiState>,
    Json(request): Json<CohortRequest>,
) -> Result<Json<ApiEnvelope<Vec<DistributionPosition>>>, ApiError> {
    if request.axis_levels.is_empty() {
        return Err(ApiError::bad_request("axis_levels must not be empty"));
    }
    let Some(pool) = &state.store else {
        return Err(ApiError::unavailable("cohort backend not configured"));
    };
    let anon_id = request
        .client_id
        .filter(|id| !id.trim().is_empty())
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    let positions = enrich_cohort(pool, &anon_id, &request.axis_levels).await?;
    Ok(Json(ApiEnvelope::new(positions)))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiEnvelope<T> {
    pub data: T,
    pub meta: ApiMeta,
}

impl<T> ApiEnvelope<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            meta: ApiMeta {
                version: "v1".into(),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiMeta {
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessStatus {
    pub status: String,
    pub question_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogResponse {
    pub question_count: usize,
    pub tracks: Vec<TrackSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrackSummary {
    pub id: String,
    pub label: String,
    pub question_count: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicQuestion {
    pub id: String,
    pub title: String,
    pub context: ScenarioContext,
    pub prompt: String,
    pub interaction: Interaction,
    pub choices: Vec<PublicChoice>,
}

impl From<&Question> for PublicQuestion {
    fn from(question: &Question) -> Self {
        Self {
            id: question.id.to_string(),
            title: question.title.clone(),
            context: question.context.clone(),
            prompt: question.prompt.clone(),
            interaction: question.interaction.clone(),
            choices: question.choices.iter().map(PublicChoice::from).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicChoice {
    pub id: String,
    pub label: String,
}

impl From<&rumble_ai_practices_domain::Choice> for PublicChoice {
    fn from(choice: &rumble_ai_practices_domain::Choice) -> Self {
        Self {
            id: choice.id.clone(),
            label: choice.label.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicAnswerFeedback {
    pub question_id: String,
    pub selected_choice_ids: Vec<String>,
    pub level: EvaluationLevel,
    pub feedback_cards: Vec<FeedbackCard>,
    pub evidence_refs: Vec<SourceRef>,
}

impl From<AnswerEvaluation> for PublicAnswerFeedback {
    fn from(evaluation: AnswerEvaluation) -> Self {
        Self {
            question_id: evaluation.question_id.to_string(),
            selected_choice_ids: evaluation.selected_choice_ids,
            level: evaluation.level,
            feedback_cards: evaluation.feedback_cards,
            evidence_refs: evaluation.evidence_refs,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateSessionRequest {
    #[serde(default)]
    pub track_id: Option<String>,
    #[serde(default)]
    pub locale: Option<String>,
    #[serde(default)]
    pub mode: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateSessionResponse {
    pub session_id: String,
    pub track_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubmitAnswerRequest {
    pub question_id: String,
    pub choice_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiErrorBody {
    pub code: String,
    pub message: String,
}

#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    body: ApiErrorBody,
}

impl ApiError {
    fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            body: ApiErrorBody {
                code: "bad_request".into(),
                message: message.into(),
            },
        }
    }

    fn not_found(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            body: ApiErrorBody {
                code: "not_found".into(),
                message: message.into(),
            },
        }
    }

    fn unavailable(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            body: ApiErrorBody {
                code: "unavailable".into(),
                message: message.into(),
            },
        }
    }

    /// A 500 that never leaks the underlying (e.g. database) error to the client.
    fn internal(_source: impl std::fmt::Display) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: ApiErrorBody {
                code: "internal".into(),
                message: "internal error".into(),
            },
        }
    }
}

impl From<SessionError> for ApiError {
    fn from(value: SessionError) -> Self {
        Self::bad_request(value.to_string())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status, Json(self.body)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{Body, to_bytes};
    use axum::http::{Method, Request};
    use rumble_ai_practices_domain::{
        Choice, Confidence, Difficulty, InteractionType, PublicationStatus, ReviewMetadata,
        RiskAxis,
    };
    use serde_json::Value;
    use tower::ServiceExt;

    #[test]
    fn health_is_ok() {
        assert_eq!(health_status().status, "ok");
    }

    #[tokio::test]
    async fn serves_installable_pwa_shell_with_security_headers() {
        let response = router(vec![question()])
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert!(response.headers().contains_key("content-security-policy"));
        assert_eq!(
            response.headers().get("x-content-type-options").unwrap(),
            "nosniff"
        );
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let html = String::from_utf8(body.to_vec()).unwrap();
        assert!(html.contains("manifest.webmanifest"));
        assert!(html.contains("/app.js"));
    }

    #[tokio::test]
    async fn exposes_manifest_service_worker_and_readyz() {
        let app = router(vec![question()]);
        let manifest = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/manifest.webmanifest")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(manifest.status(), StatusCode::OK);
        assert_eq!(
            manifest.headers().get(CONTENT_TYPE).unwrap(),
            "application/manifest+json"
        );
        let manifest_json: Value =
            serde_json::from_slice(&to_bytes(manifest.into_body(), usize::MAX).await.unwrap())
                .unwrap();
        assert_eq!(manifest_json["display"], "standalone");

        let sw = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/sw.js")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(sw.status(), StatusCode::OK);

        let ready = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/readyz")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(ready.status(), StatusCode::OK);
        assert_eq!(response_json(ready).await["data"]["status"], "ready");
    }

    #[tokio::test]
    async fn api_flow_does_not_expose_answer_metadata_in_next_question() {
        let app = router(vec![question()]);

        let create_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sessions")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"mode":"anonymous"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(create_response.status(), StatusCode::OK);
        let create_json = response_json(create_response).await;
        let session_id = create_json["data"]["session_id"].as_str().unwrap();

        let next_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri(format!("/v1/sessions/{session_id}/next"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(next_response.status(), StatusCode::OK);
        let next_json = response_json(next_response).await;
        assert_eq!(next_json["data"]["id"], "q-api-001");
        assert!(next_json["data"].get("expected_reasoning").is_none());
        assert!(next_json["data"].get("review").is_none());
        assert!(next_json["data"]["choices"][0].get("score").is_none());
        assert!(next_json["data"]["choices"][0].get("feedback").is_none());
    }

    #[tokio::test]
    async fn api_flow_submits_answer_and_returns_summary() {
        let app = router(vec![question()]);
        let create_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sessions")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"mode":"anonymous"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        let create_json = response_json(create_response).await;
        let session_id = create_json["data"]["session_id"].as_str().unwrap();

        let answer_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri(format!("/v1/sessions/{session_id}/answers"))
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"question_id":"q-api-001","choice_ids":["good"]}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(answer_response.status(), StatusCode::OK);
        let answer_json = response_json(answer_response).await;
        assert_eq!(answer_json["data"]["level"], "correct");
        assert!(answer_json["data"].get("score_delta").is_none());
        assert!(answer_json["data"].get("axis_impacts").is_none());

        let summary_response = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri(format!("/v1/sessions/{session_id}/summary"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(summary_response.status(), StatusCode::OK);
        let summary_json = response_json(summary_response).await;
        assert_eq!(summary_json["data"]["answered_count"], 1);
    }

    async fn response_json(response: axum::response::Response) -> Value {
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        serde_json::from_slice(&body).unwrap()
    }

    fn question() -> Question {
        Question {
            id: QuestionId::parse("q-api-001").unwrap(),
            version: 1,
            status: PublicationStatus::Draft,
            locale: "fr-FR".into(),
            title: "API question".into(),
            axis: RiskAxis::SourceVerification,
            difficulty: Difficulty::Beginner,
            intent: "Tester que l'API n'expose pas les réponses.".into(),
            context: ScenarioContext {
                role: None,
                scenario: "Situation API".into(),
                assets: vec![],
            },
            prompt: "Que faire ?".into(),
            interaction: Interaction {
                kind: InteractionType::SingleChoice,
                min_choices: Some(1),
                max_choices: Some(1),
            },
            choices: vec![
                Choice {
                    id: "good".into(),
                    label: "Vérifier".into(),
                    score: 1.0,
                    severity: None,
                    feedback: "Bonne pratique".into(),
                },
                Choice {
                    id: "bad".into(),
                    label: "Envoyer".into(),
                    score: -1.0,
                    severity: None,
                    feedback: "Risque".into(),
                },
            ],
            expected_reasoning: vec!["raison".into()],
            risks: vec!["risk".into()],
            sources: vec![],
            media: vec![],
            review: ReviewMetadata {
                author: "test".into(),
                reviewers: vec![],
                last_reviewed_at: None,
                confidence: Confidence::Medium,
                notes: None,
            },
        }
    }

    // ---- cohort persistence over a real Postgres (ADR 0006) ----

    async fn create_anon_session(app: &Router) -> String {
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sessions")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"mode":"anonymous"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        response_json(resp).await["data"]["session_id"]
            .as_str()
            .unwrap()
            .to_string()
    }

    async fn answer_good(app: &Router, session_id: &str) {
        app.clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri(format!("/v1/sessions/{session_id}/answers"))
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"question_id":"q-api-001","choice_ids":["good"]}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
    }

    async fn get_summary(app: &Router, session_id: &str) -> Value {
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri(format!("/v1/sessions/{session_id}/summary"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        response_json(resp).await
    }

    #[sqlx::test(migrator = "rumble_ai_practices_store::MIGRATOR")]
    async fn summary_persists_anonymously_idempotently_and_withholds_below_k(pool: PgPool) {
        let app = router_with_state(ApiState::with_store(vec![question()], pool.clone()));
        let session_id = create_anon_session(&app).await;
        answer_good(&app, &session_id).await;

        let summary = get_summary(&app, &session_id).await;
        let dists = summary["data"]["private_distributions"].as_array().unwrap();
        // a single completed run -> cohort of 1 < k -> distribution withheld
        assert_eq!(dists.len(), 1);
        assert_eq!(dists[0]["min_cohort_size_met"], false);

        // persisted once, under an OPAQUE id (never the enumerable session id)
        let stored: String =
            sqlx::query_scalar("select session_id from anonymous_sessions limit 1")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_ne!(stored, session_id);
        assert!(stored.len() >= 32, "opaque uuid, got {stored}");

        // replaying the summary must not double-count the learner (idempotent)
        let _ = get_summary(&app, &session_id).await;
        let count: i64 = sqlx::query_scalar("select count(*) from anonymous_sessions")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count, 1);
    }

    #[sqlx::test(migrator = "rumble_ai_practices_store::MIGRATOR")]
    async fn summary_exposes_distribution_at_k(pool: PgPool) {
        use rumble_ai_practices_domain::PracticeLevel;
        let k = rumble_ai_practices_session::cohort::DEFAULT_MIN_COHORT;
        // pre-seed k-1 anonymous sessions on the answered axis; the learner is k-th
        for i in 0..(k - 1) {
            store::insert_session(
                &pool,
                &format!("seed-{i}"),
                1000,
                1000,
                &[AxisOutcome {
                    axis: RiskAxis::SourceVerification,
                    level: PracticeLevel::CarefulAutonomy,
                    score: 1.0,
                }],
            )
            .await
            .unwrap();
        }

        let app = router_with_state(ApiState::with_store(vec![question()], pool.clone()));
        let session_id = create_anon_session(&app).await;
        answer_good(&app, &session_id).await;
        let summary = get_summary(&app, &session_id).await;

        let dists = summary["data"]["private_distributions"].as_array().unwrap();
        assert_eq!(dists[0]["min_cohort_size_met"], true);
        let sum: f64 = dists[0]["buckets"]
            .as_array()
            .unwrap()
            .iter()
            .map(|b| b["percent"].as_f64().unwrap())
            .sum();
        assert!((sum - 100.0).abs() < 1e-9);
    }

    // ---- offline-first cohort endpoint (POST /v1/cohort) ----

    async fn post_cohort(app: &Router, body: &str) -> axum::response::Response {
        app.clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/cohort")
                    .header("content-type", "application/json")
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap()
    }

    #[sqlx::test(migrator = "rumble_ai_practices_store::MIGRATOR")]
    async fn cohort_endpoint_withholds_below_k_and_is_idempotent(pool: PgPool) {
        let app = router_with_state(ApiState::with_store(vec![question()], pool.clone()));
        let body = r#"{"client_id":"abc","axis_levels":[{"axis":"source_verification","level":"careful_autonomy","score":1.0}]}"#;

        let resp = post_cohort(&app, body).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let json = response_json(resp).await;
        let dists = json["data"].as_array().unwrap();
        assert_eq!(dists.len(), 1);
        assert_eq!(dists[0]["min_cohort_size_met"], false);

        // same client_id -> idempotent, no double count
        let _ = post_cohort(&app, body).await;
        let count: i64 = sqlx::query_scalar("select count(*) from anonymous_sessions")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count, 1);
    }

    #[sqlx::test(migrator = "rumble_ai_practices_store::MIGRATOR")]
    async fn cohort_endpoint_exposes_at_k(pool: PgPool) {
        use rumble_ai_practices_domain::PracticeLevel;
        let k = rumble_ai_practices_session::cohort::DEFAULT_MIN_COHORT;
        for i in 0..(k - 1) {
            store::insert_session(
                &pool,
                &format!("seed-{i}"),
                1000,
                1000,
                &[AxisOutcome {
                    axis: RiskAxis::SourceVerification,
                    level: PracticeLevel::CarefulAutonomy,
                    score: 1.0,
                }],
            )
            .await
            .unwrap();
        }
        let app = router_with_state(ApiState::with_store(vec![question()], pool));
        let body =
            r#"{"axis_levels":[{"axis":"source_verification","level":"reference","score":1.0}]}"#;
        let json = response_json(post_cohort(&app, body).await).await;
        assert_eq!(json["data"][0]["min_cohort_size_met"], true);
        assert_eq!(json["data"][0]["user_bucket"], "référence");
    }

    #[tokio::test]
    async fn cohort_endpoint_rejects_empty_axis_levels() {
        let app = router(vec![question()]); // no store: input is validated first
        let resp = post_cohort(&app, r#"{"axis_levels":[]}"#).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[sqlx::test(migrator = "rumble_ai_practices_store::MIGRATOR")]
    async fn cohort_wire_contract_roundtrips_with_the_web_client_types(pool: PgPool) {
        use rumble_ai_practices_domain::PracticeLevel;
        // The real client wire types (not mirrors) drive both serde directions
        // against the live handler, so a drift between app and API would fail
        // here — this is the offline-first client↔server contract.
        use rumble_ai_practices_web::{CohortEnvelope, CohortSubmission};

        let app = router_with_state(ApiState::with_store(vec![question()], pool));
        let submission = CohortSubmission {
            client_id: Some("contract".into()),
            axis_levels: vec![AxisLevel {
                axis: RiskAxis::SourceVerification,
                level: PracticeLevel::CarefulAutonomy,
                score: 1.0,
            }],
        };
        let body = serde_json::to_string(&submission).unwrap();

        // request leg: the client's serialization is what the handler accepts
        let resp = post_cohort(&app, &body).await;
        assert_eq!(resp.status(), StatusCode::OK);

        // response leg: the client parses the envelope back with its own type
        let json = response_json(resp).await;
        let envelope: CohortEnvelope = serde_json::from_value(json).unwrap();
        assert_eq!(envelope.data.len(), 1);
        // a single learner is below k, so the position is withheld
        assert!(!envelope.data[0].min_cohort_size_met);
    }
}
