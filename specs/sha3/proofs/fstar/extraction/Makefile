FSTAR_INCLUDE_DIRS_EXTRA += ../../../../../crates/algorithms/sha3/proofs/fstar/equivalence

# Only verify spec + equivalence files in this directory.
# Implementation files (Libcrux_sha3.*) are verified from crates/algorithms/sha3.
ROOTS = Hacspec_sha3.fst \
        Hacspec_sha3.Keccak_f.fst \
        Hacspec_sha3.Sha3.fst \
        Hacspec_sha3.Sponge.fst

# Equivalence proofs are slow (many SMT queries); skip by default.
# Run with VERIFY_SLOW_MODULES=yes to verify them.
SLOW_MODULES += 

# Impl modules that fail non-lax (debug_assert! issues) or are deep
# transitive deps not reachable from this directory. Admit them.
ADMIT_MODULES += 

# Deep transitive deps that Make can't locate from this directory.
# Tell F* they're already cached so it doesn't try to build .checked for them.
OTHERFLAGS += 

include $(shell git rev-parse --show-toplevel)/fstar-helpers/Makefile.base
