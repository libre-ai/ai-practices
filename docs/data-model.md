# Modèle de données

## Types métier principaux

```rust
Question {
  id: QuestionId,
  version: u32,
  status: PublicationStatus,
  locale: Locale,
  title: String,
  axis: RiskAxis,
  difficulty: Difficulty,
  intent: String,
  context: ScenarioContext,
  interaction: Interaction,
  choices: Vec<Choice>,
  expected_reasoning: Vec<String>,
  risks: Vec<RiskTag>,
  sources: Vec<SourceRef>,
  media: Vec<MediaRef>,
  review: ReviewMetadata,
}
```

## Axes de risque

```text
data_confidentiality
source_verification
hallucination
bias_fairness
security
privacy_rgpd
human_responsibility
prompt_practice
business_context
media_synthetic
```

## Résultat de réponse

```rust
AnswerEvaluation {
  question_id,
  selected_choice_ids,
  score_delta,
  level: Correct | Partial | Risky | Incorrect,
  axis_impacts: Vec<AxisImpact>,
  feedback_cards: Vec<FeedbackCard>,
  evidence_refs: Vec<SourceRef>,
}
```

## Synthèse de session

```rust
SessionSummary {
  session_id,
  completed_at,
  answered_count,
  axis_levels: Vec<AxisLevel>,
  private_distribution: Option<DistributionPosition>,
  strengths: Vec<String>,
  recommended_practices: Vec<String>,
  next_modules: Vec<ModuleRef>,
  privacy_notice: String,
}

DistributionPosition {
  cohort_label,
  min_cohort_size_met: bool,
  buckets: Vec<DistributionBucket>,
  user_bucket: Option<String>,
}
```

## Données personnelles

### À éviter en MVP

- Nom/prénom si non nécessaire.
- Email si pas d'auth.
- Réponses libres contenant potentiellement des données sensibles.
- IP stockée dans les rapports applicatifs.

### Si mode organisation activé

- Identifiant utilisateur pseudonymisé.
- Groupe/cohorte optionnel.
- Résultats agrégés par axe.
- Distributions par niveaux/intervalles, jamais rang nominatif exact.
- Rétention courte et documentée.

## Rétention recommandée

| Donnée | Rétention |
| --- | --- |
| Corpus questions | indéfinie, versionnée |
| Audit contenu | indéfinie, traçabilité |
| Session locale anonyme | supprimable immédiatement |
| Résultat individuel authentifié | 30 jours par défaut, configurable |
| Agrégats anonymisés | 12 mois maximum |
| Logs techniques | 30 jours maximum |

## Événements métier futurs

```text
QuestionLoaded
QuestionValidated
QuestionBlocked
MediaReviewed
SessionStarted
AnswerSubmitted
FeedbackShown
SessionCompleted
AggregateReported
```

Ces événements ne doivent pas contenir de texte libre sensible ni de secret.
