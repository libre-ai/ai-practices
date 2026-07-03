# Déploiement & test local

L'app web (`apps/web`) est une PWA Dioxus 0.7.9 (WASM), mobile-first, installable.
Un seul cœur Rust ; le web est la surface diffusable maintenant, le natif suit
la voie `portal-*` (voir `multiplatform.md`).

## Tester en local (version mobile)

```bash
cd apps/web
~/.cargo/bin/dx serve --features web      # http://localhost:8080
```

- Ouvrir `http://localhost:8080` dans le navigateur.
- Mobile : DevTools → device toolbar (ou un vrai téléphone sur le même réseau
  via `--addr 0.0.0.0`, puis `http://<ip-machine>:8080`).
- Navigation : `A`/`B`/`C`/`D` sélectionnent, `Entrée` ou **Valider** valident,
  `Espace` = « je ne sais pas », `R` rejoue. Au tactile : taper un choix puis
  **Valider**.

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
d'accueil`), affichage `standalone`, icône keycap, `theme_color`.
- **iOS / Safari** : `Ajouter à l'écran d'accueil` via les tags
  `apple-mobile-web-app-*`.

**Offline (follow-up)** : le service worker (`assets/sw.js`) n'est pleinement
actif que servi à la **racine** (`/sw.js`) pour couvrir tout le shell. Sous
`dx`, les assets sont hachés sous `/assets/`, donc câbler l'offline est une
étape de déploiement : copier `sw.js` à la racine du webroot, ou poser l'en-tête
`Service-Worker-Allowed: /`. Sans cela l'enregistrement dégrade proprement et
l'app reste installable.

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
