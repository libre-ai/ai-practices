---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ../question-model.md
  - ../data-model.md
  - ../01-foundation/glossary.md
  - ../01-foundation/source-policy.md
  - ../02-research/state-of-the-art.md
  - ./competency-model.md
  - ./risk-model.md
---

# Taxonomie

## Objectif

Définir l'organisation des concepts enseignés par `rumble-ai-practices` avant de
produire un volume important de contenus. La taxonomie sert à relier concepts,
risques, compétences, sources, objectifs pédagogiques et questions.

Elle ne contient pas encore les 250 concepts finaux. Elle stabilise le modèle,
les domaines, les relations et une liste de concepts semences à partir desquels
le corpus pourra être étendu.

## Principes

- Un concept doit être utile à une décision professionnelle.
- Un concept doit se relier à au moins un risque ou une compétence.
- Un concept publié doit avoir une source ou une justification interne.
- Les synonymes et confusions dangereuses doivent être visibles.
- Les concepts purement techniques ne sont gardés que s'ils expliquent un
  réflexe utilisateur.
- La taxonomie doit rester convertible en YAML/JSON.

## Modèle canonique de concept

```yaml
id: concept-source-verification
label: Vérification des sources
status: draft # draft | review | approved | retired
summary: Contrôler qu'une affirmation ou citation IA est soutenue par une source pertinente.
domain: reliability
subdomain: source_verification
level: foundational # foundational | applied | advanced | governance
aliases:
  - contrôle des sources
not_to_confuse_with:
  - demander une source
  - citation affichée
prerequisites:
  - concept-ai-output-not-proof
related:
  - concept-hallucinated-source
  - concept-groundedness
relations:
  - type: mitigates
    target: risk-hallucinated-source
  - type: supports_competency
    target: comp-check-source
risk_axes:
  - source_verification
competencies:
  - comp-check-source
claims:
  - claim-ai-output-needs-verification
sources:
  - source-nist-ai-rmf-1-0
review:
  confidence: medium
  reviewers: []
  last_reviewed_at: null
```

## Convention d'identifiants

| Objet | Format | Exemple |
| --- | --- | --- |
| Concept | `concept-{slug}` | `concept-prompt-injection-indirecte` |
| Risque | `risk-{slug}` | `risk-secret-leakage` |
| Compétence | `comp-{verb}-{object}` | `comp-check-source` |
| Claim | `claim-{slug}` | `claim-rag-does-not-guarantee-truth` |
| Source | `source-{publisher}-{slug}-{year}` | `source-nist-ai-rmf-2023` |

Les identifiants sont stables. Un renommage de label ne change pas l'ID.

## Niveaux de concept

| Niveau | Usage | Exemple |
| --- | --- | --- |
| `foundational` | notion nécessaire à beaucoup de scénarios | donnée personnelle, source primaire |
| `applied` | notion utilisée dans une décision pratique | minimisation, vérification de citation |
| `advanced` | notion plus technique ou multi-risques | prompt injection indirecte, tool calling |
| `governance` | notion liée à politique, conformité, audit | rétention, revue humaine, DPIA |

## Domaines de premier niveau

| Domaine | Code | Rôle pédagogique |
| --- | --- | --- |
| Fondamentaux IA générative | `genai_foundations` | comprendre ce qu'une sortie IA est et n'est pas |
| Données, confidentialité et secrets | `data_confidentiality` | éviter exposition PII/secrets/documents sensibles |
| Sources, factualité et vérification | `source_reliability` | vérifier avant réutilisation |
| Hallucinations et incertitude | `hallucination_uncertainty` | reconnaître plausibilité trompeuse |
| Prompting responsable | `prompt_practice` | cadrer sans sur-exposer ni dogmatiser |
| RAG et connaissance interne | `rag_knowledge` | comprendre récupération, citations et limites |
| Agents, outils et automatisation | `agents_tools` | encadrer permissions et actions |
| Cybersécurité IA | `ai_security` | reconnaître menaces LLM/RAG/agents |
| Biais, équité et représentations | `bias_fairness` | réduire stéréotypes et impacts injustes |
| Responsabilité humaine | `human_responsibility` | maintenir supervision et décision humaine |
| RGPD, gouvernance et conformité | `governance_compliance` | appliquer politiques et cadres de risque |
| Médias synthétiques et provenance | `synthetic_media` | contrôler origine, droits, tromperie |
| Usage professionnel et organisation | `business_context` | intégrer contexte métier, limites et adoption |

