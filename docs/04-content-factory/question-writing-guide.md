---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ../question-model.md
  - ../templates/question-template.yml
  - ../03-domain-model/taxonomy.md
  - ../03-domain-model/competency-model.md
  - ../03-domain-model/risk-model.md
  - ./scenario-writing-guide.md
  - ./feedback-writing-guide.md
  - ./quality-gates.md
---

# Guide d'écriture des questions

## Objectif

Aider les auteurs à écrire des questions contextualisées, nuancées, sourcées et
validables.

Une question `rumble-ai-practices` entraîne une décision professionnelle. Elle ne
sert pas à tester une culture générale IA.

## Anatomie d'une bonne question

Une bonne question contient :

- un rôle professionnel ;
- une situation concrète ;
- une tâche à réaliser ;
- une contrainte ou un risque ;
- un outil ou contexte d'usage IA ;
- des choix plausibles ;
- une ou plusieurs réponses partiellement acceptables si nécessaire ;
- un feedback qui explique l'action recommandée ;
- des sources, claims ou politique interne ;
- des risques, concepts et compétences ;
- des métadonnées de revue.

## Intention pédagogique

Avant d'écrire, compléter :

```yaml
concept_ids:
  - concept-source-verification
competency_ids:
  - comp-check-source
risk_ids:
  - risk-hallucinated-source
misconception_ids:
  - misconception-asking-source-is-verification
learning_objective: L'utilisateur comprend que demander une source ne suffit pas : il faut vérifier la source primaire.
```

Si l'intention n'est pas claire, ne pas écrire la question.

## Formats d'interaction

| Format | Usage | Risque |
| --- | --- | --- |
| Choix unique | décision simple | peut être trop binaire |
| Choix multiples | plusieurs bonnes pratiques | demande scoring partiel clair |
| Ordonnancement | priorité d'actions | plus complexe à expliquer |
| Scénario à embranchements | arbitrage multi-étapes | coûteux à produire/revoir |
| Réponse libre | justification | risque PII, hors MVP par défaut |

Le MVP privilégie choix unique/multiples contextualisés.

## Difficulté

| Niveau | Caractéristiques |
| --- | --- |
| `beginner` | un risque principal, contexte court, choix évidents mais plausibles |
| `intermediate` | deux risques, réponse partielle possible, politique interne ou source |
| `advanced` | ambiguïté contrôlée, agent/RAG/média, arbitrage et escalade |

Une difficulté élevée ne doit pas venir d'un piège lexical.

## Choix et distracteurs

Un bon distracteur est :

- plausible ;
- relié à une misconception ;
- clairement risqué ou incomplet une fois le feedback lu ;
- non caricatural.

Un mauvais distracteur est :

- absurde ;
- humiliant ;
- dépendant d'une information absente ;
- correct dans une autre interprétation non exclue.

## Scoring

Le score mesure une décision pédagogique, pas une valeur de personne.

```yaml
choices:
  - id: verify_primary_source
    score: 1.0
    level: correct
  - id: ask_ai_for_source_only
    score: 0.2
    level: partial
  - id: reuse_without_check
    score: -1.0
    level: risky
    severity: high
```

Règles :

- au moins un choix positif ;
- réponses partielles explicites ;
- choix dangereux pénalisés avec feedback actionnable ;
- pas de score caché à finalité RH.

## Sources

Une question doit relier ses claims à :

- source publique vérifiable ;
- source interne identifiée ;
- `source_gap` si brouillon non publiable ;
- justification fictive si scénario purement narratif, sans claim fort.

Une source vendeur seule ne suffit pas pour une règle générale.

## Structure YAML recommandée

```yaml
id: q-source-verify-001
version: 1
status: draft
locale: fr-FR
title: Vérifier une source citée par l'IA
axis: source_verification
difficulty: beginner
concept_ids:
  - concept-source-verification
competency_ids:
  - comp-check-source
risk_ids:
  - risk-hallucinated-source
misconception_ids:
  - misconception-asking-source-is-verification
intent: Identifier que demander une source ne suffit pas à vérifier une affirmation.
context:
  role: chargée de veille
  scenario: Un assistant IA fournit une statistique avec une référence précise pour une note interne.
prompt: Que faites-vous avant de réutiliser cette statistique ?
interaction:
  type: multiple_choice
  min_choices: 1
  max_choices: 2
choices:
  - id: verify_primary
    label: Ouvrir la source primaire et vérifier qu'elle soutient la statistique.
    score: 1.0
    feedback: Correct. Une citation affichée n'est pas une preuve : il faut vérifier le passage source.
  - id: trust_because_cited
    label: Réutiliser la statistique car l'IA a donné une référence.
    score: -1.0
    severity: high
    feedback: Risqué. La source peut être inventée ou mal attribuée.
expected_reasoning:
  - Une source citée doit être vérifiée.
  - Le claim doit être soutenu par le passage original.
sources:
  - id: source-nist-ai-rmf-1-0
review:
  author: author-id
  reviewers: []
  last_reviewed_at: null
  confidence: medium
```

## Anti-patterns

- Question de culture générale sans décision à prendre.
- Vrai/faux trop simple sur un sujet contextuel.
- Piège lexical.
- Réponse correcte dépendant d'une règle non fournie.
- Feedback qui dit seulement « correct » ou « incorrect ».
- Prompt magique présenté comme bonne pratique universelle.
- Incitation à coller un document interne dans un outil IA externe.
- Source absente pour une affirmation forte.
- Question `approved` écrite directement par IA.

## Checklist auteur

- Ai-je donné assez de contexte pour décider ?
- L'intention pédagogique est-elle explicite ?
- Les concepts/risques/compétences sont-ils reliés ?
- Les distracteurs sont-ils réalistes sans être trompeurs ?
- Le meilleur choix est-il défendable avec les sources ?
- Les réponses partielles sont-elles notées comme partielles ?
- Le feedback propose-t-il une action concrète ?
- Le risque principal est-il explicite ?
- Le statut est-il `draft` ou `review`, jamais `approved` directement ?
- La question évite-t-elle toute PII réelle ou secret ?

## Critères d'acceptation

- La question passe le schéma.
- La correction est stable dans le contexte donné.
- Les risques sont renseignés.
- Les sources soutiennent les claims.
- La revue humaine est prévue avant publication.
- Le contenu reste compatible avec no-RH-scoring.
