#!/usr/bin/env python3
# Backport the validated candidate-(b) helper proofs from the extracted .fst
# into neon.rs:  add a `before` block with the 4 helper lemmas, and replace the
# 4 FE-form bridge bodies (layer_2/3, inv_2/3) with trivial helper calls.
# Primitives (Neon.Ntt ntt/inv_ntt_layer_3_step -> butterfly_post) are already
# backported directly in neon/ntt.rs.

fstp  = "proofs/fstar/extraction/Libcrux_ml_kem.Vector.Neon.fst"
neonp = "src/vector/neon.rs"
fst   = open(fstp).read()
neon  = open(neonp).readlines()  # 1-indexed via [i-1]

# ---- extract a helper block (its #push-options .. let lemma .. #pop-options) ----
def helper(name):
    li = fst.index("let " + name + " ")
    ps = fst.rfind("#push-options", 0, li)
    pe = fst.index("\n#pop-options", li) + len("\n#pop-options")
    return fst[ps:pe]

helpers = "\n\n".join(helper(n) for n in [
    "lemma_neon_ntt_layer_2_post",
    "lemma_neon_ntt_layer_3_post",
    "lemma_neon_inv_ntt_layer_2_post",
    "lemma_neon_inv_ntt_layer_3_post",
])
before_block = '#[hax_lib::fstar::before(r#"' + helpers + '"#)]\n'

OPTS = ('#[hax_lib::fstar::options("--z3rlimit 200 --fuel 0 --ifuel 1 '
        '--split_queries always --using_facts_from '
        "'* -Libcrux_ml_kem.Vector.Neon.Vector_type.lemma_repr_index'\")]\n")

REPR = "Libcrux_ml_kem.Vector.Neon.Vector_type.repr"

def bridge(spec, sig, prim_call, pre_b, post_b, helper_name, zetas):
    return (
        '#[inline(always)]\n' + OPTS +
        f'#[hax_lib::requires(fstar!(r#"${{spec::{spec}_pre}} (impl.f_repr ${{vector}}) {zetas}"#))]\n'
        f'#[hax_lib::ensures(|out| fstar!(r#"${{spec::{spec}_post}} (impl.f_repr ${{vector}}) {zetas} (impl.f_repr ${{out}})"#))]\n'
        f'fn {sig} {{\n'
        f'    hax_lib::fstar!(r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque ({pre_b}))"#);\n'
        f'    let result = {prim_call};\n'
        f'    hax_lib::fstar!(\n'
        f'        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque ({post_b}));\n'
        f'           {helper_name} ({REPR} ${{vector}}) ({REPR} ${{result}}) {zetas}"#\n'
        f'    );\n'
        f'    result\n'
        f'}}')

b_l2 = bridge("ntt_layer_2_step",
    "op_ntt_layer_2_step(vector: SIMD128Vector, zeta0: i16, zeta1: i16) -> SIMD128Vector",
    "ntt_layer_2_step(vector, zeta0, zeta1)", "6*3328", "7*3328",
    "lemma_neon_ntt_layer_2_post", "zeta0 zeta1")
b_l3 = bridge("ntt_layer_3_step",
    "op_ntt_layer_3_step(vector: SIMD128Vector, zeta: i16) -> SIMD128Vector",
    "ntt_layer_3_step(vector, zeta)", "5*3328", "6*3328",
    "lemma_neon_ntt_layer_3_post", "zeta")
b_i2 = bridge("inv_ntt_layer_2_step",
    "op_inv_ntt_layer_2_step(vector: SIMD128Vector, zeta0: i16, zeta1: i16) -> SIMD128Vector",
    "inv_ntt_layer_2_step(vector, zeta0, zeta1)", "3328", "2*3328",
    "lemma_neon_inv_ntt_layer_2_post", "zeta0 zeta1")
b_i3 = bridge("inv_ntt_layer_3_step",
    "op_inv_ntt_layer_3_step(vector: SIMD128Vector, zeta: i16) -> SIMD128Vector",
    "inv_ntt_layer_3_step(vector, zeta)", "2*3328", "4*3328",
    "lemma_neon_inv_ntt_layer_3_post", "zeta")

# ---- line-range splice (ranges discovered from neon.rs; 1-indexed inclusive) ----
# layer_2: 161-225 ; layer_3: 226-286 ; inv_1: 287-306 (keep) ; inv_2: 307-367 ; inv_3: 368-427
# sanity-check the boundaries
assert neon[160].strip().startswith("#[inline"), neon[160]
assert "fn op_ntt_layer_2_step" in neon[164], neon[164]
assert "fn op_ntt_layer_3_step" in neon[229], neon[229]
assert "fn op_inv_ntt_layer_1_step" in neon[290], neon[290]
assert "fn op_inv_ntt_layer_2_step" in neon[310], neon[310]
assert "fn op_inv_ntt_layer_3_step" in neon[371], neon[371]
assert "fn op_ntt_multiply" in neon[428], neon[428]

new = (
    "".join(neon[:160])                                   # 1..160 (incl layer_1)
    + before_block + "\n"                                 # helpers before layer_2
    + b_l2 + "\n\n"
    + b_l3 + "\n\n"
    + "".join(neon[286:306])                              # 287..306 inv_1 (keep)
    + b_i2 + "\n\n"
    + b_i3 + "\n\n"
    + "".join(neon[427:])                                 # 428.. ntt_multiply onward
)
open(neonp, "w").write(new)
print("Backport spliced OK (4 helpers + 4 clean bridges)")
