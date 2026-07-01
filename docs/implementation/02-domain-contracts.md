# Paquet 02 — Contrats domaine

## Objectif

Implémenter les types purs et invariants métier.

## Crate

`crates/domain`

## Interfaces attendues

```rust
Question
QuestionId
PublicationStatus
RiskAxis
Difficulty
Choice
ChoiceScore
ReviewMetadata
AnswerEvaluation
SessionSummary
```

## Invariants

- Une question publiable a au moins une réponse positive.
- Un choix a toujours un feedback.
- Un score critique négatif porte une sévérité.
- Une question `approved` a auteur, relecteur et date.
- Un média humain IA référencé sans audit bloque la publication.

## Tests

- construction valide ;
- rejet question sans choix ;
- rejet feedback manquant ;
- rejet `approved` sans reviewer ;
- sérialisation JSON stable.

## Acceptation

- Aucun adapter dans `domain`.
- Erreurs typées avec `thiserror`.
- Pas de `anyhow` dans le domaine.
