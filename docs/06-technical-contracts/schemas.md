---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ../question-model.md
  - ../03-domain-model/ontology.md
  - ../04-content-factory/quality-gates.md
  - ../../schemas/question.schema.json
  - ../../schemas/media-review.schema.json
  - ../../schemas/audit-existing.schema.json
---

# Schémas

## Objectif

Définir les schémas JSON/YAML nécessaires à la documentation exhaustive et au
corpus futur.

## Schémas existants

| Schéma | Usage |
| --- | --- |
| `schemas/question.schema.json` | questions/scénarios |
| `schemas/media-review.schema.json` | revue médias |
| `schemas/audit-existing.schema.json` | audit d'existant |

## Schémas futurs proposés

| Schéma | Objet | Priorité |
| --- | --- | --- |
| `concept.schema.json` | Concept | haute |
| `source.schema.json` | Source | haute |
| `claim.schema.json` | Claim | haute |
| `competency.schema.json` | Competency | haute |
| `risk.schema.json` | Risk | haute |
| `misconception.schema.json` | Misconception | moyenne |
| `knowledge-graph.schema.json` | Nodes/edges | moyenne |
| `review-event.schema.json` | ReviewEvent | haute |
| `learning-path.schema.json` | LearningPath | moyenne |

## Champs transverses

Tout artefact publiable doit prévoir :

```yaml
id: stable-id
status: draft|review|approved|blocked|retired
version: 1
locale: fr-FR
review:
  author: author-id
  reviewers: []
  last_reviewed_at: null
  confidence: low|medium|high
provenance:
  assisted_by_ai: false
  notes: null
```

## Règles de validation

- `id` stable et unique.
- `approved` exige reviewers et date.
- `blocked` exige raison.
- `retired` exige raison et date.
- `source_gap` interdit en `approved` sauf justification interne explicite.
- références inter-objets vérifiables.
- aucun champ libre sensible obligatoire.

## Compatibilité

Les nouveaux champs doivent être ajoutés progressivement :

1. optionnels en lecture ;
2. warnings en validation ;
3. requis pour nouveaux contenus ;
4. requis pour `approved` après migration.

## Exemple concept minimal

```yaml
id: concept-source-verification
status: draft
label: Vérification des sources
domain: source_reliability
risk_axes:
  - source_verification
competencies:
  - comp-check-source
sources:
  - source-nist-ai-rmf-1-0
```

## Critères d'acceptation

- Chaque schéma futur a un usage clair.
- Les champs publication/revue sont cohérents.
- Les blockers peuvent être validés automatiquement.
- Les migrations sont progressives.
