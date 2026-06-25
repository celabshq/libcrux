# Contributing

Third-party contributions to libcrux are welcome, be it in the form of an issue
report or a feature request via [issues](https://github.com/cryspen/libcrux)
or in the form of new code and documentation via [pull requests](https://github.com/cryspen/libcrux/pulls).

## Repository Structure

### Crates

We are in the process of restructuring the directory structure of the repository.
While many old crates do not yet follow the new structure, please use it for new
crates:

```
/crates/                   -- contains all crates except `libcrux`
/crates/primitives         -- contains crypto primitive crates
/crates/primitives/aead    -- the crate `libcrux-aead`
/crates/algorithms         -- contains crypto algorithm crates
/crates/algorithms/chacha20poly1305 -- the crate `libcrux-chacha20poly1305`
/crates/specs/             -- specification crates
/crates/utils/             -- miscellaneous crates
/crates/utils/macros       -- the `libcrux-macros` crate
/crates/utils/secrets      -- the `libcrux-secrets` crate
/crates/testing/           -- testing crates
/crates/testing/kats       -- the `libcrux-kats` crate
/crates/testing/test-utils -- the `libcrux-test-utils` crate
/crates/sys/...            -- the sys crates
```

### Per-Crate Structure

#### Crate Metadata

**Readme:** Every crate has its own readme. Ensure that the Cargo.toml refers to
that instead of the one at the repo root.

**License:** Make sure the code is appropriately licensed, most likely with
Apache-2.0 and MIT.

#### Extracted C Code

Some crates contain scripts and configuration for extracting C code from the
Rust code, and possibly a pre-extracted C version of the crate.
Relative to the crate root, the structure we aim to use is:

```
/Cargo.toml -- we are at the crate root
/extracts/
/extracts/$name/extract.sh   -- the script that extracts the code
/extracts/$name/extract.yaml -- the eurydice config for the extraction
/extracts/$name/generated/   -- the extracted code
/extracts/$name/...          -- per-extraction extra data
```

where `$name` identifies a C extraction in a particular configuration.

## Coding style

In order to help contributors adhere to the style guidelines of this project,
we've provided a [Python3 script](git-hooks/pre-commit.py) that serves as a Git pre-commit hook.

In addition to Python3, you will also need to install [rustfmt](https://github.com/rust-lang/rustfmt) and the [black](https://github.com/psf/black) formatter to use this hook. Once they're installed, simply
run `./git-hooks/setup.py` in the project root directory.

### Working with Proofs

Most of the code in libcrux is formally verified (each crate marks its verification status at the top of its Readme file).

Some of the crates contain Rust code generated from the [HACL* project](https://hacl-star.github.io).
Modifications to this code would mean that the proofs in HACL* no longer apply, so for modifications to this code,
get in touch with the libcrux or HACL* maintainers.

Other crates (specifically `libcrux-ml-kem` and `libcrux-ml-dsa`) are verified using [hax](https://cryspen.com/hax) and  [F\*](https://fstar-lang.org).
To re-run proofs on these crates, follow the instructions in their Readme files.
These proofs depend on specific versions of the `hax` binary, the `hax-lib` proof and annotations library, and the `fstar` proof assistant.

The version of `hax` is set globally for the `libcrux` workspace via the `hax-lib` Cargo dependency.
The developer is expected to install the `hax` binary at the same version as `hax-lib` by cloning the [hax repository](https://github.com/hacspec/hax),
checking out the revision corresponding to the `hax-lib` version release, and running `./setup.sh` as documented in the hax Readme.

We use the same version throughout libcrux, and inconsistencies can lead to verification or build failures (see e.g. [issue #1125](https://github.com/cryspen/libcrux/issues/1125)).
The F\*  proofs use a [Makefile](./fstar-helpers/Makefile.generic), which relies on Cargo and the workspace's version of `hax-lib` to locate F\* libraries.
Specifically, those under the [`proof-libs` directory in hax-lib](https://github.com/cryspen/hax/tree/main/hax-lib/proof-libs).

If you need to work with a specific version of `hax-lib` or `proof-libs`, update the `hax-lib` dependency in the workspace's `Cargo.toml`.  
You may use a [path dependency](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-path-dependencies), or a [Git dependency](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories) for development purposes.
If your pull request includes such a non-default dependency (e.g., a Git or path-based reference), **please mention it clearly in the PR description**.
Using a non-[crates.io](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-cratesio) dependency in a PR is acceptable for development. However, **before merging**, the dependency must be updated to point to a released version of `hax`.
If a new `hax` release is required for your PR to be merged, please [open an issue on the hax repository](https://github.com/cryspen/hax/issues/new?title=Release%20request&body=For%20libcrux%20PR%20%23XX,%20I%20need%20a%20release%20of%20hax.) to request it.

### Policy on AI-Generated Submissions

We are supportive of responsible AI use in software development.
AI can be a helpful tool, and we do not object to contributors using it as part of their workflow for writing code, drafting issues, or conducting security research.

However, submissions to this project must meet a reasonable quality bar and remain suitable for human review.
AI-generated content can be produced very quickly, but it frequently contains hallucinations, inaccurate claims, repetitive reports, or non-functional code.
Reviewing these submissions requires significant maintainer time and attention.

To keep our review process sustainable, we enforce the following guidelines:
- **Human Validation is Required**: Please ensure that any AI-assisted issue, pull request, or security report has been carefully checked and verified by a human before it is submitted.
- **Provide Actionable Context**: Issues and PRs must include clear, verified context. Bug reports must have actual reproduction steps, and security reports must have a working proof-of-concept (PoC) rather than purely theoretical or AI-hallucinated exploits.
- **Low-Quality Submissions Will Be Closed**: If our maintainers determine that an issue, pull request, or security report appears to be mass-generated, low-quality, or insufficiently validated, it may be closed without detailed review or response.
- **Use of AI tools must be disclosed**: Contributors must disclose whether AI tools were used in their submission and detail the capacity in which they were utilized.

This is not a ban on AI-assisted contributions.
It is a practical policy intended to focus our efforts on submissions that are specific, verified, and actionable.

## General Contribution Workflow
When you are ready to contribute code or documentation:
1. **Check Existing Issues**: Before starting work, check the issue tracker to see if someone is already working on the same problem.
2. **Open an Issue First**: For significant changes or new features, please open an issue to discuss your proposal with the maintainers before writing the code.
3. **Drafting your Pull Request**:
    - Keep your PRs focused and as small as reasonably possible.
    - Ensure your code adheres to the existing style and formatting of the project.
    - Include tests for any new functionality or bug fixes.
4. **Review Process**: A maintainer will review your PR. Be prepared to answer questions or make requested changes.
