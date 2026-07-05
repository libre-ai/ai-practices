---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
source_status: accepted_for_documentation_baseline
related:
  - ../01-foundation/source-policy.md
---

# Bibliographie annotée

## Statut

Cette bibliographie est une base de travail pour la Vague B. Les références
ci-dessous sont des **sources candidates** à vérifier humainement avant passage
`stable` : exactitude de l'URL, version, date, portée et claims soutenus.

Aucune source listée ici ne doit être utilisée pour publier un contenu sans revue
humaine et vérification du claim exact.

## Format canonique

```yaml
id: source-nist-ai-rmf-1-0
level: A
type: framework
publisher: NIST
label: Artificial Intelligence Risk Management Framework (AI RMF 1.0)
year: 2023
url: https://www.nist.gov/itl/ai-risk-management-framework
domains: [governance, risk, evaluation]
claim_scope: cadre de gestion des risques IA, pas certification produit
verification_required: true
```

## Niveaux utilisés

| Niveau | Usage |
| --- | --- |
| A | texte normatif, autorité publique, standard ou cadre institutionnel majeur |
| B | guide institutionnel, agence publique, organisme reconnu |
| C | recherche scientifique, article, ouvrage, méta-analyse |
| D | source technique mainteneur, sécurité, framework, projet open source |
| E | source vendeur, documentation produit ou politique fournisseur |
| F | politique interne |
| G | retour d'expérience ou cas public |

## Sources réglementaires et institutionnelles

| ID | Niveau | Source | Portée | Vérification |
| --- | --- | --- | --- | --- |
| `source-eu-gdpr-2016-679` | A | Règlement (UE) 2016/679 — RGPD | données personnelles, principes, droits, responsabilités | vérifier EUR-Lex |
| `source-eu-ai-act-2024-1689` | A | Règlement (UE) 2024/1689 — AI Act | obligations IA selon rôle et niveau de risque | vérifier version consolidée |
| `source-cnil-ai` | A/B | CNIL — ressources IA et données personnelles | doctrine française, RGPD appliqué à l'IA | vérifier page et date |
| `source-nist-ai-rmf-1-0` | A/B | NIST AI Risk Management Framework 1.0 | gouvernance et risques IA | vérifier version |
| `source-nist-genai-profile` | B | NIST Generative AI Profile | risques spécifiques IA générative | vérifier version finale |
| `source-oecd-ai-principles` | B | OECD AI Principles | principes internationaux | vérifier version |
| `source-unesco-ai-ethics` | B | UNESCO Recommendation on the Ethics of AI | principes éthiques IA | vérifier version |
| `source-iso-iec-42001-2023` | A/B | ISO/IEC 42001:2023 | système de management IA | standard payant, vérifier portée |
| `source-iso-iec-23894-2023` | A/B | ISO/IEC 23894:2023 | risk management IA | standard payant, vérifier portée |
| `source-dora-eu-2022-2554` | A | DORA | résilience opérationnelle numérique secteur financier | applicable selon secteur |

## Sources cybersécurité IA

| ID | Niveau | Source | Portée | Vérification |
| --- | --- | --- | --- | --- |
| `source-owasp-llm-top10` | D | OWASP Top 10 for Large Language Model Applications | menaces LLM applicatives | vérifier version |
| `source-mitre-atlas` | D | MITRE ATLAS | tactiques/techniques adverses IA | vérifier version |
| `source-enisa-ai-cybersecurity` | B | ENISA — AI cybersecurity guidance | menaces et contrôles IA | vérifier document exact |
| `source-anssi-ai-security` | B | ANSSI — recommandations et guides sécurité IA si disponibles | doctrine française sécurité | identifier source précise |
| `source-nist-csf-2` | A/B | NIST Cybersecurity Framework 2.0 | gestion risque cyber générale | vérifier version |

## Sources techniques IA générative / LLM / RAG

