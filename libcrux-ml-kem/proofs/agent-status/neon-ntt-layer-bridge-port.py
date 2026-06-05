p = "/Users/karthik/libcrux-ml-kem-proofs/libcrux-ml-kem/src/vector/neon.rs"
s = open(p).read()

REPR = "Libcrux_ml_kem.Vector.Neon.Vector_type.repr"

def stub(reqpost, sig, call):
    return ('#[inline(always)]\n'
            '#[hax_lib::fstar::verification_status(panic_free)]\n'
            + reqpost +
            'fn ' + sig + ' {\n'
            '    hax_lib::fstar!(r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)"#);\n'
            '    ' + call + '\n}')

# ---------- layer 1 (branch lemmas) ----------
old1 = stub(
 '#[hax_lib::requires(fstar!(r#"${spec::ntt_layer_1_step_pre} (impl.f_repr ${vector}) zeta0 zeta1 zeta2 zeta3"#))]\n'
 '#[hax_lib::ensures(|out| fstar!(r#"${spec::ntt_layer_1_step_post} (impl.f_repr ${vector}) zeta0 zeta1 zeta2 zeta3 (impl.f_repr ${out})"#))]\n',
 'op_ntt_layer_1_step(vector: SIMD128Vector, zeta0: i16, zeta1: i16, zeta2: i16, zeta3: i16) -> SIMD128Vector',
 'ntt_layer_1_step(vector, zeta0, zeta1, zeta2, zeta3)')
new1 = f'''#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --fuel 0 --ifuel 1 --split_queries always")]
#[hax_lib::requires(fstar!(r#"${{spec::ntt_layer_1_step_pre}} (impl.f_repr ${{vector}}) zeta0 zeta1 zeta2 zeta3"#))]
#[hax_lib::ensures(|out| fstar!(r#"${{spec::ntt_layer_1_step_post}} (impl.f_repr ${{vector}}) zeta0 zeta1 zeta2 zeta3 (impl.f_repr ${{out}})"#))]
fn op_ntt_layer_1_step(vector: SIMD128Vector, zeta0: i16, zeta1: i16, zeta2: i16, zeta3: i16) -> SIMD128Vector {{
    hax_lib::fstar!(r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (7*3328))"#);
    let result = ntt_layer_1_step(vector, zeta0, zeta1, zeta2, zeta3);
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (8*3328));
           let vec = {REPR} ${{vector}} in
           let out = {REPR} ${{result}} in
           reveal_opaque (`%Spec.Utils.ntt_layer_1_butterfly_post) (Spec.Utils.ntt_layer_1_butterfly_post vec);
           Hacspec_ml_kem.Commute.Chunk.lemma_ntt_layer_1_step_branch_0 vec out zeta0 zeta1 zeta2 zeta3;
           Hacspec_ml_kem.Commute.Chunk.lemma_ntt_layer_1_step_branch_1 vec out zeta0 zeta1 zeta2 zeta3;
           Hacspec_ml_kem.Commute.Chunk.lemma_ntt_layer_1_step_branch_2 vec out zeta0 zeta1 zeta2 zeta3;
           Hacspec_ml_kem.Commute.Chunk.lemma_ntt_layer_1_step_branch_3 vec out zeta0 zeta1 zeta2 zeta3"#
    );
    result
}}'''

# helper for the FE-form add/sub butterfly conjunct used by layer-2/3 (forward)
def fe_fwd(out_i, vec_i, z, vec_j):
    return (f'''                Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out {out_i}) ==
                  Hacspec_ml_kem.Parameters.impl_FieldElement__add
                    (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec {vec_i}))
                    (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe {z})
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec {vec_j})))''')

# ---------- layer 2 (p_layer_2 FE-form) ----------
old2 = stub(
 '#[hax_lib::requires(fstar!(r#"${spec::ntt_layer_2_step_pre} (impl.f_repr ${vector}) zeta0 zeta1"#))]\n'
 '#[hax_lib::ensures(|out| fstar!(r#"${spec::ntt_layer_2_step_post} (impl.f_repr ${vector}) zeta0 zeta1 (impl.f_repr ${out})"#))]\n',
 'op_ntt_layer_2_step(vector: SIMD128Vector, zeta0: i16, zeta1: i16) -> SIMD128Vector',
 'ntt_layer_2_step(vector, zeta0, zeta1)')
