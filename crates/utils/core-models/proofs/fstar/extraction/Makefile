# Hand-written proof companions live in ../spec so that this directory holds
# only generated extraction output and can be cleaned wholesale.
FSTAR_INCLUDE_DIRS_EXTRA += ../spec

# Verify the hand-written companions in ../spec as well: the default ROOTS
# globs this directory only and ../spec has no Makefile of its own.
ROOTS = $(wildcard *.fst *.fsti) $(wildcard ../spec/*.fst ../spec/*.fsti)

include $(shell git rev-parse --show-toplevel)/fstar-helpers/Makefile.base

