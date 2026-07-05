---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ../content-governance.md
  - ../local-review.md
  - ../model-delegation.md
  - ../01-foundation/source-policy.md
  - ../03-domain-model/taxonomy.md
  - ../03-domain-model/competency-model.md
  - ../03-domain-model/risk-model.md
  - ./quality-gates.md
  - ./review-process.md
  - ./prompt-library.md
---

# Content Factory

## Objectif

Décrire la chaîne de production qui transforme la connaissance validée en
contenus pédagogiques utilisables : concepts, fiches, scénarios, questions,
feedbacks et parcours.

La Content Factory n'est pas un générateur autonome. C'est un système de
production **assistée**, contrôlé par schémas, quality gates et revue humaine.

## Principe central

La Content Factory produit des brouillons, pas des vérités publiées.

```text
IA = assistance de brouillon/revue
Humain = responsabilité de publication
Schémas = garde-fous structurels
Gates = refus fail-closed
Git = traçabilité
```

Le passage à `approved` reste humain.

## Entrées

| Entrée | Source | Usage |
| --- | --- | --- |
| Sources | `docs/02-research/bibliography.md`, politiques internes | claims et contexte |
| Concepts | `docs/03-domain-model/taxonomy.md` | notions à enseigner |
| Risques | `docs/03-domain-model/risk-model.md` | erreurs à reconnaître/mitiger |
| Compétences | `docs/03-domain-model/competency-model.md` | comportements observables |
| Misconceptions | `docs/03-domain-model/misconceptions.md` | pièges pédagogiques |
| Schémas | `schemas/*.json` | validation structurelle |
| Templates | `docs/templates/*.yml` | exemples de forme |
| Prompts | `prompt-library.md` | assistance contrôlée |

## Sorties

| Sortie | Statut par défaut | Publication |
| --- | --- | --- |
| Concept YAML futur | `draft` | après source + revue |
| Question YAML | `draft` | après validation + revue humaine |
| Scénario | `draft` | intégré à question ou parcours |
| Feedback | `draft` | avec question validée |
| Fiche réflexe | `draft` | après revue source/pédagogie |
| Rapport de revue | non publiable utilisateur | preuve interne |
| Rapport d'audit | preuve qualité | conservé/versionné |

## Pipeline éditorial

```text
source -> claim -> concept -> compétence -> objectif pédagogique
       -> misconception/risk -> scénario -> question -> feedback
       -> validation schema -> gates automatiques -> revue assistée
       -> revue humaine multi-rôles -> approved | blocked | retired
```

## Étapes détaillées

### 1. Qualification de source

Entrée : source candidate.

Sortie : source classée, datée, limitée à un périmètre de claims.

Responsable : auteur + relecteur source si claim fort.

Gates : pas de source inventée, pas de source vendeur utilisée seule pour vérité
générale.

### 2. Extraction de claims

Entrée : source qualifiée.

Sortie : claims atomiques, vérifiables, limités.

Exemple :

```yaml
id: claim-rag-does-not-guarantee-truth
statement: Le RAG peut améliorer l'ancrage documentaire mais ne garantit pas que la réponse soit vraie ou correctement attribuée.
scope: usage professionnel RAG
source_ids:
  - source-lewis-rag-2020
confidence: medium
```

### 3. Mapping concept / risque / compétence

Entrée : claim ou besoin pédagogique.

Sortie : concept cible, risque principal, compétence observable.

Exemple :

```yaml
concept_id: concept-citation-not-proof
risk_id: risk-misattribution
competency_id: comp-check-source
```

### 4. Objectif pédagogique

Entrée : compétence + niveau + risque.

Sortie : objectif observable.

Exemple :

```yaml
id: lo-check-source-guided
observable_outcome: L'utilisateur identifie qu'une citation IA doit être vérifiée dans la source primaire avant réutilisation.
level: guided
```

### 5. Scénario

Entrée : objectif pédagogique.

Sortie : situation professionnelle réaliste.

Contraintes : pas de données personnelles réelles, pas de cas client identifiable,
pas de stéréotype, ambiguïté contrôlée.

