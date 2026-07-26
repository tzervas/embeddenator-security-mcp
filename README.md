# security-mcp

<!-- FLEET-BADGES:BEGIN -->
[![CI](https://github.com/tzervas/security-mcp/actions/workflows/fleet-ci.yml/badge.svg?branch=main)](https://github.com/tzervas/security-mcp/actions/workflows/fleet-ci.yml?query=branch%3Amain)
[![Security](https://github.com/tzervas/security-mcp/actions/workflows/fleet-security.yml/badge.svg?branch=main)](https://github.com/tzervas/security-mcp/actions/workflows/fleet-security.yml?query=branch%3Amain)
<!-- FLEET-BADGES:END -->

MCP server for security screening: prompt-injection defense, PII detection, and secrets scanning
**in text content passed through an MCP conversation** (tool calls, model input/output).

Sit it in front of other tools/servers so inputs and outputs can be screened consistently.

**Status:** Alpha (`0.2.0-alpha`). Heuristic regex + entropy detectors — not a compliance certificate.

## Quickstart

```bash
git clone https://github.com/tzervas/security-mcp.git
cd security-mcp

cargo build
cargo test --all-features

# MCP over stdio (hosts attach to stdin/stdout; Ctrl-C to stop)
cargo run -- --stdio
```

Expected: tests pass; `security-mcp --stdio` waits for JSON-RPC lines (no banner on stdout; logs on stderr).

Full local gate: `./scripts/check.sh`. Client snippets: [docs/mcp.example.json](docs/mcp.example.json)
(Claude Desktop) and [.mcp.json.example](.mcp.json.example) (Cursor / VS Code).

Measured state (commands + pass counts): [docs/CURRENT-STATE.md](docs/CURRENT-STATE.md).

## What this is not

**Content/text screener only** — not a repository, dependency, or supply-chain scanner. It does not
walk a filesystem or git tree, resolve dependency graphs, or replace `cargo audit`, `gitleaks`,
`trivy`, or `semgrep`. Use those for CVE/SBOM or git-history work; use this for **in-flight text**
in an MCP session.

## What it does today

- MCP JSON-RPC over **stdio** or **HTTP** (loopback default).
- **Input** screening (prompt-injection / suspicious patterns) and **output** screening (PII, secrets, entropy).
- Tools: `screen_input`, `screen_output`, `screen_content`, `check_safe`, `redact_content`, `get_config`, plus wrap helpers `proxy_status` / `proxy_configure`.
- Optional **wrap** mode (DRAFT — real child-MCP STABLE tests not yet on main): see [docs/bulletins/security-mcp-wrap.md](docs/bulletins/security-mcp-wrap.md).
- Detection is **regex + entropy**, not ML/DLP — expect false positives and negatives.

## Running

```bash
cargo run -- --help
cargo run -- --stdio          # MCP clients (required for stdio hosts)
cargo run -- --host 127.0.0.1 --port 3001   # HTTP mode
```

## MCP client config

| Host | Example |
|------|---------|
| Cursor / VS Code Copilot | [.mcp.json.example](.mcp.json.example) |
| Claude Desktop | [docs/mcp.example.json](docs/mcp.example.json) |
| Claude Code (CLI) | `claude mcp add` — below |

```bash
# -s user = every project; -s project = one repo
claude mcp add security-mcp -s user -- /absolute/path/to/security-mcp --stdio
claude mcp list
```

Cursor / VS Code (`mcp.json` or `.vscode/mcp.json`):

```json
{
  "servers": {
    "security-mcp": {
      "type": "stdio",
      "command": "security-mcp",
      "args": ["--stdio"]
    }
  }
}
```

Claude Desktop (`claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "security-mcp": {
      "command": "security-mcp",
      "args": ["--stdio"]
    }
  }
}
```

> **`--stdio` is required** for stdio hosts. Without it the server defaults to HTTP on port 3001.

## Local checks

```bash
./scripts/check.sh            # fmt + clippy + build + test (primary gate)
pre-commit install            # optional; see .pre-commit-config.yaml
```

Details: [docs/LOCAL_CHECKS.md](docs/LOCAL_CHECKS.md). Agents: [AGENTS.md](AGENTS.md), [CLAUDE.md](CLAUDE.md).

## False positive expectations

Heuristic rules only — treat matches as signals to review, not certainties. Rates below are
**qualitative expectations**, not measured precision/recall (see issue #19 / roadmap S-C1).

| Detector category | Expected FP pressure | Common causes | Recommendation |
|---|---|---|---|
| **PII** | Low to medium | Example emails/addresses, tutorial names | Lower severity or disable checkers in public/tutorial contexts |
| **Secrets & keys** | Medium | High-entropy hashes, Base64, git SHAs, build artifacts | Raise entropy threshold or prefer explicit secret shapes for highly technical domains |
| **SQL / cmd injection** | Low | Code snippets, SQL tutorials, shell examples | Prefer warn/review over block in trusted developer workflows |
| **LDAP injection** | Extremely low | Refined for nested LDAP query structures | Single `!` / `*` occurrences are usually fine |
| **Prompt injection** | Medium | Docs/tutorials that quote jailbreak phrasing | Review manually or bypass for administrative prompts |

## Docs (depth)

| Doc | Contents |
|-----|----------|
| [docs/README.md](docs/README.md) | Index of all docs |
| [docs/CURRENT-STATE.md](docs/CURRENT-STATE.md) | Measured capabilities (VERIFIED / UNVERIFIED) |
| [docs/DEVELOPMENT-PATH.md](docs/DEVELOPMENT-PATH.md) | History and decisions |
| [docs/ROADMAP.md](docs/ROADMAP.md) | Planned work + unblockers |
| [docs/ASSESSMENT.md](docs/ASSESSMENT.md) | Gap / maturity notes |

## License

MIT
