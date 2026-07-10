# Plan — Refonte apps/web en jeu Rumble de sensibilisation aux biais IA

```yaml
format: forge.plan.v0.1
kind: planning_request
source:
  product: rumble-ai-practices
  plan_id: plan-2026-07-rumble-bias-game-refonte
  created_at: "2026-07-05"
execution_policy:
  planning_only: false
  allow_execution: true
  requires_human_approval_for_execution: false
  human_gates:
    - "I0: ratification humaine des ADR superséders (0007/0008/0009) — bloque I1-I7"
    - "I1: bias_review par média réutilisé (relecteur biais) — bloque la mise en corpus"
traceability:
  - "prototype externe validé : repo rumble-ai-benchmark, prototypes/rumble-v1 (landing manifeste + play : V/F+axe, situations IA, sessions de 50)"
  - "docs/plans/2026-07-drill-formats.md — mapping des formats sur single_choice non-binaire / media_review (réutilisé, cadre 'augment' superséded)"
  - "docs/adrs/0003 (pas de scoring RH/leaderboard) — amendé : positionnement anonyme non-compétitif autorisé"
  - "docs/adrs/0004 (revue biais médias) — amendé : corpus synthétique droits-acquis autorisé sous disclosure + bias_review"
  - "docs/adrs/0006 (anonymat + rétention cohorte k-anon) — étendu au per-item"
  - "branche archive/phase3-cohort-endpoint — apps/web/e2e/cohort.spec.ts + .github/workflows/cohort-e2e.yml à porter"
depends_on: []
blocks: []
open_questions:
  - "Discipline média : bias_review complet par image (982) vs échantillon audité + le reste en status:draft masqué du corpus livré. Défaut : draft masqué, audit progressif."
  - "Rétention per-item : réutiliser DEFAULT_RETENTION_DAYS (90j) ou distinct. Défaut : réutiliser 0006."
risks:
  - id: R1
    severity: high
    description: "Override de gouvernance : refonte produit + réutilisation d'assets tiers + amendement de 3 ADR. Sans ratification humaine en I0, le reste est une violation silencieuse."
    mitigation: "I0 = gate humaine, ADR superséders écrits et ratifiés AVANT tout code (I1-I7 bloqués)."
  - id: R2
    severity: high
    description: "982 médias synthétiques, dont des visages humains IA (cas ADR 0004). Diffuser sans disclosure/audit reproduit le biais qu'on prétend combattre."
    mitigation: "disclosure synthétique obligatoire + bias_review par image livrée ; les non-audités restent status:draft, exclus du corpus servi (gate fail-closed existant)."
  - id: R3
    severity: medium
    description: "Budget PWA wasm ≤ 450 KiB gzip (ADR 0002). Les 982 webp ne s'embarquent pas dans le wasm."
    mitigation: "assets servis en statique (single-origin) + cache SW SÉLECTIF (pas de precache des 982) ; référencés par les drills, chargés à la demande."
  - id: R4
    severity: medium
    description: "Session engine 'gelé' (convergence-prep) tant que rumble-lm ne publie pas son runtime ; la cohorte per-item ajoute du serveur."
    mitigation: "la cohorte est un service anonyme séparé (extension 0006, déjà sur main), pas une extension du store de session ; aucune croissance du shim /v1/sessions*."
evidence_expectations: "chaque incrément = PR avec exit gates commandés ; aucune revendication sans sortie de commande. Rien ne merge sans preuve verte (fmt, check, test, clippy -D warnings, wasm32, deny, e2e, contrastes AA)."
```

## Contexte

Le prototype externe (repo `rumble-ai-benchmark`, `prototypes/rumble-v1`) a **prouvé le message et la mécanique** : un jeu de sensibilisation aux biais de l'IA générative, avec le manifeste « aucune image générée n'est neutre / le problème n'est pas l'humain / la responsabilité est à l'entreprise », une mécanique V/F + identification de l'axe de biais, des situations d'usage IA à juger, des sessions de 50, et un feedback social non-compétitif.

`apps/web` a **le moteur** : cœur Rust multiplateforme, Dioxus PWA offline, déployable single-origin, cohorte k-anonyme souveraine (ADR 0006), gouvernance content-as-code, discipline de tests.

