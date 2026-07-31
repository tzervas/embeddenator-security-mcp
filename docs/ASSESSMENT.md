# security-mcp — Assessment & Gap Analysis

**Date:** 2026-07-08  
**Crate:** `security-mcp` 0.2.0-alpha (Rust)  
**Role:** MCP **content/text screener** for agent tool I/O and model text  
**Consumers:** cabal-devmelopner (Wave D), webpuppet stack (pairing intended)

---

## 1. What this project is / is not

| Is | Is not |
|----|--------|
| Regex + entropy screening of **text** | Repo / git / SBOM / CVE scanner |
| MCP tools: screen in/out, redact, check_safe | Transparent multi-server proxy on `main` |
| stdio + HTTP (loopback default) | Validated ML DLP |
| Optional Rust library API | Python package |

---

## 2. Maturity

| Dimension | Score | Notes |
|-----------|-------|--------|
| Core detectors | **3** | Useful heuristics; FP/FN expected |
| MCP surface | **4** | Tools exist; comprehensive E2E stdio integration tests in place |
| Auth / multi-tenant | **4** | Token auth implemented and enforced on remote bind |
| Proxy / wrap-other-tools | **3** on main | **On main** via PR #28 (`wrap` / `proxy_*`); STABLE gate still open (S-B3 tests + consumer acks) |
| Docs honesty | **4** | Scope mostly clear after public-ready pass |
| Cabal Production load-bearing | **2** | Optional peer only until hardened |

---

## 3. In-flight branches

| Branch | Verdict |
|--------|---------|
| `main` / `dev` / `integration` | Aligned |
| **`security-proxy-integration`** | **Superseded for Wave B core** — concepts cherry-picked in PR #28; do not blind-merge the old branch |
| `claude/finish-security-mcp` | Likely alternate of public-ready pass |

---

## 4. Gaps (priority)

| Gap | Sev | Notes |
|-----|-----|--------|
| Proxy wrap incomplete STABLE | Med | Code on main (PR #28); remaining: real-child tests (S-B3), agent-mcp consumer ack, human STABLE sign-off — see `docs/bulletins/security-mcp-wrap.md` |
| Precision/recall eval | Med | Honesty for “security product” |

**Closed (was High):** “Proxy not on main” — wrap/subprocess/`proxy_status`/`proxy_configure` shipped on `main` in PR #28 (2026-07-17). ASSESSMENT maturity score updated; do not re-open that gap for code presence.

*Note: No auth on HTTP, False positive patterns, timeout/rate-limit enforcement, MCP e2e tests, and WebSocket claims have been successfully resolved as part of the Wave A hardening maintenance review.*

---

## 5. Integration fit

- **Cabal:** call as **stdio MCP peer** after tool allowlists exist; never sole gate.  
- **webpuppet:** intended chain `screen_input → tool → screen_output` or true proxy (branch).  
- **Library:** Rust only; Python cabal uses MCP client pattern (like Tero).

See [ROADMAP.md](ROADMAP.md).

## Tero index

Layer-1 citation index: [docs/tero-index/](tero-index/) (`index.json`, `INDEX.md`, `MANIFEST.toml`).
