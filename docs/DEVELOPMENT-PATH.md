# Development path — security-mcp

**Layer:** back (deep history). For measured “what works today,” see [CURRENT-STATE.md](CURRENT-STATE.md). For planned work, see [ROADMAP.md](ROADMAP.md).

This document reconstructs how the project got here from **git history, merged PRs, and releases**. Where a claim is inference rather than a cited artifact, it is marked as such.

---

## Origin and rename

| Evidence | What it shows |
|----------|----------------|
| Repo created `2026-01-04` (`gh api /repos/tzervas/security-mcp`) | Project started as a small Rust MCP security crate in the tzervas fleet. |
| Commit `719c836` / PR [#9](https://github.com/tzervas/security-mcp/pull/9) | **Breaking rename** from `embeddenator-security-mcp` → `security-mcp` (`v0.1.0-alpha.2`). Inferred from name: the crate was originally scoped under an “embeddenator” product line and was extracted/renamed to stand alone. |
| Early tags `v0.1.0`, `v0.1.4-alpha` … `v0.1.6-alpha` | Alpha versioning under 0.x from the start; no 1.x cut. |

**Decision that shaped the product:** stay a **content/text screener** for MCP tool I/O (regex + entropy), not a repo/CVE/SBOM scanner. That scope was made explicit in the public-ready pass (PR [#16](https://github.com/tzervas/security-mcp/pull/16) / commit `1b69db2`) and remains the README’s primary honesty boundary.

**Rejected alternative (inferred from README + open issues #17/#18):** folding filesystem/git scanning and external tool orchestration (`gitleaks`, `trivy`, `cargo audit`) into this process. Those remain open issues and non-goals of the core binary so the MCP path stays fast and local-first.

---

## Early product surface (Jan 2026)

| Step | Evidence |
|------|----------|
| HTTP + stdio MCP server, clap CLI | Present from early tree; axum bumped 0.7 → 0.8 in PR [#7](https://github.com/tzervas/security-mcp/pull/7). |
| Client config docs | PR [#11](https://github.com/tzervas/security-mcp/pull/11) — VS Code / Claude Desktop snippets. |
| Semver caret ranges / dep refresh | PR [#12](https://github.com/tzervas/security-mcp/pull/12). |

Detectors landed as three families (injection / PII / secrets) with unit tests in-module. Detection quality was never claimed as measured precision/recall — still open as issue [#19](https://github.com/tzervas/security-mcp/issues/19).

---

## Public-ready pass and honest scope (Jul 2026)

PR [#16](https://github.com/tzervas/security-mcp/pull/16) (`1b69db2` lineage):

- Black-box smoke tests (`tests/smoke.rs`) against the public library API.
- README scope walls: **not** a repository or supply-chain scanner.
- Gap issues filed (#17 filesystem scan, #18 external tools, #19 eval harness).
- MIT `LICENSE` aligned with `Cargo.toml`.

**Decision:** prefer **documented limitations** over feature claims. That honesty later extended to FP triage tables in the README and Wave A assessment docs.

---

## Wave A — harden what already existed

Documented in the Jul 8 assessment/roadmap commit (`00a4646`) and marked completed in PR [#27](https://github.com/tzervas/security-mcp/pull/27) / maintenance review [#25](https://github.com/tzervas/security-mcp/pull/25):

| ID | Outcome (as claimed on main after Wave A) |
|----|-------------------------------------------|
| S-A1 | HTTP token auth (`SECURITY_MCP_TOKENS` / bearer) + refuse insecure remote bind without token or `ALLOW_INSECURE_BIND=1` |
| S-A2 | MCP stdio e2e in `tests/smoke.rs` (`test_mcp_stdio_e2e`) |
| S-A3 | WebSocket/docs honesty (optional `--websocket` flag exists; not default) |
| S-A4 | `timeout_ms` + rate limit knobs enforced (unit coverage for limiter / timeout) |
| S-A5 | FP triage narrative in README |

**Decision:** harden the screening MCP **before** promoting wrap/proxy. Inferred rationale: a proxy without bind safety / auth would amplify exposure for any child tool.

---

## Tero index and agent workflow (Jul 2026)

| Step | Evidence |
|------|----------|
| Layer-1 corpus index under `docs/tero-index/` | Commits `8caf532`, `369dcc3`, `b8b2851`, etc. |
| `AGENTS.md` for cabal-devmelopner + tero | `e7d5c2a` |
| Local check parity emphasized | PR [#22](https://github.com/tzervas/security-mcp/pull/22) (`ebadbaf`) — `./scripts/check.sh` as primary gate |

**Decision:** keep a **local** primary gate (`scripts/check.sh`: fmt + clippy `-D warnings` + build + test) so agents and humans are not solely dependent on remote CI. Remote later returned to push/PR fleet workflows (see below); the local gate remains authoritative for day-to-day work.

---

## Fleet CI and self-hosted runners

| Step | Evidence |
|------|----------|
| Route linux x64 jobs to self-hosted podman | PR [#26](https://github.com/tzervas/security-mcp/pull/26) |
| P26 fleet standards (badges, issue close-on-main policy) | PRs [#33](https://github.com/tzervas/security-mcp/pull/33)/[#34](https://github.com/tzervas/security-mcp/pull/34) |
| Self-hosted resource alignment / gate hardening | PRs [#38](https://github.com/tzervas/security-mcp/pull/38)/[#40](https://github.com/tzervas/security-mcp/pull/40) |
| REUSE license bootstrap | PR [#31](https://github.com/tzervas/security-mcp/pull/31); remaining path debt tracked as [#32](https://github.com/tzervas/security-mcp/issues/32) |

**Decision:** fleet-standard Actions on self-hosted runners with honest badges on `main`. Memory/cgroup constraints on fleet runners forced explicit `CARGO_BUILD_JOBS` and debug-info reductions in `ci.yml` (commented in-tree) — inferred from workflow comments after OOM/SIGKILL incidents on the fleet.

---

## Wave B — wrap / proxy path (DRAFT on main)

| Step | Evidence |
|------|----------|
| Source concepts on long-lived branch `security-proxy-integration` | Still present on remote; bulletin cites it as triage source. |
| Wrap mode landed on `main` | PR [#28](https://github.com/tzervas/security-mcp/pull/28) (`fd24164`) — CLI `--wrap` / env, `proxy_status` / `proxy_configure`, subprocess forward, optional WS + SSE. |
| Post-merge honesty | PR [#29](https://github.com/tzervas/security-mcp/pull/29) — bulletin **Status remains DRAFT**. |
| STABLE checklist + consumer acks recorded | PR [#30](https://github.com/tzervas/security-mcp/pull/30) |

**Decision:** ship wrap **as DRAFT** with an explicit STABLE promotion checklist rather than claiming production proxy readiness. Real child-MCP roundtrip remains `#[ignore]` in `tests/proxy_integration.rs` until a fixture binary lands.

Multiple open PRs into `dev` still attempt STABLE promotion (#36, #39, #42, #45, #46 as of measurement) — not merged to `main` at the measured SHA.

---

## Production polish and 0.2.0-alpha

| Step | Evidence |
|------|----------|
| 5-min path, MCP examples, CLAUDE.md, pre-commit | PR [#35](https://github.com/tzervas/security-mcp/pull/35) (`0.1.7-alpha` lineage) |
| Release polish → tag `v0.2.0-alpha` | PR [#41](https://github.com/tzervas/security-mcp/pull/41) / release `v0.2.0-alpha` |

**Decision:** bump to **0.2.0-alpha** (not non-alpha 0.2.0). CHANGELOG states Wave B proxy/wrap and eval harness remain open relative to a non-alpha cut. Operator contract: remain 0.x until a human authorizes 1.x.

---

## Wire-format and scanner honesty fixes (late Jul 2026)

| Step | Evidence |
|------|----------|
| MCP camelCase on the wire | PR [#43](https://github.com/tzervas/security-mcp/pull/43) (`a5c524a`) — `initialize` / tools used snake_case so conforming clients could not connect; fixed with `#[serde(rename_all = "camelCase")]` + regression tests. |
| Trivy false positives on detector fixtures | PR [#44](https://github.com/tzervas/security-mcp/pull/44) (`1c6c82a`) — `trivy-secret.yaml` allow-rules scoped to placeholder token shapes so fleet-security stops training reviewers to ignore CRITICALs. |

These two fixes are structural: one unblocked real MCP clients; the other unblocked honest security scanning.

---

## Branch topology (as used in practice)

Historical flow mixed targets (`main`, `dev`, `integration`). Recent large features often PR’d directly to `main` (wrap, camelCase, trivy). `dev` still exists and currently hosts several open STABLE-promotion PRs.

**Inferred tension:** fleet branch contract prefers `work → dev → main`, but this repo’s default branch is `main` and much of the landed product history targets `main` directly. Future work should follow the operator contract unless a human directs otherwise.

---

## What did *not* happen (evidence-backed absences)

- **No crates.io publication** observed from release list (GitHub tags only: `v0.1.0` … `v0.2.0-alpha`).
- **No labeled precision/recall corpus** landed (issue #19 still open).
- **No STABLE wrap promotion** on `main` (bulletin DRAFT; ignored integration test).
- **No 1.x version** and none proposed under operator policy.

---

## Key decisions summary

1. **Content screener, not repo scanner** — keeps scope local to MCP text.
2. **Heuristic detectors only** — ship useful signals; do not claim DLP/compliance accuracy.
3. **Wave A before trust in proxy** — auth, bind safety, e2e, rate/timeout first.
4. **Wrap as DRAFT** — code on main, STABLE gated on real child MCP tests + consumer acks.
5. **Local `./scripts/check.sh` as primary gate** — fmt/clippy/build/test parity for agents.
6. **0.x-alpha until human says otherwise** — including keeping `0.2.0-alpha` rather than implying GA.
7. **Wire format must match MCP clients** — camelCase is load-bearing (#43).
8. **Scanner allowlists must not blind real findings** — shape-scoped trivy/gitleaks rules (#44).
