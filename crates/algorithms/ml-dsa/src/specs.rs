use hax_lib::*;

pub mod simd {
    use super::*;
    pub mod portable {
        use super::*;
        pub mod sample {
            use super::*;
            pub fn rejection_sample_less_than_field_modulus_pre(
                randomness: &[u8],
                out: &[i32],
            ) -> Prop {
                (randomness.len() / 3 <= 4_294_967_295 && randomness.len() / 3 <= out.len()).into()
            }

            #[fstar::replace_body(
                r#"let s = Spec.MLDSA.Math.rejection_sample_field_modulus $randomness in
                v $r <= Seq.length out /\ v $r == Seq.length s /\
                Seq.slice out 0 (v $r) == s"#
            )]
            pub fn rejection_sample_less_than_field_modulus_post(
                randomness: &[u8],
                out: &[i32],
                r: usize,
            ) -> Prop {
                true.into()
            }

            pub fn rejection_sample_less_than_eta_equals_2_pre(
                randomness: &[u8],
                out: &[i32],
            ) -> Prop {
                (randomness.len() * 2 <= 4_294_967_295 && randomness.len() * 2 <= out.len()).into()
            }

            #[fstar::replace_body(
                r#"let s = Spec.MLDSA.Math.rejection_sample_eta_2 $randomness in
                v $r <= Seq.length out /\ v $r == Seq.length s /\
                Seq.slice out 0 (v $r) == s"#
            )]
            pub fn rejection_sample_less_than_eta_equals_2_post(
                randomness: &[u8],
                out: &[i32],
                r: usize,
            ) -> Prop {
                true.into()
            }

            pub fn rejection_sample_less_than_eta_equals_4_pre(
                randomness: &[u8],
                out: &[i32],
            ) -> Prop {
                (randomness.len() * 2 <= 4_294_967_295 && randomness.len() * 2 <= out.len()).into()
            }

            #[fstar::replace_body(
                r#"let s = Spec.MLDSA.Math.rejection_sample_eta_4 $randomness in
                v $r <= Seq.length out /\ v $r == Seq.length s /\
                Seq.slice out 0 (v $r) == s"#
            )]
            pub fn rejection_sample_less_than_eta_equals_4_post(
                randomness: &[u8],
                out: &[i32],
                r: usize,
            ) -> Prop {
                true.into()
            }
        }
    }
}