new2 = r'''#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 600 --fuel 1 --ifuel 1 --split_queries always")]
#[hax_lib::requires(fstar!(r#"${spec::ntt_layer_2_step_pre} (impl.f_repr ${vector}) zeta0 zeta1"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::ntt_layer_2_step_post} (impl.f_repr ${vector}) zeta0 zeta1 (impl.f_repr ${out})"#))]
fn op_ntt_layer_2_step(vector: SIMD128Vector, zeta0: i16, zeta1: i16) -> SIMD128Vector {
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (6*3328));
           reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_2_step_branch_post) Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_2_step_branch_post"#
    );
    let result = ntt_layer_2_step(vector, zeta0, zeta1);
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (7*3328));
           let vec = LL ${vector} in
           let out = LL ${result} in
           reveal_opaque (`%Spec.Utils.ntt_layer_2_butterfly_post) (Spec.Utils.ntt_layer_2_butterfly_post vec);
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta0 0 4;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta0 1 5;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta0 2 6;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta0 3 7;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta1 8 12;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta1 9 13;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta1 10 14;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta1 11 15;
           let p_layer_2 : (b: nat{b < 4}) -> Type0 =
             fun (b: nat{b < 4}) ->
               (let z = (if b < 2 then zeta0 else zeta1) in
                let base : nat = if b < 2 then 0 else 8 in
                let off : nat = if b = 0 || b = 2 then 0 else 2 in
                let i1 : nat = base + off in
                let j1 : nat = i1 + 4 in
                let i2 : nat = i1 + 1 in
                let j2 : nat = j1 + 1 in
FWD_I1_ADD /\
FWD_J1_SUB /\
FWD_I2_ADD /\
FWD_J2_SUB) in
           assert (p_layer_2 0);
           assert (p_layer_2 1);
           assert (p_layer_2 2);
           assert (p_layer_2 3);
           assert (Spec.Utils.forall4 p_layer_2)"#
    );
    result
}'''
def fwd_sub(out_i, vec_i, z, vec_j):
    return (f'''                Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out {out_i}) ==
                  Hacspec_ml_kem.Parameters.impl_FieldElement__sub
                    (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec {vec_i}))
                    (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe {z})
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec {vec_j})))''')
new2 = (new2.replace("LL", REPR)
  .replace("FWD_I1_ADD", fe_fwd("i1","i1","z","j1"))
  .replace("FWD_J1_SUB", fwd_sub("j1","i1","z","j1"))
  .replace("FWD_I2_ADD", fe_fwd("i2","i2","z","j2"))
  .replace("FWD_J2_SUB", fwd_sub("j2","i2","z","j2")))

# ---------- layer 3 (p_layer_3 FE-form) ----------
old3 = stub(
 '#[hax_lib::requires(fstar!(r#"${spec::ntt_layer_3_step_pre} (impl.f_repr ${vector}) zeta"#))]\n'
 '#[hax_lib::ensures(|out| fstar!(r#"${spec::ntt_layer_3_step_post} (impl.f_repr ${vector}) zeta (impl.f_repr ${out})"#))]\n',
 'op_ntt_layer_3_step(vector: SIMD128Vector, zeta: i16) -> SIMD128Vector',
 'ntt_layer_3_step(vector, zeta)')
new3 = r'''#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 600 --fuel 1 --ifuel 1 --split_queries always")]
#[hax_lib::requires(fstar!(r#"${spec::ntt_layer_3_step_pre} (impl.f_repr ${vector}) zeta"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::ntt_layer_3_step_post} (impl.f_repr ${vector}) zeta (impl.f_repr ${out})"#))]
fn op_ntt_layer_3_step(vector: SIMD128Vector, zeta: i16) -> SIMD128Vector {
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (5*3328));
           reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_3_step_branch_post) Libcrux_ml_kem.Vector.Traits.Spec.ntt_layer_3_step_branch_post"#
    );
    let result = ntt_layer_3_step(vector, zeta);
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (6*3328));
           let vec = LL ${vector} in
           let out = LL ${result} in
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta 0 8;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta 1 9;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta 2 10;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta 3 11;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta 4 12;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta 5 13;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta 6 14;
           Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute vec out zeta 7 15;
           let p_layer_3 : (b: nat{b < 4}) -> Type0 =
             fun (b: nat{b < 4}) ->
               (let i1 : nat = 2 * b in
                let j1 : nat = 2 * b + 8 in
                let i2 : nat = 2 * b + 1 in
                let j2 : nat = 2 * b + 9 in
FWD_I1_ADD /\
FWD_J1_SUB /\
FWD_I2_ADD /\
FWD_J2_SUB) in
           assert (p_layer_3 0);
           assert (p_layer_3 1);
           assert (p_layer_3 2);
           assert (p_layer_3 3);
           assert (Spec.Utils.forall4 p_layer_3)"#
    );
    result
}'''
new3 = (new3.replace("LL", REPR)
  .replace("FWD_I1_ADD", fe_fwd("i1","i1","zeta","j1"))
  .replace("FWD_J1_SUB", fwd_sub("j1","i1","zeta","j1"))
  .replace("FWD_I2_ADD", fe_fwd("i2","i2","zeta","j2"))
  .replace("FWD_J2_SUB", fwd_sub("j2","i2","zeta","j2")))

