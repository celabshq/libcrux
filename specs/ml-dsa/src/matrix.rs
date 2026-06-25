use crate::createi;
/// Matrix–vector operations in the NTT domain — FIPS 204.
///
/// All inputs and outputs are in NTT representation.
use crate::parameters::{Polynomial, ZERO_POLY};
use crate::polynomial::{poly_add, poly_pointwise_mul};

/// Multiply a k×ℓ matrix (in NTT domain) by an ℓ-vector (in NTT domain).
///
/// Returns a k-vector where result[i] = Σ_j A_hat[i][j] ∘ v_hat[j].
pub(crate) fn matrix_vector_ntt<const K: usize, const L: usize>(
    a_hat: &[[Polynomial; L]; K],
    v_hat: &[Polynomial; L],
) -> [Polynomial; K] {
    createi(|i| {
        let mut acc = ZERO_POLY;
        for j in 0..L {
            let product = poly_pointwise_mul(&a_hat[i][j], &v_hat[j]);
            acc = poly_add(&acc, &product);
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ntt::{intt, ntt};

    #[test]
    fn matrix_vector_identity() {
        // Identity matrix times a vector should give the same vector
        const K: usize = 2;
        const L: usize = 2;
        let mut identity = [[ZERO_POLY; L]; K];
        // NTT of the identity polynomial [1, 0, 0, ...] is the NTT of 1
        let one_ntt = ntt({
            let mut p = ZERO_POLY;
            p[0] = 1;
            p
        });
        identity[0][0] = one_ntt;
        identity[1][1] = one_ntt;

        let v = [
            ntt({
                let mut p = ZERO_POLY;
                p[0] = 42;
                p
            }),
            ntt({
                let mut p = ZERO_POLY;
                p[0] = 7;
                p
            }),
        ];

        let result = matrix_vector_ntt::<K, L>(&identity, &v);
        let r0 = intt(result[0]);
        let r1 = intt(result[1]);
        assert_eq!(r0[0], 42);
        assert_eq!(r1[0], 7);
    }
}
