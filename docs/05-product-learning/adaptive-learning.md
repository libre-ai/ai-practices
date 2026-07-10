---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ./analytics-privacy.md
  - ./scoring-model.md
  - ./learning-paths.md
  - ../adrs/0006-anonymity-and-retention-cohort.md
---

# Apprentissage adaptatif

## Objectif

Définir une adaptation pédagogique respectueuse de la vie privée et fondée sur
les axes de progression.

## Principe

L'adaptation doit être simple, explicable et locale autant que possible. Elle ne
doit pas devenir un profilage opaque.

## Signaux autorisés

| Signal | Usage | Stockage |
| --- | --- | --- |
| axe de risque faible/fort | recommander remédiation | local ou agrégé |
| niveau de question réussi | proposer progression | local |
| choix `blocker` | afficher fiche réflexe | local/session |
| misconception répétée | proposer drill | local |
| parcours terminé | prochaine étape | local |

## Signaux interdits

- temps de réponse comme performance ;
- comparaison nominative ;
- texte libre sensible ;
- données RH ;
- inférence de personnalité ;
- fingerprinting ou tracking marketing.

## Règles adaptatives initiales

```yaml
rules:
  - id: remediate-source-verification
    if: axis.source_verification.level in [risky, weak]
    then: recommend_path: sources-and-reliability
    explanation: Plusieurs réponses montrent un besoin de vérifier les sources citées.

  - id: blocker-secret-leakage
    if: selected_choice_has_risk: risk-secret-leakage
    then: show_reflex_card: protect-secrets-before-ai
    explanation: Un secret ou log sensible ne doit pas être envoyé à un outil non autorisé.
```

## Explicabilité utilisateur

Chaque recommandation doit répondre à :

- pourquoi cette recommandation apparaît ;
- quelles données ont été utilisées ;
- comment l'ignorer ou revenir au parcours libre ;
- si l'information reste locale ou agrégée.

## Mode offline/local

Le mode offline doit pouvoir :

- calculer une synthèse privée ;
- recommander des fiches locales ;
- stocker le meilleur score local ;
- fonctionner sans backend.

## Tests attendus

- une erreur par axe recommande le bon parcours ;
- un choix blocker déclenche une fiche prioritaire ;
- sous seuil k, aucune distribution organisationnelle ;
- aucune donnée nominative nécessaire ;
- le mode offline garde un comportement utile.

## Critères d'acceptation

- L'adaptation est explicable.
- Les données sont minimisées.
- Le mode offline/local reste possible.
- Aucune règle ne produit de décision RH.
