---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ../data-model.md
  - ../03-domain-model/ontology.md
  - ../03-domain-model/knowledge-graph.md
  - ./schemas.md
---

# Modèle de données étendu

## Objectif

Préparer l'extension du modèle actuel pour concepts, compétences, sources et
graphe sans casser le code existant.

## Principe de migration

Ne pas refondre le runtime d'un coup. Ajouter les objets comme données de
référence, puis brancher progressivement validation, couverture et parcours.

## Modèle actuel à préserver

- `Question`
- `Choice`
- `RiskAxis`
- `ReviewMetadata`
- `SourceRef`
- `AnswerEvaluation`
- `SessionSummary`

## Objets à ajouter

| Objet | Usage | Stockage initial |
| --- | --- | --- |
| `Concept` | taxonomie | YAML/JSON |
| `Claim` | affirmation sourcée | YAML/JSON |
| `Source` | bibliographie structurée | YAML/JSON |
| `Competency` | compétence observable | YAML/JSON |
| `LearningObjective` | objectif d'évaluation | YAML/JSON |
| `Misconception` | piège à corriger | YAML/JSON |
| `GraphRelation` | relation entre objets | YAML/JSON |
| `ReviewEvent` | preuve de revue | métadonnées + rapport |
| `Provenance` | origine/génération/modification | métadonnées |

## Extensions proposées à `Question`

```yaml
concept_ids: []
competency_ids: []
learning_objective_ids: []
risk_ids: []
misconception_ids: []
claim_ids: []
provenance:
  assisted_by_ai: false
  source_inputs: []
```

Ces champs peuvent d'abord être optionnels, puis requis pour `review`/`approved`.

## Invariants

- `approved` exige revue humaine.
- `approved` ne tolère pas `source_gap` bloquant.
- `concept_ids` ne doivent pas référencer un concept inexistant.
- `risk_ids` doivent correspondre à un axe connu.
- `ReviewEvent` IA ne peut pas approuver.
- Les événements session ne contiennent pas de texte libre sensible.

## Plan progressif

1. Documenter schémas futurs.
2. Ajouter fixtures concepts/compétences/risques.
3. Ajouter validation référentielle dans CLI.
4. Ajouter audit de couverture.
5. Brancher recommandations de parcours.
6. Évaluer stockage dédié seulement si volume l'exige.

## Critères d'acceptation

- Les extensions sont réversibles.
- Les invariants sécurité/RGPD restent prioritaires.
- Les schémas futurs sont explicités.
- Le modèle actuel reste compatible.