| ID | Niveau | Source | Portée | Vérification |
| --- | --- | --- | --- | --- |
| `source-vaswani-attention-2017` | C | Vaswani et al., Attention Is All You Need | architecture Transformer | vérifier arXiv/paper |
| `source-brown-gpt3-2020` | C | Brown et al., Language Models are Few-Shot Learners | scaling et few-shot | vérifier paper |
| `source-kaplan-scaling-laws-2020` | C | Kaplan et al., Scaling Laws for Neural Language Models | scaling laws | vérifier paper |
| `source-lewis-rag-2020` | C | Lewis et al., Retrieval-Augmented Generation | RAG | vérifier paper |
| `source-ji-hallucination-survey-2023` | C | Survey on hallucination in natural language generation | typologie hallucinations | vérifier référence exacte |
| `source-bender-stochastic-parrots-2021` | C | Bender et al., On the Dangers of Stochastic Parrots | limites, biais, coûts, langage | vérifier ACM |
| `source-bommasani-foundation-models-2021` | C | On the Opportunities and Risks of Foundation Models | synthèse modèles de fondation | vérifier publication |

## Sources évaluation

| ID | Niveau | Source | Portée | Vérification |
| --- | --- | --- | --- | --- |
| `source-helm-2023` | C | Holistic Evaluation of Language Models (HELM) | benchmark multidimensionnel | vérifier version |
| `source-ragas` | D | RAGAS documentation/research | métriques RAG | vérifier version et limites |
| `source-trulens` | D/E | TruLens documentation | évaluation applications LLM | vérifier licence/portée |
| `source-deepeval` | D | DeepEval documentation | tests LLM applicatifs | vérifier licence/portée |

## Sources pédagogie / cognition / AI literacy

| ID | Niveau | Source | Portée | Vérification |
| --- | --- | --- | --- | --- |
| `source-long-magerko-ai-literacy-2020` | C | Long & Magerko, What is AI Literacy? | dimensions AI literacy | vérifier DOI |
| `source-ng-ai-literacy-review` | C | revues AI literacy récentes | cadre de littératie IA | identifier meilleure revue |
| `source-bloom-taxonomy` | C | Bloom et al., Taxonomy of Educational Objectives | objectifs cognitifs | vérifier édition |
| `source-mayer-multimedia-learning` | C | Mayer, Multimedia Learning | principes multimédia | vérifier édition |
| `source-sweller-cognitive-load` | C | Sweller, Cognitive Load Theory | charge cognitive | vérifier publication de référence |
| `source-roediger-karpicke-2006` | C | Test-enhanced learning / retrieval practice | pratique de récupération | vérifier DOI |
| `source-cepeda-spacing-2006` | C | Distributed practice / spacing effect | espacement | vérifier DOI |
| `source-hattie-timperley-feedback-2007` | C | The Power of Feedback | feedback pédagogique | vérifier DOI |

## Sources vendeurs à isoler

| ID | Niveau | Source | Usage autorisé | Limite |
| --- | --- | --- | --- | --- |
| `source-openai-docs` | E | Documentation OpenAI | capacités/options d'OpenAI | source vendeur uniquement |
| `source-anthropic-docs` | E | Documentation Anthropic | capacités/options d'Anthropic | source vendeur uniquement |
| `source-microsoft-ai-docs` | E | Documentation Microsoft AI | capacités/options Microsoft | source vendeur uniquement |
| `source-google-ai-docs` | E | Documentation Google AI | capacités/options Google | source vendeur uniquement |

## Sources internes futures

```yaml
id: internal-data-classification-policy
level: F
type: internal_policy
owner: security-governance
label: Politique interne de classification des données
url: internal://data-classification
verification_required: reviewer_with_access
```

## Sources à compléter avant `stable`

- Identifier les sources ANSSI précises applicables à l'IA générative.
- Sélectionner une revue AI literacy récente et robuste.
- Sélectionner une référence primaire pour hallucinations/factuality/faithfulness.
- Vérifier les versions OWASP, MITRE ATLAS et NIST GenAI Profile.
- Ajouter les dates d'accès réelles.
- Convertir en BibTeX ou YAML si le volume augmente.

## Critères d'acceptation

- Chaque entrée a type, date ou année, niveau, note de portée et besoin de
  vérification.
- Les sources incertaines sont marquées.
- Les sources vendeurs sont isolées.
- Le document peut être converti en `.bib` ou YAML ultérieurement.
