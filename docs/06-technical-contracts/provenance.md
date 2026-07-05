---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ../content-governance.md
  - ../01-foundation/source-policy.md
  - ../04-content-factory/review-process.md
  - ./data-model-extended.md
---

# Provenance et traçabilité

## Objectif

Définir comment tracer origine, source, génération, revue et modification des
contenus.

## Principes

- Chaque contenu publié doit expliquer d'où il vient.
- Une source soutient un claim précis.
- Une assistance IA doit être déclarable.
- Git reste la preuve de versionnement minimale.
- La traçabilité ne doit pas collecter inutilement de PII.

## Modèle minimal

```yaml
provenance:
  created_by: author-id
  created_at: YYYY-MM-DD
  assisted_by_ai: true
  ai_assistance:
    purpose: draft|rewrite|review|risk_scan
    model_ref: optional-local-or-provider-id
    sensitive_data_sent: false
  source_inputs:
    - source-nist-ai-rmf-1-0
  derived_from: []
  review_events:
    - review-...
  git:
    commit: optional
```

## ReviewEvent

```yaml
id: review-q-source-001-security-2026-07-05
artifact_type: question
artifact_id: q-source-001
reviewer_role: security-rgpd
decision: pass|warn|fail|blocker
findings:
  - level: warn
    message: Source interne à préciser.
created_at: 2026-07-05
evidence:
  report_path: reports/...
```

## Événements à tracer

- création ;
- modification sémantique ;
- ajout/retrait source ;
- changement de statut ;
- revue humaine ;
- revue assistée ;
- blocage ;
- retrait ;
- publication.

## Données à minimiser

- identifiants reviewers pseudonymisés si nécessaire ;
- pas de notes personnelles inutiles ;
- pas de prompts contenant secrets ;
- pas de données apprenant dans provenance corpus.

## Audit d'un contenu publié

Un auditeur doit pouvoir répondre :

1. Quel claim est enseigné ?
2. Quelle source le soutient ?
3. Quel risque/compétence est ciblé ?
4. Qui a revu ?
5. Quand ?
6. Quels blockers ont été levés ?
7. Quelle version est servie ?

## Critères d'acceptation

- Chaque contenu publié a une chaîne de revue.
- Les sources sont reliées aux claims.
- Les données personnelles de reviewers sont minimisées si nécessaire.
- L'assistance IA ne devient pas une autorité de publication.
