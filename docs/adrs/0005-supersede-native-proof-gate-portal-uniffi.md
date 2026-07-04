# ADR 0005 — Supersession du verrou natif d'ADR 0002 (portal-\*/UniFFI first-class)

## Statut

Proposée — en attente de ratification. Supersède partiellement **ADR 0002** (clauses de proof-gate natif : lignes 22, 28, 45) une fois acceptée.

## Contexte

L'objectif du produit est recadré : **construire d'excellents outils et des capacités réutilisables** ; la diffusion des `rumble-*` est un effet de bord. Le calendaire et la demande/marché sont explicitement **hors** des critères de décision — seuls valent **Sécurité > Qualité > Performance > Complétude**, plus un bonus de **réutilisabilité** (un choix qui produit un substrat réutilisé par d'autres produits vaut plus qu'un one-off).

ADR 0002 conditionne le natif à une « preuve produit / offline / auth ». Or « preuve produit » est un critère de **demande**, désormais disqualifié par l'objectif recadré.

Une revue multi-perspectives (4 lens, une par axe + réutilisabilité) tranche : sur **Qualité** (substrat réutilisable, discipline cœur/FFI/UI) et **Complétude** (vrai multiplateforme, natif par plateforme), la voie **portal-\*/UniFFI** (cœur Rust partagé + UI native SwiftUI/Compose) est supérieure. Les axes Sécurité et Performance sont quasi-neutres une fois retirés les critères interdits (« shipper vite », « pas de demande d'auth encore ») — le natif offrant même le stockage sécurisé plateforme (Keychain/Keystore) et un meilleur plafond de performance sur bas-de-gamme.

## Décision

Superséder les clauses de proof-gate natif d'ADR 0002 :

- le natif iOS/Android via **portal-\*/UniFFI** devient une voie **first-class**, buildable sans preuve produit préalable ;
- le cœur Rust (`domain`, `session`, `content`) reste l'**unique** source de logique métier, consommé par les shells natifs via **UniFFI** ;
- Dioxus reste le shell **web** (PWA), inchangé ;
- tout le reste d'ADR 0002 (Rust-first, aucun invariant métier dans l'UI, `wasm32` gate, tokens/a11y/i18n via Portal) demeure **en vigueur**.

## Règles

- Les shells natifs ne dupliquent **jamais** la logique métier : ils affichent des ViewModels produits par le cœur (comme la PWA — ADR 0003).
- Chaque shell natif a sa propre **DoD** : tests, build **signé**, accessibilité plateforme (VoiceOver / TalkBack), i18n, équivalent `prefers-reduced-motion`.
- `portal-*` fournit tokens, conventions d'a11y, i18n, adapters — pas de tokens en dur.
- `wasm32-unknown-unknown` reste un gate pour la PWA.

## Conséquences

Positives :

- substrat natif réutilisable par les `rumble-*` ;
- vrai multiplateforme (natif par plateforme, pas un webview partout) ;
- la discipline cœur / frontière UniFFI / UI native renforce la testabilité et la pureté du métier.

Coûts :

- chantier `portal-core` + shells natifs à construire ;
- frontière UniFFI à maintenir et tester ;
- surface de build par plateforme (signature, provisioning, keystore).

## Alternatives rejetées

- **dx-webview signé** : réutilise l'existant mais reste un compromis hybride (webview système), sans substrat natif réutilisable — perd sur Qualité et Complétude.
- **Superséder tout ADR 0002** : inutile ; seul le proof-gate contredit l'objectif recadré, le reste tient.
