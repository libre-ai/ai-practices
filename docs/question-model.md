# Modèle de question

## Philosophie

Une question n'est publiable que si elle encode :

- l'intention pédagogique ;
- le contexte ;
- les risques ;
- les réponses attendues et partielles ;
- l'explication ;
- les sources ;
- la revue éditoriale ;
- les biais potentiels.

## Structure logique

```yaml
id: q-data-001
version: 1
status: draft # draft | review | approved | blocked | retired
locale: fr-FR
title: Vérifier un brouillon avant usage IA
axis: data_confidentiality
difficulty: beginner
intent: Identifier que la destination publique d'un document ne suffit pas à le rendre non sensible.
context:
  role: collaborateur marketing
  scenario: Vous voulez faire relire un brouillon d'article technique par un assistant IA externe.
  assets:
    - type: document_ref
      label: brouillon_blog_tech.docx
prompt: Que faites-vous avant d'envoyer le fichier ?
interaction:
  type: multiple_choice
  min_choices: 1
  max_choices: 3
choices:
  - id: inspect_content
    label: Vérifier le contenu réel, commentaires, métadonnées et classification.
    score: 1.0
    feedback: Bonne pratique. L'intention de publication ne garantit pas l'absence de données sensibles.
  - id: send_because_public
    label: Envoyer directement car l'article est destiné à être publié.
    score: -1.0
    severity: high
    feedback: Risqué. Un brouillon peut contenir données internes, clients, roadmap, commentaires ou métadonnées.
expected_reasoning:
  - Ne pas juger la sensibilité par le nom du fichier.
  - Vérifier contenu, classification et politique interne.
risks:
  - confidentiality_leak
  - pii_leak
sources:
  - id: internal-data-policy
    label: Politique interne de classification des données
    url: internal://data-classification
    type: internal_policy
review:
  author: TBD
  reviewers: []
  last_reviewed_at: null
  confidence: medium
  notes: À valider avec politique interne.
```

## Champs obligatoires pour publication

- `id`, `version`, `status=approved`, `locale`, `title`.
- `axis`, `difficulty`, `intent`.
- `context.scenario`.
- `interaction.type`.
- `choices` avec au moins une réponse positive.
- `expected_reasoning`.
- `risks`.
- `sources` ou justification `source_gap`.
- `review.author`, `review.reviewers`, `review.last_reviewed_at`, `review.confidence`.

## Règles de qualité

- Pas de correction qui affirme une vérité non sourcée.
- Pas de piège basé sur une ambiguïté de vocabulaire.
- Pas de "bravo" sans expliquer le risque évité.
- Pas de "incorrect" si la réponse est partiellement acceptable.
- Les questions à contexte variable doivent afficher la condition.

## Statuts

| Statut | Signification |
| --- | --- |
| `draft` | brouillon non publiable |
| `review` | prêt à relire |
| `approved` | publiable |
| `blocked` | risque éditorial/sécurité/biais |
| `retired` | retiré mais conservé pour traçabilité |
