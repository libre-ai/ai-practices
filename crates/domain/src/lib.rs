//! Pure domain contracts for `rumble-ai-practices`.
//!
//! This crate owns pedagogical invariants. It must not depend on API, CLI,
//! storage, UI, network, or model-provider code.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("question id must start with `q-`: {0}")]
    InvalidQuestionId(String),
    #[error("question `{0}` must contain at least two choices")]
    NotEnoughChoices(String),
    #[error("question `{0}` must contain at least one positive choice")]
    NoPositiveChoice(String),
    #[error("choice `{choice_id}` in question `{question_id}` is missing feedback")]
    MissingChoiceFeedback {
        question_id: String,
        choice_id: String,
    },
    #[error("approved question `{0}` must include at least one source")]
    ApprovedQuestionMissingSource(String),
    #[error("approved question `{0}` must include at least one reviewer")]
    ApprovedQuestionMissingReviewer(String),
    #[error("approved question `{0}` must include a review date")]
    ApprovedQuestionMissingReviewDate(String),
    #[error("duplicate choice id `{choice_id}` in question `{question_id}`")]
    DuplicateChoiceId {
        question_id: String,
        choice_id: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct QuestionId(String);

impl QuestionId {
    pub fn parse(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();
        if value.starts_with("q-") {
            Ok(Self(value))
        } else {
            Err(DomainError::InvalidQuestionId(value))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for QuestionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationStatus {
    Draft,
    Review,
    Approved,
    Blocked,
    Retired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskAxis {
    DataConfidentiality,
    SourceVerification,
    Hallucination,
    BiasFairness,
    Security,
    PrivacyRgpd,
    HumanResponsibility,
    PromptPractice,
    BusinessContext,
    MediaSynthetic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InteractionType {
    SingleChoice,
    MultipleChoice,
    RankedActions,
    MediaReview,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Question {
    pub id: QuestionId,
    pub version: u32,
    pub status: PublicationStatus,
    pub locale: String,
    pub title: String,
    pub axis: RiskAxis,
    pub difficulty: Difficulty,
    pub intent: String,
    pub context: ScenarioContext,
    pub prompt: String,
    pub interaction: Interaction,
    pub choices: Vec<Choice>,
    pub expected_reasoning: Vec<String>,
    pub risks: Vec<String>,
    #[serde(default)]
    pub sources: Vec<SourceRef>,
    #[serde(default)]
    pub media: Vec<String>,
    pub review: ReviewMetadata,
}

impl Question {
    pub fn validate_basic(&self) -> Result<(), DomainError> {
        if !self.id.as_str().starts_with("q-") {
            return Err(DomainError::InvalidQuestionId(self.id.to_string()));
        }

        if self.choices.len() < 2 {
            return Err(DomainError::NotEnoughChoices(self.id.to_string()));
        }

        let mut seen = BTreeSet::new();
        let mut has_positive = false;
        for choice in &self.choices {
            if !seen.insert(choice.id.clone()) {
                return Err(DomainError::DuplicateChoiceId {
                    question_id: self.id.to_string(),
                    choice_id: choice.id.clone(),
                });
            }
            if choice.feedback.trim().is_empty() {
                return Err(DomainError::MissingChoiceFeedback {
                    question_id: self.id.to_string(),
                    choice_id: choice.id.clone(),
                });
            }
            if choice.score > 0.0 {
                has_positive = true;
            }
        }

        if !has_positive {
            return Err(DomainError::NoPositiveChoice(self.id.to_string()));
        }

        Ok(())
    }

    pub fn validate_for_publication(&self) -> Result<(), DomainError> {
        self.validate_basic()?;
        if self.status == PublicationStatus::Approved {
            if self.sources.is_empty() {
                return Err(DomainError::ApprovedQuestionMissingSource(
                    self.id.to_string(),
                ));
            }
            if self.review.reviewers.is_empty() {
                return Err(DomainError::ApprovedQuestionMissingReviewer(
                    self.id.to_string(),
                ));
            }
            if self
                .review
                .last_reviewed_at
                .as_deref()
                .unwrap_or("")
                .trim()
                .is_empty()
            {
                return Err(DomainError::ApprovedQuestionMissingReviewDate(
                    self.id.to_string(),
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScenarioContext {
    #[serde(default)]
    pub role: Option<String>,
    pub scenario: String,
    #[serde(default)]
    pub assets: Vec<ScenarioAsset>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScenarioAsset {
    #[serde(rename = "type")]
    pub kind: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Interaction {
    #[serde(rename = "type")]
    pub kind: InteractionType,
    #[serde(default)]
    pub min_choices: Option<u32>,
    #[serde(default)]
    pub max_choices: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Choice {
    pub id: String,
    pub label: String,
    pub score: f64,
    #[serde(default)]
    pub severity: Option<Severity>,
    pub feedback: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceRef {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(rename = "type")]
    pub kind: SourceKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    InternalPolicy,
    PublicGuidance,
    Law,
    Standard,
    Documentation,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewMetadata {
    pub author: String,
    #[serde(default)]
    pub reviewers: Vec<String>,
    #[serde(default)]
    pub last_reviewed_at: Option<String>,
    pub confidence: Confidence,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Confidence {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluationLevel {
    Correct,
    Partial,
    Risky,
    Incorrect,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnswerEvaluation {
    pub question_id: QuestionId,
    pub selected_choice_ids: Vec<String>,
    pub score_delta: f64,
    pub level: EvaluationLevel,
    pub axis_impacts: Vec<AxisImpact>,
    pub feedback_cards: Vec<FeedbackCard>,
    pub evidence_refs: Vec<SourceRef>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AxisImpact {
    pub axis: RiskAxis,
    pub score_delta: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeedbackCard {
    pub choice_id: String,
    pub message: String,
    #[serde(default)]
    pub severity: Option<Severity>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionSummary {
    pub session_id: String,
    pub completed_at: String,
    pub answered_count: usize,
    pub axis_levels: Vec<AxisLevel>,
    /// One anonymised distribution position per pedagogical axis (ADR 0003).
    /// Empty until a cohort backend fills it (ADR 0006); each carries its own
    /// `min_cohort_size_met` flag when a threshold is not met.
    #[serde(default)]
    pub private_distributions: Vec<DistributionPosition>,
    pub strengths: Vec<String>,
    pub recommended_practices: Vec<String>,
    pub next_modules: Vec<ModuleRef>,
    pub privacy_notice: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AxisLevel {
    pub axis: RiskAxis,
    pub level: PracticeLevel,
    pub score: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PracticeLevel {
    Discovery,
    GuidedPractice,
    CarefulAutonomy,
    Reference,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DistributionPosition {
    pub cohort_label: String,
    pub min_cohort_size_met: bool,
    pub buckets: Vec<DistributionBucket>,
    #[serde(default)]
    pub user_bucket: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DistributionBucket {
    pub label: String,
    pub percent: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleRef {
    pub id: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaReview {
    pub id: String,
    pub status: PublicationStatus,
    #[serde(rename = "type")]
    pub kind: MediaKind,
    pub origin: MediaOrigin,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub prompt: Option<String>,
    pub purpose: String,
    pub contains_human_like_person: bool,
    #[serde(default)]
    pub consent_or_rights: Option<String>,
    #[serde(default)]
    pub labels: Option<MediaLabels>,
    pub bias_review: BiasReview,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaKind {
    Image,
    Video,
    Audio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaOrigin {
    GeneratedAi,
    Stock,
    Internal,
    Screenshot,
    SyntheticNonHuman,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaLabels {
    #[serde(default)]
    pub synthetic_disclosed: bool,
    #[serde(default)]
    pub watermark_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BiasReview {
    #[serde(default)]
    pub stereotypes_detected: Vec<String>,
    #[serde(default)]
    pub representation_risks: Vec<String>,
    #[serde(default)]
    pub protected_attributes_inferred: Vec<String>,
    pub realism_risk: RiskLevel,
    pub deepfake_confusion_risk: RiskLevel,
    pub reviewer: String,
    pub decision: BiasReviewDecision,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BiasReviewDecision {
    Approved,
    Rework,
    Blocked,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_question() -> Question {
        Question {
            id: QuestionId::parse("q-test-001").unwrap(),
            version: 1,
            status: PublicationStatus::Draft,
            locale: "fr-FR".into(),
            title: "Test".into(),
            axis: RiskAxis::SourceVerification,
            difficulty: Difficulty::Beginner,
            intent: "Tester une règle métier suffisamment longue.".into(),
            context: ScenarioContext {
                role: None,
                scenario: "Une situation de test suffisamment claire.".into(),
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
                    label: "Bonne action".into(),
                    score: 1.0,
                    severity: None,
                    feedback: "Explication utile.".into(),
                },
                Choice {
                    id: "bad".into(),
                    label: "Mauvaise action".into(),
                    score: -1.0,
                    severity: Some(Severity::High),
                    feedback: "Explication du risque.".into(),
                },
            ],
            expected_reasoning: vec!["raison".into()],
            risks: vec!["risk".into()],
            sources: vec![],
            media: vec![],
            review: ReviewMetadata {
                author: "tester".into(),
                reviewers: vec![],
                last_reviewed_at: None,
                confidence: Confidence::Medium,
                notes: None,
            },
        }
    }

    #[test]
    fn question_id_requires_prefix() {
        assert!(QuestionId::parse("q-data-001").is_ok());
        assert!(matches!(
            QuestionId::parse("data-001"),
            Err(DomainError::InvalidQuestionId(_))
        ));
    }

    #[test]
    fn basic_question_requires_positive_choice() {
        let mut question = valid_question();
        question
            .choices
            .iter_mut()
            .for_each(|choice| choice.score = 0.0);
        assert!(matches!(
            question.validate_basic(),
            Err(DomainError::NoPositiveChoice(_))
        ));
    }

    #[test]
    fn approved_question_requires_review_metadata() {
        let mut question = valid_question();
        question.status = PublicationStatus::Approved;
        question.sources.push(SourceRef {
            id: "src".into(),
            label: "Source".into(),
            url: None,
            kind: SourceKind::Other,
        });
        assert!(matches!(
            question.validate_for_publication(),
            Err(DomainError::ApprovedQuestionMissingReviewer(_))
        ));
    }
}
