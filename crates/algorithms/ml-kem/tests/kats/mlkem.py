# This file is:
# https://github.com/bwesterb/draft-schwabe-cfrg-kyber/blob/a03ab13c241a1a0b6adc676d27be79843b03abc8/kyber.py
# with changes made to match the FIPS-203 draft as well as formatting changes
# made by the black formatter.

# WARNING This is a specification of Kyber; not a production ready
# implementation. It is slow and does not run in constant time.

# Requires the CryptoDome for SHAKE. To install, run
#
#   pip install pycryptodome pytest
from Crypto.Hash import SHAKE128, SHAKE256

import io
import hashlib
import functools
import collections

from math import floor

q = 3329
nBits = 8
zeta = 17
eta2 = 2

n = 2**nBits
inv2 = (q + 1) // 2  # inverse of 2

params = collections.namedtuple("params", ("k", "du", "dv", "eta1"))

params512 = params(k=2, du=10, dv=4, eta1=3)
params768 = params(k=3, du=10, dv=4, eta1=2)
params1024 = params(k=4, du=11, dv=5, eta1=2)


def smod(x):
    r = x % q
    if r > (q - 1) // 2:
        r -= q
    return r


# Rounds to nearest integer with ties going up
def Round(x):
    return int(floor(x + 0.5))


def Compress(x, d):
    return Round((2**d / q) * x) % (2**d)


def Decompress(y, d):
    assert 0 <= y and y <= 2**d
    return Round((q / 2**d) * y)


def BitsToWords(bs, w):
    assert len(bs) % w == 0
    return [sum(bs[i + j] * 2**j for j in range(w)) for i in range(0, len(bs), w)]


def WordsToBits(bs, w):
    return sum([[(b >> i) % 2 for i in range(w)] for b in bs], [])


def Encode(a, w):
    return bytes(BitsToWords(WordsToBits(a, w), 8))


def Decode(a, w):
    return BitsToWords(WordsToBits(a, 8), w)


def brv(x):
    """Reverses a 7-bit number"""
    return int("".join(reversed(bin(x)[2:].zfill(nBits - 1))), 2)


