# ADR 0010 — Pont navigateur typé sans `unsafe-eval`

- **Statut** : accepté
- **Date** : 2026-07-11

## Contexte

Le shell Dioxus utilisait `document::eval` pour la navigation clavier, le focus,
la mesure locale, l’export JSON, l’identifiant de cohorte et l’enregistrement du
service worker. La politique CSP devait donc autoriser `unsafe-eval`, qui ouvre
également `eval()` et `new Function()` à tout script même origine.

## Décision

Les interactions navigateur passent par un adaptateur Rust/WASM fondé sur
`web-sys`, `js-sys` et `wasm-bindgen` : écouteur clavier retenu, DOM typé, Blob,
stockage local, horloge Performance, Crypto et Service Worker. Les builds hôte,
SSR, desktop et mobile disposent d’adaptateurs sans effet.

Les métadonnées PWA sont statiques dans `apps/web/index.html`. Les assets CSS
restent déclarés via Manganis et sont injectés au build par Dioxus CLI ; aucun
composant `document::*` n’est rendu au runtime.

La CSP devient :

```text
default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self'; img-src 'self' data:; connect-src 'self'; object-src 'none'; base-uri 'self'; frame-ancestors 'none'
```

`wasm-unsafe-eval` autorise uniquement la compilation WebAssembly exigée par le
navigateur. Elle n’autorise ni `eval()` ni `new Function()`. Une gate CI interdit
la réintroduction de `document::eval` dans le code Rust.

## Conséquences

- La navigation clavier, le focus sans scroll, la métrique locale, l’export et
  le service worker gardent leur comportement sous CSP réelle.
- Les dépendances directes ajoutées sont déjà transitives et sous licences
  MIT/Apache-2.0 ; aucun service externe ni traitement de données nouveau.
- Le bundle Dioxus peut encore contenir du code d’évaluation non appelé issu de
  dépendances ; la CSP le bloque. Les E2E exécutent les interactions réelles avec
  `unsafe-eval` absent.
