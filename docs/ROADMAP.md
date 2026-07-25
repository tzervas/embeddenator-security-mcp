# security-mcp — Product Roadmap

**Status:** Living (updated 2026-07-25 from measured `main` @ `1c6c82a`)  
**North star:** Load-bearing **local** content-screening MCP that agents can put in front of risky tools — honest heuristics, optional strict modes, real proxy path when STABLE criteria are met.

Companions: [CURRENT-STATE.md](CURRENT-STATE.md) (measured), [ASSESSMENT.md](ASSESSMENT.md), [DEVELOPMENT-PATH.md](DEVELOPMENT-PATH.md), wrap bulletin [bulletins/security-mcp-wrap.md](bulletins/security-mcp-wrap.md).

**No delivery dates** are invented below. Each open item names **what would unblock it**.

---

## Waves

### Wave A — Harden what exists — **completed on main**

| ID | Work | Exit | Status |
|----|------|------|--------|
| S-A1 | Token auth for HTTP (`SECURITY_MCP_TOKENS` or bearer) | Unauthenticated remote bind rejected | **Completed** (unit: bind safety) |
| S-A2 | MCP stdio e2e tests | CI/local covers tools path | **Completed** (`test_mcp_stdio_e2e`) |
| S-A3 | WebSocket claim honesty | Docs = code (optional flag) | **Completed** |
| S-A4 | Config: enforce timeout_ms + simple rate limit | No dead knobs | **Completed** (unit coverage) |
| S-A5 | FP triage pass on noisiest patterns | Document expected FP | **Completed** (README table) |

### Wave B — Proxy / wrap path — **code on main, product STABLE not yet**

Triage source: historical branch `origin/security-proxy-integration`. Landed concepts: PR [#28](https://github.com/tzervas/security-mcp/pull/28).

| ID | Work | Status | What would unblock |
|----|------|--------|--------------------|
| S-B1 | Diff branch vs main; cherry-pick viable subprocess/proxy | **Completed** (merge #28) | — |
| S-B2 | CLI/env wrap + forward non-local MCP; tools `proxy_status` / `proxy_configure` | **Completed** (code on main; DRAFT bulletin) | — |
| S-B3 | Integration tests with **real** child MCP (not scaffold-only) | **Open** | Fixture binary or `check.sh`-built mock MCP; un-ignore `real_child_mcp_stdio_roundtrip`; green under `./scripts/check.sh` |
| S-B4 | Document pairing with webpuppet-rs-mcp / agent-mcp | **Partial** | webpuppet family acks recorded in bulletin; **agent-mcp** consumer ack still open on checklist |
| S-B5 | Promote bulletin DRAFT → STABLE | **Open** | S-B3 + remaining consumer acks + human sign-off on [bulletins/security-mcp-wrap.md](bulletins/security-mcp-wrap.md) |

**Note:** Several open PRs into `dev` claim STABLE promotion (#36, #39, #42, #45, #46 as of 2026-07-25). Treat them as **proposed, not committed** until merged to the default branch and re-measured.

### Wave C — Product quality

| ID | Work | Status | What would unblock |
|----|------|--------|--------------------|
| S-C1 | Labeled mini-corpus + precision/recall smoke metrics | **Open** (issue [#19](https://github.com/tzervas/security-mcp/issues/19)) | Curated labeled samples + harness job that fails on metric regression; decision on acceptable FP/FN floors |
| S-C2 | Redaction policies (structured JSON paths) | **Open** / proposed | Design for path selectors; tests on nested JSON; keep plain-text redaction working |
| S-C3 | Non-alpha `0.2.x` after A+B STABLE + honesty pass | **Open** | Human release decision; STABLE wrap (S-B5) or explicit scope exclusion; changelog without “alpha” caveats that still apply |
| S-C4 | JSON-RPC notifications without `id` | **Open** (defect found in docs measure) | Optional `id` on request parse; silence or no response for notifications; regression test with `notifications/initialized` |
| S-C5 | Live HTTP e2e (auth, rate limit, optional WS/SSE) | **Open** / proposed | Integration tests that bind loopback and exercise routes (today only unit-level bind/rate coverage) |

---

## Open issues → roadmap mapping

| Issue | Theme | Unblock |
|-------|-------|---------|
| [#19](https://github.com/tzervas/security-mcp/issues/19) | Validated detection accuracy | S-C1 corpus + harness |
| [#18](https://github.com/tzervas/security-mcp/issues/18) | External security-tool integration | Product decision: stay non-goal vs optional spawn-adapters; if yes, allowlist + sandbox design |
| [#17](https://github.com/tzervas/security-mcp/issues/17) | Filesystem / git-tree scanning | Product decision: remains **non-goal** of this binary unless scope expands deliberately |
| [#32](https://github.com/tzervas/security-mcp/issues/32) | REUSE path coverage debt | Finish SPDX path coverage / REUSE compliance work from bootstrap #31 |

Items #17 and #18 are **proposed expansions**, not committed scope. Default stance remains README non-goals until a human re-scopes.

---

## API surface (current — keep stable unless version bump)

### MCP tools

| Tool | Purpose |
|------|---------|
| `screen_input` | Inbound / prompt-side screen |
| `screen_output` | Outbound / tool-result screen |
| `screen_content` | Generic with direction |
| `check_safe` | Boolean-ish safety summary |
| `redact_content` | Return redacted text |
| `get_config` | Active thresholds (no secrets) |
| `proxy_status` | Child process health (wrap) |
| `proxy_configure` | Allowlisted child command + env (admin token) |

**Envelope:** JSON-RPC MCP; results use **camelCase** on the wire (PR #43). Screening payloads may still use snake_case inside tool result text JSON — treat tool `content[].text` as application JSON, not the MCP envelope.

### HTTP (implemented flags; live e2e UNVERIFIED in CURRENT-STATE)

- Default bind: `127.0.0.1`. Refuse `0.0.0.0` without tokens or `ALLOW_INSECURE_BIND=1`.
- Tokens: `SECURITY_MCP_TOKENS` / `--tokens`.
- Rate limit: `--rate-limit` / `SECURITY_MCP_RATE_LIMIT`.
- Optional: `--websocket`, `--sse`.

### Rust library (stable intent)

```rust
// Conceptual public surface — ScreeningPipeline remains the library entry
let report = pipeline.screen(/* content, direction */)?;
```

---

## Non-goals (unchanged unless re-scoped)

- Replacing gitleaks / cargo-audit / trivy / semgrep as repo scanners  
- Guaranteeing zero false negatives  
- Shipping as cabal’s **only** security control  
- Agent-cut 1.x.x release  

---

## Process / fleet follow-ups (proposed, not committed)

| Item | Why | Unblock |
|------|-----|---------|
| Align branch flow with fleet contract (`work → dev → main`) | Mixed PR targets historically | Human policy + default PR base |
| Required status checks on `main` before any auto-merge | Zero required checks today | Ruleset update **after** contexts reliably report |
| Collapse duplicate STABLE PRs on `dev` | Avoid parallel contradictory promotions | Maintainers close or merge one lineage |

---

## Version policy reminder

Repos stays **0.x.x** under commitizen / operator contract until a **human** authorizes 1.x. Current tag line: **`0.2.0-alpha`**. A GitHub Release is not crates.io publication.
