# Revue locale et gate humaine

## Position

`rumble-ai-practices` ne cible pas de déploiement automatique sur une plateforme cloud dans son flux nominal.

La distribution MVP est :

```text
build local -> lancement localhost -> revue humaine forte -> décision explicite
```

Aucun environnement distant ne doit devenir une étape implicite de validation produit.

## Pourquoi

Le risque principal du produit n'est pas seulement technique. Il touche :

- qualité pédagogique ;
- biais média ;
- interprétation RH possible ;
- confidentialité ;
- confiance utilisateur.

Ces points exigent une revue humaine explicite avant toute diffusion.

## Commandes de build local

```bash
cargo fmt --all --check
cargo check --workspace
cargo check -p rumble-ai-practices-web --target wasm32-unknown-unknown
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo deny check
cargo run -p rumble-ai-practices-cli -- validate-corpus --content content/questions --media content/media
cargo run -p rumble-ai-practices-cli -- audit-corpus --content content/questions --media content/media --out reports/audit.json
cargo build --release -p rumble-ai-practices-cli
```

## Lancement localhost

```bash
./target/release/rumble-ai-practices serve --bind 127.0.0.1:3000
```

Puis ouvrir :

```text
http://127.0.0.1:3000
```

## Smoke localhost

```bash
curl -fsS http://127.0.0.1:3000/healthz
curl -fsS http://127.0.0.1:3000/readyz
curl -fsS http://127.0.0.1:3000/manifest.webmanifest
curl -fsS http://127.0.0.1:3000/sw.js
curl -fsS http://127.0.0.1:3000/v1/catalog
curl -fsS http://127.0.0.1:3000/ | grep "Diagnostic pédagogique"
curl -fsS http://127.0.0.1:3000/ | grep "Demander des sources ne suffit pas"
```

## Gate humaine obligatoire

Avant toute diffusion, une personne responsable doit valider :

- [ ] le build local est reproductible ;
- [ ] les gates techniques sont verts ;
- [ ] le serveur tourne en localhost ;
- [ ] `/` affiche les composants Dioxus SSR attendus ;
- [ ] le parcours ne fuit pas de score/réponse côté API publique ;
- [ ] les headers sécurité sont présents ;
- [ ] le corpus affiché correspond au périmètre validé ;
- [ ] aucune donnée personnelle réelle n'est requise ;
- [ ] aucun classement nominatif n'est visible ;
- [ ] les médias IA référencés ont une revue ;
- [ ] le rapport d'audit contenu est lu ;
- [ ] la décision de diffusion est écrite.

## Décision de revue

Format recommandé :

```text
Date :
Version / commit :
Relecteur technique :
Relecteur contenu :
Commandes exécutées :
Résultat smoke localhost :
Réserves :
Décision : go / no-go
```

## Non-objectifs

- Pas de déploiement automatique.
- Pas de staging cloud par défaut.
- Pas de dépendance à une plateforme externe pour valider le produit.
- Pas de mode production tant que la gate humaine n'est pas documentée.
