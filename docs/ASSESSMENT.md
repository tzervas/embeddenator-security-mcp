# security-mcp — Assessment & Gap Analysis

**Date:** 2026-07-25 (aligned with [CURRENT-STATE.md](CURRENT-STATE.md) measure @ `1c6c82a`)  
**Crate:** `security-mcp` 0.2.0-alpha (Rust)  
**Role:** MCP **content/text screener** for agent tool I/O and model text  
**Consumers:** cabal-devmelopner (intended Wave D peer), webpuppet stack (wrap pairing DRAFT)

Scores below are **judgment**, not measurements. For pass/fail evidence see CURRENT-STATE.

---

## 1. What this project is / is not

| Is | Is not |
|----|--------|
| Regex + entropy screening of **text** | Repo / git / SBOM / CVE scanner |
| MCP tools: screen in/out, redact, check_safe, get_config | Validated ML DLP |
| stdio + HTTP (loopback default; token auth for remote bind) | Sole security control for cabal |
| Optional wrap/proxy on main (**DRAFT**, not STABLE) | Production-certified multi-server proxy |
| Optional Rust library API | Python package / crates.io-proven install path |

---

## 2. Maturity (ordinal, not %)

| Dimension | Score (1–5) | Notes |
|-----------|-------------|--------|
| Core detectors | **3** | Useful heuristics; FP/FN expected; no P/R corpus |
| MCP surface | **4** | Tools + stdio e2e + camelCase wire (#43); notifications without `id` still parse-fail |
| Auth / multi-tenant | **3** | Token + bind safety unit-tested; live HTTP e2e not measured in CURRENT-STATE |
| Proxy / wrap-other-tools | **2** on main | Code merged (#28); bulletin **DRAFT**; real child test **ignored** |
| Docs honesty | **4** | Scope walls clear; PM suite measures build/test; avoid treating badges as sole truth |
| Cabal production load-bearing | **2** | Optional peer until wrap STABLE + eval honesty |

---

## 3. Branches (as of measure)

| Branch / area | Verdict |
|---------------|---------|
| `main` @ `1c6c82a` | Default; measured green locally + fleet-ci/security success on tip |
| `dev` | Integration target for several open STABLE-promotion PRs — **not** measured as main |
| `security-proxy-integration` | Historical source for wrap; **not** the live main path (superseded by #28) |
| Open STABLE PRs (#36, #39, #42, #45, #46) | **Proposed, not committed** until merged and re-measured |

---

## 4. Gaps (priority)

| Gap | Sev | Notes / unblock |
|-----|-----|-----------------|
| Wrap not STABLE | High for proxy consumers | Real child MCP test + consumer acks + human bulletin promote |
| Precision/recall eval | Med | Issue #19; labeled corpus + harness |
| JSON-RPC notifications | Med | `id` required → parse error on `notifications/initialized` (observed 2026-07-25) |
| Live HTTP / WS / SSE e2e | Med | Flags exist; integration tests incomplete |
| REUSE path debt | Low/Med | Issue #32 |
| FS/git scan & external tools | Scope | Issues #17/#18 — still **non-goals** unless re-scoped |

Wave A items (auth, stdio e2e, timeout/rate limit, FP docs) are treated as **done on main**; do not re-open them without new regressions.

---

## 5. Integration fit

- **Cabal:** call as **stdio MCP peer** after tool allowlists exist; never sole gate.  
- **webpuppet:** intended chain `screen_input → tool → screen_output` or wrap path (DRAFT bulletin).  
- **Library:** Rust only; Python cabal uses MCP client pattern (like Tero).

See [ROADMAP.md](ROADMAP.md) and [DEVELOPMENT-PATH.md](DEVELOPMENT-PATH.md).

## Tero index

Layer-1 citation index: [docs/tero-index/](tero-index/) (`index.json`, `INDEX.md`, `MANIFEST.toml`).
