#!/usr/bin/env python3

import subprocess
import re
import sys
from pathlib import Path

import os

HAX_VERSION = "ffdf432705d409b62ec025d253a340234b59766f"
AENEAS_VERSION = "8d2077c"


def check_version(cmd: list[str], name: str, expected: str) -> None:
    result = subprocess.run(cmd, capture_output=True, text=True)
    output = result.stdout + result.stderr
    if expected not in output:
        print(f"Version mismatch for {name}: expected {expected!r} in output:\n{output}", file=sys.stderr)
        sys.exit(1)


check_version(["cargo", "hax", "--version"], "hax", HAX_VERSION)
check_version(["aeneas", "-version"], "aeneas", AENEAS_VERSION)

result = subprocess.run(
    ["cargo", "hax", "into", "aeneas-lean", '--aeneas-args="-core-models-lib"'],
    env={**os.environ, "RUSTFLAGS": "--cfg hax_backend_lean"}
)

funs_lean = Path("proofs/aeneas-lean/HacspecSha3/Extraction/Funs.lean")
content = funs_lean.read_text()

content = re.sub(
    r"(^import Aeneas\b)",
    r"\1\nimport HacspecSha3.Missing",
    content,
    count=1,
    flags=re.MULTILINE,
)

funs_lean.write_text(content)
