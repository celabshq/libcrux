use super::*;

#[cfg(hax)]
use crate::proof_utils::{lemma_mul_succ_le, valid_rate};

#[cfg(hax)]
use hax_lib::int::*;

#[cfg(hax)]
use hax_lib::prop::*;

// Workaround for hax#1698: `fstar::before`/`after` on impl blocks is silently
// dropped by the extractor, and `fstar::options` on methods inside an inherent
// impl triggers a macro-expansion error.  Attach push-options to a hax-only
// dummy function, which does extract properly.  No `#pop-options` is needed
// since this impl is the last item before the free-function items and each
// free function below sets its own options.
#[cfg(hax)]
#[hax_lib::fstar::before(r#"#push-options "--z3rlimit 800""#)]
fn _keccak_state_impl_opts() {}

#[hax_lib::attributes]
impl KeccakState<1, u64> {
    #[inline(always)]
    #[hax_lib::requires(
        valid_rate(RATE) &&
        start.to_int() + RATE.to_int() <= out.len().to_int()
    )]
    #[hax_lib::ensures(|_| future(out).len() == out.len())]
    pub(crate) fn squeeze_next_block<const RATE: usize>(&mut self, out: &mut [u8], start: usize) {
        self.keccakf1600();
        self.squeeze::<RATE>(out, start, RATE);
    }

    #[inline(always)]
    #[hax_lib::requires(
        valid_rate(RATE) &&
        RATE <= out.len()
    )]
    #[hax_lib::ensures(|_| future(out).len() == out.len())]
    pub(crate) fn squeeze_first_block<const RATE: usize>(&self, out: &mut [u8]) {
        self.squeeze::<RATE>(out, 0, RATE);
    }

    #[inline(always)]
    #[hax_lib::requires(
        valid_rate(RATE) &&
        3 * RATE <= out.len()
    )]
    #[hax_lib::ensures(|_| future(out).len() == out.len())]
    pub(crate) fn squeeze_first_three_blocks<const RATE: usize>(&mut self, out: &mut [u8]) {
        self.squeeze::<RATE>(out, 0, RATE);

        self.keccakf1600();
        self.squeeze::<RATE>(out, RATE, RATE);

        self.keccakf1600();
        self.squeeze::<RATE>(out, 2 * RATE, RATE);
    }

    /// Final partial-block step of the squeeze phase: if `output_rem != 0`,
    /// apply one Keccak-f permutation and then extract the trailing
    /// `output_rem` bytes of output into the tail of `out`; otherwise
    /// a no-op.  Factored out of `squeeze` so the final post-condition
    /// reconciling impl vs spec is proved within a small dedicated VC.
    #[inline(always)]
    #[hax_lib::requires(
        valid_rate(RATE) &&
        out.len() < usize::MAX - 200 &&
        output_rem < RATE &&
        output_rem <= out.len()
    )]
    #[hax_lib::ensures(|_| (future(out).len() == out.len()).to_prop() & {
        fstar!(r#"
            let out_len = Core_models.Slice.impl__len #u8 $out in
            let prefix_len : usize = out_len -! $output_rem in
            (($output_rem =. mk_usize 0) ==>
                self_e_future.Libcrux_sha3.Generic_keccak.f_st ==
                    $self_.st) /\
            (($output_rem <>. mk_usize 0) ==>
                self_e_future.Libcrux_sha3.Generic_keccak.f_st ==
                    Hacspec_sha3.Keccak_f.keccak_f $self_.st) /\
            (forall (k: nat). k < v prefix_len ==>
                Seq.index (out_future <: Seq.seq u8) k ==
                Seq.index ($out <: Seq.seq u8) k) /\
            (forall (k: nat). v prefix_len <= k /\ k < v out_len ==>
                Seq.index (out_future <: Seq.seq u8) k ==
                ((Core_models.Num.impl_u64__to_le_bytes
                    (self_e_future.Libcrux_sha3.Generic_keccak.f_st.[
                       (mk_usize (k - v prefix_len) /! mk_usize 8) <: usize ]
                     <: u64)
                  <: t_Array u8 (mk_usize 8)).[
                    (mk_usize (k - v prefix_len) %! mk_usize 8) <: usize ] <: u8))
        "#)
    })]
    pub(crate) fn squeeze_last<const RATE: usize>(&mut self, out: &mut [u8], output_rem: usize) {
        #[cfg(hax)]
        let out_original = out.to_vec();
        #[cfg(hax)]
        let self_original_st = self.st;
        if output_rem != 0 {
            hax_lib::fstar!(r#"EquivImplSpec.Keccakf.Portable.lemma_keccakf1600_portable $self"#);
            self.keccakf1600();
            self.squeeze::<RATE>(out, out.len() - output_rem, output_rem);
            hax_lib::fstar!(
                r#"let out_len = Core_models.Slice.impl__len #u8 $out in
                   let offset = out_len -! $output_rem in
                   let out_orig_slice : t_Slice u8 =
                     Alloc.Vec.impl_1__as_slice $out_original in
                   let new_state = Hacspec_sha3.Keccak_f.keccak_f $self_original_st in
                   assert (v $RATE <= 200);
                   assert ($self.st == new_state);
                   (* Prefix preservation: f_squeeze leaves out[k] unchanged
                      for k < offset, so out[k] == input out at k. *)
                   let aux_prefix (k: nat{k < v offset})
                     : Lemma (Seq.index ($out <: Seq.seq u8) k ==
                              Seq.index (out_orig_slice <: Seq.seq u8) k) =
                     let i : usize = mk_usize k in
                     assert (v i < v offset)
                   in
                   FStar.Classical.forall_intro aux_prefix;
                   (* Trailing bytes: f_squeeze writes the lane-formula byte
                      from new_state == $self.st at indices in [offset, out_len). *)
                   let aux_tail (k: nat{v offset <= k /\ k < v out_len})
                     : Lemma (let j : usize = mk_usize (k - v offset) in
                              Seq.index ($out <: Seq.seq u8) k ==
                              ((Core_models.Num.impl_u64__to_le_bytes
                                  (new_state.[ j /! mk_usize 8 <: usize ] <: u64)
                                <: t_Array u8 (mk_usize 8)).[ j %! mk_usize 8 <: usize ]
                               <: u8)) =
                     let i : usize = mk_usize k in
                     let j : usize = mk_usize (k - v offset) in
                     assert (v i - v offset < v $output_rem);
                     assert ((v i - v offset) / 8 < 25);
                     assert (v j == v i - v offset)
                   in
                   FStar.Classical.forall_intro aux_tail"#
            );
        }
    }

    #[inline(always)]
    #[hax_lib::requires(
        valid_rate(RATE) &&
        5 * RATE <= out.len()
    )]
    #[hax_lib::ensures(|_| future(out).len() == out.len())]
    pub(crate) fn squeeze_first_five_blocks<const RATE: usize>(&mut self, out: &mut [u8]) {
        self.squeeze::<RATE>(out, 0, RATE);

        self.keccakf1600();
        self.squeeze::<RATE>(out, RATE, RATE);

        self.keccakf1600();
        self.squeeze::<RATE>(out, 2 * RATE, RATE);

        self.keccakf1600();
        self.squeeze::<RATE>(out, 3 * RATE, RATE);

        self.keccakf1600();
        self.squeeze::<RATE>(out, 4 * RATE, RATE);
    }
}

