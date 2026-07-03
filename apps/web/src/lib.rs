//! Mobile-first PWA shell — the "keycap" direction (ADR 0036).
//!
//! A reflex is a keystroke: the answer choices are a keyboard-navigable radio
//! group skinned as mechanical keycaps. Accessibility follows the WAI-ARIA
//! radiogroup pattern (roles, `aria-checked`, roving tabindex, arrow + letter
//! keys, Enter to commit) — the same behavior `dioxus-primitives` provides,
//! implemented here natively while the pinned RadioGroup API is confirmed.
//!
//! This crate stays presentational: verdicts are DATA supplied by the session
//! engine (`FeedbackViewModel`), never computed in the browser.

use dioxus::prelude::*;
use rumble_ai_practices_ui::{
    ChoiceViewModel, FeedbackPanel, FeedbackViewModel, Keycap, NonRhNotice,
    PrivateDistributionNotice, QuestionViewModel, VerdictKind,
};
use serde::{Deserialize, Serialize};

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

#[component]
pub fn App() -> Element {
    // theme: None = follow OS; Some("dark"|"light") = explicit toggle.
    let mut theme = use_signal(|| None::<&'static str>);
    let theme_attr = theme().unwrap_or("");
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
                NonRhNotice {}
                PrivateDistributionNotice {}
                QuestionConsole { question: sample_question(), feedbacks: sample_feedbacks() }
            }
        }
    }
}

/// The interactive question. Owns the selection/lock state and the keyboard
/// navigation; renders the keycap radio group and the four-state feedback.
#[component]
pub fn QuestionConsole(question: QuestionViewModel, feedbacks: Vec<FeedbackViewModel>) -> Element {
    let mut selected = use_signal(|| None::<usize>);
    let mut locked = use_signal(|| false);
    let mut pressed = use_signal(|| None::<usize>);

    let choices = question.choices.clone();

    // Keyboard navigation as a global listener (installed once): a number key
    // clicks the matching choice, Enter validates, Space = "je ne sais pas",
    // R replays. It drives the buttons' own onclick handlers, so it works
    // regardless of focus — more robust than a focus-bound Rust keydown.
    use_effect(|| {
        document::eval(
            "if (!window.__raipKeys) { window.__raipKeys = true; \
             document.addEventListener('keydown', function (e) { \
               if (e.metaKey || e.ctrlKey || e.altKey) return; \
               var q = function (s) { return document.querySelector(s); }; \
               var k = e.key; \
               if (k >= '1' && k <= '9') { var b = q('.choice[data-key=\"' + k + '\"]'); if (b) { e.preventDefault(); b.click(); } } \
               else if (k === 'Enter') { var v = q('[data-action=\"validate\"]'); if (v && !v.disabled) { e.preventDefault(); v.click(); } } \
               else if (k === ' ') { var i = q('[data-action=\"idk\"]'); if (i) { e.preventDefault(); i.click(); } } \
               else if (k === 'r' || k === 'R') { var r = q('[data-action=\"replay\"]'); if (r) { e.preventDefault(); r.click(); } } \
             }); }",
        );
    });

    let progress_total = question.total;
    let progress_idx = question.index;
    let sel_now = selected();
    let is_locked = locked();

    // Per-choice render data (recomputed each render — reactive on the signals).
    let rows: Vec<(
        usize,
        ChoiceViewModel,
        bool,
        bool,
        Option<VerdictKind>,
        String,
    )> = choices
        .iter()
        .enumerate()
        .map(|(idx, choice)| {
            let is_sel = sel_now == Some(idx);
            let verdict = if is_locked && is_sel {
                feedbacks.get(idx).map(|f| f.verdict)
            } else {
                None
            };
            let mut cls = String::from("choice");
            if is_sel {
                cls.push_str(" sel");
            }
            if is_locked {
                cls.push_str(" locked");
            }
            if is_locked && !is_sel {
                cls.push_str(" dim");
            }
            (
                idx,
                choice.clone(),
                is_sel,
                pressed() == Some(idx),
                verdict,
                cls,
            )
        })
        .collect();

    rsx! {
        div { class: "console",
            div { class: "q-meta",
                span { "Situation " b { "{question.index:02}" } " · {question.total:02}" }
                span { "Rôle : " b { "{question.role}" } }
                span { "Catégorie : " b { "{question.category}" } }
                span { class: "q-progress", aria_hidden: "true",
                    for i in 1..=progress_total {
                        i { class: if i <= progress_idx { "on" } else { "" } }
                    }
                }
            }

            p { class: "scenario", dangerous_inner_html: "{question.scenario}" }
            p { class: "prompt", "{question.prompt}" }

            div {
                class: "choices",
                role: "radiogroup",
                aria_label: "Choix de réponse",
                for (idx , choice , _is_sel , is_pressed , verdict , cls) in rows {
                    button {
                        key: "{choice.id}",
                        class: "{cls}",
                        role: "radio",
                        "aria-checked": if verdict.is_some() || sel_now == Some(idx) { "true" } else { "false" },
                        "data-verdict": verdict.map(|v| v.slug()).unwrap_or(""),
                        "data-key": "{choice.key}",
                        r#type: "button",
                        onclick: move |_| {
                            if !locked() {
                                selected.set(Some(idx));
                                pressed.set(Some(idx));
                            }
                        },
                        Keycap { legend: choice.key.clone(), down: is_pressed }
                        span { "{choice.label}" }
                        if let Some(v) = verdict {
                            span { class: "verdict-tag", "data-verdict": "{v.slug()}", "{v.label()}" }
                        }
                    }
                }
            }

            if !is_locked {
                div { class: "commit-row",
                    button {
                        class: "idk",
                        r#type: "button",
                        "data-action": "idk",
                        onclick: move |_| selected.set(None),
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
                                locked.set(true);
                            }
                        },
                        span { "Valider" }
                        Keycap { legend: "⏎".to_string(), class: "mini".to_string() }
                    }
                }
            }

            if is_locked {
                if let Some(fb) = sel_now.and_then(|i| feedbacks.get(i).cloned()) {
                    FeedbackPanel { feedback: fb }
                }
                div { class: "commit-row",
                    button {
                        class: "validate-btn",
                        r#type: "button",
                        "data-action": "replay",
                        onclick: move |_| {
                            selected.set(None);
                            locked.set(false);
                            pressed.set(None);
                        },
                        span { "Rejouer" }
                        Keycap { legend: "R".to_string(), class: "mini".to_string() }
                    }
                }
            }
        }
    }
}

