SLOW_MODULES +=
VERIFIED_MODULES = \
	Libcrux_ml_dsa.Simd.Portable.Encoding.Commitment.fst \
	Libcrux_ml_dsa.Simd.Portable.Vector_type.fst \
	Libcrux_ml_dsa.Simd.Portable.Sample.fst \
	Libcrux_ml_dsa.Simd.Portable.Arithmetic.fst \
	Libcrux_ml_dsa.Simd.Portable.Ntt.fst \
	Libcrux_ml_dsa.Simd.Portable.Invntt.fst \
	Libcrux_ml_dsa.Simd.Portable.Encoding.Error.fst \
	Libcrux_ml_dsa.Simd.Portable.Encoding.Commitment.fst \
	Libcrux_ml_dsa.Simd.Portable.Encoding.Gamma1.fst \
	Libcrux_ml_dsa.Simd.Portable.Encoding.T0.fst \
	Libcrux_ml_dsa.Simd.Portable.Encoding.T1.fst \
	Libcrux_ml_dsa.Simd.Avx2.Vector_type.fst \
	Libcrux_ml_dsa.Simd.Avx2.Arithmetic.fst \
	Libcrux_ml_dsa.Simd.Avx2.Ntt.fst \
	Libcrux_ml_dsa.Simd.Avx2.Invntt.fst \
	Libcrux_ml_dsa.Simd.Avx2.Encoding.Error.fst \
	Libcrux_ml_dsa.Simd.Avx2.Encoding.Commitment.fst \
	Libcrux_ml_dsa.Simd.Avx2.Encoding.Gamma1.fst \
	Libcrux_ml_dsa.Simd.Avx2.Encoding.T0.fst \
	Libcrux_ml_dsa.Simd.Avx2.Encoding.T1.fst \
	Libcrux_ml_dsa.Simd.Traits.fsti \
	Libcrux_ml_dsa.Simd.Traits.Spec.fst \
	Libcrux_ml_dsa.Simd.Traits.fst \
	Libcrux_ml_dsa.Constants.fst \
	Libcrux_ml_dsa.Types.fst \
	Libcrux_ml_dsa.Arithmetic.fst \
	Libcrux_ml_dsa.Polynomial.fst \
	end_of_verified_modules

VERIFIED_OR_SLOW_MODULES = $(sort $(SLOW_MODULES) $(VERIFIED_MODULES))
ADMIT_MODULES = $(filter-out ${VERIFIED_OR_SLOW_MODULES}, $(wildcard *.fst)) $(filter-out ${VERIFIED_OR_SLOW_MODULES}, $(wildcard *.fsti))
FSTAR_INCLUDE_DIRS_EXTRA += ../spec \
	      $(shell git rev-parse --show-toplevel)/fstar-helpers/fstar-bitvec \
	      $(shell git rev-parse --show-toplevel)/libcrux-ml-kem/proofs/fstar/spec

include $(shell git rev-parse --show-toplevel)/fstar-helpers/Makefile.base
