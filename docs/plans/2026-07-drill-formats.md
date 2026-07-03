# Plan — drill-formats (2026-07 wave)

```yaml
format: forge.plan.v0.1
kind: planning_request
source:
  product: rumble-ai-practices
  plan_id: plan-2026-07-drill-formats
  created_at: "2026-07-03"
execution_policy:
  planning_only: false
  allow_execution: true
  requires_human_approval_for_execution: false
  human_gates:
    - "I1: promotion corpus review→approved (relecture finale humaine, bloque I2-I7)"
traceability:
  - "docs/benchmark-jeu-competitif.md — Familles A-E, backlog P1-P3"
  - "docs/adrs/0003 (pas de leaderboard/scoring RH), docs/adrs/0004 (revue biais médias)"
  - "schemas/question.schema.json — interaction types déclarés vs évaluation moteur (écart media_review/ranked_actions)"
  - "docs/plans/2026-07-ai-practices-convergence-prep.md — I4 (extraction crates/scoring)"
depends_on:
  - "plan-2026-07-ai-practices-convergence-prep (I4) — requis pour l'incrément moteur I2"
blocks: []
open_questions:
  - "ranked_actions : top-1 (MVP retenu) vs classement complet drag & drop (suivi ultérieur)"
risks:
  - id: R1
    severity: medium
    description: "APIs Dioxus 0.7 non vérifiées (Event<KeyboardData>, storage gloo-storage vs web-sys) ; prescrire sans vérifier violerait la règle empirique."
    mitigation: "Vérification docs.rs + licence cargo-deny obligatoire avant d'écrire I6/I7 ; marquées hypothèses dans les incréments."
  - id: R2
    severity: high
    description: "La gate humaine I1 (30 questions review→approved) conditionne tout le chantier ; les audits 101-105 sont fixed mais reviewer: TBD."
    mitigation: "I1 fournit l'évidence par question ; la relecture finale et le remplissage des reviewers restent humains."
evidence_expectations: "chaque incrément = PR avec exit gates commandés ; aucune revendication sans sortie de commande"
```

## Contexte

Une veille externe a restitué les cinq formats de drill de l'jeu compétitif de référence (jeu compétitif de référence)
et proposé un « prompt de reconstruction » ciblant Next.js + Prisma + admin CRUD, mécaniques
compétitives incluses. Ce plan remplace ce prompt : le tri des mécaniques est déjà ratifié
(`docs/benchmark-jeu-competitif.md`), la stack existe (workspace Rust + Dioxus 0.7), et le
seul delta réel est l'adoption des **formats de présentation de drill** (Famille E de la note
de benchmark) sans le moteur compétitif.

Verdict sur le prompt externe :

| Élément du prompt                                | Verdict                  | Raison                                                                                                      |
| ------------------------------------------------ | ------------------------ | ----------------------------------------------------------------------------------------------------------- |
| Stack Next.js + Prisma + DB + admin CRUD         | REJETÉ                   | Stack et gouvernance content-as-code existantes (corpus YAML validé en CI, workflow draft→review→approved). |
| Vies, chrono, multiplicateur, ligues, classement | REJETÉ (déjà arbitré)    | Benchmark Familles A/D + ADR 0003 ; ne pas re-litiger.                                                      |
| « Générer 15-20 questions »                      | REJETÉ                   | 30 questions sourcées existent ; la croissance passe par la gouvernance éditoriale.                         |
| Framework mnémotechnique de prompting            | ADAPTÉ                   | `prompt_dogma` est bloquant ; devient fiche réflexe « aides, pas garanties » (axe `prompt_practice`).       |
| « Ça dépend », tags de biais, feedback nuancé    | DÉJÀ PRÉSENT (supérieur) | Scoring non-binaire, `risks[]`, sources obligatoires, feedback par choix.                                   |
| Les 5 formats de drill                           | À PRENDRE/ADAPTER        | Voir Famille E du benchmark ; mapping ci-dessous.                                                           |

## Mapping des formats sur le modèle existant

