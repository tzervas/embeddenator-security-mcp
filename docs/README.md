# Docs index — security-mcp

Project-management and deep documentation. Front door for humans deciding whether to use the tool: root [README.md](../README.md).

## Project management (start here for PM questions)

| Doc | Question it answers |
|-----|---------------------|
| [DEVELOPMENT-PATH.md](DEVELOPMENT-PATH.md) | How we got here; decisions and rejected alternatives (evidence-cited). |
| [CURRENT-STATE.md](CURRENT-STATE.md) | What works **today**, measured (commands + output, VERIFIED/UNVERIFIED). |
| [ROADMAP.md](ROADMAP.md) | Planned work and **what would unblock** each item. |
| [ASSESSMENT.md](ASSESSMENT.md) | Gap analysis / maturity snapshot (keep in sync with CURRENT-STATE). |

## Operations and standards

| Doc | Topic |
|-----|--------|
| [LOCAL_CHECKS.md](LOCAL_CHECKS.md) | `./scripts/check.sh`, pre-commit, local vs remote gates |
| [FLEET_STANDARDS.md](FLEET_STANDARDS.md) | Fleet CI/security workflows, issue-close policy, badges |
| [bulletins/security-mcp-wrap.md](bulletins/security-mcp-wrap.md) | Wrap/proxy interface bulletin (DRAFT until STABLE checklist clears) |

## Client snippets and indices

| Path | Topic |
|------|--------|
| [mcp.example.json](mcp.example.json) | Claude Desktop MCP example |
| [tero-index/](tero-index/) | Layer-1 Tero corpus index for agents |

## Agent entrypoints (repo root)

- [AGENTS.md](../AGENTS.md) — cabal / tero workflow  
- [CLAUDE.md](../CLAUDE.md) — cargo / check commands for agents  
- [CONTRIBUTING.md](../CONTRIBUTING.md) — contribution basics  
- [REUSE-DEBT.md](../REUSE-DEBT.md) — license path debt notes  
