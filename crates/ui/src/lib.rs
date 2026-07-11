//! Dioxus UI contracts for `rumble-ai-practices`, styled by Libre IA Design System 2.0.
//!
//! Components here stay presentational. Scoring, validation and session
//! decisions belong to the Rust core crates, not to UI: a verdict arrives as
//! data (`VerdictKind` on a `FeedbackViewModel`), it is never computed here.
//!
//! Appearance is tokens-only: components carry semantic classes and a
//! `data-verdict` slug; status meaning also has a label and glyph, never color alone.

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

    /// A shape carried alongside the color, so the verdict never depends on
    /// color alone (WCAG 1.4.1).
    pub fn symbol(self) -> &'static str {
        match self {
            VerdictKind::Juste => "✓",
            VerdictKind::Partiel => "≈",
            VerdictKind::Risque => "⚠",
            VerdictKind::Faux => "✗",
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

/// A per-category line motif, drawn as an inline SVG. Used as a faint header
/// watermark on the question and as a colored vignette in the synthesis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MotifKind {
    /// Shield — confidentiality.
    Shield,
    /// Chain link — sources / reliability.
    Link,
    /// Portraits — media bias.
    Portrait,
    /// Envelope — prompt safety / data minimization.
    Envelope,
}

/// A single "source" link the AI produced, with a verification note.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkChip {
    pub url: String,
    pub note: String,
}

/// A field of a client ticket. `value` is treated as PII and rendered redacted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicketRow {
    pub label: String,
    pub value: String,
}

/// The concrete object a situation is about, rendered as a visible artifact
/// rather than described in prose.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScenarioArtifact {
    /// A shared file (attachment card). `flag` carries the sensitivity hint.
    File {
        name: String,
        meta: String,
        flag: Option<String>,
    },
    /// The AI's "sources" (link chips, each with a verification note).
    Links(Vec<LinkChip>),
    /// A generated image (stylized placeholder + tag). `alt` describes it.
    Image { ai_tag: String, alt: String },
    /// An actual generated media file to display as a real `<img>` (the drill's
    /// visual). `alt` describes it; the AI-synthetic disclosure is shown by the
    /// view (ADR 0004/0008).
    Generated { src: String, alt: String },
    /// A client ticket whose values are personal data (rendered redacted).
    Ticket(Vec<TicketRow>),
}

