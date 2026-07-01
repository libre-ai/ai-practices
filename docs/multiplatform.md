# Stratégie multi-plateforme

## Principe

Un seul cœur métier Rust. Plusieurs surfaces. Aucune réimplémentation du scoring ou de la validation dans l'UI.

```text
Rust core
  ├─ CLI local
  ├─ API serveur
  ├─ PWA Dioxus/WASM
  ├─ Desktop Tauri
  └─ Mobile shell différé
```

## Ordre de livraison

### P0 — Contrats et CLI

Objectif : prouver que le corpus est valide et que les sessions sont calculées correctement sans UI.

Livrables :

- `validate-corpus`
- `audit-corpus`
- `run-session --fixture`
- rapports JSON déterministes

### P1 — PWA mobile-first

Objectif : premier usage réel sur desktop et mobile navigateur.

Livrables :

- parcours responsive ;
- feedback immédiat ;
- synthèse ;
- accessibilité clavier/screen reader ;
- tests Playwright mobile.

### P2 — Serveur self-hostable

Objectif : partage de parcours et agrégats anonymisés.

Livrables :

- API Axum ;
- Postgres ;
- auth optionnelle ;
- export agrégé.

### P3 — Desktop

Objectif : usage offline/local si besoin prouvé.

Livrables :

- shell Tauri 2 ;
- stockage SQLite local ;
- import/export corpus ;
- pas de secret frontend.

### P4 — Mobile shell

Objectif : uniquement si PWA insuffisante.

Conditions d'entrée :

- auth web prouvée ;
- offline prouvé ;
- besoin de notifications/capteurs/install store explicite ;
- budget maintenance accepté.

## Anti-objectifs

- Créer trois UI différentes.
- Développer iOS/Android natif avant validation du produit.
- Mettre de la logique métier en TypeScript/Swift/Kotlin.
- Utiliser un SDK propriétaire qui enferme le produit.

## Gates multi-plateforme

- `cargo test --workspace`
- `cargo check --target wasm32-unknown-unknown`
- tests d'accessibilité sur parcours critique ;
- smoke mobile viewport ;
- vérification absence de secrets/PII dans logs et stockage navigateur ;
- taille bundle surveillée mais non prioritaire avant sécurité/qualité.
