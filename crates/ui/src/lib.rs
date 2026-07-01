//! Dioxus UI contracts for `rumble-ai-practices`.
//!
//! Components here must stay presentational. Scoring, validation and session
//! decisions belong to Rust core crates, not to UI components.

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

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
    ]
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestionViewModel {
    pub id: String,
    pub title: String,
    pub scenario: String,
    pub prompt: String,
    pub choices: Vec<ChoiceViewModel>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChoiceViewModel {
    pub id: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeedbackViewModel {
    pub level: String,
    pub messages: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SummaryViewModel {
    pub answered_count: usize,
    pub strengths: Vec<String>,
    pub recommendations: Vec<String>,
    pub privacy_notice: String,
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

#[component]
pub fn QuestionCard(question: QuestionViewModel) -> Element {
    rsx! {
        article { class: "question-card", "data-question-id": "{question.id}",
            h2 { "{question.title}" }
            p { class: "scenario", "{question.scenario}" }
            p { class: "prompt", "{question.prompt}" }
            ul { class: "choices",
                for choice in question.choices {
                    li { key: "{choice.id}",
                        button { "data-choice-id": "{choice.id}", "{choice.label}" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn FeedbackPanel(feedback: FeedbackViewModel) -> Element {
    rsx! {
        section { class: "feedback-panel", "data-level": "{feedback.level}",
            h2 { "Feedback" }
            p { "Niveau : {feedback.level}" }
            ul {
                for message in feedback.messages {
                    li { "{message}" }
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
    fn renders_question_without_answer_metadata() {
        let html = dioxus_ssr::render_element(rsx! {
            QuestionCard { question: QuestionViewModel {
                id: "q-test".into(),
                title: "Titre".into(),
                scenario: "Scénario".into(),
                prompt: "Que faire ?".into(),
                choices: vec![ChoiceViewModel { id: "good".into(), label: "Vérifier".into() }],
            } }
        });
        assert!(html.contains("Vérifier"));
        assert!(!html.contains("score"));
        assert!(!html.contains("feedback"));
    }

    #[test]
    fn renders_feedback_and_summary() {
        let feedback_html = dioxus_ssr::render_element(rsx! {
            FeedbackPanel { feedback: FeedbackViewModel {
                level: "correct".into(),
                messages: vec!["Bonne pratique".into()],
            } }
        });
        assert!(feedback_html.contains("Bonne pratique"));

        let summary_html = dioxus_ssr::render_element(rsx! {
            SummaryPanel { summary: SummaryViewModel {
                answered_count: 1,
                strengths: vec!["Sources".into()],
                recommendations: vec!["Données".into()],
                privacy_notice: "Privé".into(),
            } }
        });
        assert!(summary_html.contains("Synthèse privée"));
        assert!(summary_html.contains("Privé"));
    }
}
