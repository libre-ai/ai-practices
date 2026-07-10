---
status: draft
owner: content-governance
review_required: true
last_reviewed_at: null
supersedes: null
canonical_until_stable:
  - ../vision.md
  - ../content-governance.md
  - ../question-model.md
  - ../security-rgpd.md
---

# Programme documentaire exhaustif

## Objet

Ce document lance le chantier documentaire complet de `rumble-ai-practices`.
Il sert de contrat de production avant de générer massivement des concepts,
questions, scénarios, fiches réflexes ou parcours.

L'objectif n'est pas d'empiler des pages. L'objectif est de créer une
**documentation de référence, structurée, maintenable et auditable** pour un
produit d'entraînement professionnel aux usages responsables de l'IA.

## Principe directeur

`rumble-ai-practices` n'est pas un quiz IA. C'est un produit pédagogique audité
fondé sur trois actifs :

1. un référentiel de connaissances versionné ;
2. un modèle de compétences professionnelles liées à l'IA ;
3. une chaîne de production de contenus avec validation humaine.

L'application web/native est une surface de restitution. Le cœur durable est le
corpus, sa traçabilité, ses sources et ses règles de revue.

## Invariants hérités du produit

- Contenu-as-data : les contenus sont versionnés, relisibles et auditables.
- Validation humaine obligatoire avant publication.
- Pas de scoring RH nominatif, pas de leaderboard, pas de sanction individuelle.
- Souveraineté : build local, self-hostable, licences compatibles MIT/Apache/MPL.
- Nuance pédagogique : scénarios contextualisés plutôt que vrai/faux simplistes.
- Sécurité/RGPD avant complétude du corpus.
- Aucun média IA humain réaliste sans justification et audit de biais.
- Les sources ne sont jamais inventées ; les incertitudes sont explicites.

## Architecture documentaire cible

```text
docs/
  00-programme/
    programme-documentaire.md
    roadmap-documentaire.md
    decision-log.md

  01-foundation/
    project-charter.md
    vision-extended.md
    principles.md
    glossary.md
    source-policy.md
    contribution-policy.md

  02-research/
    state-of-the-art.md
    ai-literacy.md
    cognitive-science.md
    generative-ai.md
    llm.md
    rag.md
    agents.md
    hallucinations.md
    evaluation.md
    cybersecurity.md
    governance-regulation.md
    bibliography.md

  03-domain-model/
    taxonomy.md
    ontology.md
    knowledge-graph.md
    competency-model.md
    ai-literacy-framework.md
    misconceptions.md
    risk-model.md

  04-content-factory/
    content-factory.md
    editorial-guide.md
    question-writing-guide.md
    scenario-writing-guide.md
    feedback-writing-guide.md
    review-process.md
    quality-gates.md
    prompt-library.md

  05-product-learning/
    pedagogy.md
    learning-paths.md
    adaptive-learning.md
    scoring-model.md
    analytics-privacy.md
    ux-principles.md

  06-technical-contracts/
    data-model-extended.md
    schemas.md
    api-contracts-extended.md
    pipeline.md
    multi-agent-review.md
    provenance.md

  07-benchmark/
    benchmark-ai-literacy-tools.md
    benchmark-serious-games.md
    benchmark-corporate-training.md
```

## Statuts documentaires

| Statut | Signification | Publier comme référence ? |
| --- | --- | --- |
| `stub` | squelette, plan, questions directrices | non |
| `draft` | contenu rédigé mais non relu | non |
| `review` | prêt pour revue humaine | non |
| `stable` | accepté comme référence du projet | oui |
| `deprecated` | remplacé par un autre document | non |

Tant qu'un document de cette nouvelle arborescence n'est pas `stable`, les docs
historiques du repo restent canoniques.

## Définition of Done d'un document stable

Un document ne peut passer `stable` que si :

- son objectif est clair ;
- son périmètre et ses non-objectifs sont explicites ;
- il cite les sources ou politiques internes nécessaires ;
- il contient des critères d'acceptation vérifiables ;
- il ne contredit pas `docs/content-governance.md` ni `docs/security-rgpd.md` ;
- il indique les dépendances amont/aval ;
- il a été relu par au moins un humain compétent du sujet ;
- il est aligné avec sécurité > qualité > performance > complétude.

## Ordre de production recommandé

### Vague A — Fondation

But : aligner le vocabulaire, les règles et la légitimité du projet.

Livrables :

- `project-charter.md`
- `vision-extended.md`
- `principles.md`
- `glossary.md`
- `source-policy.md`
- `contribution-policy.md`

### Vague B — Recherche et modèle de domaine

But : formaliser ce que le produit enseigne et pourquoi.

Livrables :

- `state-of-the-art.md`
- `taxonomy.md`
- `competency-model.md`
- `ai-literacy-framework.md`
- `risk-model.md`
- `misconceptions.md`

### Vague C — Industrialisation éditoriale

But : transformer la connaissance validée en contenus utilisables.

Livrables :

- `content-factory.md`
- `editorial-guide.md`
- `question-writing-guide.md`
- `quality-gates.md`
- `prompt-library.md`
- `pipeline.md`
- `provenance.md`

### Vague D — Produit, mesure et benchmark

But : relier la pédagogie, l'expérience produit et l'amélioration continue.

Livrables :

- `pedagogy.md`
- `learning-paths.md`
- `adaptive-learning.md`
- `analytics-privacy.md`
- `benchmark-ai-literacy-tools.md`

## Non-objectifs du chantier documentaire

- Copier le contenu, la structure interne ou les prompts d'un produit existant.
- Créer un LMS générique.
- Remplacer une validation juridique ou sécurité.
- Publier automatiquement du contenu généré par IA.
- Introduire un usage RH implicite par la documentation.
- Définir une stack cloud obligatoire.

## Décisions à acter avant production massive

1. Niveau de profondeur attendu pour la revue de littérature.
2. Format canonique des sources et références.
3. Modèle minimal de concept.
4. Modèle minimal de compétence.
5. Statut attendu des premiers contenus générés : `draft` par défaut.
6. Règles de passage de `draft` à `review` puis `approved`.

## Critères d'acceptation du lancement

Le chantier documentaire est considéré lancé quand :

- l'arborescence cible existe ;
- chaque document cible a un squelette avec objectif, périmètre et critères ;
- le `decision-log.md` contient les décisions initiales ;
- `docs/README.md` pointe vers ce programme ;
- aucun document ne prétend être `stable` sans revue humaine.
