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
use rumble_ai_practices_domain::{
    AxisLevel, Difficulty, DistributionPosition, EvaluationLevel, Question, RiskAxis,
};
use rumble_ai_practices_session::{complete_session, start_session, submit_answer};
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
       var q = function (s) { var el = document.querySelector(s); \
         return el && getComputedStyle(el).visibility !== 'hidden' ? el : null; }; \
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
    // The chosen choice id per answered question, in parcours order (`None` = "je
    // ne sais pas"). Replayed through the engine at the end to derive the same
    // per-axis levels the server computes — the offline-first cohort payload.
    let mut answers = use_signal(Vec::<Option<String>>::new);
    // The raw parcours questions, aligned index-for-index with `corpus` (both
    // come from the deterministic `parcours_questions`), so a recorded choice can
    // be replayed onto the real `Question` to score it.
    let questions = use_hook(parcours_questions);
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
            main { class: "app-shell",
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
                                    answers.write().clear();
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
                                    on_continue: move |(choice_id, feedback): (
                                        Option<String>,
                                        Option<FeedbackViewModel>,
                                    )| {
                                        results.write().push((category, motif, feedback));
                                        answers.write().push(choice_id);
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
                        // Replay the recorded choices through the engine to get the
                        // same per-axis levels the server computes — the anonymous
                        // offline-first cohort payload (ADR 0006).
                        let axis_levels = axis_levels_from_answers(&questions, &answers.read());
                        rsx! {
                            SummaryStage {
                                summary,
                                axis_levels,
                                on_restart: move |_| {
                                    results.write().clear();
                                    answers.write().clear();
                                    stage.set(Stage::Intro);
                                },
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
            p { class: "intro-eyebrow", "Sensibilisation · biais de l'IA" }
            h1 { class: "intro-title", "Aucune image générée n'est neutre." }
            p { class: "intro-lede",
                "Une IA ne « photographie » pas le réel : elle en tire une version parmi une infinité. Ce choix statistique est toujours un biais — même quand l'image paraît positive ou « diverse »."
            }
            ul { class: "intro-facts",
                li {
                    span { class: "intro-k", "Le déclic" }
                    span { "un prompt en apparence neutre, un résultat biaisé. Ici, on apprend à le voir — surtout là où il se cache." }
                }
                li {
                    span { class: "intro-k", "La thèse" }
                    span { "le problème n'est pas l'humain : le vrai danger, c'est la fausse confiance dans l'outil — et c'est à l'entreprise de l'encadrer." }
                }
                li {
                    span { class: "intro-k", "La session" }
                    span { "{total} situations à juger · rejouable · rien de nominatif" }
                }
                li {
                    span { class: "intro-k", "Le positionnement" }
                    span { "anonyme et solidaire — jamais un classement : « tu n'es pas seul à t'être fait avoir »" }
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
                    span { "Commencer" }
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
    on_continue: EventHandler<(Option<String>, Option<FeedbackViewModel>)>,
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
    // The domain choice id of the selection, threaded up on "continue" so the
    // parent can replay it through the engine for the cohort payload.
    let chosen_choice_id: Option<String> =
        sel_now.and_then(|i| choices.get(i).map(|choice| choice.id.clone()));

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

            // Both layers share one grid cell so the console keeps the max
            // height of the question/answered views: the reveal never shrinks
            // the document (no scroll clamp — the in-place contract holds).
            div { class: "console-stack",
                div { class: if is_locked { "stack-layer stack-ghost" } else { "stack-layer" },
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
                }
                if is_locked {
                    div { class: "stack-layer",
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
                                        let feedback = sel_now.and_then(|i| feedbacks.get(i).cloned());
                                        on_continue.call((chosen_choice_id.clone(), feedback));
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

/// Replay the recorded choices onto the real questions and run them through the
/// engine, then aggregate with `complete_session` — the SAME per-axis levelling
/// the server uses. The UI never invents its own scoring (ADR 0003); it reuses
/// the engine so the client-computed `axis_levels` posted to `/v1/cohort` match
/// exactly what the server-side session path would have produced.
///
/// `answers[i]` is the choice for `questions[i]` (`None` = "je ne sais pas",
/// which submits nothing, so that axis simply has no impact — an honest gap).
fn axis_levels_from_answers(questions: &[Question], answers: &[Option<String>]) -> Vec<AxisLevel> {
    let mut state = match start_session("cohort", questions.to_vec()) {
        Ok(state) => state,
        Err(_) => return Vec::new(),
    };
    for (question, choice) in questions.iter().zip(answers.iter()) {
        if let Some(choice_id) = choice {
            let _ = submit_answer(&mut state, &question.id, vec![choice_id.clone()]);
        }
    }
    complete_session(&state).axis_levels
}

/// The offline-first cohort submission the client posts to `/v1/cohort` — the
/// anonymous per-axis result plus an optional opaque idempotency key, no
/// nominative field (ADR 0006). Public so the API's integration test can drive
/// the exact wire contract (both serde directions) against the live handler,
/// with the real client type rather than a mirror that could drift.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortSubmission {
    pub client_id: Option<String>,
    pub axis_levels: Vec<AxisLevel>,
}

/// The API envelope the client reads back (`{ data: [...] }`); `meta` is ignored.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortEnvelope {
    pub data: Vec<DistributionPosition>,
}

/// The result of the offline-first cohort call. `Offline` is the safe default:
/// the local synthesis stands on its own, and nothing is lost when the network
/// (or the backend) is unavailable.
#[derive(Debug, Clone, PartialEq)]
enum CohortState {
    Loading,
    // Built only by the wasm transport; off-wasm it is still matched by
    // `CohortPanel` but never constructed, which the host build flags as dead
    // code — allow it there, it is live in the browser.
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    Online(Vec<DistributionPosition>),
    Offline,
}

/// POST the anonymous per-axis result to `/v1/cohort` and read back the
/// k-anonymous distribution. Any failure (offline, backend absent, malformed
/// response) degrades to `Offline` — never an error surfaced to the learner.
///
/// wasm-only: the transport lives in the browser. On host/SSR (tests, prerender)
/// the stub below keeps the summary local, which is exactly the offline path.
#[cfg(target_arch = "wasm32")]
async fn fetch_cohort(client_id: Option<String>, axis_levels: Vec<AxisLevel>) -> CohortState {
    use gloo_net::http::Request;

    let payload = CohortSubmission {
        client_id,
        axis_levels,
    };
    let request = match Request::post("/v1/cohort").json(&payload) {
        Ok(request) => request,
        Err(_) => return CohortState::Offline,
    };
    match request.send().await {
        Ok(response) if response.ok() => match response.json::<CohortEnvelope>().await {
            Ok(envelope) => CohortState::Online(envelope.data),
            Err(_) => CohortState::Offline,
        },
        _ => CohortState::Offline,
    }
}

#[cfg(not(target_arch = "wasm32"))]
async fn fetch_cohort(_client_id: Option<String>, _axis_levels: Vec<AxisLevel>) -> CohortState {
    CohortState::Offline
}

/// A stable, opaque idempotency key persisted in `localStorage` so a repeat
/// visit is not double-counted in the cohort (ADR 0006). It is a random token
/// only — no PII, no nominative link. `None` lets the server mint one.
#[cfg(target_arch = "wasm32")]
async fn stable_client_id() -> Option<String> {
    const JS: &str = "(function(){ try { var k='raip_cohort_id'; \
        var v=localStorage.getItem(k); \
        if(!v){ v=(self.crypto&&crypto.randomUUID)?crypto.randomUUID():String(Date.now())+Math.random(); \
        localStorage.setItem(k,v); } return v; } catch(e){ return ''; } })()";
    match document::eval(JS).join::<String>().await {
        Ok(id) if !id.is_empty() => Some(id),
        _ => None,
    }
}

#[cfg(not(target_arch = "wasm32"))]
async fn stable_client_id() -> Option<String> {
    None
}

/// The end-of-parcours screen: the private local synthesis, the anonymous cohort
/// comparison (fetched once, offline-first), and the export / restart controls.
#[component]
fn SummaryStage(
    summary: SummaryViewModel,
    axis_levels: Vec<AxisLevel>,
    on_restart: EventHandler<()>,
) -> Element {
    let mut cohort = use_signal(|| CohortState::Loading);
    let export = export_script(&summary_json(&summary));

    // Fire the offline-first cohort call once, on mount. Skipped entirely when
    // there is nothing to situate (an all-"idk" parcours).
    use_future(move || {
        let axis_levels = axis_levels.clone();
        async move {
            if axis_levels.is_empty() {
                cohort.set(CohortState::Offline);
                return;
            }
            let client_id = stable_client_id().await;
            cohort.set(fetch_cohort(client_id, axis_levels).await);
        }
    });

    rsx! {
        SummaryPanel { summary }
        CohortPanel { state: cohort() }
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
                onclick: move |_| on_restart.call(()),
                span { "Recommencer" }
                Keycap { legend: "R".to_string(), class: "mini".to_string() }
            }
        }
    }
}

/// Renders the anonymous cohort comparison. Loading and offline are quiet,
/// reassuring states — the local synthesis above already stands alone.
#[component]
fn CohortPanel(state: CohortState) -> Element {
    match state {
        CohortState::Loading => rsx! {
            section {
                class: "cohort-panel",
                "data-state": "loading",
                "aria-live": "polite",
                p { class: "cohort-note", "Comparaison anonyme à la cohorte…" }
            }
        },
        CohortState::Offline => rsx! {
            section { class: "cohort-panel", "data-state": "offline",
                p { class: "cohort-note",
                    "Hors ligne : synthèse locale uniquement. Rien n'a été envoyé."
                }
            }
        },
        CohortState::Online(positions) => rsx! {
            section { class: "cohort-panel", "data-state": "online",
                h2 { class: "cohort-title", "Situez-vous dans la cohorte" }
                p { class: "cohort-lede",
                    "Agrégé et anonyme : aucune position nominative, aucun classement."
                }
                for position in positions {
                    CohortAxis { position }
                }
            }
        },
    }
}

/// One risk axis: either the withheld notice (cohort below the k-anonymity
/// threshold) or the distribution across practice bands, with the learner's own
/// band marked.
#[component]
fn CohortAxis(position: DistributionPosition) -> Element {
    if !position.min_cohort_size_met {
        return rsx! {
            div { class: "cohort-axis", "data-withheld": "true",
                div { class: "cohort-axis-head", "{position.cohort_label}" }
                p { class: "cohort-withheld",
                    "Cohorte encore trop petite pour vous situer sans risque de ré-identification. Position masquée (k-anonymat)."
                }
            }
        };
    }
    let user_band = position.user_bucket.clone();
    rsx! {
        div { class: "cohort-axis",
            div { class: "cohort-axis-head", "{position.cohort_label}" }
            div { class: "cohort-bars",
                for bucket in position.buckets {
                    {
                        let is_you = user_band.as_deref() == Some(bucket.label.as_str());
                        let pct = format!("{:.0}", bucket.percent);
                        rsx! {
                            div {
                                class: if is_you { "cohort-bar you" } else { "cohort-bar" },
                                "data-you": if is_you { "true" } else { "false" },
                                span { class: "cohort-bar-label", "{bucket.label}" }
                                span { class: "cohort-bar-track", aria_hidden: "true",
                                    span { class: "cohort-bar-fill", style: "inline-size: {pct}%" }
                                }
                                span { class: "cohort-bar-pct", "{pct} %" }
                            }
                        }
                    }
                }
            }
            if let Some(band) = user_band {
                p { class: "cohort-you-note",
                    "Votre pratique sur cet axe : "
                    b { "{band}" }
                }
            }
        }
    }
}

/// The embedded content corpus (one YAML file per source axis-set). `include_str!`
/// bakes it into the wasm at build time — the client parses and drives it through
/// the engine at runtime, no filesystem, no backend.
const CORPUS_FILES: &[&str] = &[
    include_str!("../../../content/questions/bias-visual.yml"),
    include_str!("../../../content/questions/situations.yml"),
    include_str!("../../../content/questions/profiles.yml"),
    include_str!("../../../content/questions/deepfakes.yml"),
];

/// Sort rank for the learning curve: beginner → intermediate → advanced.
fn difficulty_rank(difficulty: Difficulty) -> u8 {
    match difficulty {
        Difficulty::Beginner => 0,
        Difficulty::Intermediate => 1,
        Difficulty::Advanced => 2,
    }
}

/// Number of drills drawn per play session.
const SESSION_SIZE: usize = 50;

/// Draw a session from the embedded bias-game corpus: up to `SESSION_SIZE`
/// valid drills, round-robin across risk axes for variety, then ordered by a
/// gentle difficulty curve. Kept separate from the view-model mapping so the
/// selection is testable on the real content.
fn parcours_questions() -> Vec<Question> {
    let mut all: Vec<Question> = Vec::new();
    for raw in CORPUS_FILES {
        if let Ok(questions) = parse_questions_yaml(raw) {
            all.extend(questions.into_iter().filter(|q| q.validate_basic().is_ok()));
        }
    }
    let mut session: Vec<Question> = interleave_by_axis(all)
        .into_iter()
        .take(SESSION_SIZE)
        .collect();
    // stable sort keeps the axis-interleaved order within a difficulty tier
    session.sort_by_key(|q| difficulty_rank(q.difficulty));
    session
}

/// Round-robin questions across their risk axes so a session mixes topics
/// instead of clustering all drills of one axis together.
fn interleave_by_axis(questions: Vec<Question>) -> Vec<Question> {
    let mut buckets: std::collections::BTreeMap<RiskAxis, Vec<Question>> =
        std::collections::BTreeMap::new();
    for q in questions {
        buckets.entry(q.axis).or_default().push(q);
    }
    let mut lists: Vec<Vec<Question>> = buckets.into_values().collect();
    for list in lists.iter_mut() {
        list.reverse(); // so pop() yields the original front order
    }
    let mut out = Vec::new();
    loop {
        let mut progressed = false;
        for list in lists.iter_mut() {
            if let Some(q) = list.pop() {
                out.push(q);
                progressed = true;
            }
        }
        if !progressed {
            break;
        }
    }
    out
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
    // A media_review drill shows its actual generated visual (the drill IS the
    // image); the AI-synthetic disclosure is rendered by the view (ADR 0004/0008).
    if let Some(file) = question.media.first() {
        return Some(ScenarioArtifact::Generated {
            src: format!("/assets/media/{file}"),
            alt: question.title.clone(),
        });
    }
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
        // the manifesto gate: everything is biased, non-competitive, with a start
        assert!(html.contains("Aucune image générée"));
        assert!(html.contains("jamais un classement"));
        assert!(html.contains("Commencer"));
        // the question is not shown before the gate
        assert!(!html.contains("radiogroup"));
        // no numeric scoring ever
        assert!(!html.contains("score_delta"));
        assert!(!html.contains("axis_impacts"));
    }

    #[test]
    fn corpus_is_a_bias_session_from_real_content() {
        let qs = corpus();
        // a play session: non-empty, capped at SESSION_SIZE, all engine-derived
        assert!(!qs.is_empty(), "session is non-empty");
        assert!(
            qs.len() <= SESSION_SIZE,
            "session capped at {SESSION_SIZE}, got {}",
            qs.len()
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
    fn session_is_capped_and_ordered_by_difficulty() {
        let session = parcours_questions();
        assert!(!session.is_empty());
        assert!(session.len() <= SESSION_SIZE);
        let ranks: Vec<u8> = session
            .iter()
            .map(|q| difficulty_rank(q.difficulty))
            .collect();
        assert!(
            ranks.windows(2).all(|w| w[0] <= w[1]),
            "session must be non-decreasing in difficulty, got {ranks:?}"
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

    #[test]
    fn cohort_axis_levels_are_empty_when_all_skipped() {
        let questions = parcours_questions();
        let answers = vec![None; questions.len()];
        assert!(
            axis_levels_from_answers(&questions, &answers).is_empty(),
            "an all-idk parcours has no per-axis levels to situate"
        );
    }

    #[test]
    fn cohort_axis_levels_reuse_the_engine_aggregation() {
        let questions = parcours_questions();
        assert!(!questions.is_empty());
        // answer only the first question, with its first choice
        let mut answers = vec![None; questions.len()];
        answers[0] = Some(questions[0].choices[0].id.clone());

        let levels = axis_levels_from_answers(&questions, &answers);
        assert!(
            !levels.is_empty(),
            "an answered question yields at least one axis level"
        );
        assert!(
            levels.iter().any(|level| level.axis == questions[0].axis),
            "the answered question's axis is levelled"
        );

        // The client payload is byte-for-byte what the server-side session path
        // computes — the UI reuses the engine, it does not re-score (ADR 0003).
        let mut state = start_session("cohort-test", questions.clone()).unwrap();
        submit_answer(
            &mut state,
            &questions[0].id,
            vec![questions[0].choices[0].id.clone()],
        )
        .unwrap();
        assert_eq!(levels, complete_session(&state).axis_levels);
    }

    #[test]
    fn summary_stage_renders_local_synthesis_and_cohort_region() {
        // The signal/future props must be built inside the Dioxus runtime, so
        // the stage is rendered through a harness component.
        #[component]
        fn Harness() -> Element {
            let summary = build_summary(&[("Confidentialité", MotifKind::Shield, None)]);
            rsx! {
                SummaryStage {
                    summary,
                    axis_levels: Vec::new(),
                    on_restart: move |_| {},
                }
            }
        }
        let html = dioxus_ssr::render_element(rsx! { Harness {} });
        // the private local synthesis stands on its own
        assert!(html.contains("summary-panel"));
        assert!(html.contains("Synthèse privée"));
        // the anonymous cohort region is present (offline-first placeholder)
        assert!(html.contains("cohort-panel"));
        // export + restart controls survive the extraction into SummaryStage
        assert!(html.contains("data-action=\"export\""));
        assert!(html.contains("data-action=\"restart\""));
    }

    #[test]
    fn cohort_panel_renders_online_distribution_and_withholds_below_k() {
        use rumble_ai_practices_domain::DistributionBucket;
        // one axis at/above k with the learner's own band marked, and one axis
        // below k whose position must be withheld (not shown as a distribution).
        let positions = vec![
            DistributionPosition {
                cohort_label: "Vérification des sources".into(),
                min_cohort_size_met: true,
                buckets: vec![
                    DistributionBucket {
                        label: "découverte".into(),
                        percent: 20.0,
                    },
                    DistributionBucket {
                        label: "référence".into(),
                        percent: 80.0,
                    },
                ],
                user_bucket: Some("référence".into()),
            },
            DistributionPosition {
                cohort_label: "Confidentialité".into(),
                min_cohort_size_met: false,
                buckets: vec![],
                user_bucket: None,
            },
        ];
        let html = dioxus_ssr::render_element(
            rsx! { CohortPanel { state: CohortState::Online(positions) } },
        );

        // the distribution renders: bands, a percentage, and the learner's band
        assert!(html.contains("cohort-bars"));
        assert!(html.contains("80 %"));
        assert!(html.contains("data-you=\"true\""));
        assert!(html.contains("Vérification des sources"));
        // the sub-k axis is withheld with an explicit k-anonymity notice
        assert!(html.contains("data-withheld=\"true\""));
        assert!(html.contains("k-anonymat"));
    }
}
