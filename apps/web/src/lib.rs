//! Mobile-first PWA shell — the "keycap" direction (ADR 0036).
//!
//! A reflex is a keystroke, but a responsible decision takes a beat: the keycap
//! actuation is deliberately calm (UX direction a). The flow is a short parcours
//! — onboarding gate → one situation per risk axis → private synthesis — so the learner can see
//! which reflexes to reinforce.
//!
//! This crate stays presentational: verdicts are DATA (`FeedbackViewModel`),
//! never computed in the browser (ADR 0003). The corpus is the real content
//! (`content/questions/*.yml`, embedded), driven through the session engine.

use dioxus::prelude::*;
use rumble_ai_practices_content::parse_questions_yaml;
use rumble_ai_practices_domain::{Difficulty, EvaluationLevel, Question, RiskAxis};
use rumble_ai_practices_session::{start_session, submit_answer};
use rumble_ai_practices_ui::{
    CategoryMotif, CategoryOutcome, ChoiceViewModel, FeedbackPanel, FeedbackViewModel, Keycap,
    MotifKind, QuestionViewModel, ScenarioArtifact, ScenarioArtifactView, ScenarioFraming,
    SummaryPanel, SummaryViewModel, VerdictKind,
};
use serde::{Deserialize, Serialize};

/// The engine's evaluation level *is* the UI verdict — the UI never scores.
pub fn verdict_from_level(level: EvaluationLevel) -> VerdictKind {
    match level {
        EvaluationLevel::Correct => VerdictKind::Juste,
        EvaluationLevel::Partial => VerdictKind::Partiel,
        EvaluationLevel::Risky => VerdictKind::Risque,
        EvaluationLevel::Incorrect => VerdictKind::Faux,
    }
}

#[cfg(feature = "ssr")]
pub fn render_app_html() -> String {
    dioxus_ssr::render_element(rsx! { App {} })
}

/// Typed client workflow contract (no runtime HTTP wiring here).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiRoutes {
    pub create_session: &'static str,
    pub next_question_template: &'static str,
    pub submit_answer_template: &'static str,
    pub summary_template: &'static str,
}

impl Default for ApiRoutes {
    fn default() -> Self {
        Self {
            create_session: "/v1/sessions",
            next_question_template: "/v1/sessions/{session_id}/next",
            submit_answer_template: "/v1/sessions/{session_id}/answers",
            summary_template: "/v1/sessions/{session_id}/summary",
        }
    }
}

/// A training question, built from a real content `Question` through the engine:
/// per-choice feedback carries an engine-derived verdict (no scoring in the UI).
/// `category` stays a `&'static str` (from the risk axis) so recorded results
/// need no allocation.
#[derive(Debug, Clone, PartialEq)]
pub struct TrainingQuestion {
    pub id: String,
    pub category: &'static str,
    pub role: String,
    pub motif: MotifKind,
    pub framing: ScenarioFraming,
    pub artifact: Option<ScenarioArtifact>,
    pub prompt: String,
    pub choices: Vec<ChoiceViewModel>,
    pub feedbacks: Vec<FeedbackViewModel>,
}

impl TrainingQuestion {
    fn view_model(&self, index: usize, total: usize) -> QuestionViewModel {
        QuestionViewModel {
            id: self.id.clone(),
            index,
            total,
            role: self.role.clone(),
            category: self.category.into(),
            motif: self.motif,
            framing: self.framing.clone(),
            artifact: self.artifact.clone(),
            prompt: self.prompt.clone(),
            choices: self.choices.clone(),
        }
    }
}

/// Where the learner is in the parcours.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stage {
    Intro,
    Question(usize),
    Summary,
}

/// Global keyboard routing, installed once on `document`. It works on every
/// stage by dispatching to the active screen's buttons via their `data-action`
/// (Enter → validate/continue/start/restart, Space → idk, R → replay/restart,
/// E → export) and to `.choice[data-key]` for the number keys, briefly showing
/// the keycap actuation.
///
/// Why `document::eval` and not `web-sys`: a document-level listener is the
/// portable way to catch keys regardless of focus across all Dioxus render
/// targets (web / desktop / mobile share this crate). A `web-sys` window
/// listener would be web-only and would need a `RuntimeGuard` dance; lifting all
/// per-question state to `App` would break component encapsulation. Behavior is
/// pinned by the Playwright e2e (`apps/web/e2e/parcours.spec.ts`), which drives
/// real key events.
const GLOBAL_KEYS_JS: &str = "if (!window.__raipKeys) { window.__raipKeys = true; \
     document.addEventListener('keydown', function (e) { \
       if (e.metaKey || e.ctrlKey || e.altKey) return; \
       var q = function (s) { return document.querySelector(s); }; \
       var k = e.key; \
       if (k >= '1' && k <= '9') { var b = q('.choice[data-key=\"' + k + '\"]'); \
         if (b) { e.preventDefault(); var c = b.querySelector('.cap'); \
           if (c) { c.classList.add('is-down'); setTimeout(function(){c.classList.remove('is-down');},160); } b.click(); } } \
       else if (k === 'Enter') { var v = q('[data-action=\"validate\"]') || q('[data-action=\"continue\"]') || q('[data-action=\"start\"]') || q('[data-action=\"restart\"]'); \
         if (v && !v.disabled) { e.preventDefault(); v.click(); } } \
       else if (k === ' ') { var i = q('[data-action=\"idk\"]'); if (i) { e.preventDefault(); i.click(); } } \
       else if (k === 'r' || k === 'R') { var r = q('[data-action=\"replay\"]') || q('[data-action=\"restart\"]'); if (r) { e.preventDefault(); r.click(); } } \
       else if (k === 'e' || k === 'E') { var x = q('[data-action=\"export\"]'); if (x) { e.preventDefault(); x.click(); } } \
     }); }";

