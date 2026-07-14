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

## Preuves locales vérifiées

| Preuve | Résultat |
| --- | --- |
| `./scripts/test-postgres-disposable.sh` | PASS, **78 tests** workspace, PostgreSQL jetable via socket temporaire |
| `cargo run -p rumble-ai-practices-cli -- validate-corpus --content content/questions` | validation corpus disponible localement |
| `cargo run -p rumble-ai-practices-cli -- validate-activities --activities content/activities` | validation activités disponible localement |
| `cargo run -p rumble-ai-practices-cli -- audit-corpus --content content/questions --media content/media --out reports/audit.json` | audit contenu disponible localement |
| `cargo run -p rumble-ai-practices-cli -- run-session --fixture fixtures/session-basic.json --content content/questions --media content/media --out reports/session-basic.json` | synthèse pédagogique locale sur fixture |
| `cargo run -p rumble-ai-practices-cli -- serve --bind 127.0.0.1:3000` | API + PWA locales, smoke via `/readyz`, `/manifest.webmanifest`, `/sw.js` |

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
| Résultat pédagogique | **prouvé local/CI** | synthèse locale sans score individuel automatique | le produit rend une synthèse pédagogique, pas un score RH individuel |
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
| Staging | **later** | `docs/local-review.md` et `docs/deploy.md` refusent le staging distant comme gate nominal | pas de staging cloud comme preuve produit |
| Production ops | **later** | même doctrine | aucune opération prod ne fait partie de la readiness locale |
| Alpha publique | **later** | non revendiquée | ne pas confondre cockpit local et diffusion publique |
| E2E navigateur comme preuve de readiness | **later** | non retenu ici | pas de prétention à une preuve browser E2E pour ce cockpit |

## Gates

| Gate | Attendu | État | Verdict |
| --- | --- | --- | --- |
| **P0** | état local runnable, corpus validé/audité, fixture session, API/PWA locale | **prouvé local/CI** | le socle local existe et passe |
| **P1** | contenu humain `approved`, pilote privé, runtime partagé borné | **blocked** | absence d'`approved` et runtime partagé non prouvé |
| **P2** | opérations, release, diffusion | **later** | hors périmètre de readiness actuelle |

## Conclusion

Le produit a un **socle local solide**, mais il n'est **pas ready** :
P0 est prouvé, P1 est bloqué, P2 est à venir.

**0 issue ouverte != readiness.**
