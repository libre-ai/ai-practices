---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ../scoring.md
  - ../question-model.md
  - ../02-research/ai-literacy.md
  - ../02-research/cognitive-science.md
  - ./risk-model.md
  - ./taxonomy.md
---

# Modèle de compétences

## Objectif

Définir les compétences professionnelles observables que le produit entraîne.

Une compétence doit être visible dans une décision ou une justification. Elle ne
se limite pas à savoir réciter une définition.

## Principes

- Pas de certification RH individuelle.
- Pas de score global humiliant.
- Une compétence est évaluée par comportements observables.
- Les niveaux décrivent une aide pédagogique, pas un grade employé.
- Une compétence se relie à des concepts, risques et objectifs pédagogiques.

## Dimensions

| Dimension | Intention | Exemples |
| --- | --- | --- |
| `understand` | comprendre capacités/limites | sortie IA ≠ preuve, contexte, variabilité |
| `protect` | protéger données et accès | PII, secrets, minimisation, outil autorisé |
| `verify` | vérifier sources et sorties | source primaire, hallucination, citation |
| `frame` | cadrer une tâche IA | objectif, contraintes, contexte sûr |
| `judge` | arbitrer selon risque | proportionnalité, impact, politique interne |
| `secure` | reconnaître menaces IA | prompt injection, agents, tool permissions |
| `bias` | détecter biais/représentations | stéréotype, proxy, média synthétique |
| `account` | maintenir responsabilité | revue humaine, documentation, escalade |

## Niveaux

| Niveau | Description | Indice observable |
| --- | --- | --- |
| `awareness` | reconnaît le risque ou la notion | identifie qu'il y a un problème |
| `guided` | applique une règle dans un scénario simple | choisit l'action recommandée |
| `autonomous` | arbitre dans un contexte ambigu | justifie conditions, limites, escalade |
| `mentor` | explique et aide autrui | formule une règle contextualisée et transférable |

## Compétences canoniques

| ID | Dimension | Compétence | Comportement observable |
| --- | --- | --- | --- |
| `comp-understand-ai-output` | understand | Comprendre le statut d'une sortie IA | traite la sortie comme proposition à vérifier |
| `comp-protect-data` | protect | Protéger données et secrets | refuse/anonymise avant usage non autorisé |
| `comp-classify-data-context` | protect | Identifier la sensibilité d'un contexte | repère PII, secret, document interne, logs |
| `comp-check-source` | verify | Vérifier une affirmation | consulte source primaire ou signale incertitude |
| `comp-detect-hallucination` | verify | Identifier plausibilité trompeuse | ne réutilise pas une sortie non contrôlée |
| `comp-frame-task` | frame | Cadrer une demande IA | donne objectif/contraintes sans sur-exposer |
| `comp-assess-risk` | judge | Évaluer le risque d'usage | adapte outil, données et supervision au niveau d'impact |
| `comp-apply-policy` | judge | Appliquer politique interne | reconnaît quand règle organisationnelle prime |
| `comp-secure-agent-tools` | secure | Encadrer agents/outils | limite permissions et vérifie effets avant action |
| `comp-handle-prompt-injection` | secure | Réagir à une injection | traite document externe comme donnée non instruction |
| `comp-handle-bias` | bias | Reconnaître biais et représentations | questionne données, exemples et impacts sur groupes |
| `comp-review-synthetic-media` | bias | Évaluer média synthétique | vérifie provenance, droits, biais, consentement |
| `comp-human-accountability` | account | Maintenir responsabilité humaine | ne délègue pas décision sensible à l'IA seule |
| `comp-document-decision` | account | Documenter un arbitrage | trace hypothèses, sources, limites et revue |
| `comp-escalate` | account | Escalader au bon rôle | sollicite sécurité/DPO/juridique/métier si nécessaire |

## Modèle canonique

```yaml
id: comp-check-source
label: Vérifier une affirmation IA
dimension: verify
summary: Contrôler qu'une affirmation importante est soutenue par une source pertinente.
levels:
  awareness:
    observable_behaviors:
      - reconnaît qu'une citation IA peut être fausse
  guided:
    observable_behaviors:
      - remonte à une source primaire
      - signale l'incertitude si la source manque
  autonomous:
    observable_behaviors:
      - arbitre selon enjeu, date, source et politique interne
  mentor:
    observable_behaviors:
      - explique à autrui comment vérifier sans se fier au style de réponse
risk_axes:
  - source_verification
  - hallucination
concepts:
  - concept-source-verification
  - concept-hallucinated-source
  - concept-primary-source
assessment_methods:
  - scenario_choice
  - explanation_review
```

## Méthodes d'observation

| Méthode | Usage | Limite |
| --- | --- | --- |
| `scenario_choice` | QCM ou choix multiples | peut masquer raisonnement |
| `ranking` | ordonner actions par prudence | demande feedback clair |
| `branching_scenario` | arbitrage multi-étapes | plus coûteux à produire |
| `explanation_review` | justification écrite | risque PII si libre, à encadrer |
| `simulation` | agent/RAG/workflow | nécessite garde-fous forts |

Le MVP privilégie `scenario_choice` et feedbacks structurés, sans réponses libres
sensibles par défaut.

## Mapping compétences -> risques

| Compétence | Axes principaux |
| --- | --- |
| `comp-protect-data` | `data_confidentiality`, `privacy_rgpd`, `security` |
| `comp-check-source` | `source_verification`, `hallucination` |
| `comp-frame-task` | `prompt_practice`, `data_confidentiality` |
| `comp-assess-risk` | `business_context`, `human_responsibility` |
| `comp-secure-agent-tools` | `security`, `human_responsibility` |
| `comp-handle-bias` | `bias_fairness`, `media_synthetic` |
| `comp-human-accountability` | `human_responsibility`, `governance_compliance` |
| `comp-document-decision` | `business_context`, `source_verification` |

## Scoring pédagogique

Le scoring doit produire :

- signaux par compétence ;
- axes à renforcer ;
- recommandations de remédiation ;
- synthèse privée pour l'utilisateur ;
- agrégats anonymes côté organisation si activés.

Le scoring ne doit pas produire :

- classement nominatif ;
- note RH ;
- certification légale ;
- export individuel au manager.

## Critères d'acceptation

- Chaque compétence correspond à un comportement observable.
- Chaque compétence est reliée à des concepts et risques.
- Le scoring reste pédagogique.
- Les niveaux ne créent pas de certification RH implicite.
- Les méthodes d'observation respectent la minimisation des données.
