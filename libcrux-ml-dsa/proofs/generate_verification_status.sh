#!/usr/bin/env bash
# Generate proofs/verification_status.md from Rust source annotations and Makefile ADMIT list.
# Run from the libcrux-ml-kem directory.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
exec python3 "$SCRIPT_DIR/generate_verification_status.py" "$@"
