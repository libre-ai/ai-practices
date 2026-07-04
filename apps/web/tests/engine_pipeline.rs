//! Proof of the ADR 0003 data flow: a real content question, loaded and
//! validated from `content/questions/*.yml`, is scored by the session engine
//! into an `EvaluationLevel`, which maps 1:1 to the UI's `VerdictKind`.
//!
//! This is the seam the web app's corpus consumes (`verdict_from_level`, reused
//! here). Keeping it as a gated test proves the pipeline end-to-end.

use rumble_ai_practices_content::load_questions;
use rumble_ai_practices_domain::EvaluationLevel;
use rumble_ai_practices_session::{start_session, submit_answer};
use rumble_ai_practices_ui::VerdictKind;
use rumble_ai_practices_web::verdict_from_level;

fn media_corpus_path() -> String {
    // repo-relative, resolved from the crate manifest (portable, no machine path)
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../content/questions/media.yml"
    )
    .to_string()
}

#[test]
fn real_content_flows_through_the_engine_into_a_verdict() {
    let questions = load_questions(media_corpus_path()).expect("media corpus loads and validates");
    let question = questions
        .iter()
        .find(|q| q.id.as_str() == "q-media-002")
        .expect("q-media-002 present")
        .clone();
    let qid = question.id.clone();

    // the recommended choice (score 1.0, no negative) → Correct → Juste
    let mut good = start_session("t-good", vec![question.clone()]).unwrap();
    let eval = submit_answer(
        &mut good,
        &qid,
        vec!["disclose_synthetic_provenance".into()],
    )
    .unwrap();
    assert_eq!(eval.level, EvaluationLevel::Correct);
    assert_eq!(verdict_from_level(eval.level), VerdictKind::Juste);
    // the feedback rendered by the UI comes from the content, via the engine
    assert!(
        eval.feedback_cards
            .iter()
            .any(|c| c.choice_id == "disclose_synthetic_provenance")
    );

    // a harmful choice (score -1.0) → Risky → Risqué
    let mut bad = start_session("t-bad", vec![question]).unwrap();
    let eval = submit_answer(&mut bad, &qid, vec!["publish_without_label".into()]).unwrap();
    assert_eq!(eval.level, EvaluationLevel::Risky);
    assert_eq!(verdict_from_level(eval.level), VerdictKind::Risque);
}

#[test]
fn every_verdict_level_has_a_ui_mapping() {
    // exhaustive: the four engine levels map onto the four-verdict spectrum
    for (level, verdict) in [
        (EvaluationLevel::Correct, VerdictKind::Juste),
        (EvaluationLevel::Partial, VerdictKind::Partiel),
        (EvaluationLevel::Risky, VerdictKind::Risque),
        (EvaluationLevel::Incorrect, VerdictKind::Faux),
    ] {
        assert_eq!(verdict_from_level(level), verdict);
    }
}
