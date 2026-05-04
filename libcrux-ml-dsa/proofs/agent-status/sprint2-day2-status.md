# Sprint 2 Day 2 — Matrix body proofs

Started: 2026-05-06 12:50

## Plan
- Task 4: vector_times_ring_element (simplest, single flat loop) — start
- Task 1: compute_as1_plus_s2 (nested + sequential loop)
- Task 3: compute_matrix_x_mask (nested loop, mask copy)
- Task 2: compute_w_approx (nested, weak ensures)

## Notes
- invert_ntt_montgomery ensures `is_i32b_array_opaque 4211177` (Sprint 1 tight bound)
- Posts mostly want FIELD_MAX (8380416) — need weakening via `is_i32b_array_larger`
  (no SMTPat; manual call required).

## Active sub-task: Task 4 vector_times_ring_element
- 12:50 begin: write loop invariant + post-invert weakening lemma.
