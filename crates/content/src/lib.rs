//! Versioned content loading and fail-closed corpus validation.

use rumble_ai_practices_domain::{
    Activity, BiasReviewDecision, Confidence, DomainError, MediaReview, PublicationStatus, Question,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContentError {
    #[error("I/O error on `{path}`: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("unsupported content extension for `{0}`")]
    UnsupportedExtension(PathBuf),
    #[error("YAML parse error on `{path}`: {source}")]
    Yaml {
        path: PathBuf,
        source: yaml_serde::Error,
    },
    #[error("JSON parse error on `{path}`: {source}")]
    Json {
        path: PathBuf,
        source: serde_json::Error,
    },
    #[error("domain validation error: {0}")]
    Domain(#[from] DomainError),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationSeverity {
    Info,
    Warn,
    Fail,
    Blocker,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFinding {
    pub severity: ValidationSeverity,
    pub question_id: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorpusReport {
    pub files_read: usize,
    pub questions_read: usize,
    pub approved_questions: usize,
    pub media_reviews_read: usize,
    pub findings: Vec<ValidationFinding>,
}

impl CorpusReport {
    pub fn is_success(&self) -> bool {
        !self.findings.iter().any(|finding| {
            matches!(
                finding.severity,
                ValidationSeverity::Fail | ValidationSeverity::Blocker
            )
        })
    }
}

#[derive(Debug, Clone)]
pub struct LoadedCorpus {
    pub questions: Vec<Question>,
    pub report: CorpusReport,
}

#[derive(Debug, Clone)]
pub struct LoadedContent {
    pub questions: Vec<Question>,
    pub media_reviews: Vec<MediaReview>,
    pub report: CorpusReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityValidationFinding {
    pub severity: ValidationSeverity,
    pub activity_id: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityReport {
    pub files_read: usize,
    pub activities_read: usize,
    pub approved_activities: usize,
    pub findings: Vec<ActivityValidationFinding>,
}

impl ActivityReport {
    pub fn is_success(&self) -> bool {
        !self.findings.iter().any(|finding| {
            matches!(
                finding.severity,
                ValidationSeverity::Fail | ValidationSeverity::Blocker
            )
        })
    }
}

#[derive(Debug, Clone)]
pub struct LoadedActivities {
    pub activities: Vec<Activity>,
    pub report: ActivityReport,
}

pub fn load_questions(path: impl AsRef<Path>) -> Result<Vec<Question>, ContentError> {
    let mut questions = Vec::new();
    for file in collect_content_files(path.as_ref())? {
        questions.extend(load_question_file(&file)?);
    }
    Ok(questions)
}

/// Parse a question list from a raw YAML string — no filesystem, so it is usable
/// from a wasm client that embeds the corpus via `include_str!`. Shares the same
/// schema owner (`QuestionList`) as the path-based loader.
pub fn parse_questions_yaml(raw: &str) -> Result<Vec<Question>, ContentError> {
    yaml_serde::from_str::<QuestionList>(raw)
        .map(QuestionList::into_vec)
        .map_err(|source| ContentError::Yaml {
            path: PathBuf::from("<embedded>"),
            source,
        })
}

pub fn load_media_reviews(path: impl AsRef<Path>) -> Result<Vec<MediaReview>, ContentError> {
    let path = path.as_ref();
    if !path.exists() {
        return Ok(Vec::new());
    }

    let mut reviews = Vec::new();
    for file in collect_content_files(path)? {
        reviews.extend(load_media_file(&file)?);
    }
    Ok(reviews)
}

pub fn validate_content(
    question_path: impl AsRef<Path>,
    media_path: impl AsRef<Path>,
) -> Result<LoadedContent, ContentError> {
    let mut loaded = validate_corpus(question_path)?;
    let media_reviews = load_media_reviews(media_path)?;
    loaded.report.media_reviews_read = media_reviews.len();
    validate_media_links(
        &loaded.questions,
        &media_reviews,
        &mut loaded.report.findings,
    );

    Ok(LoadedContent {
        questions: loaded.questions,
        media_reviews,
        report: loaded.report,
    })
}

pub fn validate_activities(path: impl AsRef<Path>) -> Result<LoadedActivities, ContentError> {
    let files = collect_content_files(path.as_ref())?;
    let mut activities = Vec::new();
    let mut findings = Vec::new();
    let mut seen_ids = BTreeSet::new();

    for file in &files {
        for activity in load_activity_file(file)? {
            if !seen_ids.insert(activity.id.to_string()) {
                findings.push(ActivityValidationFinding {
                    severity: ValidationSeverity::Blocker,
                    activity_id: Some(activity.id.to_string()),
                    message: "duplicate activity id".into(),
                });
            }
            if let Err(error) = activity.validate_for_publication() {
                findings.push(ActivityValidationFinding {
                    severity: match activity.status {
                        PublicationStatus::Approved => ValidationSeverity::Blocker,
                        _ => ValidationSeverity::Fail,
                    },
                    activity_id: Some(activity.id.to_string()),
                    message: error.to_string(),
                });
            }
            activities.push(activity);
        }
    }

    let report = ActivityReport {
        files_read: files.len(),
        activities_read: activities.len(),
        approved_activities: activities
            .iter()
            .filter(|activity| activity.status == PublicationStatus::Approved)
            .count(),
        findings,
    };
    Ok(LoadedActivities { activities, report })
}

pub fn validate_corpus(path: impl AsRef<Path>) -> Result<LoadedCorpus, ContentError> {
    let files = collect_content_files(path.as_ref())?;
    let mut questions = Vec::new();
    let mut findings = Vec::new();
    let mut seen_ids = BTreeSet::new();

    for file in &files {
        let file_questions = load_question_file(file)?;
        for question in file_questions {
            if !seen_ids.insert(question.id.to_string()) {
                findings.push(ValidationFinding {
                    severity: ValidationSeverity::Blocker,
                    question_id: Some(question.id.to_string()),
                    message: "duplicate question id".into(),
                });
            }

            if let Err(err) = question.validate_for_publication() {
                findings.push(ValidationFinding {
                    severity: match question.status {
                        PublicationStatus::Approved => ValidationSeverity::Blocker,
                        _ => ValidationSeverity::Fail,
                    },
                    question_id: Some(question.id.to_string()),
                    message: err.to_string(),
                });
            }

            if question.status == PublicationStatus::Approved
                && matches!(question.review.confidence, Confidence::Low)
            {
                findings.push(ValidationFinding {
                    severity: ValidationSeverity::Warn,
                    question_id: Some(question.id.to_string()),
                    message: "approved question has low confidence".into(),
                });
            }

            questions.push(question);
        }
    }

    let report = CorpusReport {
        files_read: files.len(),
        approved_questions: questions
            .iter()
            .filter(|question| question.status == PublicationStatus::Approved)
            .count(),
        questions_read: questions.len(),
        media_reviews_read: 0,
        findings,
    };

    Ok(LoadedCorpus { questions, report })
}

fn validate_media_links(
    questions: &[Question],
    media_reviews: &[MediaReview],
    findings: &mut Vec<ValidationFinding>,
) {
    let mut seen_media_ids = BTreeSet::new();
    for review in media_reviews {
        if !seen_media_ids.insert(review.id.clone()) {
            findings.push(ValidationFinding {
                severity: ValidationSeverity::Blocker,
                question_id: None,
                message: format!("duplicate media review id `{}`", review.id),
            });
        }
    }

    for question in questions {
        for media_id in &question.media {
            let Some(review) = media_reviews.iter().find(|review| review.id == *media_id) else {
                findings.push(ValidationFinding {
                    severity: match question.status {
                        PublicationStatus::Approved => ValidationSeverity::Blocker,
                        _ => ValidationSeverity::Fail,
                    },
                    question_id: Some(question.id.to_string()),
                    message: format!(
                        "media `{media_id}` is referenced without a media-review record"
                    ),
                });
                continue;
            };

            if question.status == PublicationStatus::Approved {
                if review.status != PublicationStatus::Approved {
                    findings.push(ValidationFinding {
                        severity: ValidationSeverity::Blocker,
                        question_id: Some(question.id.to_string()),
                        message: format!("media `{media_id}` is not approved"),
                    });
                }
                if review.bias_review.decision != BiasReviewDecision::Approved {
                    findings.push(ValidationFinding {
                        severity: ValidationSeverity::Blocker,
                        question_id: Some(question.id.to_string()),
                        message: format!("media `{media_id}` bias review is not approved"),
                    });
                }
            }
        }
    }
}

fn collect_content_files(path: &Path) -> Result<Vec<PathBuf>, ContentError> {
    let metadata = fs::metadata(path).map_err(|source| ContentError::Io {
        path: path.to_path_buf(),
        source,
    })?;

    if metadata.is_file() {
        return Ok(vec![path.to_path_buf()]);
    }

    let mut files = Vec::new();
    collect_from_dir(path, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_from_dir(path: &Path, files: &mut Vec<PathBuf>) -> Result<(), ContentError> {
    for entry in fs::read_dir(path).map_err(|source| ContentError::Io {
        path: path.to_path_buf(),
        source,
    })? {
        let entry = entry.map_err(|source| ContentError::Io {
            path: path.to_path_buf(),
            source,
        })?;
        let entry_path = entry.path();
        if entry_path.is_dir() {
            collect_from_dir(&entry_path, files)?;
        } else if is_content_file(&entry_path) {
            files.push(entry_path);
        }
    }
    Ok(())
}

fn is_content_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("yml" | "yaml" | "json")
    )
}

fn load_question_file(path: &Path) -> Result<Vec<Question>, ContentError> {
    load_list_file::<QuestionList>(path).map(QuestionList::into_vec)
}

fn load_activity_file(path: &Path) -> Result<Vec<Activity>, ContentError> {
    load_list_file::<ActivityList>(path).map(ActivityList::into_vec)
}

fn load_media_file(path: &Path) -> Result<Vec<MediaReview>, ContentError> {
    load_list_file::<MediaReviewList>(path).map(MediaReviewList::into_vec)
}

fn load_list_file<T>(path: &Path) -> Result<T, ContentError>
where
    T: for<'de> Deserialize<'de>,
{
    let raw = fs::read_to_string(path).map_err(|source| ContentError::Io {
        path: path.to_path_buf(),
        source,
    })?;

    match path.extension().and_then(|ext| ext.to_str()) {
        Some("yml" | "yaml") => {
            yaml_serde::from_str::<T>(&raw).map_err(|source| ContentError::Yaml {
                path: path.to_path_buf(),
                source,
            })
        }
        Some("json") => serde_json::from_str::<T>(&raw).map_err(|source| ContentError::Json {
            path: path.to_path_buf(),
            source,
        }),
        _ => Err(ContentError::UnsupportedExtension(path.to_path_buf())),
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum QuestionList {
    Many(Vec<Question>),
    One(Box<Question>),
}

impl QuestionList {
    fn into_vec(self) -> Vec<Question> {
        match self {
            Self::Many(questions) => questions,
            Self::One(question) => vec![*question],
        }
    }
}

impl From<QuestionList> for Vec<Question> {
    fn from(value: QuestionList) -> Self {
        value.into_vec()
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ActivityList {
    Many(Vec<Activity>),
    One(Box<Activity>),
}

impl ActivityList {
    fn into_vec(self) -> Vec<Activity> {
        match self {
            Self::Many(activities) => activities,
            Self::One(activity) => vec![*activity],
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum MediaReviewList {
    Many(Vec<MediaReview>),
    One(Box<MediaReview>),
}

impl MediaReviewList {
    fn into_vec(self) -> Vec<MediaReview> {
        match self {
            Self::Many(reviews) => reviews,
            Self::One(review) => vec![*review],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_pilot_corpus() {
        let loaded = validate_corpus("../../content/questions").expect("pilot corpus parses");
        assert!(loaded.report.questions_read >= 5);
        assert!(loaded.report.is_success());
    }

    #[test]
    fn validates_content_with_media_reviews() {
        let loaded = validate_content("../../content/questions", "../../content/media")
            .expect("pilot content parses");
        assert!(loaded.report.questions_read >= 5);
        // bias-game corpus: pilot(1) + bias-visual(50) + deepfakes(27) + profiles(31) = 109
        assert_eq!(loaded.media_reviews.len(), 109);
        assert!(loaded.report.is_success());
    }

    #[test]
    fn validates_website_reconstruction_activities_as_drafts() {
        let loaded =
            validate_activities("../../content/activities").expect("activity corpus parses");
        assert_eq!(loaded.report.activities_read, 3);
        assert_eq!(loaded.report.approved_activities, 0);
        assert!(loaded.report.is_success());
        assert!(
            loaded
                .activities
                .iter()
                .all(|activity| activity.status == PublicationStatus::Draft)
        );
    }
}
