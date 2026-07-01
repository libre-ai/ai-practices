# ADR 0001 — Frontière produit

## Statut

Acceptée.

## Contexte

Le besoin initial vient d'un outil existant d'évaluation des connaissances IA jugé biaisé, simpliste et parfois faux. La tentation serait de cloner le fonctionnement en remplaçant les questions.

## Décision

Créer `rumble-ai-practices` comme produit Rumble spécialisé dans l'entraînement aux pratiques IA, avec gouvernance de contenu et audit des biais comme cœur produit.

Le repo possède : parcours, corpus, scoring pédagogique, audit média, UX et validation corpus.

Le repo ne possède pas : LMS générique, orchestration agentique, ingestion générique, notation RH.

## Conséquences

- Le contenu est une première classe du système.
- L'app ne peut pas être considérée terminée si le corpus est faible.
- Les primitives réutilisables pourront migrer vers Wrench/Gear/Bolt plus tard.
- `rumble-lm` reste une possibilité d'intégration, mais ce repo porte le cas d'usage spécialisé.

## Alternatives rejetées

- **Simple clone du quiz existant** : conserve les mauvais réflexes.
- **Pack de contenu uniquement pour `rumble-lm`** : trop tôt tant que la gouvernance spécifique n'est pas prouvée.
- **Outil RH d'évaluation** : risque social, RGPD et qualité trop élevé.
