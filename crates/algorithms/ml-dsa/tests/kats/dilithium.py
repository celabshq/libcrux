# The files in this directory have been taken from:
#
# https://github.com/GiacomoPope/dilithium-py/pull/1
#
# with some modifications.

import os

from polynomials import *
from modules import *
from shake_wrapper import Shake128, Shake256
from utils import *
from ntt_helper import NTTHelperDilithium

try:
    from aes256_ctr_drbg import AES256_CTR_DRBG
except ImportError as e:
    print("Error importing AES256 CTR DRBG. Have you tried installing requirements?")
    print(f"ImportError: {e}\n")
    print("Dilithium will work perfectly fine with system randomness")

DEFAULT_PARAMETERS = {
    "dilithium2": {
        "n": 256,
        "q": 8380417,
        "d": 13,
        "k": 4,
        "l": 4,
        "eta": 2,
        "eta_bound": 15,
        "tau": 39,
        "omega": 80,
        "gamma_1": 131072,  # 2^17
        "gamma_2": 95232,  # (q-1)/88
        "ctildebytes": 32,
    },
    "dilithium3": {
        "n": 256,
        "q": 8380417,
        "d": 13,
        "k": 6,
        "l": 5,
        "eta": 4,
        "eta_bound": 9,
        "tau": 49,
        "omega": 55,
        "gamma_1": 524288,  # 2^19
        "gamma_2": 261888,  # (q-1)/88
        "ctildebytes": 48,
    },
    "dilithium5": {
        "n": 256,
        "q": 8380417,
        "d": 13,
        "k": 8,
        "l": 7,
        "eta": 2,
        "eta_bound": 15,
        "tau": 60,
        "omega": 75,
        "gamma_1": 524288,  # 2^19
        "gamma_2": 261888,  # (q-1)/88
        "ctildebytes": 64,
    },
}


