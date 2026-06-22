#!/usr/bin/env python3
"""Extraction driver for the clean-Z_q ML-DSA spec (`hacspec_ml_dsa`) → aeneas-lean.

Produces the `HacspecMlDsa` Lean library: the machine-EXTRACTED Rust spec that
the libcrux-iot ML-DSA poly-layer FCs re-target against, so the hand-written
`LibcruxIotMlDsa/Spec/Pure.lean` is no longer the trusted standalone reference
(spec-extraction campaign, `~/.claude/plans/iot-mldsa-spec-extraction-campaign.md`).

Mirrors `libcrux-iot/ml-dsa/hax_aeneas.py`:
1. (Version pins are advisory here — run under the hax-evit opam switch with
   SKIP_VERSION_CHECK=1; aeneas/charon resolve to ~/.cargo/bin even under the
   switch, exactly as the impl-side extraction.)
2. Run `cargo hax into aeneas-lean` with a NARROW Charon `--start-from` list:
   only the NTT/poly/arith fns the poly-layer FCs reference. Everything else
   (vector_*, rounding vector wrappers, poly_mod_pm, parameters glue,
   sampling/encoding/ml_dsa/hash/matrix) is out of scope and pruned.
3. Patch the generated `HacspecMlDsa/Extraction/Funs.lean` in-place.

NARROW START_FROM rationale: the only `createi(|i| {…})` closures with
multi-statement / branching bodies that aeneas-lean cannot translate are
`ntt::ntt_layer` and `intt::intt_layer` (factored into `*_coeff` helpers below).
`polynomial::poly_mod_pm` and the `vector_*` nested-closure functions have the
same problem but are NOT reachable from these roots, so we never touch them.
"""

import os
import re
import subprocess
import sys
from pathlib import Path

# Charon translation roots. Anything not reachable from these is dropped.
START_FROM = [
    "crate::ntt::ntt",
    "crate::ntt::intt",
    "crate::polynomial::poly_add",
    "crate::polynomial::poly_sub",
    "crate::polynomial::poly_pointwise_mul",
    "crate::polynomial::poly_infinity_norm",
    "crate::arithmetic::coeff_norm",
    "crate::arithmetic::mod_q",
]

# Defensive opaque list (these modules are not reachable from the narrow roots
# anyway, but keep them opaque so an accidental transitive edge can't pull in
# unmodeled hash/encoding/sampling bodies).
OPAQUE = [
    "crate::encoding::*",
    "crate::sampling::*",
    "crate::ml_dsa::*",
    "crate::hash_functions::*",
    "crate::matrix::*",
    "crate::error::*",
]

charon_args = " ".join(
    [f"--start-from {root}" for root in START_FROM] +
    [f"--opaque {item}" for item in OPAQUE]
)

result = subprocess.run(
    ["cargo", "hax", "into", "aeneas-lean",
     "--aeneas-args=-core-models-lib",
     f"--charon-args={charon_args}"],
    env={**os.environ, "RUSTFLAGS": "--cfg hax_backend_lean"},
)
if result.returncode != 0:
    sys.exit(result.returncode)

funs_lean = Path("proofs/aeneas-lean/HacspecMlDsa/Extraction/Funs.lean")
content = funs_lean.read_text()

# Import the hand-written `Missing.lean` (if present) right after CoreModels,
# mirroring the ml-kem / impl-side patch.
missing_path = Path("proofs/aeneas-lean/HacspecMlDsa/Extraction/Missing.lean")
if missing_path.exists() and "HacspecMlDsa.Extraction.Missing" not in content:
    content = content.replace(
        "import CoreModels",
        "import CoreModels\n"
        "import HacspecMlDsa.Extraction.Missing",
        1,
    )

# Convert `axiom` declarations emitted for `--opaque` items to `opaque`
# (`axiom` shows up in `#print axioms`; `opaque` does not).
content = re.sub(r"^axiom ", "opaque ", content, flags=re.MULTILINE)

funs_lean.write_text(content)
print("Patched", funs_lean)
