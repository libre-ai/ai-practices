//! Mobile-first PWA shell contract.
//!
//! This crate deliberately contains no scoring or content validation logic.
//! It composes UI components and a typed client workflow contract. Runtime HTTP
//! wiring can be added without moving business rules into the browser.

use dioxus::prelude::*;
use rumble_ai_practices_ui::{
    ChoiceViewModel, FeedbackPanel, FeedbackViewModel, NonRhNotice, PrivateDistributionNotice,
    QuestionCard, QuestionViewModel, SummaryPanel, SummaryViewModel,
};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
pub fn render_app_html() -> String {
    dioxus_ssr::render_element(rsx! { App {} })
}

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrainingStage {
    Intro,
    Question,
    Feedback,
    Summary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrainingFlowModel {
    pub stage: TrainingStage,
    pub session_id: Option<String>,
    pub question: Option<QuestionViewModel>,
    pub feedback: Option<FeedbackViewModel>,
    pub summary: Option<SummaryViewModel>,
}

impl TrainingFlowModel {
    pub fn intro() -> Self {
        Self {
            stage: TrainingStage::Intro,
            session_id: None,
            question: None,
            feedback: None,
            summary: None,
        }
    }
}

#[component]
pub fn App() -> Element {
    rsx! {
        main { class: "app-shell",
            h1 { "Rumble AI Practices" }
            NonRhNotice {}
            PrivateDistributionNotice {}
            TrainingFlow { model: sample_flow() }
        }
    }
}

#[component]
pub fn TrainingFlow(model: TrainingFlowModel) -> Element {
    let routes = ApiRoutes::default();
    rsx! {
        section { class: "training-flow", "data-stage": "{stage_name(&model.stage)}",
            match model.stage {
                TrainingStage::Intro => rsx! {
                    div { class: "next-step",
                        h2 { "Parcours MVP" }
                        p { "Démarrer un entraînement aux réflexes IA : confidentialité, sources, biais média et responsabilité." }
                        button { "data-api": routes.create_session, "Démarrer" }
                    }
                },
                TrainingStage::Question => rsx! {
                    if let Some(question) = model.question.clone() {
                        QuestionCard { question }
                    }
                },
                TrainingStage::Feedback => rsx! {
                    if let Some(feedback) = model.feedback.clone() {
                        FeedbackPanel { feedback }
                    }
                    button { "data-api-template": routes.next_question_template, "Question suivante" }
                },
                TrainingStage::Summary => rsx! {
                    if let Some(summary) = model.summary.clone() {
                        SummaryPanel { summary }
                    }
                },
            }
        }
    }
}

fn stage_name(stage: &TrainingStage) -> &'static str {
    match stage {
        TrainingStage::Intro => "intro",
        TrainingStage::Question => "question",
        TrainingStage::Feedback => "feedback",
        TrainingStage::Summary => "summary",
    }
}

fn sample_flow() -> TrainingFlowModel {
    TrainingFlowModel {
        stage: TrainingStage::Question,
        session_id: Some("sample".into()),
        question: Some(sample_question()),
        feedback: Some(sample_feedback()),
        summary: Some(sample_summary()),
    }
}

fn sample_question() -> QuestionViewModel {
    QuestionViewModel {
        id: "q-source-001".into(),
        title: "Demander des sources ne suffit pas".into(),
        scenario: "Vous demandez à un assistant IA de fournir des liens.".into(),
        prompt: "Quelle action est nécessaire avant de partager la réponse ?".into(),
        choices: vec![
            ChoiceViewModel {
                id: "verify_links".into(),
                label: "Vérifier existence, contenu, date et autorité des sources.".into(),
            },
            ChoiceViewModel {
                id: "trust_links".into(),
                label: "Partager directement car des liens sont fournis.".into(),
            },
        ],
    }
}

fn sample_feedback() -> FeedbackViewModel {
    FeedbackViewModel {
        level: "exemple".into(),
        messages: vec!["Le feedback réel vient du moteur de session Rust.".into()],
    }
}

fn sample_summary() -> SummaryViewModel {
    SummaryViewModel {
        answered_count: 0,
        strengths: vec!["Aucun résultat calculé dans le shell statique.".into()],
        recommendations: vec!["Brancher le client sur l'API /v1/sessions.".into()],
        privacy_notice: "Aucun classement nominatif.".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_pwa_shell_copy() {
        let html = dioxus_ssr::render_element(rsx! { App {} });
        assert!(html.contains("Rumble AI Practices"));
        assert!(html.contains("Diagnostic pédagogique"));
        assert!(html.contains("Positionnement privé"));
        assert!(html.contains("Demander des sources ne suffit pas"));
        assert!(html.contains("classement nominatif"));
    }

    #[test]
    fn renders_intro_with_api_contract() {
        let html =
            dioxus_ssr::render_element(rsx! { TrainingFlow { model: TrainingFlowModel::intro() } });
        assert!(html.contains("data-api=\"/v1/sessions\""));
        assert!(html.contains("Démarrer"));
    }

    #[test]
    fn renders_feedback_stage_without_scores() {
        let html = dioxus_ssr::render_element(rsx! {
            TrainingFlow { model: TrainingFlowModel {
                stage: TrainingStage::Feedback,
                session_id: Some("s1".into()),
                question: None,
                feedback: Some(FeedbackViewModel {
                    level: "correct".into(),
                    messages: vec!["Bonne pratique".into()],
                }),
                summary: None,
            } }
        });
        assert!(html.contains("Bonne pratique"));
        assert!(!html.contains("score_delta"));
        assert!(!html.contains("axis_impacts"));
    }
}