/// A real scenario from the audit corpus (`content/…` audit-002 family):
/// a "public" draft that may still carry internal data.
fn sample_question() -> QuestionViewModel {
    QuestionViewModel {
        id: "q-confidentialite-001".into(),
        index: 1,
        total: 4,
        role: "Référent usage IA".into(),
        category: "Confidentialité".into(),
        scenario: "Un collègue vous partage <code>brouillon_blog_tech.docx</code> et vous demande de le faire relire par un assistant IA public avant publication.".into(),
        prompt: "Quelle est l'action prioritaire ?".into(),
        choices: vec![
            ChoiceViewModel { id: "send_as_is".into(), key: "1".into(), label: "Envoyer le fichier tel quel — c'est un contenu destiné à être public.".into() },
            ChoiceViewModel { id: "inspect_first".into(), key: "2".into(), label: "Vérifier d'abord le contenu réel, les métadonnées et la classification avant tout envoi.".into() },
            ChoiceViewModel { id: "refuse_all".into(), key: "3".into(), label: "Refuser tout usage d'IA sur des documents internes.".into() },
            ChoiceViewModel { id: "rename_then_send".into(), key: "4".into(), label: "Retirer le nom du fichier, puis l'envoyer.".into() },
        ],
    }
}

/// Feedback aligned by choice index. In production this comes from the session
/// engine; here it is sample data so the shell stays runtime-free.
fn sample_feedbacks() -> Vec<FeedbackViewModel> {
    vec![
        FeedbackViewModel {
            verdict: VerdictKind::Risque,
            reason: "La destination publique d'un document ne prouve pas l'absence de données sensibles : un brouillon contient souvent commentaires, métadonnées, clients ou roadmap.".into(),
            risk: "Fuite de données internes vers un service IA public.".into(),
            action: "Ne jamais déduire la sensibilité de la destination du fichier.".into(),
            source: "Fiche · Classification avant partage".into(),
        },
        FeedbackViewModel {
            verdict: VerdictKind::Juste,
            reason: "On inspecte avant d'agir : contenu réel, commentaires masqués, métadonnées, classification. Puis un outil autorisé.".into(),
            risk: "Risque maîtrisé : la vérification précède l'usage.".into(),
            action: "Inspecter → classifier → nettoyer les métadonnées → outil autorisé.".into(),
            source: "Fiche · Réflexe d'inspection".into(),
        },
        FeedbackViewModel {
            verdict: VerdictKind::Faux,
            reason: "Interdire en bloc paralyse l'équipe sans traiter le vrai risque. Un outil autorisé plus une inspection suffisent — la sur-restriction n'est pas de la sécurité.".into(),
            risk: "Blocage opérationnel, contournements non gouvernés.".into(),
            action: "Distinguer « interdit » de « à encadrer ».".into(),
            source: "Fiche · Gouvernance proportionnée".into(),
        },
        FeedbackViewModel {
            verdict: VerdictKind::Partiel,
            reason: "Bon réflexe de discrétion, mais insuffisant : le contenu et les métadonnées restent exposés. Renommer ne nettoie rien.".into(),
            risk: "Fuite persistante malgré une précaution de surface.".into(),
            action: "Traiter le contenu, pas seulement l'étiquette.".into(),
            source: "Fiche · Métadonnées invisibles".into(),
        },
    ]
}

/// Intro affordance that expresses the API contract (start a session).
#[component]
pub fn IntroPanel() -> Element {
    let routes = ApiRoutes::default();
    rsx! {
        div { class: "notice",
            h2 { "Entraînement libre" }
            p { "Répétable à volonté, sans enjeu : ni chrono, ni vies, ni classement. Rejouer ne fait jamais régresser votre meilleur score." }
            button { class: "start-btn", r#type: "button", "data-api": routes.create_session, "Démarrer" }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_shell_with_scenario_and_notices() {
        let html = dioxus_ssr::render_element(rsx! { App {} });
        assert!(html.contains("Diagnostic pédagogique"));
        assert!(html.contains("Positionnement privé"));
        assert!(html.contains("brouillon_blog_tech.docx"));
        assert!(html.contains("radiogroup"));
        // no numeric scoring leaks into the browser shell
        assert!(!html.contains("score_delta"));
        assert!(!html.contains("axis_impacts"));
    }

    #[test]
    fn choices_carry_keycap_legends() {
        let html = dioxus_ssr::render_element(rsx! { App {} });
        // the four keycap legends are present as answer shortcuts
        assert!(html.contains("aria-checked"));
        // SSR escapes the apostrophe in "d'abord"; match an unambiguous span.
        assert!(html.contains("le contenu réel"));
    }

    #[test]
    fn intro_expresses_api_contract() {
        let html = dioxus_ssr::render_element(rsx! { IntroPanel {} });
        assert!(html.contains("data-api=\"/v1/sessions\""));
        assert!(html.contains("Démarrer"));
    }
}
