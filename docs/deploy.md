# Déploiement & test local

L'app web (`apps/web`) est une PWA Dioxus 0.7.9 (WASM), mobile-first, installable.
Un seul cœur Rust ; le web est la surface diffusable maintenant, le natif suit
la voie `portal-*` (voir `multiplatform.md`).

## Single-origin local run

Run the full single-origin deployable (API + static web bundle) on one address:

```bash
cargo install dioxus-cli --version 0.7.9 --locked    # Dioxus CLI (note: not the homebrew `dx`/Deno)
dx build --platform web --release                     # Build wasm bundle to target/dx/…/release/web/public/
cargo run -p rumble-ai-practices-cli -- serve         # Serve on http://127.0.0.1:3000
```

Or use the provided justfile shortcut:

```bash
just serve-local
```

- Ouvrir `http://127.0.0.1:3000` dans le navigateur.
- API routes (`/v1/*`, `/healthz`, `/readyz`) are routed first.
- Unknown paths fall back to `index.html` (SPA routing).
- Service worker is served at `/sw.js` with the correct scope header.

## Tester en local (version mobile dev)

For quick iteration during development without rebuilding the full static bundle:

```bash
cd apps/web
~/.cargo/bin/dx serve --features web      # http://localhost:8080
```

- Ouvrir `http://localhost:8080` dans le navigateur.
- Mobile : DevTools → device toolbar (ou un vrai téléphone sur le même réseau
  via `--addr 0.0.0.0`, puis `http://<ip-machine>:8080`).
- Parcours : une porte d'onboarding (objectif, « pas une évaluation RH »,
  durée, données), une situation par axe de risque (issue du contenu réel), une
  synthèse privée par catégorie, un export
  JSON local.
- Chaque situation montre son **artefact** (pièce jointe façon keycap) plutôt
  que de le décrire : fichier `.docx` avec drapeau de sensibilité, liens-sources
  marqués « non vérifié », image générée par IA, ticket client à PII caviardée.
  Cadrage **hybride** : fil de discussion (collègue / IA qui répond) pour les
  échanges, scénario posé pour les autres. Un motif de catégorie en filigrane
  d'en-tête et en vignette de synthèse (bouclier / lien / portrait / enveloppe).
- Navigation clavier (listener global, actif sur tous les écrans) : `Entrée`
  lance / valide / continue / recommence selon l'écran, `1`–`4` sélectionnent,
  `Espace` = « je ne sais pas », `R` rejoue / recommence, `E` exporte la
  synthèse. Au tactile : taper un choix le sélectionne, un second appui sur le
  même choix valide (geste unique).
- À la validation, la réponse s'affiche **en place** (le choix reste épinglé, le
  feedback apparaît en fondu à la place des choix) — aucun reflux ni scroll.

## Builds par plateforme (un seul code Rust)

`scripts/build-all.sh [web|macos|ios|android|all]` construit chaque cible via
`dx --platform`. Statut vérifié localement (builds réels, non signés / debug
sauf web release) :

| Cible                   | Commande                             | Artefact                                                      | Statut                         |
| ----------------------- | ------------------------------------ | ------------------------------------------------------------- | ------------------------------ |
| **Web / PWA**           | `dx build --platform web --release`  | `…/release/web/public/` (728 K ; wasm **258 KiB gzip** ≤ 450) | ✅ diffusable                  |
| **macOS**               | `dx build --platform macos`          | `…/debug/macos/RumbleAiPracticesWebApp.app` (26 M)            | ✅ build                       |
| **iOS**                 | `dx build --platform ios`            | `…/debug/ios/RumbleAiPracticesWebApp.app` (22 M)              | ✅ build (Xcode)               |
| **Android**             | `dx build --platform android`        | `…/debug/android/app/` (SDK+NDK requis)                       | ✅ build                       |
| Windows / Linux desktop | `dx build --platform windows\|linux` | —                                                             | cross-compile (non buildé ici) |

Tous partagent le même `Root` et le même cœur Rust ; aucune réimplémentation du
scoring. Les builds natifs ici sont **debug et non signés** — la publication
store (App Store / Play Store) exige signature + provisioning, et la voie
premium native (SwiftUI/Compose) reste `portal-*` / UniFFI (voir
`multiplatform.md`). Le budget wasm (≤ 450 KiB gzip, profil release size-tuned)
est tenu : 258 KiB.

