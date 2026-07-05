---
status: draft
owner: content-governance
review_required: true
last_reviewed_at: null
---

# Roadmap documentaire

## Objectif

Planifier la production de la documentation exhaustive sans confondre vitesse et
qualité. La roadmap est séquencée par dépendances intellectuelles, pas par dates.

## Fonction de coût

Décider selon l'ordre suivant :

1. sécurité et conformité ;
2. qualité pédagogique et exactitude ;
3. maintenabilité ;
4. performance de production ;
5. complétude.

La documentation exhaustive ne doit jamais accélérer la publication de contenus
non relus.

## Vague A — Fondation

### Livrables

- `../01-foundation/project-charter.md`
- `../01-foundation/vision-extended.md`
- `../01-foundation/principles.md`
- `../01-foundation/glossary.md`
- `../01-foundation/source-policy.md`
- `../01-foundation/contribution-policy.md`

### Acceptation

- Les invariants produit sont cohérents avec les ADR existantes.
- La politique de sources distingue sources normatives, scientifiques, vendeurs
  et politiques internes.
- Le glossaire réduit les ambiguïtés sur IA, LLM, RAG, agent, hallucination,
  biais, donnée personnelle, secret et source.

### État courant

Les 6 documents de Vague A ont passé la revue humaine `foundation-review` et
sont en statut `stable` depuis le 2026-07-05.

## Vague B — Recherche

### Livrables

- `../02-research/state-of-the-art.md`
- dossiers thématiques : IA générative, LLM, RAG, agents, hallucinations,
  évaluation, cybersécurité, gouvernance/régulation, sciences cognitives.
- `../02-research/bibliography.md`

### Acceptation

- Chaque affirmation structurante est sourcée ou explicitement marquée comme
  hypothèse.
- Les limites et controverses sont documentées.
- Les sources commerciales sont séparées des sources scientifiques/normatives.

### État courant

Les 12 documents de Vague B ont été validés par revue humaine et sont en statut
`stable` depuis le 2026-07-05.

## Vague C — Modèle de domaine

### Livrables

- `../03-domain-model/taxonomy.md`
- `../03-domain-model/ontology.md`
- `../03-domain-model/knowledge-graph.md`
- `../03-domain-model/competency-model.md`
- `../03-domain-model/ai-literacy-framework.md`
- `../03-domain-model/misconceptions.md`
- `../03-domain-model/risk-model.md`

### Acceptation

- Le modèle de concept est stabilisé avant d'écrire les 250 concepts.
- Les relations entre concepts ont une typologie explicite.
- Les compétences sont observables par scénario, pas seulement déclaratives.

### État courant

Les 7 documents de Vague C ont été validés par revue humaine et sont en statut
`stable` depuis le 2026-07-05. Ils formalisent le modèle avant la production
massive des 250 concepts et 1000 relations.

## Vague D — Content Factory

### Livrables

- `../04-content-factory/content-factory.md`
- `../04-content-factory/editorial-guide.md`
- `../04-content-factory/question-writing-guide.md`
- `../04-content-factory/scenario-writing-guide.md`
- `../04-content-factory/feedback-writing-guide.md`
- `../04-content-factory/review-process.md`
- `../04-content-factory/quality-gates.md`
- `../04-content-factory/prompt-library.md`

### Acceptation

- Tout contenu généré par IA commence en `draft`.
- La revue humaine reste obligatoire.
- Les gates refusent explicitement les sources inventées, les fuites PII/secrets,
  les corrections dogmatiques et les biais non maîtrisés.

### État courant

Les 8 documents de Vague D ont été validés par revue humaine et sont en statut
`stable` depuis le 2026-07-05. Ils définissent le pipeline Content Factory, les
guides rédactionnels, les gates et les prompts de gouvernance.

## Vague E — Produit et contrats

### Livrables

- `../05-product-learning/*`
- `../06-technical-contracts/*`
- `../07-benchmark/*`

### Acceptation

- Le scoring reste pédagogique, non RH.
- Les analytics sont anonymes ou agrégés selon les règles existantes.
- Les schémas techniques ne contredisent pas les modèles métier existants.

### État courant

Les 15 documents de Vague E ont été validés par revue humaine et sont en statut
`stable` depuis le 2026-07-05. Ils relient pédagogie, parcours, adaptation,
scoring, analytics, UX, contrats techniques, pipeline, provenance et benchmarks.

## Corpus — Production draft-first

### Livrables initiaux

- `../../content/README.md`
- `../../content/sources/core.yml`
- `../../content/claims/core.yml`
- `../../content/concepts/core.yml`
- `../../content/risks/core.yml`
- `../../content/competencies/core.yml`
- `../../content/misconceptions/core.yml`
- `../../content/learning-objectives/core.yml`
- `../../content/learning-paths/core.yml`
- `../../content/graph/core.yml`
- `../../content/questions/corpus-seed.yml`

### Acceptation

- Tous les nouveaux artefacts corpus commencent en `draft`.
- Les questions restent compatibles avec le schéma existant.
- Les objets étendus préparent les futurs schémas sans casser le CLI actuel.
- Aucun contenu n'est publiable sans revue humaine.

### État courant

Le socle corpus initial est créé en `draft`. Un premier lot large ajoute
`content/concepts/batch-001.yml` et `content/questions/batch-001.yml`, portant le
corpus validable à 70 questions lues par le CLI, toujours 0 `approved`. Il sert à
démarrer la production et la revue, pas à diffuser du contenu approuvé.

## Jalons de revue

| Jalon | Condition de passage |
| --- | --- |
| `foundation-review` | terminé — Vague A en `stable` |
| `research-outline-review` | terminé — Vague B en `stable` |
| `domain-model-freeze` | terminé — Vague C en `stable` |
| `factory-ready` | terminé — Vague D en `stable` |
| `corpus-scale-start` | démarré — socle corpus en `draft`, publication interdite sans revue |

## Critères d'acceptation globaux

- Les vagues sont séquencées par dépendances, pas par pression de volume.
- Aucun contenu massif n'est lancé avant stabilisation des modèles concept/source/compétence/risque.
- Chaque jalon produit une preuve relisible : fichiers, revue ou rapport.
- Les documents non stabilisés restent `stub`, `draft` ou `review`, jamais `stable`.
