# Programme — ai-practices : outil pro multiplateforme, backend-adossé

> Roadmap stratégique (humaine). Se décompose en plans d'exécution
> `forge.plan.v0.1` par phase le moment venu. Complète
> `2026-07-ai-practices-convergence-prep.md` (portal-forge, contrat runtime lm,
> hygiène), qu'elle ne remplace pas.

## Objectif et fonction de coût

**Construire un excellent outil et des capacités réutilisables.** La diffusion
des `rumble-*` est un effet de bord, pas la contrainte.

Critères de décision valides, dans l'ordre : **Sécurité > Qualité > Performance >
Complétude**, plus un bonus de **réutilisabilité**. Sont **hors critères**
(interdits) : calendaire, demande/marché, MVP-vs-nice-to-have, « version
simplifiée », « trop ambitieux ».

## Décisions actées

| #   | Décision                                                                                                                                                                                                                                        | Référence                           |
| --- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------- |
| D1  | Natif via **portal-\*/UniFFI** (cœur Rust + UI SwiftUI/Compose), first-class ; supersession du proof-gate d'ADR 0002.                                                                                                                           | ADR 0005                            |
| D2  | Backend de cohorte **anonyme par conception** (session anonyme, k-anonymat, rétention, audit, zéro PII).                                                                                                                                        | ADR 0006 (opérationnalise ADR 0003) |
| D3  | **Build-local, deploy différé** : backend en qualité production tournant en local (Postgres docker) ; la **posture local-only** (pas d'infra payante, gouvernance écosystème) reste en vigueur ; le déploiement est un effet de bord ultérieur. | (ce plan)                           |
| D4  | **Anonymat conçu avant tout code backend** — l'ADR 0006 front-run la Phase 2.                                                                                                                                                                   | ADR 0006                            |
| D5  | Contenu relu/approuvé **en dernier** ; le filtre par statut `approved` existe déjà (garde fail-closed en place).                                                                                                                                | (ce plan)                           |

## Invariants (non négociables)

- Aucun invariant métier dans l'UI (web ou native) — le cœur Rust score, l'UI
  affiche (ADR 0003).
- Bounded context : une base par produit, aucun `*_ADDON_URI`/table croisé
  (garde-fou CI dédié).
- Zéro PII en logs ; tokens-only (aucune couleur en dur) ; `wasm32` gate PWA.
- Rien ne merge sans preuve verte (fmt, tests, clippy `-D warnings`, wasm32,
  deny, e2e, contrastes AA).

## Phases (séquencées par dépendances, sans calendrier)

### Phase 0 — Gouvernance

Ratifier ADR 0005 (supersession native) + ADR 0006 (anonymat). Ce sont les
préalables ; rien de structurel ne démarre avant.
**DoD** : les deux ADR mergés ; `decision-log` mis à jour ; ADR 0002 annoté
« proof-gate supersédé par 0005 ».

### Phase 1 — Contrats & schéma anonyme (conception, peu de code)

Concevoir le schéma DB anonyme et les contrats d'API (les routes `ApiRoutes`
existent déjà côté app) : session anonyme, soumission, synthèse, **distribution
k-anonyme**. Fixer `k` et le délai de rétention.
**DoD** : schéma + migrations écrits (pas encore branchés) ; contrats d'API
typés + fixtures partagées ; ADR 0006 respecté ligne à ligne ; tests de schéma
(propriétés d'anonymat) rouges-puis-verts en TDD.

### Phase 2 — Backend (local, qualité production)

Câbler `crate api` → service **axum** + **Postgres local** (docker). Implémenter :
scoring **côté serveur** (réutilise `crate session`), agrégation **k-anonyme**
(404 sous seuil), **rétention** (purge planifiée), **audit** d'accès, **puits
RUM** anonyme.
**DoD** : tests d'intégration prouvant l'anonymat **sous suppression** et **sous
seuil** ; zéro PII en logs vérifié ; `cargo deny`/gates verts ; docs API +
exemples ; **pas** de déploiement (D3).

### Phase 3 — App web ↔ backend

La PWA consomme le backend : fetch questions, soumet réponses, reçoit verdicts
**et** distribution de cohorte du serveur. Le corpus embarqué devient **fallback
offline** ; SW offline câblé à la racine. RUM POSté (anonyme).
**DoD** : e2e du parcours en mode connecté + mode offline (fallback) ; la
synthèse affiche un positionnement **réel** (ou « indisponible » sous seuil) ;
`private_distribution` n'est plus `None` ; gates + e2e verts.

### Phase 4 — Cœur UniFFI + shells natifs (portal-\*)

Exposer le cœur Rust via **UniFFI** ; construire les shells **SwiftUI** (iOS) et
**Compose** (Android) consommant ce cœur, stylés par **portal-\*** (tokens,
a11y, i18n). Builds **signés**.
**DoD** : chaque shell a ses tests + a11y plateforme (VoiceOver/TalkBack) +
build signé ; zéro duplication de logique métier (frontière UniFFI testée) ;
parité de verdict web/native prouvée par fixtures partagées.

### Phase 5 — Contenu

Relecture/approbation humaine du corpus (`review` → `approved`), largeur,
équilibre de difficulté. Activer le filtre par statut `approved` (déjà
supporté).
**DoD** : ≥ N questions `approved` par axe, relues et sourcées ; l'app ne sert
que de l'`approved` en cible de diffusion ; `maturity/rumble-ai-practices.json`
honnête.

### Effet de bord (n'importe quand après Ph.2) — Déploiement

Web statique + API sur Clever Cloud (addon Postgres, chiffrement-at-rest,
secrets), canal Gear Cable ; soumission stores pour le natif. **Décision
séparée**, non bloquante pour le programme.

## Dépendances

- Ph.0 → Ph.1 → Ph.2 → Ph.3 : strictement séquentiel (l'anonymat front-run).
- Ph.4 (natif) dépend de Ph.3 (contrats stables) mais peut démarrer le cœur
  UniFFI dès Ph.2.
- Ph.5 (contenu) indépendante ; parallélisable à tout moment, requise avant
  diffusion externe.
- Le déploiement dépend de Ph.2 (backend prêt) ; le reste est indépendant.

## Ce qui NE change pas

L'app actuelle (PWA content-driven, moteur, tests, e2e, revue) reste la base
vivante. Le corpus embarqué survit comme fallback offline. Portal web (Dioxus)
reste la voie web par défaut d'ADR 0002.