**Décision (override conscient de la gouvernance, prise en connaissance de cause)** : `apps/web` **devient** ce jeu de sensibilisation. Ce n'est pas l'adoption « augment » qu'avait actée `2026-07-drill-formats.md` (drills ajoutés au parcours pro) : c'est un **remplacement** de l'expérience produit, le parcours pro passant en archive réactivable. Cet override amende trois ADR ; il est **tracé et ratifié en I0** avant tout code — c'est la condition qui distingue l'override assumé de la violation silencieuse.

Ce qui **n'est pas** overridé, par choix de prudence (coût nul, risque réel couvert) : la **discipline média** (disclosure synthétique + `bias_review` par image) et le **caractère non-compétitif** (agrégat anonyme k-anon, ni streak ni classement).

## Décisions actées

| #   | Décision                                                                                                                                                                                                                                            | Réf. / supersession                                                                |
| --- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| DA1 | `apps/web` devient le jeu de sensibilisation aux biais ; le parcours pro (30 questions gouvernées) passe en branche archive, **réactivable**, sorti de l'app livrée.                                                                                | ADR 0007 (nouveau) ; supersède le cadre « augment » de `2026-07-drill-formats.md`  |
| DA2 | Réutilisation du corpus synthétique droits-acquis (les 982 visuels du prototype), **hébergé souverain**, chaque image **signalée synthétique** + `bias_review` rempli avant mise en corpus servi.                                                   | ADR 0008 (nouveau) ; amende ADR 0004                                               |
| DA3 | Feedback social **per-item** (« X% ont jugé cette image non biaisée ») = agrégat **anonyme k-anon**, extension de la cohorte 0006 ; positionnement de fin de session en **quartiles + courbe de distribution**. Ni streak, ni classement nominatif. | ADR 0009 (nouveau) ; étend ADR 0006, amende la posture d'affichage d'ADR 0003      |
| DA4 | Mécaniques portées sur le modèle **existant** : `media_review` (image → biaisée/nuancé + axe dominant) et `single_choice` non-binaire (situations IA : OK / à surveiller / risqué). Reframe « tout output IA est un tirage → un biais ».            | `2026-07-drill-formats.md` (mapping réutilisé) ; `crates/domain` `InteractionType` |
| DA5 | Sessions de **50** items, mécaniques alternées ; contenu = **tous** les types du prototype (bias-visual, situations, statements, profiles, deepfakes), les plus bruts en `status: draft`.                                                           | (ce plan)                                                                          |
| DA6 | Manifeste = **couche éditoriale** : landing (écran 0) + cadrage des feedbacks.                                                                                                                                                                      | (ce plan)                                                                          |

## Invariants (non négociables — repris du repo, amendés là où l'override l'exige)

- **Souveraineté** : hébergement des médias et de la cohorte dans la stack souveraine (Clever Cloud) ; aucun hôte non souverain.
- **Zéro PII** : cohorte per-item strictement anonyme, k-anonyme, non-nominative, rétention bornée (ADR 0006/0009). Zéro PII en logs.
- **Cœur Rust score, l'UI affiche** (ADR 0003 conservé sur ce point) : aucune règle métier dans l'UI ; le scoring reste dans `crates/session`/`crates/scoring`.
- **Discipline média** (ADR 0004 amendé, pas aboli) : tout média = provenance + statut + disclosure synthétique + `bias_review` ; non-audité ⇒ `status: draft` ⇒ exclu du corpus servi (fail-closed).
- **Tokens-only** (aucune couleur en dur), **wasm32 gate**, **budget wasm ≤ 450 KiB gzip** (ADR 0002).
- **Rien ne merge sans preuve verte** : fmt, check, test, clippy `-D warnings`, wasm32, `cargo deny`, e2e, contrastes AA.
- **Bounded context** : une base par produit, aucun `*_ADDON_URI`/table croisé.

## Réutilisé vs remplacé

