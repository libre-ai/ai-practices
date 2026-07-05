---
status: stable
owner: content-governance
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ./project-charter.md
  - ./principles.md
  - ./source-policy.md
  - ../content-governance.md
  - ../question-model.md
  - ../local-review.md
  - ../testing-strategy.md
---

# Politique de contribution

## Objectif

Permettre des contributions humaines ou assistées par IA sans dégrader la qualité,
la sécurité, la conformité ou la confiance du corpus.

Cette politique s'applique aux contributions documentaires, éditoriales et
techniques qui touchent la base de connaissances, les questions, les scénarios,
les médias, les prompts, les schémas ou la validation.

## Principes

- Une contribution doit être petite, relisible et réversible.
- Un contenu nouveau commence en `draft`.
- Une IA peut aider à produire un brouillon, jamais approuver.
- Une source doit être vérifiable ou explicitement marquée comme gap.
- La sécurité/RGPD et l'absence d'usage RH implicite priment sur la complétude.
- Les commandes de vérification doivent être indiquées.
- Les contributeurs doivent pouvoir expliquer leurs décisions éditoriales.

## Types de contributions acceptées

| Type | Exemples | Statut attendu |
| --- | --- | --- |
| Concept | ajout ou correction dans la future taxonomie | `draft` ou `review` |
| Source | ajout, retrait, qualification, date de revue | selon usage, jamais source inventée |
| Question | scénario, choix, feedback, risques | `draft` par défaut |
| Média | référence, audit, remplacement, retrait | non publié sans revue média |
| Documentation | guide, politique, benchmark, revue | `draft` ou `review` |
| Prompt | prompt de brouillon ou revue | non opérationnel sans revue |
| Schéma | validation JSON/YAML | tests requis |
| Code | crates, CLI, API, UI | gates techniques requis |
| Signalement | biais, erreur, obsolescence, faille | traité comme issue/revue prioritaire selon gravité |

## Contributions refusées

- Contenu généré et marqué directement `approved`.
- Source inventée ou invérifiable présentée comme référence.
- Reproduction d'un contenu propriétaire tiers.
- Question qui pousse à exposer PII, secrets, logs ou code sensible.
- Mécanique de scoring RH ou classement nominatif.
- Média IA humain réaliste non audité.
- Dépendance majeure sans justification licence/souveraineté/maintenance.
- Tracking ou analytics non documentés.
- Contournement de validation schema ou quality gate.
- Prompts destinés à cloner fidèlement un produit existant.

## Cycle de vie d'un contenu

```text
draft -> review -> approved
          |          |
          v          v
       blocked    retired
```

### `draft`

Statut par défaut. Peut contenir des incertitudes et `source_gap`, mais doit les
rendre visibles.

### `review`

Prêt pour relecture humaine. Le contenu doit être complet, passer les validations
locales et indiquer les points restant à vérifier.

### `approved`

Publiable uniquement après revue humaine adaptée et métadonnées complètes.

### `blocked`

Non publiable. Utilisé pour source inventée, risque sécurité/RGPD, biais non
maîtrisé, ambiguïté majeure ou usage RH implicite.

### `retired`

Retiré de la publication mais conservé pour traçabilité.

## Processus standard

1. Lire les documents de cadrage applicables.
2. Créer ou modifier le contenu en statut approprié.
3. Renseigner intention, risques, sources et revue attendue.
4. Exécuter les validations locales pertinentes.
5. Décrire les changements et les commandes exécutées.
6. Demander une revue humaine adaptée.
7. Corriger, bloquer ou retirer selon retour.
8. Passage `approved` uniquement par mainteneur corpus.

## Documents à lire avant contribution

### Pour contenu/question

- `docs/content-governance.md`
- `docs/question-model.md`
- `docs/01-foundation/source-policy.md`
- `docs/04-content-factory/quality-gates.md`

### Pour média

- `docs/grille-audit-biais-media.md`
- `docs/adrs/0004-media-ai-bias-review.md`

### Pour sécurité/RGPD

- `docs/security-rgpd.md`
- `docs/adrs/0006-anonymity-and-retention-cohort.md`

### Pour code

- `AGENTS.md`
- `docs/architecture.md`
- `docs/testing-strategy.md`
- `docs/implementation-plan.md`

### Pour docs fondation/recherche

- `docs/00-programme/programme-documentaire.md`
- `docs/01-foundation/project-charter.md`
- `docs/01-foundation/principles.md`
- `docs/01-foundation/source-policy.md`

## Usage de l'IA par les contributeurs

### Autorisé

- reformuler ;
- proposer des variantes ;
- détecter des risques ;
- générer des contre-exemples ;
- produire un premier brouillon en `draft` ;
- aider à structurer une revue ;
- suggérer des sources à vérifier.

### Interdit

