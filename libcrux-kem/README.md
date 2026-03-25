# Key Encapsulation Mechanism (KEM)

![pre-verification]

This crate provides a KEM interface.

Available algorithms:

* [`Algorithm::X25519`]\: x25519 ECDH KEM.
* [`Algorithm::Secp256r1`]\: NIST P256 ECDH KEM.
* [`Algorithm::MlKem512`]\: ML-KEM 512 from [FIPS 203].
* [`Algorithm::MlKem768`]\: ML-KEM 768 from [FIPS 203].
* [`Algorithm::MlKem1024`]\: ML-KEM 1024 from [FIPS 203].
* [`Algorithm::X25519MlKem768Draft00`]\: Hybrid x25519 - ML-KEM 768 [draft kem for hpke](https://www.ietf.org/archive/id/draft-westerbaan-cfrg-hpke-xyber768d00-00.html).
* [`Algorithm::XWingKemDraft06`]\: Hybrid x25519 - ML-KEM 768 [draft xwing kem for hpke](https://www.ietf.org/archive/id/draft-connolly-cfrg-xwing-kem-06.html).

```Rust
use libcrux_kem::*;

let mut rng = rand::rngs::OsRng;
let (sk_a, pk_a) = key_gen(Algorithm::MlKem768, &mut rng).unwrap();
let received_pk = pk_a.encode();

let pk = PublicKey::decode(Algorithm::MlKem768, &received_pk).unwrap();
let (ss_b, ct_b) = pk.encapsulate(&mut rng).unwrap();
let received_ct = ct_b.encode();

let ct_a = Ct::decode(Algorithm::MlKem768, &received_ct).unwrap();
let ss_a = ct_a.decapsulate(&sk_a).unwrap();
assert_eq!(ss_b.encode(), ss_a.encode());
```

[FIPS 203]: https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.203.ipd.pdf
[verified]: ../.assets/verified-brightgreen.svg
[pre-verification]: ../.assets/pre_verification-orange.svg
