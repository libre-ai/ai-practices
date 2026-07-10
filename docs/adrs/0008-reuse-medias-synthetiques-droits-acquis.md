# ADR 0008 — Réutilisation de médias synthétiques droits-acquis pour l'analyse de biais

## Statut

Acceptée. **Amende ADR 0004** (revue biais médias) pour le cas d'usage « analyse de biais » du jeu de sensibilisation (ADR 0007).

## Contexte

ADR 0004 refuse par défaut les médias humains générés par IA dans le corpus, **sauf** cas « analyse d'un média synthétique » (média signalé synthétique + grille d'audit biais remplie + relecteur approuve), et met **hors-scope** la réutilisation d'assets de source tierce (propriété, visages réels, hébergement non souverain).

Le jeu de sensibilisation (ADR 0007) repose sur un corpus de **visuels synthétiques droits-acquis** dont l'objet pédagogique est précisément d'**apprendre à repérer le biais dans ces images** — soit exactement le cas d'usage que 0004 autorisait déjà sous conditions.

## Décision

- Réutilisation du **corpus synthétique droits-acquis** autorisée pour le jeu, **hébergé souverain** (single-origin, aucun hôte tiers).
- **Discipline média conservée** (0004 amendé, pas aboli) : chaque image **servie** porte `synthetic_disclosed: true` **et** une fiche `bias_review` remplie (stéréotypes détectés, risques de représentation, décision, relecteur). Les non-audités restent `status: draft`, **exclus du corpus servi** (fail-closed existant).
- Les **droits** sont documentés **hors repo** (archive de provenance = preuve de droits) ; le livrable ne porte **aucune trace de la source** (marque, URLs, noms de fichiers qui trahiraient la réponse).

## Règles

- Aucun média servi sans disclosure synthétique **et** `bias_review.decision != rework`.
- Provenance et statut par média ; gouvernance `draft→review` maintenue (le gate `approved` est relâché pour la piste sensibilisation, cf. ADR 0007).
- Souveraineté d'hébergement **obligatoire** (pas de bucket/hyperscaler tiers).

## Conséquences

Positives :

- volume de contenu immédiat pour un message éprouvé ;
- la discipline d'audit **sert la pédagogie** : on enseigne le biais sur des cas concrets, audités et assumés comme synthétiques.

Coûts :

- coût d'audit `bias_review` par image livrée (l'audit est la condition de mise en corpus) ;
- obligation de disclosure ; gestion de la provenance et des droits hors repo.

## Alternatives rejetées

- **Interdire toute réutilisation (0004 strict)** : prive le jeu de son corpus prouvé, sans gain de neutralité.
- **Réutiliser sans disclosure ni audit** : reproduit le biais qu'on prétend combattre ; risque droit à l'image et stéréotype.
- **Régénérer un corpus original ex nihilo** : coûteux et sans garantie de meilleure neutralité — tout output génératif est un tirage, donc un biais (c'est le propos même du jeu).
