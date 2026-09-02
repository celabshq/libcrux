use hax_lib::*;

pub(crate) const PRIME: u32 = 8380417;

pub(crate) const MONT_R: u32 = 8380417;

pub(crate) const FIELD_MAX: u32 = 8380416;

pub(crate) const FIELD_MID: u32 = 4190208;

pub(crate) const NTT_BASE_BOUND: u32 = FIELD_MID;

const COEFFICIENTS_IN_SIMD_UNIT: usize = 8;

type SIMDContent = [i32; COEFFICIENTS_IN_SIMD_UNIT];

pub(crate) fn int_is_i32(i: Int) -> bool {
    i <= i32::MAX.to_int() && i >= i32::MIN.to_int()
}

#[cfg_attr(hax, hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#))]
#[cfg_attr(hax, hax_lib::fstar::after(r#"
    let bounded_add_pre (a b: t_Array i32 (sz 8)) (b1:nat) (b2:nat):
        Lemma (requires (Spec.Utils.is_i32b_array_opaque b1 a /\ Spec.Utils.is_i32b_array_opaque b2 b /\ b1 + b2 <= 4294967295))
                (ensures (Libcrux_ml_dsa.Simd.Traits.Specs.add_pre a b))
               [SMTPat (Libcrux_ml_dsa.Simd.Traits.Specs.add_pre a b); 
                SMTPat (Spec.Utils.is_i32b_array_opaque b1 a);
                SMTPat (Spec.Utils.is_i32b_array_opaque b2 b)] = 
        reveal_opaque (`%$add_pre) ($add_pre)
    "#))]
pub(crate) fn add_pre(lhs: &SIMDContent, rhs: &SIMDContent) -> Prop {
    forall(|i: usize| {
        implies(
            i < COEFFICIENTS_IN_SIMD_UNIT,
            int_is_i32(lhs[i].to_int() + rhs[i].to_int()),
        )
    })
}

#[cfg_attr(hax, hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#))]
#[cfg_attr(hax, hax_lib::fstar::after(r#"
    let bounded_add_post (a b a_future: t_Array i32 (sz 8)) (b1 b2 b3:nat):
        Lemma (requires (Spec.Utils.is_i32b_array_opaque b1 a /\ Spec.Utils.is_i32b_array_opaque b2 b /\
                    b1 + b2 <= b3 /\ Libcrux_ml_dsa.Simd.Traits.Specs.add_post a b a_future))
            (ensures (Spec.Utils.is_i32b_array_opaque b3 a_future))
            [SMTPat (Libcrux_ml_dsa.Simd.Traits.Specs.add_post a b a_future); 
            SMTPat (Spec.Utils.is_i32b_array_opaque b1 a);
            SMTPat (Spec.Utils.is_i32b_array_opaque b2 b);
            SMTPat (Spec.Utils.is_i32b_array_opaque b3 a_future)] = 
        reveal_opaque (`%$add_post) ($add_post)
    "#))]
pub(crate) fn add_post(lhs: &SIMDContent, rhs: &SIMDContent, future_lhs: &SIMDContent) -> Prop {
    forall(|i: usize| {
        implies(
            i < COEFFICIENTS_IN_SIMD_UNIT,
            future_lhs[i].to_int() == (lhs[i].to_int() + rhs[i].to_int()),
        )
    })
}

#[cfg_attr(hax, hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#))]
#[cfg_attr(hax, hax_lib::fstar::after(r#"
    let bounded_sub_pre (a b: t_Array i32 (sz 8)) (b1:nat) (b2:nat):
        Lemma (requires (Spec.Utils.is_i32b_array_opaque b1 a /\ Spec.Utils.is_i32b_array_opaque b2 b /\ b1 + b2 <= 4294967295))
              (ensures (Libcrux_ml_dsa.Simd.Traits.Specs.sub_pre a b))
              [SMTPat (Libcrux_ml_dsa.Simd.Traits.Specs.sub_pre a b); 
               SMTPat (Spec.Utils.is_i32b_array_opaque b1 a);
               SMTPat (Spec.Utils.is_i32b_array_opaque b2 b)] = 
        reveal_opaque (`%$sub_pre) ($sub_pre)
    "#))]
pub(crate) fn sub_pre(lhs: &SIMDContent, rhs: &SIMDContent) -> Prop {
    forall(|i: usize| {
        implies(
            i < COEFFICIENTS_IN_SIMD_UNIT,
            int_is_i32(lhs[i].to_int() - rhs[i].to_int()),
        )
    })
}

#[cfg_attr(hax, hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#))]
#[cfg_attr(hax, hax_lib::fstar::after(r#"
    let bounded_sub_post (a b a_future: t_Array i32 (sz 8)) (b1 b2 b3:nat):
        Lemma (requires (Spec.Utils.is_i32b_array_opaque b1 a /\ Spec.Utils.is_i32b_array_opaque b2 b /\
                        b1 + b2 <= b3 /\ Libcrux_ml_dsa.Simd.Traits.Specs.sub_post a b a_future))
                (ensures (Spec.Utils.is_i32b_array_opaque b3 a_future))
                [SMTPat (Libcrux_ml_dsa.Simd.Traits.Specs.sub_post a b a_future); 
                SMTPat (Spec.Utils.is_i32b_array_opaque b1 a);
                SMTPat (Spec.Utils.is_i32b_array_opaque b2 b);
                SMTPat (Spec.Utils.is_i32b_array_opaque b3 a_future)] = 
                reveal_opaque (`%$sub_post) ($sub_post)
    "#))]
pub(crate) fn sub_post(lhs: &SIMDContent, rhs: &SIMDContent, future_lhs: &SIMDContent) -> Prop {
    forall(|i: usize| {
        implies(
            i < COEFFICIENTS_IN_SIMD_UNIT,
            future_lhs[i].to_int() == (lhs[i].to_int() - rhs[i].to_int()),
        )
    })
}