# ---------- inv layer 1 (branch lemmas) ----------
oldi1 = stub(
 '#[hax_lib::requires(fstar!(r#"${spec::inv_ntt_layer_1_step_pre} (impl.f_repr ${vector}) zeta0 zeta1 zeta2 zeta3"#))]\n'
 '#[hax_lib::ensures(|out| fstar!(r#"${spec::inv_ntt_layer_1_step_post} (impl.f_repr ${vector}) zeta0 zeta1 zeta2 zeta3 (impl.f_repr ${out})"#))]\n',
 'op_inv_ntt_layer_1_step(vector: SIMD128Vector, zeta0: i16, zeta1: i16, zeta2: i16, zeta3: i16) -> SIMD128Vector',
 'inv_ntt_layer_1_step(vector, zeta0, zeta1, zeta2, zeta3)')
newi1 = f'''#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --fuel 0 --ifuel 1 --split_queries always")]
#[hax_lib::requires(fstar!(r#"${{spec::inv_ntt_layer_1_step_pre}} (impl.f_repr ${{vector}}) zeta0 zeta1 zeta2 zeta3"#))]
#[hax_lib::ensures(|out| fstar!(r#"${{spec::inv_ntt_layer_1_step_post}} (impl.f_repr ${{vector}}) zeta0 zeta1 zeta2 zeta3 (impl.f_repr ${{out}})"#))]
fn op_inv_ntt_layer_1_step(vector: SIMD128Vector, zeta0: i16, zeta1: i16, zeta2: i16, zeta3: i16) -> SIMD128Vector {{
    hax_lib::fstar!(r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (4*3328))"#);
    let result = inv_ntt_layer_1_step(vector, zeta0, zeta1, zeta2, zeta3);
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 3328);
           let vec = {REPR} ${{vector}} in
           let out = {REPR} ${{result}} in
           reveal_opaque (`%Spec.Utils.inv_ntt_layer_1_butterfly_post) (Spec.Utils.inv_ntt_layer_1_butterfly_post vec);
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_ntt_layer_1_step_branch_0 vec out zeta0 zeta1 zeta2 zeta3;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_ntt_layer_1_step_branch_1 vec out zeta0 zeta1 zeta2 zeta3;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_ntt_layer_1_step_branch_2 vec out zeta0 zeta1 zeta2 zeta3;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_ntt_layer_1_step_branch_3 vec out zeta0 zeta1 zeta2 zeta3"#
    );
    result
}}'''

# inverse FE-form: out i1 = vec i1 + vec j1 ; out j1 = z * (vec j1 - vec i1)
def inv_add(out_x, vec_a, vec_b):
    return (f'''                Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out {out_x}) ==
                  Hacspec_ml_kem.Parameters.impl_FieldElement__add
                    (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec {vec_a}))
                    (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec {vec_b}))''')
def inv_mulsub(out_x, z, vec_j, vec_i):
    return (f'''                Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index out {out_x}) ==
                  Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                    (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe {z})
                    (Hacspec_ml_kem.Parameters.impl_FieldElement__sub
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec {vec_j}))
                      (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index vec {vec_i})))''')

# ---------- inv layer 2 (p_inv_2 FE-form) ----------
oldi2 = stub(
 '#[hax_lib::requires(fstar!(r#"${spec::inv_ntt_layer_2_step_pre} (impl.f_repr ${vector}) zeta0 zeta1"#))]\n'
 '#[hax_lib::ensures(|out| fstar!(r#"${spec::inv_ntt_layer_2_step_post} (impl.f_repr ${vector}) zeta0 zeta1 (impl.f_repr ${out})"#))]\n',
 'op_inv_ntt_layer_2_step(vector: SIMD128Vector, zeta0: i16, zeta1: i16) -> SIMD128Vector',
 'inv_ntt_layer_2_step(vector, zeta0, zeta1)')
