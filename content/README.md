# Corpus `rumble-ai-practices`

Ce dossier contient les données pédagogiques versionnées du produit.

## Statut du lancement corpus

Le corpus structuré démarre en **draft-first** : les nouveaux artefacts produits
par la Content Factory sont non publiables tant qu'ils n'ont pas passé la revue
humaine prévue par `docs/04-content-factory/review-process.md`.

## Arborescence

```text
content/
  sources/              sources structurées
  claims/               affirmations vérifiables
  concepts/             taxonomie opérationnelle
  risks/                risques pédagogiques
  competencies/         compétences observables
  misconceptions/       croyances erronées à corriger
  learning-objectives/  objectifs pédagogiques observables
  learning-paths/       parcours et remédiations
  graph/                relations entre objets
  questions/            questions compatibles avec le schéma existant
  media/                médias et audits média
  audit/                audits d'existant
```

## Règles

- Tout nouvel artefact commence en `draft`.
- Les questions servies aux utilisateurs doivent être `approved`.
- Aucun contenu généré par IA n'est publiable sans revue humaine.
- Aucun média humain synthétique n'est utilisable sans audit.
- Les sources doivent soutenir des claims précis.
- Les données personnelles, secrets, logs et tokens réels sont interdits dans les
  exemples.

## Lots actuels

- `content/questions/corpus-seed.yml` : 11 questions seed en `draft`.
- `content/questions/batch-001.yml` : 29 questions supplémentaires en `draft`.
- `content/concepts/core.yml` + `content/concepts/batch-001.yml` : concepts de base et extension en `draft`.

## Vérifications utiles

```bash
cargo run -p rumble-ai-practices-cli -- validate-corpus --content content/questions
cargo run -p rumble-ai-practices-cli -- audit-corpus --content content/questions --media content/media --out reports/audit.json
```

Les dossiers `sources/`, `claims/`, `concepts/`, `risks/`, `competencies/`,
`misconceptions/`, `learning-objectives/`, `learning-paths/` et `graph/` posent
le socle du futur modèle étendu. Leur validation CLI dédiée reste à implémenter.