/// How the situation is staged: as a chat exchange (a message with the
/// artifact attached) or as a posed editorial scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScenarioFraming {
    Thread { sender: String, message: String },
    Posed { scenario: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestionViewModel {
    pub id: String,
    pub index: usize,
    pub total: usize,
    pub role: String,
    pub category: String,
    pub motif: MotifKind,
    pub framing: ScenarioFraming,
    pub artifact: Option<ScenarioArtifact>,
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

/// One category's outcome in the private synthesis: the verdict earned and the
/// single reflex to keep. `verdict = None` means the learner answered "je ne
/// sais pas".
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryOutcome {
    pub category: String,
    pub motif: MotifKind,
    pub verdict: Option<VerdictKind>,
    pub takeaway: String,
}

impl CategoryOutcome {
    /// Verdict slug for `[data-verdict]`, or `"idk"` for a skipped answer.
    pub fn slug(&self) -> &'static str {
        self.verdict.map(|v| v.slug()).unwrap_or("idk")
    }

    /// Verdict glyph, or a question mark for a skipped answer (WCAG 1.4.1).
    pub fn glyph(&self) -> &'static str {
        self.verdict.map(|v| v.symbol()).unwrap_or("?")
    }

    /// Verdict label, or "À explorer" for a skipped answer.
    pub fn label(&self) -> &'static str {
        self.verdict.map(|v| v.label()).unwrap_or("À explorer")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SummaryViewModel {
    pub answered_count: usize,
    pub outcomes: Vec<CategoryOutcome>,
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

/// A per-category line motif (inline SVG, `currentColor`). Faint header
/// watermark on the question, colored vignette in the synthesis.
#[component]
pub fn CategoryMotif(kind: MotifKind, #[props(default)] class: String) -> Element {
    rsx! {
        svg {
            class: "motif-svg {class}",
            "viewBox": "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            "stroke-width": "1.5",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            "aria-hidden": "true",
            match kind {
                MotifKind::Shield => rsx! {
                    path { "d": "M12 3l7 3v5c0 4.5-3 8-7 10-4-2-7-5.5-7-10V6z" }
                    path { "d": "M9 12l2 2 4-4" }
                },
                MotifKind::Link => rsx! {
                    path { "d": "M10 13a4 4 0 0 0 6 .5l2.5-2.5a4 4 0 0 0-5.7-5.7L11 7" }
                    path { "d": "M14 11a4 4 0 0 0-6-.5L5.5 13a4 4 0 0 0 5.7 5.7L13 17" }
                },
                MotifKind::Portrait => rsx! {
                    rect { "x": "3", "y": "4", "width": "18", "height": "16", "rx": "2" }
                    circle { "cx": "8.5", "cy": "10", "r": "1.6" }
                    circle { "cx": "15.5", "cy": "10", "r": "1.6" }
                    path { "d": "M4 18l5-5 3 3 3-3 5 5" }
                },
                MotifKind::Envelope => rsx! {
                    rect { "x": "3", "y": "5", "width": "18", "height": "14", "rx": "2" }
                    path { "d": "M3 7l9 6 9-6" }
                },
            }
        }
    }
}

/// Renders the concrete artifact of a situation (file, links, image, ticket)
/// so the object is seen, not just described.
#[component]
pub fn ScenarioArtifactView(artifact: ScenarioArtifact) -> Element {
    match artifact {
        ScenarioArtifact::File { name, meta, flag } => rsx! {
            div { class: "attach",
                div { class: "attach-thumb", "aria-hidden": "true",
                    svg { "viewBox": "0 0 24 24", fill: "none", stroke: "currentColor",
                        "stroke-width": "1.6", "stroke-linecap": "round", "stroke-linejoin": "round",
                        path { "d": "M14 3H7a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V8z" }
                        path { "d": "M14 3v5h5" }
                        path { "d": "M8.5 13h7" }
                        path { "d": "M8.5 16h7" }
                    }
                }
                div { class: "attach-body",
                    div { class: "attach-name", "{name}" }
                    div { class: "attach-meta", "{meta}" }
                    if let Some(f) = flag {
                        span { class: "attach-flag", "{f}" }
                    }
                }
            }
        },
        ScenarioArtifact::Links(items) => rsx! {
            div { class: "links",
                for item in items {
                    div { class: "chip",
                        span { class: "chip-dot", "aria-hidden": "true" }
                        span { class: "chip-url", "{item.url}" }
                        span { class: "chip-note", "{item.note}" }
                    }
                }
            }
        },
        ScenarioArtifact::Image { ai_tag, alt } => rsx! {
            figure { class: "imgframe", role: "img", "aria-label": "{alt}",
                span { class: "ai-tag", "{ai_tag}" }
                svg { "viewBox": "0 0 160 80", fill: "none", "aria-hidden": "true",
                    g { stroke: "currentColor", "stroke-width": "1.4", opacity: "0.9",
                        circle { "cx": "34", "cy": "30", "r": "9" }
                        path { "d": "M20 58c0-9 6-14 14-14s14 5 14 14" }
                        circle { "cx": "80", "cy": "30", "r": "9" }
                        path { "d": "M66 58c0-9 6-14 14-14s14 5 14 14" }
                        circle { "cx": "126", "cy": "30", "r": "9" }
                        path { "d": "M112 58c0-9 6-14 14-14s14 5 14 14" }
                    }
                }
            }
        },
        ScenarioArtifact::Ticket(rows) => rsx! {
            div { class: "attach attach-ticket",
                div { class: "attach-thumb", "aria-hidden": "true",
                    svg { "viewBox": "0 0 24 24", fill: "none", stroke: "currentColor",
                        "stroke-width": "1.6", "stroke-linecap": "round", "stroke-linejoin": "round",
                        path { "d": "M4 5h16v5a2 2 0 0 0 0 4v5H4v-5a2 2 0 0 0 0-4z" }
                        path { "d": "M9 5v14" }
                    }
                }
                div { class: "ticket",
                    for row in rows {
                        div { class: "ticket-row",
                            b { "{row.label}" }
                            span { class: "redact", "{row.value}" }
                        }
                    }
                }
            }
        },
        ScenarioArtifact::Generated { src, alt } => rsx! {
            figure { class: "imgframe imgframe-media",
                span { class: "ai-tag", "Image générée par IA" }
                img { class: "gen-img", src: "{src}", alt: "{alt}", loading: "lazy" }
            }
        },
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
                span { class: "glyph", aria_hidden: "true", "{feedback.verdict.symbol()}" }
                span { "{feedback.verdict.label()} — voici pourquoi" }
            }
            p { class: "fb-reason", "{feedback.reason}" }
            // The detail folds under a headless, accessible Collapsible
            // (dioxus-primitives, ADR 0036). Verdict + reason stay the primary
            // message; the reflex breakdown is opt-in (closed by default so it
            // does not overwhelm the calm read).
            Collapsible { class: "fb-collapsible", default_open: false,
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
            p { class: "summary-count",
                "Réflexes passés en revue : " b { "{summary.answered_count}" }
            }
            // The full verdict spectrum, one row per category: glyph + verdict +
            // the single reflex to keep. No ranking, no score — just where each
            // reflex stands and what to remember.
            ul { class: "summary-list",
                for outcome in summary.outcomes {
                    li { class: "summary-row", "data-verdict": "{outcome.slug()}",
                        div { class: "summary-vignette", aria_hidden: "true",
                            CategoryMotif { kind: outcome.motif }
                        }
                        div { class: "summary-body",
                            div { class: "summary-head",
                                span { class: "summary-glyph", aria_hidden: "true", "{outcome.glyph()}" }
                                span { class: "summary-cat", "{outcome.category}" }
                                span { class: "verdict-tag", "data-verdict": "{outcome.slug()}",
                                    "{outcome.label()}"
                                }
                            }
                            p { class: "summary-takeaway", "{outcome.takeaway}" }
                        }
                    }
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
    fn renders_summary_per_category_with_glyphs_no_ranking() {
        let html = dioxus_ssr::render_element(rsx! {
            SummaryPanel { summary: SummaryViewModel {
                answered_count: 2,
                outcomes: vec![
                    CategoryOutcome {
                        category: "Sources".into(),
                        motif: MotifKind::Link,
                        verdict: Some(VerdictKind::Juste),
                        takeaway: "Vérifier chaque source individuellement.".into(),
                    },
                    CategoryOutcome {
                        category: "Confidentialité".into(),
                        motif: MotifKind::Shield,
                        verdict: None,
                        takeaway: "À explorer.".into(),
                    },
                ],
                privacy_notice: "Aucun classement nominatif.".into(),
            } }
        });
        assert!(html.contains("Synthèse privée"));
        assert!(html.contains("Aucun classement nominatif."));
        assert!(html.contains("Sources"));
        // the verdict glyph and the "je ne sais pas" fallbacks are rendered
        assert!(html.contains('✓'));
        assert!(html.contains("data-verdict=\"idk\""));
        assert!(html.contains("À explorer"));
        // still no numeric scoring
        assert!(!html.contains("score_delta"));
    }

    #[test]
    fn renders_file_artifact_with_and_without_flag() {
        let with_flag = dioxus_ssr::render_element(rsx! {
            ScenarioArtifactView {
                artifact: ScenarioArtifact::File {
                    name: "brouillon.docx".into(),
                    meta: "Word · 84 Ko".into(),
                    flag: Some("sensibilité inconnue".into()),
                }
            }
        });
        assert!(with_flag.contains("attach-thumb"));
        assert!(with_flag.contains("brouillon.docx"));
        assert!(with_flag.contains("attach-flag"));
        assert!(with_flag.contains("sensibilité inconnue"));

        let no_flag = dioxus_ssr::render_element(rsx! {
            ScenarioArtifactView {
                artifact: ScenarioArtifact::File {
                    name: "note.pdf".into(),
                    meta: "PDF".into(),
                    flag: None,
                }
            }
        });
        assert!(no_flag.contains("note.pdf"));
        assert!(!no_flag.contains("attach-flag"));
    }

    #[test]
    fn renders_link_chips_for_each_source() {
        let html = dioxus_ssr::render_element(rsx! {
            ScenarioArtifactView {
                artifact: ScenarioArtifact::Links(vec![
                    LinkChip { url: "arxiv.org/abs/1".into(), note: "non vérifié".into() },
                    LinkChip { url: "blog.example.com".into(), note: "périmé ?".into() },
                ])
            }
        });
        assert_eq!(html.matches("class=\"chip\"").count(), 2);
        assert!(html.contains("arxiv.org/abs/1"));
        assert!(html.contains("non vérifié"));

        // empty list renders the container but no chips
        let empty = dioxus_ssr::render_element(rsx! {
            ScenarioArtifactView { artifact: ScenarioArtifact::Links(vec![]) }
        });
        assert!(empty.contains("class=\"links\""));
        assert!(!empty.contains("class=\"chip\""));
    }

    #[test]
    fn renders_image_artifact_with_accessible_label() {
        let html = dioxus_ssr::render_element(rsx! {
            ScenarioArtifactView {
                artifact: ScenarioArtifact::Image {
                    ai_tag: "généré par IA".into(),
                    alt: "Trois ingénieurs identiques".into(),
                }
            }
        });
        assert!(html.contains("role=\"img\""));
        assert!(html.contains("aria-label=\"Trois ingénieurs identiques\""));
        assert!(html.contains("généré par IA"));
    }

    #[test]
    fn renders_ticket_artifact_with_redacted_values() {
        let html = dioxus_ssr::render_element(rsx! {
            ScenarioArtifactView {
                artifact: ScenarioArtifact::Ticket(vec![
                    TicketRow { label: "Client".into(), value: "Jean Dupont".into() },
                    TicketRow { label: "E-mail".into(), value: "jean@x.fr".into() },
                ])
            }
        });
        // each PII value sits in a redact span (visually hidden)
        assert_eq!(html.matches("class=\"redact\"").count(), 2);
        assert!(html.contains("Client"));
    }

    #[test]
    fn renders_every_category_motif_variant() {
        for (kind, path_fragment) in [
            (MotifKind::Shield, "M12 3l7 3v5"),
            (MotifKind::Link, "M10 13a4 4"),
            (MotifKind::Portrait, "x=\"3\""),
            (MotifKind::Envelope, "M3 7l9 6 9-6"),
        ] {
            let html = dioxus_ssr::render_element(rsx! { CategoryMotif { kind } });
            assert!(html.contains("motif-svg"), "svg rendered for {kind:?}");
            assert!(
                html.contains(path_fragment),
                "expected path {path_fragment} for {kind:?}, got: {html}"
            );
        }
    }
}
