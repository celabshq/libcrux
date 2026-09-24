// This file adapted from https://github.com/AU-COBRA/AUCurves/blob/d4159c1b8a91f2f832761c72f00caa8fb59ec472/safegcd-rs/src/safegcd.rs
//! Generic Bernstein-Yang divstep modular inversion.
//!
//! Port of `libsecp256k1`'s `secp256k1_modinv64.c` (Peter Dettman, 2020,
//! MIT), parameterized by const-generic limb count `N` so the same code
//! body services all prime sizes.  Each curve plugs in its own
//! `[ModInfo; N]` constant (modulus in 5×62, 7×62, 9×62 ... signed limbs;
//! `modulus^{-1} mod 2^62`; outer iteration count).
//!
//! Per-curve callable entry points live in sibling modules:
//!   `safegcd25519`, `safegcd_secp256k1`, `safegcd_p256`, `safegcd_bn254`, …
//!
//! Algorithm details: see `safegcd25519.rs` header.  All curves use the
//! same Wuille structure (δ₀ = 1/2, 10 outer × 59 divsteps for 256-bit;
//! scales linearly with prime bits).  This is the EUROCRYPT 2026 paper's
//! Inverter 2X structure (Bernstein–Chen–Harrison–Huang–Maxwell–Wang–
//! Wuille–Yang).

/// 2^62 − 1.
pub const M62: u64 = u64::MAX >> 2;
pub const M62_I: i64 = M62 as i64;

/// `N`×62-bit signed limb representation.  Value = sum(v[i]·2^(62·i)).
#[derive(Copy, Clone, Debug)]
pub struct Signed62<const N: usize> {
    pub v: [i64; N],
}

/// 2×2 transition matrix.  `[[u, v], [q, r]]`, scaled by 2^62 in the chunked
/// algorithm (the `_` suffix on `v_` is to avoid clashing with the variable
/// name `v` in callers).
#[derive(Copy, Clone, Debug)]
pub struct Trans2x2 {
    pub u: i64,
    pub v_: i64,
    pub q: i64,
    pub r: i64,
}

/// Prime metadata.  The same struct serves every curve via const generics.
pub struct ModInfo<const N: usize> {
    /// p in N×62 signed limbs.  All limbs ∈ [0, 2^62).
    pub modulus: [i64; N],
    /// `modulus^{-1} mod 2^62` = inverse of `modulus[0]` under arithmetic mod 2^62.
    pub modulus_inv_62: u64,
    /// Outer iteration count (each iteration is 59 divsteps).  For δ₀=1/2 and
    /// b-bit prime, `outer_iters ≥ ⌈(9437·b + 1)/(4096·59)⌉`.  Typically
    /// 10 for 256-bit, 15 for 381-bit, 20 for 509-bit, 30 for 761-bit.
    pub outer_iters: usize,
}

// ─── const-time Newton iteration for `m^{-1} mod 2^62` ────────────────────

/// Compute `m^{-1} mod 2^62` via 6 Newton iterations.
/// Requires `m & 1 == 1` (i.e. `m` is odd — true for the bottom limb of any
/// odd prime, the only case we use).
pub const fn modinv_62(m: u64) -> u64 {
    let mut x: u64 = 1;
    let mut i = 0;
    while i < 6 {
        x = x.wrapping_mul(2u64.wrapping_sub(m.wrapping_mul(x)));
        i += 1;
    }
    x & M62
}

// ─── 59-step CT divstep word loop (curve-independent) ─────────────────────

