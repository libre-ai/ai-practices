# Paquet 04 — Moteur d'audit

## Objectif

Produire des findings déterministes sur le corpus et l'audit de l'existant.

## Crate

`crates/audit`

## Checks MVP

- Correction sans source.
- Question vrai/faux sans contexte suffisant.
- Feedback contenant des formulations interdites connues.
- Média humain IA sans revue.
- Source URL absente ou invalide.
- Question `approved` proche de l'expiration de revue.

## Interfaces attendues

```rust
AuditFinding { id, severity, category, message, location, recommendation }
audit_corpus(questions) -> AuditReport
audit_media(media_refs) -> AuditReport
```

## Sévérités

- `info`
- `warn`
- `fail`
- `blocker`

## Acceptation

- Audit déterministe.
- Sortie JSON stable.
- Les blockers font échouer la CLI.
- Pas d'appel LLM.
