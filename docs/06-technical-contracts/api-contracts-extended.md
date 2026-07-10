---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ../api-contracts.md
  - ../data-model.md
  - ../security-rgpd.md
  - ../adrs/0006-anonymity-and-retention-cohort.md
  - ./data-model-extended.md
---

# Contrats API étendus

## Objectif

Préparer les endpoints nécessaires au mode backend sans compromettre anonymat et
séparation métier/UI.

## Principes API

- Sessions anonymes par défaut.
- Pas de texte libre utilisateur en MVP.
- Scoring côté serveur si backend activé.
- Corpus servi en `approved` uniquement.
- Distribution indisponible sous seuil k.
- Contrats testables localement.

## Endpoints cible

| Endpoint | Usage | Données sensibles |
| --- | --- | --- |
| `GET /healthz` | santé runtime | non |
| `GET /readyz` | readiness dépendances | non |
| `GET /v1/catalog` | parcours publiés | non |
| `GET /v1/questions/{id}` | question approuvée | non |
| `POST /v1/sessions` | créer session | éviter PII |
| `GET /v1/sessions/{id}/next` | prochaine question | session token |
| `POST /v1/sessions/{id}/answers` | soumettre choix | choix, pas texte libre |
| `GET /v1/sessions/{id}/summary` | synthèse privée | session token |
| `GET /v1/cohorts/{id}/distribution` | distribution k-anonyme | sous seuil = 404/204 |
| `POST /v1/rum` | RUM anonyme | zéro PII |
| `GET /v1/admin/audit-report` | audit corpus | admin, pas apprenant nominatif |

## Enveloppe d'erreur

```json
{
  "error": {
    "code": "cohort_threshold_not_met",
    "message": "Distribution unavailable for this cohort",
    "details": []
  },
  "meta": {
    "request_id": "...",
    "version": "v1"
  }
}
```

## Codes d'erreur spécifiques

| Code | Sens |
| --- | --- |
| `validation_failed` | payload invalide |
| `session_not_found` | session inconnue/expirée |
| `question_not_available` | question non approved/retired |
| `cohort_threshold_not_met` | k-anonymat non atteint |
| `rate_limited` | protection abus |
| `content_not_approved` | tentative de servir contenu non publiable |

## Contraintes privacy

- session_id aléatoire non dérivable ;
- pas de nom/email en mode anonyme ;
- pas d'IP dans réponse ou rapport applicatif ;
- cookies stricts si web auth ;
- logs sans payload sensible ;
- purge selon rétention.

## Fixtures attendues

- création session anonyme ;
- soumission réponse ;
- synthèse sans distribution ;
- distribution sous seuil refusée ;
- question retired non servie ;
- audit report corpus.

## Critères d'acceptation

- Aucune PII nécessaire.
- Scoring côté serveur si backend activé.
- Sous seuil k, distribution indisponible.
- Contrats testables localement.
- Les contrats ne créent pas d'API LMS générique.
