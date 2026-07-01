# Paquet 07 — API Axum

## Objectif

Exposer le moteur de session via HTTP après stabilisation du core.

## Crate

`crates/api`

## Endpoints

Voir `docs/api-contracts.md`.

## Règles

- API adapter mince.
- Validation d'entrée stricte.
- Pas de PII en logs.
- CORS fermé par défaut.
- Auth optionnelle, pas nécessaire au mode anonyme local.

## Tests

- healthz ;
- catalog ;
- création session ;
- réponse ;
- synthèse ;
- payload invalide ;
- logs sans réponse libre.

## Acceptation

- Serveur local démarre.
- Parcours complet via HTTP.
- Pas de DB obligatoire pour MVP.
