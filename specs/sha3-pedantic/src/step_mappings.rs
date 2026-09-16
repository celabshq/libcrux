//! The five step mappings — FIPS 202, Sec. 3.2.
//!
//! Each takes a state array `A` and returns the updated array `A′`; only `ι`
//! takes a second input, the round index `i_r`. The bodies below follow the
//! numbered Steps of Algorithms 1-6 literally, including `ρ`'s offsets (which
//! the Standard computes by a walk rather than tabulating) and `ι`'s round
//! constants (which come out of the `rc` LFSR of Algorithm 5, not a table).

use crate::state_array::StateArray;

/// `a mod b` for possibly negative `a` — the Standard's `mod` is the
/// mathematical one, e.g. `(z - 1) mod w` in Algorithm 1.
fn imod(a: i64, b: i64) -> usize {
    (((a % b) + b) % b) as usize
}

/// Algorithm 1: `θ(A)`.
pub fn theta<const W: usize>(a: &StateArray<W>) -> StateArray<W> {
    let w = W as i64;

    // 1. C[x, z] = A[x, 0, z] ⊕ A[x, 1, z] ⊕ A[x, 2, z] ⊕ A[x, 3, z] ⊕ A[x, 4, z].
    let mut c = [[false; W]; 5];
    for x in 0..5 {
        for z in 0..W {
            c[x][z] = a.a[x][0][z] ^ a.a[x][1][z] ^ a.a[x][2][z] ^ a.a[x][3][z] ^ a.a[x][4][z];
        }
    }

    // 2. D[x, z] = C[(x-1) mod 5, z] ⊕ C[(x+1) mod 5, (z - 1) mod w].
    let mut d = [[false; W]; 5];
    for x in 0..5 {
        for z in 0..W {
            d[x][z] = c[imod(x as i64 - 1, 5)][z] ^ c[(x + 1) % 5][imod(z as i64 - 1, w)];
        }
    }

    // 3. A′[x, y, z] = A[x, y, z] ⊕ D[x, z].
    let mut out = *a;
    for x in 0..5 {
        for y in 0..5 {
            for z in 0..W {
                out.a[x][y][z] = a.a[x][y][z] ^ d[x][z];
            }
        }
    }
    out
}

/// Algorithm 2: `ρ(A)`.
pub fn rho<const W: usize>(a: &StateArray<W>) -> StateArray<W> {
    let w = W as i64;
    let mut out = StateArray::<W>::zero();

    // 1. For all z, A′[0, 0, z] = A[0, 0, z].
    for z in 0..W {
        out.a[0][0][z] = a.a[0][0][z];
    }

    // 2. Let (x, y) = (1, 0).
    let (mut x, mut y) = (1usize, 0usize);

    // 3. For t from 0 to 23:
    for t in 0..24i64 {
        // a. A′[x, y, z] = A[x, y, (z - (t+1)(t+2)/2) mod w].
        let offset = (t + 1) * (t + 2) / 2;
        for z in 0..W {
            out.a[x][y][z] = a.a[x][y][imod(z as i64 - offset, w)];
        }
        // b. (x, y) = (y, (2x + 3y) mod 5).
        let (nx, ny) = (y, (2 * x + 3 * y) % 5);
        x = nx;
        y = ny;
    }
    out
}

/// Algorithm 3: `π(A)`.
pub fn pi<const W: usize>(a: &StateArray<W>) -> StateArray<W> {
    let mut out = StateArray::<W>::zero();
    // 1. A′[x, y, z] = A[(x + 3y) mod 5, x, z].
    for x in 0..5 {
        for y in 0..5 {
            for z in 0..W {
                out.a[x][y][z] = a.a[(x + 3 * y) % 5][x][z];
            }
        }
    }
    out
}

/// Algorithm 4: `χ(A)`.
pub fn chi<const W: usize>(a: &StateArray<W>) -> StateArray<W> {
    let mut out = StateArray::<W>::zero();
    // 1. A′[x, y, z] = A[x, y, z] ⊕ ((A[(x+1) mod 5, y, z] ⊕ 1) · A[(x+2) mod 5, y, z]).
    for x in 0..5 {
        for y in 0..5 {
            for z in 0..W {
                out.a[x][y][z] = a.a[x][y][z] ^ (!a.a[(x + 1) % 5][y][z] & a.a[(x + 2) % 5][y][z]);
            }
        }
    }
    out
}

/// Algorithm 5: `rc(t)`.
///
/// `t` may be negative: Algorithm 7 indexes rounds from `12 + 2l - n_r`, which
/// is negative when `n_r > 12 + 2l` (Sec. 3.4 gives `KECCAK-p[b, 30]` as an
/// example), and Algorithm 6 evaluates `rc(j + 7·i_r)`.
pub fn rc(t: i64) -> bool {
    // 1. If t mod 255 = 0, return 1.
    let t = imod(t, 255);
    if t == 0 {
        return true;
    }
    // 2. Let R = 10000000.
    let mut r = [false; 9];
    r[0] = true;
    // 3. For i from 1 to t mod 255:
    for _ in 1..=t {
        // a. R = 0 || R;
        let mut shifted = [false; 9];
        shifted[1..9].copy_from_slice(&r[0..8]);
        r = shifted;
        // b. R[0] = R[0] ⊕ R[8];
        r[0] ^= r[8];
        // c. R[4] = R[4] ⊕ R[8];
        r[4] ^= r[8];
        // d. R[5] = R[5] ⊕ R[8];
        r[5] ^= r[8];
        // e. R[6] = R[6] ⊕ R[8];
        r[6] ^= r[8];
        // f. R = Trunc_8[R].
        r[8] = false;
    }
    // 4. Return R[0].
    r[0]
}

/// Algorithm 6: `ι(A, i_r)`.
pub fn iota<const W: usize>(a: &StateArray<W>, i_r: i64) -> StateArray<W> {
    // 1. A′ = A.
    let mut out = *a;

    // 2. Let RC = 0^w.
    let mut round_constant = [false; W];

    // 3. For j from 0 to l, RC[2^j - 1] = rc(j + 7·i_r).
    for j in 0..=StateArray::<W>::L {
        round_constant[(1usize << j) - 1] = rc(j as i64 + 7 * i_r);
    }

    // 4. For all z, A′[0, 0, z] = A′[0, 0, z] ⊕ RC[z].
    //    Lane (0, 0) is read out, updated and written back in one piece: a
    //    compound assignment through the nested projection `out.a[0][0][z]`
    //    is what the Lean extraction cannot follow.
    let mut lane = out.a[0][0];
    for z in 0..W {
        lane[z] ^= round_constant[z];
    }
    out.a[0][0] = lane;
    out
}
