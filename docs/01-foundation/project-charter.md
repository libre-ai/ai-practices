---
status: stable
owner: product-content
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ../vision.md
  - ../product-boundaries.md
  - ../content-governance.md
  - ../security-rgpd.md
  - ../adrs/0001-product-boundary.md
  - ../adrs/0003-content-governance-and-no-rh-scoring.md
  - ../adrs/0006-anonymity-and-retention-cohort.md
---

# Charte projet

## Résumé exécutif

`rumble-ai-practices` est un produit pédagogique audité qui entraîne des
réflexes professionnels d'usage de l'IA. Il ne cherche pas à mesurer une
performance individuelle, ni à certifier légalement une compétence. Il aide les
utilisateurs à prendre de meilleures décisions face à des situations concrètes :
données à protéger, sources à vérifier, limites à reconnaître, biais à éviter,
responsabilité humaine à maintenir.

Le cœur du projet n'est pas l'interface. Le cœur est un corpus versionné,
sourcé, relu et gouverné, adossé à une base de connaissances et à un modèle de
compétences observables.

## Mission

Construire une base de référence et un produit d'entraînement permettant à des
professionnels de mieux décider quand ils utilisent des systèmes d'IA, en
particulier l'IA générative, dans des situations de travail.

La mission se décline en quatre verbes :

1. **Comprendre** les capacités, limites et risques des systèmes IA.
2. **Reconnaître** les situations où l'usage de l'IA change le niveau de risque.
3. **Agir** avec des pratiques concrètes : minimiser, vérifier, documenter,
   escalader, refuser si nécessaire.
4. **Transférer** ces réflexes dans le travail quotidien, au-delà du quiz.

## Problème adressé

Les formations IA rapides et les quiz génériques créent souvent une illusion de
maturité : l'utilisateur apprend des slogans, des prompts ou des règles isolées,
mais ne sait pas arbitrer dans une situation professionnelle ambiguë.

Exemples de dérives que le produit combat :

- envoyer un document interne à un assistant IA externe parce qu'il est « destiné
  à être publié » ;
- croire qu'une citation générée par IA vaut vérification ;
- confondre anonymisation, pseudonymisation et suppression d'un nom ;
- appliquer un prompt magique plutôt qu'une politique de sécurité ;
- déléguer une décision sensible à une sortie IA plausible ;
- transformer un parcours de sensibilisation en classement RH.

## Théorie du changement

Si les utilisateurs sont exposés à des scénarios réalistes, reçoivent un
feedback nuancé, voient les sources et comprennent les conséquences de leurs
choix, alors ils développent des réflexes transférables : protéger les données,
vérifier, douter au bon moment, documenter et demander une revue humaine.

Le produit ne suppose donc pas qu'un bon apprentissage vient d'un gros volume de
questions. Il suppose qu'il vient d'une combinaison :

- situations proches du travail réel ;
- correction pédagogique actionnable ;
- répétition sur les axes de risque ;
- sources et limites visibles ;
- absence de pression compétitive nominative ;
- gouvernance éditoriale stricte.

## Publics utilisateurs

### Utilisateurs finaux

- collaborateurs qui utilisent des assistants IA pour texte, analyse, code,
  synthèse, traduction, recherche ou créativité ;
- équipes métier manipulant documents clients, contrats, données personnelles,
  informations internes, code ou décisions à impact ;
- managers qui doivent promouvoir un usage responsable sans transformer la
  formation en contrôle individuel.

### Utilisateurs organisationnels

- responsables formation et sensibilisation ;
- RSSI, DPO, conformité, juridique ;
- responsables transformation numérique ;
- directions métier souhaitant des signaux agrégés de maturité.

### Contributeurs

- auteurs de questions et scénarios ;
- relecteurs métier ;
- relecteurs sécurité/RGPD ;
- relecteurs biais/médias ;
- mainteneurs du corpus ;
- mainteneurs techniques.

## Ce que le projet est

- Un produit pédagogique audité.
- Un corpus de scénarios professionnels versionnés.
- Une base de connaissances sourcée sur les pratiques IA.
- Un cadre de compétences observables.
- Une chaîne de production de contenu avec revue humaine.
- Un produit self-hostable et local-first pour la revue.
- Un support de signaux agrégés, anonymes ou pseudonymisés selon le mode
  d'usage.

