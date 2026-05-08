# power2round_one_ring_element — Refactor Decision (2026-05-04)

## Decision: no refactor; accept admit on `power2round_vector` for Sprint A

## Background

`power2round_vector` (`src/arithmetic.rs:85`) has verification_status `panic_free`
with a bare `admit ()` body. Its post (t0 bounded by pow2 12, t1 in [0, pow2 10))
is declared and trusted.

The body proof fails because hax cannot cleanly extract the simultaneous
`&mut t0[i]` + `&mut t1[i]` mutable-slice-element borrows passed to
`power2round_one_ring_element` inside the outer loop. A previous sprint
attempted the body proof and reverted.

Three options were considered:
- A: return t1 as value instead of `&mut` (eliminates one simultaneous borrow)
- B: take t0 by value, return tuple
- C: accept admit, defer body discharge to Sprint B

## Decision: Option C

**No Rust signature changes.**

Rationale:
- `power2round_vector`'s post is trusted. `generate_key_pair` uses it on Thu
  without needing the body to be proven.
- Sprint A success criterion explicitly allows this admit.
- Body discharge is a Sprint B task; adding it to that backlog is acceptable.

## Sprint A impact

- `generate_key_pair` panic-free flip (Thu): proceeds using the trusted post.
- Wed agents: no `power2round_one_ring_element` refactor work needed.
- Sprint B backlog: add `power2round_vector` body proof (needs new approach or
  hax upstream fix for the simultaneous `&mut slice[i]` pattern).