class Dilithium:
    def __init__(self, parameter_set):
        self.n = parameter_set["n"]
        self.q = parameter_set["q"]
        self.d = parameter_set["d"]
        self.k = parameter_set["k"]
        self.l = parameter_set["l"]
        self.eta = parameter_set["eta"]
        self.eta_bound = parameter_set["eta_bound"]
        self.tau = parameter_set["tau"]
        self.omega = parameter_set["omega"]
        self.gamma_1 = parameter_set["gamma_1"]
        self.gamma_2 = parameter_set["gamma_2"]
        self.beta = self.tau * self.eta

        self.ctildebytes = parameter_set["ctildebytes"]

        self.R = PolynomialRing(self.q, self.n, ntt_helper=NTTHelperDilithium)
        self.M = Module(self.R)

        self.drbg = None
        self.random_bytes = os.urandom

    """
    The following two methods allow us to use deterministic
    randomness throughout all of Dilithium. This is helpful
    for the KAT tests more than anything!
    """

    def set_drbg_seed(self, seed):
        """
        Setting the seed switches the entropy source
        from os.urandom to AES256 CTR DRBG

        Note: requires pycryptodome for AES impl.
        (Seemed overkill to code my own AES for Kyber)
        """
        self.drbg = AES256_CTR_DRBG(seed)
        self.random_bytes = self.drbg.random_bytes

    def reseed_drbg(self, seed):
        """
        Reseeds the DRBG, errors if a DRBG is not set.

        Note: requires pycryptodome for AES impl.
        (Seemed overkill to code my own AES for Kyber)
        """
        if self.drbg is None:
            raise Warning(
                f"Cannot reseed DRBG without first initialising. Try using `set_drbg_seed`"
            )
        else:
            self.drbg.reseed(seed)

    """
    H() uses Shake256 to hash data to 32 and 64 bytes in a
    few places in the code
    """

    @staticmethod
    def _h(input_bytes, length):
        """
        H: B^*  -> B^*
        """
        return Shake256.digest(input_bytes, length)

    """
    Figure 3 (Supporting algorithms for Dilithium)
    `_make_hint/_use_hint` is applied to matricies and `_make_hint_poly/_use_hint_poly`
    applies to the polynomials, which are elements of the matricies.

    `_make_hint_poly/_use_hint_poly` uses the util functions `use_hint/make_hint`
    which works on field elements (see utils.py)

        https://pq-crystals.org/dilithium/data/dilithium-specification-round3-20210208.pdf
    """

    def _make_hint(self, v1, v2, alpha):
        matrix = [
            [
                self._make_hint_poly(p1, p2, alpha)
                for p1, p2 in zip(v1.rows[i], v2.rows[i])
            ]
            for i in range(v1.m)
        ]
        return self.M(matrix)

    def _use_hint(self, v1, v2, alpha):
        matrix = [
            [
                self._use_hint_poly(p1, p2, alpha)
                for p1, p2 in zip(v1.rows[i], v2.rows[i])
            ]
            for i in range(v1.m)
        ]
        return self.M(matrix)

    def _make_hint_poly(self, p1, p2, alpha):
        coeffs = [make_hint(r, z, alpha, self.q) for r, z in zip(p1.coeffs, p2.coeffs)]
        return self.R(coeffs)

    def _use_hint_poly(self, p1, p2, alpha, is_ntt=False):
        coeffs = [use_hint(h, r, alpha, self.q) for h, r in zip(p1.coeffs, p2.coeffs)]
        return self.R(coeffs)

    @staticmethod
    def _sum_hint(hint):
        """
        Helper function to count the number of coeffs == 1
        in all the polynomials of a matrix
        """
        return sum(c for row in hint.rows for p in row for c in p)

    def _sample_in_ball(self, seed, is_ntt=False):
        """
        Figure 2 (Sample in Ball)
            https://pq-crystals.org/dilithium/data/dilithium-specification-round3-20210208.pdf

        Create a random 256-element array with τ ±1’s and (256 − τ) 0′s using
        the input seed ρ (and an SHAKE256) to generate the randomness needed
        """

        def rejection_sample(i, xof):
            """
            Sample random bytes from `xof_bytes` and
            interpret them as integers in {0, ..., 255}

            Rejects values until a value j <= i is found
            """
            while True:
                j = xof.read(1)
                j = int.from_bytes(j, "little")
                if j <= i:
                    return j

        # Initialise the XOF
        Shake256.absorb(seed)

        # Set the first 8 bytes for the sign, and leave the rest for
        # sampling.
        sign_bytes = Shake256.read(8)
        sign_int = int.from_bytes(sign_bytes, "little")

        # Set the list of coeffs to be 0
        coeffs = [0 for _ in range(self.n)]

        # Now set tau values of coeffs to be ±1
        for i in range(256 - self.tau, self.n):
            j = rejection_sample(i, Shake256)
            coeffs[i] = coeffs[j]
            coeffs[j] = 1 - 2 * (sign_int & 1)
            sign_int >>= 1

        return self.R(coeffs, is_ntt=is_ntt)

    def _sample_error_polynomial(self, rho_prime, i, is_ntt=False):
        def rejection_sample(xof):
            """
            Sample a random byte from `xof_bytes` and
            interpret it as two integers in {0, ..., 2η}
            by considering the top and bottom four bits

            Rejects values until a value j < 2η is found
            """
            while True:
                js = []

                # Consider two values for each byte (top and bottom four bits)
                j = xof.read(1)
                j = int.from_bytes(j, "little")
                j0 = j & 0x0F
                j1 = j >> 4

                # rejection sample
                if j0 < self.eta_bound:
                    if self.eta == 2:
                        j0 %= 5
                    js.append(self.eta - j0)

                if j1 < self.eta_bound:
                    if self.eta == 2:
                        j1 %= 5
                    js.append(self.eta - j1)

                if js:
                    return js

        # Initialise the XOF
        seed = rho_prime + int.to_bytes(i, 2, "little")
        Shake256.absorb(seed)

        # Sample bytes for all n coeffs
        # TODO: make this better.
        coeffs = []
        while len(coeffs) < self.n:
            js = rejection_sample(Shake256)
            coeffs += js

        # Remove the last byte if we ended up overfilling
        if len(coeffs) > self.n:
            coeffs = coeffs[: self.n]

        return self.R(coeffs, is_ntt=is_ntt)

    def _sample_matrix_polynomial(self, rho, i, j, is_ntt=False):
        def rejection_sample(xof):
            """
            Sample three random bytes from `xof` and
            interpret them as integers in {0, ..., 2^23 - 1}

            Rejects values until a value j < q is found
            """
            while True:
                j_bytes = xof.read(3)
                j = int.from_bytes(j_bytes, "little")
                j &= 0x7FFFFF
                if j < self.q:
                    return j

        # Initialise the XOF
        seed = rho + bytes([j, i])
        Shake128.absorb(seed)
        coeffs = [rejection_sample(Shake128) for _ in range(self.n)]

        return self.R(coeffs, is_ntt=is_ntt)

    def _sample_mask_polynomial(self, rho_prime, i, kappa, is_ntt=False):
        if self.gamma_1 == (1 << 17):
            bit_count = 18
            total_bytes = 576  # (256 * 18) / 8
        else:
            bit_count = 20
            total_bytes = 640  # (256 * 20) / 8

        # Initialise the XOF
        seed = rho_prime + int.to_bytes(kappa + i, 2, "little")
        xof_bytes = Shake256.digest(seed, total_bytes)
        r = int.from_bytes(xof_bytes, "little")
        mask = (1 << bit_count) - 1
        coeffs = [self.gamma_1 - ((r >> bit_count * i) & mask) for i in range(self.n)]

        return self.R(coeffs, is_ntt=is_ntt)

    def _expandA(self, rho, is_ntt=False):
        """
        Helper function which generates a element of size
        k x l from a seed `rho`.

        When `transpose` is set to True, the matrix A is
        built as the transpose.
        """
        matrix = [
            [
                self._sample_matrix_polynomial(rho, i, j, is_ntt=is_ntt)
                for j in range(self.l)
            ]
            for i in range(self.k)
        ]
        return self.M(matrix)

    def _expandS(self, rho_prime, is_ntt=False):
        s1_elements = [
            self._sample_error_polynomial(rho_prime, i, is_ntt=is_ntt)
            for i in range(self.l)
        ]
        s2_elements = [
            self._sample_error_polynomial(rho_prime, i, is_ntt=is_ntt)
            for i in range(self.l, self.l + self.k)
        ]

        s1 = self.M(s1_elements).transpose()
        s2 = self.M(s2_elements).transpose()
        return s1, s2

    def _expandMask(self, rho_prime, kappa, is_ntt=False):
        elements = [
            self._sample_mask_polynomial(rho_prime, i, kappa, is_ntt=is_ntt)
            for i in range(self.l)
        ]
        return self.M(elements).transpose()

    @staticmethod
    def _pack_pk(rho, t1):
        return rho + t1.bit_pack_t1()

    def _pack_sk(self, rho, K, tr, s1, s2, t0):
        s1_bytes = s1.bit_pack_s(self.eta)
        s2_bytes = s2.bit_pack_s(self.eta)
        t0_bytes = t0.bit_pack_t0()
        return rho + K + tr + s1_bytes + s2_bytes + t0_bytes

    def _pack_h(self, h):
        non_zero_positions = [
            [i for i, c in enumerate(poly.coeffs) if c == 1]
            for row in h.rows
            for poly in row
        ]
        packed = []
        offsets = []
        for positions in non_zero_positions:
            packed.extend(positions)
            offsets.append(len(packed))

        padding_len = self.omega - offsets[-1]
        packed.extend([0 for _ in range(padding_len)])
        return bytes(packed + offsets)

    def _pack_sig(self, c_tilde, z, h):
        return c_tilde + z.bit_pack_z(self.gamma_1) + self._pack_h(h)

    def _unpack_pk(self, pk_bytes):
        rho, t1_bytes = pk_bytes[:32], pk_bytes[32:]
        t1 = self.M.bit_unpack_t1(t1_bytes, self.k, 1)
        return rho, t1

    def _unpack_sk(self, sk_bytes):
        if self.eta == 2:
            s_bytes = 96
        else:
            s_bytes = 128
        s1_len = s_bytes * self.l
        s2_len = s_bytes * self.k
        t0_len = 416 * self.k
        if len(sk_bytes) != 2 * 32 + 64 + s1_len + s2_len + t0_len:
            raise ValueError("SK packed bytes is of the wrong length")

        # Split bytes between seeds and vectors
        sk_seed_bytes, sk_vec_bytes = sk_bytes[:128], sk_bytes[128:]

        # Unpack seed bytes
        rho, K, tr = sk_seed_bytes[:32], sk_seed_bytes[32:64], sk_seed_bytes[64:128]

        # Unpack vector bytes
        s1_bytes = sk_vec_bytes[:s1_len]
        s2_bytes = sk_vec_bytes[s1_len : s1_len + s2_len]
        t0_bytes = sk_vec_bytes[-t0_len:]

        # Unpack bytes to vectors
        s1 = self.M.bit_unpack_s(s1_bytes, self.l, 1, self.eta)
        s2 = self.M.bit_unpack_s(s2_bytes, self.k, 1, self.eta)
        t0 = self.M.bit_unpack_t0(t0_bytes, self.k, 1)

        return rho, K, tr, s1, s2, t0

    def _unpack_h(self, h_bytes):
        offsets = [0] + list(h_bytes[-self.k :])
        non_zero_positions = [
            list(h_bytes[offsets[i] : offsets[i + 1]]) for i in range(self.k)
        ]

        matrix = []
        for poly_non_zero in non_zero_positions:
            coeffs = [0 for _ in range(self.n)]
            for non_zero in poly_non_zero:
                coeffs[non_zero] = 1
            matrix.append([self.R(coeffs)])
        return self.M(matrix)

    def _unpack_sig(self, sig_bytes):
        c_tilde = sig_bytes[: self.ctildebytes]
        z_bytes = sig_bytes[self.ctildebytes : -(self.k + self.omega)]
        h_bytes = sig_bytes[-(self.k + self.omega) :]

        z = self.M.bit_unpack_z(z_bytes, self.l, 1, self.gamma_1)
        h = self._unpack_h(h_bytes)
        return c_tilde, z, h

    def keygen(self):
        # Random seed (with domain separation)
        zeta = self.random_bytes(32)
        domain_separated_zeta = zeta + self.k.to_bytes(1, "little") + self.l.to_bytes(1, "little")
        self.keygen_seed = zeta
        # Expand with an XOF (SHAKE256)
        seed_bytes = self._h(domain_separated_zeta, 128)

        # Split bytes into suitable chunks
        rho, rho_prime, K = seed_bytes[:32], seed_bytes[32:96], seed_bytes[96:]

        # Generate matrix A ∈ R^(kxl)
        A = self._expandA(rho, is_ntt=True)

        # Generate the error vectors s1 ∈ R^l, s2 ∈ R^k
        s1, s2 = self._expandS(rho_prime)
        s1_hat = s1.copy_to_ntt()

        # Matrix multiplication
        t = (A @ s1_hat).from_ntt() + s2

        t1, t0 = t.power_2_round(self.d)

        # Pack up the bytes
        pk = self._pack_pk(rho, t1)
        tr = self._h(pk, 64)

        sk = self._pack_sk(rho, K, tr, s1, s2, t0)
        return pk, sk

    def sign_pre_hashed_shake128(self, sk_bytes, m, ctx=b"", rnd=None):
        shake128_oid = b'\x06\x09\x60\x86\x48\x01\x65\x03\x04\x02\x0B'
        m_hashed = Shake128.digest(m, 32)
        m_prime = b'\x01' + len(ctx).to_bytes(1, "little") + ctx + shake128_oid + m_hashed

        return self.sign_internal(sk_bytes, m_prime, rnd)

    def sign(self, sk_bytes, m, ctx=b"", rnd=None):
        m_prime = b'\x00' + len(ctx).to_bytes(1, "little") + ctx + m
        return self.sign_internal(sk_bytes, m_prime, rnd)

    def sign_internal(self, sk_bytes, m, rnd):
        # unpack the secret key
        rho, K, tr, s1, s2, t0 = self._unpack_sk(sk_bytes)

        # Generate matrix A ∈ R^(kxl)
        A = self._expandA(rho, is_ntt=True)

        # Set seeds and nonce (kappa)
        mu = self._h(tr + m, 64)
        kappa = 0
        rnd = self.random_bytes(32)
        self.signing_randomness = rnd
        rho_prime = self._h(K + rnd + mu, 64)

        # Precompute NTT representation
        s1.to_ntt()
        s2.to_ntt()
        t0.to_ntt()

        alpha = self.gamma_2 << 1
        while True:
            y = self._expandMask(rho_prime, kappa)
            y_hat = y.copy_to_ntt()

            # increment the nonce
            kappa += self.l

            w = (A @ y_hat).from_ntt()

            # Extract out both the high and low bits
            w1, w0 = w.decompose(alpha)

            # Create challenge polynomial
            w1_bytes = w1.bit_pack_w(self.gamma_2)
            c_tilde = self._h(mu + w1_bytes, self.ctildebytes)
            c = self._sample_in_ball(c_tilde)  # SEEDBYTES

            # Store c in NTT form
            c.to_ntt()

            z = y + s1.scale(c).from_ntt()
            if z.check_norm_bound(self.gamma_1 - self.beta):
                continue

            w0_minus_cs2 = w0 - s2.scale(c).from_ntt()
            if w0_minus_cs2.check_norm_bound(self.gamma_2 - self.beta):
                continue

            c_t0 = t0.scale(c).from_ntt()
            # c_t0.reduce_coefficents()

            if c_t0.check_norm_bound(self.gamma_2):
                continue

            w0_minus_cs2_plus_ct0 = w0_minus_cs2 + c_t0

            h = self._make_hint(w0_minus_cs2_plus_ct0, w1, alpha)

            if self._sum_hint(h) > self.omega:
                continue

            return self._pack_sig(c_tilde, z, h)

    def verify_pre_hashed(self, pk_bytes, m, sig_bytes, ctx=b""):
        shake128_oid = b'\x06\x09\x60\x86\x48\x01\x65\x03\x04\x02\x0B'
        m_hashed = Shake128.digest(m, 256)
        m_prime = b'\x01' + len(ctx).to_bytes(1, "little") + ctx + shake128_oid + m_hashed

        return self.verify_internal(sk_bytes, m_prime, rnd)

    def verify(self, pk_bytes, m, sig_bytes, ctx=b""):
        m_prime = b'\x00' + len(ctx).to_bytes(1, "little") + ctx + m
        return self.verify_internal(sk_bytes, m_prime, rnd)

    def verify_internal(self, pk_bytes, m, sig_bytes):
        rho, t1 = self._unpack_pk(pk_bytes)
        c_tilde, z, h = self._unpack_sig(sig_bytes)

        if self._sum_hint(h) > self.omega:
            return False

        if z.check_norm_bound(self.gamma_1 - self.beta):
            return False

        A = self._expandA(rho, is_ntt=True)

        tr = self._h(pk_bytes, 64)  # TRBYTES
        mu = self._h(tr + m, 64)
        c = self._sample_in_ball(c_tilde)

        # Convert to NTT for computation
        c.to_ntt()
        z.to_ntt()

        t1 = t1.scale(1 << self.d)
        t1.to_ntt()

        Az_minus_ct1 = (A @ z) - t1.scale(c)
        Az_minus_ct1.from_ntt()

        w_prime = self._use_hint(h, Az_minus_ct1, 2 * self.gamma_2)
        w_prime_bytes = w_prime.bit_pack_w(self.gamma_2)

        return c_tilde == self._h(mu + w_prime_bytes, self.ctildebytes)


Dilithium2 = Dilithium(DEFAULT_PARAMETERS["dilithium2"])
Dilithium3 = Dilithium(DEFAULT_PARAMETERS["dilithium3"])
Dilithium5 = Dilithium(DEFAULT_PARAMETERS["dilithium5"])