#[component]
pub fn App() -> Element {
    // theme: None = follow OS; Some("dark"|"light") = explicit toggle.
    let mut theme = use_signal(|| None::<&'static str>);
    let mut stage = use_signal(|| Stage::Intro);
    // recorded answers: (category, motif, chosen feedback | None for "idk").
    let mut results = use_signal(Vec::<(&'static str, MotifKind, Option<FeedbackViewModel>)>::new);
    let corpus = use_hook(corpus);
    let total = corpus.len();
    let theme_attr = theme().unwrap_or("");

    // Install the global keyboard navigation once at the app root. Covered by
    // the Playwright e2e (real key events); see GLOBAL_KEYS_JS for the routing
    // and the rationale for using document::eval over web-sys.
    use_effect(|| {
        document::eval(GLOBAL_KEYS_JS);
    });

    rsx! {
        div { class: "app-root", "data-theme": "{theme_attr}",
            div { class: "app-shell",
                header { class: "app-mast",
                    span { class: "brandmark", "rumble" b { "·" } "ai" b { "·" } "practices" }
                    button {
                        class: "theme-toggle",
                        r#type: "button",
                        onclick: move |_| {
                            let next = match theme() {
                                Some("dark") => Some("light"),
                                Some("light") => None,
                                _ => Some("dark"),
                            };
                            theme.set(next);
                        },
                        "◐ thème"
                    }
                }

                match stage() {
                    Stage::Intro => rsx! {
                        IntroGate {
                            total,
                            on_start: move |_| {
                                // guard: never enter a question if the corpus failed
                                // to load (empty) — avoids an out-of-bounds access.
                                if total > 0 {
                                    results.write().clear();
                                    stage.set(Stage::Question(0));
                                }
                            },
                        }
                    },
                    // `get` (not indexing) so a corpus/index mismatch degrades to
                    // the summary instead of panicking.
                    Stage::Question(i) => match corpus.get(i) {
                        None => rsx! {
                            SummaryPanel { summary: build_summary(&results.read()) }
                        },
                        Some(q) => {
                            let category = q.category;
                            let motif = q.motif;
                            rsx! {
                                QuestionConsole {
                                    key: "{q.id}",
                                    question: q.view_model(i + 1, total),
                                    feedbacks: q.feedbacks.clone(),
                                    is_last: i + 1 == total,
                                    on_continue: move |feedback: Option<FeedbackViewModel>| {
                                        results.write().push((category, motif, feedback));
                                        if i + 1 < total {
                                            stage.set(Stage::Question(i + 1));
                                        } else {
                                            stage.set(Stage::Summary);
                                        }
                                    },
                                }
                            }
                        }
                    },
                    Stage::Summary => {
                        let summary = build_summary(&results.read());
                        let export = export_script(&summary_json(&summary));
                        rsx! {
                            SummaryPanel { summary }
                            div { class: "commit-row",
                                button {
                                    class: "idk",
                                    r#type: "button",
                                    "data-action": "export",
                                    onclick: move |_| {
                                        document::eval(&export);
                                    },
                                    span { "Exporter (JSON)" }
                                    Keycap { legend: "E".to_string(), class: "mini".to_string() }
                                }
                                button {
                                    class: "validate-btn",
                                    r#type: "button",
                                    "data-action": "restart",
                                    autofocus: true,
                                    onclick: move |_| {
                                        results.write().clear();
                                        stage.set(Stage::Intro);
                                    },
                                    span { "Recommencer" }
                                    Keycap { legend: "R".to_string(), class: "mini".to_string() }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// The onboarding gate the UX spec requires: objective, "not an HR evaluation",
/// duration, data collected, and how to leave. Nothing starts before it.
#[component]
fn IntroGate(total: usize, on_start: EventHandler<()>) -> Element {
    rsx! {
        section { class: "intro-gate",
            p { class: "intro-eyebrow", "Entraînement libre" }
            h1 { class: "intro-title", "Entraînez vos réflexes face à l'IA" }
            p { class: "intro-lede",
                "Pas une évaluation RH, pas un classement. Dans une situation concrète, quel est le comportement responsable ?"
            }
            ul { class: "intro-facts",
                li {
                    span { class: "intro-k", "Durée" }
                    span { "≈ 6–8 min · {total} situations · une par risque" }
                }
                li {
                    span { class: "intro-k", "Données" }
                    span { "vos réponses et verdicts, par catégorie — rien de nominatif" }
                }
                li {
                    span { class: "intro-k", "Confidentialité" }
                    span { "analyse anonymisée, exportable en local, zéro traceur externe" }
                }
                li {
                    span { class: "intro-k", "Liberté" }
                    span { "rejouable à volonté — rejouer ne dégrade jamais votre progression" }
                }
            }
            if total == 0 {
                // fail-closed: the corpus could not be loaded — never offer to start
                p { class: "intro-unavailable",
                    "Contenu momentanément indisponible. Réessayez plus tard."
                }
            } else {
                button {
                    class: "validate-btn intro-start",
                    r#type: "button",
                    "data-action": "start",
                    autofocus: true,
                    onclick: move |_| on_start.call(()),
                    span { "Démarrer" }
                    Keycap { legend: "⏎".to_string(), class: "mini".to_string() }
                }
            }
        }
    }
}

/// RUM: stamp the moment a choice becomes the current selection.
const RUM_MARK_SELECT: &str = "window.__raipSelectAt = performance.now();";

/// RUM: on validation, record the selection→validation delay (ms) — the
/// direction-(a) metric ("does slowing the reveal make people deliberate?").
/// Kept anonymous and local (localStorage only), no personal data.
const RUM_MARK_VALIDATE: &str = "if (window.__raipSelectAt) { \
     var d = performance.now() - window.__raipSelectAt; \
     (window.__raipDelays = window.__raipDelays || []).push(Math.round(d)); \
     try { localStorage.setItem('raip_delays', JSON.stringify(window.__raipDelays)); } catch (e) {} }";

/// On lock, move focus to Continue without scrolling — the reveal happens in
/// place, so the viewport must not jump (the pain the swap removes).
const FOCUS_CONTINUE_NO_SCROLL: &str = "requestAnimationFrame(function () { \
     var c = document.querySelector('[data-action=\"continue\"]'); \
     if (c) c.focus({ preventScroll: true }); });";

/// One interactive situation. Owns select/lock state. On validation the console
/// swaps in place — the chosen choice stays pinned and the four-state feedback
/// crossfades into the space the choices held, so nothing reflows or scrolls.
#[component]
pub fn QuestionConsole(
    question: QuestionViewModel,
    feedbacks: Vec<FeedbackViewModel>,
    is_last: bool,
    on_continue: EventHandler<Option<FeedbackViewModel>>,
) -> Element {
    let mut selected = use_signal(|| None::<usize>);
    let mut locked = use_signal(|| false);
    // "je ne sais pas" is a real, honest answer: it locks with no verdict and
    // reveals the reflex to know, rather than silently clearing the selection.
    let mut idk = use_signal(|| false);

    let choices = question.choices.clone();
    // the reflex to surface when the learner opts out of guessing
    let juste_action = feedbacks
        .iter()
        .find(|f| f.verdict == VerdictKind::Juste)
        .map(|f| f.action.clone())
        .unwrap_or_default();

    // Global keyboard listener lives in `App`. On lock we only move focus to
    // Continue (without scroll — the reveal is in place).
    use_effect(move || {
        if locked() {
            document::eval(FOCUS_CONTINUE_NO_SCROLL);
        }
    });

    let progress_total = question.total;
    let progress_idx = question.index;
    let sel_now = selected();
    let is_locked = locked();
    let is_idk = idk();

    // scenario staging (hybrid framing) + its artifact + the header motif
    let motif = question.motif;
    let framing = question.framing.clone();
    let artifact = question.artifact.clone();

    // roving tabindex over the interactive choices (unlocked view)
    let rows: Vec<(usize, ChoiceViewModel, String, &'static str)> = choices
        .iter()
        .enumerate()
        .map(|(idx, choice)| {
            let is_sel = sel_now == Some(idx);
            let cls = if is_sel {
                "choice sel".to_string()
            } else {
                "choice".to_string()
            };
            let tab = if is_sel || (sel_now.is_none() && idx == 0) {
                "0"
            } else {
                "-1"
            };
            (idx, choice.clone(), cls, tab)
        })
        .collect();

    // the chosen choice + its feedback, for the in-place answer view. The
    // verdict is pulled out (Copy) so the pinned row can read it after the
    // feedback itself is moved into FeedbackPanel.
    let pinned = sel_now
        .and_then(|i| choices.get(i).cloned().zip(feedbacks.get(i).cloned()))
        .map(|(choice, fb)| (choice.key, choice.label, fb.verdict, fb));

    rsx! {
        div { class: "console",
            div { class: "q-meta",
                span { "Situation " b { "{question.index:02}" } " · {question.total:02}" }
                if !question.role.is_empty() {
                    span { "Rôle : " b { "{question.role}" } }
                }
                span { "Catégorie : " b { "{question.category}" } }
                span { class: "q-count", "{question.index} / {question.total}" }
                span { class: "q-progress", aria_hidden: "true",
                    for i in 1..=progress_total {
                        i { class: if i <= progress_idx { "on" } else { "" } }
                    }
                }
            }

            // header watermark: a faint category motif behind the meta
            div { class: "console-filigrane", aria_hidden: "true",
                CategoryMotif { kind: motif }
            }

            // hybrid framing: a chat thread (an exchange) or a posed scenario
            match framing {
                ScenarioFraming::Thread { sender, message } => {
                    let initial = sender.chars().next().unwrap_or('•').to_string();
                    rsx! {
                        div { class: "thread",
                            div { class: "msg",
                                div { class: "avatar", aria_hidden: "true", "{initial}" }
                                div { class: "bubble",
                                    div { class: "who", "{sender}" }
                                    p { "{message}" }
                                    if let Some(art) = artifact.clone() {
                                        ScenarioArtifactView { artifact: art }
                                    }
                                }
                            }
                        }
                    }
                }
                ScenarioFraming::Posed { scenario } => rsx! {
                    p { class: "scenario", "{scenario}" }
                    if let Some(art) = artifact.clone() {
                        ScenarioArtifactView { artifact: art }
                    }
                },
            }

            if !is_locked {
                p { class: "prompt", "{question.prompt}" }
                p { class: "calm-hint", "Prenez le temps : lisez chaque option avant de valider." }
                div {
                    class: "choices",
                    role: "radiogroup",
                    aria_label: "Choix de réponse",
                    for (idx , choice , cls , tab) in rows {
                        button {
                            key: "{choice.id}",
                            class: "{cls}",
                            role: "radio",
                            "aria-checked": if sel_now == Some(idx) { "true" } else { "false" },
                            "data-key": "{choice.key}",
                            tabindex: "{tab}",
                            r#type: "button",
                            onclick: move |_| {
                                // one-gesture touch: first tap selects, a tap on the
                                // already-selected choice validates.
                                if selected() == Some(idx) {
                                    document::eval(RUM_MARK_VALIDATE);
                                    locked.set(true);
                                } else {
                                    selected.set(Some(idx));
                                    document::eval(RUM_MARK_SELECT);
                                }
                            },
                            Keycap { legend: choice.key.clone() }
                            span { class: "choice-label", "{choice.label}" }
                        }
                    }
                }
                div { class: "commit-row",
                    button {
                        class: "idk",
                        r#type: "button",
                        "data-action": "idk",
                        onclick: move |_| {
                            selected.set(None);
                            idk.set(true);
                            locked.set(true);
                        },
                        span { "Je ne sais pas" }
                        Keycap { legend: "espace".to_string() }
                    }
                    button {
                        class: "validate-btn",
                        r#type: "button",
                        "data-action": "validate",
                        disabled: sel_now.is_none(),
                        onclick: move |_| {
                            if selected().is_some() {
                                document::eval(RUM_MARK_VALIDATE);
                                locked.set(true);
                            }
                        },
                        span { "Valider" }
                        Keycap { legend: "⏎".to_string(), class: "mini".to_string() }
                    }
                }
            } else {
                // In-place answer: chosen choice pinned, feedback crossfades in.
                div { class: "answered reveal",
                    match (is_idk, pinned) {
                        // honest opt-out: no verdict, but the reflex to know is shown
                        (true, _) => rsx! {
                            div { class: "choice locked pinned", "data-verdict": "idk",
                                Keycap { legend: "?".to_string() }
                                span { class: "choice-label", "Je ne sais pas" }
                                span { class: "verdict-tag", "data-verdict": "idk", "À explorer" }
                            }
                            section {
                                class: "feedback-panel",
                                "data-verdict": "idk",
                                "aria-live": "polite",
                                div { class: "fb-verdict",
                                    span { class: "glyph", aria_hidden: "true", "?" }
                                    span { "Réponse non tranchée" }
                                }
                                p { class: "fb-reason",
                                    "Ne pas trancher est plus honnête que deviner. Le réflexe à connaître :"
                                }
                                p { class: "idk-action", "{juste_action}" }
                            }
                        },
                        (false, Some((key, label, verdict, fb))) => rsx! {
                            div { class: "choice sel locked pinned", "data-verdict": "{verdict.slug()}",
                                Keycap { legend: key }
                                span { class: "choice-label", "{label}" }
                                span { class: "verdict-tag", "data-verdict": "{verdict.slug()}",
                                    span { class: "glyph", aria_hidden: "true", "{verdict.symbol()} " }
                                    "{verdict.label()}"
                                }
                            }
                            FeedbackPanel { feedback: fb }
                        },
                        (false, None) => rsx! {},
                    }
                    div { class: "commit-row",
                        button {
                            class: "idk",
                            r#type: "button",
                            "data-action": "replay",
                            onclick: move |_| {
                                selected.set(None);
                                idk.set(false);
                                locked.set(false);
                            },
                            span { "Rejouer" }
                            Keycap { legend: "R".to_string(), class: "mini".to_string() }
                        }
                        button {
                            class: "validate-btn",
                            r#type: "button",
                            "data-action": "continue",
                            onclick: move |_| {
                                on_continue.call(sel_now.and_then(|i| feedbacks.get(i).cloned()));
                            },
                            span { if is_last { "Voir la synthèse" } else { "Question suivante" } }
                            Keycap { legend: "⏎".to_string(), class: "mini".to_string() }
                        }
                    }
                }
            }
        }
    }
}

/// Aggregate the parcours into a private, per-category synthesis: one row per
/// category with the verdict earned and the single reflex to keep (the chosen
/// feedback's "action"). No ranking, no score.
fn build_summary(
    results: &[(&'static str, MotifKind, Option<FeedbackViewModel>)],
) -> SummaryViewModel {
    let outcomes = results
        .iter()
        .map(|(category, motif, feedback)| CategoryOutcome {
            category: (*category).to_string(),
            motif: *motif,
            verdict: feedback.as_ref().map(|f| f.verdict),
            takeaway: feedback
                .as_ref()
                .map(|f| f.action.clone())
                .unwrap_or_else(|| {
                    "À explorer : rejouez cette situation pour ancrer le réflexe.".to_string()
                }),
        })
        .collect();
    SummaryViewModel {
        answered_count: results.len(),
        outcomes,
        privacy_notice:
            "Aucun classement nominatif. Signaux agrégés par catégorie, exportables en local."
                .to_string(),
    }
}

/// Serialize the synthesis to a JSON string for the local export. Contains only
/// categories and verdicts — no personal data, no identifier.
///
/// The result is embedded verbatim into a `document::eval` (as a JS object
/// literal), so the two unicode line separators U+2028 / U+2029 — which serde
/// leaves raw, are legal in JSON, but terminate a JS line — are escaped. Content
/// is trusted today, but this keeps the embedding robust regardless of source.
fn summary_json(summary: &SummaryViewModel) -> String {
    serde_json::to_string_pretty(summary)
        .unwrap_or_else(|_| "{}".to_string())
        .replace('\u{2028}', "\\u2028")
        .replace('\u{2029}', "\\u2029")
}

/// Client-side download of the synthesis JSON (Blob + anchor click). The RUM
/// selection→validation delays (direction-a metric) are merged in at download
/// time so the export is the metric's destination — local, no beacon, no PII.
fn export_script(json: &str) -> String {
    format!(
        "(function(){{ var data = {json}; \
         var d = (window.__raipDelays || []).slice(); \
         var median = null; \
         if (d.length) {{ var s = d.slice().sort(function(a,b){{ return a - b; }}); median = s[Math.floor(s.length / 2)]; }} \
         data.rum = {{ select_to_validate_ms: d, median_ms: median }}; \
         var blob = new Blob([JSON.stringify(data, null, 2)], {{ type: 'application/json' }}); \
         var url = URL.createObjectURL(blob); var a = document.createElement('a'); \
         a.href = url; a.download = 'rumble-ai-practices-synthese.json'; \
         document.body.appendChild(a); a.click(); document.body.removeChild(a); \
         setTimeout(function(){{ URL.revokeObjectURL(url); }}, 0); }})();"
    )
}

/// The embedded content corpus (one YAML file per source axis-set). `include_str!`
/// bakes it into the wasm at build time — the client parses and drives it through
/// the engine at runtime, no filesystem, no backend.
const CORPUS_FILES: &[&str] = &[
    include_str!("../../../content/questions/privacy-rgpd.yml"),
    include_str!("../../../content/questions/reliability.yml"),
    include_str!("../../../content/questions/responsibility-bias.yml"),
    include_str!("../../../content/questions/security-prompt.yml"),
    include_str!("../../../content/questions/data-sovereignty.yml"),
    include_str!("../../../content/questions/media.yml"),
    include_str!("../../../content/questions/pilot.yml"),
];

/// Sort rank for the learning curve: beginner → intermediate → advanced.
fn difficulty_rank(difficulty: Difficulty) -> u8 {
    match difficulty {
        Difficulty::Beginner => 0,
        Difficulty::Intermediate => 1,
        Difficulty::Advanced => 2,
    }
}

/// Select the parcours from the embedded corpus: one valid question per risk
/// axis (covers the whole surface), ordered by difficulty (gentle learning
/// curve). Kept separate from the view-model mapping so the selection is
/// testable on the real content.
fn parcours_questions() -> Vec<Question> {
    let mut all: Vec<Question> = Vec::new();
    for raw in CORPUS_FILES {
        if let Ok(questions) = parse_questions_yaml(raw) {
            all.extend(questions.into_iter().filter(|q| q.validate_basic().is_ok()));
        }
    }
    let mut axes_seen: Vec<RiskAxis> = Vec::new();
    let mut selected: Vec<Question> = all
        .into_iter()
        .filter(|q| {
            if axes_seen.contains(&q.axis) {
                false
            } else {
                axes_seen.push(q.axis);
                true
            }
        })
        .collect();
    // stable sort keeps the by-axis order within a difficulty tier
    selected.sort_by_key(|q| difficulty_rank(q.difficulty));
    selected
}

/// The parcours as presentational questions. Verdicts are derived by the engine
/// — the UI never scores (ADR 0003).
fn corpus() -> Vec<TrainingQuestion> {
    parcours_questions()
        .iter()
        .map(training_question_from)
        .collect()
}

/// Run a single-choice answer through the engine to get its evaluation level.
/// Inputs are always valid here (the question is loaded and the id is its own).
fn evaluate_choice(question: &Question, choice_id: &str) -> EvaluationLevel {
    start_session("build", vec![question.clone()])
        .and_then(|mut state| submit_answer(&mut state, &question.id, vec![choice_id.to_string()]))
        .map(|evaluation| evaluation.level)
        .unwrap_or(EvaluationLevel::Incorrect)
}

/// A short, human category label for a risk axis (French UI).
fn category_for(axis: RiskAxis) -> &'static str {
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

/// The header/vignette motif for a risk axis.
fn motif_for(axis: RiskAxis) -> MotifKind {
    match axis {
        RiskAxis::DataConfidentiality | RiskAxis::PrivacyRgpd | RiskAxis::BusinessContext => {
            MotifKind::Shield
        }
        RiskAxis::SourceVerification | RiskAxis::Hallucination => MotifKind::Link,
        RiskAxis::BiasFairness | RiskAxis::MediaSynthetic => MotifKind::Portrait,
        RiskAxis::Security | RiskAxis::PromptPractice | RiskAxis::HumanResponsibility => {
            MotifKind::Envelope
        }
    }
}

/// Content scenarios are written as posed narration, so that is the framing.
fn framing_for(question: &Question) -> ScenarioFraming {
    ScenarioFraming::Posed {
        scenario: question.context.scenario.clone(),
    }
}

/// If the scenario declares an asset, show it as an attachment card.
fn artifact_for(question: &Question) -> Option<ScenarioArtifact> {
    question
        .context
        .assets
        .first()
        .map(|asset| ScenarioArtifact::File {
            name: asset.label.clone(),
            meta: asset.kind.replace('_', " "),
            flag: None,
        })
}

/// A French label for a content risk slug. The content tags risks in English
/// snake_case; the UI is French, so translate known slugs (unknown ones fall
/// back to a de-underscored form rather than leaking a raw slug).
fn risk_label_fr(slug: &str) -> String {
    match slug {
        "misinformation" => "désinformation",
        "pii_leak" => "fuite de données personnelles",
        "overconfidence" => "excès de confiance",
        "responsibility_dilution" => "dilution de responsabilité",
        "jurisdiction_exposure" => "exposition juridictionnelle",
        "hallucination" => "hallucination",
        "deepfake_confusion" => "confusion deepfake",
        "data_reuse" => "réutilisation des données",
        "confidentiality_leak" => "fuite de confidentialité",
        "automation_bias" => "biais d'automatisation",
        "unverified_claim" => "affirmation non vérifiée",
        "transparency_gap" => "défaut de transparence",
        "security_exposure" => "exposition de sécurité",
        "secret_leak" => "fuite de secret",
        "safety_risk" => "risque de sûreté",
        "rgpd_violation" => "violation RGPD",
        "prompt_dogma" => "dogmatisme du prompt",
        "outdated_information" => "information périmée",
        "media_bias" => "biais médiatique",
        "vulnerable_users" => "usagers vulnérables",
        "vendor_lockin" | "vendor_dependency" => "dépendance fournisseur",
        "unfair_outcome" | "unfair_evaluation" => "résultat inéquitable",
        "unauthorized_action" => "action non autorisée",
        "stereotype_reinforcement" => "renforcement de stéréotypes",
        "service_continuity" => "continuité de service",
        "retention_risk" => "risque de conservation",
        "reputational_harm" => "atteinte à la réputation",
        "reidentification" => "ré-identification",
        "quality_regression" => "régression de qualité",
        "purpose_violation" => "détournement de finalité",
        "prompt_injection" => "injection de prompt",
        "policy_violation" => "violation de politique",
        "overreliance" => "sur-dépendance",
        "no_rollback" => "absence de retour arrière",
        "misattribution" => "mauvaise attribution",
        "low_authority_source" => "source peu fiable",
        "legal_risk" => "risque juridique",
        "lack_of_recourse" => "absence de recours",
        "internal_information_exposure" => "exposition d'information interne",
        "hallucinated_source" => "source hallucinée",
        "guardrail_bypass" => "contournement des garde-fous",
        "discrimination" => "discrimination",
        "data_exfiltration" => "exfiltration de données",
        "credential_exposure" => "exposition d'identifiants",
        "confirmation_bias" => "biais de confirmation",
        "compliance_gap" => "manquement de conformité",
        "automated_harm" => "préjudice automatisé",
        other => return other.replace('_', " "),
    }
    .to_string()
}

/// Join the question's risk tags into a readable French line for the reflex
/// detail.
fn humanize_risks(risks: &[String]) -> String {
    if risks.is_empty() {
        return "—".to_string();
    }
    risks
        .iter()
        .map(|risk| risk_label_fr(risk))
        .collect::<Vec<_>>()
        .join(" · ")
}

/// Map a content `Question` (with per-choice scores) to the presentational
/// `TrainingQuestion`: each choice gets an engine-derived verdict, and the
/// question-level reasoning/risks/sources fill the reflex detail.
fn training_question_from(question: &Question) -> TrainingQuestion {
    let choices = question
        .choices
        .iter()
        .enumerate()
        .map(|(index, choice)| ChoiceViewModel {
            id: choice.id.clone(),
            key: (index + 1).to_string(),
            label: choice.label.clone(),
        })
        .collect();

    let risk = humanize_risks(&question.risks);
    let action = if question.expected_reasoning.is_empty() {
        "Reprenez le réflexe attendu pour cette situation.".to_string()
    } else {
        question.expected_reasoning.join(" · ")
    };
    let source = question
        .sources
        .first()
        .map(|s| s.label.clone())
        .unwrap_or_else(|| "—".to_string());

    let feedbacks = question
        .choices
        .iter()
        .map(|choice| FeedbackViewModel {
            verdict: verdict_from_level(evaluate_choice(question, &choice.id)),
            reason: choice.feedback.clone(),
            risk: risk.clone(),
            action: action.clone(),
            source: source.clone(),
        })
        .collect();

    TrainingQuestion {
        id: question.id.as_str().to_string(),
        category: category_for(question.axis),
        role: question.context.role.clone().unwrap_or_default(),
        motif: motif_for(question.axis),
        framing: framing_for(question),
        artifact: artifact_for(question),
        prompt: question.prompt.clone(),
        choices,
        feedbacks,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opens_on_the_onboarding_gate_not_a_question() {
        let html = dioxus_ssr::render_element(rsx! { App {} });
        // the gate states objective, not-HR, duration, data, freedom
        assert!(html.contains("Entraînez vos réflexes"));
        assert!(html.contains("Pas une évaluation RH"));
        assert!(html.contains("Démarrer"));
        // the question is not shown before the gate
        assert!(!html.contains("radiogroup"));
        // no numeric scoring ever
        assert!(!html.contains("score_delta"));
        assert!(!html.contains("axis_impacts"));
    }

    #[test]
    fn corpus_is_built_from_real_content_one_per_axis() {
        let qs = corpus();
        // one situation per risk axis (10 axes), all engine-derived
        assert_eq!(qs.len(), 10, "one question per risk axis");
        let mut categories: Vec<&str> = qs.iter().map(|q| q.category).collect();
        categories.sort_unstable();
        categories.dedup();
        assert_eq!(
            categories.len(),
            10,
            "each axis maps to a distinct category"
        );
        for q in &qs {
            assert!(q.id.starts_with("q-"), "real content id: {}", q.id);
            assert!(q.choices.len() >= 2, "at least two choices in {}", q.id);
            assert_eq!(
                q.choices.len(),
                q.feedbacks.len(),
                "each choice has an engine-derived verdict in {}",
                q.id
            );
            assert!(!q.prompt.is_empty(), "real prompt in {}", q.id);
            // risks are shown in French, not raw English slugs
            for fb in &q.feedbacks {
                assert!(
                    !fb.risk.contains('_'),
                    "risk humanized in {}: {}",
                    q.id,
                    fb.risk
                );
            }
        }
    }

    #[test]
    fn intro_gate_is_fail_closed_on_empty_corpus() {
        #[component]
        fn Harness() -> Element {
            rsx! { IntroGate { total: 0, on_start: move |_| {} } }
        }
        let html = dioxus_ssr::render_element(rsx! { Harness {} });
        // no way to start, and an explicit unavailable notice
        assert!(html.contains("indisponible"));
        assert!(!html.contains("data-action=\"start\""));
    }

    #[test]
    fn risk_labels_are_french() {
        assert_eq!(risk_label_fr("pii_leak"), "fuite de données personnelles");
        assert_eq!(risk_label_fr("rgpd_violation"), "violation RGPD");
        // unknown slug falls back to a de-underscored form (still no raw slug look)
        assert_eq!(risk_label_fr("some_new_risk"), "some new risk");
    }

    #[test]
    fn question_console_renders_keycaps_from_real_content() {
        // The EventHandler prop must be built inside the Dioxus runtime, so the
        // console is rendered through a tiny harness component rather than from
        // the test's top-level scope.
        #[component]
        fn Harness() -> Element {
            let qs = corpus();
            let q = &qs[0];
            rsx! {
                QuestionConsole {
                    question: q.view_model(1, qs.len()),
                    feedbacks: q.feedbacks.clone(),
                    is_last: false,
                    on_continue: move |_| {},
                }
            }
        }
        let html = dioxus_ssr::render_element(rsx! { Harness {} });
        assert!(html.contains("radiogroup"));
        assert!(html.contains("data-key=\"1\""));
        assert!(html.contains("data-key=\"2\""));
        // a real question renders its console (calm hint + real prompt exists).
        // The prompt text is not asserted verbatim: SSR escapes apostrophes,
        // which most real French prompts contain.
        assert!(html.contains("Prenez le temps"));
        assert!(!corpus()[0].prompt.is_empty());
    }

    #[test]
    fn summary_is_per_category_with_takeaways_and_no_ranking() {
        let fb = |verdict, action: &str| FeedbackViewModel {
            verdict,
            reason: "r".into(),
            risk: "risk".into(),
            action: action.into(),
            source: "src".into(),
        };
        let results = vec![
            (
                "Confidentialité",
                MotifKind::Shield,
                Some(fb(VerdictKind::Juste, "Inspecter avant d'envoyer.")),
            ),
            (
                "Sources",
                MotifKind::Link,
                Some(fb(VerdictKind::Risque, "Vérifier chaque lien.")),
            ),
            ("Biais média", MotifKind::Portrait, None),
        ];
        let s = build_summary(&results);
        assert_eq!(s.answered_count, 3);
        assert_eq!(s.outcomes.len(), 3);
        // the chosen feedback's action becomes the category takeaway
        let conf = s
            .outcomes
            .iter()
            .find(|o| o.category == "Confidentialité")
            .unwrap();
        assert_eq!(conf.verdict, Some(VerdictKind::Juste));
        assert!(conf.takeaway.contains("Inspecter"));
        // "je ne sais pas" surfaces as an explore-more outcome, not a score
        let bias = s
            .outcomes
            .iter()
            .find(|o| o.category == "Biais média")
            .unwrap();
        assert_eq!(bias.verdict, None);
        assert_eq!(bias.slug(), "idk");
        // a skipped answer gets the "explore more" takeaway, not an empty string
        assert!(bias.takeaway.contains("À explorer"));
        assert!(s.privacy_notice.contains("Aucun classement nominatif"));
    }

    #[test]
    fn all_skipped_parcours_yields_explore_takeaways() {
        let results = vec![
            ("Sources", MotifKind::Link, None),
            ("RGPD", MotifKind::Shield, None),
        ];
        let s = build_summary(&results);
        assert_eq!(s.answered_count, 2);
        assert!(s.outcomes.iter().all(|o| o.verdict.is_none()));
        assert!(s.outcomes.iter().all(|o| o.takeaway.contains("À explorer")));
    }

    #[test]
    fn export_json_is_valid_and_carries_no_personal_data() {
        let results = vec![(
            "Sources",
            MotifKind::Link,
            Some(FeedbackViewModel {
                verdict: VerdictKind::Juste,
                reason: "r".into(),
                risk: "risk".into(),
                action: "Vérifier chaque lien.".into(),
                source: "src".into(),
            }),
        )];
        let json = summary_json(&build_summary(&results));
        // round-trips through serde -> it is valid JSON
        let parsed: SummaryViewModel = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.answered_count, 1);
        assert_eq!(parsed.outcomes[0].category, "Sources");
        // the export script embeds the JSON as a download gesture
        assert!(export_script(&json).contains("rumble-ai-practices-synthese.json"));
    }

    #[test]
    fn parcours_is_ordered_by_difficulty() {
        let ranks: Vec<u8> = parcours_questions()
            .iter()
            .map(|q| difficulty_rank(q.difficulty))
            .collect();
        assert_eq!(ranks.len(), 10);
        assert!(
            ranks.windows(2).all(|w| w[0] <= w[1]),
            "parcours must be non-decreasing in difficulty, got {ranks:?}"
        );
    }

    #[test]
    fn summary_json_escapes_js_line_separators() {
        // a takeaway carrying U+2028 must not survive raw into the eval string
        let outcome = CategoryOutcome {
            category: "X".into(),
            motif: MotifKind::Shield,
            verdict: Some(VerdictKind::Juste),
            takeaway: "avant\u{2028}après".into(),
        };
        let summary = SummaryViewModel {
            answered_count: 1,
            outcomes: vec![outcome],
            privacy_notice: "p".into(),
        };
        let json = summary_json(&summary);
        assert!(!json.contains('\u{2028}'), "raw U+2028 must be escaped");
        assert!(json.contains("\\u2028"));
    }
}
