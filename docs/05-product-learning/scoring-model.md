---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ../scoring.md
  - ../question-model.md
  - ../adrs/0003-content-governance-and-no-rh-scoring.md
  - ../adrs/0006-anonymity-and-retention-cohort.md
  - ../03-domain-model/risk-model.md
---

# Modèle de scoring pédagogique

## Objectif

Étendre le scoring existant en gardant sa finalité pédagogique et non RH.

## Ce que le score mesure

Le score mesure des choix dans un contexte donné : protection des données,
vérification, prudence, escalade, responsabilité. Il ne mesure pas la valeur
professionnelle d'une personne.

## Unités de calcul

| Unité | Rôle |
| --- | --- |
| `choice_score` | valeur pédagogique d'un choix |
| `risk_impact` | risque créé/mitigé |
| `axis_level` | synthèse par axe |
| `competency_signal` | signal faible/fort par compétence |
| `feedback_card` | remédiation actionnable |

## Pondération recommandée

| Score | Sens |
| --- | --- |
| `+1.0` | bonne pratique essentielle |
| `+0.5` | bonne pratique utile mais insuffisante seule |
| `0.0` | dépend du contexte ou neutre |
| `-0.5` | erreur mineure |
| `-1.0` | erreur risquée |
| `-2.0` | erreur critique : PII, secret, décision sensible, média non signalé |

## Niveaux par axe

| Niveau | Description utilisateur |
| --- | --- |
| `discovery` | Vous identifiez certains risques mais manquez encore de méthode. |
| `guided_practice` | Vous appliquez les bons réflexes dans les cas simples. |
| `careful_autonomy` | Vous contextualisez et savez vérifier/refuser si nécessaire. |
| `reference` | Vous raisonnez par risques, sources, gouvernance et transmission. |

## Règles no-RH

- Pas de note globale publique.
- Pas de leaderboard nominatif.
- Pas de rang individuel exact.
- Pas de seuil automatique apte/inapte.
- Pas d'export manager individuel.
- Comparaison privée seulement si cohorte anonyme suffisante.

## Cas partiels

Une réponse partielle doit être reconnue :

- bonne intuition ;
- condition manquante ;
- action complémentaire ;
- score intermédiaire ;
- feedback spécifique.

Exemple : demander une source = partiel ; vérifier source primaire = correct.

## Erreurs critiques

Une erreur critique doit déclencher :

- feedback clair ;
- fiche réflexe ;
- recommandation de remédiation ;
- jamais humiliation ;
- jamais transmission nominative.

## Synthèse utilisateur

La synthèse affiche :

- axes forts ;
- axes à renforcer ;
- pratiques recommandées ;
- prochains modules ;
- rappel confidentialité ;
- distribution privée seulement si disponible.

## Agrégats organisationnels

Uniquement si activés :

- par cohorte suffisamment grande ;
- par axe ou module ;
- sans rang ;
- sans export nominatif ;
- avec rétention documentée.

## Critères d'acceptation

- La finalité pédagogique est explicite.
- Les agrégats respectent k-anonymat.
- Les cas partiels sont représentables.
- Le modèle reste compatible avec `docs/scoring.md`.
