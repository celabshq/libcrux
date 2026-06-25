/// Errors raised by the ML-DSA spec.

#[derive(Debug, PartialEq, Eq)]
pub enum MlDsaError {
    /// `SampleInBall` (FIPS 204, Algorithm 29) exhausted the 1024-byte
    /// hash buffer without finding τ valid Fisher-Yates positions.
    ///
    /// FIPS 204 specifies `SampleInBall` as an unbounded loop that keeps
    /// squeezing the SHAKE-256 stream until success; this spec
    /// approximates that with a single 1024-byte block, which lets the
    /// proof obligations be discharged but introduces a finite failure
    /// mode that FIPS itself does not have.  The probability of hitting
    /// this case for the standard parameter sets (τ ≤ 60, expected
    /// consumption ≈ τ · 256 / (256 − τ) ≈ 76 bytes) is below 2⁻¹²⁸,
    /// i.e. *probabilistically impossible* in practice — we surface
    /// it only because the spec departs from FIPS in this one place
    /// and we want that departure to be explicit rather than silent.
    SampleInBallExhausted,
    /// `Sign_internal`'s outer rejection-sampling loop (FIPS 204
    /// Algorithm 7, lines 14–31) ran to its budget cap without
    /// producing a signature satisfying the commitment-norm and
    /// hint-count checks.  Probability < 2⁻¹²⁸ per FIPS for standard
    /// parameter sets.
    RejectionSamplingExhausted,
    /// `Verify_internal` could not decode the signature byte string —
    /// a length / hint-encoding check rejected before any cryptographic
    /// equality could be evaluated.
    MalformedSignature,
    /// `Verify_internal` decoded the signature but a downstream check
    /// (signer-response infinity norm, hint reconstruction, or
    /// commitment-hash equality) rejected.
    InvalidSignature,
}