| Format source                           | Interaction (schéma existant)                              | Delta à construire                                                         |
| --------------------------------------- | ---------------------------------------------------------- | -------------------------------------------------------------------------- |
| Affirmation à évaluer + illustration    | `single_choice` non-binaire (« ça dépend » scoré 0.0/+0.5) | Variante de présentation UI + média SVG                                    |
| Évaluation d'image générée (biaisée/OK) | `media_review`                                             | Scoring + UI + médias audités ; nuance « diversité forcée = artefact »     |
| Détection deepfake                      | `media_review`                                             | Idem + pédagogie « la vérification de source prime sur les tells visuels » |
| Revue de prompt en UI chat              | `ranked_actions` (MVP top-1)                               | Scoring + UI chat, transcription dans `context.assets`                     |
| MCQ                                     | `single_choice`                                            | Rien (existant)                                                            |

## État du code (vérifié 2026-07-03)

- `crates/domain/src/lib.rs:96-101` : `InteractionType` a les 4 variantes ; struct `MediaReview`
  présente avec `contains_human_like_person` (ADR 0004).
- `crates/session/src/lib.rs:61-142` `submit_answer()` : scoring uniforme ; l.149 traite
  `MediaReview` comme `SingleChoice` — aucune évaluation spécifique `media_review`/`ranked_actions`.
- `crates/ui/src/lib.rs` : `QuestionCard` (l.78), `FeedbackPanel` (l.95), `SummaryPanel` (l.110) —
  pas d'affichage média, pas de clavier.
- `apps/web/src/lib.rs:39-53` : `TrainingFlowModel` sans localStorage ni clavier ;
  `apps/web/assets/` ne contient que tokens.css + styles.css ; `content/media/` vide.
- Corpus : 30 questions toutes en `status: review` (MVP « 30 validées » non atteint) ;
  audits `content/audit/pilot-vs-cos.yml` 101-105 tous `fixed`, `reviewer: TBD`.
- Contrainte benchmark : aucune mécanique codée tant que le MVP corpus n'est pas atteint
  → I1 verrouille I2-I7.

## Incréments

### I0 — Docs : addendum benchmark Famille E + backlog P1/P2 (PR: drill-formats-docs)

Sans dépendance. Fichiers : `docs/benchmark-jeu-competitif.md` (Famille E),
`docs/ux.md` (modes, clavier, présentation chat), `docs/scoring.md` (meilleur score sans
régression), `docs/product-boundaries.md` (non-objectifs importés), `docs/security-rgpd.md`
(stockage local + checklist conformité), ce plan.

Exit gates : docs commitées, cohérence croisée relue (aucun code).

### I1 — Corpus MVP : promotion review→approved (gate humaine, verrouille I2-I7)

Fichiers : `content/questions/*.yml` (review.reviewers, last_reviewed_at, status),
`content/audit/pilot-vs-cos.yml` (reviewer à remplir).

Travail : vérifier question par question que chaque correctif d'audit 101-105 est reflété
dans le contenu ; relecture finale humaine (relecteur métier + biais) ; flip
`status: review → approved` ; remplacer `reviewer: TBD`.

Exit gates :

```bash
cargo run -p rumble-ai-practices-cli -- validate-corpus --content content/questions  # 0 blocker
cargo run -p rumble-ai-practices-cli -- audit-corpus --content content/questions     # 0 blocker
grep -c "status: approved" content/questions/*.yml   # total = 30
```

### I2 — Scoring `media_review` + `ranked_actions` (TDD, dans crates/scoring)

Dépendances : I1 + convergence-prep I4 (extraction `crates/scoring`). Repli si I4 retardé :
implémenter dans `crates/session` et migrer à l'extraction (dette explicite, à arbitrer).

Travail : tests d'abord — `media_review` : décision qualitative → score déterministe ;
`ranked_actions` top-1 : action prioritaire scorée +1.0, hiérarchie de risque
PII > sources > contexte > envoi ; `validate_choice_count` spécialisé par type.

Exit gates : `cargo test --workspace` ✓ ; `cargo clippy --workspace -- -D warnings` ✓ ;
tests rouges→verts documentés dans la PR.

