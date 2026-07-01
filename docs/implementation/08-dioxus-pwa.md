# Paquet 08 — PWA Dioxus

## Objectif

Créer la première surface utilisateur multi-plateforme : web/PWA mobile-first.

## Crates/apps

- `crates/ui`
- `apps/web`

## Parcours critique

1. Accueil + notice non-RH.
2. Choix parcours.
3. Question.
4. Réponse.
5. Feedback.
6. Synthèse.

## Règles

- Pas de scoring dans l'UI.
- Composants accessibles.
- Pas de CDN/fonts tiers.
- Pas de token en storage navigateur.
- Responsive mobile-first.

## Tests

- `cargo check --target wasm32-unknown-unknown`.
- Smoke navigateur quand harness ajouté.
- Vérification manuelle clavier.
- Vérification localStorage/sessionStorage.

## Acceptation

- Parcours complet en viewport mobile.
- Feedback lisible.
- Synthèse sans classement nominatif ; distribution privée possible si fournie par le core.
- Aucune donnée sensible persistée par défaut.