/// Absorb phase of `keccak1`: initialise a Keccak state, absorb all full
/// rate-byte blocks of `input`, then pad and absorb the final partial block
/// with domain-separation byte `DELIM` and the pad10*1 terminator.
///
/// The ensures clause asserts direct equality with the spec function
/// `Hacspec_sha3.Sponge.absorb`. The loop invariant uses the spec helper
/// `absorb_blocks` (block-indexed analogue of `absorb_rec`, avoiding the
/// slice-of-slice reasoning that triggers a Z3 4.13.3 LP-solver bug in
/// older proofs based on `absorb_rec` recursion).
#[hax_lib::requires(valid_rate(RATE))]
#[hax_lib::ensures(|result| fstar!(r#"
    $result.st ==
      Hacspec_sha3.Sponge.absorb $RATE $DELIM $input
"#))]
#[hax_lib::fstar::options("--fuel 1 --ifuel 1 --z3rlimit 800 --split_queries always")]
#[inline]
pub(crate) fn absorb<const RATE: usize, const DELIM: u8>(input: &[u8]) -> KeccakState<1, u64> {
    let mut s = KeccakState::<1, u64>::new();
    let input_len = input.len();
    let input_blocks = input_len / RATE;
    let input_rem = input_len % RATE;
    hax_lib::fstar!(
        r#"let zeros : t_Array u64 (mk_usize 25) =
               Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25) in
           Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_base
               zeros $RATE (mk_usize 0) $input"#
    );
    for i in 0..input_blocks {
        hax_lib::loop_invariant!(|i: usize| {
            fstar!(
                r#"let zeros : t_Array u64 (mk_usize 25) =
                       Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25) in
                   v $i <= v $input_blocks /\
                   $s.st ==
                     Hacspec_sha3.Sponge.absorb_blocks
                       zeros $RATE (mk_usize 0) $i $input"#
            )
        });
        #[cfg(hax)]
        lemma_mul_succ_le(i, input_blocks, RATE);

        hax_lib::fstar!(
            r#"let zeros : t_Array u64 (mk_usize 25) =
                   Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25) in
               let inputs : t_Array (t_Slice u8) (mk_usize 1) =
                   let list = [ $input ] in
                   FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                   Rust_primitives.Hax.array_of_list 1 list in
               assert (inputs.[ mk_usize 0 ] == $input);
               EquivImplSpec.Sponge.Portable.Steps.lemma_absorb_block_portable
                   $RATE $s inputs ($i *! $RATE);
               Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_tail
                   zeros $RATE (mk_usize 0) $i ($i +! mk_usize 1) $input"#
        );

        s.absorb_block::<RATE>(&[input], i * RATE);
    }
    hax_lib::fstar!(
        r#"let zeros : t_Array u64 (mk_usize 25) =
               Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25) in
           let inputs : t_Array (t_Slice u8) (mk_usize 1) =
               let list = [ $input ] in
               FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
               Rust_primitives.Hax.array_of_list 1 list in
           assert (inputs.[ mk_usize 0 ] == $input);
           EquivImplSpec.Sponge.Portable.Steps.lemma_absorb_last_portable
               $RATE $DELIM $s inputs ($input_len -! $input_rem) $input_rem;
           Hacspec_sha3.Sponge.Lemmas.lemma_absorb_rec_via_blocks
               zeros $RATE $DELIM $input"#
    );
    s.absorb_final::<RATE, DELIM>(&[input], input_len - input_rem, input_rem);
    s
}


