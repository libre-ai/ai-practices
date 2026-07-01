# Paquet 09 — Desktop et mobile

## Objectif

Évaluer les shells desktop/mobile seulement après PWA prouvée.

## Desktop Tauri

Conditions d'entrée :

- core stable ;
- PWA utilisable ;
- besoin offline/local validé ;
- stratégie SQLite définie ;
- sécurité secrets validée.

## Mobile shell

Conditions d'entrée plus strictes :

- PWA insuffisante documentée ;
- besoin app-store ou notifications ;
- budget maintenance accepté ;
- auth WebView prouvée ;
- privacy review mise à jour.

## Non-objectifs immédiats

- SwiftUI/Compose natif.
- Trois interfaces divergentes.
- Notifications push avant preuve pédagogique.
- Collecte offline de données sensibles.

## Acceptation

Une ADR dédiée est obligatoire avant tout code desktop/mobile natif.