newi2 = r'''#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 600 --fuel 1 --ifuel 1 --split_queries always")]
#[hax_lib::requires(fstar!(r#"${spec::inv_ntt_layer_2_step_pre} (impl.f_repr ${vector}) zeta0 zeta1"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::inv_ntt_layer_2_step_post} (impl.f_repr ${vector}) zeta0 zeta1 (impl.f_repr ${out})"#))]
fn op_inv_ntt_layer_2_step(vector: SIMD128Vector, zeta0: i16, zeta1: i16) -> SIMD128Vector {
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 3328);
           reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_2_step_branch_post) Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_2_step_branch_post"#
    );
    let result = inv_ntt_layer_2_step(vector, zeta0, zeta1);
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (2*3328));
           let vec = LL ${vector} in
           let out = LL ${result} in
           reveal_opaque (`%Spec.Utils.inv_ntt_layer_2_butterfly_post) (Spec.Utils.inv_ntt_layer_2_butterfly_post vec);
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta0 0 4;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta0 1 5;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta0 2 6;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta0 3 7;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta1 8 12;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta1 9 13;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta1 10 14;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta1 11 15;
           let p_inv_2 : (b: nat{b < 4}) -> Type0 =
             fun (b: nat{b < 4}) ->
               (let z = (if b < 2 then zeta0 else zeta1) in
                let base : nat = if b < 2 then 0 else 8 in
                let off : nat = if b = 0 || b = 2 then 0 else 2 in
                let i1 : nat = base + off in
                let j1 : nat = i1 + 4 in
                let i2 : nat = i1 + 1 in
                let j2 : nat = j1 + 1 in
INV_I1 /\
INV_J1 /\
INV_I2 /\
INV_J2) in
           assert (p_inv_2 0);
           assert (p_inv_2 1);
           assert (p_inv_2 2);
           assert (p_inv_2 3);
           assert (Spec.Utils.forall4 p_inv_2)"#
    );
    result
}'''
newi2 = (newi2.replace("LL", REPR)
  .replace("INV_I1", inv_add("i1","i1","j1"))
  .replace("INV_J1", inv_mulsub("j1","z","j1","i1"))
  .replace("INV_I2", inv_add("i2","i2","j2"))
  .replace("INV_J2", inv_mulsub("j2","z","j2","i2")))

# ---------- inv layer 3 (p_inv_layer_3 FE-form) ----------
oldi3 = stub(
 '#[hax_lib::requires(fstar!(r#"${spec::inv_ntt_layer_3_step_pre} (impl.f_repr ${vector}) zeta"#))]\n'
 '#[hax_lib::ensures(|out| fstar!(r#"${spec::inv_ntt_layer_3_step_post} (impl.f_repr ${vector}) zeta (impl.f_repr ${out})"#))]\n',
 'op_inv_ntt_layer_3_step(vector: SIMD128Vector, zeta: i16) -> SIMD128Vector',
 'inv_ntt_layer_3_step(vector, zeta)')
newi3 = r'''#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 600 --fuel 1 --ifuel 1 --split_queries always")]
#[hax_lib::requires(fstar!(r#"${spec::inv_ntt_layer_3_step_pre} (impl.f_repr ${vector}) zeta"#))]
#[hax_lib::ensures(|out| fstar!(r#"${spec::inv_ntt_layer_3_step_post} (impl.f_repr ${vector}) zeta (impl.f_repr ${out})"#))]
fn op_inv_ntt_layer_3_step(vector: SIMD128Vector, zeta: i16) -> SIMD128Vector {
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (2*3328));
           reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_3_step_branch_post) Libcrux_ml_kem.Vector.Traits.Spec.inv_ntt_layer_3_step_branch_post"#
    );
    let result = inv_ntt_layer_3_step(vector, zeta);
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (4*3328));
           let vec = LL ${vector} in
           let out = LL ${result} in
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta 0 8;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta 1 9;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta 2 10;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta 3 11;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta 4 12;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta 5 13;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta 6 14;
           Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_pair_commute vec out zeta 7 15;
           let p_inv_layer_3 : (b: nat{b < 4}) -> Type0 =
             fun (b: nat{b < 4}) ->
               (let i1 : nat = 2 * b in
                let j1 : nat = 2 * b + 8 in
                let i2 : nat = 2 * b + 1 in
                let j2 : nat = 2 * b + 9 in
INV_I1 /\
INV_J1 /\
INV_I2 /\
INV_J2) in
           assert (p_inv_layer_3 0);
           assert (p_inv_layer_3 1);
           assert (p_inv_layer_3 2);
           assert (p_inv_layer_3 3);
           assert (Spec.Utils.forall4 p_inv_layer_3)"#
    );
    result
}'''
newi3 = (newi3.replace("LL", REPR)
  .replace("INV_I1", inv_add("i1","i1","j1"))
  .replace("INV_J1", inv_mulsub("j1","zeta","j1","i1"))
  .replace("INV_I2", inv_add("i2","i2","j2"))
  .replace("INV_J2", inv_mulsub("j2","zeta","j2","i2")))

for (old,new,name) in [(old1,new1,"l1"),(old2,new2,"l2"),(old3,new3,"l3"),
                       (oldi1,newi1,"i1"),(oldi2,newi2,"i2"),(oldi3,newi3,"i3")]:
    assert s.count(old)==1, f"stub {name} not found uniquely ({s.count(old)})"
    s = s.replace(old,new,1)

open(p,"w").write(s)
print("Phase B NTT-layer upgrades spliced OK")
