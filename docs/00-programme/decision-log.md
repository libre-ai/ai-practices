---
status: draft
owner: content-governance
review_required: true
last_reviewed_at: null
---

# Decision log documentaire

Ce journal trace les décisions prises pour le chantier documentaire exhaustif.
Les ADR produit existantes restent la source principale pour les décisions
architecturales ou réglementaires.

## D-001 — Créer une couche documentaire exhaustive sans remplacer les docs existantes

**Décision** : ajouter une arborescence `docs/00-programme` à `docs/07-benchmark`
pour préparer la documentation longue.

**Raison** : permettre un travail de fond sans casser les contrats et guides déjà
utilisés par le code et le corpus.

**Conséquence** : les nouveaux documents restent `stub` ou `draft` tant qu'ils ne
sont pas relus. Les documents historiques restent canoniques.

## D-002 — Commencer par les modèles avant les volumes

**Décision** : définir les modèles de concept, compétence, source, risque,
relation et revue avant d'écrire les centaines d'entrées.

**Raison** : éviter une documentation volumineuse mais incohérente.

**Conséquence** : les premiers livrables sont des plans détaillés et des cadres
de validation.

## D-003 — Toute production IA reste non publiable par défaut

**Décision** : un contenu généré par IA est `draft` par défaut.

**Raison** : alignement avec `docs/content-governance.md` et le principe de revue
humaine obligatoire.

**Conséquence** : la prompt library doit aider à produire des brouillons, pas à
publier automatiquement.

## D-004 — Corpus original, pas de reproduction de produit existant

**Décision** : les benchmarks peuvent analyser des approches pédagogiques, mais
ne doivent pas recréer fidèlement les contenus, prompts ou mécaniques propriétaires
d'un produit tiers.

**Raison** : maîtrise juridique, qualité et différenciation.

**Conséquence** : les scénarios produits doivent être originaux, sourcés et liés
au modèle de compétences maison.

## D-005 — Prioriser la traçabilité sur la quantité

**Décision** : une petite quantité de contenus bien sourcés vaut mieux qu'un gros
corpus invérifiable.

**Raison** : le risque principal du produit est éditorial.

**Conséquence** : la roadmap documentaire exige une politique de sources et des
gates qualité avant la production massive.

## D-006 — Vague A prête pour revue humaine

**Décision** : les documents `docs/01-foundation/*` passent en statut `review`
après enrichissement complet de la charte, vision, principes, glossaire,
politique de sources et politique de contribution.

**Raison** : ils couvrent désormais les invariants nécessaires pour lancer les
vagues recherche et modèle de domaine sans redécider le périmètre.

**Conséquence** : aucune de ces docs n'est encore canonique ; le passage
`stable` demande une revue humaine `foundation-review`.

## D-007 — Vague A stabilisée après revue humaine

**Décision** : les 6 documents `docs/01-foundation/*` passent en statut `stable`
avec `last_reviewed_at: 2026-07-05`.

**Raison** : l'utilisateur a indiqué que la revue Vague A est terminée.

**Conséquence** : la Fondation devient la référence canonique pour les vagues
suivantes, tout en restant modifiable par nouvelle revue.

## D-008 — Vague B Recherche prête pour revue

**Décision** : les 12 documents `docs/02-research/*` passent en statut `review`
après rédaction du dossier recherche : état de l'art, bibliographie, AI literacy,
sciences cognitives, IA générative, LLM, RAG, agents, hallucinations,
évaluation, cybersécurité, gouvernance/régulation.

**Raison** : les contenus structurent les consensus, limites, controverses et
implications corpus nécessaires à la Vague C.

**Conséquence** : les sources restent candidates et doivent être vérifiées avant
passage `stable`, notamment pour la gouvernance/régulation qui demande revue
DPO/juridique.

## D-009 — Vague B stabilisée après validation humaine

**Décision** : les 12 documents `docs/02-research/*` passent en statut `stable`
avec `last_reviewed_at: 2026-07-05`.

**Raison** : l'utilisateur a explicitement validé Vague B.

**Conséquence** : la recherche devient la base de référence pour le modèle de
domaine. Les documents restent des synthèses pédagogiques ; les claims juridiques
ou réglementaires devront toujours être vérifiés dans leur contexte d'usage.

## D-010 — Vague C Modèle de domaine prête pour revue

