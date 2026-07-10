---
status: stable
owner: product-content
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ./project-charter.md
  - ./vision-extended.md
  - ../content-governance.md
  - ../security-rgpd.md
  - ../testing-strategy.md
---

# Principes de conception

## Objectif

Ce document définit les principes qui guident les décisions produit,
éditoriales, pédagogiques, techniques et organisationnelles de
`rumble-ai-practices`.

Un principe doit servir à trancher. S'il ne permet pas de refuser un contenu, une
fonctionnalité ou une dépendance, il est trop vague.

## Hiérarchie d'arbitrage

Quand deux objectifs entrent en tension, arbitrer dans cet ordre :

1. **Sécurité et conformité** : protéger personnes, données, secrets,
   responsabilité et confiance.
2. **Qualité** : exactitude, nuance, pédagogie, maintenabilité, auditabilité.
3. **Performance** : rapidité d'exécution, fluidité, coût, automatisation.
4. **Complétude** : volume de contenu, nombre de fonctionnalités, couverture.

La complétude ne justifie jamais un contenu non relu, une source faible ou une
collecte de données inutile.

## Principes produit

### P1 — Situation avant définition

Partir d'un cas concret avant d'introduire le concept. L'utilisateur apprend
mieux quand il doit décider dans une situation proche de son travail.

**Implique** : scénarios avec rôle, donnée, outil, contrainte et conséquence.

**Refuse** : questions abstraites qui testent seulement une définition.

### P2 — Nuance avant binaire

Reconnaître les réponses partielles, conditionnelles et risquées. Une bonne
pratique dépend souvent du contexte : outil interne ou externe, type de donnée,
politique d'entreprise, niveau d'impact.

**Implique** : scores partiels, feedbacks contextualisés, conditions explicites.

**Refuse** : vrai/faux simplistes sur des sujets à forte variabilité.

### P3 — Feedback avant score

Le score guide, mais le feedback enseigne. Une réponse sans explication est une
occasion d'apprentissage perdue.

**Implique** : chaque choix significatif explique risque, raison et action
recommandée.

**Refuse** : « correct », « incorrect » ou « bravo » sans justification.

### P4 — Confiance utilisateur avant engagement artificiel

L'engagement ne doit pas venir de la honte, de la rareté, du classement ou de la
peur. La confiance est plus importante que la rétention.

**Implique** : synthèse privée, pas de leaderboard nominatif, pas de chrono
punitif.

**Refuse** : vies, tickets, ligues, prix individuels, classement manager.

### P5 — Organisation apprenante, pas surveillance

L'organisation peut apprendre des agrégats ; elle ne doit pas surveiller des
individus sous couvert de formation.

**Implique** : agrégats par axe, seuils k-anonymes, rétention courte, notice
claire.

**Refuse** : export nominatif des erreurs, score global individuel transmis au
management.

## Principes éditoriaux

### E1 — Source avant affirmation forte

Une affirmation structurante doit être soutenue par une source adaptée ou marquée
comme incertaine. Une source faible peut inspirer une recherche, pas justifier
une correction publiée.

**Implique** : `source_gap` en brouillon, hiérarchie de sources, date de revue.

**Refuse** : source inventée, citation décorative, lien qui ne soutient pas le
claim.

### E2 — Action avant slogan

Une correction doit dire quoi faire : vérifier, anonymiser, choisir un autre
outil, escalader, documenter, refuser.

**Implique** : feedbacks orientés comportement.

**Refuse** : « soyez prudent », « utilisez l'IA de manière responsable » sans
action concrète.

### E3 — Contexte avant dogme

Une règle doit indiquer son périmètre. Les politiques internes, secteurs et
outils peuvent modifier la bonne réponse.

**Implique** : conditions visibles, exceptions explicites, référence à la
politique interne si nécessaire.

**Refuse** : prompt engineering présenté comme vérité universelle.

### E4 — Ambiguïté contrôlée, pas piège

Un scénario peut être ambigu pour entraîner le jugement, mais la correction doit
être défendable avec le contexte fourni.

**Implique** : ambiguïtés utiles, feedbacks expliquant les hypothèses.

**Refuse** : pièges lexicaux, informations manquantes indispensables.

### E5 — Biais traité, pas rationalisé

Un biais narratif ou visuel ne doit pas être défendu après coup. S'il n'est pas
nécessaire pédagogiquement et maîtrisé, il doit être supprimé ou bloqué.

**Implique** : revue biais/médias, justification d'usage, alternatives.

**Refuse** : stéréotypes, images IA humaines décoratives, représentations non
questionnées.

## Principes pédagogiques

### G1 — Transfert vers le travail réel

Un contenu est utile s'il aide l'utilisateur à agir hors du produit.

**Implique** : actions « dès demain », cas métier, fiches réflexes.

**Refuse** : trivia IA sans lien avec une décision professionnelle.

### G2 — Retrieval practice responsable

Faire rappeler ou appliquer une notion renforce l'apprentissage, mais il faut
éviter les questions de mémoire qui masquent le jugement.

**Implique** : répétition de risques sous différents contextes.

**Refuse** : répétition mécanique de définitions.

### G3 — Charge cognitive maîtrisée

Un scénario doit contenir assez d'information pour décider, pas assez pour noyer
l'utilisateur.

**Implique** : progression de difficulté, découpage, feedback clair.

**Refuse** : cas trop longs, multiples pièges simultanés au niveau débutant.

### G4 — Métacognition

Le produit doit apprendre à reconnaître ce que l'on sait, ce que l'on ignore et
ce qu'il faut vérifier.

**Implique** : réponses du type « je ne peux pas conclure », signalement
d'incertitude, escalade.

