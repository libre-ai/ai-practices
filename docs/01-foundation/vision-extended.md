---
status: stable
owner: product-content
review_required: true
last_reviewed_at: 2026-07-05
related:
  - ../vision.md
  - ../product-boundaries.md
  - ../adrs/0003-content-governance-and-no-rh-scoring.md
  - ./project-charter.md
  - ./principles.md
---

# Vision étendue

## Thèse

La maturité IA en entreprise ne se mesure pas à la capacité de réciter des règles
de prompt. Elle se mesure à la capacité de prendre une décision responsable dans
un contexte donné : quelles données, quel outil, quelles sources, quel niveau de
risque, quel contrôle humain, quelle traçabilité et quelle limite d'usage.

`rumble-ai-practices` doit entraîner ce jugement. Le produit ne demande pas
« connais-tu l'IA ? ». Il demande : « dans cette situation professionnelle, quel
comportement réduit réellement le risque sans bloquer inutilement le travail ? ».

## Vision courte

Construire la base de connaissances et le produit de référence pour apprendre,
pratiquer et auditer les usages responsables de l'IA générative en contexte
professionnel, sans surveillance RH, sans contenu non relu et sans dépendance
obligatoire à un fournisseur externe.

## Pourquoi maintenant

L'IA générative est devenue accessible à grande échelle, mais les pratiques
organisationnelles restent instables :

- des collaborateurs utilisent des outils IA avant que les politiques internes
  soient claires ;
- des contenus plausibles mais faux circulent plus vite que les vérifications ;
- des données personnelles, secrets, logs ou documents internes peuvent être
  exposés par méconnaissance ;
- des directions veulent mesurer la maturité sans toujours distinguer formation,
  audit et surveillance ;
- les interfaces IA masquent parfois la complexité : outils, mémoire, agents,
  connecteurs, RAG, droits d'accès.

Le produit répond à ce moment en fournissant des scénarios pratiques et un corpus
maintenable plutôt qu'une simple liste de consignes.

## Problème approfondi

Les formations IA rapides produisent souvent quatre illusions.

### 1. Illusion de maîtrise

Croire qu'une bonne formulation suffit. Un prompt clair peut améliorer la sortie,
mais ne garantit ni vérité, ni conformité, ni confidentialité, ni absence de
biais.

### 2. Illusion de source

Croire qu'une citation affichée est vérifiée. Une IA peut inventer une source,
mal l'attribuer, citer une source réelle qui ne soutient pas l'affirmation ou
s'appuyer sur un document obsolète.

### 3. Illusion de conformité

Croire qu'une règle générale remplace l'analyse du contexte. La décision dépend
des données, de l'outil, de la politique interne, du secteur, du pays, du niveau
d'impact et des personnes concernées.

### 4. Illusion d'automatisation responsable

Croire qu'un agent ou un workflow automatisé reste sûr parce qu'il exécute une
intention humaine initiale. Les outils, permissions, documents récupérés et
sorties intermédiaires modifient le risque.

## Vision cible

Créer un référentiel ouvert et un produit d'entraînement qui rendent observables
les bons réflexes professionnels :

- ne pas exposer de secret ou donnée personnelle ;
- vérifier avant de réutiliser ;
- demander de l'aide à l'IA sans déléguer la responsabilité ;
- reconnaître l'incertitude et la documenter ;
- adapter l'outil et la donnée au niveau de risque ;
- contrôler les sources, médias et artefacts générés ;
- comprendre quand escalader vers sécurité, DPO, juridique ou métier ;
- refuser un usage quand la situation est trop risquée ou insuffisamment cadrée.

## Promesse utilisateur

À la fin d'un parcours, l'utilisateur doit pouvoir dire :

- « Je sais quelles données je ne dois pas envoyer. »
- « Je sais pourquoi une source générée n'est pas une preuve. »
- « Je sais comment vérifier une sortie avant de la réutiliser. »
- « Je sais quand demander une validation humaine. »
- « Je sais reconnaître les limites de mon propre jugement. »
- « Je sais quoi faire demain dans mon contexte. »

La promesse n'est pas de devenir expert IA. La promesse est d'améliorer la
qualité de décision dans les situations ordinaires où l'IA est utilisée.

## Promesse organisation

L'organisation obtient :

- un corpus gouverné, auditable et versionné ;
- des scénarios alignables avec ses politiques internes ;
- des signaux agrégés par axe de risque, non des notes individuelles ;
- une preuve de revue des contenus et médias ;
- une trajectoire self-hostable, local-first et compatible RGPD ;
- un support pour harmoniser sécurité, formation, DPO, juridique et métiers.

## Positionnement

`rumble-ai-practices` se situe entre :

- un quiz de sensibilisation, trop superficiel ;
- un LMS complet, trop large ;
- une formation technique IA, trop spécialisée ;
- un audit conformité, trop juridique ;
- un outil RH, explicitement hors périmètre.

Le produit est un entraînement aux pratiques : scénario, choix, feedback,
source, revue, progression.

## Différenciation

