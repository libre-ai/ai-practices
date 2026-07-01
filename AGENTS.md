# rumble-ai-practices — consignes agents

## Identité

`rumble-ai-practices` est un produit Rumble multi-plateforme qui entraîne des réflexes professionnels d'usage de l'IA. Le cœur du risque est éditorial : une application techniquement correcte mais un corpus biaisé ou faux est un échec.

## Doctrine

- **Rust-first** : invariants métier, scoring, validation corpus et audit vivent en Rust.
- **Content-as-data** : les questions sont des fichiers versionnés, pas du texte codé dans l'UI.
- **Validation humaine obligatoire** : aucune question ou correction générée par IA n'est publiée sans revue.
- **Nuance > binaire** : préférer les scénarios contextualisés aux vrai/faux simplistes.
- **Feedback pédagogique** : expliquer le risque et l'action recommandée, pas seulement "bravo/incorrect".
- **Pas d'usage RH implicite** : pas de leaderboard nominatif, pas de score global humiliant, pas de profilage caché.
- **Médias IA sous contrôle** : pas de visage humain généré par IA hors cas justifié et audité.
- **Souveraineté** : self-hostable, build local et revue localhost par défaut ; aucune plateforme cloud dans le flux nominal.
- **Preuve > promesse** : tout incrément doit laisser une commande de vérification reproductible.

## Architecture cible

```text
crates/
  domain/    types purs : Question, Scenario, Choice, RiskAxis, Score, Feedback
  content/   chargement YAML/JSON, validation schema, règles de publication
  audit/     audit existant, biais média, cohérence source/correction
  session/   progression, réponses, synthèse pédagogique
  api/       adapter HTTP Axum
  cli/       validation, audit, import/export, fixtures
  ui/        composants Dioxus réutilisables
apps/
  web/       PWA Dioxus/WASM mobile-first
  desktop/   Tauri 2 après PWA validée
  mobile/    différé, pas de natif avant preuve produit
content/
  questions/ corpus versionné
schemas/     contrats lisibles par agents
```

## Quality gates attendus dès que le code existe

```bash
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo deny check
```

Pour la PWA quand elle existe :

```bash
# nom exact à stabiliser avec la stack Dioxus retenue
cargo check --target wasm32-unknown-unknown
# puis smoke navigateur/mobile avec Playwright si un harness web est ajouté
```

## Règles de modification

- Lire les docs et fichiers concernés avant édition.
- Préférer de petits changements réversibles.
- Documenter toute décision structurante dans `docs/adrs/`.
- Ne jamais ajouter de dépendance majeure sans justification licence, souveraineté, maintenance, alternatives rejetées.
- Ne jamais introduire de collecte de données personnelles sans mise à jour de `docs/security-rgpd.md`.
- Ne jamais publier de contenu pédagogique sans métadonnées de revue.
- Ne jamais masquer un biais média par une explication défensive : si doute, marquer `blocked`.

## Priorité d'implémentation

1. Créer workspace Rust minimal.
2. Implémenter `crates/domain` et tests d'invariants.
3. Implémenter `crates/content` avec validation de `schemas/question.schema.json`.
4. Implémenter `crates/audit` avec rapports déterministes.
5. Créer CLI `validate-corpus` et `audit-corpus`.
6. Ajouter `crates/session` et synthèse pédagogique.
7. Ajouter API Axum.
8. Ajouter PWA Dioxus.
9. Évaluer desktop Tauri.
10. Évaluer mobile seulement après preuves sécurité/offline.