### I3 — Extension corpus : questions `media_review`/`ranked_actions` + fiches média

Dépendances : I1 (gouvernance), I2 (le moteur sait les évaluer).

Fichiers : `content/questions/media.yml` (+2 `media_review`, dont nuance « diversité
forcée = artefact possible »), `content/questions/deepfake.yml` (nouveau, +1 `media_review`
pédagogie source-first), `content/questions/security-prompt.yml` (+1 `ranked_actions` revue
de prompt chat), fiches `content/media/*.yml` (decision `approved`), médias SVG originaux
sans visage humain dans `apps/web/assets/media/`, `fixtures/session-media-review.json`,
`fixtures/session-ranked-actions.json`.

Exit gates : `validate-corpus` ✓ ; `run-session --fixture fixtures/session-media-review.json` ✓
scores attendus ; nouvelles questions en workflow draft→review (leur approbation suit la
gouvernance, ne bloque pas I4-I7).

### I4 — UI `MediaReviewCard` (Dioxus)

Fichiers : `crates/ui/src/lib.rs` (composant + variante dans `QuestionCard` selon
`interaction.type`), `apps/web/src/lib.rs` (résolution `question.media[]` → URL asset),
CSS (image 40 % / choix 60 % desktop, stack mobile), tests SSR (`dioxus_ssr::render_element`
rend `<img>` avec `alt`).

Exit gates : `cargo test -p rumble-ai-practices-ui` ✓ ;
`cargo check --target wasm32-unknown-unknown -p rumble-ai-practices-web --features web` ✓ ;
alt-text présent (WCAG).

### I5 — UI `RankedActionsCard` (top-1 action, style chat)

Transcription chat depuis `context.assets`, 4 cartes d'action, sélection + confirmation.
Le drag & drop de classement complet est un suivi ultérieur. Exit gates : idem I4.

### I6 — Clavier 1-9 + Entrée, accessibilité

Touches `1`-`9` = sélection, `Entrée` = validation, aucun bonus de vitesse (benchmark
Famille D). Hypothèse à vérifier sur docs.rs avant d'écrire : API `Event<KeyboardData>`
Dioxus 0.7. Exit gates : build wasm32 ✓, clippy ✓, parcours clavier complet revu
(`docs/ux.md`).

### I7 — Meilleur score sans régression, côté client (localStorage)

Contrat : client-only, zéro PII, zéro persistance serveur (respecte le frozen shim,
cf. convergence-prep). Hypothèse à vérifier sur docs.rs : `gloo-storage` vs `web-sys`
(licence via `cargo deny`). Exit gates : build wasm32 ✓ ; `cargo deny check` ✓ ; revue :
clés namespacées `best_q_*`, aucune donnée nominative.

## Hors scope

- Déploiement (« publier ») — chantier séparé après preuve produit (`docs/security-rgpd.md`,
  exécution locale par défaut).
- ADR « garde-fou de charge cognitive » (P3 du benchmark, ADR obligatoire).
- Toute mécanique rejetée : chrono, vies, ligues, multiplicateurs, leaderboard.
- Réutilisation des assets de la source (propriété tierce, visage réel, hébergement non
  souverain).

## Vérification end-to-end du chantier

```bash
cargo fmt --all --check && cargo check --workspace --all-targets && \
cargo test --workspace && cargo clippy --workspace -- -D warnings
cargo run -p rumble-ai-practices-cli -- validate-corpus --content content/questions
cargo run -p rumble-ai-practices-cli -- audit-corpus --content content/questions
cargo run -p rumble-ai-practices-cli -- run-session --fixture fixtures/session-media-review.json
cargo run -p rumble-ai-practices-cli -- run-session --fixture fixtures/session-ranked-actions.json
cargo check --target wasm32-unknown-unknown -p rumble-ai-practices-web --features web
# Parcours manuel PWA : cargo run -p rumble-ai-practices-cli -- serve → 127.0.0.1:3000
# → une question de chaque format au clavier seul, feedback nuancé, badge meilleur score après re-jeu
```
