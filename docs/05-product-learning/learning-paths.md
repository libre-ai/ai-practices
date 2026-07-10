---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ./pedagogy.md
  - ./scoring-model.md
  - ../03-domain-model/taxonomy.md
  - ../03-domain-model/competency-model.md
  - ../03-domain-model/risk-model.md
---

# Parcours d'apprentissage

## Objectif

Structurer des parcours par niveau, rôle et risque sans enfermer l'utilisateur
dans un profil RH.

## Principes

- Un parcours est une aide pédagogique, pas une certification.
- La progression est explicable.
- Les recommandations utilisent le minimum de données.
- Le mode local/offline doit rester possible.
- Les parcours sont composés de contenus `approved` uniquement en diffusion.

## Parcours socle

### `ai-practices-basics`

Objectif : réflexes essentiels.

Modules :

1. sortie IA ≠ preuve ;
2. données personnelles et secrets ;
3. source citée ≠ source vérifiée ;
4. feedback, revue humaine et responsabilité ;
5. synthèse privée.

### `data-and-privacy`

Objectif : protéger données et conformité.

Modules : classification, minimisation, pseudonymisation/anonymisation, outil
autorisé, logs/secrets.

### `sources-and-reliability`

Objectif : vérifier factualité et sources.

Modules : hallucination, source inventée, mauvaise attribution, RAG, obsolescence.

### `security-and-agents`

Objectif : reconnaître risques LLM/RAG/agents.

Modules : prompt injection, tool calling, permissions, action externe, logs.

### `bias-media-responsibility`

Objectif : identifier biais, médias synthétiques et responsabilité humaine.

Modules : stéréotypes, provenance, média IA, deepfake, décision à impact.

## Règles de progression

| Signal | Recommandation |
| --- | --- |
| erreurs `data_confidentiality` | parcours `data-and-privacy` |
| erreurs `source_verification` | parcours `sources-and-reliability` |
| choix `blocker` sécurité | fiche réflexe immédiate + module sécurité |
| réponses partielles répétées | drill ciblé misconception |
| réussite stable par axe | proposer niveau intermédiaire/avancé |

## Prérequis

- Les parcours débutants n'exigent aucun vocabulaire technique.
- Les parcours agents/RAG exigent notions : sortie IA, source, donnée sensible.
- Les parcours gouvernance exigent : responsabilité humaine, politique interne,
  minimisation.

## Données nécessaires

Localement :

- questions vues ;
- choix sélectionnés ;
- axes faibles/forts ;
- meilleur score local si activé.

Backend organisationnel optionnel :

- session pseudonymisée ou anonyme ;
- agrégats par axe ;
- cohorte si seuil k atteint.

Jamais nécessaire : nom, email, rang individuel, texte libre sensible.

## Critères d'acceptation

- Les parcours sont explicables.
- La personnalisation respecte la minimisation.
- Les sorties sont pédagogiques, pas RH.
- Chaque parcours se relie à compétences, risques et contenus validés.
