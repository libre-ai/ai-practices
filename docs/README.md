# Documentation technique

Cette documentation sert de cahier de passation pour humains et agents d'implémentation légers.

## Programme documentaire exhaustif

Le chantier long de documentation est cadré dans
[`00-programme/programme-documentaire.md`](00-programme/programme-documentaire.md).
Les documents `00-programme/` à `07-benchmark/` préparent la base exhaustive
recherche → modèle de domaine → content factory → produit → contrats. Tant qu'ils
ne sont pas `stable`, les documents historiques listés ci-dessous restent
canoniques.

## Ordre de lecture recommandé

0. [`00-programme/programme-documentaire.md`](00-programme/programme-documentaire.md) — cadrage du chantier documentaire exhaustif.
1. [`vision.md`](vision.md) — pourquoi le produit existe.
2. [`cahier-des-charges.md`](cahier-des-charges.md) — comportement attendu.
3. [`product-boundaries.md`](product-boundaries.md) — ce que le repo possède/refuse.
4. [`adrs/`](adrs/) — décisions structurantes.
5. [`stack.md`](stack.md) et [`multiplatform.md`](multiplatform.md) — stack cible.
6. [`data-model.md`](data-model.md), [`question-model.md`](question-model.md), [`scoring.md`](scoring.md) — contrats métier.
7. [`content-governance.md`](content-governance.md) et [`grille-audit-biais-media.md`](grille-audit-biais-media.md) — garde-fous éditoriaux.
8. [`local-review.md`](local-review.md) — build local, localhost et gate humaine.
9. [`product-readiness.md`](product-readiness.md) — cockpit local de readiness canonique.
10. [`model-delegation.md`](model-delegation.md) — délégation à des modèles légers.
11. [`implementation/`](implementation/) — paquets de travail pour agents.
12. [`benchmark-jeu-competitif.md`](benchmark-jeu-competitif.md) — veille : ce qu'on prend / rejette d'un jeu externe d'entraînement IA, et pourquoi (non-objectifs assumés).

## Principe de documentation

Chaque doc doit être :

- actionnable par un agent ;
- testable ou vérifiable ;
- explicite sur les non-objectifs ;
- alignée sur sécurité > qualité > performance > complétude.