**Refuse** : forcer une certitude quand le contexte ne suffit pas.

## Principes sécurité/RGPD

### S1 — Minimisation par défaut

Ne collecter, stocker, transmettre ou afficher que ce qui est nécessaire.

**Implique** : pas de réponse libre sensible par défaut, pas d'email si inutile,
logs nettoyés.

**Refuse** : collecte « au cas où », analytics intrusifs, prompts utilisateurs
persistés sans besoin.

### S2 — Secrets et PII sont des blockers

Un contenu qui encourage l'exposition de secrets, données personnelles, logs ou
code sensible doit être bloqué.

**Implique** : gates sécurité, revue RGPD, exemples sûrs.

**Refuse** : « collez votre document/client/log dans l'outil IA » sans contrôle.

### S3 — Responsabilité humaine préservée

L'IA peut assister, pas porter seule une décision à impact.

**Implique** : validation humaine, escalade, documentation des limites.

**Refuse** : automatisation de décisions sensibles sans supervision.

### S4 — Transparence sur les données

L'utilisateur doit savoir ce qui est collecté, pourquoi, combien de temps et sous
quelle forme.

**Implique** : notices courtes, mode local, agrégats expliqués.

**Refuse** : collecte cachée, finalité RH implicite.

## Principes techniques

### T1 — Invariants métier dans le cœur

Les règles de scoring, validation, statuts, risques et revue appartiennent au
cœur métier, pas à une UI isolée.

**Implique** : Rust-first, tests, fixtures partagées.

**Refuse** : duplication de règles dans web/native.

### T2 — Content-as-data

Les questions, médias, sources et futurs concepts sont des données versionnées,
validées par schéma et auditables.

**Implique** : YAML/JSON, CLI, rapports, statuts.

**Refuse** : contenu codé en dur dans l'interface.

### T3 — Fail closed

En cas de doute sur source, statut, sécurité, média ou revue, ne pas publier.

**Implique** : `draft` par défaut, `blocked` si risque non résolu, erreurs
explicites.

**Refuse** : publication par défaut ou contournement silencieux.

### T4 — Preuve reproductible

Un changement doit laisser une commande ou preuve de vérification.

**Implique** : `cargo test`, validation corpus, rapports, diff lisible.

**Refuse** : « ça marche chez moi » sans preuve.

### T5 — Réversibilité

Préférer des changements petits, ciblés et réversibles, surtout sur modèles et
schémas.

**Implique** : migrations progressives, compatibilité, ADR pour rupture.

**Refuse** : refonte large non testée.

## Principes de souveraineté

### V1 — Local-first

Le flux nominal de développement, revue et validation doit fonctionner en local.

**Implique** : CLI, fixtures, localhost, pas de service externe obligatoire.

**Refuse** : dépendance obligatoire à un SaaS pour valider le corpus.

### V2 — Open source compatible

Les dépendances doivent respecter la politique licence et être maintenables.

**Implique** : licences MIT/Apache/MPL/BSD compatibles, justification des
exceptions.

**Refuse** : AGPL/SSPL ou licence incompatible sans décision explicite.

### V3 — Pas d'hyperscaler obligatoire

Le produit peut être déployé, mais ne doit pas dépendre d'un hyperscaler pour sa
fonction essentielle.

**Implique** : abstractions, self-hosting, déploiement séparé.

**Refuse** : fonctionnalités centrales impossibles hors plateforme propriétaire.

## Principes de contribution

### C1 — Brouillon par défaut

Un contenu nouveau commence en `draft`.

### C2 — Revue humaine obligatoire

Aucun prompt, modèle ou agent ne peut approuver un contenu.

### C3 — Petit diff, grande traçabilité

Les contributions doivent être relisibles, sourcées et testables.

### C4 — Déclarer l'assistance IA

L'assistance IA doit être assumée si elle influence contenu, sources ou
structure.

## Anti-principes

Ces pratiques doivent déclencher une revue ou un refus :

- récompenser la vitesse au détriment du jugement ;
- réduire la sécurité à une case à cocher ;
- présenter une sortie IA comme vraie sans vérification ;
- faire croire qu'un prompt peut remplacer une politique interne ;
- confondre sensibilisation et évaluation RH ;
- publier un contenu sans source ni justification ;
- créer des contenus en masse sans modèle de qualité ;
- masquer une incertitude ;
- utiliser un média synthétique humain comme décoration ;
- introduire une dépendance externe majeure pour gagner du temps ;
- collecter une donnée personnelle parce qu'elle pourrait être utile plus tard.

## Checklist de décision rapide

Avant d'accepter une fonctionnalité, un contenu ou une dépendance :

1. Protège-t-elle les données et personnes concernées ?
2. Améliore-t-elle la qualité de décision utilisateur ?
3. Est-elle sourcée, testable ou auditable ?
4. Préserve-t-elle l'absence d'usage RH implicite ?
5. Fonctionne-t-elle localement ou reste-t-elle self-hostable ?
6. Peut-on la retirer sans casser le produit ?
7. Le volume ajouté reste-t-il maintenable ?

Si la réponse à 1, 3 ou 4 est non, refuser ou bloquer.

## Critères d'acceptation

- Les principes sont applicables à une décision produit, éditoriale ou technique.
- Les anti-principes permettent de refuser un contenu ou une fonctionnalité.
- Les principes ne contredisent pas les ADR existantes.
- Les contributeurs peuvent les utiliser comme checklist de revue.
- La hiérarchie sécurité > qualité > performance > complétude est explicite.

## Statut de revue

Ce document a passé la revue humaine `foundation-review` et sert de référence
canonique pour la Vague A.
