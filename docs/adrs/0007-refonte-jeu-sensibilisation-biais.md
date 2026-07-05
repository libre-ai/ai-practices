# ADR 0007 — Refonte d'`apps/web` en jeu de sensibilisation aux biais IA

## Statut

Acceptée. **Supersède** le cadre « augment » du plan `2026-07-drill-formats.md` et **amende la posture d'affichage** d'ADR 0003 (autorise un positionnement anonyme non-compétitif).

## Contexte

Un prototype externe validé a prouvé le **message** et la **mécanique** d'un jeu de sensibilisation aux biais de l'IA générative : manifeste « aucune image générée n'est neutre / le problème n'est pas l'humain / la responsabilité est à l'entreprise », mécanique Vrai-Faux + identification de l'axe de biais, situations d'usage IA à juger, sessions de 50, feedback social non-compétitif.

`apps/web` a le **moteur** : cœur Rust multiplateforme, PWA offline, déployable single-origin, cohorte k-anonyme souveraine (ADR 0006), gouvernance content-as-code.

Le plan `2026-07-drill-formats.md` avait acté une adoption **« augment »** (ajouter les formats de drill au parcours pro gouverné). Décision produit, prise en connaissance de cause : `apps/web` **devient** ce jeu de sensibilisation — remplacement de l'expérience, pas ajout.

## Décision

- `apps/web` sert le **jeu de sensibilisation aux biais** : landing manifeste (écran 0), mécaniques `media_review` (image → biaisée/nuancé + axe dominant) et `single_choice` non-binaire (situations IA : OK / à surveiller / risqué), sessions de 50, registre engagé.
- Le **parcours pro** (30 questions gouvernées RGPD/sécurité/…) passe en **branche archive, réactivable**, sorti de l'app livrée. Le **moteur** (`crates/domain`, `session`, `scoring`, `ui`, cohorte, PWA) est **réutilisé**, non jeté.
- **Amendement d'ADR 0003** : la posture « pas d'affichage de cohorte / pas de gamification » est assouplie **pour ce produit** — un feedback **agrégé, anonyme, non-compétitif** (distribution, position vs cohorte) est autorisé. L'interdit de fond d'ADR 0003 **demeure** : pas de classement nominatif, pas d'évaluation RH, **pas de scoring dans l'UI** (le cœur Rust score).

## Règles

- Aucune règle métier ni scoring dans l'UI (ADR 0003 conservé sur ce point) ; l'UI affiche ce que le cœur calcule.
- La landing manifeste est **éditoriale** — elle ne calcule rien.
- Le parcours pro archivé reste `validate-corpus`-propre et **restaurable** (procédure documentée) ; l'archivage ne le supprime pas.

## Conséquences

Positives :

- réutilisation d'un moteur mûr (cohorte, PWA, souveraineté) pour un produit à message fort ;
- un seul produit cohérent, un seul backend ;
- le manifeste devient l'expérience, pas une note de bas de page.

Coûts :

- mise de côté d'un parcours pro testé et gouverné ;
- révision de gouvernance à tracer (cet ADR + 0008 + 0009) ;
- l'override doit être **ratifié avant tout code** (gate humaine I0 du plan de refonte).

## Alternatives rejetées

- **Augment (plan `drill-formats`)** : ajouter les drills au parcours pro — cohérent avec la gouvernance existante mais ne livre pas la vision produit du prototype.
- **Deux modes distincts** : parcours pro + jeu biais coexistant — double surface, dilue le message, alourdit la maintenance.
- **Nouveau repo/produit séparé** : duplique le moteur mûr (cohorte, PWA), viole la réutilisabilité (fonction de coût du programme).
