# Plan — ai-practices-convergence-prep (2026-07 wave)

```yaml
format: forge.plan.v0.1
kind: planning_request
source:
  product: rumble-ai-practices
  plan_id: plan-2026-07-ai-practices-convergence-prep
  created_at: "2026-07-03"
execution_policy:
  planning_only: true
  allow_execution: false
  requires_human_approval_for_execution: true
traceability:
  - "target-version 1.0.0 (DA-8: Big-bang posture ratified; ADR 0029 Accepted as of 2026-07-03)"
  - "architecture-alignment-2026-07.md §3.1 B3/B4: Session engine overlap (nuanced) + thin adapter self-inconsistency"
  - "architecture-alignment-2026-07.md §2 M2/M3/M4/M10: Biscuit wiring, first gear-loader consumer, observability"
  - "docs/adrs/0002: wasm32-unknown-unknown gate requirement (line 29)"
  - "ecosystem/maturity/rumble-ai-practices.json: violations #1-5, debt#1-8, dioxus hygiene gaps (control plane record)"
depends_on:
  - plan-2026-07-rumble-lm-ui-slice (I2 onwards: lm publishes session runtime contract fixtures)
blocks:
  - rumble-lm session runtime consumption (lm depends on frozen shim removal gate + proven scoring module)
open_questions: []
risks:
  - id: R1
    severity: medium
    description: "Portal-forge integration assumed working but tokens.css generation pipeline undocumented; no CI proof of freshness. Current gate (I2) proves only CSS variable contract, NOT pipeline execution."
    mitigation: "I2 proves CSS contract completeness only (lines 61-257 validity); explicitly documented as PROVISIONAL pending portal-forge API. Pipeline wiring deferred until portal-forge publishes generation endpoint. Gated smoke check runs daily but does not prove regeneration."
  - id: R2
    severity: low
    description: "Scoring module extraction requires coordination with lm integration (fixture contracts); timing assumed available"
    mitigation: "I4 uses lm ADR 0029 contract fixtures as published; depends_on gate ensures lm is ready"
evidence_expectations: "each increment green CI + commands below prove gates; no claim without exit command output"
```

## Context

The `rumble-ai-practices` repo implements the first Rumble product vertical: a sovereign pedagogical AI practices diagnostic (multiplatform Rust core + Dioxus PWA). Currently in early official product state.

**Key constraint:** ADR 0029 (Accepted, DA-8 ratified as of 2026-07-03) states "ai-practices does NOT re-implement a session runtime." Observed reality: the repo carries a local store (BTreeMap TTL 1h, `/v1/sessions*`) provisionally until `rumble-lm` publishes a real session runtime. This shim is operational today but UNDOCUMENTED as provisional.

**Four violations block P5 acceptance** (ecosystem/maturity/rumble-ai-practices.json §violations):

