#!/usr/bin/env python3
"""
Extraction driver for the ML-DSA spec → aeneas-lean.
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

charon_args = " ".join(
    [f"--start-from {root}" for root in START_FROM]
)

result = subprocess.run(
    ["cargo", "hax", "into", "aeneas-lean",
     "--aeneas-args=-core-models-lib",
     f"--charon-args={charon_args}"],
    env={**os.environ, "RUSTFLAGS": "--cfg hax_backend_lean"},
)
if result.returncode != 0:
    sys.exit(result.returncode)