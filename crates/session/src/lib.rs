//! Session engine: answer evaluation and private pedagogical summaries.

/// Anonymous k-anonymous cohort aggregation + retention (ADR 0006).
pub mod cohort;

use rumble_ai_practices_domain::{
    AnswerEvaluation, AxisImpact, AxisLevel, EvaluationLevel, FeedbackCard, InteractionType,
    ModuleRef, PracticeLevel, Question, QuestionId, RiskAxis, SessionSummary, SourceRef,
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
    let default_max = match question.interaction.kind {
        InteractionType::SingleChoice => 1,
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
        Choice, Confidence, Difficulty, Interaction, InteractionType, PublicationStatus,
        ReviewMetadata, ScenarioContext,
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
}
