# Stratégie de tests

## Priorité

1. Sécurité.
2. Qualité/invariants métier.
3. Performance suffisante.
4. Complétude fonctionnelle.

## Couches de tests

### Unitaires domaine

- Scoring positif/négatif.
- Réponses partielles.
- Erreurs critiques.
- Synthèse par axe.
- Statuts de publication.

### Validation corpus

- Champs obligatoires.
- Sources manquantes.
- Média non audité.
- Question `approved` sans reviewer.
- Choice sans feedback.
- Pondération invalide.

### Audit contenu

- Détection de corrections interdites connues.
- Liens de sources invalides ou non autorisés.
- Médias avec humains IA non justifiés.
- Questions trop binaires sans contexte.

### API

- Création session.
- Soumission réponse.
- Synthèse.
- Rate limit.
- Pas de fuite de pondération interne si non nécessaire.

### UI/PWA

- Parcours complet mobile viewport.
- Accessibilité clavier.
- Feedback affiché.
- Aucun token en storage navigateur.
- CSP.

### Sécurité/RGPD

- Logs sans PII.
- Export agrégé sans données nominatives.
- Suppression session.
- CSRF si auth.

## Tests golden

Le corpus doit produire des rapports déterministes :

```bash
rumble-ai-practices validate-corpus --content content/questions --schema schemas/question.schema.json
rumble-ai-practices audit-corpus --content content/questions --out reports/audit.json
rumble-ai-practices run-session --fixture fixtures/session-basic.json --out reports/session-summary.json
diff -u fixtures/expected-session-summary.json reports/session-summary.json
```

## Gates CI futures

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo deny check
cargo check --target wasm32-unknown-unknown
```

## Critères de non-régression éditoriale

Une MR qui ajoute une question doit prouver :

- question validée ;
- sources présentes ;
- feedback complet ;
- média audité ou absent ;
- pas de statut `approved` sans relecteur.
