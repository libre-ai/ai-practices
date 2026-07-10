---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ../05-product-learning/learning-paths.md
  - ../05-product-learning/analytics-privacy.md
  - ../01-foundation/contribution-policy.md
---

# Benchmark formation entreprise

## Objectif

Analyser les pratiques de formation en entreprise applicables au produit.

## Formats comparés

| Format | Force | Limite |
| --- | --- | --- |
| microlearning | facile à intégrer | risque superficialité |
| atelier | discussion et contexte | coûteux à scaler |
| e-learning | diffusion large | passivité |
| simulation | transfert | production coûteuse |
| campagne sécurité | répétition | fatigue utilisateur |
| communauté de pratique | ancrage métier | demande animation |
| fiche réflexe | action immédiate | ne mesure pas transfert seule |

## Bonnes pratiques à retenir

- modules courts ;
- cas métier ;
- relances espacées ;
- feedback concret ;
- adaptation par rôle sans profilage ;
- mesure agrégée ;
- transparence sur finalité ;
- gouvernance de contenu.

## Limites à éviter

- conformité de façade ;
- obligation annuelle sans remédiation ;
- score transmis au manager ;
- contenu obsolète ;
- absence de source ;
- formation identique pour tous les risques ;
- collecte excessive.

## Impacts pour `rumble-ai-practices`

- proposer parcours courts par axe ;
- permettre replay et remédiation ;
- fournir exports de corpus si besoin ;
- garder admin contenu via git/CLI au début ;
- produire rapports agrégés, pas nominaux ;
- prévoir revue périodique.

## Grille de comparaison future

```yaml
program: ...
format: microlearning|workshop|simulation|course
frequency: ...
measurement: none|individual|aggregated
privacy_risk: low|medium|high
content_governance: unknown|documented|audited
transfer_support: low|medium|high
ideas_for_rumble:
  - ...
```

## Critères d'acceptation

- Les recommandations sont compatibles avec privacy/anonymat.
- La formation ne devient pas évaluation disciplinaire.
- Le corpus reste exportable.
- Les formats retenus renforcent transfert et maintenabilité.