## Relations autorisées

| Relation | Sens | Exemple |
| --- | --- | --- |
| `requires` | prérequis conceptuel | `concept-rag` requires `concept-retrieval` |
| `clarifies` | précise ou désambiguïse | pseudonymisation clarifies anonymisation |
| `contrasts_with` | distingue deux notions proches | source citée vs source vérifiée |
| `mitigates` | réduit un risque | minimisation mitigates PII leak |
| `creates_risk` | introduit ou augmente un risque | tool calling creates excessive agency |
| `supports_competency` | contribue à une compétence | source primaire supports check-source |
| `evidenced_by` | relié à une source | claim evidenced_by source |
| `assessed_by` | mesurable par contenu | compétence assessed_by question |
| `remediated_by` | recommandé en remédiation | risk remediated_by learning path |
| `blocked_by` | empêche publication si non résolu | concept with source_gap blocked_by source-policy |

## Concepts semences par domaine

### `genai_foundations`

- `concept-ai-output-not-proof` — une sortie IA est une proposition, pas une
  preuve.
- `concept-generative-ai` — production de texte, code, média ou structure.
- `concept-llm` — modèle de langage génératif.
- `concept-token` — unité de traitement du texte.
- `concept-context-window` — limite de contexte disponible.
- `concept-sampling-variability` — variabilité probabiliste des sorties.
- `concept-model-versioning` — capacités variables selon version.
- `concept-vendor-claim` — affirmation fournisseur à contextualiser.

### `data_confidentiality`

- `concept-personal-data` — donnée personnelle au sens large.
- `concept-sensitive-data` — donnée sensible ou à risque élevé.
- `concept-business-secret` — information confidentielle métier.
- `concept-technical-secret` — token, clé, credential, secret technique.
- `concept-data-classification` — niveau public/interne/confidentiel/restreint.
- `concept-data-minimization` — réduction au nécessaire.
- `concept-pseudonymization` — réduction d'identifiants directs, non anonymat.
- `concept-anonymization` — transformation empêchant raisonnablement la
  réidentification.

### `source_reliability`

- `concept-primary-source` — source originale ou autorité de référence.
- `concept-secondary-source` — synthèse ou commentaire.
- `concept-source-verification` — vérification claim/source.
- `concept-hallucinated-source` — source inventée.
- `concept-misattribution` — source réelle mais mal utilisée.
- `concept-citation-not-proof` — citation affichée ≠ preuve.
- `concept-source-freshness` — date/version de source.
- `concept-source-gap` — absence de source robuste.

### `hallucination_uncertainty`

- `concept-hallucination` — sortie plausible non fidèle.
- `concept-plausibility-bias` — ton confiant trompeur.
- `concept-critical-omission` — information importante absente.
- `concept-overconfidence` — certitude non justifiée.
- `concept-uncertainty-disclosure` — signaler limites et hypothèses.
- `concept-verification-threshold` — niveau de vérification selon enjeu.
- `concept-obsolescence` — information périmée.

### `prompt_practice`

- `concept-task-framing` — cadrer rôle, objectif et contraintes.
- `concept-safe-context` — fournir contexte sans donnée sensible.
- `concept-output-format` — demander une structure exploitable.
- `concept-prompt-dogma` — croyance qu'un prompt suffit.
- `concept-few-shot-risk` — exemples qui exposent ou biaisent.
- `concept-ask-for-sources-limitation` — demander source reste insuffisant.
- `concept-refusal-and-escalation` — refuser ou escalader si risque trop fort.

### `rag_knowledge`

- `concept-rag` — récupération augmentée par génération.
- `concept-retrieval` — recherche documentaire.
- `concept-embedding` — représentation vectorielle.
- `concept-chunking` — découpage documentaire.
- `concept-reranking` — réordonnancement des résultats.
- `concept-groundedness` — ancrage dans les sources fournies.
- `concept-document-freshness` — date/version des documents.
- `concept-rag-poisoning` — contamination du corpus.
- `concept-indirect-prompt-injection` — instruction malveillante dans document.

### `agents_tools`

- `concept-agent` — système enchaînant actions/outils.
- `concept-tool-calling` — appel d'outil par modèle/orchestrateur.
- `concept-agent-permission` — droit de lire/écrire/envoyer.
- `concept-least-privilege` — permission minimale nécessaire.
- `concept-human-approval` — validation avant action à impact.
- `concept-agent-memory` — stockage entre interactions.
- `concept-action-audit` — trace des actions.
- `concept-rollback` — capacité de retour arrière.

