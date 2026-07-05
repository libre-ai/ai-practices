---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ./taxonomy.md
  - ./competency-model.md
  - ./risk-model.md
  - ../data-model.md
  - ../question-model.md
  - ../../schemas/question.schema.json
  - ../06-technical-contracts/schemas.md
---

# Ontologie métier

## Objectif

Définir les objets métier et leurs relations pour éviter les divergences entre
documentation, schémas, code et corpus.

Cette ontologie n'est pas une ontologie OWL complète. C'est un modèle métier
canonique pour `rumble-ai-practices`.

## Principes

- Un objet a une responsabilité unique.
- Les objets publiables ont un statut et une revue.
- Les claims sont séparés des sources : une source soutient un claim précis.
- Les compétences sont observables via contenus et sessions.
- Les risques sont évalués par axe et sévérité pédagogique.
- Les données personnelles des apprenants restent hors corpus.

## Vue synthétique

```text
Source -> Claim -> Concept -> Competency -> LearningObjective
                         \        \          \
                          \        \          -> Question -> Choice -> Feedback
                           \        -> Risk ----/
                            -> Misconception

Media -> MediaReview -> Question
ReviewEvent -> any publishable artifact
Session -> AnswerEvaluation -> SessionSummary
```

## Objets canoniques

### `Source`

Référence documentaire, normative, scientifique, technique, vendeur ou interne.

Champs clés :

- `id`
- `level`
- `type`
- `publisher`
- `label`
- `url | internal_ref`
- `published_at | year`
- `accessed_at`
- `claim_scope`
- `limitations`
- `review`

Invariant : une source ne soutient pas tout ; elle soutient des claims limités.

### `Claim`

Affirmation vérifiable utilisée dans la documentation ou le corpus.

Champs clés :

- `id`
- `statement`
- `scope`
- `confidence`
- `source_ids`
- `limitations`
- `last_reviewed_at`

Invariant : un claim fort sans source devient `source_gap` et ne peut pas être
publié dans un contenu `approved`.

### `Concept`

Notion enseignable reliée à des risques et compétences.

Champs clés :

- `id`
- `label`
- `summary`
- `domain`
- `level`
- `aliases`
- `not_to_confuse_with`
- `prerequisites`
- `relations`
- `risk_axes`
- `competencies`
- `claims`
- `review`

Invariant : un concept doit contribuer à au moins une compétence ou un risque.

### `Risk`

Risque pédagogique ou opérationnel à reconnaître/mitiger.

Champs clés :

- `id`
- `axis`
- `severity_default`
- `description`
- `indicators`
- `mitigations`
- `blockers`
- `related_concepts`

Invariant : un risque de PII/secret ou usage RH implicite peut bloquer un contenu.

### `Competency`

Capacité professionnelle observable.

Champs clés :

- `id`
- `label`
- `dimension`
- `levels`
- `observable_behaviors`
- `risk_axes`
- `concepts`
- `assessment_methods`

Invariant : une compétence n'est valide que si elle peut être observée dans un
scénario, une question ou une justification.

### `LearningObjective`

Objectif pédagogique reliant compétence, niveau, concept et contenu.

Champs clés :

- `id`
- `competency_id`
- `level`
- `concept_ids`
- `risk_ids`
- `observable_outcome`
- `assessment_hint`

Invariant : un objectif doit être formulé comme comportement observable, pas
comme thème vague.

### `Misconception`

Croyance erronée ou raccourci dangereux.

Champs clés :

- `id`
- `statement`
- `why_seductive`
- `risk_ids`
- `correction`
- `scenario_hooks`

Invariant : la correction doit être actionnable et non humiliante.

### `Scenario`

Situation professionnelle contextualisée.

Champs clés :

- `role`
- `task`
- `data_context`
- `tool_context`
- `constraint`
- `stakes`
- `ambiguity_level`

Invariant : une réponse correcte doit être déterminable à partir du contexte.

