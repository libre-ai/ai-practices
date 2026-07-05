---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ../security-rgpd.md
  - ../data-model.md
  - ../content-governance.md
  - ../02-research/cybersecurity.md
  - ../02-research/governance-regulation.md
  - ./taxonomy.md
---

# Modèle de risques

## Objectif

Structurer les risques que les questions doivent faire reconnaître, éviter ou
mitiger.

Le modèle de risques sert à :

- écrire des scénarios ;
- scorer des choix ;
- produire des feedbacks ;
- déclencher des quality gates ;
- alimenter les synthèses privées et agrégats anonymes.

## Axes existants à préserver

| Axe | Rôle |
| --- | --- |
| `data_confidentiality` | données internes, secrets, documents sensibles |
| `source_verification` | sources, citations, claims |
| `hallucination` | sorties plausibles mais non fidèles |
| `bias_fairness` | biais, équité, représentations |
| `security` | attaques, secrets, agents, outils |
| `privacy_rgpd` | données personnelles, minimisation, droits |
| `human_responsibility` | supervision, revue, décision à impact |
| `prompt_practice` | cadrage de tâche, contexte sûr, limites prompt |
| `business_context` | politique interne, métier, proportionnalité |
| `media_synthetic` | médias IA, provenance, deepfake, consentement |

## Modèle canonique de risque

```yaml
id: risk-hallucinated-source
axis: source_verification
family: reliability
severity_default: high
likelihood_default: medium
description: Une source est citée ou attribuée alors qu'elle n'existe pas ou ne soutient pas l'affirmation.
indicators:
  - référence invérifiable
  - URL sans rapport
  - citation trop précise sans trace
triggers:
  - generated_citation
  - legal_or_scientific_claim
mitigations:
  - vérifier la source primaire
  - signaler l'incertitude
  - ne pas publier si non vérifié
blockers:
  - claim juridique publié avec source inventée
related_concepts:
  - concept-hallucinated-source
  - concept-source-verification
competencies:
  - comp-check-source
review_period_months: 6
```

## Sévérité pédagogique

| Sévérité | Définition | Effet produit |
| --- | --- | --- |
| `low` | erreur de méthode sans exposition significative | feedback simple |
| `medium` | risque réel mais récupérable ou limité | feedback + remédiation |
| `high` | exposition de donnée, décision à impact ou sécurité sérieuse | pénalité forte + action claire |
| `blocker` | contenu encourageant une pratique dangereuse ou illégitime | publication interdite |

## Probabilité indicative

| Niveau | Définition |
| --- | --- |
| `rare` | cas exceptionnel ou très contrôlé |
| `possible` | plausible selon contexte |
| `likely` | fréquent dans usages IA professionnels |
| `systemic` | inhérent au workflow ou à la gouvernance |

La probabilité ne doit pas donner une fausse précision. Elle aide à prioriser la
pédagogie.

## Familles de risques

| Famille | Exemples | Réflexe attendu |
| --- | --- | --- |
| Données | PII, secrets, documents clients, logs | minimiser, anonymiser, choisir outil autorisé |
| Fiabilité | hallucination, source inventée, contexte obsolète | vérifier, citer source primaire, signaler incertitude |
| Sécurité | prompt injection, exfiltration, agent trop permissif | isoler, limiter permissions, valider effets |
| Biais | stéréotype, représentation inéquitable, proxy discriminant | questionner données et impact |
| Responsabilité | décision sensible automatisée, absence de revue | maintenir supervision humaine |
| Conformité | RGPD, AI Act, politique interne | appliquer règle plus restrictive et documenter |
| Média | deepfake, image synthétique, provenance absente | auditer, signaler, éviter humains réalistes non nécessaires |
| Organisation | usage RH implicite, outil non autorisé | clarifier finalité, agrégats, politique interne |

## Risques canoniques initiaux

