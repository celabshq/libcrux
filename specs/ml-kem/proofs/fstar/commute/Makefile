FSTAR_INCLUDE_DIRS_EXTRA += ../extraction \
	      $(shell git rev-parse --show-toplevel)/libcrux-ml-kem/proofs/fstar/spec \
	      $(shell git rev-parse --show-toplevel)/libcrux-ml-kem/proofs/fstar/extraction \
	      $(shell git rev-parse --show-toplevel)/fstar-helpers/fstar-bitvec \
	      $(shell git rev-parse --show-toplevel)/libcrux-intrinsics/proofs/fstar/extraction

ENABLE_HINTS = --use_hints --record_hints

include $(shell git rev-parse --show-toplevel)/fstar-helpers/Makefile.base
