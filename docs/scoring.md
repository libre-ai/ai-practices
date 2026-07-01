# Scoring pédagogique

## Principe

Le scoring mesure des réflexes pratiques, pas une valeur professionnelle. Il doit aider l'utilisateur à progresser et à se situer, pas à être classé publiquement.

## Niveaux recommandés

- **Découverte** : identifie certains risques mais manque de méthode.
- **Pratique encadrée** : applique les bons réflexes dans les cas simples.
- **Autonomie prudente** : sait contextualiser, vérifier, refuser si nécessaire.
- **Référent usage IA** : raisonne par risques, sources, gouvernance et transmission.

## Axes de synthèse

- Données & confidentialité.
- Sources & vérification.
- Hallucinations & limites.
- Biais & représentations.
- Sécurité opérationnelle.
- RGPD & vie privée.
- Responsabilité humaine.
- Contexte métier.

## Règles

- Pas de note globale publique.
- Pas de leaderboard nominatif.
- Positionnement relatif privé autorisé, par distribution anonymisée et seuil minimum de cohorte.
- Pas de rang individuel exact si la cohorte est trop petite.
- Pas de seuil automatique "apte/inapte".
- Les erreurs critiques peuvent être signalées, mais toujours avec remédiation.
- Une réponse partielle doit être reconnue comme telle.
- Le niveau final doit inclure des recommandations actionnables.

## Pondération

Chaque choix peut avoir :

```text
+1.0 bonne pratique essentielle
+0.5 bonne pratique utile mais insuffisante seule
 0.0 neutre / dépend du contexte
-0.5 erreur mineure
-1.0 erreur risquée
-2.0 erreur critique : secret, PII, décision automatisée sensible, deepfake non signalé
```

## Sortie utilisateur

Exemple :

```text
Niveau : Pratique encadrée
Positionnement privé : vous êtes dans le même intervalle que 42 % des sessions anonymisées sur ce parcours.
Distribution : Découverte 18 % · Pratique encadrée 42 % · Autonomie prudente 31 % · Référent 9 %

Vos points forts :
- Vous identifiez bien les risques de sources inventées.
- Vous pensez à demander le contexte métier.

À renforcer :
- Ne déduisez pas la confidentialité d'un fichier à partir de son nom.
- Vérifiez les liens fournis par une IA : existence, contenu, date, autorité.

Prochaine fiche : Vérifier une source citée par IA.
```

Le positionnement doit rester :

- privé pour l'utilisateur ;
- affiché par intervalles, pas en rang exact ;
- disponible uniquement si la cohorte anonymisée atteint un seuil minimal ;
- explicité comme indicateur pédagogique, pas comme preuve de compétence professionnelle.

## Sortie organisation

Uniquement si activée :

```text
Cohorte : équipe support — 42 sessions
Distribution globale : Découverte 18 % · Pratique encadrée 42 % · Autonomie prudente 31 % · Référent 9 %
Axe le plus maîtrisé : contexte métier
Axe à renforcer : données & confidentialité
Aucune donnée nominative exportée
```
