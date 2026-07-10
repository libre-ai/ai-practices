---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ../04-content-factory/prompt-library.md
  - ../04-content-factory/review-process.md
  - ../04-content-factory/quality-gates.md
  - ../model-delegation.md
---

# Revue multi-agent assistée

## Objectif

Définir une architecture de revue assistée par plusieurs rôles IA, sans déléguer
la décision finale.

## Rôles d'agents

| Agent | Vérifie | Peut approuver |
| --- | --- | --- |
| `content-review-agent` | clarté, contexte, feedback | non |
| `security-rgpd-review-agent` | PII, secrets, conformité | non |
| `bias-review-agent` | stéréotypes, médias | non |
| `source-analysis-agent` | portée des sources | non |
| `quality-gate-agent` | gates structurés | non |
| `pedagogy-agent` futur | niveau, transfert | non |

## Flux

```text
contenu draft/review
  -> agents spécialisés en parallèle
  -> rapport agrégé
  -> conflits visibles
  -> revue humaine
  -> décision mainteneur corpus
```

## Rapport agrégé

```yaml
artifact_id: q-...
overall_proposed: pass|warn|fail|blocker
agent_findings:
  - agent: security-rgpd-review-agent
    level: blocker
    evidence: ...
conflicts:
  - content-agent pass mais security-agent blocker
human_reviews_required:
  - security-rgpd
final_decision: human_only
```

## Gestion des conflits

- Le finding le plus sévère remonte.
- Un blocker sécurité/RGPD/biais/RH bloque par défaut.
- Les désaccords sont conservés.
- Le mainteneur humain tranche ou demande réécriture.

## Sécurité des agents

- Ne pas envoyer PII/secrets à un service externe non autorisé.
- Utiliser corpus de revue minimal.
- Masquer données si nécessaire.
- Journaliser sans contenu sensible.
- Indiquer modèle/version si utile à la traçabilité.

## Limites

- Les agents peuvent halluciner.
- Les agents peuvent rater un biais ou risque métier.
- Les agents ne remplacent pas DPO, sécurité, juridique ou métier.
- L'absence de finding n'est pas preuve de qualité.

## Critères d'acceptation

- Les agents ne peuvent pas approuver.
- Les conflits sont visibles.
- Les incertitudes sont conservées.
- Aucune donnée sensible non nécessaire.
- Les rapports servent la revue humaine.
