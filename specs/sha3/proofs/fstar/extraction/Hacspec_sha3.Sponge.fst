module Hacspec_sha3.Sponge
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

/// XOR a block of message bytes into the state (little-endian, lane-interleaved).
/// Corresponds to the `S ⊕ (Pi || 0^c)` step of Algorithm 8.
let xor_block_into_state (state: t_Array u64 (mk_usize 25)) (block: t_Slice u8) (rate: usize)
    : Prims.Pure (t_Array u64 (mk_usize 25))
      (requires
        rate <=. mk_usize 200 && (rate %! mk_usize 8 <: usize) =. mk_usize 0 &&
        (Core_models.Slice.impl__len #u8 block <: usize) >=. rate)
      (fun _ -> Prims.l_True) =
  Hacspec_sha3.createi #u64
    (mk_usize 25)
    #(usize -> u64)
    (fun i ->
        let i:usize = i in
        if i <. (rate /! mk_usize 8 <: usize) <: bool
        then
          (state.[ i ] <: u64) ^.
          (Core_models.Num.impl_u64__from_le_bytes (Core_models.Result.impl__unwrap #(t_Array u8
                      (mk_usize 8))
                  #Core_models.Array.t_TryFromSliceError
                  (Core_models.Convert.f_try_into #(t_Slice u8)
                      #(t_Array u8 (mk_usize 8))
                      #FStar.Tactics.Typeclasses.solve
                      (block.[ {
                            Core_models.Ops.Range.f_start = mk_usize 8 *! i <: usize;
                            Core_models.Ops.Range.f_end
                            =
                            (mk_usize 8 *! i <: usize) +! mk_usize 8 <: usize
                          }
                          <:
                          Core_models.Ops.Range.t_Range usize ]
                        <:
                        t_Slice u8)
                    <:
                    Core_models.Result.t_Result (t_Array u8 (mk_usize 8))
                      Core_models.Array.t_TryFromSliceError)
                <:
                t_Array u8 (mk_usize 8))
            <:
            u64)
          <:
          u64
        else state.[ i ] <: u64)

/// Extract `len` bytes from the rate portion of the state (little-endian, lane-interleaved).
/// Corresponds to `Trunc_r(S)` in Algorithm 8.
let squeeze_state
      (v_OUTPUT_LEN: usize)
      (state: t_Array u64 (mk_usize 25))
      (output: t_Array u8 v_OUTPUT_LEN)
      (out_offset len: usize)
    : Prims.Pure (t_Array u8 v_OUTPUT_LEN)
      (requires
        len <=. mk_usize 200 &&
        (Core_models.Slice.impl__len #u8 (output <: t_Slice u8) <: usize) >=. len &&
        out_offset <=.
        ((Core_models.Slice.impl__len #u8 (output <: t_Slice u8) <: usize) -! len <: usize))
      (ensures
        fun result ->
          let result:t_Array u8 v_OUTPUT_LEN = result in
          b2t
          ((Core_models.Slice.impl__len #u8 (result <: t_Slice u8) <: usize) =.
            (Core_models.Slice.impl__len #u8 (output <: t_Slice u8) <: usize)
            <:
            bool) /\
          (forall (i: usize).
              b2t
              (if i <. (Core_models.Slice.impl__len #u8 (output <: t_Slice u8) <: usize) <: bool
                then
                  if i <. out_offset <: bool
                  then (result.[ i ] <: u8) =. (output.[ i ] <: u8) <: bool
                  else
                    if i <. (out_offset +! len <: usize) <: bool
                    then
                      (result.[ i ] <: u8) =.
                      ((Core_models.Num.impl_u64__to_le_bytes (state.[ (i -! out_offset <: usize) /!
                                mk_usize 8
                                <:
                                usize ]
                              <:
                              u64)
                          <:
                          t_Array u8 (mk_usize 8)).[ (i -! out_offset <: usize) %! mk_usize 8
                          <:
                          usize ]
                        <:
                        u8)
                      <:
                      bool
                    else (result.[ i ] <: u8) =. (output.[ i ] <: u8) <: bool
                else true))) =
  let (bytes: t_Array u8 (mk_usize 200)):t_Array u8 (mk_usize 200) =
    Hacspec_sha3.createi #u8
      (mk_usize 200)
      #(usize -> u8)
      (fun i ->
          let i:usize = i in
          (Core_models.Num.impl_u64__to_le_bytes (state.[ i /! mk_usize 8 <: usize ] <: u64)
            <:
            t_Array u8 (mk_usize 8)).[ i %! mk_usize 8 <: usize ]
          <:
          u8)
  in
  let _:Prims.unit =
    Proof_Utils.Lemmas.lemma_index_update_at_range output
      ({
          Core_models.Ops.Range.f_start = out_offset;
          Core_models.Ops.Range.f_end = out_offset +! len
        }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Seq.slice bytes 0 (v len))
  in
  let output:t_Array u8 v_OUTPUT_LEN =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range output
      ({
          Core_models.Ops.Range.f_start = out_offset;
          Core_models.Ops.Range.f_end = out_offset +! len <: usize
        }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (output.[ {
                Core_models.Ops.Range.f_start = out_offset;
                Core_models.Ops.Range.f_end = out_offset +! len <: usize
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
          (bytes.[ { Core_models.Ops.Range.f_start = mk_usize 0; Core_models.Ops.Range.f_end = len }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
        <:
        t_Slice u8)
  in
  output

/// Absorb one full block: XOR it into the state, then apply Keccak-f.
/// Corresponds to one iteration of the absorb loop in Algorithm 8 (step 6).
let absorb_block (state: t_Array u64 (mk_usize 25)) (block: t_Slice u8) (rate: usize)
    : Prims.Pure (t_Array u64 (mk_usize 25))
      (requires
        rate <=. mk_usize 200 && (rate %! mk_usize 8 <: usize) =. mk_usize 0 &&
        (Core_models.Slice.impl__len #u8 block <: usize) =. rate)
      (fun _ -> Prims.l_True) =
  let state:t_Array u64 (mk_usize 25) = xor_block_into_state state block rate in
  Hacspec_sha3.Keccak_f.keccak_f state

#push-options "--z3rlimit 200"

/// Build the padded last block: copy remaining message bytes, add the
/// domain-separation byte `delim`, and set the final bit of pad10*1.
/// Returns a `rate`-byte buffer ready to be absorbed via `xor_block_into_state`.
let pad_last_block (message: t_Slice u8) (msg_offset remaining rate: usize) (delim: u8)
    : Prims.Pure (t_Array u8 (mk_usize 200))
      (requires
        rate >. mk_usize 0 && rate <=. mk_usize 200 && (rate %! mk_usize 8 <: usize) =. mk_usize 0 &&
        remaining <. rate &&
        msg_offset <=. (Core_models.Slice.impl__len #u8 message <: usize) &&
        remaining <=. ((Core_models.Slice.impl__len #u8 message <: usize) -! msg_offset <: usize))
      (fun _ -> Prims.l_True) =
  let buffer:t_Array u8 (mk_usize 200) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 200) in
  let buffer:t_Array u8 (mk_usize 200) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_range buffer
      ({ Core_models.Ops.Range.f_start = mk_usize 0; Core_models.Ops.Range.f_end = remaining }
        <:
        Core_models.Ops.Range.t_Range usize)
      (Core_models.Slice.impl__copy_from_slice #u8
          (buffer.[ {
                Core_models.Ops.Range.f_start = mk_usize 0;
                Core_models.Ops.Range.f_end = remaining
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
          (message.[ {
                Core_models.Ops.Range.f_start = msg_offset;
                Core_models.Ops.Range.f_end = msg_offset +! remaining <: usize
              }
              <:
              Core_models.Ops.Range.t_Range usize ]
            <:
            t_Slice u8)
        <:
        t_Slice u8)
  in
  let buffer:t_Array u8 (mk_usize 200) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize buffer remaining delim
  in
  let buffer:t_Array u8 (mk_usize 200) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize buffer
      (rate -! mk_usize 1 <: usize)
      ((buffer.[ rate -! mk_usize 1 <: usize ] <: u8) |. mk_u8 128 <: u8)
  in
  buffer

#pop-options

#push-options "--z3rlimit 200"

/// Absorb the final (possibly partial) block: pad it, XOR into state, and
/// apply Keccak-f.
/// Combines `pad_last_block` + `absorb_block`.
let absorb_final
      (state: t_Array u64 (mk_usize 25))
      (message: t_Slice u8)
      (msg_offset remaining rate: usize)
      (delim: u8)
    : Prims.Pure (t_Array u64 (mk_usize 25))
      (requires
        rate >. mk_usize 0 && rate <=. mk_usize 200 && (rate %! mk_usize 8 <: usize) =. mk_usize 0 &&
        remaining <. rate &&
        msg_offset <=. (Core_models.Slice.impl__len #u8 message <: usize) &&
        remaining <=. ((Core_models.Slice.impl__len #u8 message <: usize) -! msg_offset <: usize))
      (fun _ -> Prims.l_True) =
  let block:t_Array u8 (mk_usize 200) = pad_last_block message msg_offset remaining rate delim in
  absorb_block state
    (block.[ { Core_models.Ops.Range.f_start = mk_usize 0; Core_models.Ops.Range.f_end = rate }
        <:
        Core_models.Ops.Range.t_Range usize ]
      <:
      t_Slice u8)
    rate

#pop-options

/// Final partial-block step of [squeeze]: if `output_rem != 0`, apply
/// one Keccak-f permutation and extract the trailing `output_rem`
/// bytes; otherwise a no-op.  Factored out so the libcrux impl\'s
/// corresponding helper can have a matching shape, keeping the
/// equivalence proof\'s final reconciliation local.
let squeeze_last
      (v_OUTPUT_LEN: usize)
      (state: t_Array u64 (mk_usize 25))
      (output: t_Array u8 v_OUTPUT_LEN)
      (rate output_rem: usize)
    : Prims.Pure (t_Array u64 (mk_usize 25) & t_Array u8 v_OUTPUT_LEN)
      (requires
        rate >. mk_usize 0 && rate <=. mk_usize 200 && (rate %! mk_usize 8 <: usize) =. mk_usize 0 &&
        output_rem <. rate &&
        output_rem <=. v_OUTPUT_LEN &&
        v_OUTPUT_LEN <. (Core_models.Num.impl_usize__MAX -! mk_usize 200 <: usize))
      (fun _ -> Prims.l_True) =
  let _:usize = rate in
  if output_rem <>. mk_usize 0
  then
    let state:t_Array u64 (mk_usize 25) = Hacspec_sha3.Keccak_f.keccak_f state in
    let output:t_Array u8 v_OUTPUT_LEN =
      squeeze_state v_OUTPUT_LEN state output (v_OUTPUT_LEN -! output_rem <: usize) output_rem
    in
    state, output <: (t_Array u64 (mk_usize 25) & t_Array u8 v_OUTPUT_LEN)
  else state, output <: (t_Array u64 (mk_usize 25) & t_Array u8 v_OUTPUT_LEN)

/// Recursively absorb the remaining bytes of `message`: peel off one full
/// `rate`-byte block, XOR it into the state, apply Keccak-f, then recurse on
/// the tail slice. Once fewer than `rate` bytes remain, pad and absorb the
/// partial final block.
/// This recursive form is chosen so the extracted F* definition lines up
/// block-for-block with the libcrux impl\'s absorb loop.
let rec absorb_rec (state: t_Array u64 (mk_usize 25)) (rate: usize) (delim: u8) (message: t_Slice u8)
    : Prims.Pure (t_Array u64 (mk_usize 25))
      (requires
        rate >. mk_usize 0 && rate <=. mk_usize 200 && (rate %! mk_usize 8 <: usize) =. mk_usize 0)
      (fun _ -> Prims.l_True)
      (decreases
        (Rust_primitives.Hax.Int.from_machine (Core_models.Slice.impl__len #u8 message <: usize)
          <:
          Hax_lib.Int.t_Int)) =
  if (Core_models.Slice.impl__len #u8 message <: usize) <. rate
  then
    absorb_final state
      message
      (mk_usize 0)
      (Core_models.Slice.impl__len #u8 message <: usize)
      rate
      delim
  else
    let state:t_Array u64 (mk_usize 25) =
      absorb_block state
        (message.[ {
              Core_models.Ops.Range.f_start = mk_usize 0;
              Core_models.Ops.Range.f_end = rate
            }
            <:
            Core_models.Ops.Range.t_Range usize ]
          <:
          t_Slice u8)
        rate
    in
    absorb_rec state
      rate
      delim
      (message.[ { Core_models.Ops.Range.f_start = rate } <: Core_models.Ops.Range.t_RangeFrom usize
        ]
        <:
        t_Slice u8)

#push-options "--z3rlimit 200"

/// Iterative middle-loop of [absorb]: for each block index `k ∈ [i, input_blocks)`,
/// XOR `input[k*rate..(k+1)*rate]` into the state and apply Keccak-f via
/// [absorb_block]. Returns the state after the final [absorb_block].
/// Shape chosen to mirror [squeeze_blocks], so the F* equivalence proof can line
/// up the libcrux impl\'s `fold_range 0 input_blocks` step-by-step against this
/// recursion via tail-extension lemmas, dodging the slice-of-slice reasoning
/// that triggers a Z3 4.13.3 LP-solver internal-assertion bug in the older
/// proof that used `absorb_rec` with progressive `message[rate..]` tails.
let rec absorb_blocks
      (state: t_Array u64 (mk_usize 25))
      (rate i input_blocks: usize)
      (input: t_Slice u8)
    : Prims.Pure (t_Array u64 (mk_usize 25))
      (requires
        rate >. mk_usize 0 && rate <=. mk_usize 200 && (rate %! mk_usize 8 <: usize) =. mk_usize 0 &&
        i <=. input_blocks &&
        input_blocks <=. ((Core_models.Slice.impl__len #u8 input <: usize) /! rate <: usize))
      (fun _ -> Prims.l_True)
      (decreases
        ((Rust_primitives.Hax.Int.from_machine input_blocks <: Hax_lib.Int.t_Int) -
          (Rust_primitives.Hax.Int.from_machine i <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int)) =
  if i <. input_blocks
  then
    let state:t_Array u64 (mk_usize 25) =
      absorb_block state
        (input.[ {
              Core_models.Ops.Range.f_start = i *! rate <: usize;
              Core_models.Ops.Range.f_end = (i *! rate <: usize) +! rate <: usize
            }
            <:
            Core_models.Ops.Range.t_Range usize ]
          <:
          t_Slice u8)
        rate
    in
    absorb_blocks state rate (i +! mk_usize 1 <: usize) input_blocks input
  else state

#pop-options

/// Apply Keccak-f to `state` exactly `n` times.  Right-add recursion
/// so `iterate_keccak_f(n+1, state) == keccak_f(iterate_keccak_f(n, state))`
/// is by definitional unfold at fuel 1 — consumers don\'t need a
/// separate lemma to peel one round off the right.
let rec iterate_keccak_f (n: usize) (state: t_Array u64 (mk_usize 25))
    : Prims.Tot (t_Array u64 (mk_usize 25))
      (decreases (Rust_primitives.Hax.Int.from_machine n <: Hax_lib.Int.t_Int)) =
  if n =. mk_usize 0
  then state
  else
    Hacspec_sha3.Keccak_f.keccak_f (iterate_keccak_f (n -! mk_usize 1 <: usize) state
        <:
        t_Array u64 (mk_usize 25))

#push-options "--z3rlimit 300"

/// Recursive middle-loop of [squeeze]: for each block index `i ∈ [i, output_blocks)`,
/// apply `keccak_f` and then extract `rate` bytes at `i * rate`. Returns the state
/// after the final `keccak_f`.
/// Shape chosen to mirror `absorb_rec`, so the F* equivalence proof can line up
/// the libcrux impl\'s `fold_range 1 output_blocks` step-by-step against this
/// recursion via `lemma_fold_range_step`, the same pattern used for absorb.
let rec squeeze_blocks
      (v_OUTPUT_LEN: usize)
      (state: t_Array u64 (mk_usize 25))
      (rate i output_blocks: usize)
      (output: t_Array u8 v_OUTPUT_LEN)
    : Prims.Pure (t_Array u64 (mk_usize 25) & t_Array u8 v_OUTPUT_LEN)
      (requires
        rate >. mk_usize 0 && rate <=. mk_usize 200 && (rate %! mk_usize 8 <: usize) =. mk_usize 0 &&
        i <=. output_blocks &&
        output_blocks <=.
        ((Core_models.Slice.impl__len #u8 (output <: t_Slice u8) <: usize) /! rate <: usize))
      (fun _ -> Prims.l_True)
      (decreases
        ((Rust_primitives.Hax.Int.from_machine output_blocks <: Hax_lib.Int.t_Int) -
          (Rust_primitives.Hax.Int.from_machine i <: Hax_lib.Int.t_Int)
          <:
          Hax_lib.Int.t_Int)) =
  if i <. output_blocks
  then
    let state:t_Array u64 (mk_usize 25) = Hacspec_sha3.Keccak_f.keccak_f state in
    let output:t_Array u8 v_OUTPUT_LEN =
      squeeze_state v_OUTPUT_LEN state output (i *! rate <: usize) rate
    in
    squeeze_blocks v_OUTPUT_LEN state rate (i +! mk_usize 1 <: usize) output_blocks output
  else state, output <: (t_Array u64 (mk_usize 25) & t_Array u8 v_OUTPUT_LEN)

#pop-options

#push-options "--z3rlimit 200"

/// Absorb phase of the Keccak sponge (FIPS 202, Algorithm 8, step 6 combined
/// with the pad10*1 padding of Algorithm 9).
/// Splits `message` into `rate`-byte blocks, XORing each into the state and
/// applying Keccak-f. The final partial block is padded with the domain
/// separation byte `delim` and the pad10*1 terminator `0x80` before being
/// absorbed.
let absorb (rate: usize) (delim: u8) (message: t_Slice u8)
    : Prims.Pure (t_Array u64 (mk_usize 25))
      (requires
        rate >. mk_usize 0 && rate <=. mk_usize 200 && (rate %! mk_usize 8 <: usize) =. mk_usize 0)
      (fun _ -> Prims.l_True) =
  absorb_rec (Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25) <: t_Array u64 (mk_usize 25))
    rate
    delim
    message

#pop-options

#push-options "--z3rlimit 500"

/// Squeeze phase of the Keccak sponge (FIPS 202, Algorithm 8, steps 8–9).
/// Extracts `OUTPUT_LEN` bytes from `state`, applying Keccak-f between each
/// `rate`-byte block of output.
/// Byteform definition: byte at position `k` lives in block `b = k / rate`
/// (or the trailing partial block if `b == OUTPUT_LEN / rate`); within a
/// block the offset is `j = k - b * rate`; the value is the `(j mod 8)`-th
/// little-endian byte of `iterate_keccak_f(b, state)`\'s lane `(j / 8)`.
/// Equivalent to FIPS-202 Algorithm 8: for each full block apply keccak_f
/// and extract `rate` bytes; the trailing partial block uses one more
/// keccak_f before extracting `OUTPUT_LEN mod rate` bytes.
let squeeze (v_OUTPUT_LEN: usize) (state: t_Array u64 (mk_usize 25)) (rate: usize)
    : Prims.Pure (t_Array u8 v_OUTPUT_LEN)
      (requires
        rate >. mk_usize 0 && rate <=. mk_usize 200 && (rate %! mk_usize 8 <: usize) =. mk_usize 0 &&
        v_OUTPUT_LEN <. (Core_models.Num.impl_usize__MAX -! mk_usize 200 <: usize))
      (fun _ -> Prims.l_True) =
  Hacspec_sha3.createi #u8
    v_OUTPUT_LEN
    #(usize -> u8)
    (fun k ->
        let k:usize = k in
        let b:usize = k /! rate in
        let j:usize = k -! (b *! rate <: usize) in
        let state_b:t_Array u64 (mk_usize 25) = iterate_keccak_f b state in
        (Core_models.Num.impl_u64__to_le_bytes (state_b.[ j /! mk_usize 8 <: usize ] <: u64)
          <:
          t_Array u8 (mk_usize 8)).[ j %! mk_usize 8 <: usize ])

#pop-options

/// Keccak sponge — FIPS 202, Algorithm 8 combined with pad10*1 (Algorithm 9).
/// 1. Absorb: split `message` into `rate`-byte blocks, XOR each into the
///    state, and apply Keccak-f. The final partial block is padded with
///    the domain separation byte `delim` and the pad10*1 terminator `0x80`.
/// 2. Squeeze: extract `OUTPUT_LEN` bytes from the state, applying
///    Keccak-f between each `rate`-byte block of output.
/// The `OUTPUT_LEN < usize::MAX - 200` precondition is a Rust implementation
/// artifact to prevent arithmetic overflow; FIPS 202 places no upper bound
/// on the output length.
let keccak (v_OUTPUT_LEN rate: usize) (delim: u8) (message: t_Slice u8)
    : Prims.Pure (t_Array u8 v_OUTPUT_LEN)
      (requires
        rate >. mk_usize 0 && rate <=. mk_usize 200 && (rate %! mk_usize 8 <: usize) =. mk_usize 0 &&
        v_OUTPUT_LEN <. (Core_models.Num.impl_usize__MAX -! mk_usize 200 <: usize))
      (fun _ -> Prims.l_True) =
  squeeze v_OUTPUT_LEN (absorb rate delim message <: t_Array u64 (mk_usize 25)) rate
