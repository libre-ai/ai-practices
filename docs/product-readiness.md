# Cockpit local de readiness

_Canonique au 2026-07-14 · snapshot `main@41302e7`_

- Maturité officielle : **Dojo**
- Disponibilité : **discovery**

Ce cockpit documente ce qui est **prouvé localement/CI** au snapshot courant, et ce qui reste **absent, partiel ou bloqué**. Il ne remplace pas une décision de readiness.

## Légende

- **prouvé local/CI** : vérifié par une commande reproductible.
- **implemented-unhosted** : implémenté dans le repo, sans preuve d'exploitation hébergée.
- **partial** : une partie du besoin existe, mais pas le gate complet.
- **blocked** : le gate attendu ne peut pas être levé aujourd'hui.
- **later** : hors périmètre de readiness immédiate.

## Commandes et preuves du dépôt

La suite PostgreSQL a été rejouée pour cette mise à jour. Les autres lignes sont les chemins canoniques documentés et couverts par le dépôt ; elles ne constituent pas une preuve d'hébergement.

| Preuve | Résultat / limite |
| --- | --- |
| `./scripts/test-postgres-disposable.sh` | PASS rejoué : **78 tests** workspace, PostgreSQL jetable via socket temporaire |
| `cargo run -p rumble-ai-practices-cli -- validate-corpus --content content/questions` | validation corpus locale, couverte par les tests |
| `cargo run -p rumble-ai-practices-cli -- validate-activities --activities content/activities` | validation activités locale, couverte par les tests |
| `cargo run -p rumble-ai-practices-cli -- audit-corpus --content content/questions --media content/media --out reports/audit.json` | audit contenu local, couvert par les tests |
| `cargo run -p rumble-ai-practices-cli -- run-session --fixture fixtures/session-basic.json --content content/questions --media content/media --out reports/session-basic.json` | synthèse pédagogique locale sur fixture |
| `cargo run -p rumble-ai-practices-cli -- serve --bind 127.0.0.1:3000` | chemin single-origin API + PWA documenté ; aucune URL hébergée prouvée |

## Content / editorial

| Zone | État | Preuve | Lecture |
| --- | --- | --- | --- |
| Corpus questions | **partial** | `content/questions` contient **374 draft** et **30 review**, **0 approved** | le corpus est gouverné et vérifiable, mais rien n'est publiable sans revue humaine finale |
| Activités | **blocked** | `content/activities/core.yml` contient **3 activités**, toutes `draft` | les activités reconstruites existent seulement comme brouillons ; aucune activité `approved` |
| Validation / audit | **prouvé local/CI** | commandes CLI ci-dessus + tests de corpus | la chaîne éditoriale est contrôlée localement, mais cela ne vaut pas publication |

## Session / API / PWA

| Zone | État | Preuve | Lecture |
| --- | --- | --- | --- |
| Fixture session | **prouvé local/CI** | `fixtures/session-basic.json` + `run-session` | le parcours pédagogique local est reproductible |
| API locale | **implemented-unhosted** | `serve` + smoke localhost | l'API et les routes de base existent en local |
| PWA locale | **implemented-unhosted** | `manifest.webmanifest`, `sw.js`, tests `apps/web/tests/*` | le shell web est présent et consommable localement |
| Parcours navigateur mobile | **prouvé local/CI** | `apps/web/e2e/parcours.spec.ts`, `.github/workflows/e2e.yml` | Playwright vérifie le parcours sur Chromium mobile ; cela ne prouve aucun hébergement |
| Résultat pédagogique | **prouvé local/CI** | synthèse locale sans score individuel automatique | le produit rend une synthèse pédagogique, pas un score RH individuel |
| Wrappers macOS/iOS/Android | **partial** | builds locaux documentés dans `docs/deploy.md` | artefacts debug/non signés ; aucune publication store ni convergence plateforme revendiquée |
| Runtime partagé de session | **blocked** | absent du chemin vérifié | aucune preuve de runtime partagé borné pour un pilote privé |

## Data / security

| Zone | État | Preuve | Lecture |
| --- | --- | --- | --- |
| Suite de tests | **prouvé local/CI** | `./scripts/test-postgres-disposable.sh` | la suite complète tourne avec PostgreSQL jetable, sans base durable |
| Données personnelles | **prouvé local/CI** | `docs/security-rgpd.md` + tests UI/export | pas de token en storage, export local sans PII attendue |
| Inspection DB | **implemented-unhosted** | manifest d'inspection DB et workflow associé | la classification est cadrée, mais ce n'est pas un signal de readiness produit |
| Revue médias | **partial** | corpus et audits présents | le contrôle médias existe, mais ne suffit pas à lever les gates éditoriaux |

## Operations

| Zone | État | Preuve | Lecture |
| --- | --- | --- | --- |
| Staging | **blocked** | aucun hôte ni runbook staging exécuté au snapshot | le dépôt documente une diffusion possible, mais aucune preuve hébergée n'est fournie |
| Production ops | **later** | aucune preuve opérationnelle | aucune opération prod ne fait partie de la readiness actuelle |
| Alpha publique | **later** | non revendiquée | ne pas confondre cockpit local et diffusion publique |
| E2E navigateur | **prouvé local/CI** | Playwright Chromium mobile + workflows E2E | preuve du parcours local uniquement, pas de staging ni de vrai appareil |

## Gates

| Gate | Attendu | État | Verdict |
| --- | --- | --- | --- |
| **P0** | état local runnable, corpus validé/audité, fixture session, API/PWA locale et parcours navigateur CI | **prouvé local/CI** | le socle local existe et passe |
| **P1** | contenu humain `approved`, pilote privé, runtime partagé borné | **blocked** | absence d'`approved` et runtime partagé non prouvé |
| **P2** | opérations, release, diffusion | **later** | hors périmètre de readiness actuelle |

## Conclusion

Le produit a un **socle local solide**, mais il n'est **pas ready** :
P0 est prouvé, P1 est bloqué, P2 est à venir.

**0 issue ouverte != readiness.**
