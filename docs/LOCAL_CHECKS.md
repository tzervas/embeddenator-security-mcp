# Local checks (CI parity)

Day-to-day quality gates should run **locally** so you are not solely dependent on remote CI.
Remote workflows also run on push/PR (fleet + legacy CI); local remains the fastest feedback loop
for agents and developers.

> **Correction (2026-07-25):** An earlier revision of this file said workflows were
> **manual only** (`workflow_dispatch`). That is **false** for current
> `.github/workflows/fleet-ci.yml`, `fleet-security.yml`, and `ci.yml` — they trigger on
> `push` / `pull_request` to trunk branches **and** support `workflow_dispatch`.
> The comment inside `scripts/check.sh` may still say “manual-only remote”; treat the
> workflow YAML as source of truth for triggers (docs-only pass does not edit scripts).

## Run everything the primary gate expects

```bash
./scripts/check.sh
```

This is the **primary** quality gate (fmt + clippy `-D warnings` + build + test).
`check.sh` prefers the **stable** toolchain via rustup when available.

Optional:

```bash
./scripts/check.sh --quick   # reserved; see script for actual flags
./scripts/check.sh --fix    # apply formatters instead of --check
```

### Optional pre-commit

```bash
pre-commit install
pre-commit run --all-files
```

Config: [`.pre-commit-config.yaml`](../.pre-commit-config.yaml) (whitespace/toml hooks +
`cargo fmt --check`; full `./scripts/check.sh` on `pre-push`). Pre-commit is convenience —
`./scripts/check.sh` remains authoritative.

## Resource limits

On shared hosts, bound cargo concurrency, e.g.:

```bash
export CARGO_BUILD_JOBS=3
./scripts/check.sh
```

## Tero index

```bash
# from a checkout that can see the generator (sibling tero-mcp recommended):
python3 ../tero-mcp/scripts/generate_lite_index.py --root "$(pwd)"
```

Artifacts land in `docs/tero-index/` (`index.json`, `INDEX.md`, `MANIFEST.toml`, `README.md`).

## Remote

- **fleet-ci** / **fleet-security**: push/PR to `main`/`dev` (+ schedule where configured); self-hosted podman runners. See [FLEET_STANDARDS.md](FLEET_STANDARDS.md).
- **CI** (`ci.yml`): also push/PR; local-parity job.
- Manual: GitHub **Actions → Run workflow** (`workflow_dispatch`).

Measured green/red samples: [CURRENT-STATE.md](CURRENT-STATE.md).
