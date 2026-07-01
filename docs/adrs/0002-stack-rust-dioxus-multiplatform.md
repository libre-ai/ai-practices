# ADR 0002 — Stack Rust-first / Dioxus multi-plateforme

## Statut

Acceptée.

## Contexte

Le projet doit être le premier `rumble-*` multi-plateforme de cette famille. Le risque est de lancer trop tôt plusieurs clients et de dupliquer la logique métier.

L'écosystème Rumble converge vers Rust core + Dioxus pour les produits interactifs.

## Décision

Adopter :

- Rust crates pures pour domaine, contenu, audit, session ;
- CLI Rust comme premier client complet ;
- API Axum si serveur nécessaire ;
- Dioxus/WASM pour PWA mobile-first ;
- Tauri 2 pour desktop seulement après preuve PWA ;
- mobile shell différé.

## Règles

- Aucun invariant métier dans l'UI.
- Pas de TypeScript durable pour le scoring ou validation contenu.
- Pas de natif iOS/Android avant preuve produit.
- `wasm32-unknown-unknown` devient un gate dès l'apparition de la PWA.

## Conséquences

Positives :

- un seul cœur métier ;
- tests déterministes ;
- trajectoire souveraine ;
- multi-plateforme progressive.

Coûts :

- Dioxus demande discipline et maturité ;
- l'UI peut avancer moins vite qu'une stack JS connue ;
- mobile natif est différé.

## Alternatives rejetées

- **Next.js durable** : rapide mais risque de logique métier côté client.
- **Expo/React Native** : mobile rapide mais divergence stack Rumble.
- **SwiftUI/Compose natifs** : trop coûteux pour un MVP.