/// Compute the 2×2 transition matrix and new zeta after 59 divsteps.
/// Matrix is scaled by 2^62 (initial identity ×8 + 59 steps each ×2 = ×2^65).
///
/// libsecp256k1 `modinv64_impl.h:167`.  Only touches bottom limbs of f, g.
pub fn divsteps_59(mut zeta: i64, f0: u64, g0: u64) -> (i64, Trans2x2) {
    let mut u: u64 = 8;
    let mut v: u64 = 0;
    let mut q: u64 = 0;
    let mut r: u64 = 8;
    let mut f: u64 = f0;
    let mut g: u64 = g0;

    let mut i = 3;
    while i < 62 {
        let c1 = (zeta >> 63) as u64;
        let c2 = (g & 1).wrapping_neg();

        let x = (f ^ c1).wrapping_sub(c1);
        let y = (u ^ c1).wrapping_sub(c1);
        let z = (v ^ c1).wrapping_sub(c1);

        g = g.wrapping_add(x & c2);
        q = q.wrapping_add(y & c2);
        r = r.wrapping_add(z & c2);

        let mask1 = c1 & c2;
        zeta = (zeta ^ (mask1 as i64)).wrapping_sub(1);

        f = f.wrapping_add(g & mask1);
        u = u.wrapping_add(q & mask1);
        v = v.wrapping_add(r & mask1);

        g >>= 1;
        u = u.wrapping_shl(1);
        v = v.wrapping_shl(1);

        i += 1;
    }

    (
        zeta,
        Trans2x2 {
            u: u as i64,
            v_: v as i64,
            q: q as i64,
            r: r as i64,
        },
    )
}

// ─── generic (d,e) update mod p ───────────────────────────────────────────

/// `(d, e) ← t·(d, e) / 2^62  (mod p)`.
/// Const-generic over limb count N.  libsecp256k1 `modinv64_impl.h:411`.
pub fn update_de_62<const N: usize>(
    d: &mut Signed62<N>,
    e: &mut Signed62<N>,
    t: &Trans2x2,
    info: &ModInfo<N>,
) {
    let sd = d.v[N - 1] >> 63;
    let se = e.v[N - 1] >> 63;
    let mut md = (t.u & sd) + (t.v_ & se);
    let mut me = (t.q & sd) + (t.r & se);

    let mut cd: i128 = (t.u as i128) * (d.v[0] as i128) + (t.v_ as i128) * (e.v[0] as i128);
    let mut ce: i128 = (t.q as i128) * (d.v[0] as i128) + (t.r as i128) * (e.v[0] as i128);

    let cd_lo = cd as u64;
    let ce_lo = ce as u64;
    md = md.wrapping_sub(
        ((info
            .modulus_inv_62
            .wrapping_mul(cd_lo)
            .wrapping_add(md as u64))
            & M62) as i64,
    );
    me = me.wrapping_sub(
        ((info
            .modulus_inv_62
            .wrapping_mul(ce_lo)
            .wrapping_add(me as u64))
            & M62) as i64,
    );

    cd += (info.modulus[0] as i128) * (md as i128);
    ce += (info.modulus[0] as i128) * (me as i128);
    debug_assert_eq!((cd as u64) & M62, 0);
    debug_assert_eq!((ce as u64) & M62, 0);
    cd >>= 62;
    ce >>= 62;

    // Middle limbs: accumulate t·(d,e)[i] + modulus[i]·(md,me); store d.v[i-1].
    let mut i = 1usize;
    while i < N {
        cd += (t.u as i128) * (d.v[i] as i128) + (t.v_ as i128) * (e.v[i] as i128);
        ce += (t.q as i128) * (d.v[i] as i128) + (t.r as i128) * (e.v[i] as i128);
        // We unconditionally multiply by modulus[i] — for primes with zero
        // limbs (e.g., p25519 has zero middle limbs in libsecp256k1's encoding;
        // for p25519 in 2^62 radix they're all non-zero) the multiplication
        // by 0 is a no-op anyway.  Skipping with a runtime branch hurts CT.
        cd += (info.modulus[i] as i128) * (md as i128);
        ce += (info.modulus[i] as i128) * (me as i128);
        if i < N - 1 {
            d.v[i - 1] = (cd as i64) & M62_I;
            cd >>= 62;
            e.v[i - 1] = (ce as i64) & M62_I;
            ce >>= 62;
        } else {
            // Last middle limb: same logic, then store the final top limb.
            d.v[i - 1] = (cd as i64) & M62_I;
            cd >>= 62;
            e.v[i - 1] = (ce as i64) & M62_I;
            ce >>= 62;
            d.v[i] = cd as i64;
            e.v[i] = ce as i64;
        }
        i += 1;
    }
}

