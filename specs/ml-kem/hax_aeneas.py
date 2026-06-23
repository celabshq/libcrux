#!/usr/bin/env python3
"""Hax into aeneas-lean extraction script for hacspec ML-KEM.

Extracts the entire spec crate and patches the generated
`HacspecMlKem/Extraction/Funs.lean` to import `HacspecMlKem.Missing`
and to apply known mis-extraction workarounds.
"""

import os
import re
import shutil
import subprocess
import sys
from pathlib import Path

HAX_VERSION = "ffdf432705d409b62ec025d253a340234b59766f"
AENEAS_VERSION = "8d2077c"

def check_version(cmd: list[str], expected: str) -> None:
    result = subprocess.run(cmd, capture_output=True, text=True)
    output = result.stdout + result.stderr
    if expected not in output:
        if os.environ.get("SKIP_VERSION_CHECK") == "1":
            print(f"warning: version mismatch for {cmd[0]} (expected {expected!r}); continuing because SKIP_VERSION_CHECK=1", file=sys.stderr)
            return
        print(f"Version mismatch for {cmd[0]}: expected {expected!r} in output:\n{output}", file=sys.stderr)
        sys.exit(1)

check_version(["cargo", "hax", "--version"], HAX_VERSION)
check_version(["aeneas", "-version"], AENEAS_VERSION)

result = subprocess.run(
    ["cargo", "hax", "into", "aeneas-lean",
                   "--aeneas-args=-core-models-lib"],
    env={**os.environ, "RUSTFLAGS": "--cfg hax_backend_lean"}
)

funs_lean = Path("proofs/aeneas-lean/HacspecMlKem/Extraction/Funs.lean")

# Aeneas can return non-zero while still emitting a partial `Funs.lean`
# Apply the import patch whenever the file exists; only abort if extraction
# produced nothing at all.
if not funs_lean.exists():
    sys.exit(result.returncode if result.returncode != 0 else 1)
if result.returncode != 0:
    print(
        f"warning: hax/aeneas exited with code {result.returncode}; "
        f"applying patches to partial {funs_lean}.",
        file=sys.stderr,
    )

content = funs_lean.read_text()

# Pull in our hand-written stubs (`HacspecMlKem.Missing`).
content = re.sub(
    r"import CoreModels",
    "import CoreModels\nimport HacspecMlKem.Missing",
    content,
    count=1,
)

# Increase recursion depth for `ntt.ZETAS`.
content = content.replace(
    "/-- [hacspec_ml_kem::ntt::ZETAS]",
    "set_option maxRecDepth 1000 in\n/-- [hacspec_ml_kem::ntt::ZETAS]",
    1,
)

# Aeneas emits `fmt::rt::Argument::new_display` with two
# arguments (the `Display` instance + the value), but rust-core-models
# defines it with one. The blocks below appear only in panic-path
# formatting (`fail panic` follows them), so we block-comment them
# away.
PANIC_FMT_BLOCK_RX = re.compile(
    r"    let a ←\n"
    r"      core\.fmt\.rt\.Argument\.new_display core\.Usize\.Insts\.CoreFmtDisplay [a-zA-Z_0-9]+\n"
    r"    let _ ←\n"
    r"      core\.fmt\.Arguments\.new\n"
    r"        \(Array\.make [0-9]+#usize \[\n"
    r"(?:[^\]]+\n)+"
    r"          \]\) \(Array\.make [0-9]+#usize \[ a \]\)"
)
def _comment_panic_fmt(m: 're.Match[str]') -> str:
    return "/-\n" + m.group(0) + "\n-/"
content = PANIC_FMT_BLOCK_RX.sub(_comment_panic_fmt, content)

# `cmp.PartialEq` has only `eq` — drop the synthesised `ne` field.
content = re.sub(
    r"\n  ne := [^\n]+(?=\n})",
    "",
    content,
)

# `cmp.Eq` has only `PartialEqInst` — drop the synthesised
# `assert_fields_are_eq` field.
content = re.sub(
    r"\n  assert_fields_are_eq :=\n    [^\n]+(?=\n})",
    "",
    content,
)

funs_lean.write_text(content)