#[hax_lib::requires(
    valid_rate(RATE) &&
    output_blocks > 0 &&
    output_blocks == output.len() / RATE &&
    output.len() < usize::MAX - 200
)]
#[hax_lib::ensures(|_| (future(output).len() == output.len()).to_prop() & {
    fstar!(r#"
    let spec_out : t_Slice u8 =
                       Hacspec_sha3.Sponge.squeeze (${output.len()}) ${s.st} $RATE in
    s_future.f_st == Hacspec_sha3.Sponge.iterate_keccak_f (output_blocks -! sz 1) s.f_st /\
    (forall (k: nat). 
        (k < v output_blocks * v v_RATE) ==>
            Seq.index output_future k == Seq.index spec_out k)        
    "#)})]
#[hax_lib::fstar::options("--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always")]
fn squeeze_blocks<const RATE: usize>(s: &mut KeccakState<1, u64>, output: &mut [u8], output_blocks: usize) {
    #[cfg(hax)]
    let output_len = output.len();
    #[cfg(hax)]
    let s_init_st = s.st;
    #[cfg(hax)]
    let output_initial = output.to_vec().as_slice();

    s.squeeze::<RATE>(output, 0, RATE);

    hax_lib::fstar!(
            r#"let spec_out : t_Slice u8 =
                   Hacspec_sha3.Sponge.squeeze $output_len $s_init_st $RATE in
               assert (v $output_blocks >= 1);
               assert (v $RATE <= 200);
               assert (s_init_st == Hacspec_sha3.Sponge.iterate_keccak_f (mk_usize 0) s_init_st);
               let aux (k: nat{k < v $RATE })
                 : Lemma (Seq.index ($output <: Seq.seq u8) k ==
                          Seq.index (spec_out <: Seq.seq u8) k)
                 = let i : usize = mk_usize k in
                   assert (v i == k);
                   assert (v i / 8 < 25);
                   FStar.Math.Lemmas.small_div k (v $RATE);
                   assert (v i / v $RATE = 0)
               in
               FStar.Classical.forall_intro aux"#
        );
        
    for i in 1..output_blocks {
        hax_lib::loop_invariant!(|i: usize| (output.len() == output_len).to_prop() & {
            fstar!(
                r#"let spec_out : t_Slice u8 =
                       Hacspec_sha3.Sponge.squeeze $output_len $s_init_st $RATE in
                   v $i >= 1 /\ v $i <= v $output_blocks /\
                   v $i * v $RATE <= v $output_len /\
                   $s.st ==
                     Hacspec_sha3.Sponge.iterate_keccak_f
                       ($i -! mk_usize 1) $s_init_st /\
                   (forall (k: nat). (k < v $i * v $RATE) ==>
                      Seq.index ($output <: Seq.seq u8) k ==
                      Seq.index (spec_out <: Seq.seq u8) k)"#
            )
        });

        #[cfg(hax)]
        lemma_mul_succ_le(i, output_blocks, RATE);

        hax_lib::fstar!(
            r#"Libcrux_sha3.Proof_utils.Lemmas.lemma_div_mul_mod $output_len $RATE;
               assert (v $i * v $RATE + v $RATE <= v $output_len);
               assert (v $output_blocks == v $output_len / v $RATE);
               assert (v $RATE >= 1);
               assert (v $i * v $RATE <= v $output_len);
               Math.Lemmas.nat_times_nat_is_nat (v i) (v $RATE);
               assert (v $output_len < v Core_models.Num.impl_usize__MAX);
               assert (v $i * v $RATE < v Core_models.Num.impl_usize__MAX);
               EquivImplSpec.Sponge.Portable.Steps.lemma_squeeze_one_step_portable
                   $RATE $s_init_st $s $output output_initial $i"#
        );

        s.keccakf1600();

        hax_lib::fstar!(r#"
            assert (s.f_st == Hacspec_sha3.Sponge.iterate_keccak_f i s_init_st)
        "#);

        s.squeeze::<RATE>(output, i * RATE, RATE);

        hax_lib::fstar!(r#"
            assert (v i * v v_RATE + v v_RATE <= v output_len);
            Math.Lemmas.distributivity_add_left (v i) 1 (v v_RATE)
        "#);
    }
}

/// Squeeze phase of `keccak1`: extract `output.len()` bytes from `s`,
/// applying Keccak-f between each full rate-byte block of output.
///
/// The ensures clause pins the result to the byteform spec
/// `Hacspec_sha3.Sponge.squeeze` (per-byte: byte k uses
/// `iterate_keccak_f(k/RATE, s_init)`'s lane).  Body proof:
///   - establish the byteform invariant after the first block
///   - cite `lemma_squeeze_one_step_portable` per loop iteration to
///     advance the invariant from i to i+1
///   - reconcile the trailing partial block (via `squeeze_last`) with
///     the byteform's last block.
/// 
#[hax_lib::requires(
    valid_rate(RATE) &&
    output.len() < usize::MAX - 200
)]
#[hax_lib::ensures(|_| (future(output).len() == output.len()).to_prop() & {
    fstar!(r#"(output_future <: t_Slice u8) ==
              (Hacspec_sha3.Sponge.squeeze
                 (Core_models.Slice.impl__len #u8 $output)
                 $s.st
                 $RATE <: t_Slice u8)"#)
})]
#[hax_lib::fstar::options("--fuel 1 --ifuel 1 --z3rlimit 400")]
#[inline]
pub(crate) fn squeeze<const RATE: usize>(mut s: KeccakState<1, u64>, output: &mut [u8]) {
    let output_len = output.len();
    let output_blocks = output_len / RATE;
    let output_rem = output_len % RATE;

    #[cfg(hax)]
    let s_init_st = s.st;

    if output_blocks == 0 {
        s.squeeze::<RATE>(output, 0, output_len);
        hax_lib::fstar!(
            r#"let spec_out : t_Slice u8 =
                   Hacspec_sha3.Sponge.squeeze $output_len $s_init_st $RATE in
               assert (v $output_len < v $RATE);
               assert (v $RATE <= 200);
               let aux (k: nat{k < v $output_len })
                 : Lemma (Seq.index ($output <: Seq.seq u8) k ==
                          Seq.index (spec_out <: Seq.seq u8) k)
                 = let i : usize = mk_usize k in
                   assert (v i == k);
                   assert (v i / 8 < 25);
                   FStar.Math.Lemmas.small_div k (v $RATE);
                   assert (v i / v $RATE = 0)
               in
               FStar.Classical.forall_intro aux;
               Seq.lemma_eq_intro ($output <: Seq.seq u8) (spec_out <: Seq.seq u8)"#
        );
    } else {
        // Capture output state after squeeze_blocks for the prefix-preservation
        // lemma below.  squeeze_blocks's ensures pin output to the byteform
        // spec on full blocks; squeeze_last preserves the prefix and writes
        // the trailing partial block via the byteform-shape ensures.
        squeeze_blocks::<RATE>(&mut s, output, output_blocks);
        #[cfg(hax)]
        let output_after_blocks = output.to_vec();
        s.squeeze_last::<RATE>(output, output_rem);
        hax_lib::fstar!(r#"
            Math.Lemmas.lemma_div_mod (v $output_len) (v $RATE);
            let spec_out : t_Slice u8 =
                Hacspec_sha3.Sponge.squeeze $output_len $s_init_st $RATE in
            assert (v $output_blocks >= 1);
            (* iterate_keccak_f output_blocks s_init unfolds to keccak_f-of-prev. *)
            assert (Hacspec_sha3.Sponge.iterate_keccak_f $output_blocks $s_init_st ==
                    Hacspec_sha3.Keccak_f.keccak_f
                      (Hacspec_sha3.Sponge.iterate_keccak_f
                         ($output_blocks -! mk_usize 1) $s_init_st));
            let output_after_blocks_slice : t_Slice u8 =
                Alloc.Vec.impl_1__as_slice $output_after_blocks in
            let aux (k: nat{k < v $output_len })
                : Lemma (Seq.index ($output <: Seq.seq u8) k ==
                         Seq.index (spec_out <: Seq.seq u8) k) =
              if k < v $output_len - v $output_rem
              then
                EquivImplSpec.Sponge.Portable.Steps.lemma_squeeze_prefix_preserved_portable
                  $RATE $s_init_st output_after_blocks_slice $output
                  $output_blocks $output_rem k
              else begin
                assert (v $output_rem > 0);
                assert ($s.st ==
                        Hacspec_sha3.Sponge.iterate_keccak_f $output_blocks $s_init_st);
                EquivImplSpec.Sponge.Portable.Steps.lemma_squeeze_trailing_byteform_portable
                  $RATE $s_init_st $s.st $output $output_blocks $output_rem k
              end
            in
            FStar.Classical.forall_intro aux;
            Seq.lemma_eq_intro ($output <: Seq.seq u8) (spec_out <: Seq.seq u8)
        "#);
    }
}

#[hax_lib::requires(
    valid_rate(RATE) &&
    output.len() < usize::MAX - 200
)]
#[hax_lib::ensures(|_| (future(output).len() == output.len()).to_prop() & {
    fstar!(r#"(output_future <: t_Slice u8) ==
              (Hacspec_sha3.Sponge.keccak
                 (Core_models.Slice.impl__len #u8 $output)
                 $RATE $DELIM $input <: t_Slice u8)"#)
})]
#[hax_lib::fstar::options("--fuel 1 --ifuel 1 --z3rlimit 200")]
#[inline]
pub(crate) fn keccak1<const RATE: usize, const DELIM: u8>(input: &[u8], output: &mut [u8]) {
    let s = absorb::<RATE, DELIM>(input);
    squeeze::<RATE>(s, output);
}
