# Proof-annotation conventions

This document explains **why F\* fragments appear in the Rust source** and
**where every piece of a proof lives**, for the crates verified with hax
(currently `libcrux-ml-kem` and `libcrux-ml-dsa`; see [PROOFS.md](PROOFS.md)
for what is verified). The rule set is deliberately small enough to check by
eye: placement is decided by *what kind of thing an annotation is*, never by
its size or by solver convenience.

## The placement rule

| What | Where | Why |
|---|---|---|
| Function contracts (`#[hax_lib::requires]`, `#[ensures]`) | On the function | The contract *is* the function's specification; it must be readable where the function is read. |
| Solver settings (`#[hax_lib::fstar::options]`) | On the function | Per-declaration resource limits; one line. |
| Loop invariants (`hax_lib::loop_invariant!`) and decreases clauses | In the loop | Syntactically bound to their position. |
| Ghost snapshots (`#[cfg(hax)] let …`) | In the body | Name an intermediate state so a later proof step can refer to it; erased from compiled code. |
| Body proof scripts (`proof!(…)`) | In the body, at the step they justify | The position-bound proof of *this* function's obligations: revealing an opaque definition, instantiating a named lemma at concrete arguments, asserting a bridging fact. Everything not position-bound is *not* here. |
| Named theory — every reusable F\* definition (`val`, `let`, `Lemma`) | Hand-written companion modules in `proofs/fstar/spec/` | Real, independently verified F\* with a name; reviewed as ordinary F\* files, reused across functions. |
| Reference specifications | `specs/<algorithm>/` (Hacspec style), plus `#[cfg(hax)]` spec modules in-crate | What the code is proven *against*. |

`#[hax_lib::fstar::before]`/`after` attributes are **reserved** for text that
hax must inject verbatim and that cannot be a named definition: one-line
solver/visibility directives (`[@@ "opaque_to_smt"]`, `#restart-solver`) and
interface includes. A multi-line F\* definition inside a `before`/`after`
string is a convention violation unless it carries an exception tag (below).

Body proof scripts are **never** relocated into companion modules, even when
technically possible: partial factoring produces two conventions whose
discriminator is solver behaviour, which cannot be explained or predicted.
Scripts *call* named companion lemmas; they don't move into them.

## Exception tags

Some theory blocks cannot move out of the Rust source for hard technical
reasons. Each such block carries an in-code tag on the line above it:

```rust
// proof-residence: locked(own-const)   — cites this module's extracted constants/types/
//                                        functions (moving it would create a module cycle)
// proof-residence: spec-host           — this module's own spec-predicate vocabulary and
//                                        its lemma API, cited by same-file contracts
// proof-residence: clean-context       — proof saturates in a companion's clean context;
//                                        kept where the host function's ambient facts hold
// proof-residence: hint-keystone       — proof is cold-fragile; relocation shifts
//                                        recorded solver hints (needs restructure first)
// proof-residence: cold-gate           — file keeps --z3refresh for cold-start stability
```

`scripts/annotation_lint.py` scans both crates and reports every multi-line
definition block in a `before`/`after` attribute that lacks a tag. (Reporting
mode during the migration; CI-enforcing once the sweep is complete.)

## The trust surface

Unproven obligations (`admit`, `assume`, `assume val`, `magic`,
`--admit_smt_queries true`, lax-checked modules) are enumerable mechanically and
tracked as a **monotonically non-increasing ledger**. The ledger is computed
*only* from build artifacts by [`scripts/trust_ledger.py`](scripts/trust_ledger.py)
(four planes: F\* obligations, extraction coverage, Makefile `SLOW`/`ADMIT`,
post-extraction patches — see [scripts/README-trust-ledger.md](scripts/README-trust-ledger.md)),
so no source marker can shrink the reported surface. A committed baseline lives at
each crate's `proofs/trust-ledger.baseline.json`; `trust_ledger.py --check` fails CI
whenever the surface grows. A postcondition on a function whose proof is admitted is
a *trusted assumption* and must carry a source comment justifying it against the code.
See each crate's `proofs/…verification_status.md` for the per-function tier.

### Declaring an inline trust obligation

A body-level `admit ()` / `assume (…)` must be *declared*, not written raw. Use the
in-crate wrappers instead of `proof!(…)`, and label the enclosing fn:

```rust
#[libcrux_macros::trusted(inline-admit)]      // fn-level summary label (mandatory)
fn f() {
    trusted_admit!("hax-limitation: <one-line why this is trusted>");
    // ...
}
```

- `trusted_admit!("<cat>: reason")` / `trusted_assume!("<cat>: reason", r#"assume (…)"#)`
  are byte-identical to the raw `proof!` mechanism (the reason is Rust-only) — they add
  a category+reason a reviewer and the reconciler can read, nothing more.
- Category prefix ∈ { `unprovable-termination:`, `hax-limitation:`, `trusted-extern:`,
  `validated-axiom:`, `pending-proof(<ref>):`, `slow-proof:` }; long prose stays a comment.
- Every fn with a body wrapper **must** carry the matching-kind
  `#[libcrux_macros::trusted(inline-admit|inline-assume)]` label and vice-versa.
- Enforced by `scripts/annotation_lint.py` (V2 reason-format, V2b label↔body sync,
  V3 ban raw `proof!("admit ()")`/`proof!(assume …)`) and `trust_ledger.py --check`.

