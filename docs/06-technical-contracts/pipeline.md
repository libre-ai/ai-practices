---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ../04-content-factory/content-factory.md
  - ../04-content-factory/quality-gates.md
  - ../testing-strategy.md
  - ./schemas.md
  - ./provenance.md
---

# Pipeline technique et éditorial

## Objectif

Relier validations de contenu, prompts, CLI, revue humaine et publication.

## Pipeline local cible

```text
format docs -> validate schemas -> validate corpus -> audit corpus/media
            -> review reports -> human gate -> approved content served
```

## Étapes

| Étape | Commande/outil | Bloque publication |
| --- | --- | --- |
| format/lint docs | futur markdown lint | non au début |
| validation corpus | CLI existant | oui |
| audit corpus | CLI existant | oui si blocker |
| audit média | CLI/rapport | oui si média sensible |
| validation graphe | futur CLI | oui pour nouveaux contenus liés |
| revue assistée | prompts/agents | non seule |
| revue humaine | MR/review | oui |
| build/test | cargo/playwright selon périmètre | oui pour release |

## Commandes actuelles

```bash
cargo run -p rumble-ai-practices-cli -- validate-corpus --content content/questions
cargo run -p rumble-ai-practices-cli -- audit-corpus --content content/questions --media content/media --out reports/audit.json
cargo test --workspace
```

## Commandes futures

```bash
cargo run -p rumble-ai-practices-cli -- validate-concepts --content content/concepts
cargo run -p rumble-ai-practices-cli -- validate-graph --graph content/graph.yml
cargo run -p rumble-ai-practices-cli -- audit-coverage --content content/questions --graph content/graph.yml
```

## Rapports attendus

- erreurs schéma ;
- blockers quality gates ;
- sources manquantes/expirées ;
- couverture concepts/compétences/risques ;
- médias sans revue ;
- contenus `approved` invalides ;
- revue humaine manquante.

## Publication

Seuls les contenus `approved` doivent être servis en cible de diffusion. Les
contenus `draft`/`review` peuvent être visibles en environnement de revue locale,
jamais en parcours utilisateur standard.

## Rollback

Un contenu problématique doit pouvoir être :

- passé `blocked` ;
- passé `retired` ;
- remplacé par version précédente ;
- exclu du catalogue ;
- tracé dans le rapport d'audit.

## Critères d'acceptation

- Le pipeline est local-first.
- Les blockers arrêtent la publication.
- Les rapports sont reproductibles.
- Les agents ne publient pas.
- Le rollback éditorial est documenté.
