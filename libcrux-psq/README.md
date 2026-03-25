# Post-Quantum Pre-Shared-Key Protocol (PSQ) #

![pre-verification]

This crate implements a protocol for establishing, and registering a
post-quantum shared secret between an initiator `I` and a responder
`R`. The protocol is inspired by the Noise protocol framework, adapted
to potentially incorporate post-quantum key encapsultion (PQ-KEMs).

The following protocol description makes use of several cryptographic
primitives and notations which are [explained in detail below](#cryptographic-building-blocks-&-notation).

## Handshake
The PSQ handshake exists in two modes, [*query mode*](#query-mode) and [*registration
mode*](#registration-mode). In both, initiator and responder are assumed to share knowledge
of a common protocol context `context` which is incorporated into the
handshake transcript and thus serves as a domain separator between
different instantiations of the PSQ handshake.

The purpose of a registration mode run is that initiator and responder
[establish a session](#derived-sessions-&-secret-export) based on a shared secret `K_S` which is protected
against harvest-now-decrypt-later (HNDL) attacks from a quantum
adversary, if a ciphersuite is used that includes a PQ-KEM. From the
shared session secret any number of bidirectional transport channels
between initiator and responder can be created, or a derived secret
may be exported for external use. If the shared session secret enjoys
HNDL-protection, so do the derived transport channels and exported
secrets.

The purpose of a query mode run is that the initiator can send one payload
to the responder, which can return one response to the initiator.

See below for a description of the different [supported ciphersuites](#ciphersuite-support).

### Registration Mode
For registration mode, it is assumed that the initiator is aware of
the relevant long-term public keys of the responder, i.e. at minimum
the responder's long-term Diffie-Hellman (DH) public key `pub_R` and
optionally the responder's long-term PQ-KEM encapsulation key
`pqek_R`.

The initiator will include in its first message to the responder an
*authenticator* which can either be a long-term Diffie-Hellman public
key `pub_I` or a signature of the protocol transcript under an
included long-term verification key `vk_I`. We assume the responder
can validate the authenticity of the authenticator out-of-band.

In addition to the shared session secret that is the final outcome of
the registration mode run, the initiator may include in its first
message an application defined registration payload, and the responder
may include in its response an application defined response
payload.

If a PQ-KEM ciphersuite is employed the shared session secret as well
as both payloads are protected against HNDL attacks.

The shared secret can be used to derive a large number of secure
transport sessions between initiator and responder (see below).

#### Diffie-Hellman based initiator authentication
```
Common Inputs:
    - context
    
Inputs of I (Initiator):
    - registration_payload
    - registration_outer_aad
    - registration_inner_aad
    - pub_R
    - (priv_I, pub_I)
    - pqek_R (optional)
    
Inputs of R (Responder): 
    - (priv_R, pub_R)
    - (pqdk_R, pqek_R)
    - query handler f,
    - response_aad, 
    - registration handler f
    
I: 
    (epriv_I, epub_I) <- DH.KeyGen()
    tx0 = hash(0 | context | pub_R | epub_I)
    ss_dh_outer = DH.Derive(epriv_I, pub_R)
    K_0  = KDF(ss_dh_outer, tx0)
    if pqek_R provided
        (enc_pq, ss_pq) <- PQKEM.Encapsulate(pqek_R)
    tx1 = hash(1 | tx0 | pub_I | [pqek_S] | [enc_pq])
    ss_dh_inner = DH.Derive(priv_I, pub_R)
    K_1 = KDF(K_0 | ss_dh_inner | [ss_pq], tx1)
    ctxt_inner <- AEAD.Encrypt(K_1, registration_payload, registration_inner_aad)
    ctxt_outer <- AEAD.Encrypt(K_0, (pub_I | ctxt_inner | registration_inner_aad | [enc_pq]), registration_outer_aad)
    
I -> R: (epub_I, ctxt_outer, registration_outer_aad)

R:
    tx0 = hash(0 | context | pub_R | epub_I)
    ss_dh_outer = DH.Derive(priv_R, epub_I)
    K_0 = KDF(ss_dh_outer, tx0)
    (pub_I | ctxt_inner | registration_inner_aad | [enc_pq]) = AEAD.Decrypt(K_0, ctxt_outer, registration_outer_aad)
    if enc_pq provided
        ss_pq <- PQKEM.Decapsulate(pqdk_R, enc_pq)
    tx1 = hash(1 | tx0 | pub_I | [pqek_S] | [enc_pq])
    ss_dh_inner = DH.Derive(priv_R, pub_I)
    K_1 = KDF(K_0 | ss_dh_inner | [ss_pq], tx1)
    registration_payload = AEAD.Decrypt(K_1, ctxt_inner, registration_inner_aad)
    ...
    response_payload <- f(registration_payload)
    ...
    (epriv_R, epub_R) <- DH.KeyGen()
    tx2 = hash(2 | tx1 | epub_R)
    ss_dh_response_1 = DH.Derive(epriv_R, pub_I)
    ss_dh_response_2 = DH.Derive(epriv_R, epub_I)
    K_2 = KDF(K_1 | ss_dh_response_1 | ss_dh_response_2, tx2)
    ctxt_response <- AEAD.Encrypt(K_2, response_payload, response_aad)
    
R -> I: (epub_R, ctxt_response, response_aad)

I: 
    tx2 = hash(2 | tx1 | epub_R)
    ss_dh_response_1 = DH.Derive(priv_I, epub_R)
    ss_dh_response_2 = DH.Derive(epriv_I, epub_R)
    K_2 = KDF(K_1 | ss_dh_response_1 | ss_dh_response_2, tx2)
    response_payload = AEAD.Decrypt(K_2, ctxt_response, response_aad)

```

#### Signature-based Initiator Authentication
```
Common Inputs:
    - context
    
Inputs of I: 
    - registration_payload
    - registration_outer_aad
    - registration_inner_aad
    - pub_R
    - (priv_I, vk_I)
    - pqpk_R (optional)
    
Inputs of R: 
    - (priv_R, pub_R)
    - (pqsk_R, pqpk_R)
    - query handler f,
    - response_aad, 
    - registration handler f
    
I: 
    (epriv_I, epub_I) <- DH.KeyGen()
    tx0 = hash(0 | context | pub_R | epub_I)
    ss_dh_outer = DH.Derive(epriv_I, pub_R)
    K_0  = KDF(ss_dh_outer, tx0)
    if pqpk_R provided
        (enc_pq, ss_pq) <- PQKEM.Encapsulate(pqpk_S)
    tx1 = hash(1 | tx0 | vk_I | [pqpk_S] | [enc_pq])
    sigC = Sig.Sign(priv_I, tx1)
    K_1 = KDF(K_0 | [ss_pq], tx1 | sigC)
    ctxt_inner <- AEAD.Encrypt(K_1, registration_payload, registration_inner_aad)
    ctxt_outer <- AEAD.Encrypt(K_0, (vk_I | ctxt_inner | registration_inner_aad | | sigC | [enc_pq]), registration_outer_aad)
    
I -> R: (epub_I, ctxt_outer, registration_outer_aad)

R:
    tx0 = hash(0 | context | pub_R | epub_I)
    ss_dh_outer = DH.Derive(priv_R, epub_I)
    K_0 = KDF(ss_dh_outer)
    (vk_I | ctxt_inner | registration_inner_aad | sigC | [enc_pq]) = AEAD.Decrypt(K_0, ctxt_outer, registration_outer_aad)
    tx1 = hash(1 | tx0 | vk_I | [pqpk_S] | [enc_pq])
    if !Sig.Verify(vk_I, tx1, sigC)
        abort
    if enc_pq provided
        ss_pq <- PQKEM.Decapsulate(pqsk_R, enc_pq)
    K_1 = KDF(K_0 | [ss_pq], tx1 | sigC)
    registration_payload = AEAD.Decrypt(K_1, ctxt_inner, registration_inner_aad)
    ...
    response_payload <- f(registration_payload)
    ...
    (epriv_R = y, epub_R = g^y) <- DH.KeyGen()
    tx2 = hash(2 | tx1 | epub_R)
    ss_dh_response = DH.Derive(epriv_R, epub_I)
    K_2 = KDF(K_1 | ss_dh_response, tx2)
    ctxt_response <- AEAD.Encrypt(K_2, response_payload, response_aad)
    
R -> I: (epub_R, ctxt_response, response_aad)

I: 
    tx2 = hash(2 | tx1 | epub_R)
    ss_dh_response = DH.Derive(epriv_I, epub_R)
    K_2 = KDF(K_1 | ss_dh_response, tx2)
    response_payload = AEAD.Decrypt(K_2, ctxt_response, response_aad)
```

### Query Mode
The purpose of a query mode run is that the initiator can send one application-defined query payload
to the responder, which can return one application-defined response payload to the initiator.

These payloads **do not** enjoy post quantum protection.

```
Common inputs:
    - context

Inputs of I (Initiator):
    - query_payload
    - query_aad
    - pub_R
    
Inputs of R (Responder): 
    - (priv_R, pub_R)
    - response_aad
    - query handler f

I: 
    (epriv_I, epub_I) <- DH.KeyGen()
    tx0 = hash(0 | context | pub_R | epub_I)
    dh_shared_secret_query = DH.Derive(epriv_I, pub_R)
    K_0  = KDF(dh_shared_secret_query, tx0)
    ctxt_query <- AEAD.Encrypt(K_0, query_payload, query_aad)
    
I -> R: (epub_I, ctxt_query, query_aad)

R:
    tx0 = hash(0 | context | pub_R | epub_I)
    dh_shared_secret_query = DH.Derive(priv_R, epub_I)
    K_0 = KDF(dh_shared_secret_query, tx0)
    query_payload = AEAD.Decrypt(K_0, ctxt_query, query_aad)
    ...
    response_payload <- f(query_payload)
    ...
    (epriv_R, epub_R) <- DH.KeyGen()
    tx2 = hash(2 | tx0 | epub_R)
    dh_shared_secret_response_1 = DH.Derive(priv_R, epub_I)
    dh_shared_secret_response_2 = DH.Derive(epriv_R, epub_I)
    K_2 = KDF(K_0 | dh_shared_secret_response_1 | dh_shared_secret_response_2, tx2)
    ctxt_response <- AEAD.Encrypt(K_2, response_payload, response_aad)

R -> I: (epub_R, ctxt_response, response_aad)

I:
    tx2 = hash(2 | tx0 | epub_R)
    dh_shared_secret_response_1 = DH.Derive(epriv_I, pub_R)
    dh_shared_secret_response_2 = DH.Derive(epriv_I, epub_R)
    K_2 = KDF(K_0 | dh_shared_secret_response_1 | dh_shared_secret_response_2, tx2)
    response_payload = AEAD.Decrypt(K_2, ctxt_response, response_aad)
```

## Derived Sessions & Secret Export
Given a shared secret `K_2` and the final handshake transcript `tx2`
initiator and receiver derive a main session key as well as an
associated session identifier
```
K_S = KDF(K_2, "session key" | tx2)
session_ID = KDF(K_S, "shared key id")
```
They also compute an associated public key binder value
```
pk_binder = KDF(K_S, pub_A | pub_B | [pqek_B])
```
or
```
pk_binder = KDF(K_S, vk_A | pub_B | [pqek_B])
```
depending on the authentication mode.

From this main session key, initiator and responder can derive
bidirectional transport keys for many secure channels, where
`channel_counter` identifies the particular channel:
```
K_i2r = KDF(K_S, "i2r channel key" | pk_binder | channel_counter)
K_r2i = KDF(K_S, "r2i channel key" | pk_binder | channel_counter)
```

Additionally, both parties can export secrets of any length for external use, which are derived as
```
K = KDF(K_S, context | "PSQ secret export")
```
where `context` is an application-defined context string for the
exported secret.

## Session Secret Import
Given an existing session with main session key `K_S` the session
can be re-keyed with an external secret `psk`. This is achieved by
deriving an imported key from `K_S` and `psk` and updating the most
recent session transcript `tx` with the old session ID that is about
to become invalid:
```
K_import = KDF(K_S || psk, "secret import")
tx' = Hash(tx || session_ID)
```
Now, the new main session key is created by treating `K_import` and `tx'` as
though they were the outcome of a PSQ handshake:
```
K_S' = KDF(K_import, "session secret" | tx')
session_ID' = KDF(K_S', "shared key id")
```

## Cryptographic Building Blocks & Notation
The description of the PSQ protocol below relies on several
cryptographic building blocks represented as abstract interfaces.

### Diffie-Hellman key exchange
The PSQ handshake uses a Diffie-Hellman key exchange `DH` with the
following interface:
- `DH.KeyGen()` generates pair of Diffie-Hellmann public and private
  keys. For a protocol participant `X`, we will denote their long-term
  DH public key as `pub_X` and their long-term DH private key as
  `priv_X`. Ephemeral public keys will be denoted `epub_X` and `epriv_X`.
- `DH.Derive(sk, pk)` takes as input a DH private key and a DH public
  key and derives a DH shared secret.
  
### Post-Quantum Key Encapsulation mechanims
The PSQ handshake may a post-quantum key encapsulation mechanism to
incorporate a PQ-secure shared secret into the session secret.
- `PQKEM.KeyGen()` generates a pair of encapsulation and decapsulation
  keys for the PQ-KEM. For a protocol participant `X`, we will denote
  their long-term PQ-KEM encapsulation key as `pqek_X` and their
  long-term PQ-KEM decapsulation key as `pqdk_X`.
- `PQKEM.Encapsulate(pqek)` encapsulates a shared secret towards a
  PQ-KEM encapsulation key, outputting the encapsulation value
  `enc_pq` and the shared secret `ss_pq`.
- `PQKEM.Decapsulate(pqdk, enc_pq)` decapsulates a shared secret
  `ss_pq` from the encapsulation value `enc_pq`. 
  
### Digital Signatures

The PSQ handshake may use digital signatures for initiator authentication:
- `Sig.KeyGen()` generates a pair of signing and verification
  keys. For a protocol participant `X`, we denote their long-term
  signature verification key as `vk_X` and their long-term signing key
  as `sk_X`.
- `Sig.Sign(sk, m)` signs message `m` using signing key `sk`,
  producing a signature `sig`.
- `Sig.Verify(vk, m, sig)` attempts to verify purported signature
  `sig` on message `m` under verification key `vk`. If successful,
  outputs `true`, otherwise `false`.
  
### Authenticated Encryption
The PSQ handshake uses authenticated encryption with associated data
for encrypting handshake and transport payloads.
- `AEAD.Encrypt(K, plaintext, aad)` encrypts message `plaintext` and
  authenticates associated data `aad` under key `K`, outputting
  ciphertext `ctxt`. The authentication tag is left implicit in this
  description.
- `AEAD.Decrypt(K, ctxt, aad)` attempts to decrypt and authenticate
  `ctxt` and `aad` under key `K` returning the original `plaintext` if
  successful and aborting the protocol otherwise.

### Key derivation
AEAD encryption keys are obtained using a key derivation function.
- `KDF(ikm, info)` derives a fresh AEAD key `K` from initial key
  material `ikm` and non-confidential context information `info`.

### Cryptographic Hashing
A cryptographic hash function `hash` is used to keep a running hash of
the protocol transcript.

### (De)-Serialization
Messages and primitive inputs, e.g. to hash functions or KDFs, in PSQ
are serialized using TLS codec as defined in RFC 8446. In our notation
we write `x | y` for the concatenation of two inputs, which is
realized internally via structure types in the TLS presentation
language. An input `z` that is optionally provided is denoted in
square brackets as `[z]` and whether it is present or not is encoded
in the serialization.

## Ciphersuite Support
PSQ supports ciphersuites according to the following mask:
```
OUTER_PQKEM_AUTH_AEAD_KDF
```
where
- `OUTER` is the elliptic curve Diffie-Hellman key exchange used for
  the outer messsage layer. Supported curves at this point are:
  `X25519`.
- `PQKEM` is the PQ-KEM used in the inner message. Supported PQ-KEMs
  at this point are: `MLKEM768`, `CLASSICMCELIECE` (using feature
  `classic-mceliece`) and `NONE` (indicating no PQ-KEM will be used).
- `AUTH` is the method of initiator authentication used in the inner
  message. Supported authentication methods at this point are
  authentication via the initiator's long-term `X25519` public key, or
  authentication via a signature under the initiator's long-term
  signing key. Supported signature schemes at this point are:
  `MLDSA65` and `ED25519`. Initiator long term public and signing keys
  are assumed to be available to responder out-of-band.
- `AEAD` is the AEAD used for encrypting message payloads. Supported
  AEADs at this point are: `CHACHA20POLY1305` and `AESGCM128`.
- `KDF` is the key derivation function used to derive AEAD
  keys. Supported KDFs at this point are `HKDFSHA256`.

The full list of supported ciphersuites is as follows:
```
X25519_NONE_X25519_CHACHA20POLY1305_HKDFSHA256
X25519_MLKEM768_X25519_CHACHA20POLY1305_HKDFSHA256
X25519_CLASSICMCELIECE_X25519_CHACHA20POLY1305_HKDFSHA256
X25519_NONE_X25519_AESGCM128_HKDFSHA256
X25519_MLKEM768_X25519_AESGCM128_HKDFSHA256
X25519_CLASSICMCELIECE_X25519_AESGCM128_HKDFSHA256
X25519_NONE_ED25519_CHACHA20POLY1305_HKDFSHA256
X25519_MLKEM768_ED25519_CHACHA20POLY1305_HKDFSHA256
X25519_CLASSICMCELIECE_ED25519_CHACHA20POLY1305_HKDFSHA256
X25519_NONE_ED25519_AESGCM128_HKDFSHA256
X25519_MLKEM768_ED25519_AESGCM128_HKDFSHA256
X25519_CLASSICMCELIECE_ED25519_AESGCM128_HKDFSHA256
X25519_NONE_MLDSA65_CHACHA20POLY1305_HKDFSHA256
X25519_MLKEM768_MLDSA65_CHACHA20POLY1305_HKDFSHA256
X25519_CLASSICMCELIECE_MLDSA65_CHACHA20POLY1305_HKDFSHA256
X25519_NONE_MLDSA65_AESGCM128_HKDFSHA256
X25519_MLKEM768_MLDSA65_AESGCM128_HKDFSHA256
X25519_CLASSICMCELIECE_MLDSA65_AESGCM128_HKDFSHA256
```

## PSQ v1
Under optional feature `v1`, this crate implements the first version of PSQ, a
protocol for establishing and mutually registering a pre-shared key
such that the protocol messages are secure against
harvest-now-decrypt-later (HNDL) passive quantum attackers.

This implementation is exposed via the `libcrux_psq::v1` module.

The protocol between initator `A` and receiver `B` roughly works as follows:

```
A:  (K_pq, enc_pq) <- PQPSK.Encaps(pqpk_B, sctx)
    (K_regA, N_regA) <- KDF(K_pq, "AEAD-Responder-Initiator")
    (K_regB, N_regB) <- KDF(K_pq, "AEAD-Initiator-Responder")
    PSK <- KDF(K_pq, "PSK-Registration")
    ts <- get_timestamp()
    signature <- Sign(sk_A, enc_pq)

A -> B: (enc_pq, ctxt_A = AEAD.Encrypt(K_regA, N_regA, ts || signature || vk_A || ...))

B:  K_pq <- PQPSK.Decaps(pqsk_B, pqpk_B, sctx)
    (K_regA, N_regA) <- KDF(K_pq, "AEAD-Responder-Initiator")
    (K_regB, N_regB) <- KDF(K_pq, "AEAD-Initiator-Responder")
    (ts || signature || vk_A || ...) <- AEAD.Decrypt(K_reqA, N_regA, ctxt)
    if Verify(vk, signature, enc_pq) != 1 || ts_elapsed(ts, psk_ttl) then 
        ABORT
    else
    PSK <- KDF(K_pq, "PSK-Registration")
    psk_handle <- gen_handle()
    store (psk_handle, PSK)

B -> A: ctxt_B = AEAD.Encrypt(K_regB, N_regB, psk_handle)

A:  psk_handle <- AEAD.Decrypt(K_reqB, N_regB, ctxt_B)
    PSK <- KDF(K_pq, "PSK-Registration")
        store (psk_handle, PSK)
```

where `PQPSK.Encaps(pqpk_B, sctx)` denotes the following procedure:

```
(ik, enc) <- PQ-KEM.Encaps(pk_B)
K_0 <- KDF(ik, pk_B || enc || sctxt)
K_m <- KDF(K_0, "Confirmation")
K <- KDF(K_0, "PQ-PSK")
mac <- MAC(K_m, "MAC-Input")
return (K, enc||mac)
```

`PQPSK.Decaps(sk_B, pk_B, enc||mac)` denotes the following procedure:

```
ik <- PQ-KEM.Decaps(pqsk_B, enc)
K_0 <- KDF(ik, pk_B || enc || sctxt)
K_m <- KDF(K_0, "Confirmation")
K <- KDF(K_0, "PQ-PSK")
recomputed_mac <- MAC(K_m, "MAC-Input")
if mac != recomputed_mac then ABORT
else return K
```

and

* `pqpk_B` is the receiver's KEM public key,
* `pqsk_B` is the receiver's KEM private key,
* `sctx` is context information for the given session of the protocol,
* `psk_ttl` specifies for how long the PSK should be considered valid, and
* `psk_handle` is a storage handle for the established PSK, designated by the responder.
  
The crate implements the protocol based on several different internal
KEMs:

* `MlKem768`, a lattice-based post-quantum KEM, in the process
 of being standardized by NIST
* `XWingKemDraft02`, a hybrid post-quantum KEM, combining `X25519`
 and `ML-KEM 768` based KEMs
* `Classic McEliece`, a code-based post-quantum KEM & Round 4
 candidate in the NIST PQ competition, available under feature
 `classic-mceliece` and implemented using the third-party crate
 [`classic-mceliece-rust`](https://crates.io/crates/classic-mceliece-rust).
* `X25519`, an elliptic-curve Diffie-Hellman KEM. ⚠️ This KEM *does
 not* provide post-quantum security and is included only for testing
 and benchmarking purposes under feature `test-utils`.

For `MlKem768`, `XWingKemDraft06`, and `X25519` we use `libcrux`'s
own optimized implementations.

[verified]: ../.assets/verified-brightgreen.svg
[pre-verification]: ../.assets/pre_verification-orange.svg

