# Paquet 03 — Validation contenu

## Objectif

Charger le corpus versionné et refuser tout contenu non publiable.

## Crate

`crates/content`

## Entrées

- `content/questions/*.yml` ou `.json`.
- `schemas/question.schema.json`.

## Interfaces attendues

```rust
load_questions(path) -> Result<Vec<Question>, ContentError>
validate_question(question) -> Result<ValidationReport, ContentError>
validate_corpus(path) -> Result<CorpusReport, ContentError>
```

## Règles fail-closed

- Fichier illisible = erreur.
- Champ obligatoire absent = erreur.
- Question `approved` sans source/revue = erreur.
- Média référencé sans audit = erreur.
- Statut inconnu = erreur.

## Tests

- fixture valide ;
- fixture champ manquant ;
- fixture source manquante ;
- fixture média non audité ;
- rapport JSON stable.

## Acceptation

- Le corpus exemple passe.
- Une question volontairement invalide échoue.
- Le rapport est exploitable en CI.