La différenciation ne vient pas du nombre de questions, mais de la qualité du
jugement entraîné.

| Dimension | Quiz classique | `rumble-ai-practices` |
| --- | --- | --- |
| Forme | Vrai/faux ou QCM isolé | scénario professionnel contextualisé |
| Correction | bonne/mauvaise réponse | risque, nuance, action, source |
| Corpus | texte édité ad hoc | données versionnées, schéma, audit |
| Mesure | score global | axes pédagogiques et synthèse privée |
| Organisation | classement possible | agrégats anonymes/k-anonymes |
| IA générative | producteur direct de contenu | assistant de brouillon, jamais approbateur |
| Média | décoration possible | média audité, biais contrôlé |
| Source | rarement traçable | claim relié à source ou source gap |

## Couches stratégiques

### Research Layer

Base de connaissances versionnée : concepts, sources, risques, compétences,
misconceptions, relations et niveau de confiance.

### Content Factory

Chaîne de production de contenus : prompts, brouillons, schémas, revue assistée,
gates qualité, validation humaine, rapports.

### Runtime Layer

Applications et API : session, scoring pédagogique, synthèse privée, offline,
backend de cohorte anonyme si activé.

La vision impose que le Runtime ne compense jamais une Research Layer faible ou
une Content Factory non gouvernée.

## Expérience cible

1. L'utilisateur reçoit un scénario réaliste.
2. Il choisit une ou plusieurs actions.
3. Le produit explique les conséquences : données, sources, sécurité, biais,
   responsabilité.
4. Le feedback propose une action concrète.
5. Une synthèse privée montre les axes à renforcer.
6. L'organisation ne voit que des signaux agrégés conformes à la politique de
   confidentialité.

## Scénarios exemplaires

Le produit doit couvrir des situations comme :

- faire relire un brouillon contenant des commentaires internes ;
- demander une synthèse d'un contrat client ;
- utiliser un assistant IA pour corriger du code contenant des secrets ;
- vérifier une affirmation produite avec une source plausible ;
- exploiter un RAG interne dont les documents sont obsolètes ;
- configurer un agent qui peut envoyer des emails ou modifier un ticket ;
- générer une image pour une communication interne ;
- résumer des retours utilisateurs contenant des données personnelles ;
- arbitrer entre rapidité, conformité et validation humaine.

## Horizon long

À terme, le projet doit pouvoir alimenter :

- une PWA d'entraînement ;
- des applications natives via cœur métier partagé ;
- des modules de formation internes ;
- des exports de corpus ;
- des tableaux de bord agrégés ;
- des revues de maturité par axe, sans notation nominative ;
- des packs de contenu spécialisés par secteur ou politique interne ;
- une veille documentaire intégrée au cycle de revue.

## Ce qui doit rester vrai à l'échelle

Quand le corpus passera de quelques dizaines à plusieurs milliers de ressources,
les invariants suivants doivent rester vrais :

- chaque contenu a un statut ;
- chaque contenu publié a une revue humaine ;
- chaque affirmation forte a une source ou une justification interne ;
- chaque média sensible est audité ;
- chaque compétence est observable ;
- chaque score est pédagogique ;
- chaque donnée personnelle collectée est justifiée, minimisée et documentée ;
- chaque dépendance importante est compatible avec la souveraineté du projet.

## Échecs à éviter

Le projet échoue si :

- il devient un quiz de culture générale IA ;
- il publie des contenus générés sans revue ;
- il invente ou tolère des sources fragiles ;
- il encourage implicitement l'envoi de données sensibles à des outils externes ;
- il produit un score exploitable comme note RH ;
- il sacrifie l'auditabilité au volume ;
- il dépend d'une plateforme fermée pour fonctionner ;
- il masque ses incertitudes.

## Implications produit

- Les parcours doivent privilégier les décisions contextualisées.
- Les feedbacks doivent être plus importants que le score.
- Les questions doivent être liées à des risques et compétences.
- Le mode organisation doit être conçu pour l'anonymat et la confiance.
- Les mécaniques compétitives doivent être rejetées sauf justification ADR.
- Les contenus doivent expirer ou repasser en revue selon leur domaine.

## Implications techniques

- Le corpus reste data-driven.
- Les invariants métier restent dans le cœur Rust.
- Le CLI reste un outil central de validation locale.
- Les schémas doivent évoluer avec la taxonomie, pas contre elle.
- Les applications clientes ne doivent pas réimplémenter les règles métier.
- Les logs et rapports ne doivent pas contenir de PII ou secrets.

## Critères d'acceptation

- La vision étendue complète `../vision.md` sans le remplacer tant qu'elle n'est
  pas `stable`.
- Les illusions et différenciateurs se traduisent en exigences éditoriales.
- L'horizon long reste compatible avec souveraineté, anonymat et no-RH-scoring.
- Les échecs à éviter sont suffisamment concrets pour guider une revue produit.
- Les couches Research / Content Factory / Runtime sont clairement séparées.

## Statut de revue

Ce document a passé la revue humaine `foundation-review` et sert de référence
canonique pour la Vague A.
