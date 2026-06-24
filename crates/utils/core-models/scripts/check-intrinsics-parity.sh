#!/usr/bin/env bash
# CI gate for the SIMD intrinsics trust-base coverage.
# Phase C2 of the trust-base sprint. See INTRINSICS-TRUST-PLAN.md.
#
# What it does:
#   1. Re-runs the audit script (intrinsics-audit.py).
#   2. Asserts T1 totals match the libcrux surface (`pub fn` count).
#      A new pub fn in `intrinsics/src/{avx2,arm64}.rs` grows T1, fails
#      this check, and forces the contributor to either model+test the
#      new wrapper (raising the coverage threshold) or remove it.
#   3. Asserts D6.1 (Rust-model coverage) and D6.2 (test coverage)
#      counts have not regressed below the committed thresholds.
#   4. Runs `cargo test -p core-models` (host).
#   5. On macOS hosts with the x86_64 target installed, also runs the
#      AVX2 mk! cross-compile pass via `--target x86_64-apple-darwin`.
#
# Threshold-bump protocol:
#   When a new wrapper gains body+test, re-run this script. It will
#   fail with the new (higher) covered count. Bump the THRESHOLD_*
#   constants below to match in the same commit. The script then
#   permanently forbids regression below that point.

set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"

CSV="crates/utils/core-models/proofs/intrinsics-trust-index.csv"
AUDIT="crates/utils/core-models/scripts/intrinsics-audit.py"

# Coverage thresholds (counts, not percentages — exact integer comparison).
# Update in lockstep with the CSV when new wrappers gain body+test.
EXPECT_T1=193
EXPECT_T1_AVX2=99
EXPECT_T1_ARM64=94
THRESHOLD_D61=180
THRESHOLD_D62=180
THRESHOLD_D61_AVX2=87
THRESHOLD_D62_AVX2=87
THRESHOLD_D61_ARM64=93
THRESHOLD_D62_ARM64=93

LOG_DIR="${RUNNER_TEMP:-/tmp}"

audit_log="$LOG_DIR/intrinsics-audit.log"
test_log="$LOG_DIR/intrinsics-cargo-test.log"
xtest_log="$LOG_DIR/intrinsics-cargo-test-x86.log"

echo "==> Running audit"
python3 "$AUDIT" > "$audit_log" 2>&1 || {
  echo "FAIL: audit script errored"
  cat "$audit_log"
  exit 1
}
tail -n1 "$audit_log"

# Counts derived from the regenerated CSV. Columns:
#   1=name 2=arch 3=in_T1 4=underlying 5=has_body 6=has_mk_test ...
T1=$(awk -F, 'NR>1 && $3=="true" {n++} END{print n+0}' "$CSV")
T1_AVX2=$(awk -F, 'NR>1 && $3=="true" && $2=="avx2" {n++} END{print n+0}' "$CSV")
T1_ARM64=$(awk -F, 'NR>1 && $3=="true" && $2=="arm64" {n++} END{print n+0}' "$CSV")
D61=$(awk -F, 'NR>1 && $3=="true" && $5=="true" {n++} END{print n+0}' "$CSV")
D62=$(awk -F, 'NR>1 && $3=="true" && $6=="true" {n++} END{print n+0}' "$CSV")
D61_AVX2=$(awk -F, 'NR>1 && $3=="true" && $5=="true" && $2=="avx2" {n++} END{print n+0}' "$CSV")
D62_AVX2=$(awk -F, 'NR>1 && $3=="true" && $6=="true" && $2=="avx2" {n++} END{print n+0}' "$CSV")
D61_ARM64=$(awk -F, 'NR>1 && $3=="true" && $5=="true" && $2=="arm64" {n++} END{print n+0}' "$CSV")
D62_ARM64=$(awk -F, 'NR>1 && $3=="true" && $6=="true" && $2=="arm64" {n++} END{print n+0}' "$CSV")

echo "==> Coverage: T1=$T1 (avx2=$T1_AVX2 arm64=$T1_ARM64)"
echo "    D6.1 covered=$D61 (avx2=$D61_AVX2/$T1_AVX2 arm64=$D61_ARM64/$T1_ARM64)"
echo "    D6.2 covered=$D62 (avx2=$D62_AVX2/$T1_AVX2 arm64=$D62_ARM64/$T1_ARM64)"

fail=0
ck() {
  local name="$1" actual="$2" cmp="$3" want="$4"
  if [[ "$cmp" == "==" ]]; then
    if [[ "$actual" -ne "$want" ]]; then
      echo "FAIL: $name=$actual (expected ==$want)"
      fail=1
    fi
  else
    if [[ "$actual" -lt "$want" ]]; then
      echo "FAIL: $name=$actual (expected >=$want)"
      fail=1
    fi
  fi
}

ck T1            "$T1"         == "$EXPECT_T1"
ck T1_avx2       "$T1_AVX2"    == "$EXPECT_T1_AVX2"
ck T1_arm64      "$T1_ARM64"   == "$EXPECT_T1_ARM64"
ck D6.1_total    "$D61"        ">=" "$THRESHOLD_D61"
ck D6.2_total    "$D62"        ">=" "$THRESHOLD_D62"
ck D6.1_avx2     "$D61_AVX2"   ">=" "$THRESHOLD_D61_AVX2"
ck D6.2_avx2     "$D62_AVX2"   ">=" "$THRESHOLD_D62_AVX2"
ck D6.1_arm64    "$D61_ARM64"  ">=" "$THRESHOLD_D61_ARM64"
ck D6.2_arm64    "$D62_ARM64"  ">=" "$THRESHOLD_D62_ARM64"

if [[ $fail -ne 0 ]]; then
  echo
  echo "Coverage regression. Either:"
  echo "  - add body+test for the new/regressed wrappers, or"
  echo "  - bump the THRESHOLD_* / EXPECT_* in $0 if intentional."
  exit 1
fi

echo "==> cargo test -p core-models"
if ! cargo test -p core-models > "$test_log" 2>&1; then
  echo "FAIL: cargo test -p core-models"
  grep -E "FAILED|error\[|panicked|test result:" "$test_log" | head -n 50 || true
  exit 1
fi
grep "test result:" "$test_log" | head -n 5

# AVX2 mk! tests are gated on x86 target_arch. Cross-compile when the host
# is a Mac that has the x86_64-apple-darwin toolchain installed.
host_os="$(uname -s 2>/dev/null || echo unknown)"
if [[ "$host_os" == "Darwin" ]] && rustup target list --installed 2>/dev/null \
     | grep -qx "x86_64-apple-darwin"; then
  echo "==> cargo test -p core-models --target x86_64-apple-darwin"
  if ! cargo test -p core-models --target x86_64-apple-darwin > "$xtest_log" 2>&1; then
    echo "FAIL: cargo test --target x86_64-apple-darwin"
    grep -E "FAILED|error\[|panicked|test result:" "$xtest_log" | head -n 50 || true
    exit 1
  fi
  grep "test result:" "$xtest_log" | head -n 5
else
  echo "==> skipping x86_64-apple-darwin cross-compile (host=$host_os, target not installed)"
fi

echo
echo "OK: D6.1=$D61/$T1 D6.2=$D62/$T1 (avx2 $D61_AVX2/$T1_AVX2, arm64 $D61_ARM64/$T1_ARM64)"
