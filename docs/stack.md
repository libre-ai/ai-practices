# Stack technique

## Décision recommandée

Adopter la trajectoire Rumble interactive : **Rust-first + Dioxus**.

| Couche | Choix | Pourquoi |
| --- | --- | --- |
| Domaine | Rust crates pures | déterminisme, tests, portabilité |
| Validation contenu | Rust + Serde + règles métier ; JSON Schema documenté | fail-closed, agent-readable |
| CLI | Rust + clap | premier client complet, facile pour agents légers |
| API | Axum + Tokio | robuste, self-hostable, cohérent écosystème |
| DB serveur | Hors MVP ; PostgreSQL + SQLx si persistance validée | éviter stockage prématuré |
| Local/offline | SQLite + SQLx, optionnel | desktop/mobile futur |
| UI | Dioxus | convergence Rumble, Rust/WASM, partage composants |
| Desktop | Tauri 2 après PWA | shell léger, Rust-friendly |
| Mobile | PWA d'abord, shell différé | éviter complexité app-store/native trop tôt |
| Auth | Hors MVP ; OIDC/Biscuit si usage organisationnel validé | éviter complexité avant gate humaine |
| Distribution | Build local + localhost + gate humaine | revue forte avant diffusion |
| IA runtime | aucune obligatoire en MVP | éviter dépendance externe et contenu non revu |

## Licences acceptées dans `deny.toml`

- MIT, Apache-2.0, BSD-3-Clause, ISC, Unicode-3.0.
- Zlib accepté uniquement comme licence permissive transitive utilisée par Dioxus (`const_format`, `slotmap`, `konst`).
- AGPL, SSPL, BSL restent interdites.

## Interdits par défaut

- AWS/GCP/Azure pour compute, storage, DB, AI.
- Firebase/Supabase/Vercel/Netlify/Auth0/Clerk.
- Google Fonts ou CDN tiers par défaut.
- OpenAI/Anthropic en runtime produit sans waiver explicite.
- Médias humains générés automatiquement pour décoration.
- Dépendances AGPL/SSPL/BSL.

## Dépendances Rust probables

À introduire uniquement quand le code arrive :

```toml
serde = { version = "1", features = ["derive"] }
serde_json = "1"
yaml_serde = "0.10" # fork maintenu de serde_yaml ; corpus YAML lisible par humains
schemars = "0.8"
jsonschema = "0.18"
thiserror = "2"
anyhow = "1"
clap = { version = "4", features = ["derive"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
axum = "0.8"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.8", default-features = false, features = ["runtime-tokio", "tls-rustls", "postgres", "sqlite", "uuid", "chrono", "json"] }
dioxus = { version = "0.7", default-features = false, features = ["macro", "html", "signals"] }
```

## Challenge stack

Ne pas créer desktop + mobile natif au démarrage. Pour un premier Rumble multi-plateforme, le bon ordre est :

1. Rust core testable.
2. CLI complet.
3. PWA mobile-first.
4. Desktop Tauri si besoin offline/local réel.
5. Mobile shell seulement si la PWA ne suffit pas.

Le piège serait d'appeler "multi-plateforme" une multiplication de shells vides. Le produit doit d'abord prouver ses invariants.
