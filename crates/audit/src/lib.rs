//! Deterministic editorial and safety audit checks for the corpus.

use rumble_ai_practices_domain::{PublicationStatus, Question};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditSeverity {
    Info,
    Warn,
    Fail,
    Blocker,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditCategory {
    MissingSource,
    MisleadingFeedback,
    Oversimplification,
    PromptDogma,
    PrivacyRisk,
    SecurityRisk,
    MediaBias,
    ReviewGap,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditFinding {
    pub severity: AuditSeverity,
    pub category: AuditCategory,
    pub question_id: String,
    pub message: String,
    pub recommendation: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditReport {
    pub questions_audited: usize,
    pub findings: Vec<AuditFinding>,
}

impl AuditReport {
    pub fn has_blockers(&self) -> bool {
        self.findings
            .iter()
            .any(|finding| finding.severity == AuditSeverity::Blocker)
    }
}

pub fn audit_corpus(questions: &[Question]) -> AuditReport {
    let mut findings = Vec::new();

    for question in questions {
        audit_question(question, &mut findings);
    }

    AuditReport {
        questions_audited: questions.len(),
        findings,
    }
}

fn audit_question(question: &Question, findings: &mut Vec<AuditFinding>) {
    if question.sources.is_empty() {
        findings.push(AuditFinding {
            severity: match question.status {
                PublicationStatus::Approved => AuditSeverity::Blocker,
                _ => AuditSeverity::Warn,
            },
            category: AuditCategory::MissingSource,
            question_id: question.id.to_string(),
            message: "question has no source".into(),
            recommendation:
                "add a public source, standard, law, or explicit internal policy reference".into(),
        });
    }

    if question.status == PublicationStatus::Approved && question.review.reviewers.is_empty() {
        findings.push(AuditFinding {
            severity: AuditSeverity::Blocker,
            category: AuditCategory::ReviewGap,
            question_id: question.id.to_string(),
            message: "approved question has no reviewer".into(),
            recommendation: "move back to review until human review metadata is complete".into(),
        });
    }

    if !question.media.is_empty() {
        findings.push(AuditFinding {
            severity: match question.status {
                PublicationStatus::Approved => AuditSeverity::Blocker,
                _ => AuditSeverity::Fail,
            },
            category: AuditCategory::MediaBias,
            question_id: question.id.to_string(),
            message: "question references media; media review linkage is not implemented yet"
                .into(),
            recommendation: "keep as draft/review until media-review records are validated".into(),
        });
    }

    let searchable = question_text(question).to_lowercase();
    for rule in forbidden_feedback_rules() {
        if searchable.contains(rule.needle) {
            findings.push(AuditFinding {
                severity: rule.severity,
                category: rule.category.clone(),
                question_id: question.id.to_string(),
                message: rule.message.into(),
                recommendation: rule.recommendation.into(),
            });
        }
    }
}

fn question_text(question: &Question) -> String {
    let mut text = format!(
        "{} {} {} {}",
        question.title, question.intent, question.context.scenario, question.prompt
    );
    for choice in &question.choices {
        text.push(' ');
        text.push_str(&choice.label);
        text.push(' ');
        text.push_str(&choice.feedback);
    }
    text
}

struct ForbiddenRule {
    needle: &'static str,
    severity: AuditSeverity,
    category: AuditCategory,
    message: &'static str,
    recommendation: &'static str,
}

fn forbidden_feedback_rules() -> Vec<ForbiddenRule> {
    vec![
        ForbiddenRule {
            needle: "la vérification est déjà incluse",
            severity: AuditSeverity::Blocker,
            category: AuditCategory::MisleadingFeedback,
            message: "feedback suggests that asking for sources is equivalent to verifying them",
            recommendation: "rewrite: sources must be opened and checked for existence, content, date, and authority",
        },
        ForbiddenRule {
            needle: "aucune donnée",
            severity: AuditSeverity::Warn,
            category: AuditCategory::Oversimplification,
            message: "absolute claim about absence of data may be unsafe without context",
            recommendation: "prefer content/classification inspection language",
        },
        ForbiddenRule {
            needle: "toujours donner une identité",
            severity: AuditSeverity::Warn,
            category: AuditCategory::PromptDogma,
            message: "prompting method appears as an absolute rule",
            recommendation: "frame prompting techniques as aids, not guarantees",
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use rumble_ai_practices_domain::{
        Choice, Confidence, Difficulty, Interaction, InteractionType, QuestionId, ReviewMetadata,
        RiskAxis, ScenarioContext,
    };

    #[test]
    fn flags_misleading_source_feedback() {
        let question = Question {
            id: QuestionId::parse("q-audit-001").unwrap(),
            version: 1,
            status: PublicationStatus::Draft,
            locale: "fr-FR".into(),
            title: "Sources".into(),
            axis: RiskAxis::SourceVerification,
            difficulty: Difficulty::Beginner,
            intent: "Tester une correction dangereuse.".into(),
            context: ScenarioContext {
                role: None,
                scenario: "Une question demande des sources.".into(),
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
                    id: "bad".into(),
                    label: "Envoyer".into(),
                    score: 1.0,
                    severity: None,
                    feedback: "La vérification est déjà incluse.".into(),
                },
                Choice {
                    id: "other".into(),
                    label: "Autre".into(),
                    score: -1.0,
                    severity: None,
                    feedback: "Autre réponse.".into(),
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
        };

        let report = audit_corpus(&[question]);
        assert!(report.findings.iter().any(|finding| {
            finding.category == AuditCategory::MisleadingFeedback
                && finding.severity == AuditSeverity::Blocker
        }));
    }
}