class Poly:
    def __init__(self, cs=None):
        self.cs = (0,) * n if cs is None else tuple(cs)
        assert len(self.cs) == n

    def __add__(self, other):
        return Poly((a + b) % q for a, b in zip(self.cs, other.cs))

    def __neg__(self):
        return Poly(q - a for a in self.cs)

    def __sub__(self, other):
        return self + -other

    def __str__(self):
        return f"Poly({self.cs}"

    def __eq__(self, other):
        return self.cs == other.cs

    def NTT(self):
        cs = list(self.cs)
        layer = n // 2
        zi = 0
        while layer >= 2:
            for offset in range(0, n - layer, 2 * layer):
                zi += 1
                z = pow(zeta, brv(zi), q)

                for j in range(offset, offset + layer):
                    t = (z * cs[j + layer]) % q
                    cs[j + layer] = (cs[j] - t) % q
                    cs[j] = (cs[j] + t) % q
            layer //= 2
        return Poly(cs)

    def RefNTT(self):
        # Slower, but simpler, version of the NTT.
        cs = [0] * n
        for i in range(0, n, 2):
            for j in range(n // 2):
                z = pow(zeta, (2 * brv(i // 2) + 1) * j, q)
                cs[i] = (cs[i] + self.cs[2 * j] * z) % q
                cs[i + 1] = (cs[i + 1] + self.cs[2 * j + 1] * z) % q
        return Poly(cs)

    def InvNTT(self):
        cs = list(self.cs)
        layer = 2
        zi = n // 2
        while layer < n:
            for offset in range(0, n - layer, 2 * layer):
                zi -= 1
                z = pow(zeta, brv(zi), q)

                for j in range(offset, offset + layer):
                    t = (cs[j + layer] - cs[j]) % q
                    cs[j] = (inv2 * (cs[j] + cs[j + layer])) % q
                    cs[j + layer] = (inv2 * z * t) % q
            layer *= 2
        return Poly(cs)

    def MulNTT(self, other):
        """Computes self o other, the multiplication of self and other
        in the NTT domain."""
        cs = [None] * n
        for i in range(0, n, 2):
            a1 = self.cs[i]
            a2 = self.cs[i + 1]
            b1 = other.cs[i]
            b2 = other.cs[i + 1]
            z = pow(zeta, 2 * brv(i // 2) + 1, q)
            cs[i] = (a1 * b1 + z * a2 * b2) % q
            cs[i + 1] = (a2 * b1 + a1 * b2) % q
        return Poly(cs)

    def Compress(self, d):
        return Poly(Compress(c, d) for c in self.cs)

    def Decompress(self, d):
        return Poly(Decompress(c, d) for c in self.cs)

    def Encode(self, d):
        return Encode(self.cs, d)


def sampleUniform(stream):
    cs = []
    while True:
        b = stream.read(3)
        d1 = b[0] + 256 * (b[1] % 16)
        d2 = (b[1] >> 4) + 16 * b[2]
        assert d1 + 2**12 * d2 == b[0] + 2**8 * b[1] + 2**16 * b[2]
        for d in [d1, d2]:
            if d >= q:
                continue
            cs.append(d)
            if len(cs) == n:
                return Poly(cs)


def CBD(a, eta):
    assert len(a) == 64 * eta
    b = WordsToBits(a, 8)
    cs = []
    for i in range(n):
        cs.append((sum(b[:eta]) - sum(b[eta : 2 * eta])) % q)
        b = b[2 * eta :]
    return Poly(cs)


def XOF(seed, j, i):
    h = SHAKE128.new()
    h.update(seed + bytes([j, i]))
    return h


def PRF1(seed, nonce):
    assert len(seed) == 32
    h = SHAKE256.new()
    h.update(seed + bytes([nonce]))
    return h


def PRF2(seed, msg):
    assert len(seed) == 32
    h = SHAKE256.new()
    h.update(seed + msg)
    return h.read(32)


def G(seed):
    h = hashlib.sha3_512(seed).digest()
    return h[:32], h[32:]


def H(msg):
    return hashlib.sha3_256(msg).digest()


class Vec:
    def __init__(self, ps):
        self.ps = tuple(ps)

    def NTT(self):
        return Vec(p.NTT() for p in self.ps)

    def InvNTT(self):
        return Vec(p.InvNTT() for p in self.ps)

    def DotNTT(self, other):
        """Computes the dot product <self, other> in NTT domain."""
        return sum((a.MulNTT(b) for a, b in zip(self.ps, other.ps)), Poly())

    def __add__(self, other):
        return Vec(a + b for a, b in zip(self.ps, other.ps))

    def Compress(self, d):
        return Vec(p.Compress(d) for p in self.ps)

    def Decompress(self, d):
        return Vec(p.Decompress(d) for p in self.ps)

    def Encode(self, d):
        return Encode(sum((p.cs for p in self.ps), ()), d)

    def __eq__(self, other):
        return self.ps == other.ps


def EncodeVec(vec, w):
    return Encode(sum([p.cs for p in vec.ps], ()), w)


def DecodeVec(bs, k, w):
    cs = Decode(bs, w)
    return Vec(Poly(cs[n * i : n * (i + 1)]) for i in range(k))


def DecodePoly(bs, w):
    return Poly(Decode(bs, w))


class Matrix:
    def __init__(self, cs):
        """Samples the matrix uniformly from seed rho"""
        self.cs = tuple(tuple(row) for row in cs)

    def MulNTT(self, vec):
        """Computes matrix multiplication A*vec in the NTT domain."""
        return Vec(Vec(row).DotNTT(vec) for row in self.cs)

    def T(self):
        """Returns transpose of matrix"""
        k = len(self.cs)
        return Matrix((self.cs[j][i] for j in range(k)) for i in range(k))


def sampleMatrix(rho, k):
    return Matrix([[sampleUniform(XOF(rho, j, i)) for j in range(k)] for i in range(k)])


def sampleNoise(sigma, eta, offset, k):
    return Vec(CBD(PRF1(sigma, i + offset).read(64 * eta), eta) for i in range(k))


def constantTimeSelectOnEquality(a, b, ifEq, ifNeq):
    # WARNING! In production code this must be done in a
    # data-independent constant-time manner, which this implementation
    # is not. In fact, many more lines of code in this
    # file are not constant-time.
    return ifEq if a == b else ifNeq


def InnerKeyGen(seed, params, ipd):
    assert len(seed) == 32
    if ipd:
            rho, sigma = G(seed)
    else:
            rho, sigma = G(seed + bytes([params.k]))
    A = sampleMatrix(rho, params.k)
    s = sampleNoise(sigma, params.eta1, 0, params.k)
    e = sampleNoise(sigma, params.eta1, params.k, params.k)
    sHat = s.NTT()
    eHat = e.NTT()
    tHat = A.MulNTT(sHat) + eHat
    pk = EncodeVec(tHat, 12) + rho
    sk = EncodeVec(sHat, 12)
    return (pk, sk)


def InnerEnc(pk, msg, seed, params):
    assert len(msg) == 32
    tHat = DecodeVec(pk[:-32], params.k, 12)
    rho = pk[-32:]
    A = sampleMatrix(rho, params.k)
    r = sampleNoise(seed, params.eta1, 0, params.k)
    e1 = sampleNoise(seed, eta2, params.k, params.k)
    e2 = sampleNoise(seed, eta2, 2 * params.k, 1).ps[0]
    rHat = r.NTT()
    u = A.T().MulNTT(rHat).InvNTT() + e1
    m = Poly(Decode(msg, 1)).Decompress(1)
    v = tHat.DotNTT(rHat).InvNTT() + e2 + m
    c1 = u.Compress(params.du).Encode(params.du)
    c2 = v.Compress(params.dv).Encode(params.dv)
    return c1 + c2


def InnerDec(sk, ct, params):
    split = params.du * params.k * n // 8
    c1, c2 = ct[:split], ct[split:]
    u = DecodeVec(c1, params.k, params.du).Decompress(params.du)
    v = DecodePoly(c2, params.dv).Decompress(params.dv)
    sHat = DecodeVec(sk, params.k, 12)
    return (v - sHat.DotNTT(u.NTT()).InvNTT()).Compress(1).Encode(1)


def KeyGen(seed, params, ipd = False):
    assert len(seed) == 64
    z = seed[32:]
    pk, sk2 = InnerKeyGen(seed[:32], params, ipd)
    h = H(pk)
    return (pk, sk2 + pk + h + z)


def Enc(pk, seed, params):
    assert len(seed) == 32

    m = seed
    K, r = G(m + H(pk))
    ct = InnerEnc(pk, m, r, params)
    return (ct, K)


def Dec(sk, ct, params):
    sk2 = sk[: 12 * params.k * n // 8]
    pk = sk[12 * params.k * n // 8 : 24 * params.k * n // 8 + 32]
    h = sk[24 * params.k * n // 8 + 32 : 24 * params.k * n // 8 + 64]
    z = sk[24 * params.k * n // 8 + 64 : 24 * params.k * n // 8 + 96]
    m2 = InnerDec(sk, ct, params)
    K2, r2 = G(m2 + h)
    ct2 = InnerEnc(pk, m2, r2, params)
    return constantTimeSelectOnEquality(
        ct2,
        ct,
        K2,  # if ct == ct2
        PRF2(z, ct),  # if ct != ct2
    )