## Ce que le projet n'est pas

- Un outil RH de classement individuel.
- Une certification légale de compétence.
- Un clone de quiz compétitif.
- Un annuaire de prompts magiques.
- Un moteur RAG ou agentique générique.
- Une plateforme LMS complète.
- Un outil de surveillance des employés.
- Une dispense de formation métier, sécurité ou juridique.
- Une source d'avis juridique automatisé.
- Un produit dépendant d'un fournisseur cloud unique.

## Périmètre fonctionnel

### Inclus

- Parcours d'entraînement aux pratiques IA.
- Questions et scénarios contextualisés.
- Feedback pédagogique par choix et par axe de risque.
- Sources et preuves éditoriales.
- Audit de médias utilisés par le produit.
- Validation locale du corpus.
- Synthèse privée pour l'utilisateur.
- Agrégats anonymes ou k-anonymes pour l'organisation si le mode backend est
  activé.
- Exports de corpus ou d'artefacts pédagogiques si compatibles avec la
  gouvernance.

### Exclus

- Décisions RH individuelles.
- Leaderboards nominatifs.
- Chronos punitifs, vies, tickets ou mécaniques de rareté.
- Publication automatique de contenu généré par IA.
- Génération décorative de visages humains réalistes.
- Collecte de réponses libres sensibles par défaut.
- Envoi de corpus ou résultats vers des services tiers non maîtrisés.

## Actifs durables à produire

| Actif | Rôle |
| --- | --- |
| Taxonomie | Organiser les concepts IA professionnels. |
| Modèle de compétences | Décrire les comportements observables à entraîner. |
| Modèle de risques | Relier choix, conséquences et mesures de mitigation. |
| Graphe de connaissances | Relier concepts, sources, compétences, risques et contenus. |
| Politique de sources | Garantir la traçabilité et la qualité des références. |
| Guides éditoriaux | Rendre les contenus cohérents et maintenables. |
| Prompt library | Assister la production sans remplacer la revue humaine. |
| Schémas | Valider les contenus comme données. |
| Corpus validé | Fournir les questions, scénarios et fiches réellement utilisés. |
| Rapports d'audit | Prouver la qualité, les blocages et les décisions de revue. |

## Principes non négociables

1. La sécurité et la confidentialité passent avant la complétude pédagogique.
2. Une correction doit expliquer le raisonnement, pas seulement donner un score.
3. Une source citée doit être vérifiable ou explicitement interne.
4. Une question ambiguë doit être réécrite ou bloquée.
5. Les mesures d'organisation sont agrégées, anonymisées ou pseudonymisées selon
   les règles RGPD du produit.
6. La gamification ne doit jamais créer de honte, pression RH ou compétition
   nominative.
7. Une IA peut aider à rédiger, jamais approuver.
8. Les données personnelles et secrets sont évités par défaut.
9. Les médias synthétiques humains sont exceptionnels et audités.
10. Les décisions structurantes sont documentées.

## Gouvernance

### Rôles

| Rôle | Responsabilité principale | Pouvoir de blocage |
| --- | --- | --- |
| Mainteneur produit | Cohérence produit, priorités, frontières | oui |
| Mainteneur corpus | Statut final des contenus, cohérence éditoriale | oui |
| Auteur | Brouillon, intention pédagogique, sources initiales | non |
| Relecteur métier | Réalisme, vocabulaire, plausibilité | oui sur contenu métier |
| Relecteur sécurité/RGPD | PII, secrets, conformité, risques sécurité | oui |
| Relecteur biais/médias | Stéréotypes, représentations, audit média | oui |
| Relecteur pédagogique | Clarté, niveau, feedback, transfert | oui |
| Mainteneur technique | Schémas, validation, CLI, runtime | oui sur qualité technique |

### Règle de publication

Un contenu non relu est un contenu non publiable. Le passage en publication
requiert une preuve de revue humaine, même si une IA a participé à la rédaction.

### Décisions structurantes

Une décision doit être documentée dans une ADR ou dans le decision log si elle
modifie :