### Declaring a whole-function trust obligation

Whole-function trust — a `lax` / `panic_free` body, an `opaque` type, or an
`exclude`d item — is declared with the same `#[libcrux_macros::trusted(…)]`
attribute, parameterized by kind + reason. The wrapper EMITS the `hax_lib`
mechanism the site used before (byte-identical extraction) and adds the
machine-readable category+reason a reviewer and the reconciler can read:

```rust
#[libcrux_macros::trusted(panic_free, "pending-proof(#1234): <one-line why>")]
fn f() { … }
```

| kind         | emitted hax mechanism (under `cfg(hax)`)          |
|--------------|---------------------------------------------------|
| `lax`        | `hax_lib::fstar::verification_status(lax)`         |
| `panic_free` | `hax_lib::fstar::verification_status(panic_free)`  |
| `opaque`     | `hax_lib::opaque`                                  |
| `exclude`    | `hax_lib::exclude`                                 |

- The mechanism is emitted under `cfg_attr(hax, …)`, so a normal build is
  unaffected and under hax it reduces to exactly the prior attribute.
- The `"<category>: <reason>"` argument is mandatory (unlike the inline-*
  summaries); the category prefix is checked by `annotation_lint.py` (V2), and a
  reason-less wrapper is flagged, not silently ignored.
- A `panic_free` / `lax` body admits its `ensures` unchecked, so the reason
  documents the *existing* justification for that trusted post — it is not a new
  assumption (see "The trust surface"). Long prose stays as an adjacent comment.

### Declaring a companion-axiom trust obligation

A hand-written companion module in `proofs/fstar/spec/` may contain a genuine
**axiom** — an `assume val`, an `assume (…)`, or a `let … = admit ()` that models
a primitive/intrinsic F\* cannot verify (a movemask bound, a PSHUFB semantics, a
hash oracle, a `to_le_bytes` byte formula). Each such axiom carries a machine-
readable tag on the line directly above the declaration:

```fstar
[@@ "trusted: <category>: <one-line reason>"]
assume val lemma_movemask_ps_bound (a: …) : Lemma (…)
```

- The `<category>` is the same vocabulary as the Rust markers
  (`validated-axiom:`, `trusted-extern:`, `pending-proof(<ref>):`, …). `annotation_lint.py`
  V4 strips the `trusted:` prefix and validates the remainder with `reason_ok`.
- **One tag per axiom**: V4 checks the per-file bijection `#tags == #obligations`
  for every git-tracked `spec/` module (ml-dsa 6, ml-kem 21).
- Keep reasons **token-safe**: never write the bare word `assume`, or the text
  `admit ()` / `magic ()` / `assume val` / `admit_smt_queries true`, inside a
  reason. The plane-1 obligation scanner intentionally does not mask string
  literals (it mirrors `fstar_admits`), so a stray token would read as a real
  obligation. `trust_scan.mask_trusted_reason_strings` blanks `"trusted: …"`
  interiors as a belt-and-suspenders backstop, but token-safe reasons keep the
  scanner in agreement with `fstar_admits`.
- `[@@ "trusted: …"]` is an inert string attribute; it changes no VC. But editing
  a hint-carrying companion `.fst` invalidates its `.checked` and re-proves its
  dependents cold in CI — batch tag edits and let CI do the closure build.

### Declaring a module-level trust obligation — module/config mirrors

Two module-level trust surfaces are mirrored the same way, in the git-tracked
**authority** file (the `.fst`/`.fsti` are gitignored/extracted, so a per-source
header would be clobbered on re-extraction):

- **Verified-on-cadence (`SLOW_MODULES`) / admitted (`ADMIT_MODULES`)** — a
  `# trusted-module: <module> : <category>: <reason>` comment in the F\* extraction
  `Makefile` next to the module lists. `annotation_lint.py` V5 checks the
  bijection `{SLOW ∪ ADMIT} == {annotated modules}`, that ADMIT_MODULES is empty
  (the ratchet target — `trust_ledger` `reconcile()` blocks *growth*, V5 asserts
  the absolute 0), and `reason_ok` on each.
- **Dropped from extraction (`-i` filters)** — a `# trusted-module: <token> :
  <category>: <reason>` comment in `hax.py` / `hax.sh` for every `-<crate>::…`
  module-exclusion filter (an *absent* module is worse than an admitted one, see
  [`…verification_status.md`]). V6 checks the bijection
  `{exclusion tokens} == {annotations}` + `reason_ok`. (In `hax.sh` the reasons
  sit above the `\`-continued command, since a trailing comment would break the
  line continuation.)

V4/V5/V6 run on the committed tree (no extraction needed) and are wired into both
`annotation_lint.py` and `trust_ledger.py --check`.

## Reading a verified function (reviewer quick answers)

- *"Why is this F\* fragment in the code?"* — Every F\* fragment in a `.rs`
  file is one of: the function's contract, a loop invariant, a `proof!`
  script (position-bound by construction), a one-line solver directive, or a
  tagged exception with its reason at the site.
- *"Where are the proof annotations?"* — Contracts and scripts at each
  function; all named theory in `proofs/fstar/spec/*.fst`; reference specs in
  `specs/`; nothing proof-related hides anywhere else.
