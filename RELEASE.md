# Release


## Crates

Main crate: `libcrux`

Crates that can be used directly
- blake2
- sha2
- sha3
- ml-dsa
- ml-kem
- chacha20poly1305
- hkdf
- hmac
- curve25519
- ecdsa
- ed25519
- rsa

Additional protocols
- psq

Higher level crates
- ecdh
- kem

### Utility crates

- hacl-rs
- traits
- macros
- poly1305
- p256

### Other libraries

- sys/libjade

### Legacy

- sys/hacl

### Independently Versioned

- blake2

## Conventions

- **Versions.** We use semantic versioning, and each crate has its own
    version. Before releasing `X.Y.Z`, we publish `X.Y.Z-pre.1` (then
  `-pre.2`, ...) of the same crates, so that users can test them.
- **Changelogs.** Every crate has a `CHANGELOG.md`. Changes that are
    not released yet go under `## [X.Y.Z] (Unreleased)`, where `X.Y.Z`
    is the next version of the crate, so the next version is decided
    when a change lands (see `CONTRIBUTING.md`). Only semver-relevant
    changes (to the public API with the default features, the features
    or `rust-version`) and changes to hax annotations need necessarily
    need changelog entries.
- **Dependency Pins.** Workspace crates depend on each other with
    exact pins (`=x.y.z`) in `[workspace.dependencies]`. Releasing a
    crate therefore means releasing every crate that depends on it,
    directly or indirectly.
- **When to release.** User-facing changes on the Rust side:
    changes to the published Rust sources other than comments, changes
    to `Cargo.toml` other than dev-dependencies, and changes to
    inherited workspace settings. This includes hax annotations and
    hax-only code (`hax_lib` attributes and contracts, `cfg(hax)`),
    because consumers run hax on the published sources.  Comments,
    documentation, non-Rust files (such as extracted C code), tests,
    benches, examples and fuzz targets don't need a release.
- **Tags.** Every final release is tagged `<crate>-v<version>`, e.g.
    `libcrux-ml-kem-v0.0.12`, on the merge commit of the release pull request.
    Pre-releases are not tagged.

## Releasing

1. **Plan.** On an up-to-date `main`, find the crates with changes
    since their last release tag, and crates that depend on
    them. Check that the changelog of each crate has a `## [X.Y.Z]
    (Unreleased)` section that covers its changes, and fix missing
    sections first.
   
2. **Pre-release pull request.** On a new branch, set the version of
    each crate to `X.Y.Z-pre.1`, update the pins on it in
    `[workspace.dependencies]` and in other manifests (such as
    `specs/`), and rename its changelog section to `## [X.Y.Z-pre.1]
    (YYYY-MM-DD)`. Open a pull request for this and after it is merged,
    publish the crates from the merge commit on `main` in dependency
    order (see below), with a dry run first. Pre-releases are not
    tagged.

3. **Testing.** Let users test the pre-release. If needed, release
    another pre-release (`-pre.2`).

4. **Release pull request.** On a new branch, remove the `-pre.N`
    suffixes from the versions, pins and changelog headings and set the
    release date. Open and merge a pull request as above.

5. **Publish and tag.** From the merge commit on `main`, publish the
    crates in dependency order, then create and push a tag
    `<crate>-v<version>` for each published crate.

6. **Afterwards.** Check that the documentation built on docs.rs.

### Publish Order

This snippet prints a valid publishing order of the crates in the workspace:

```
cargo depgraph --workspace-only --all-deps |
  gvpr 'E { string tl = gsub($.tail.label, " ", "-"), hl = gsub($.head.label, " ", "-"); printf("%s %s\n", hl, tl); }' |
  tsort
```

It requires [cargo-depgraph](https://github.com/jplatte/cargo-depgraph) and
[graphviz](https://graphviz.org/download/).