- la frontière produit ;
- le modèle de données ;
- la politique RGPD/sécurité ;
- le scoring ;
- la gouvernance de contenu ;
- les dépendances majeures ;
- la posture de souveraineté.

## Politique de licence et souveraineté

Le code du dépôt est sous licence MIT. Les dépendances doivent rester compatibles
avec la doctrine du projet : MIT, Apache-2.0, BSD, MPL-2.0 ou équivalent
acceptable ; AGPL, SSPL ou dépendances à usage incompatible sont interdites sans
décision explicite.

La trajectoire nominale est :

- build local reproductible ;
- validation locale du corpus ;
- self-hostable ;
- pas de dépendance obligatoire à un hyperscaler ;
- préférence pour des briques open source et souveraines ;
- déploiement éventuel séparé du modèle documentaire.

## Données et vie privée

### Par défaut

- Pas de nom/prénom requis.
- Pas d'email requis pour un parcours local.
- Pas de réponses libres sensibles par défaut.
- Pas de stockage nominatif de score.
- Pas de logs contenant PII, secrets, prompts utilisateur sensibles ou documents.

### En mode organisation

- Identifiant pseudonymisé si nécessaire.
- Agrégats par axe plutôt que score global nominatif.
- k-anonymat pour distributions de cohorte.
- Rétention courte et documentée.
- Transparence envers l'utilisateur sur les données utilisées.

## Critères de succès

### Pour l'utilisateur

- Il comprend les risques qu'il a correctement identifiés.
- Il sait quoi faire différemment demain.
- Il distingue règle générale, politique interne et cas contextuel.
- Il n'est ni humilié, ni classé publiquement, ni poussé à cacher ses erreurs.

### Pour l'organisation

- Elle obtient des signaux agrégés par axe de risque.
- Elle peut auditer le corpus et les médias.
- Elle peut prouver une gouvernance de contenu.
- Elle peut adapter les scénarios à ses politiques internes.
- Elle n'introduit pas de surveillance RH déguisée.

### Pour les mainteneurs

- Le corpus reste versionné et validable localement.
- Les sources et limites sont explicites.
- Les règles de refus sont connues.
- Les contributions sont petites, relisibles et réversibles.

## Risques principaux

| Risque | Description | Mitigation |
| --- | --- | --- |
| Risque éditorial | Corpus faux, dogmatique ou obsolète | revue humaine, sources, quality gates |
| Risque RGPD | Collecte ou exposition de PII | minimisation, anonymat, logs zéro PII |
| Risque RH | Score utilisé contre des individus | ADR no-RH, agrégats, UX privée |
| Risque sécurité | Incitation à exposer secrets ou code sensible | gates sécurité, prompts de revue, blocage |
| Risque biais | Scénarios ou médias stéréotypés | revue biais, audit média |
| Risque fournisseur | Dépendance à un outil ou cloud | self-hostable, open source, contrats abstraits |
| Risque de volume | Beaucoup de contenus peu maintenables | modèles, statuts, revue périodique |

## Métriques de pilotage acceptables

### Acceptables

- Nombre de contenus `approved` par axe.
- Âge moyen des revues de contenu.
- Nombre de blockers détectés avant publication.
- Couverture des compétences par scénarios.
- Distribution agrégée par axe si k-anonymat respecté.
- Taux de complétion d'un parcours sans données nominatives inutiles.

### Non acceptables

- Classement individuel public.
- Score global nominatif transmis au manager.
- Identification des personnes ayant échoué à un axe.
- Mesure de vitesse utilisée comme performance.
- Tracking comportemental non nécessaire.

## Critères d'acceptation de cette charte

- La mission et les non-objectifs sont compréhensibles par un contributeur
  externe.
- Les publics ne créent pas d'usage RH implicite.
- Les actifs à produire correspondent à la roadmap documentaire.
- Les règles de gouvernance préservent la revue humaine obligatoire.
- Les exigences souveraineté/RGPD sont compatibles avec les ADR existantes.
- Les critères de succès sont mesurables sans surveillance individuelle.

## Statut de revue

Ce document a passé la revue humaine `foundation-review` et sert de référence
canonique pour la Vague A.
