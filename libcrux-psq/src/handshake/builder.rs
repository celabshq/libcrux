use rand::CryptoRng;

use super::{
    builders::BuilderError as Error,
    initiator::registration::RegistrationInitiator,
    // pqkem::{PQKeyPair, PQPublicKey},
    responder::Responder,
};
use crate::handshake::{
    ciphersuite::{initiator::InitiatorCiphersuite, responder::ResponderCiphersuite},
    dhkem::DHPublicKey,
    initiator::query::QueryInitiator,
};

const RECENT_KEYS_DEFAULT_BOUND: usize = 100;

pub struct PrincipalBuilder<'a, Rng: CryptoRng> {
    rng: Rng,
    context: &'a [u8],
    inner_aad: &'a [u8],
    outer_aad: &'a [u8],
    responder_recent_keys_upper_bound: usize,
}

impl<'a, Rng: CryptoRng> PrincipalBuilder<'a, Rng> {
    /// Create a new builder.
    pub fn new(rng: Rng) -> Self {
        Self {
            rng,
            context: &[],
            inner_aad: &[],
            outer_aad: &[],
            responder_recent_keys_upper_bound: RECENT_KEYS_DEFAULT_BOUND,
        }
    }

    // properties

    /// Set the context.
    pub fn context(mut self, context: &'a [u8]) -> Self {
        self.context = context;
        self
    }

    /// Set the inner AAD.
    pub fn inner_aad(mut self, inner_aad: &'a [u8]) -> Self {
        self.inner_aad = inner_aad;
        self
    }

    /// Set the outer AAD.
    pub fn outer_aad(mut self, outer_aad: &'a [u8]) -> Self {
        self.outer_aad = outer_aad;
        self
    }

    pub fn recent_keys_upper_bound(mut self, recent_keys_upper_bound: usize) -> Self {
        // A bound of 0 would make the rate-limit cache's evict-when-full
        // check (`len() == upper_bound`) never trigger again after the first
        // insertion, silently disabling the cache instead of limiting it.
        self.responder_recent_keys_upper_bound = recent_keys_upper_bound.max(1);
        self
    } // builders

    /// Build a new [`QueryInitiator`].
    ///
    /// This requires that a `responder_ecdh_pk` is set.
    /// It also uses the `context` and `outer_aad`.
    pub fn build_query_initiator(
        self,
        peer_longterm_ecdh_pk: &'a DHPublicKey,
    ) -> Result<QueryInitiator<'a>, Error> {
        QueryInitiator::new(
            peer_longterm_ecdh_pk,
            self.context,
            self.outer_aad,
            self.rng,
        )
        .map_err(|_| Error::PrincipalBuilderState)
    }

    /// Build a new [`RegistrationInitiator`].
    ///
    /// This requires that a `longterm_ecdh_keys` and a `peer_longterm_ecdh_pk`
    /// is set.
    /// It also uses the `context`, `inner_aad`, `outer_aad`, and
    /// `peer_longterm_pq_pk`.
    pub fn build_registration_initiator(
        self,
        ciphersuite: InitiatorCiphersuite<'a>,
    ) -> Result<RegistrationInitiator<'a, Rng>, Error> {
        RegistrationInitiator::new(
            ciphersuite,
            self.context,
            self.inner_aad,
            self.outer_aad,
            self.rng,
        )
        .map_err(|_| Error::PrincipalBuilderState)
    }

    /// Build a new [`Responder`].
    ///
    /// This requires that a `longterm_ecdh_keys`, and `recent_keys_upper_bound`
    /// is set. It also uses the `context`, `outer_aad`, and
    /// `longterm_pq_keys`.
    pub fn build_responder(
        self,
        ciphersuite: ResponderCiphersuite<'a>,
    ) -> Result<Responder<'a, Rng>, Error> {
        Ok(Responder::new(
            ciphersuite,
            self.context,
            self.outer_aad,
            self.responder_recent_keys_upper_bound,
            self.rng,
        ))
    }
}

#[cfg(test)]
mod recent_keys_upper_bound_tests {
    use super::*;

    // `recent_keys_upper_bound(0)` used to be stored verbatim, which silently
    // disabled the responder's replay/rate-limit cache (the evict-when-full
    // check `len() == upper_bound` never fires again once `len()` passes 0).
    // It must be clamped to a minimum of 1.
    #[test]
    fn zero_is_clamped_to_one() {
        let builder = PrincipalBuilder::new(rand::rng()).recent_keys_upper_bound(0);
        assert_eq!(builder.responder_recent_keys_upper_bound, 1);
    }

    #[test]
    fn nonzero_is_unchanged() {
        let builder = PrincipalBuilder::new(rand::rng()).recent_keys_upper_bound(42);
        assert_eq!(builder.responder_recent_keys_upper_bound, 42);
    }
}
