# Versioning and releases

## The policy

`security-mcp` is versioned **0.x.y** and stays there. Commitizen enforces this with
`major_version_zero = true` in [`.cz.toml`](../.cz.toml). Moving to **1.x.x requires an explicit
human authorization** — full production readiness, hardening, and a maintainer decision. **No
agent, and no automation, may cut or propose a 1.x.x release.**

## Under `major_version_zero`, MINOR is the breaking position

This is the detail most often got wrong. While the major is pinned at 0:

| Change                        | Bump      | Example           |
| ----------------------------- | --------- | ----------------- |
| `fix:`                        | PATCH     | 0.2.0 → 0.2.1     |
| `feat:`                       | PATCH     | 0.2.0 → 0.2.1     |
| `feat!:` / `BREAKING CHANGE:` | **MINOR** | 0.2.0 → **0.3.0** |

A consumer pinning "latest compatible" therefore pins the **minor** — `security-mcp = "0.2"`, or a
moving tag `v0.2`. Never `"1"`, and never a bare `v1` tag: under this scheme `0.2` and `0.3` are
*incompatible* releases, exactly as `1.x` and `2.x` would be after a 1.0 cut.

With `major_version_zero` **absent**, commitizen treats a breaking change as MAJOR and mints
`1.0.0` on the first `feat!:` — a version nobody authorized.

## The `-alpha` suffix: read this before your first `cz bump`

This crate is on a prerelease line (`0.2.0-alpha`, and `0.1.6-alpha` before it). Commitizen does
**not** treat that suffix the way the hand-maintained tags have. Two things to know, both measured
against this configuration rather than assumed:

**1. `cz` normalises the spelling.** It reads `0.2.0-alpha` and reports it as `0.2.0-a0`:

```
$ cz version --project
0.2.0-a0
```

`Cargo.toml` still says `0.2.0-alpha` and `cargo metadata` still reports `0.2.0-alpha` — the two
agree on the *version*, they disagree on how to *spell* the prerelease identifier.

**2. A plain `cz bump` finalises the prerelease — it does not make another alpha.** Under semver a
prerelease sorts *before* its release, so any increment resolves to the plain version:

```
$ cz bump --yes --dry-run          # on a fix: commit
bump: version 0.2.0-a0 → 0.2.0
increment detected: PATCH

$ cz bump --yes --dry-run          # on a feat!: / BREAKING CHANGE commit
bump: version 0.2.0-a0 → 0.2.0
increment detected: MINOR
```

That is correct semver, and it is probably what you want — but it is not "bump the alpha".

**To stay on the alpha line**, pass `--prerelease alpha`. Note the spelling it produces:

```
$ cz bump --yes --dry-run --prerelease alpha
bump: version 0.2.0-a0 → 0.2.0-a1
```

`0.2.0-a1`, not `0.2.0-alpha.1`. So the tag would be `v0.2.0-a1`, breaking the `-alpha` convention
the existing tags and crates.io releases use.

**Recommendation:** finalise off the prerelease line at the next release — let `cz bump` take this
to plain `0.2.0` and drop `-alpha` from the scheme entirely. The suffix is not buying anything the
`0.x` major does not already communicate (0.x *is* the "no compatibility promise" signal), and
keeping it means fighting the tool on every bump. This is a maintainer decision, not an agent one.

## Version files

[`.cz.toml`](../.cz.toml) lists every place the version appears under `version_files`, so
`cz bump` moves them together and they cannot drift:

- `Cargo.toml` — `[package] version`
- `docs/ASSESSMENT.md` — the `**Crate:** security-mcp <version>` header line
- `.cz.toml` itself — `version`

`Cargo.lock` records this package's own version too; refresh it with `cargo build` after a bump.
Do not hand-edit any of these — run the tool:

```bash
cz bump --yes --dry-run     # show what would happen, change nothing
cz bump                     # move every version file + create the tag
cz version --project        # what this project currently claims to be
```

The `version` key in `.cz.toml` is the version cz bumps *from*, so it must track the newest
released tag. If it lags behind the tag list, the next `cz bump` re-mints a version that already
has a tag.

## A GitHub Release is not a registry publication

These are two different things, and conflating them is how sibling repos in this fleet drifted:

- **A git tag / GitHub Release** is a marker plus notes. It publishes nothing consumable.
- **A crates.io publication** is the artifact dependents actually resolve.

For this crate they currently disagree:

| | Newest |
| --- | --- |
| git tag | `v0.2.0-alpha` |
| **crates.io (`security-mcp`)** | **`0.1.6-alpha`** |

`0.2.0-alpha` is tagged and released on GitHub but has **never been published to crates.io**.
Anyone reading the tag list would reasonably conclude otherwise. When you claim a version is
released, say *where*.

## Release steps

1. Land work on `dev` via a work branch — never straight to `main`.
2. `cz bump` on the release branch: this moves every version file and creates the tag locally.
3. Open the release PR `dev` → `main`. Merge with a **merge commit**, never a squash.
4. Push the tag; the `release` workflow (`workflow_dispatch`) builds the GitHub Release.
5. **Publishing to crates.io is a separate, deliberate step.** It is not automatic, and until it
   runs, the version is not released to consumers.
