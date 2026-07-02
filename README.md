# rumble-ai-practices

> **Rumble Officialized Product**: This is a Rumble-tier product with official governance. Documentation is intentionally in French to reflect the product's primary audience and strategic positioning.

**Layer:** Rumble — Product  
**Role:** multi-plateforme d'entraînement aux pratiques professionnelles de l'IA  
**Mission:** aider des collaborateurs à renforcer leurs réflexes IA par des scénarios réalistes, sourcés, non binaires et audités contre les biais.

---

## Positionnement

`rumble-ai-practices` n'est pas un quiz de culture générale IA et ne doit pas devenir un outil de notation RH.

Le produit sert à :

- diagnostiquer des réflexes professionnels ;
- entraîner les bonnes pratiques : confidentialité, vérification, sources, biais, RGPD, sécurité, responsabilité ;
- expliquer pourquoi une réponse est bonne, risquée ou incomplète ;
- tracer la qualité éditoriale des questions, médias et corrections.

Le produit refuse :

- les classements nominatifs ;
- les corrections péremptoires sans source ;
- les images/vidéos générées par IA non auditées ;
- l'usage disciplinaire ou RH implicite ;
- la génération automatique de questions publiée sans revue humaine.

## Stack cible

Trajectoire Rumble interactive : **Rust-first + Portal client platform**. Dioxus/PWA reste la voie rapide par défaut pour prouver le produit ; SwiftUI/Compose sont des voies natives first-class si le besoin pédagogique et la vérification locale le justifient.

```text
crates/domain          types purs, scénarios, scoring, invariants
crates/content         chargement/validation du corpus versionné
crates/audit           audit des questions, médias, biais et corrections
crates/session         progression, réponses, feedback, synthèse
crates/api             adapter HTTP Axum
crates/cli             validation locale, import/export, audit corpus
crates/ui              composants métier consommant Portal
apps/web               PWA mobile-first Rust/WASM
apps/desktop           Tauri 2 après PWA prouvée
apps/mobile            SwiftUI/Compose via Portal après preuve offline/auth
content/questions      corpus pédagogique versionné
schemas                contrats JSON/YAML lisibles par agents
```

Voir [`docs/stack.md`](docs/stack.md) et [`docs/multiplatform.md`](docs/multiplatform.md).

## Première stratégie produit

1. **Audit de l'existant** : prouver les erreurs, biais, dogmes et risques.
2. **Corpus contract-first** : modèle de question, sources, relecture, scoring.
3. **Core Rust** : invariants testables sans UI ni réseau.
4. **CLI de validation** : premier client complet pour modèles légers.
5. **PWA mobile-first** : première surface utilisateur multi-plateforme.
6. **Desktop/mobile** : seulement après gates de sécurité, accessibilité et contenu.

## Démarrage

```bash
cargo test --workspace
cargo run -p rumble-ai-practices-cli -- validate-corpus --content content/questions
cargo run -p rumble-ai-practices-cli -- audit-corpus --content content/questions --media content/media --out reports/audit.json
cargo run -p rumble-ai-practices-cli -- inspect-question --id q-source-001 --content content/questions --media content/media
cargo run -p rumble-ai-practices-cli -- run-session --fixture fixtures/session-basic.json --content content/questions --media content/media --out reports/session-basic.json
cargo run -p rumble-ai-practices-cli -- serve --bind 127.0.0.1:3000
# puis ouvrir http://127.0.0.1:3000 ; smoke API/PWA : /readyz, /manifest.webmanifest, /sw.js
```

## Démarrage documentaire

```bash
# Lire le cadrage produit
open docs/vision.md
open docs/cahier-des-charges.md

# Lire les décisions structurantes
open docs/adrs/0001-product-boundary.md
open docs/adrs/0002-stack-rust-dioxus-multiplatform.md
open docs/adrs/0003-content-governance-and-no-rh-scoring.md
open docs/adrs/0004-media-ai-bias-review.md

# Donner un paquet de travail à un agent léger
open docs/implementation/00-agent-operating-model.md
open docs/implementation/03-content-validation.md
```

## Critère de succès MVP

Le MVP est acceptable quand :

- 30 questions/scénarios sont validés par la gouvernance de contenu ;
- chaque question a explication, sources, risques et date de revue ;
- tout média IA a une revue de biais documentée ;
- le CLI refuse un corpus invalide ;
- la PWA permet un parcours complet sans stocker de donnée sensible ;
- la synthèse affiche des axes de progression et un positionnement privé anonymisé, pas une note RH.

## Documentation

- [Vision](docs/vision.md)
- [Cahier des charges](docs/cahier-des-charges.md)
- [Architecture](docs/architecture.md)
- [Stack](docs/stack.md)
- [Multi-plateforme](docs/multiplatform.md)
- [Modèle de données](docs/data-model.md)
- [Contrats API](docs/api-contracts.md)
- [Sécurité & RGPD](docs/security-rgpd.md)
- [Revue locale & gate humaine](docs/local-review.md)
- [Gouvernance contenu](docs/content-governance.md)
- [Grille biais média](docs/grille-audit-biais-media.md)
- [Stratégie de tests](docs/testing-strategy.md)
- [Plan d'implémentation](docs/implementation-plan.md)
- [Délégation à des modèles légers](docs/model-delegation.md)