// ─── generic (f,g) update (no mod, exact divide by 2^62) ──────────────────

/// `(f, g) ← t·(f, g) / 2^62`.  No modular reduction.
/// libsecp256k1 `modinv64_impl.h:500`.
pub fn update_fg_62<const N: usize>(f: &mut Signed62<N>, g: &mut Signed62<N>, t: &Trans2x2) {
    let mut cf: i128 = (t.u as i128) * (f.v[0] as i128) + (t.v_ as i128) * (g.v[0] as i128);
    let mut cg: i128 = (t.q as i128) * (f.v[0] as i128) + (t.r as i128) * (g.v[0] as i128);

    debug_assert_eq!((cf as u64) & M62, 0);
    debug_assert_eq!((cg as u64) & M62, 0);
    cf >>= 62;
    cg >>= 62;

    let mut i = 1usize;
    while i < N {
        cf += (t.u as i128) * (f.v[i] as i128) + (t.v_ as i128) * (g.v[i] as i128);
        cg += (t.q as i128) * (f.v[i] as i128) + (t.r as i128) * (g.v[i] as i128);
        if i < N - 1 {
            f.v[i - 1] = (cf as i64) & M62_I;
            cf >>= 62;
            g.v[i - 1] = (cg as i64) & M62_I;
            cg >>= 62;
        } else {
            f.v[i - 1] = (cf as i64) & M62_I;
            cf >>= 62;
            g.v[i - 1] = (cg as i64) & M62_I;
            cg >>= 62;
            f.v[i] = cf as i64;
            g.v[i] = cg as i64;
        }
        i += 1;
    }
}

// ─── normalize d into [0, p) ──────────────────────────────────────────────

/// Bring `r ∈ (−2p, p)` to `[0, p)`; if `sign < 0`, also negate.
/// libsecp256k1 `modinv64_impl.h:88`.
///
/// **Constant-time discipline**: each sign-extension mask (`>> 63` to produce
/// 0 or -1) is routed through `core::hint::black_box` so LLVM can't pattern-
/// match the `((x >> 63) & y) + z` mask-and-merge into `if x < 0 { z+y } else { z }`.
/// Track F valgrind CT test caught two such regressions (lines 225 + 248);
/// libsecp256k1's CT test catches the analogous failure in its C code.
pub fn normalize_62<const N: usize>(r: &mut Signed62<N>, sign: i64, info: &ModInfo<N>) {
    // First conditional add: brings r ∈ (−p, p).
    let cond_add = core::hint::black_box(r.v[N - 1] >> 63);
    let mut i = 0;
    while i < N {
        r.v[i] += info.modulus[i] & cond_add;
        i += 1;
    }

    let cond_negate = core::hint::black_box(sign >> 63);
    let mut i = 0;
    while i < N {
        r.v[i] = (r.v[i] ^ cond_negate) - cond_negate;
        i += 1;
    }

    // Carry propagate.
    let mut i = 0;
    while i < N - 1 {
        r.v[i + 1] += r.v[i] >> 62;
        r.v[i] &= M62_I;
        i += 1;
    }

    // Second conditional add: brings r ∈ [0, p).
    let cond_add = core::hint::black_box(r.v[N - 1] >> 63);
    let mut i = 0;
    while i < N {
        r.v[i] += info.modulus[i] & cond_add;
        i += 1;
    }

    let mut i = 0;
    while i < N - 1 {
        r.v[i + 1] += r.v[i] >> 62;
        r.v[i] &= M62_I;
        i += 1;
    }
}