| ID | Axe | Sévérité | Description courte |
| --- | --- | --- | --- |
| `risk-pii-leak` | `privacy_rgpd` | high | donnée personnelle exposée à un outil non autorisé |
| `risk-secret-leakage` | `security` | high | token, clé, log ou secret métier exposé |
| `risk-confidential-document-leak` | `data_confidentiality` | high | document interne/client envoyé sans contrôle |
| `risk-hallucinated-source` | `source_verification` | high | source inventée ou non vérifiable |
| `risk-misattribution` | `source_verification` | medium/high | source réelle qui ne soutient pas le claim |
| `risk-obsolete-context` | `hallucination` | medium | réponse fondée sur information périmée |
| `risk-critical-omission` | `hallucination` | high | information essentielle absente du résumé |
| `risk-prompt-injection` | `security` | high | instruction de détournement directe |
| `risk-indirect-prompt-injection` | `security` | high | instruction malveillante dans document/source |
| `risk-excessive-agent-permission` | `security` | high | agent avec droits trop larges |
| `risk-unsafe-action-automation` | `human_responsibility` | high | action externe sans validation |
| `risk-bias-stereotype` | `bias_fairness` | medium/high | représentation stéréotypée |
| `risk-proxy-discrimination` | `bias_fairness` | high | variable neutre corrélée à groupe protégé |
| `risk-synthetic-human-media` | `media_synthetic` | high | visage/personne IA sans justification/audit |
| `risk-rh-scoring` | `business_context` | blocker | score utilisé comme évaluation RH nominative |
| `risk-vendor-overtrust` | `business_context` | medium | conformité déduite d'un claim fournisseur |
| `risk-source-gap-approved` | `source_verification` | blocker | contenu approuvé avec source manquante |

## Blockers éditoriaux

Un contenu doit être `blocked` si :

- il recommande d'exposer PII, secrets, logs ou code sensible ;
- il présente une source inventée comme valide ;
- il utilise un score à finalité RH nominative ;
- il publie un média humain IA non audité ;
- il automatise une décision à impact sans supervision ;
- il affirme une obligation juridique non sourcée ;
- il transforme une politique interne inconnue en règle générale ;
- il rend la bonne réponse indéterminable.

## Mapping risque -> quality gates

| Risque | Gate automatique | Gate humain |
| --- | --- | --- |
| `risk-source-gap-approved` | refuser `approved` + `source_gap` | revue source |
| `risk-rh-scoring` | détecter leaderboard/export nominatif | revue produit/RGPD |
| `risk-synthetic-human-media` | média synthetic_human sans review | revue biais/média |
| `risk-pii-leak` | mots/fixtures sensibles simples | revue sécurité/RGPD |
| `risk-hallucinated-source` | source manquante/mal formée | revue claim/source |
| `risk-excessive-agent-permission` | tags agent + action externe | revue sécurité |

## Scoring des choix

Un choix peut avoir plusieurs impacts :

```yaml
choice_id: send_document_directly
score: -1.0
risk_impacts:
  - risk_id: risk-confidential-document-leak
    severity: high
    axis: data_confidentiality
  - risk_id: risk-pii-leak
    severity: medium
    axis: privacy_rgpd
feedback_pattern: expliquer le contenu réel, métadonnées, politique interne
```

Les scores négatifs signalent un risque pédagogique, pas une faute RH.

## Synthèse utilisateur

La synthèse doit :

- regrouper par axe ;
- proposer pratiques recommandées ;
- éviter un score global ;
- rester privée ;
- afficher les limites de mesure.

## Agrégats organisationnels

Les agrégats doivent :

- respecter k-anonymat ;
- ne pas afficher rang individuel ;
- être par axe ou module ;
- éviter les faibles effectifs ;
- respecter rétention.

## Critères d'acceptation

- Chaque risque est relié à un axe, une sévérité et une mitigation.
- Les risques de PII/secrets sont toujours traités comme prioritaires.
- Le modèle ne crée pas de score RH.
- Les risques alimentent directement les quality gates.
- Les blockers sont fail-closed.
