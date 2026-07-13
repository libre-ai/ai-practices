//! Session engine: answer evaluation and private pedagogical summaries.

/// Anonymous k-anonymous cohort aggregation + retention (ADR 0006).
pub mod cohort;

use rumble_ai_practices_domain::{
    Activity, ActivityId, AnswerEvaluation, AxisImpact, AxisLevel, EvaluationLevel, FeedbackCard,
    InteractionType, ModuleRef, PracticeLevel, PublicationStatus, Question, QuestionId, RiskAxis,
    SessionSummary, SourceRef,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SessionError {
    #[error("session requires at least one question")]
    EmptyQuestionSet,
    #[error("unknown question `{0}`")]
    UnknownQuestion(String),
    #[error("question `{0}` has already been answered")]
    AlreadyAnswered(String),
    #[error("question `{question_id}` does not contain choice `{choice_id}`")]
    UnknownChoice {
        question_id: String,
        choice_id: String,
    },
    #[error("question `{0}` requires at least one selected choice")]
    EmptyAnswer(String),
    #[error("question `{question_id}` requires at least {min} selected choice(s), got {actual}")]
    TooFewChoices {
        question_id: String,
        min: usize,
        actual: usize,
    },
    #[error("question `{question_id}` accepts at most {max} selected choice(s), got {actual}")]
    TooManyChoices {
        question_id: String,
        max: usize,
        actual: usize,
    },
    #[error("activity attempt targets `{attempted}` but loaded activity is `{loaded}`")]
    ActivityMismatch { attempted: String, loaded: String },
    #[error("activity `{0}` requires non-empty evidence references before submission")]
    ActivityEvidenceRequired(String),
    #[error("activity `{0}` requires a stop reason when stopped")]
    ActivityStopReasonRequired(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityAttemptStatus {
    EvidenceSubmitted,
    Stopped,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityAttempt {
    pub activity_id: ActivityId,
    pub status: ActivityAttemptStatus,
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    #[serde(default)]
    pub stop_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityOutcome {
    pub format: String,
    pub activity_id: ActivityId,
    pub publication_status: PublicationStatus,
    pub draft_preview: bool,
    pub attempt_status: ActivityAttemptStatus,
    pub observable_outcome: String,
    pub submitted_evidence_refs: Vec<String>,
    pub stop_reason: Option<String>,
    pub success_criteria: Vec<String>,
    pub feedback_principle: String,
    pub remediation: String,
    pub human_review_required: bool,
    pub next_action: String,
}

pub fn run_activity(
    activity: &Activity,
    attempt: ActivityAttempt,
) -> Result<ActivityOutcome, SessionError> {
    if activity.id != attempt.activity_id {
        return Err(SessionError::ActivityMismatch {
            attempted: attempt.activity_id.to_string(),
            loaded: activity.id.to_string(),
        });
    }
    if attempt.status == ActivityAttemptStatus::EvidenceSubmitted
        && (attempt.evidence_refs.is_empty()
            || attempt
                .evidence_refs
                .iter()
                .any(|evidence| evidence.trim().is_empty()))
    {
        return Err(SessionError::ActivityEvidenceRequired(
            activity.id.to_string(),
        ));
    }
    if attempt.status == ActivityAttemptStatus::Stopped
        && attempt
            .stop_reason
            .as_deref()
            .unwrap_or("")
            .trim()
            .is_empty()
    {
        return Err(SessionError::ActivityStopReasonRequired(
            activity.id.to_string(),
        ));
    }
    let next_action = match attempt.status {
        ActivityAttemptStatus::EvidenceSubmitted => {
            "Faire examiner les preuves par une personne ; aucune réussite n’est attribuée automatiquement."
        }
        ActivityAttemptStatus::Stopped => {
            "Appliquer la condition d’arrêt, corriger le contexte puis rejouer l’activité."
        }
    };
    Ok(ActivityOutcome {
        format: "libre-ai.ai-practices.activity-outcome.v1".into(),
        activity_id: activity.id.clone(),
        publication_status: activity.status,
        draft_preview: activity.status != PublicationStatus::Approved,
        attempt_status: attempt.status,
        observable_outcome: activity.objective.observable_outcome.clone(),
        submitted_evidence_refs: attempt.evidence_refs,
        stop_reason: attempt.stop_reason,
        success_criteria: activity.success_criteria.clone(),
        feedback_principle: activity.feedback.principle.clone(),
        remediation: activity.feedback.remediation.clone(),
        human_review_required: true,
        next_action: next_action.into(),
    })
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionState {
    pub session_id: String,
    pub questions: Vec<Question>,
    pub answers: BTreeMap<String, AnswerEvaluation>,
}

pub fn start_session(
    session_id: impl Into<String>,
    questions: Vec<Question>,
) -> Result<SessionState, SessionError> {
    if questions.is_empty() {
        return Err(SessionError::EmptyQuestionSet);
    }
    Ok(SessionState {
        session_id: session_id.into(),
        questions,
        answers: BTreeMap::new(),
    })
}

pub fn submit_answer(
    state: &mut SessionState,
    question_id: &QuestionId,
    choice_ids: Vec<String>,
) -> Result<AnswerEvaluation, SessionError> {
    if choice_ids.is_empty() {
        return Err(SessionError::EmptyAnswer(question_id.to_string()));
    }
    if state.answers.contains_key(question_id.as_str()) {
        return Err(SessionError::AlreadyAnswered(question_id.to_string()));
    }

    let question = state
        .questions
        .iter()
        .find(|candidate| &candidate.id == question_id)
        .ok_or_else(|| SessionError::UnknownQuestion(question_id.to_string()))?;

    let selected: BTreeSet<_> = choice_ids.iter().cloned().collect();
    validate_choice_count(question, selected.len())?;

    let mut score_delta = 0.0;
    let mut feedback_cards = Vec::new();
    let mut has_negative = false;
    let mut has_critical = false;

    for selected_id in &selected {
        let choice = question
            .choices
            .iter()
            .find(|choice| choice.id == *selected_id)
            .ok_or_else(|| SessionError::UnknownChoice {
                question_id: question_id.to_string(),
                choice_id: selected_id.clone(),
            })?;
        score_delta += choice.score;
        has_negative |= choice.score < 0.0;
        has_critical |= choice.score <= -2.0;
        feedback_cards.push(FeedbackCard {
            choice_id: choice.id.clone(),
            message: choice.feedback.clone(),
            severity: choice.severity,
        });
    }

    let max_positive: f64 = question
        .choices
        .iter()
        .filter(|choice| choice.score > 0.0)
        .map(|choice| choice.score)
        .sum();

    let level = if has_critical {
        EvaluationLevel::Risky
    } else if score_delta >= max_positive && !has_negative {
        EvaluationLevel::Correct
    } else if score_delta > 0.0 {
        EvaluationLevel::Partial
    } else if has_negative {
        EvaluationLevel::Risky
    } else {
        EvaluationLevel::Incorrect
    };

    let evaluation = AnswerEvaluation {
        question_id: question.id.clone(),
        selected_choice_ids: selected.into_iter().collect(),
        score_delta,
        level,
        axis_impacts: vec![AxisImpact {
            axis: question.axis,
            score_delta,
        }],
        feedback_cards,
        evidence_refs: question.sources.clone(),
    };

    state
        .answers
        .insert(question_id.to_string(), evaluation.clone());
    Ok(evaluation)
}

fn validate_choice_count(question: &Question, actual: usize) -> Result<(), SessionError> {
    let default_min = usize::from(matches!(
        question.interaction.kind,
        InteractionType::SingleChoice
            | InteractionType::MultipleChoice
            | InteractionType::MediaReview
    ));
    let min = question
        .interaction
        .min_choices
        .unwrap_or(default_min as u32) as usize;
    // A media review is a single verdict on the media (biased/authentic/…),
    // so it is single-select by default like SingleChoice. Authors may still
    // widen it with an explicit max_choices.
    let default_max = match question.interaction.kind {
        InteractionType::SingleChoice | InteractionType::MediaReview => 1,
        _ => question.choices.len(),
    };
    let max = question
        .interaction
        .max_choices
        .unwrap_or(default_max as u32) as usize;

    if actual < min {
        return Err(SessionError::TooFewChoices {
            question_id: question.id.to_string(),
            min,
            actual,
        });
    }
    if actual > max {
        return Err(SessionError::TooManyChoices {
            question_id: question.id.to_string(),
            max,
            actual,
        });
    }
    Ok(())
}

pub fn complete_session(state: &SessionState) -> SessionSummary {
    let mut per_axis: BTreeMap<RiskAxis, (f64, usize)> = BTreeMap::new();
    for evaluation in state.answers.values() {
        for impact in &evaluation.axis_impacts {
            let entry = per_axis.entry(impact.axis).or_insert((0.0, 0));
            entry.0 += impact.score_delta;
            entry.1 += 1;
        }
    }

    let axis_levels = per_axis
        .into_iter()
        .map(|(axis, (score, count))| {
            let average = if count == 0 {
                0.0
            } else {
                score / count as f64
            };
            AxisLevel {
                axis,
                level: practice_level(average),
                score: average,
            }
        })
        .collect::<Vec<_>>();

    let strengths = axis_levels
        .iter()
        .filter(|axis| {
            matches!(
                axis.level,
                PracticeLevel::CarefulAutonomy | PracticeLevel::Reference
            )
        })
        .map(|axis| format!("Axe {:?} solide", axis.axis))
        .collect();

    let recommended_practices = axis_levels
        .iter()
        .filter(|axis| {
            matches!(
                axis.level,
                PracticeLevel::Discovery | PracticeLevel::GuidedPractice
            )
        })
        .map(|axis| format!("Renforcer l'axe {:?}", axis.axis))
        .collect();

    SessionSummary {
        session_id: state.session_id.clone(),
        completed_at: "local-fixture".into(),
        answered_count: state.answers.len(),
        axis_levels,
        // populated by the API layer with k-anonymous cohort access (ADR 0006)
        private_distributions: Vec::new(),
        strengths,
        recommended_practices,
        next_modules: vec![ModuleRef {
            id: "review-feedback".into(),
            label: "Relire les fiches réflexes associées".into(),
        }],
        privacy_notice:
            "Synthèse privée : aucun classement nominatif n'est produit par le moteur de session."
                .into(),
    }
}

fn practice_level(score: f64) -> PracticeLevel {
    if score >= 0.9 {
        PracticeLevel::Reference
    } else if score >= 0.5 {
        PracticeLevel::CarefulAutonomy
    } else if score > 0.0 {
        PracticeLevel::GuidedPractice
    } else {
        PracticeLevel::Discovery
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionFixture {
    pub session_id: String,
    pub answers: Vec<FixtureAnswer>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FixtureAnswer {
    pub question_id: QuestionId,
    pub choice_ids: Vec<String>,
}

pub fn run_fixture(
    questions: Vec<Question>,
    fixture: SessionFixture,
) -> Result<SessionSummary, SessionError> {
    let mut state = start_session(fixture.session_id, questions)?;
    for answer in fixture.answers {
        submit_answer(&mut state, &answer.question_id, answer.choice_ids)?;
    }
    Ok(complete_session(&state))
}

pub fn source_refs_for_evaluation(evaluation: &AnswerEvaluation) -> &[SourceRef] {
    &evaluation.evidence_refs
}

#[cfg(test)]
mod tests {
    use super::*;
    use rumble_ai_practices_domain::{
        ActivityClaim, ActivityFeedback, ActivityObjective, ActivityType, AiAssistance, Choice,
        Confidence, Difficulty, Interaction, InteractionType, PublicationStatus, ReviewMetadata,
        ScenarioContext,
    };

    fn question() -> Question {
        Question {
            id: QuestionId::parse("q-session-001").unwrap(),
            version: 1,
            status: PublicationStatus::Draft,
            locale: "fr-FR".into(),
            title: "Session".into(),
            axis: RiskAxis::Security,
            difficulty: Difficulty::Beginner,
            intent: "Tester le moteur de session.".into(),
            context: ScenarioContext {
                role: None,
                scenario: "Situation".into(),
                assets: vec![],
            },
            prompt: "Choix ?".into(),
            interaction: Interaction {
                kind: InteractionType::SingleChoice,
                min_choices: Some(1),
                max_choices: Some(1),
            },
            choices: vec![
                Choice {
                    id: "good".into(),
                    label: "Bonne".into(),
                    score: 1.0,
                    severity: None,
                    feedback: "Bonne pratique".into(),
                },
                Choice {
                    id: "bad".into(),
                    label: "Mauvaise".into(),
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

    #[test]
    fn evaluates_correct_answer() {
        let mut state = start_session("s1", vec![question()]).unwrap();
        let evaluation = submit_answer(
            &mut state,
            &QuestionId::parse("q-session-001").unwrap(),
            vec!["good".into()],
        )
        .unwrap();
        assert_eq!(evaluation.level, EvaluationLevel::Correct);
    }

    fn media_review_question() -> Question {
        let mut q = question();
        q.id = QuestionId::parse("q-media-001").unwrap();
        q.interaction = Interaction {
            kind: InteractionType::MediaReview,
            min_choices: None,
            max_choices: None,
        };
        q.media = vec!["asset.webp".into()];
        q.choices = vec![
            Choice {
                id: "biased".into(),
                label: "Biaisé".into(),
                score: 1.0,
                severity: None,
                feedback: "Vu.".into(),
            },
            Choice {
                id: "neutral".into(),
                label: "Neutre".into(),
                score: -1.0,
                severity: None,
                feedback: "Le piège.".into(),
            },
            Choice {
                id: "unsure".into(),
                label: "Incertain".into(),
                score: 0.0,
                severity: None,
                feedback: "Nuance.".into(),
            },
        ];
        q
    }

    #[test]
    fn media_review_defaults_to_single_verdict() {
        // A media review is one verdict on the media: with no explicit
        // max_choices, selecting several choices must be rejected.
        let mut state = start_session("s1", vec![media_review_question()]).unwrap();
        let result = submit_answer(
            &mut state,
            &QuestionId::parse("q-media-001").unwrap(),
            vec!["biased".into(), "neutral".into()],
        );
        assert!(matches!(
            result,
            Err(SessionError::TooManyChoices { max: 1, .. })
        ));
    }

    #[test]
    fn media_review_single_verdict_scores_correct() {
        let mut state = start_session("s1", vec![media_review_question()]).unwrap();
        let eval = submit_answer(
            &mut state,
            &QuestionId::parse("q-media-001").unwrap(),
            vec!["biased".into()],
        )
        .unwrap();
        assert_eq!(eval.level, EvaluationLevel::Correct);
    }

    #[test]
    fn refuses_duplicate_answer() {
        let mut state = start_session("s1", vec![question()]).unwrap();
        let id = QuestionId::parse("q-session-001").unwrap();
        submit_answer(&mut state, &id, vec!["good".into()]).unwrap();
        assert!(matches!(
            submit_answer(&mut state, &id, vec!["bad".into()]),
            Err(SessionError::AlreadyAnswered(_))
        ));
    }

    #[test]
    fn enforces_single_choice_max() {
        let mut state = start_session("s1", vec![question()]).unwrap();
        let id = QuestionId::parse("q-session-001").unwrap();
        assert!(matches!(
            submit_answer(&mut state, &id, vec!["good".into(), "bad".into()]),
            Err(SessionError::TooManyChoices {
                max: 1,
                actual: 2,
                ..
            })
        ));
    }

    fn activity() -> Activity {
        Activity {
            id: ActivityId::parse("activity-session-test").unwrap(),
            version: 1,
            status: PublicationStatus::Draft,
            locale: "fr-FR".into(),
            title: "Vérifier une sortie".into(),
            kind: ActivityType::Scenario,
            objective: ActivityObjective {
                observable_outcome: "Relier une affirmation à une preuve.".into(),
                competency_ids: vec!["comp-check-source".into()],
            },
            prerequisites: vec![],
            situation: "Une sortie plausible doit être vérifiée.".into(),
            instructions: vec!["Consulter la source.".into()],
            permitted_data: vec![],
            environment: None,
            success_criteria: vec!["Une preuve est jointe.".into()],
            feedback: ActivityFeedback {
                principle: "Une sortie n'est pas une preuve.".into(),
                remediation: "Revenir à la source primaire.".into(),
            },
            risks: vec!["Erreur plausible.".into()],
            limits: vec!["Cas synthétique.".into()],
            stop_conditions: vec!["Arrêter si la source manque.".into()],
            source_refs: vec!["source-nist-ai-rmf-1-0".into()],
            claims: vec![ActivityClaim {
                claim_id: "claim-ai-output-not-proof".into(),
                statement: "Une sortie doit être vérifiée.".into(),
                source_ids: vec!["source-nist-ai-rmf-1-0".into()],
            }],
            media: vec![],
            ai_assistance: AiAssistance {
                used: false,
                details: None,
            },
            review: ReviewMetadata {
                author: "test".into(),
                reviewers: vec![],
                last_reviewed_at: None,
                confidence: Confidence::Low,
                notes: None,
            },
            retire_when: "Le contrat change.".into(),
        }
    }

    #[test]
    fn activity_submission_never_auto_awards_success() {
        let activity = activity();
        let outcome = run_activity(
            &activity,
            ActivityAttempt {
                activity_id: activity.id.clone(),
                status: ActivityAttemptStatus::EvidenceSubmitted,
                evidence_refs: vec!["evidence:local-check".into()],
                stop_reason: None,
            },
        )
        .unwrap();
        assert!(outcome.draft_preview);
        assert!(outcome.human_review_required);
        assert!(outcome.next_action.contains("personne"));
    }

    #[test]
    fn activity_submission_without_evidence_is_refused() {
        let activity = activity();
        assert!(matches!(
            run_activity(
                &activity,
                ActivityAttempt {
                    activity_id: activity.id.clone(),
                    status: ActivityAttemptStatus::EvidenceSubmitted,
                    evidence_refs: vec![],
                    stop_reason: None,
                },
            ),
            Err(SessionError::ActivityEvidenceRequired(_))
        ));
    }

    #[test]
    fn stopped_activity_requires_and_preserves_a_reason() {
        let activity = activity();
        assert!(matches!(
            run_activity(
                &activity,
                ActivityAttempt {
                    activity_id: activity.id.clone(),
                    status: ActivityAttemptStatus::Stopped,
                    evidence_refs: vec![],
                    stop_reason: None,
                },
            ),
            Err(SessionError::ActivityStopReasonRequired(_))
        ));
        let outcome = run_activity(
            &activity,
            ActivityAttempt {
                activity_id: activity.id.clone(),
                status: ActivityAttemptStatus::Stopped,
                evidence_refs: vec![],
                stop_reason: Some("La source autorisée est indisponible.".into()),
            },
        )
        .unwrap();
        assert_eq!(
            outcome.stop_reason.as_deref(),
            Some("La source autorisée est indisponible.")
        );
        assert!(outcome.human_review_required);
    }
}
