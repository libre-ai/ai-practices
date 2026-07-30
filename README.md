# AI Practices

Entraînement professionnel à une pratique sourcée et responsable de l'IA — ni quiz, ni scoring RH (couche 1).

Pour les professionnelles et professionnels qui utilisent l'IA au travail, qui rencontre des formations superficielles ou des questionnaires RH déguisés, ce projet permet de s'entraîner à une pratique sourcée et responsable de l'IA, en produisant des parcours d'entraînement vérifiables, sans notation cachée, sans dépendre de : aucun scoring RH, aucune donnée transmise.

## État du projet

<!-- libre-ai:project-status:begin -->
<!-- Section générée depuis project.v1.yaml — ne pas éditer à la main. -->

- Situation actuelle : L'application Practices (garde no-transmission, fonctionnement local-only) est greffée et verte sur les briques épinglées ; les parcours d'entraînement couvrant le référentiel de pratique restent à écrire.
- Maturité : usable
- Exposition : spec-published
- Confiance : medium
- Preuves vérifiées le : 2026-07-30
- Avancement : 20 % du périmètre actuellement déclaré

<!-- libre-ai:project-status:end -->

## Vérifier

- `bun install && bun run check` — la chaîne de gates du dépôt, tests inclus.
- La fiche [`project.v1.yaml`](./project.v1.yaml) est l'autorité de l'état du projet ; la section « État du projet » ci-dessus en est générée et un gate de flotte échoue si elles divergent.
- La provenance de chaque chemin migré depuis le hub est tracée dans l'index de migration de `libre-ai/libre-ai` (`ecosystem/migration-index.v1.yaml`).
