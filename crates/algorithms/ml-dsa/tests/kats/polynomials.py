import random
from copy import deepcopy
from utils import *


class PolynomialRing:
    """
    Initialise the polynomial ring:

        R = GF(q) / (X^n + 1)
    """

    def __init__(self, q, n, ntt_helper=None):
        self.q = q
        self.n = n
        self.element = PolynomialRing.Polynomial
        self.ntt_helper = ntt_helper

    def gen(self, is_ntt=False):
        return self([0, 1], is_ntt=is_ntt)

    def random_element(self, is_ntt=False):
        coefficients = [random.randint(0, self.q - 1) for _ in range(self.n)]
        return self(coefficients, is_ntt=is_ntt)

    def bit_unpack(self, input_bytes, n_bits):
        if len(input_bytes) % 8 * n_bits != 0:
            raise ValueError(
                "Input bytes do not have a length compatible with the bit length"
            )

        r = int.from_bytes(input_bytes, "little")
        mask = (1 << n_bits) - 1
        return [(r >> n_bits * i) & mask for i in range(self.n)]

    def bit_unpack_t0(self, input_bytes, is_ntt=False):
        altered_coeffs = self.bit_unpack(input_bytes, 13)
        coefficients = [(1 << 12) - c for c in altered_coeffs]
        return self(coefficients, is_ntt=False)

    def bit_unpack_t1(self, input_bytes, is_ntt=False):
        coefficients = self.bit_unpack(input_bytes, 10)
        return self(coefficients, is_ntt=False)

    def bit_unpack_s(self, input_bytes, eta, is_ntt=False):
        # Level 2 and 5 parameter set
        if eta == 2:
            altered_coeffs = self.bit_unpack(input_bytes, 3)
        # Level 3 parameter set
        elif eta == 4:
            altered_coeffs = self.bit_unpack(input_bytes, 4)
        else:
            raise ValueError("Expected eta to be either 2 or 4")
        coefficients = [eta - c for c in altered_coeffs]
        return self(coefficients, is_ntt=False)

    def bit_unpack_w(self, input_bytes, gamma_2, is_ntt=False):
        # Level 2 parameter set
        if gamma_2 == 95232:
            coefficients = self.bit_unpack(input_bytes, 6)
        # Level 3 and 5 parameter set
        elif gamma_2 == 261888:
            coefficients = self.bit_unpack(input_bytes, 4)
        else:
            raise ValueError("Expected gamma_2 to be either (q-1)/88 or (q-1)/32")
        return self(coefficients, is_ntt=False)

    def bit_unpack_z(self, input_bytes, gamma_1, is_ntt=False):
        # Level 2 parameter set
        if gamma_1 == (1 << 17):
            altered_coeffs = self.bit_unpack(input_bytes, 18)
        # Level 3 and 5 parameter set
        elif gamma_1 == (1 << 19):
            altered_coeffs = self.bit_unpack(input_bytes, 20)
        else:
            raise ValueError("Expected gamma_1 to be either 2^17 or 2^19")
        coefficients = [gamma_1 - c for c in altered_coeffs]
        return self(coefficients, is_ntt=False)

    def __call__(self, coefficients, is_ntt=False):
        if isinstance(coefficients, int):
            return self.element(self, [coefficients], is_ntt)
        if not isinstance(coefficients, list):
            raise TypeError(
                f"Polynomials should be constructed from a list of integers, of length at most d = {self.n}"
            )
        return self.element(self, coefficients, is_ntt)

    def __eq__(self, other):
        return self.n == other.n and self.q == other.q

    def __repr__(self):
        return f"Univariate Polynomial Ring in x over Finite Field of size {self.q} with modulus x^{self.n} + 1"

    class Polynomial:
        def __init__(self, parent, coefficients, is_ntt=False):
            self.parent = parent
            self.coeffs = self.parse_coefficients(coefficients)
            self.is_ntt = is_ntt

        def is_zero(self):
            """
            Return if polynomial is zero: f = 0
            """
            return all(c == 0 for c in self.coeffs)

        def is_constant(self):
            """
            Return if polynomial is constant: f = c
            """
            return all(c == 0 for c in self.coeffs[1:])

        def parse_coefficients(self, coefficients):
            """
            Helper function which right pads with zeros
            to allow polynomial construction as
            f = R([1,1,1])
            """
            l = len(coefficients)
            if l > self.parent.n:
                raise ValueError(
                    f"Coefficients describe polynomial of degree greater than maximum degree {self.parent.n}"
                )
            elif l < self.parent.n:
                coefficients = coefficients + [0 for _ in range(self.parent.n - l)]
            return coefficients

        def reduce_coefficents(self):
            """
            Reduce all coefficents modulo q
            """
            self.coeffs = [c % self.parent.q for c in self.coeffs]
            return self

        def add_mod_q(self, x, y):
            """
            add two coefficents modulo q
            """
            tmp = x + y
            if tmp >= self.parent.q:
                tmp -= self.parent.q
            return tmp

        def sub_mod_q(self, x, y):
            """
            sub two coefficents modulo q
            """
            tmp = x - y
            if tmp < 0:
                tmp += self.parent.q
            return tmp

        def schoolbook_multiplication(self, other):
            """
            Naive implementation of polynomial multiplication
            suitible for all R_q = F_1[X]/(X^n + 1)
            """
            n = self.parent.n
            a = self.coeffs
            b = other.coeffs
            new_coeffs = [0 for _ in range(n)]
            for i in range(n):
                for j in range(0, n - i):
                    new_coeffs[i + j] += a[i] * b[j]
            for j in range(1, n):
                for i in range(n - j, n):
                    new_coeffs[i + j - n] -= a[i] * b[j]
            return [reduce_mod_pm(c, self.parent.q) for c in new_coeffs]

        """
        The next methods rely on the parent
        `PolynomialRing` having a  `ntt_helper` from 
        ntt_helper.py and are used for NTT speediness.
        """

        def to_ntt(self):
            if self.parent.ntt_helper is None:
                raise ValueError(
                    "Can only perform NTT transform when parent element has an NTT Helper"
                )
            return self.parent.ntt_helper.to_ntt(self)

        def copy_to_ntt(self):
            new_poly = self.parent(deepcopy(self.coeffs), is_ntt=self.is_ntt)
            return self.parent.ntt_helper.to_ntt(new_poly)

        def from_ntt(self):
            if self.parent.ntt_helper is None:
                raise ValueError(
                    "Can only perform NTT transform when parent element has an NTT Helper"
                )
            return self.parent.ntt_helper.from_ntt(self)

        def copy_from_ntt(self):
            new_poly = self.parent(deepcopy(self.coeffs), is_ntt=self.is_ntt)
            return self.parent.ntt_helper.from_ntt(new_poly)

        def to_montgomery(self):
            """
            Multiply every element by 2^32 mod q

            Only implemented (currently) for n = 256
            """
            if self.parent.ntt_helper is None:
                raise ValueError(
                    "Can only perform Mont. reduction when parent element has an NTT Helper"
                )
            return self.parent.ntt_helper.to_montgomery(self)

        def from_montgomery(self):
            """
            Divide every element by 2^32 mod q

            Only implemented (currently) for n = 256
            """
            if self.parent.ntt_helper is None:
                raise ValueError(
                    "Can only perform Mont. reduction when parent element has an NTT Helper"
                )
            return self.parent.ntt_helper.from_montgomery(self)

        def ntt_multiplication(self, other):
            """
            Number Theoretic Transform multiplication.
            Only implemented (currently) for n = 256
            """
            if self.parent.ntt_helper is None:
                raise ValueError(
                    "Can only perform ntt reduction when parent element has an NTT Helper"
                )
            if not (self.is_ntt and other.is_ntt):
                raise ValueError(
                    "Can only multiply using NTT if both polynomials are in NTT form"
                )
            # function in ntt_helper.py
            new_coeffs = self.parent.ntt_helper.ntt_coefficient_multiplication(
                self.coeffs, other.coeffs
            )
            return self.parent(new_coeffs, is_ntt=True)

        """
        The following four functions are specific for dilithium, but
        act naturally on polynomials, and so are added as methods here.
        """

        def power_2_round(self, d):
            power_2 = 1 << d
            r1_coeffs = []
            r0_coeffs = []
            for c in self.coeffs:
                r = c % self.parent.q
                r0 = reduce_mod_pm(r, power_2)
                r1_coeffs.append((r - r0) >> d)
                r0_coeffs.append(r0)

            r1_poly = self.parent(r1_coeffs, is_ntt=self.is_ntt)
            r0_poly = self.parent(r0_coeffs, is_ntt=self.is_ntt)

            return r1_poly, r0_poly

        def high_bits(self, alpha, is_ntt=False):
            coeffs = [high_bits(c, alpha, self.parent.q) for c in self.coeffs]
            return self.parent(coeffs, is_ntt=is_ntt)

        def low_bits(self, alpha, is_ntt=False):
            coeffs = [low_bits(c, alpha, self.parent.q) for c in self.coeffs]
            return self.parent(coeffs, is_ntt=is_ntt)

        """
        Compute the high and low bits at the same time
        Not in the pseudocode, but needed for the more
        efficient signing which we implement based on
        section 5.1
        """

        def decompose(self, alpha, is_ntt=False):
            coeff_high = []
            coeff_low = []
            for c in self.coeffs:
                r1, r0 = decompose(c, alpha, self.parent.q)
                coeff_high.append(r1)
                coeff_low.append(r0)
            return self.parent(coeff_high, is_ntt=is_ntt), self.parent(
                coeff_low, is_ntt=is_ntt
            )

        def check_norm_bound(self, bound):
            """
            Returns true if the inf norm of any coeff
            is greater or equal to the bound.
            """
            return any(check_norm_bound(c, bound, self.parent.q) for c in self.coeffs)

        """
        The following bit_pack functions are specific for Dilithium
        but are currently added as methods for the Polynomial class
        as it seemed the most natural way to do this.
        """

        @staticmethod
        def bit_pack(coeffs, n_bits, n_bytes):
            r = 0
            for c in reversed(coeffs):
                r <<= n_bits
                r |= c
            return r.to_bytes(n_bytes, "little")

        def bit_pack_t0(self):
            # 416 = 256 * 13 // 8
            altered_coeffs = [(1 << 12) - c for c in self.coeffs]
            return self.bit_pack(altered_coeffs, 13, 416)

        def bit_pack_t1(self):
            # 320 = 256 * 10 // 8
            return self.bit_pack(self.coeffs, 10, 320)

        def bit_pack_s(self, eta):
            altered_coeffs = [eta - c for c in self.coeffs]
            # Level 2 and 5 parameter set
            if eta == 2:
                return self.bit_pack(altered_coeffs, 3, 96)
            # Level 3 parameter set
            elif eta == 4:
                return self.bit_pack(altered_coeffs, 4, 128)
            else:
                raise ValueError("Expected eta to be either 2 or 4")

        def bit_pack_w(self, gamma_2):
            # Level 2 parameter set
            if gamma_2 == 95232:
                return self.bit_pack(self.coeffs, 6, 192)
            # Level 3 and 5 parameter set
            elif gamma_2 == 261888:
                return self.bit_pack(self.coeffs, 4, 128)
            else:
                raise ValueError("Expected gamma_2 to be either (q-1)/88 or (q-1)/32")

        def bit_pack_z(self, gamma_1):
            altered_coeffs = [gamma_1 - c for c in self.coeffs]
            # Level 2 parameter set
            if gamma_1 == (1 << 17):
                return self.bit_pack(altered_coeffs, 18, 576)
            # Level 3 and 5 parameter set
            elif gamma_1 == (1 << 19):
                return self.bit_pack(altered_coeffs, 20, 640)
            else:
                raise ValueError("Expected gamma_1 to be either 2^17 or 2^19")

        def __neg__(self):
            """
            Returns -f, by negating all coefficients
            """
            neg_coeffs = [(-x % self.parent.q) for x in self.coeffs]
            return self.parent(neg_coeffs, is_ntt=self.is_ntt)

        def __add__(self, other):
            if isinstance(other, PolynomialRing.Polynomial):
                if self.is_ntt ^ other.is_ntt:
                    raise ValueError(
                        f"Both or neither polynomials must be in NTT form before multiplication"
                    )
                new_coeffs = [
                    self.add_mod_q(x, y) for x, y in zip(self.coeffs, other.coeffs)
                ]
            elif isinstance(other, int):
                new_coeffs = self.coeffs.copy()
                new_coeffs[0] = self.add_mod_q(new_coeffs[0], other)
            else:
                raise NotImplementedError(
                    f"Polynomials can only be added to each other"
                )
            return self.parent(new_coeffs, is_ntt=self.is_ntt)

        def __radd__(self, other):
            return self.__add__(other)

        def __iadd__(self, other):
            self = self + other
            return self

        def __sub__(self, other):
            if self.is_ntt ^ other.is_ntt:
                raise ValueError(
                    f"Both or neither polynomials must be in NTT form before multiplication"
                )
            if isinstance(other, PolynomialRing.Polynomial):
                new_coeffs = [
                    self.sub_mod_q(x, y) for x, y in zip(self.coeffs, other.coeffs)
                ]
            elif isinstance(other, int):
                new_coeffs = self.coeffs.copy()
                new_coeffs[0] = self.sub_mod_q(new_coeffs[0], other)
            else:
                raise NotImplementedError(
                    f"Polynomials can only be subracted from each other"
                )
            return self.parent(new_coeffs, is_ntt=self.is_ntt)

        def __rsub__(self, other):
            return self.__sub__(other)

        def __isub__(self, other):
            self = self - other
            return self

        def __mul__(self, other):
            if isinstance(other, PolynomialRing.Polynomial):
                if self.is_ntt and other.is_ntt:
                    return self.ntt_multiplication(other)
                elif self.is_ntt ^ other.is_ntt:
                    raise ValueError(
                        f"Both or neither polynomials must be in NTT form before multiplication"
                    )
                else:
                    new_coeffs = self.schoolbook_multiplication(other)
            elif isinstance(other, int):
                new_coeffs = [(c * other) % self.parent.q for c in self.coeffs]
            else:
                raise NotImplementedError(
                    f"Polynomials can only be multiplied by each other, or scaled by integers"
                )
            return self.parent(new_coeffs, is_ntt=self.is_ntt)

        def __rmul__(self, other):
            return self.__mul__(other)

        def __imul__(self, other):
            self = self * other
            return self

        def __pow__(self, n):
            if not isinstance(n, int):
                raise TypeError(
                    f"Exponentiation of a polynomial must be done using an integer."
                )

            # Deal with negative scalar multiplication
            if n < 0:
                raise ValueError(
                    f"Negative powers are not supported for elements of a Polynomial Ring"
                )
            f = self
            g = self.parent(1, is_ntt=self.is_ntt)
            while n > 0:
                if n % 2 == 1:
                    g = g * f
                f = f * f
                n = n // 2
            return g

        def __eq__(self, other):
            if isinstance(other, PolynomialRing.Polynomial):
                return self.coeffs == other.coeffs and self.is_ntt == other.is_ntt
            elif isinstance(other, int):
                if self.is_constant() and (other % self.parent.q) == self.coeffs[0]:
                    return True
            return False

        def __getitem__(self, idx):
            return self.coeffs[idx]

        def __repr__(self):
            """
            TODO make this look nice when there
            are negative coeffs...
            """
            ntt_info = ""
            if self.is_ntt:
                ntt_info = " (NTT form)"
            if self.is_zero():
                return "0" + ntt_info

            info = []
            for i, c in enumerate(self.coeffs):
                if c != 0:
                    if i == 0:
                        info.append(f"{c}")
                    elif i == 1:
                        if c == 1:
                            info.append("x")
                        else:
                            info.append(f"{c}*x")
                    else:
                        if c == 1:
                            info.append(f"x^{i}")
                        else:
                            info.append(f"{c}*x^{i}")
            return " + ".join(info) + ntt_info

        def __str__(self):
            return self.__repr__()
