---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ../security-rgpd.md
  - ../data-model.md
  - ../adrs/0006-anonymity-and-retention-cohort.md
  - ./scoring-model.md
---

# Analytics et vie privée

## Objectif

Définir les mesures utiles au produit sans violer minimisation, anonymat et
confiance utilisateur.

## Principes

- Mesurer pour améliorer le produit et les parcours, pas surveiller.
- Collecter le minimum.
- Zéro PII en logs.
- Agréger sous seuil k.
- Rétention courte.
- Transparence utilisateur.

## Événements autorisés MVP

| Événement | Données | Finalité |
| --- | --- | --- |
| `session_started` | session_id anonyme, track, locale | fonctionnement parcours |
| `question_shown` | question_id, axis | progression |
| `answer_submitted` | question_id, choice_ids, score_delta | feedback et synthèse |
| `feedback_shown` | question_id, feedback_level | amélioration UX |
| `session_completed` | axes agrégés, count | synthèse |
| `rum_error` | route, code erreur, user_agent tronqué si utile | fiabilité technique |

## Données interdites

- nom/prénom ;
- email en mode anonyme ;
- IP stockée dans rapports applicatifs ;
- texte libre utilisateur ;
- prompts ou documents fournis ;
- secrets/logs/code ;
- rang individuel ;
- identifiant RH ;
- fingerprinting.

## Agrégation

Règles :

- distribution seulement si `n >= k` ;
- pas de petits groupes réidentifiants ;
- buckets larges ;
- pas de combinaison fine axe + équipe + période si effectif faible ;
- suppression ou regroupement sous seuil.

## Rétention recommandée

| Donnée | Rétention |
| --- | --- |
| session locale anonyme | supprimable immédiatement |
| meilleur score local | côté client, effaçable |
| résultat individuel authentifié | 30 jours par défaut si activé |
| agrégats anonymisés | 12 mois maximum |
| logs techniques | 30 jours maximum |
| rapports corpus | indéfini, sans PII apprenant |

## Notice utilisateur minimale

Afficher avant session :

- finalité pédagogique ;
- pas une évaluation RH ;
- données collectées ;
- mode local/anonyme si applicable ;
- règles de comparaison de cohorte ;
- droit de quitter/effacer localement.

## Tests attendus

- aucun événement ne contient PII ;
- sous seuil k, distribution indisponible ;
- logs masquent IP/identifiants ;
- rétention documentée ;
- mode offline sans collecte serveur.

## Critères d'acceptation

- Aucune donnée personnelle non nécessaire.
- Logs sans PII.
- Sous seuil k, pas de distribution.
- Rétention documentée.
- Les analytics ne créent pas de finalité RH implicite.
