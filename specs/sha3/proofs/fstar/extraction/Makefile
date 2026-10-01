FSTAR_INCLUDE_DIRS_EXTRA += ../../../../../crates/algorithms/sha3/proofs/fstar/equivalence

# Verify the four root spec modules; the implementation (Libcrux_sha3.*) is
# verified from crates/algorithms/sha3.
ROOTS = Hacspec_sha3.fst \
        Hacspec_sha3.Keccak_f.fst \
        Hacspec_sha3.Sha3.fst \
        Hacspec_sha3.Sponge.fst

SLOW_MODULES += 

ADMIT_MODULES += 

OTHERFLAGS += 

include $(shell git rev-parse --show-toplevel)/fstar-helpers/Makefile.base