Android : le SDK/NDK doit être visible (`ANDROID_HOME`, `ANDROID_NDK_HOME`) — le
script les résout depuis `~/Library/Android/sdk`.

## Diffusion

### Web + PWA (maintenant)

Servir `public/` comme site statique (Clever Cloud app statique, `CC_WEBROOT`
pointant sur le `public/`, pattern `rumble-cos`). Sur HTTPS :

- **Android / Chrome** : installable via le manifest (`Ajouter à l'écran
d'accueil`), affichage `standalone`, icône Libre IA, `theme_color` Vert Libre.
- **iOS / Safari** : `Ajouter à l'écran d'accueil` via les tags
  `apple-mobile-web-app-*`.

**Offline** : `public/sw.js` est copié par `dx` à la racine du webroot afin que
le service worker couvre tout le shell. `assets/sw.js` reste la source suivie ;
les deux fichiers doivent rester identiques lors d'une évolution du cache.

### Desktop et mobile natifs

`scripts/build-all.sh macos|ios|android` produit les bundles natifs (Dioxus
enveloppe l'UI dans un webview système : wry sur desktop, WKWebView/Android
WebView sur mobile). Pour la publication :

- **Desktop** : signer le `.app` (macOS notarization) ; Windows/Linux se
  cross-compilent (`dx build --platform windows|linux`) ou se buildent sur la
  cible.
- **Mobile store** : signature + provisioning (iOS) / keystore (Android), puis
  Gear Cable pour le canal de release. La voie **premium native** (SwiftUI /
  Compose plutôt que webview) reste `portal-*` / UniFFI, différée par produit
  selon la demande réelle (voir `multiplatform.md`). Le cœur Rust reste unique
  quel que soit le shell.

## Invariants

- Aucune logique de scoring dans l'UI (ADR 0003) — les verdicts arrivent en
  données depuis le moteur de session.
- Apparence via tokens Portal uniquement (`assets/tokens.css`), zéro couleur en
  dur (ADR 0033 / ADR 0036).
- `prefers-reduced-motion` et navigation clavier respectés.
- Le verdict ne dépend jamais de la couleur seule : un glyphe l'accompagne
  (`✓ ≈ ⚠ ✗`, WCAG 1.4.1).

## Tests e2e (parcours réel)

Playwright pilote le parcours dans un vrai navigateur (chromium + viewport mobile
Pixel 7) — la version reproductible et gatée des vérifications manuelles :

```bash
cd apps/web && npm ci && npx playwright install chromium
npx playwright test            # réutilise un `dx serve` sur :8080 s'il tourne
```

Couvre : porte d'onboarding, lancement au clavier, geste tactile unique + bascule
en place sans scroll, navigation clavier (`1-4` / Entrée / Espace / `R`), « je ne
sais pas », parcours complet → synthèse par catégorie, export JSON. Gaté en CI
(`.github/workflows/e2e.yml`).

## Mesure (qualité observable)

- **Contraste** : `apps/web/tests/wcag_contrast.rs` vérifie que chaque encre de
  verdict atteint AA (≥ 4.5:1) sur les deux thèmes ; les valeurs miroir de
  `tokens.css` (à garder synchronisées).
- **a11y + budget perf** : `.github/workflows/a11y-perf.yml` lance axe-core et
  Lighthouse CI (`lighthouserc.json`) sur le build release — a11y ≥ 0.9 bloquant,
  CLS ≤ 0.1 bloquant (garde-fou du reflux en place), perf en avertissement.
- **RUM** : le délai sélection→validation est marqué côté client
  (`window.__raipDelays`, `localStorage['raip_delays']`) et **inclus dans
  l'export JSON** (`rum.select_to_validate_ms` + `rum.median_ms`) — la métrique
  de la direction (a) (« ralentir la révélation fait-il réfléchir ? »). Aucune
  donnée personnelle, local uniquement.
- **Chaîne de contenu (ADR 0003)** : l'app **embarque** le corpus réel
  (`content/questions/*.yml` via `include_str!`), le parse/valide (`content`),
  et fait dériver chaque verdict par le moteur (`session` : score →
  `EvaluationLevel` → `VerdictKind`). Le scoring reste dans le moteur, jamais
  dans l'UI. `apps/web/tests/engine_pipeline.rs` verrouille la chaîne ; le
  parcours est « une situation par axe de risque » (sélection dans
  `corpus()`, triviale à changer).
