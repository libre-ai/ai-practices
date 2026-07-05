---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
source_status: accepted_for_documentation_baseline
related:
  - ./hallucinations.md
  - ./rag.md
  - ../04-content-factory/quality-gates.md
---

# Évaluation des systèmes et contenus IA

## Objectif

Définir les approches d'évaluation pertinentes pour juger réponses IA, contenus
pédagogiques et corpus, sans transformer une métrique en autorité finale.

## Objets à évaluer

1. Une sortie IA dans un scénario utilisateur.
2. Une question ou un feedback du corpus.
3. Une source ou citation.
4. Un système RAG/agent dans un workflow.
5. Une session pédagogique.

## Dimensions

| Dimension | Question | Méthodes possibles |
| --- | --- | --- |
| Factualité | est-ce vrai ? | source primaire, revue humaine, tests |
| Groundedness | est-ce soutenu par le contexte ? | comparaison source/réponse, rubrics |
| Faithfulness | respecte-t-elle les documents/contraintes ? | vérification claim par claim |
| Utilité | aide-t-elle la tâche ? | revue métier, feedback utilisateur |
| Sécurité | expose-t-elle PII/secrets ou action dangereuse ? | gates, red teaming, secret scanning |
| Biais | produit-elle représentation injuste ? | revue biais, tests de variantes |
| Pédagogie | apprend-on le bon réflexe ? | rubrics, revue pédagogique |
| Maintenabilité | est-ce traçable/versionné ? | schémas, audit corpus |

## Méthodes

### Revue humaine

Indispensable pour publication, claims juridiques, sécurité, biais et jugement
pédagogique.

### Tests automatiques

Utiles pour schémas, champs obligatoires, statuts, liens, dates, absence de
contenus interdits simples.

### LLM-as-judge

Peut aider à repérer incohérences, risques ou formulations faibles. Ne doit pas
approuver, surtout si le même type de modèle produit le contenu.

### Benchmarks

Utiles pour comparer tendances, insuffisants pour prédire une tâche métier
spécifique.

### Red teaming

Utile pour découvrir vulnérabilités, contournements et abus, mais ne prouve pas
absence de risque.

## Rubric minimale pour une question

| Critère | PASS | WARN | FAIL/BLOCKER |
| --- | --- | --- | --- |
| Contexte | décision possible | détail mineur manquant | bonne réponse indéterminable |
| Source | claim soutenu | source faible mais signalée | source inventée |
| Sécurité | aucun secret/PII exposé | exemple à clarifier | incite exposition |
| Feedback | risque + action | manque nuance | correction dogmatique |
| Biais | aucun signal | représentation à revoir | stéréotype non traité |
| RH | usage pédagogique privé | ambiguïté d'affichage | score nominatif |

## Évaluation RAG

- pertinence des documents récupérés ;
- fidélité de la réponse ;
- exactitude des citations ;
- gestion des contradictions ;
- robustesse aux injections ;
- respect des droits d'accès.

## Évaluation agentique

- permissions minimales ;
- validation humaine avant actions à impact ;
- logs sans PII/secrets ;
- rollback possible ;
- séparation instruction/donnée ;
- résistance aux documents malveillants.

## Risques d'évaluation

- Sur-optimiser pour une métrique.
- Confondre score benchmark et sécurité réelle.
- Utiliser un juge IA qui reproduit les biais du générateur.
- Ignorer les cas rares mais graves.
- Valider un contenu pédagogiquement mauvais parce qu'il est factuellement exact.

## Implications gates

Les quality gates doivent distinguer :

- automatique bloquant : schéma, statut, source gap en approved, PII évidente ;
- automatique warning : source vendeur seule, date ancienne ;
- humain bloquant : sécurité/RGPD, biais, correction dangereuse ;
- humain pédagogique : clarté, niveau, transfert.

## Critères d'acceptation

- Les métriques ne remplacent pas la responsabilité humaine.
- Les rubrics sont actionnables.
- Les risques d'auto-évaluation par modèle sont signalés.
- Le document alimente `quality-gates.md`.
