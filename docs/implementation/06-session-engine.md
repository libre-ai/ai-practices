# Paquet 06 — Moteur de session

## Objectif

Évaluer les réponses et produire feedback + synthèse.

## Crate

`crates/session`

## Interfaces attendues

```rust
start_session(track, questions) -> SessionState
submit_answer(state, question_id, choice_ids) -> AnswerEvaluation
complete_session(state) -> SessionSummary
```

## Règles

- Une question ne peut être répondue qu'une fois sauf mode entraînement explicite.
- Le feedback est calculé côté Rust, jamais côté UI.
- Les réponses partielles restent pédagogiques.
- La synthèse ne contient pas de classement nominatif, mais peut contenir un positionnement privé par distribution anonymisée.

## Tests

- réponse correcte ;
- réponse partielle ;
- réponse risquée ;
- erreur critique ;
- synthèse par axe ;
- session vide refusée.

## Acceptation

- Fonctionne sans API ni UI.
- Sorties sérialisables.
- Pas de stockage durable dans la crate.