// ─── top-level generic inversion ──────────────────────────────────────────

/// Compute `x^{-1} mod p` (or 0 if `x ≡ 0`), where `p`, iteration count, and
/// limb count come from `info`.  The output `Signed62` has every limb in
/// `[0, 2^62)` and represents an integer in `[0, p)`.
pub fn invert<const N: usize>(x: Signed62<N>, info: &ModInfo<N>) -> Signed62<N> {
    let mut d: Signed62<N> = Signed62 { v: [0; N] };
    let mut e: Signed62<N> = Signed62 { v: [0; N] };
    e.v[0] = 1;
    let mut f: Signed62<N> = Signed62 { v: info.modulus };
    let mut g: Signed62<N> = x;
    let mut zeta: i64 = -1; // δ₀ = 1/2 encoding

    let mut i = 0;
    while i < info.outer_iters {
        let (new_zeta, t) = divsteps_59(zeta, f.v[0] as u64, g.v[0] as u64);
        zeta = new_zeta;
        update_de_62(&mut d, &mut e, &t, info);
        update_fg_62(&mut f, &mut g, &t);
        i += 1;
    }

    normalize_62(&mut d, f.v[N - 1], info);
    d
}

// ─── limb-format conversions (384-bit / 6×u64 saturated) ──────────────────

/// 6×u64 saturated → 7×62 signed.  For 381–384-bit primes.
pub fn from_saturated_6(x: &[u64; 6]) -> Signed62<7> {
    let m = M62 as i64;
    Signed62 {
        v: [
            (x[0] & M62) as i64 & m,
            (((x[0] >> 62) | (x[1] << 2)) & M62) as i64,
            (((x[1] >> 60) | (x[2] << 4)) & M62) as i64,
            (((x[2] >> 58) | (x[3] << 6)) & M62) as i64,
            (((x[3] >> 56) | (x[4] << 8)) & M62) as i64,
            (((x[4] >> 54) | (x[5] << 10)) & M62) as i64,
            (x[5] >> 52) as i64,
        ],
    }
}

/// 7×62 signed → 6×u64 saturated.
pub fn to_saturated_6(s: &Signed62<7>) -> [u64; 6] {
    let [v0, v1, v2, v3, v4, v5, v6] = [
        s.v[0] as u64,
        s.v[1] as u64,
        s.v[2] as u64,
        s.v[3] as u64,
        s.v[4] as u64,
        s.v[5] as u64,
        s.v[6] as u64,
    ];
    [
        v0 | (v1 << 62),
        (v1 >> 2) | (v2 << 60),
        (v2 >> 4) | (v3 << 58),
        (v3 >> 6) | (v4 << 56),
        (v4 >> 8) | (v5 << 54),
        (v5 >> 10) | (v6 << 52),
    ]
}

/// Const-fn: build a 7×62 `ModInfo` from a 6×u64 LE modulus.
pub const fn modinfo_from_6u64(modulus_6x64: [u64; 6], outer_iters: usize) -> ModInfo<7> {
    let m0 = modulus_6x64[0] & M62;
    let m1 = ((modulus_6x64[0] >> 62) | (modulus_6x64[1] << 2)) & M62;
    let m2 = ((modulus_6x64[1] >> 60) | (modulus_6x64[2] << 4)) & M62;
    let m3 = ((modulus_6x64[2] >> 58) | (modulus_6x64[3] << 6)) & M62;
    let m4 = ((modulus_6x64[3] >> 56) | (modulus_6x64[4] << 8)) & M62;
    let m5 = ((modulus_6x64[4] >> 54) | (modulus_6x64[5] << 10)) & M62;
    let m6 = modulus_6x64[5] >> 52;
    ModInfo {
        modulus: [
            m0 as i64, m1 as i64, m2 as i64, m3 as i64, m4 as i64, m5 as i64, m6 as i64,
        ],
        modulus_inv_62: modinv_62(m0),
        outer_iters,
    }
}
