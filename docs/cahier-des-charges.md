# Cahier des charges

## Publics

- Collaborateurs non experts IA.
- Managers et fonctions support.
- Profils techniques ayant besoin d'un rappel sécurité/RGPD.
- Référents IA internes qui relisent ou animent des parcours.

## Parcours MVP

1. L'utilisateur choisit un parcours : généraliste, données sensibles, vérification, biais média, usage métier.
2. Le produit affiche une situation concrète.
3. L'utilisateur choisit une ou plusieurs actions.
4. Le produit donne un feedback immédiat : correct, partiel, risqué, incorrect.
5. Le produit explique pourquoi, avec sources ou politique interne.
6. En fin de session, l'utilisateur reçoit une synthèse par axes de progression.
7. L'utilisateur peut voir son positionnement privé dans une distribution anonymisée si le seuil de cohorte est atteint.
8. L'organisation ne voit que des agrégats par groupe si activés.

## Axes pédagogiques

- Données, confidentialité et classification.
- Vérification, sources et hallucinations.
- Biais, représentations et médias générés.
- Sécurité opérationnelle : secrets, logs, code, fichiers.
- RGPD, droits des personnes, minimisation.
- Responsabilité humaine et décisions sensibles.
- Prompting utile mais non dogmatique.
- Usage métier et cadrage de contexte.

## Types d'activités

| Type | Usage | À éviter |
| --- | --- | --- |
| Scénario contextualisé | Cas principal | contexte artificiel ou moralisateur |
| Choix multiple pondéré | Plusieurs bonnes pratiques possibles | réponse unique si le réel est nuancé |
| Détection de risque | Données, média, prompt, source | piège basé sur une formulation ambiguë |
| Classement d'actions | Prioriser les bons réflexes | scoring opaque |
| Analyse de média | Biais, deepfake, consentement | juger l'authenticité d'une personne réelle |
| Fiche réflexe | Remédiation | slogan sans action concrète |

## Comportement attendu

- Toute correction doit expliquer le raisonnement.
- Toute norme doit citer une source ou une règle interne.
- Toute question doit être relue avant publication.
- Les erreurs partielles doivent être distinguées des erreurs dangereuses.
- Les médias IA doivent être identifiés, documentés et audités.
- Les résultats individuels doivent rester privés par défaut.
- Le positionnement relatif doit être affiché par distribution anonymisée, pas par classement nominatif.

## Comportement interdit

- "Le prompt demande des sources donc la vérification est incluse".
- "Le fichier est destiné à publication donc il ne contient pas de données sensibles".
- "Une méthode de prompt est une vérité universelle".
- "Image IA bizarre = deepfake ; image propre = authentique".
- "Bon score IA = bon collaborateur".

## Acceptation MVP

- Corpus initial : 30 questions validées minimum.
- 100 % des questions ont : intention, risque, explication, sources, date de revue, statut.
- 100 % des médias ont une fiche d'audit ou sont absents.
- CLI de validation fail-closed : un champ critique manquant bloque la publication.
- Une session complète fonctionne localement sans service externe.
- La synthèse utilisateur ne contient pas de jugement RH.
- La distribution de positionnement est privée, anonymisée et désactivée si cohorte trop petite.
