//! Dioxus UI contracts for `rumble-ai-practices` — the "keycap" direction.
//!
//! Components here stay presentational. Scoring, validation and session
//! decisions belong to the Rust core crates, not to UI: a verdict arrives as
//! data (`VerdictKind` on a `FeedbackViewModel`), it is never computed here.
//!
//! Appearance is tokens-only: components carry semantic classes and a
//! `data-verdict` slug; the colors live in `assets/tokens.css`.

use dioxus::prelude::*;
use dioxus_primitives::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use serde::{Deserialize, Serialize};

/// The four-state verdict spectrum. The product refuses the binary quiz: an
/// answer can be right, partial, risky, or wrong — never just pass/fail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerdictKind {
    Juste,
    Partiel,
    Risque,
    Faux,
}

impl VerdictKind {
    /// CSS slug consumed by `[data-verdict="…"]` in tokens.css.
    pub fn slug(self) -> &'static str {
        match self {
            VerdictKind::Juste => "juste",
            VerdictKind::Partiel => "partiel",
            VerdictKind::Risque => "risque",
            VerdictKind::Faux => "faux",
        }
    }

    /// Human label shown on the locked keycap and the feedback header.
    pub fn label(self) -> &'static str {
        match self {
            VerdictKind::Juste => "Juste",
            VerdictKind::Partiel => "Partiel",
            VerdictKind::Risque => "Risqué",
            VerdictKind::Faux => "Incorrect",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesignToken {
    pub name: String,
    pub value: String,
}

pub fn base_tokens() -> Vec<DesignToken> {
    vec![
        DesignToken {
            name: "tone".into(),
            value: "pedagogical".into(),
        },
        DesignToken {
            name: "comparison".into(),
            value: "private-distribution-no-leaderboard".into(),
        },
        DesignToken {
            name: "navigation".into(),
            value: "keycap-keyboard-first".into(),
        },
    ]
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestionViewModel {
    pub id: String,
    pub index: usize,
    pub total: usize,
    pub role: String,
    pub category: String,
    pub scenario: String,
    pub prompt: String,
    pub choices: Vec<ChoiceViewModel>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChoiceViewModel {
    pub id: String,
    /// Keycap legend, e.g. "A". Drives the keyboard shortcut too.
    pub key: String,
    pub label: String,
}

/// Feedback for a single answer. All fields are data supplied by the session
/// engine; the UI only renders them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeedbackViewModel {
    pub verdict: VerdictKind,
    pub reason: String,
    pub risk: String,
    pub action: String,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SummaryViewModel {
    pub answered_count: usize,
    pub strengths: Vec<String>,
    pub recommendations: Vec<String>,
    pub privacy_notice: String,
}

/// A single mechanical keycap. `down` renders the pressed (actuated) state.
#[component]
pub fn Keycap(
    legend: String,
    #[props(default)] class: String,
    #[props(default)] down: bool,
) -> Element {
    let state = if down { "cap is-down" } else { "cap" };
    rsx! {
        span { class: "{state} {class}", aria_hidden: "true", "{legend}" }
    }
}

#[component]
pub fn NonRhNotice() -> Element {
    rsx! {
        section { class: "notice notice-privacy",
            h2 { "Diagnostic pédagogique" }
            p { "Ce parcours entraîne des réflexes IA. Il ne produit pas de classement nominatif ni de décision RH." }
        }
    }
}

#[component]
pub fn PrivateDistributionNotice() -> Element {
    rsx! {
        section { class: "notice notice-distribution",
            h2 { "Positionnement privé" }
            p { "La distribution anonymisée sert à se situer par intervalle, sans leaderboard ni rang individuel public." }
        }
    }
}

/// The four-state feedback. Given a verdict, renders the calm reveal:
/// reason → risk in play → action to keep → reflex card.
#[component]
pub fn FeedbackPanel(feedback: FeedbackViewModel) -> Element {
    let slug = feedback.verdict.slug();
    rsx! {
        section {
            class: "feedback-panel",
            "data-verdict": "{slug}",
            "aria-live": "polite",
            div { class: "fb-verdict",
                span { class: "dot" }
                span { "{feedback.verdict.label()} — voici pourquoi" }
            }
            p { "{feedback.reason}" }
            // The detail folds under a headless, accessible Collapsible
            // (dioxus-primitives, ADR 0036). Verdict + reason stay visible;
            // the reflex breakdown is one keystroke away.
            Collapsible { class: "fb-collapsible", default_open: true,
                CollapsibleTrigger { class: "fb-trigger", "Détail du réflexe" }
                CollapsibleContent {
                    div { class: "fb-grid",
                        div { class: "fb-cell",
                            h4 { "Risque en jeu" }
                            p { "{feedback.risk}" }
                        }
                        div { class: "fb-cell",
                            h4 { "Action à retenir" }
                            p { "{feedback.action}" }
                        }
                        div { class: "fb-cell",
                            h4 { "Fiche réflexe" }
                            p { "{feedback.source}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SummaryPanel(summary: SummaryViewModel) -> Element {
    rsx! {
        section { class: "summary-panel",
            h2 { "Synthèse privée" }
            p { "Questions répondues : {summary.answered_count}" }
            h3 { "Points forts" }
            ul {
                for strength in summary.strengths {
                    li { "{strength}" }
                }
            }
            h3 { "À renforcer" }
            ul {
                for recommendation in summary.recommendations {
                    li { "{recommendation}" }
                }
            }
            p { class: "privacy", "{summary.privacy_notice}" }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_no_leaderboard_token() {
        assert!(
            base_tokens()
                .iter()
                .any(|token| token.value.contains("no-leaderboard"))
        );
    }

    #[test]
    fn verdict_slugs_and_labels_are_stable() {
        assert_eq!(VerdictKind::Juste.slug(), "juste");
        assert_eq!(VerdictKind::Risque.label(), "Risqué");
        assert_eq!(VerdictKind::Faux.label(), "Incorrect");
    }

    #[test]
    fn renders_non_rh_notice() {
        let html = dioxus_ssr::render_element(rsx! { NonRhNotice {} });
        assert!(html.contains("Diagnostic pédagogique"));
        assert!(html.contains("décision RH"));
    }

    #[test]
    fn renders_private_distribution_notice() {
        let html = dioxus_ssr::render_element(rsx! { PrivateDistributionNotice {} });
        assert!(html.contains("Positionnement privé"));
        assert!(html.contains("sans leaderboard"));
    }

    #[test]
    fn feedback_carries_verdict_but_no_numeric_score() {
        let html = dioxus_ssr::render_element(rsx! {
            FeedbackPanel { feedback: FeedbackViewModel {
                verdict: VerdictKind::Juste,
                reason: "On inspecte avant d'agir.".into(),
                risk: "Fuite maîtrisée.".into(),
                action: "Inspecter puis outil autorisé.".into(),
                source: "Fiche réflexe d'inspection".into(),
            } }
        });
        assert!(html.contains("data-verdict=\"juste\""));
        // SSR escapes the apostrophe in "d'agir", so match an unambiguous span.
        assert!(html.contains("On inspecte avant"));
        assert!(html.contains("Juste"));
        assert!(!html.contains("score_delta"));
        assert!(!html.contains("axis_impacts"));
    }

    #[test]
    fn renders_summary_without_ranking() {
        let html = dioxus_ssr::render_element(rsx! {
            SummaryPanel { summary: SummaryViewModel {
                answered_count: 1,
                strengths: vec!["Sources".into()],
                recommendations: vec!["Données".into()],
                privacy_notice: "Aucun classement nominatif.".into(),
            } }
        });
        assert!(html.contains("Synthèse privée"));
        assert!(html.contains("Aucun classement nominatif."));
    }
}
