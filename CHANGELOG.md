# Changelog

All notable changes to security-mcp will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-07-26

### Changed
- **Graduated off the `-alpha` prerelease suffix: `0.2.0-alpha` → `0.2.0`.** The tagged
  `v0.2.0-alpha` build was not usable as an MCP server at all — clients refused the snake_case
  handshake it emitted (see Fixed, below), so nothing meaningfully shipped under that tag. With
  the wire format fixed, and with the new `.cz.toml` (`major_version_zero = true`) now enforcing
  the 0.x compatibility line on every future bump, the `-alpha` suffix stopped adding information
  the `0.x` major doesn't already carry. Mechanically, `cz bump` finalizes a prerelease to the
  plain version by default (`--prerelease alpha` would instead mint `0.2.0-a1`, a spelling
  matching neither this tag lineage nor any prior release) — see `docs/VERSIONING.md`.
- Added `.cz.toml` (commitizen config, `major_version_zero = true`) so version lockstep across
  `Cargo.toml`, `README.md`, and `docs/ASSESSMENT.md` is enforced by tooling instead of by hand.
  Added `docs/VERSIONING.md` and a `CONTRIBUTING.md` pointer to it.
- Dependencies: `base64` `^0.22` → `^0.23`.
- CI: `actions/checkout` `4` → `7`, `astral-sh/setup-uv` `5` → `7`.

### Added
- Wave B S-B3 wrap integration tests: real OS child JSON-RPC mock + binary-level
  `security-mcp --stdio --wrap` forward (`tests/proxy_integration.rs`). Closes the
  code-presence side of the old “proxy not on main” High gap (code already on main
  via PR #28; docs honesty in ASSESSMENT/ROADMAP).

### Security
- **Eliminated two permanent false-positive CRITICALs from `fleet-security`.** Trivy's secret
  scanner flagged this crate's own detector fixtures in `src/patterns.rs`
  (`ghp_xxxx…` / `gho_xxxx…` placeholders asserting `GITHUB_TOKEN` matches) as GitHub tokens.
  gitleaks was already allowlisted for exactly these (0.2.0-alpha); trivy was not, so every
  scan reported CRITICALs that could never be actioned — the kind of finding that trains
  reviewers to ignore the scanner precisely where it matters most.
  Added `trivy-secret.yaml` (auto-loaded; `--secret-config` defaults to it, so no workflow
  change). The allow-rule is scoped **by token shape, not by path**: only provider-prefixed
  tokens whose body is a run of placeholder characters are allowed. Verified by negative test —
  a correctly-formed 36-character `ghp_` token injected into that same file is still reported
  CRITICAL, so the file is not blinded.

### Fixed
- **MCP handshake was unparseable to conforming clients.** `initialize` serialized its result in
  snake_case (`protocol_version`, `server_info`, `list_changed`; also `input_schema` and `is_error`
  on the tool path) where the MCP wire format is camelCase, so clients dropped the connection with
  no diagnostic — the server itself built, started, and returned a well-formed JSON-RPC frame, so
  manual stdio smoke tests looked healthy. **In plain terms: the tagged `v0.2.0-alpha` build was
  unusable as an MCP server — no conforming client could complete the handshake.** Added
  `#[serde(rename_all = "camelCase")]` to `ToolsCapability`, `InitializeResult`, `Tool` and
  `CallToolResult`. Fields with an explicit `#[serde(rename = …)]` (`type`, `enum`) are unaffected.
  **This is a breaking wire-format change for any client that was adapted to the broken
  snake_case spelling to work around the alpha build** — such a client must be updated to read
  camelCase to keep working against `0.2.0`.

### Added
- Regression tests pinning the MCP wire names in both directions (camelCase present, snake_case
  absent) so a struct edit cannot silently break the handshake again.
- README: Claude Code (CLI) registration via `claude mcp add`, alongside the existing
  Cursor / VS Code and Claude Desktop examples.

## [0.2.0-alpha] - 2026-07-21

### Added
- Fleet CI standards: PR/issue templates, `fleet-ci.yml`, `fleet-security.yml`, meta issue-close/reopen workflows (`docs/FLEET_STANDARDS.md`).

### Changed
- CI hardened for self-hosted runners: no bare `sudo apt-get`, pinned gitleaks install, guarded system deps, `CARGO_BUILD_JOBS`/toolchain setup for fleet cargo jobs.
- gitleaks allowlist scoped to intentional detector-fixture secret shapes (`src/patterns.rs`, smoke tests) so fleet-security scans stay green without masking real findings.
- Version bump `0.1.7-alpha` → `0.2.0-alpha` (CI/fleet hardening; still alpha — Wave B proxy/wrap path and eval harness remain open per `docs/ROADMAP.md`).

## [0.1.7-alpha] - 2026-07-16

### Added
- 5-minute path in README (`cargo build` / `cargo test` / `security-mcp --stdio`).
- MCP host examples: `docs/mcp.example.json` (Claude Desktop), `.mcp.json.example` (Cursor / VS Code).
- `CLAUDE.md` with cargo / check.sh command surface for agents.
- Optional `.pre-commit-config.yaml` (fmt + pre-push full `scripts/check.sh`); primary gate remains `./scripts/check.sh`.

### Changed
- `scripts/check.sh` and `docs/LOCAL_CHECKS.md` note pre-commit as optional convenience.
- Version bump `0.1.6-alpha` → `0.1.7-alpha` (docs / agent-surface production polish).

### Prior unreleased notes (landed earlier on main)
- `LICENSE` file (MIT, matching `Cargo.toml`'s declared license).
- `tests/smoke.rs`: black-box integration smoke tests against the public API.
- README: clarified content/text screener (not repo/CVE scanner).
- Chore: tero-index / AGENTS / local CI parity hygiene.

## [0.1.0-alpha.2] - 2025-01-22

### Changed
- **BREAKING**: Renamed crate from `embeddenator-security-mcp` to `security-mcp`
- Improved prompt injection patterns for better detection coverage
- Replaced manual Default implementations with `#[default]` derive attribute
- Fixed unused variable warning in PII redaction
- Updated test cases for more realistic injection scenarios

### Fixed
- Pattern matching for "disregard all" variant of prompt injection
- Clippy warnings for derivable Default implementations

## [0.1.0-alpha.1] - 2025-01-19

### Added
- Initial security MCP server implementation
- JSON-RPC 2.0 over stdio transport
- Security screening tools:
  - `screen_input` - Screen user input for security threats
  - `screen_output` - Screen AI output for PII/secrets
  - `check_safe` - Quick safety check
  - `scan_full` - Comprehensive security scan
- Detection capabilities:
  - PII detection (email, SSN, credit cards, phone numbers)
  - Secret detection (API keys, tokens, passwords)
  - Injection detection (SQL, command, prompt injection)
- Configurable severity thresholds
- Risk scoring and automated blocking

[Unreleased]: https://github.com/tzervas/security-mcp/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/tzervas/security-mcp/compare/v0.2.0-alpha...v0.2.0
[0.2.0-alpha]: https://github.com/tzervas/security-mcp/compare/v0.1.7-alpha...v0.2.0-alpha
[0.1.7-alpha]: https://github.com/tzervas/security-mcp/compare/v0.1.0-alpha.2...v0.1.7-alpha
[0.1.0-alpha.2]: https://github.com/tzervas/security-mcp/compare/v0.1.0-alpha.1...v0.1.0-alpha.2
[0.1.0-alpha.1]: https://github.com/tzervas/security-mcp/releases/tag/v0.1.0-alpha.1