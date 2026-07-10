---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ../benchmark-jeu-competitif.md
  - ../adrs/0003-content-governance-and-no-rh-scoring.md
  - ../02-research/cognitive-science.md
  - ../05-product-learning/pedagogy.md
---

# Benchmark serious games

## Objectif

Analyser les mécaniques de serious games utiles à l'apprentissage sans importer
la compétition toxique.

## Mécaniques compatibles

| Mécanique | Pourquoi utile | Garde-fou |
| --- | --- | --- |
| scénario contextualisé | transfert métier | pas de cas réel identifiable |
| feedback immédiat | apprentissage | toujours actionnable |
| embranchements | arbitrage | complexité progressive |
| badges privés | encouragement | pas de classement |
| replay libre | retrieval practice | pas de pénalité régression |
| fiches débloquées | remédiation | pas de rationnement |
| progression par axes | clarté | pas de note RH |

## Mécaniques rejetées

- leaderboard nominatif ;
- chrono noté ;
- vies/game over ;
- tickets d'entraînement ;
- ligues promotion/relégation ;
- prix individuels ;
- multiplicateurs de score ;
- streak culpabilisant.

## Critères d'analyse

- apprentissage ou simple engagement ;
- respect de l'utilisateur ;
- transfert vers situation professionnelle ;
- accessibilité ;
- données collectées ;
- risque d'usage RH ;
- maintenabilité du contenu.

## Recommandations pour le produit

- privilégier entraînement libre ;
- rendre les erreurs sûres ;
- valoriser amélioration privée ;
- afficher remédiations plutôt que sanctions ;
- utiliser badges seulement comme repères privés ;
- refuser vitesse comme métrique.

## Critères d'acceptation

- Les mécaniques rejetées sont justifiées.
- Les recommandations respectent ADR 0003.
- Le transfert métier reste central.
- Aucune mécanique ne crée pression RH implicite.
