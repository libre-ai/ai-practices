# ADR 0003 — Gouvernance de contenu, positionnement relatif privé et absence de scoring RH

## Statut

Acceptée.

## Contexte

Évaluer les connaissances IA de collaborateurs peut être utile pédagogiquement, mais dangereux si le score devient un outil RH implicite. Les quiz IA existants peuvent aussi produire de fausses certitudes.

## Décision

Le produit est un diagnostic pédagogique et un entraînement aux réflexes. Il ne produit pas de classement nominatif ni de décision RH.

Le produit peut afficher à l'utilisateur son **positionnement relatif privé** dans une distribution anonymisée, par axe pédagogique, si les seuils anti-réidentification sont respectés. Exemple : "vous êtes dans le groupe `autonomie prudente` sur l'axe sources & vérification ; 35 % des sessions anonymisées sont au-dessus, 42 % dans le même intervalle".

Toute question publiée doit être revue, sourcée et versionnée. Les résultats organisationnels sont agrégés par défaut.

## Conséquences

- UX orientée progression, pas jugement.
- Agrégats anonymisés et seuils de groupe.
- Positionnement relatif visible par l'utilisateur, jamais sous forme de classement public.
- Pas d'export manager nominatif en MVP.
- Toute demande de scoring RH nécessite nouvelle ADR, DPIA et validation explicite.

## Alternatives rejetées

- **Score global /20** : trop réducteur et socialement risqué.
- **Leaderboard nominatif** : favorise la comparaison sociale plutôt que l'apprentissage et expose les personnes.
- **Certification individuelle automatique** : qualité et responsabilité insuffisantes pour le MVP.
