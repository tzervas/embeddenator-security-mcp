# Current state — security-mcp

**Layer:** back (measured).  
**Measurement date (UTC):** 2026-07-25  
**Measured commit:** `1c6c82a034c156c3424372556cfc3de2b1f6b89c` (`1c6c82a`, `main` tip at measure time)  
**Crate version (Cargo.toml / `--version`):** `0.2.0-alpha`  
**Toolchain used here:** `cargo 1.98.0-nightly` / `rustc 1.98.0-nightly` for ad-hoc runs; `./scripts/check.sh` forced `stable` via rustup and also passed.

Every capability below is tagged **VERIFIED** (exercised in this run or by a test that passed in this run) or **UNVERIFIED** (not exercised, ignored test, or only code-present).

---

## Snapshot

| Item | Value |
|------|--------|
| Role | MCP **content/text screener** (prompt-injection / PII / secrets heuristics) |
| Maturity | Alpha — not a compliance or DLP certificate |
| Default bind | HTTP `127.0.0.1:3001` unless `--stdio` |
| Wrap/proxy | Code on `main`, bulletin **DRAFT** (not STABLE) |
| Registry publish | Not measured / no crates.io claim |

---

## Capabilities

| Capability | Status | Evidence |
|------------|--------|----------|
| `cargo build` / binary runs | **VERIFIED** | `cargo run -- --help` and `--version` succeeded |
| Version string | **VERIFIED** | `security-mcp 0.2.0-alpha` |
| MCP stdio transport | **VERIFIED** | Manual JSON-RPC session + `test_mcp_stdio_e2e` |
| `initialize` camelCase wire | **VERIFIED** | Manual stdio: response fields `protocolVersion`, `serverInfo`, `listChanged`; unit tests `protocol::tests::*` |
| Tools: `screen_input`, `screen_output`, `screen_content`, `check_safe`, `redact_content`, `get_config` | **VERIFIED** (list + sample call) | `tools/list` returned all eight tools; `check_safe` on `"hello world"` → `is_safe: true` |
| Tools: `proxy_status`, `proxy_configure` | **VERIFIED** listed; **UNVERIFIED** end-to-end configure/forward | Present in `tools/list`; no live admin-token configure + child roundtrip in this run |
| Input screening (injection heuristics) | **VERIFIED** | Unit/pipeline tests + smoke library API |
| Output screening (PII/secrets heuristics) | **VERIFIED** | `known_secret_is_flagged`, detector unit tests |
| Benign content not flagged (smoke case) | **VERIFIED** | `benign_string_is_not_flagged` |
| Batch / rayon path | **UNVERIFIED** | Dependency present; no dedicated test output in this run |
| HTTP mode listen + request | **UNVERIFIED** | Code path present (`SecurityServer::run`); not started in this measure run |
| HTTP token auth / remote bind refusal | **VERIFIED** (unit) | `server::tests::test_bind_safety_all_cases` passed; live HTTP auth **UNVERIFIED** |
| Rate limiter | **VERIFIED** (unit) | `server::tests::test_rate_limiter` |
| Screening timeout | **VERIFIED** (unit) | `pipeline::tests::test_screening_timeout` |
| Wrap mode (`--wrap` + child) | **UNVERIFIED** as product STABLE | Scaffold tests pass; `real_child_mcp_stdio_roundtrip` **ignored** |
| WebSocket MCP (`--websocket`) | **UNVERIFIED** | Flag exists; no live WS client exercised |
| SSE audit stream (`--sse`) | **UNVERIFIED** | Flag exists; no live SSE client exercised |
| Detection precision/recall | **UNVERIFIED** | No labeled corpus; issue #19 open |
| JSON-RPC **notifications** (no `id`) | **UNVERIFIED** / known defect | See [Known defects](#known-defects-observed) — parse error observed |
| Claude Desktop / Cursor live host attach | **UNVERIFIED** | Config examples exist; hosts not launched here |
| crates.io install path | **UNVERIFIED** | Not published in this measurement |

---

## How this was measured

Resource bound: `export CARGO_BUILD_JOBS=3` for all cargo invocations.

### 1. Identity and toolchain

```text
$ git rev-parse HEAD
1c6c82a034c156c3424372556cfc3de2b1f6b89c

$ date -u +%Y-%m-%dT%H:%M:%SZ
2026-07-25T23:21:14Z

$ cargo --version && rustc --version
cargo 1.98.0-nightly (a335d47ff 2026-06-26)
rustc 1.98.0-nightly (4c9d2bfe4 2026-07-01)
```

### 2. CLI help and version

```text
$ cargo run -- --version
security-mcp 0.2.0-alpha

$ cargo run -- --help
# (abridged) options include: --stdio, --host, --port, --pii, --secrets,
# --injection, --block-high, --allow-warnings, --allow-redacted,
# --rate-limit, --tokens, --wrap, --wrap-command, --wrap-arg,
# --websocket, --sse
```

### 3. Full test suite

```text
$ cargo test --all-features
# ...
# lib unit tests:
test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s
# bin unit tests: 0 tests
# tests/proxy_integration.rs:
test result: ok. 2 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.12s
#   (ignored: real_child_mcp_stdio_roundtrip — requires real child MCP binary)
# tests/smoke.rs:
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 11.57s
# doc-tests: 0 tests
```

**Totals this run:** **32 passed**, **0 failed**, **1 ignored**.

### 4. Primary local gate

```text
$ ./scripts/check.sh
# fmt --check + clippy -D warnings + build + test (stable toolchain)
# ...
OK: checks passed (security-mcp)
```

Exit code **0**. Re-ran tests under stable inside the script: same 32 pass / 1 ignore pattern.

### 5. Runnable stdio quickstart (manual)

```bash
printf '%s\n' \
  '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"docs-measure","version":"0.0.1"}}}' \
  '{"jsonrpc":"2.0","method":"notifications/initialized"}' \
  '{"jsonrpc":"2.0","id":2,"method":"tools/list"}' \
  '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"check_safe","arguments":{"content":"hello world"}}}' \
  | timeout 15 cargo run --quiet -- --stdio
```

Observed (trimmed):

```text
{"jsonrpc":"2.0","id":1,"result":{"capabilities":{"tools":{"listChanged":true}},"protocolVersion":"2024-11-05","serverInfo":{"name":"security-mcp","version":"0.2.0-alpha"}}}
{"jsonrpc":"2.0","id":0,"error":{"code":-32700,"message":"Parse error"}}
{"jsonrpc":"2.0","id":2,"result":{"tools":[ ... screen_input, screen_output, screen_content, check_safe, redact_content, get_config, proxy_status, proxy_configure ...]}}
{"jsonrpc":"2.0","id":3,"result":{"content":[{"text":"{\n  \"direction_checked\": \"both\",\n  \"input_safe\": true,\n  \"is_safe\": true,\n  \"output_safe\": true\n}","type":"text"}],"isError":false}}
```

Stderr: `Starting Security MCP Server in stdio mode` (logging correctly on stderr).

### 6. Remote CI (GitHub Actions)

```bash
gh api '/repos/tzervas/security-mcp/actions/runs?per_page=10'
# and branch=main sample:
```

| When (UTC) | Workflow | Event | Branch / SHA | Conclusion |
|------------|----------|-------|--------------|------------|
| 2026-07-23T04:53:22Z | CI | push | `main` `1c6c82a` | **success** |
| 2026-07-23T04:53:22Z | fleet-ci | push | `main` `1c6c82a` | **success** |
| 2026-07-23T04:53:22Z | fleet-security | push | `main` `1c6c82a` | **success** |
| 2026-07-25T17:54:04Z | fleet-ci / fleet-security / CI | pull_request | `jules-…` `2bfdf6c` | **success** |
| 2026-07-23T04:53:22Z | close-issues-on-main | pull_request | `claude/harden-secret-scan` | **failure** (meta workflow; not the build gate) |

### 7. Branch protection / required checks

```bash
gh api /repos/tzervas/security-mcp/rules/branches/main
```

Rules present: deletion block, non-fast-forward, pull_request (0 approving reviews). **No required status checks** in the ruleset payload. Auto-merge is therefore unsafe for unverified work (operator policy).

---

## Known defects and gaps observed

| Item | Severity | Evidence |
|------|----------|----------|
| JSON-RPC notifications without `id` fail parse | Med | Manual stdio: `notifications/initialized` → `-32700 Parse error`. `JsonRpcRequest.id` is required (`src/protocol.rs`). MCP clients commonly send id-less notifications. **Not fixed in this docs PR.** |
| Wrap STABLE incomplete | High for proxy users | `tests/proxy_integration.rs` ignores `real_child_mcp_stdio_roundtrip`; bulletin remains DRAFT. |
| Detection quality unbenchmarked | Med | Issue #19; README already discloses. |
| REUSE path coverage debt | Low/Med | Issue #32. |
| `docs/LOCAL_CHECKS.md` claimed “manual only” CI | Docs bug | Workflows `fleet-ci.yml` / `ci.yml` trigger on **push and pull_request** as well as `workflow_dispatch`. Corrected in this suite. |
| `docs/ASSESSMENT.md` claimed wrap “paused on branch” | Docs bug | Wrap merged via PR #28; DRAFT on main. Corrected in this suite. |
| Multiple concurrent STABLE-promotion PRs on `dev` | Process risk | Open PRs #36, #39, #42, #45, #46 (titles claim STABLE) while main bulletin is still DRAFT — do not treat those branches as measured main. |

---

## Test inventory (what the suite actually covers)

| Suite | Count (this run) | Focus |
|-------|------------------|--------|
| `src/**` unit tests | 26 pass | patterns, detectors, pipeline, screeners, tools, rate limit, bind safety, camelCase wire, echo subprocess mock |
| `tests/smoke.rs` | 4 pass | public API secret/benign, config defaults, stdio e2e `tools/list` + `screen_input` |
| `tests/proxy_integration.rs` | 2 pass, 1 ignore | wrap disabled status, router scaffold; **not** real child MCP |
| doctests | 0 | — |

---

## Environment notes for re-measurement

- Bound cargo parallelism: `CARGO_BUILD_JOBS=3` (or lower on small hosts).
- `./scripts/check.sh` uses `RUSTUP_TOOLCHAIN` default **stable** and clears sccache wrappers unless `SECURITY_MCP_USE_SCCACHE=1`.
- Stdio protocol requires **no banner on stdout**; logs go to stderr (verified).
- Prefer re-running this file’s command list after any detector or protocol change before trusting badges alone.