**Décision** : les 7 documents `docs/03-domain-model/*` passent en statut
`review` après formalisation de la taxonomie, ontologie, graphe de connaissances,
modèle de compétences, framework de littératie IA, misconceptions et modèle de
risques.

**Raison** : il faut stabiliser les modèles avant de produire les 250 concepts,
1000 relations et contenus massifs.

**Conséquence** : la production massive reste différée ; Vague C doit passer en
`stable` avant d'industrialiser les schémas et la Content Factory.

## D-011 — Vague C stabilisée après validation humaine

**Décision** : les 7 documents `docs/03-domain-model/*` passent en statut
`stable` avec `last_reviewed_at: 2026-07-05`.

**Raison** : l'utilisateur a explicitement validé Vague C.

**Conséquence** : taxonomie, ontologie, graphe, compétences, framework,
misconceptions et risques deviennent la base de référence pour la Content
Factory.

## D-012 — Vague D Content Factory prête pour revue

**Décision** : les 8 documents `docs/04-content-factory/*` passent en statut
`review` après formalisation du pipeline, des guides d'écriture, du processus de
revue, des quality gates et de la bibliothèque de prompts.

**Raison** : l'industrialisation de contenu doit être documentée avant toute
production massive de questions, scénarios ou fiches.

**Conséquence** : les prompts restent des prompts de gouvernance et ne peuvent
pas approuver. Le passage `stable` doit confirmer que la revue humaine et les
gates fail-closed sont bien préservés.

## D-013 — Vague D stabilisée après validation humaine

**Décision** : les 8 documents `docs/04-content-factory/*` passent en statut
`stable` avec `last_reviewed_at: 2026-07-05`.

**Raison** : l'utilisateur a explicitement validé Vague D.

**Conséquence** : le pipeline Content Factory, les guides, gates et prompts de
gouvernance deviennent la référence pour la production assistée de contenus.

## D-014 — Vague E Produit/Contrats/Benchmarks prête pour revue

**Décision** : les 15 documents `docs/05-product-learning/*`,
`docs/06-technical-contracts/*` et `docs/07-benchmark/*` passent en statut
`review` après formalisation des parcours, scoring, analytics, UX, contrats,
schémas, pipeline, provenance, revue multi-agent et benchmarks.

**Raison** : il faut relier pédagogie, runtime, API, schémas et veille avant de
lancer la production massive de corpus.

**Conséquence** : la production massive reste autorisée uniquement en `draft` et
après revue Vague E ; les documents techniques ne déclenchent pas encore de
migration ou d'implémentation.

## D-015 — Vague E stabilisée après validation humaine

**Décision** : les 15 documents `docs/05-product-learning/*`,
`docs/06-technical-contracts/*` et `docs/07-benchmark/*` passent en statut
`stable` avec `last_reviewed_at: 2026-07-05`.

**Raison** : l'utilisateur a explicitement validé Vague E.

**Conséquence** : les décisions produit, contrats techniques, analytics,
provenance et benchmarks deviennent la référence pour lancer le corpus.

## D-016 — Démarrage du corpus draft-first

**Décision** : créer un socle corpus structuré en `draft` : sources, claims,
concepts, risques, compétences, misconceptions, objectifs pédagogiques,
parcours, graphe et questions seed.

**Raison** : les vagues documentaires A à E sont stabilisées ; la production de
corpus peut démarrer, mais uniquement avec revue humaine avant publication.

**Conséquence** : les objets étendus préparent les futurs schémas sans casser le
schéma question actuel. Les nouvelles questions restent compatibles avec le CLI
existant et non publiables tant qu'elles sont `draft`.

## D-017 — Premier lot large de corpus en draft

**Décision** : ajouter `content/concepts/batch-001.yml` et
`content/questions/batch-001.yml` pour étendre le corpus initial avec 48 concepts
et 29 questions supplémentaires, tous en `draft`.

**Raison** : après stabilisation des vagues A à E et création du seed, il faut
augmenter la couverture des axes avant la revue éditoriale détaillée.

**Conséquence** : le corpus atteint 70 questions lues par le CLI, toujours avec
0 question `approved`. La diffusion reste interdite tant qu'une revue humaine ne
passe pas les contenus en `review` puis `approved`.

## Critères d'acceptation

- Chaque décision a un identifiant stable.
- La raison et la conséquence sont explicites.
- Les décisions structurantes sont promues en ADR si elles dépassent le chantier documentaire.
- Aucune décision ne contredit les ADR existantes sans document de supersession.
