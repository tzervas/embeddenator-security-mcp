# Versioning and releases

## The policy

`security-mcp` is versioned **0.x.y** and stays there. Commitizen enforces this with
`major_version_zero = true` in [`.cz.toml`](../.cz.toml). Moving to **1.x.x requires an explicit
human authorization** — full production readiness, hardening, and a maintainer decision. **No
agent, and no automation, may cut or propose a 1.x.x release.**

## Under `major_version_zero`, MINOR is the breaking position

This is the detail most often got wrong. While the major is pinned at 0:

| Change                        | Bump      | Example           |
| ------------------------------ | --------- | ----------------- |
| `fix:`                         | PATCH     | 0.2.0 → 0.2.1     |
| `feat:`                        | **MINOR** | 0.2.0 → **0.3.0** |
| `feat!:` / `BREAKING CHANGE:`  | **MINOR** | 0.2.0 → **0.3.0** |

A consumer pinning "latest compatible" therefore pins the **minor** — `security-mcp = "0.2"`, or a
moving tag `v0.2`. Never `"1"`, and never a bare `v1` tag: under this scheme `0.2` and `0.3` are
*incompatible* releases, exactly as `1.x` and `2.x` would be after a 1.0 cut.

With `major_version_zero` **absent**, commitizen treats a breaking change as MAJOR and mints
`1.0.0` on the first `feat!:` — a version nobody authorized. (This crate ran without a `.cz.toml`
at all until 0.2.0; that gap is closed now.)

## Why the `-alpha` suffix was dropped at 0.2.0

Releases through `v0.2.0-alpha` carried a `-alpha` prerelease identifier (`0.1.0-alpha.1` …
`0.2.0-alpha`). As of `0.2.0` the crate is on plain `0.x.y` versions, for two independent reasons:

1. **The tagged `v0.2.0-alpha` build did not work as an MCP server.** It serialized the
   `initialize` response (and the tool-call path) in snake_case
   (`protocol_version`/`server_info`/`list_changed`/`input_schema`/`is_error`) where the MCP wire
   format requires camelCase, so conforming clients could not complete the handshake. Once that
   was fixed (`0.2.0`, see `CHANGELOG.md`), there was no reason to re-mint another prerelease of a
   build that was, functionally, the first one that worked.
2. **Commitizen does not have a clean way to keep re-minting `-alpha`.** Under
   `version_scheme = "semver"`, a plain `cz bump` on a prerelease **finalizes** it to the release
   version — `0.2.0-alpha` → `0.2.0` — regardless of whether the triggering commit is `fix:`,
   `feat:`, or breaking. Staying on the alpha line requires `cz bump --prerelease alpha`, which
   does not reproduce the existing tag spelling: it produces `0.2.0-a1`, not `0.2.0-alpha.1` or
   another `0.2.0-alpha`. That spelling matches neither this repo's prior tags nor any prior
   release, so it would itself be a new, undocumented convention.

The `0.x` major already signals "no compatibility promise" under `major_version_zero`; the
`-alpha` suffix was not adding information beyond that once the build actually worked. Catalog and
README prose may keep describing the project as "alpha" / "active development" in narrative
terms — that is a maturity statement, not a version-string concern, and the two are allowed to
diverge.

If a future prerelease line is needed (e.g. staging an experimental build ahead of a minor bump),
use `cz bump --prerelease alpha` deliberately and knowingly accept the `-aN` spelling it produces,
or mint the prerelease tag by hand and document the convention here first.

## Version files

[`.cz.toml`](../.cz.toml) lists every place the version appears under `version_files`, so
`cz bump` moves them together and they cannot drift:

- `Cargo.toml` — `[package] version`
- `README.md` — the `**Status:**` line
- `docs/ASSESSMENT.md` — the `**Crate:** security-mcp <version>` header line
- `.cz.toml` itself — `version`. NOTE: this one is **not** in `version_files`;
  commitizen owns and rewrites its own `version` key directly. Listing it in
  `version_files` would be wrong, not redundant.

`Cargo.lock` is `.gitignore`d in this repo (not tracked), so it is not a lockstep concern; running
`cargo build` after a bump regenerates it locally from `Cargo.toml`. Do not hand-edit any tracked
version file — run the tool:

```bash
cz bump --yes --dry-run     # show what would happen, change nothing
cz bump                     # move every version file + create the tag
cz version --project        # what this project currently claims to be
```

The `version` key in `.cz.toml` is the version cz bumps *from*, so it must track the newest
released tag. If it lags behind the tag list, the next `cz bump` re-mints a version that already
has a tag.

`docs/tero-index/index.json` is a generated corpus index and separately carries stale changelog
anchors from old releases; it is refreshed by its own generator script, not by `cz bump` or by
hand-editing.

## A GitHub Release is not a registry publication

These are two different things, and conflating them is how sibling repos in this fleet drifted:

- **A git tag / GitHub Release** is a marker plus notes. It publishes nothing consumable.
- **A crates.io publication** is the artifact dependents actually resolve.

When you claim a version is released, say *where*. Publishing to crates.io is a separate,
deliberate step — it is not automatic on tag push, and until it runs, the version is not released
to crates.io consumers even though the tag and GitHub Release exist.

## Release steps

1. Land work on `dev` via a work branch — never straight to `main`.
2. `cz bump` on the release branch: this moves every version file and creates the tag locally.
3. Open the release PR `dev` → `main`. Merge with a **merge commit**, never a squash.
4. Push the tag; the `release` workflow (`workflow_dispatch`) builds the GitHub Release.
5. **Publishing to crates.io is a separate, deliberate step.** It is not automatic, and until it
   runs, the version is not released to consumers.