### `ai_security`

- `concept-prompt-injection` — instruction de détournement.
- `concept-indirect-prompt-injection` — injection via source externe.
- `concept-data-exfiltration` — sortie non autorisée de données.
- `concept-insecure-output-handling` — exécution/reprise sans contrôle.
- `concept-secret-scanning` — détection de secrets.
- `concept-supply-chain-ai` — chaîne modèles/outils/dépendances.
- `concept-red-teaming` — recherche structurée de failles.
- `concept-logging-sensitive-data` — logs contenant données sensibles.

### `bias_fairness`

- `concept-bias` — écart systématique à impact injuste.
- `concept-stereotype` — représentation simplificatrice.
- `concept-proxy-discrimination` — variable corrélée à caractéristique protégée.
- `concept-representation-harm` — dommage de représentation.
- `concept-fairness-metric-tension` — définitions d'équité contradictoires.
- `concept-bias-review` — revue biais humaine/assistée.
- `concept-inclusive-scenario` — scénario évitant stéréotypes.

### `human_responsibility`

- `concept-human-in-the-loop` — intervention humaine dans processus.
- `concept-human-on-the-loop` — supervision d'un système autonome.
- `concept-accountability` — responsabilité de décision.
- `concept-decision-impact` — niveau d'impact sur personnes/organisation.
- `concept-documentation-of-decision` — trace des hypothèses et limites.
- `concept-escalation` — demander validation compétente.
- `concept-automation-bias` — surconfiance dans sortie automatisée.

### `governance_compliance`

- `concept-gdpr` — cadre données personnelles.
- `concept-ai-act` — cadre européen IA.
- `concept-dpia` — analyse d'impact données.
- `concept-retention` — durée de conservation.
- `concept-purpose-limitation` — finalité déterminée.
- `concept-internal-policy` — règle organisationnelle.
- `concept-audit-trail` — trace vérifiable sans PII inutile.
- `concept-k-anonymity` — seuil d'agrégation.

### `synthetic_media`

- `concept-synthetic-media` — média généré/modifié par IA.
- `concept-deepfake` — média trompeur représentant une personne.
- `concept-media-provenance` — origine, droits, génération.
- `concept-human-face-generation-risk` — risque visage humain IA.
- `concept-consent` — accord des personnes représentées.
- `concept-media-bias` — biais visuel ou narratif.
- `concept-watermarking-limit` — marquage non suffisant seul.

### `business_context`

- `concept-use-case-risk` — risque dépendant du cas d'usage.
- `concept-tool-authorization` — outil approuvé par l'organisation.
- `concept-workflow-boundary` — frontière entre aide et production.
- `concept-review-before-publication` — revue avant diffusion.
- `concept-role-specific-policy` — règle selon métier.
- `concept-productivity-tradeoff` — arbitrage productivité/risque.
- `concept-training-not-rh-evaluation` — formation ≠ notation RH.

## Prérequis transverses

```text
concept-ai-output-not-proof
  -> concept-source-verification
  -> concept-hallucinated-source
  -> concept-verification-threshold

concept-personal-data
  -> concept-data-minimization
  -> concept-pseudonymization
  -> concept-dpia

concept-agent
  -> concept-tool-calling
  -> concept-agent-permission
  -> concept-least-privilege
```

## Critères de publication d'un concept

Un concept peut passer de `draft` à `review` si :

- domaine, résumé, niveau et risques sont renseignés ;
- au moins une compétence ou un objectif pédagogique l'utilise ;
- les confusions dangereuses sont identifiées ;
- les sources ou `source_gap` sont visibles.

Un concept peut devenir `approved` seulement si :

- les sources sont acceptées ;
- le concept n'est pas redondant avec un concept existant ;
- un relecteur humain valide la définition ;
- les relations critiques sont cohérentes ;
- il peut être testé par une question, fiche ou scénario.

## Critères d'acceptation

- Aucun concept n'est publié sans source ou `source_gap` explicite.
- Chaque concept se relie à au moins un risque ou une compétence.
- Les synonymes et ambiguïtés sont indiqués.
- La taxonomie peut être convertie en YAML/JSON ultérieurement.
- Les domaines couvrent les axes de risque existants.