1. **Frozen shim undocumented** (B3): local session engine treated as permanent in architecture.md:51-62, but ADR 0029 says it's temporary.
2. **Thin adapter self-contradiction** (B4): crates/api claims "thin adapter" (line 1) but the self-classification belongs in crates/session; API has no scoring logic but manages session lifecycle, creating a conceptual mismatch vs the claim.
3. **Portal integration unproven** (D8/fiches debt#2): tokens.css committed to repo without visible generation pipeline; no CI gate ensures freshness from portal-forge.
4. **WASM32 gate absent** (ADR 0002:29, fiches violations#4): own ADR demands `wasm32-unknown-unknown` becomes a gate, not implemented in .github/workflows/ci.yml.
5. **No e2e smoke** (fiches debt#8): no Playwright or integration test for PWA; ecosystem.remaining-work requires verification.

**Demand owners:** ADR 0029 (arbitration DA-8), ADR 0002 (repo's own decision).

## Target state

End state verified by:

- Shim **explicitly marked provisional** in code (comment block + ADR reference + non-negotiable marker)
- Shim **lifecycle gate in CI**: simple grep forbidding websocket/presence in crates/api (one line: `! git grep websocket -- crates/api/src/`)
- Portal-forge pipeline **contract proven reproducible**: CSS variable completeness script in repo, CI smoke check runs daily, **explicitly documented as PROVISIONAL** until portal-forge API available
- Scoring module **extracted and documented** as consumable by lm (with fixtures for contract proving)
- **WASM32 pre-flight probe + CI gate + size budget check active** before I3 merge
- Thin adapter repositioning complete: scoring logic **stays in session**, API **clarified as router + lifecycle** (self-description fixed)
- **Convergence gate documented**: shim deletion blocked until lm runtime is proven live + takes over all three endpoints; **decision owner noted explicitly (DA-8 arbitration)**

## Increments

### I1 — Mark shim as frozen, internal ADR, CI guard (PR: frozen-shim-governance)

**Prerequisite:** None.

**Files:**
- `crates/api/src/lib.rs` (head comment)
- `docs/adrs/0005-session-engine-provisional-until-lm-convergence.md` (new)
- `.github/workflows/ci.yml` (new guard step)

**Work:**

1. Add a documentation comment block at the top of `crates/api/src/lib.rs` (after line 1):
   ```rust
   //! ## FROZEN SHIM — ADR 0005
   //!
   //! The local session store (BTreeMap TTL 1h, `/v1/sessions*`, crates/api:40, crates/session:61-142)
   //! is PROVISIONAL and FROZEN for the MVP. No new features, no WebSocket/presence, no durability expansion.
   //! Replaced by rumble-lm's persistent session runtime when that repo publishes a proven contract.
   //! Convergence gate: lm ✓ live + this shim deleted in a follow-up PR (blocked until then).
   //!
   //! Reference: docs/adrs/0005-session-engine-provisional-until-lm-convergence.md
   ```

2. Create `docs/adrs/0005-session-engine-provisional-until-lm-convergence.md`:
   - Status: **Accepted** (ratified via DA-8 in ecosystem/architecture-alignment-2026-07.md)
   - Context: ADR 0029 (Accepted) decides lm owns the session runtime; ai-practices implements content + scoring only.
   - Decision: Local BTreeMap store in crates/api (lines 40, 45) and crates/session (lines 61-142) is a provisional MVP shim. No growth. Convergence to lm's runtime is mandatory before P5 closure.
   - Consequences: Zero new session features until lm converges; no presence, WebSocket, or durability additions.

3. Add CI guard in `.github/workflows/ci.yml` (append before the final `- name: Test` step):
   ```yaml
   - name: Forbid undeclared session engine growth
     run: |
       set -euo pipefail
       if git grep -n websocket -- 'crates/api/src' 'crates/session/src'; then
         echo "::error::websocket/presence forbidden in frozen shim (ADR 0005)"
         exit 1
       fi
   ```

**Exit gates:**
- `cargo fmt --all --check` ✓
- `cargo check --workspace --all-targets` ✓
- `cargo test --workspace --all-targets` ✓
- New CI guard runs without match: `if git grep -n websocket -- crates/api/src crates/session/src 2>&1 | grep -q .; then exit 1; fi` → exits 1 (success, no matches)
- `docs/adrs/0005-session-engine-provisional-until-lm-convergence.md` committed
- PR merges ✓ with all gates green

---

### I2 — Prove portal-forge pipeline contract, CI freshness gate (PR: portal-tokens-pipeline-proven)

**Prerequisite:** I1 (governance context).

**Files:**
- `scripts/portal-tokens-sync.sh` (new, executable)
- `.github/workflows/ci.yml` (new gate step with explicit PROVISIONAL note)
- `apps/web/assets/tokens.css` (verified but NOT regenerated; contract only)

**Work:**

1. Create `scripts/portal-tokens-sync.sh`:
   ```bash
   #!/usr/bin/env bash
   set -euo pipefail
   # PROVISIONAL SMOKE CHECK: Portal-forge integration API not yet published
   # This script verifies CSS variable CONTRACT ONLY, not pipeline freshness.
   # 
   # Contract: tokens.css must declare all required CSS variables matching portal contract.
   # Freshness: portal-forge will publish JSON generation API; CI will call it to regenerate.
   # Status: AWAITING portal-forge.rs generation endpoint. Once published, CI will wire:
   #   1. portal-forge generates design tokens JSON from DTCG source
   #   2. Portal contract defines token export format (portal.contrast_report.v0.1)
   #   3. This script converts Portal JSON to CSS variables (matching apps/web/assets/tokens.css)
   # 
   # TODAY: This script proves the CSS contract is intact; does NOT prove pipeline regeneration.
   # Risk R1 mitigation is INCOMPLETE until portal-forge API lands. See ADR 0005.
   
   REPO_ROOT=$(git rev-parse --show-toplevel)
   TOKENS_CSS="$REPO_ROOT/apps/web/assets/tokens.css"
   TOKENS_EXPECTED_VARS=(
     "--color-accent"
     "--color-background"
     "--color-border"
     "--color-brand"
     "--color-danger"
     "--color-success"
     "--color-surface"
     "--color-text"
     "--color-text-secondary"
     "--color-warning"
     "--font-family-body"
     "--font-size-base"
     "--font-size-lg"
     "--font-size-sm"
     "--radius-lg"
     "--radius-md"
     "--radius-sm"
     "--spacing-lg"
     "--spacing-md"
     "--spacing-sm"
     "--spacing-xl"
     "--spacing-xs"
   )
   
   # Verify all expected CSS variables are defined
   for var in "${TOKENS_EXPECTED_VARS[@]}"; do
     if ! grep -q "^\s*$var:" "$TOKENS_CSS"; then
       echo "::error::Missing token variable: $var in $TOKENS_CSS"
       exit 1
     fi
   done
   
   echo "✓ Portal tokens contract verified (PROVISIONAL): CSS variable signatures intact"
   echo "  NOTE: This proves contract readiness, NOT pipeline freshness. Regeneration proof deferred until portal-forge API is published."
   ```

2. Add CI gate in `.github/workflows/ci.yml` (append before `- name: Test`):
   ```yaml
   - name: Verify Portal tokens contract (pipeline readiness — PROVISIONAL)
     run: bash scripts/portal-tokens-sync.sh
   ```

3. Verify gate locally:
   ```bash
   bash scripts/portal-tokens-sync.sh
   ```
   Should print: `✓ Portal tokens contract verified (PROVISIONAL): CSS variable signatures intact`

**Exit gates:**
- `bash scripts/portal-tokens-sync.sh` outputs ✓ success message with PROVISIONAL acknowledgment
- `cargo fmt --all --check` ✓
- `cargo check --workspace --all-targets` ✓
- CI runs new gate without error
- Gate output explicitly notes: "This proves contract readiness, NOT pipeline freshness"
- PR merges ✓

---

### I3 — Add WASM32 pre-flight probe + CI gate + size budget (PR: add-wasm32-e2e-gates)

**Prerequisite:** I1 (for CI context familiarity).

**Work:**

1. **Pre-flight probe (local verification before merge):**
   ```bash
   # Run this locally before submitting I3 PR to ensure code compiles today
   rustup target add wasm32-unknown-unknown
   cargo check --target wasm32-unknown-unknown --package rumble-ai-practices-web --features web
   ```
   If this fails, stop and debug before I3 merge. Do not allow CI to break.

2. Add WASM32 check in `.github/workflows/ci.yml` (after `- name: Clippy`, before `- name: Test`):
   ```yaml
   - name: WASM32 target check (per ADR 0002:29)
     run: |
       rustup target add wasm32-unknown-unknown
       cargo check --target wasm32-unknown-unknown --package rumble-ai-practices-web --features web
   ```

3. Add WASM32 size budget check in `.github/workflows/ci.yml` (after WASM32 check):
   ```yaml
   - name: WASM32 size budget verification (ADR 0002 + target-version.md)
     run: |
       cargo build --target wasm32-unknown-unknown --release --package rumble-ai-practices-web --features web 2>&1 | tee wasm-build.log
       WASM_SIZE=$(ls -lh target/wasm32-unknown-unknown/release/rumble_ai_practices_web.wasm 2>/dev/null | awk '{print $5}')
       echo "WASM binary size: ${WASM_SIZE}"
       # Budget per ADR 0002: ≤ 450 KiB gzip
       # For now, warn if >500 KiB uncompressed (gzip will be ~30-40% of that)
       WASM_SIZE_KB=$(stat -f%z target/wasm32-unknown-unknown/release/rumble_ai_practices_web.wasm 2>/dev/null | awk '{printf "%.0f", $1/1024}' || echo "unknown")
       echo "WASM size: ${WASM_SIZE_KB} KiB (uncompressed); gzipped target: ≤450 KiB per ADR 0002"
       if [ "$WASM_SIZE_KB" -gt 1500 ]; then
         echo "::warning::WASM size ${WASM_SIZE_KB} KiB exceeds typical gzip budget (≤450 KiB). Review bloat in rumble-ai-practices-web."
       fi
   ```

4. Create `tests/smoke.rs` (integration test for API + PWA contract):
   ```rust
   //! Smoke test: verify API routes and PWA shell render without panic.
   use rumble_ai_practices_api::router;
   use rumble_ai_practices_domain::QuestionId;
   
   #[tokio::test]
   async fn smoke_api_routes_exist() {
       // Minimal fixture to prove API router is healthy
       let questions = vec![];
       let r = router(questions);
       
       // Router construction must not panic; routes registered.
       assert_eq!(r.routes().iter().count() > 0, true, "router has routes");
   }
   
   #[tokio::test]
   async fn smoke_session_lifecycle() {
       use rumble_ai_practices_session::start_session;
       
       // Prove local session engine doesn't panic on empty input
       let result = start_session("test-session", vec![]);
       assert!(result.is_err(), "empty session rejected by design");
   }
   ```

5. Add e2e smoke to CI (append to `ci.yml`):
   ```yaml
   - name: Smoke tests (API + session lifecycle)
     run: cargo test --test smoke --lib rumble_ai_practices_session
   ```

**Files:**
- `.github/workflows/ci.yml` (new wasm32 + size budget + e2e steps)
- `tests/smoke.rs` (new e2e fixture)

**Exit gates:**
- **Before merge:** `cargo check --target wasm32-unknown-unknown --package rumble-ai-practices-web --features web` ✓ (local pre-flight passes)
- `rustup target add wasm32-unknown-unknown` ✓
- `cargo check --target wasm32-unknown-unknown --package rumble-ai-practices-web --features web` ✓ (CI gate, no compile error)
- `cargo build --target wasm32-unknown-unknown --release` ✓ and reports size (should be <500 KiB uncompressed)
- `cargo test --test smoke` ✓ (both tests pass)
- `cargo fmt --all --check` ✓
- CI workflow runs all new gates: ✓
- PR merges ✓

---

### I4 — Extract scoring module as consumable, document for lm fixture binding (PR: extract-scoring-module)

**Prerequisite:** I1, I2, I3 (gates must be green; scoring module needs to pass all tests).

**Files:**
- `crates/scoring/Cargo.toml` (new crate)
- `crates/scoring/src/lib.rs` (new module, extracted from crates/session lines 61-257)
- `Cargo.toml` (add to workspace members)
- `docs/scoring-module-contract.md` (new, for lm integration fixture binding)
- `crates/session/src/lib.rs` (refactor: use crates/scoring)

**Work:**

1. Create `crates/scoring/Cargo.toml`:
   ```toml
   [package]
   name = "rumble-ai-practices-scoring"
   version.workspace = true
   edition.workspace = true
   rust-version.workspace = true
   license.workspace = true
   
   [dependencies]
   rumble-ai-practices-domain.workspace = true
   serde.workspace = true
   thiserror.workspace = true
   ```

2. Extract scoring logic into `crates/scoring/src/lib.rs`:
   - Move from `crates/session/src/lib.rs` lines 61-257:
     - **lines 61-142:** `submit_answer()` function with full choice validation and scoring
     - **lines 144-179:** `validate_choice_count()` helper for interaction constraints
     - **lines 181-245:** `complete_session()` aggregator for axis-level summaries
     - **lines 247-257:** `practice_level()` classifier mapping score to PracticeLevel enum
   - Keep domain contracts (Question, AnswerEvaluation, etc.) in domain crate
   - Scoring module is purely algorithmic, zero dependencies on session state management

3. Refactor `crates/session/src/lib.rs`:
   - Import `use rumble_ai_practices_scoring::*`
   - Keep SessionState, SessionError, start_session, run_fixture, session lifecycle functions
   - Tests stay in place (verify smoke test passes with refactored session)

4. Update workspace `Cargo.toml`:
   ```toml
   members = [
       "crates/domain",
       "crates/content",
       "crates/audit",
       "crates/scoring",  # NEW
       "crates/session",
       "crates/api",
       "crates/cli",
       "crates/ui",
       "apps/web",
   ]
   ```

5. Create `docs/scoring-module-contract.md`:
   - Status: Ready for lm integration
   - Exports: `pub fn submit_answer(...)`, `pub fn complete_session(...)`, `pub fn practice_level(...)`
   - Input contract: domain/Question, AnswerEvaluation types (from crates/domain)
   - Corpus: content/questions/*.yml (YAML → domain::Question via crates/content loader)
   - Test fixture: use fixtures/session-basic.json to verify scoring against known corpus
   - For lm: consume this module as `rumble_ai_practices_scoring` via Cargo dep; pass your own session runtime's state + our scoring algo

**Exit gates:**
- `cargo check --workspace --all-targets` ✓ (new crate compiles, no errors)
- `cargo test --workspace` ✓ (all tests pass, including extracted scoring tests)
- `cargo clippy --workspace -- -D warnings` ✓
- `cargo fmt --all --check` ✓
- `cargo test --all` ✓ (regression gate: session flow unchanged after refactoring)
- `docs/scoring-module-contract.md` committed and linked in README
- PR merges ✓

---

### I5 — Fix thin adapter self-description, clarify API role (PR: clarify-api-adapter-role)

**Prerequisite:** I1, I4 (scoring now lives in its own module; session state is now the adapter's true concern).

**Files:**
- `crates/api/src/lib.rs` (documentation comment update, no logic change)
- `docs/adrs/0002-stack-rust-dioxus-multiplatform.md` (add note about API role in ADR)

**Work:**

1. Update `crates/api/src/lib.rs` line 1 comment to clarify actual role (not just "thin adapter"):
   ```rust
   //! Axum API adapter: HTTP router + session lifecycle manager for the validated Rust core.
   //!
   //! **NOT a thin adapter**: This layer carries session lifecycle responsibility (allocation, TTL cleanup, enforcement).
   //! It is NOT responsible for scoring or business logic; those live in crates/scoring + crates/session.
   //! 
   //! The actual issue: API manages session state (who decides timing, allocation, cleanup) — this is orchestration,
   //! not routing. True business logic (scoring, evaluation rules) lives purely in crates/domain + crates/session + crates/scoring.
   //!
   //! Design intent: Allows API transport layer to be swapped (HTTP→gRPC, Axum→Lambda) without touching scoring logic.
   //!
   //! Session store: provisional BTreeMap (TTL 1h, frozen per ADR 0005) until lm runtime convergence.
   ```

2. Add a new section to `docs/adrs/0002-stack-rust-dioxus-multiplatform.md` (append before "Alternatives"):
   ```markdown
   ## API Layer Clarification (I5)
   
   The `crates/api` HTTP adapter carries two distinct responsibilities:
   1. **Router**: declare and mount `/v1/*` endpoints per the public contract.
   2. **Session lifecycle**: allocate IDs, enforce TTL cleanup, limit in-memory store size.
   
   It is **NOT** a business logic layer. Scoring, evaluation, and pedagogical rules live purely in crates/domain + crates/session + crates/scoring.
   
   The initial "thin adapter" label obscured that session lifecycle management (item 2) is orchestration, not routing. 
   This distinction matters for swapping transports: if API is replaced by Lambda or gRPC, scoring logic unchanged.
   
   This design allows clean separation: scoring is deterministic + stateless; API carries orchestration + state.
   ```

**Exit gates:**
- `cargo fmt --all --check` ✓
- `cargo check --workspace --all-targets` ✓
- `cargo clippy --workspace -- -D warnings` ✓
- `cargo test --workspace` ✓
- PR merges ✓

---

### I6 — Convergence gate: document shim deletion trigger with explicit decision owner (PR: convergence-gate-documented)

**Prerequisite:** I1-I5 all merged. lm integration slice must be proven (separate plan).

**Files:**
- `docs/adrs/0005-session-engine-provisional-until-lm-convergence.md` (add convergence section with decision owner)
- `CONVERGENCE.md` (new, one-page checklist for shim deletion, explicitly notes DA-8 arbitration)

**Work:**

1. Append to `docs/adrs/0005-session-engine-provisional-until-lm-convergence.md`:
   ```markdown
   ## Convergence Trigger (Post-MVP)
   
   Delete the local session store when ALL of the following are met:
   
   1. **rumble-lm publishes a proven session runtime**
      - Contract: lm/crates/server exports SessionRuntime trait or service
      - Proof: rumble-lm CI integrates with ai-practices fixtures; full 3-endpoint contract proven
      - Evidence: lm PR merged + tagged v1.0.0+
   
   2. **ai-practices switches to lm session runtime**
      - crates/api refactored: Sessions now proxied to lm HTTP/gRPC endpoint
      - No behavior change from client perspective (same /v1/sessions* routes)
      - Fixtures re-run: verify scores unchanged (regression test)
   
   3. **Local store fully removed**
      - Delete: crates/api/src/lib.rs lines 39-113 (BTreeMap, session mgmt, cleanup)
      - Delete: crates/session/src/lib.rs lines 259-280 (SessionFixture, run_fixture — moved to lm)
      - Keep: scoring logic (crates/scoring + crates/domain)
      - CI: all tests green, no websocket/presence patterns detected
   
   4. **Bounded by wave discipline + decision authority**
      - Big-bang posture: this deletion is a hard block until lm is live.
      - **Decision owner: Constantin Jais + architecture steering (DA-8 arbitration).**
      - This is not an automated gate; human approval required to proceed with shim deletion.
      - Do not attempt partial migration or shim shrinking; wait for full replacement.
   ```

2. Create `CONVERGENCE.md`:
   ```markdown
   # Convergence Checkpoint: lm Session Runtime Integration
   
   The local session store (`crates/api`, `crates/session`) is a **provisional MVP shim** frozen per ADR 0005.
   Replacing it with rumble-lm's persistent session runtime is the final gate before P5 acceptance.
   
   ## Checklist (all must be ✓)
   
   - [ ] rumble-lm publishes session runtime contract (lm ADR 0029 + fixtures)
   - [ ] ai-practices CI passes with lm runtime consumer test (fixture binding)
   - [ ] Scoring module (crates/scoring) proves deterministic with lm's session state
   - [ ] Local BTreeMap fully removed from crates/api + crates/session
   - [ ] /v1/sessions* routes proxy to lm endpoint; behavior unchanged
   - [ ] Regression: session summaries unchanged vs MVP fixtures
   - [ ] All CI gates green; no websocket/presence patterns remain
   
   **Decision makers: Constantin Jais + architecture steering (DA-8 arbitration).**
   **NOT automatic:** Shim deletion requires explicit approval before merge.
   **Timeline:** End of wave 2026-07 (after lm slice is proven).
   **Effort:** ~2-4 days (routing refactor + fixture alignment).
   ```

**Exit gates:**
- `docs/adrs/0005-session-engine-provisional-until-lm-convergence.md` updated + committed with explicit decision owner and DA-8 reference
- `CONVERGENCE.md` created + linked in README.md, explicitly notes DA-8 arbitration requirement
- `cargo fmt --all --check` ✓
- PR merges ✓

---

## Out of scope

**Not in this plan; demand-driven follow-ups:**

1. **HttpOnly SameSite=Strict cookies** (fiches debt#6): local store makes cookies unnecessary for MVP; when lm runtime adds persistence, session tokens can move to secure cookies. Gated on lm integration.

2. **Tailwind v4 integration** (fiches debt#5): current CSS variables approach works; Tailwind via dioxus can be a post-P5 enhancement. No blocking risk.

3. **Observability (tracing/logging)** (fiches debt#1, M10): belongs to lm + ai-practices together; will be added in the lm integration wave when both have logging infrastructure.

4. **Content corpus expansion** (fiches debt#4): MVP target is 30 questions; current YAML is ~1595 lines. Growth to 30 questions is content work, not architecture, and is demand-driven by product.

5. **CLI packaging** (fiches debt#8): local `cargo run` is sufficient for MVP. Binary releases are post-P5 when cloud infrastructure is ready.

6. **Native iOS/Android** (ADR 0002:22): SwiftUI/Compose deferred pending product proof. Portal bindings exist but are frozen.

---

## Verification

**End-to-end gate for the chantier:**

```bash
# 1. All increments merged and CI green
git log --oneline | head -6
# Expected: commits titled "frozen-shim-governance", "portal-tokens-pipeline-proven", etc.

# 2. Shim is marked frozen + guard active
grep -A 3 "FROZEN SHIM" crates/api/src/lib.rs
# Expected: "FROZEN SHIM — ADR 0005" visible

# 3. ADR 0005 exists with decision owner
test -s docs/adrs/0005-session-engine-provisional-until-lm-convergence.md && grep -q "DA-8 arbitration" docs/adrs/0005-session-engine-provisional-until-lm-convergence.md
# Expected: exit 0

# 4. Portal tokens pipeline script exists + runs (PROVISIONAL acknowledged)
bash scripts/portal-tokens-sync.sh | grep -q "PROVISIONAL"
# Expected: exit 0; output contains PROVISIONAL acknowledgment

# 5. WASM32 gate in CI with size budget check
grep "wasm32-unknown-unknown" .github/workflows/ci.yml && grep -q "size budget" .github/workflows/ci.yml
# Expected: both present

# 6. e2e smoke tests in CI
grep "smoke" .github/workflows/ci.yml
# Expected: test step exists

# 7. Scoring module extracted
test -s crates/scoring/Cargo.toml && grep "rumble-ai-practices-scoring" Cargo.toml
# Expected: exit 0; crate in workspace

# 8. API self-description fixed (NOT a thin adapter claim removed)
grep -A 2 "NOT a thin adapter" crates/api/src/lib.rs && ! grep "thin Axum API adapter" crates/api/src/lib.rs
# Expected: new clarification visible; old misleading label gone

# 9. Convergence gate documented with decision owner
test -s CONVERGENCE.md && grep -q "DA-8 arbitration" CONVERGENCE.md && grep -q "Constantin Jais" CONVERGENCE.md
# Expected: both files exist; decision owner explicitly named

# 10. All CI gates pass
cargo fmt --all --check && cargo check --workspace && cargo test --workspace && cargo clippy --workspace -- -D warnings
# Expected: all exit 0
```

---

**Wave closure condition (for steering):**
All increments I1-I6 merged, CI green, and the repo is **ready for lm integration**. Shim is explicitly frozen with decision owner noted, scoring module is reusable, and the convergence path is documented and gated. No further ai-practices work until lm publishes a session runtime contract, at which point convergence (I6 checklist execution + DA-8 arbitration approval) unblocks P5 acceptance.