- inventer des sources ;
- décider de la conformité ;
- publier sans revue ;
- masquer l'origine incertaine d'une affirmation ;
- envoyer des données sensibles à un service IA non autorisé ;
- copier un produit existant ;
- transformer une sortie IA en preuve.

### Déclaration recommandée

Dans une MR ou note de contribution, indiquer brièvement :

```text
Assistance IA : oui/non
Usage : reformulation / brouillon / revue risque / autre
Sources vérifiées par humain : oui/non
Contenu publié directement : non
```

## Exigences de sources

Une contribution éditoriale doit :

- relier les affirmations fortes à des sources ;
- utiliser le niveau de source adapté ;
- indiquer les `source_gap` ;
- dater les sources ;
- ne pas sur-interpréter une source vendeur ;
- faire relire les claims juridiques/RGPD par un profil adapté.

## Exigences sécurité/RGPD

Une contribution ne doit pas :

- inclure de vraie donnée personnelle dans un exemple ;
- inclure de secret, token, clé, log sensible ou code confidentiel ;
- encourager l'envoi d'informations internes à un outil externe non autorisé ;
- ajouter une collecte de données sans mise à jour des docs RGPD ;
- introduire un identifiant nominatif dans un rapport ou log ;
- affaiblir l'anonymat de cohorte.

Exemples fictifs : utiliser des noms génériques, données synthétiques et valeurs
manifestement non réelles.

## Exigences biais et médias

- Éviter les stéréotypes de rôle, genre, âge, origine, handicap ou métier.
- Ne pas utiliser de visage humain généré par IA sans justification forte.
- Renseigner provenance, droits et revue des médias.
- Bloquer un média en cas de doute raisonnable.
- Préférer des scénarios textuels ou schématiques quand l'image n'apporte rien.

## Exigences techniques

Pour le code, respecter les gates du repo :

```bash
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo deny check
```

Pour le corpus :

```bash
cargo run -p rumble-ai-practices-cli -- validate-corpus --content content/questions
cargo run -p rumble-ai-practices-cli -- audit-corpus --content content/questions --media content/media --out reports/audit.json
```

Pour la documentation seule, indiquer les vérifications adaptées : liens, statuts,
critères d'acceptation, cohérence avec les ADR.

## Format attendu d'une contribution

Une MR ou demande de revue doit contenir :

```md
## Objet

## Fichiers changés

## Statut des contenus

## Sources ajoutées/modifiées

## Risques sécurité/RGPD/biais

## Commandes de vérification

## Points demandant revue humaine
```

## Critères de revue

Le relecteur vérifie :

- périmètre clair ;
- absence de données sensibles ;
- sources adaptées ;
- cohérence avec les principes ;
- statut correct ;
- feedback actionnable si contenu pédagogique ;
- pas d'usage RH implicite ;
- tests ou validations exécutés ;
- réversibilité du changement.

## Gestion des désaccords

En cas de désaccord :

1. Identifier l'axe concerné : sécurité, qualité, performance, complétude,
   souveraineté/conformité.
2. Revenir aux ADR et principes.
3. Si le risque touche PII, secrets, sécurité ou RH, bloquer par défaut.
4. Documenter la décision dans le decision log ou une ADR si elle est durable.

## Politique de retrait

Un contenu doit être retiré ou repassé en `review` si :

- une source est invalidée ;
- une politique interne change ;
- un relecteur signale un risque sécurité/RGPD ;
- un biais est découvert ;
- la correction devient instable ;
- la date de revue expire.

Le retrait doit conserver la raison et, si possible, un remplacement.

## Contribution externe et propriété intellectuelle

Les contributeurs doivent proposer du contenu original ou compatible avec la
licence du projet. Les benchmarks peuvent analyser des produits tiers, mais ne
doivent pas reproduire leurs contenus, prompts, interfaces ou mécaniques
propriétaires.

Le contenu inspiré d'une source doit être reformulé, sourcé et limité à ce que la
licence et le droit permettent.

## Checklist auteur

Avant demande de revue :

- [ ] J'ai lu les docs applicables.
- [ ] Le statut est correct.
- [ ] Les sources sont vérifiées ou les gaps sont explicites.
- [ ] Aucune PII ou secret réel n'est présent.
- [ ] Le contenu ne crée pas d'usage RH implicite.
- [ ] Les médias sont absents ou audités.
- [ ] Les validations locales pertinentes sont exécutées.
- [ ] Les points d'incertitude sont signalés.

## Critères d'acceptation

- La contribution est petite et relisible.
- Le statut est correct.
- Les changements sémantiques incrémentent la version quand le modèle l'exige.
- Les sources sont adaptées.
- Les commandes de validation sont indiquées dans la MR.
- L'assistance IA éventuelle est déclarée sans lui donner autorité.
- Les critères sécurité/RGPD/biais sont vérifiables.

## Statut de revue

Ce document a passé la revue humaine `foundation-review` et sert de référence
canonique pour la Vague A.
