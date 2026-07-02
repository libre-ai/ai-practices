# ADR 0002 — Stack Rust-first / Portal multi-plateforme

## Statut

Acceptée.

## Contexte

Le projet doit être le premier `rumble-*` multi-plateforme de cette famille. Le risque est de lancer trop tôt plusieurs clients et de dupliquer la logique métier.

L'écosystème Rumble converge vers Rust-first product core + Portal client platform pour les produits interactifs. Dioxus/PWA est la voie rapide par défaut ; SwiftUI/Compose sont des voies natives first-class lorsque le besoin produit et la vérification locale le justifient.

## Décision

Adopter :

- Rust crates pures pour domaine, contenu, audit, session ;
- CLI Rust comme premier client complet ;
- API Axum si serveur nécessaire ;
- PWA Rust/WASM mobile-first consommant Portal ;
- Tauri 2 pour desktop seulement après preuve PWA ;
- SwiftUI/Compose via Portal seulement après preuve produit/offline/auth.

## Règles

- Aucun invariant métier dans l'UI.
- Pas de TypeScript durable pour le scoring ou validation contenu.
- Pas de natif iOS/Android sans preuve produit et vérification locale.
- `wasm32-unknown-unknown` devient un gate dès l'apparition de la PWA.
- Les tokens, conventions d'accessibilité, i18n UI et adapters natifs/web partagés viennent de Portal.

## Conséquences

Positives :

- un seul cœur métier ;
- tests déterministes ;
- trajectoire souveraine ;
- multi-plateforme progressive.

Coûts :

- Portal demande discipline contractuelle et intégration multi-repo ;
- la voie PWA Rust peut avancer moins vite qu'une stack JS connue ;
- mobile natif reste conditionné à une preuve produit vérifiable.

## Alternatives rejetées

- **Next.js durable** : rapide mais risque de logique métier côté client.
- **Expo/React Native** : mobile rapide mais divergence stack Rumble.
- **SwiftUI/Compose natifs immédiats sans Portal** : trop coûteux et source de duplication métier pour un MVP.