| Réutilisé (l'ingénierie)                                                                    | Remplacé / archivé                                  |
| ------------------------------------------------------------------------------------------- | --------------------------------------------------- |
| Cœur Rust, `crates/domain` (`InteractionType`, `media_review`), session engine, `crates/ui` | Parcours pro (30 YAML) → `archive/`, sorti de l'app |
| Dioxus PWA offline, single-origin (axum sert le wasm), service worker                       | `IntroGate` pro → landing manifeste                 |
| Cohorte k-anon souveraine (ADR 0006, sur `main`), rétention, purge                          | Sélection « 1 question/axe » → sessions de 50       |
| Tokens, a11y, gates CI, `cargo deny`                                                        | Registre « pro/sobre » → registre manifeste         |

## Incréments

### I0 — Gouvernance : ADR superséders + ce plan (gate humaine, verrouille I1-I7)

Fichiers : `docs/adrs/0007-refonte-jeu-sensibilisation-biais.md`, `docs/adrs/0008-reuse-medias-synthetiques-droits-acquis.md`, `docs/adrs/0009-cohorte-per-item-k-anon.md`, ce plan, `docs/README.md` (index ADR).

Travail : écrire chaque ADR (Statut : Proposée → Acceptée après ratification humaine) — contexte, décision, ce qu'elle supersède/amende, conséquences, alternatives rejetées. 0007 : pivot produit + amende la posture d'affichage 0003. 0008 : amende 0004 (corpus droits-acquis sous disclosure+audit). 0009 : étend 0006 au per-item anonyme.

Exit gates : 3 ADR commitées, cohérence croisée relue (aucun code), ratification humaine (flip Statut → Acceptée).

### I1 — Schéma de contenu drills + import + pipeline assets

Dépendances : I0. Fichiers : `docs/templates/bias-drill-template.yml` (nouveau, dérivé de `media-review-template.yml`), `content/questions/bias-visual.yml` + `situations.yml` + (draft) `statements.yml`/`profiles.yml`/`deepfakes.yml`, fiches `content/media/*.yml` (disclosure + `bias_review`), assets webp dans `apps/web/assets/media/` (ou `content/media/`, servi statique), `crates/content` (validation étendue si besoin).

Travail : convertir les drills JSON du prototype vers le YAML gouverné (mapping `bias-visual → media_review`, `situations → single_choice` non-binaire) ; importer les 982 webp souverains ; remplir `bias_review` sur le lot audité (le reste `status: draft`, exclu). Reframe « tout est biaisé » dans les libellés/feedbacks.

Exit gates : `validate-corpus` 0 blocker ; les images servies ont toutes `synthetic_disclosed: true` + `bias_review.decision != rework` ; budget assets vérifié (pas d'embed wasm).

### I2 — Moteur : scoring `media_review` non-binaire + reframe

Dépendances : I1. (Le mapping `media_review` est déjà prévu par `drill-formats.md` I2 ; si `crates/scoring` n'est pas extrait, implémenter dans `crates/session`, dette explicite.)

Travail : TDD — `media_review` : décision qualitative (biaisé / nuance « diversité forcée = artefact ») → score déterministe + axe dominant ; `single_choice` non-binaire pour situations (OK/surveiller/risqué). Logique « la bonne réponse à 'est-ce biaisé ?' est toujours 'oui, mais…' ».

Exit gates : `cargo test --workspace` ✓ ; `cargo clippy --workspace -- -D warnings` ✓ ; tests rouges→verts dans la PR ; `run-session --fixture fixtures/session-bias-visual.json` scores attendus.

### I3 — UI Dioxus : landing manifeste + console de drill + sessions de 50

Dépendances : I1, I2. Fichiers : `apps/web/src/lib.rs` (landing, console drill, sessions de 50, interleave), `crates/ui/src/lib.rs` (`BiasDrillCard` : hero image, gen-prompt, V/F + axe), CSS tokens.

Travail : `IntroGate` → landing manifeste (le déclic, « le problème n'est pas l'humain », coulisses) ; console : image hero + « à partir d'un prompt comme … » + V/F puis identification d'axe ; situations en bulle chat ; tirage de 50 items alternés, compteur X/50 ; démarrage direct (pas de sas redondant avec la landing) ; clavier 1-9 + Entrée (ADR 0002, API `Event<KeyboardData>` **à vérifier docs.rs** avant écriture).

Exit gates : `cargo test -p rumble-ai-practices-ui` ✓ (SSR rend `<img alt>`) ; `cargo check --target wasm32-unknown-unknown -p rumble-ai-practices-web --features web` ✓ ; contrastes AA ; parcours clavier revu.

### I4 — Backend cohorte per-item k-anon (TDD, étend 0006)

Dépendances : I0 (ADR 0009). Fichiers : `crates/store` (table `item_answers(item_id, choice_id, count)` + `distribution_for_item` + purge rétention), `crates/api` (endpoint per-item, extension de `/v1/cohort` ou `/v1/item-cohort`), `crates/domain` (types réponse : `ItemDistribution { min_cohort_size_met, buckets[], user_bucket }`).

Travail : TDD rouge→vert — masquage sous k (`DEFAULT_MIN_COHORT`), idempotence par `client_id` opaque, rétention, zéro nominatif. Réutilise le pattern `insert_session`/`distribution` de 0006.

Exit gates : `cargo test --workspace` ✓ (sous-k masqué, ≥k exposé, idempotent) ; `cargo clippy -D warnings` ✓ ; aucun champ nominatif dans le schéma.

### I5 — Câblage crowd : feedback per-item + positionnement de fin

Dépendances : I3, I4. Fichiers : `apps/web/src/lib.rs` (POST per-item après chaque réponse → barre + phrase solidaire ; fin de session : quartiles + courbe de distribution via l'axe-cohorte 0006), `crates/ui` (`CrowdBar`, `DistributionCurve`).

Travail : après chaque choix, « 62 % ont jugé cette image non biaisée — le piège fonctionne » (masqué sous k, dégradation offline gracieuse) ; fin de session : quartiles + courbe. Ton solidaire, jamais « tu es meilleur que X ».

Exit gates : build wasm32 ✓ ; offline : la synthèse locale rend seule (pas de blocage) ; k-anon respecté à l'affichage.

### I6 — Archivage du parcours pro (réactivable) + routing

Dépendances : I3. Travail : sortir les 30 YAML pro + le cadrage parcours de l'app livrée (déplacés/documentés, restaurables depuis `archive/`), IntroGate/routing pointent la landing manifeste. Aucun chemin mort dans l'app livrée.

Exit gates : `validate-corpus` sur le corpus servi ✓ ; app ne sert que le jeu biais ; procédure de réactivation documentée.

### I7 — e2e + CI + prep deploy

Dépendances : I3-I5. Fichiers : porter `apps/web/e2e/cohort.spec.ts` + `.github/workflows/cohort-e2e.yml` depuis `archive/phase3-cohort-endpoint` (vérifier les sélecteurs contre le nouveau DOM), + drill e2e (un item de chaque mécanique au clavier, feedback crowd, offline).

Exit gates : e2e verts (single-origin binaire + Postgres) ; CI cohort-e2e passe ; parcours PWA manuel OK.

## Hors scope

- **Déploiement** (« publier ») — chantier séparé après preuve produit (exécution locale par défaut, cf. `docs/security-rgpd.md`).
- **Mécaniques compétitives** : chrono, vies, ligues, multiplicateurs, classement nominatif — restent rejetées (on n'en a pas besoin).
- **Convergence session engine → rumble-lm** — le shim reste gelé ; ce plan n'y touche pas.
- **Natif iOS/Android** — différé (ADR 0002/0005).

## Vérification end-to-end du chantier

```bash
cargo fmt --all --check && cargo check --workspace --all-targets && \
cargo test --workspace && cargo clippy --workspace -- -D warnings
cargo run -p rumble-ai-practices-cli -- validate-corpus --content content/questions   # 0 blocker
cargo run -p rumble-ai-practices-cli -- run-session --fixture fixtures/session-bias-visual.json
cargo check --target wasm32-unknown-unknown -p rumble-ai-practices-web --features web
# ADR superséders ratifiés :
grep -l "Statut" docs/adrs/0007-*.md docs/adrs/0008-*.md docs/adrs/0009-*.md
# Corpus servi : toutes images disclosed + auditées
# Parcours manuel PWA : cargo run -p ...-cli -- serve → 127.0.0.1:3000
#   → landing manifeste → 50 items alternés au clavier → feedback crowd par item → quartiles de fin
```
