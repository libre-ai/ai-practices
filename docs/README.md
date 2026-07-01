# Documentation technique

Cette documentation sert de cahier de passation pour humains et agents d'implémentation légers.

## Ordre de lecture recommandé

1. [`vision.md`](vision.md) — pourquoi le produit existe.
2. [`cahier-des-charges.md`](cahier-des-charges.md) — comportement attendu.
3. [`product-boundaries.md`](product-boundaries.md) — ce que le repo possède/refuse.
4. [`adrs/`](adrs/) — décisions structurantes.
5. [`stack.md`](stack.md) et [`multiplatform.md`](multiplatform.md) — stack cible.
6. [`data-model.md`](data-model.md), [`question-model.md`](question-model.md), [`scoring.md`](scoring.md) — contrats métier.
7. [`content-governance.md`](content-governance.md) et [`grille-audit-biais-media.md`](grille-audit-biais-media.md) — garde-fous éditoriaux.
8. [`local-review.md`](local-review.md) — build local, localhost et gate humaine.
9. [`model-delegation.md`](model-delegation.md) — délégation à des modèles légers.
10. [`implementation/`](implementation/) — paquets de travail pour agents.
11. [`benchmark-jeu-competitif.md`](benchmark-jeu-competitif.md) — veille : ce qu'on prend / rejette d'un jeu externe d'entraînement IA, et pourquoi (non-objectifs assumés).

## Principe de documentation

Chaque doc doit être :

- actionnable par un agent ;
- testable ou vérifiable ;
- explicite sur les non-objectifs ;
- alignée sur sécurité > qualité > performance > complétude.
