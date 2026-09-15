# Hand-written proof companions live in ../spec so that this directory holds
# only generated extraction output and can be cleaned wholesale.
FSTAR_INCLUDE_DIRS_EXTRA += ../spec

# Verify the hand-written companions in ../spec as well.  The default ROOTS
# globs THIS directory only, and ../spec has no Makefile of its own (unlike
# ml-kem's spec/commute and ml-dsa's spec, which are verified by their own
# invocation), so a companion that no extracted module happens to import would
# otherwise never be checked at all.
ROOTS = $(wildcard *.fst *.fsti) $(wildcard ../spec/*.fst ../spec/*.fsti)

include $(shell git rev-parse --show-toplevel)/fstar-helpers/Makefile.base

