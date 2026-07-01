# Paquet 05 — CLI

## Objectif

Créer le premier client complet pour agents et CI.

## Crate

`crates/cli`

## Commandes

```bash
rumble-ai-practices validate-corpus --content content/questions
rumble-ai-practices audit-corpus --content content/questions --out reports/audit.json
rumble-ai-practices run-session --fixture fixtures/session-basic.json --out reports/session-summary.json
rumble-ai-practices inspect-question --id q-data-001 --content content/questions
```

## Règles

- Code sortie `0` si OK.
- Code sortie `1` si validation/audit fail.
- Code sortie `2` si erreur usage CLI.
- Sorties JSON pour CI.
- Pas de secret dans les logs.

## Acceptation

- `--help` fonctionne.
- `validate-corpus` passe sur corpus exemple.
- `audit-corpus` produit un rapport.
- Tests CLI avec fixtures.