### 6. Question

Entrée : scénario.

Sortie : YAML de question en `draft`.

Contraintes : choix plausibles, scores pédagogiques, sources, risques, review
metadata.

### 7. Feedback

Entrée : choix et risques.

Sortie : correction actionnable.

Formule :

```text
jugement du choix + risque créé/évité + raison + action recommandée + source/limite
```

### 8. Validation automatique

Entrée : fichier de contenu.

Sortie : pass/warn/fail/blocker.

Contrôles : schéma, champs obligatoires, statut/revue, source gap, média,
risques, feedbacks.

### 9. Revue assistée

Entrée : contenu validé structurellement.

Sortie : rapport non final.

Rôles IA possibles : contenu, sécurité/RGPD, biais, sources, pédagogie. Aucun ne
peut approuver.

### 10. Revue humaine

Entrée : contenu + rapports.

Sortie : `approved`, `blocked`, retour auteur ou `retired`.

Responsable : mainteneur corpus avec reviewers adaptés.

## Matrice de responsabilités

| Étape | Auteur | IA assistée | Relecteur humain | Mainteneur corpus |
| --- | --- | --- | --- | --- |
| Source | propose | résume/risques | vérifie portée | valide politique |
| Claim | rédige | détecte ambiguïtés | confirme | arbitre |
| Concept | propose | suggère relations | valide domaine | maintient taxonomie |
| Scénario | rédige | variantes | vérifie réalisme/biais | accepte statut |
| Question | rédige | brouillon | vérifie pédagogie/sécurité | approuve ou bloque |
| Feedback | rédige | reformule | vérifie nuance | approuve ou bloque |
| Média | propose | signale biais | audite | bloque si doute |
| Publication | non | jamais | recommande | décide |

## Statuts

| Statut | Sens | Qui peut y passer |
| --- | --- | --- |
| `draft` | brouillon non publiable | auteur |
| `review` | prêt pour revue | auteur après validations |
| `approved` | publiable | mainteneur corpus après revue humaine |
| `blocked` | non publiable | tout relecteur si blocker |
| `retired` | retiré | mainteneur corpus |

## Gates obligatoires

- Validité schéma.
- Source acceptable ou `source_gap` non publiable.
- Aucune incitation à exposer PII/secrets.
- Feedback actionnable.
- Pas de média IA humain réaliste non audité.
- Statut cohérent avec revue.
- Pas d'usage RH implicite.
- Compétence et risque reliés.

## Mode local-first

La Content Factory doit pouvoir fonctionner en local :

```bash
cargo run -p rumble-ai-practices-cli -- validate-corpus --content content/questions
cargo run -p rumble-ai-practices-cli -- audit-corpus --content content/questions --media content/media --out reports/audit.json
```

Commandes futures candidates :

```bash
cargo run -p rumble-ai-practices-cli -- validate-concepts --content content/concepts
cargo run -p rumble-ai-practices-cli -- audit-coverage --content content/questions --graph content/graph.yml
```

## Métriques de pilotage

Acceptables :

- nombre de contenus `approved` par axe ;
- âge des revues ;
- blockers détectés avant publication ;
- couverture concepts/compétences/risques ;
- sources expirées ;
- contenus avec `source_gap`.

Interdites :

- productivité individuelle d'auteurs comme score ;
- classement d'apprenants ;
- taux d'erreur individuel transmis au management ;
- nombre de contenus générés par IA comme indicateur de qualité.

## Definition of Done d'un contenu publié

Un contenu peut être publié si :

- schéma valide ;
- statut `approved` ;
- revue humaine documentée ;
- sources ou justification interne ;
- risques et compétences renseignés ;
- feedback actionnable ;
- absence de blocker sécurité/RGPD/biais/RH ;
- date de revue renseignée ;
- historique git disponible.

## Critères d'acceptation

- Chaque étape a une entrée, une sortie et un responsable.
- Les responsabilités IA/humain sont séparées.
- Le pipeline peut être exécuté localement.
- Les erreurs bloquantes empêchent la publication.
- Les artefacts sont reliés aux modèles de domaine stabilisés.