### `Question`

Contenu d'entraînement validé par schéma existant.

Champs clés : voir `../question-model.md`.

Relations nouvelles proposées :

- `concept_ids`
- `competency_ids`
- `learning_objective_ids`
- `claim_ids`
- `misconception_ids`

Invariant : une question `approved` doit avoir revue humaine et sources/justification.

### `Choice`

Option de réponse, avec score et feedback.

Champs clés :

- `id`
- `label`
- `score`
- `severity`
- `feedback`
- `risk_impacts`
- `concept_signals`

Invariant : un choix risqué doit expliquer le risque créé.

### `FeedbackCard`

Unité de correction pédagogique affichable.

Champs clés :

- `level`
- `message`
- `risk_id`
- `recommended_action`
- `source_refs`
- `next_concepts`

Invariant : feedback = risque + raison + action.

### `MediaAsset`

Média utilisé par le produit.

Champs clés :

- `id`
- `type`
- `origin`
- `synthetic_status`
- `rights`
- `provenance`
- `review_id`

Invariant : média IA humain réaliste sans revue = bloqué.

### `ReviewEvent`

Trace de revue humaine ou assistée.

Champs clés :

- `id`
- `artifact_type`
- `artifact_id`
- `reviewer_role`
- `decision`
- `findings`
- `created_at`
- `evidence`

Invariant : une revue assistée par IA ne peut pas décider `approved`.

### `LearningPath`

Séquence pédagogique par niveau, rôle ou risque.

Champs clés :

- `id`
- `target_level`
- `competency_ids`
- `question_ids`
- `remediation_rules`

Invariant : ne doit pas créer de profil RH nominatif.

### `Session`

Exécution d'un parcours par un utilisateur.

Champs clés : voir `../data-model.md`.

Invariant : les résultats individuels restent privés ou pseudonymisés selon mode,
jamais leaderboard nominatif.

## Cardinalités principales

| Relation | Cardinalité |
| --- | --- |
| Source -> Claim | 1..n claims par source ; 1..n sources par claim |
| Claim -> Concept | 0..n concepts par claim ; 0..n claims par concept |
| Concept -> Risk | n..n |
| Concept -> Competency | n..n |
| Competency -> LearningObjective | 1..n |
| LearningObjective -> Question | n..n |
| Question -> Choice | 2..n |
| Question -> Source/Claim | 0..n, obligatoire pour `approved` sauf justification interne |
| Question -> ReviewEvent | 1..n pour `approved` |
| MediaAsset -> MediaReview | 1..n si média utilisé |
| Session -> AnswerEvaluation | 0..n |

## Invariants testables

- Aucun `approved` sans revue humaine.
- Aucun `approved` avec `source_gap` bloquant.
- Aucun contenu avec média IA humain réaliste sans `MediaReview`.
- Aucun leaderboard nominatif.
- Aucun claim juridique sans source A/F et revue adaptée.
- Aucun choix correct qui expose PII/secrets dans un outil non autorisé.
- Aucun cycle dans les relations `requires`.
- Aucun concept orphelin après passage `approved`.

## Diff avec le modèle actuel

Le modèle actuel couvre surtout `Question`, `Choice`, `ReviewMetadata`, `SourceRef`,
`RiskAxis`, `SessionSummary`. Les extensions proposées ajoutent :

- `Concept`
- `Claim`
- `Competency`
- `LearningObjective`
- `Misconception`
- `ReviewEvent` généralisé
- relations graph entre objets

Ces extensions ne nécessitent pas une refonte immédiate du code. Elles servent de
cible pour `schemas.md` et les futures migrations.

## Critères d'acceptation

- Chaque objet a une responsabilité claire.
- Les doublons sont signalés.
- Les migrations nécessaires sont séparées du modèle conceptuel.
- Les invariants sécurité/RGPD/no-RH sont testables.
- Le modèle reste compatible avec le corpus actuel.
