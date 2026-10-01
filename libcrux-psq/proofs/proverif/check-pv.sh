#!/usr/bin/env bash
# Re-extract the libcrux-psq PSQ handshake to ProVerif with the hax ProVerif
# backend, compose it with the symbolic crypto model (psq_crypto.pvl), and
# run the analyses. With `--load-only`, only check that every analysis loads,
# which takes seconds instead of hours.
#
#   HAX_HOME : a hax checkout, for the ProVerif libraries in
#              hax-lib/proof-libs/proverif (HAX_PROVERIF_DIR is also accepted).
#              cargo-hax and proverif are taken from PATH.
set -uo pipefail
# script lives at libcrux-psq/proofs/proverif/check-pv.sh; run from workspace root
cd "$(dirname "$0")/../../.."
HAX="${HAX_HOME:-${HAX_PROVERIF_DIR:?set HAX_HOME to a hax checkout}}"
PVLIB="$HAX/hax-lib/proof-libs/proverif"
PRIM="$PVLIB/primitives.pvl"
RESULT="$PVLIB/result.pvl"
PVD=libcrux-psq/proofs/proverif
EX="$PVD/extraction"

# Entry points: the initiators and the responder, with everything they call.
# The crypto boundary is annotated in src/** with
# `#[hax_lib::proverif::replace_body(...)]` and modeled in psq_crypto.pvl;
# serialization is bypassed there.
INC='-** +libcrux_psq::handshake::initiator::** +libcrux_psq::handshake::responder::**'
# psq_crypto.pvl models these trait items once for every impl.
KEEP='tls_codec::** libcrux_psq::**::CiphersuiteBase::**'
cargo hax -C -p libcrux-psq --features hax-pv ';' into -i "$INC" proverif --keep-trait-calls "$KEEP" || exit 1

# psq_crypto.pvl defines `pv_serialize`, `pv_deserialize` and
# `CiphersuiteBase::name`, which lib.pvl calls and which match constructors of
# lib.pvl, so it declares those constructors itself: drop their declarations
# from lib.pvl.
python3 - "$PVD/psq_crypto.pvl" "$EX/lib.pvl" > "$EX/lib.clean.pvl" <<'PY'
import re,sys
decl=re.compile(r'^fun\s+([A-Za-z0-9_]+)\s*\([^)]*\)\s*:\s*bitstring\s*\[data\]\.\s*$', re.M)
strip={m.group(1) for m in decl.finditer(open(sys.argv[1]).read())}
out=[]
for s in re.split(r'(?<=\.)\n', open(sys.argv[2]).read()):
    # Items may be preceded by `(* ... *)` comments.
    body=re.sub(r'^\s*(?:\(\*.*?\*\)\s*)+', '', s, flags=re.S)
    m=decl.match(body)
    if m and m.group(1) in strip: continue
    out.append(s)
sys.stdout.write('\n'.join(out))
PY

# Drop the abstract declarations of missingdecl.pvl that psq_crypto.pvl defines.
python3 - "$PVD/psq_crypto.pvl" "$EX/missingdecl.pvl" > "$EX/missingdecl.dedup.pvl" <<'PY'
import re,sys
defs=set(re.findall(r'^(?:fun|letfun|const)\s+([A-Za-z0-9_]+)', open(sys.argv[1]).read(), re.M))
for l in open(sys.argv[2]):
    m=re.match(r'^(?:fun|const)\s+([A-Za-z0-9_]+)', l)
    if not (m and m.group(1) in defs): sys.stdout.write(l)
PY

# ---------------------------------------------------------------------------
# Run the two query-mode analyses against the composed model and assert the
# P1–P11 verdicts from psq-design/models/psq_query.pv. The shared setup/roles/
# aliases/nounif live in psq_query_lib.pvl; the queries split into:
#   analysis.pv      full responder (drives read_message_contents): P1, P4–P11
#   analysis_auth.pv auth-core responder (drives decrypt_outer_message): P2, P3
# P2/P3 are responder-authentication correspondences; the DH-commutativity model
# does not terminate through the full read_message_contents bookkeeping (rate
# limiter / ciphersuite coercion / state machine), so they run against the
# security-equivalent auth core.
LIBS=(-lib "$PRIM" -lib "$RESULT" -lib "$PVD/psq_crypto.pvl" -lib "$EX/missingdecl.dedup.pvl"
      -lib "$EX/lib.clean.pvl" -lib "$EX/psq_query_lib.pvl")
# Primary verdict per query. Skip ProVerif's secondary `RESULT (but event(...)
# is true.)` annotation that follows a false injective-correspondence query
# (e.g. R2c replay): it is not a separate query verdict.
verdicts () { grep '^RESULT' "$1" | grep -v '(but' | grep -oE 'is (true|false)' | awk '{print $2}' | tr '\n' ' '; }

# The registration analyses run against psq_reg_lib.pvl instead.
LIBS_REG=(-lib "$PRIM" -lib "$RESULT" -lib "$PVD/psq_crypto.pvl" -lib "$EX/missingdecl.dedup.pvl"
          -lib "$EX/lib.clean.pvl" -lib "$EX/psq_reg_lib.pvl")

# `--load-only` checks that every analysis loads, without running its queries.
if [ "${1:-}" = --load-only ]; then
  rc=0
  for f in "$EX"/*.pv; do
    case "$(basename "$f")" in
      loadcheck.pv) continue ;;
      analysis.pv | analysis_auth.pv) L=("${LIBS[@]}") ;;
      *) L=("${LIBS_REG[@]}") ;;
    esac
    if out=$(proverif -parse-only "${L[@]}" "$f" 2>&1); then
      echo "loads: $(basename "$f")"
    else
      echo "LOAD FAILED: $(basename "$f")"; printf '%s\n' "$out" | grep -m3 Error; rc=1
    fi
  done
  exit $rc
fi

# If the queries aren't present yet, fall back to a bare load-check.
if [ ! -f "$EX/analysis.pv" ]; then
  printf 'process\n  0\n' > "$EX/loadcheck.pv"
  LOG=$(mktemp); proverif "${LIBS[@]}" "$EX/loadcheck.pv" > "$LOG" 2>&1
  NERR=$(grep -c '^Error:' "$LOG")
  [ "$NERR" -eq 0 ] && echo "MODEL LOADS OK (0 errors)." || { echo "LOAD FAILED"; grep '^Error:' "$LOG"|head; exit 1; }
  echo "(no queries yet)"; exit 0
fi

LOG_MAIN=$(mktemp); LOG_AUTH=$(mktemp)
proverif "${LIBS[@]}" "$EX/analysis.pv"      > "$LOG_MAIN" 2>&1
proverif "${LIBS[@]}" "$EX/analysis_auth.pv" > "$LOG_AUTH" 2>&1
NERR=$(( $(grep -c '^Error:' "$LOG_MAIN") + $(grep -c '^Error:' "$LOG_AUTH") ))
if [ "$NERR" -ne 0 ]; then echo "LOAD FAILED ($NERR errors):"; grep '^Error:' "$LOG_MAIN" "$LOG_AUTH" | head; exit 1; fi

# Expected verdicts (psq_query.pv "Expected:" tags).
#   analysis.pv      P1×4 reachable(false), P4 false, P5–P9 true, P10 false, P11 false
#   analysis_auth.pv P2 true, P3 true
EXP_MAIN="false false false false false true true true true true false false "
EXP_AUTH="true true "
GOT_MAIN=$(verdicts "$LOG_MAIN"); GOT_AUTH=$(verdicts "$LOG_AUTH")
echo "MODEL LOADS OK (0 errors)."
echo "  P1,P4..P11  got: $GOT_MAIN"
echo "              exp: $EXP_MAIN"
echo "  P2,P3       got: $GOT_AUTH"
echo "              exp: $EXP_AUTH"
QUERY_OK=0
[ "$GOT_MAIN" = "$EXP_MAIN" ] && [ "$GOT_AUTH" = "$EXP_AUTH" ] && QUERY_OK=1

# ---------------------------------------------------------------------------
# Registration mode — MESSAGE-1 analyses (shared lib psq_reg_lib.pvl). Drives the
# real extracted registration initiator/responder (auth core), on a SOUND honest
# run (sanity_reg_passive.pv: InitReg*+RespReg* reachable under a PASSIVE attacker
# with K_1 agreement — the non-vacuity foundation). Reproduces psq-design/models/
# psq_registration_{dh,sig}_msg1.pv exactly, including R2c (replay) and R9 (not
# forward-secret):
#   DH  : R1 reachable, R2a false (DH initiator-auth not post-quantum sound),
#         R2b true, R2c false, R4 true, R6 true, R9 false, nonvac false
#                                              -> false false true false true true false false
#   sig : R1 reachable, R2a true (signature-based auth is PQ-sound),
#         R2c false, R4 true, R6 true, R9 false, nonvac false
#                                              -> false true false true true false false
# R9-false reconstructs reg_secret under endpoint compromise (no ephemeral break),
# doubling as the leak / non-vacuity control for the secrecy queries R4/R6.
REG_OK=1
if [ -f "$EX/analysis_reg_dh_msg1.pv" ]; then
  # The session analyses drive the full handshake + into_session and are the
  # heaviest runs (DH commutativity; the DH session ~58 min, the sig session ~40
  # min — its responder-auth correspondences R3/R3i are the long poles). Launch
  # both in the background now so they overlap each other and the (faster) msg1
  # analyses below; collected at the end.
  LOG_DHSESS=$(mktemp); LOG_SIGSESS=$(mktemp)
  if [ -f "$EX/analysis_reg_dh_session.pv" ]; then
    proverif "${LIBS_REG[@]}" "$EX/analysis_reg_dh_session.pv" > "$LOG_DHSESS" 2>&1 &
    DHSESS_PID=$!
  fi
  if [ -f "$EX/analysis_reg_sig_session.pv" ]; then
    proverif "${LIBS_REG[@]}" "$EX/analysis_reg_sig_session.pv" > "$LOG_SIGSESS" 2>&1 &
    SIGSESS_PID=$!
  fi
  LOG_SAN=$(mktemp); LOG_RDH=$(mktemp); LOG_RSIG=$(mktemp)
  LOG_SSAN=$(mktemp); LOG_SSSAN=$(mktemp)
  # Passive-first sanity: the honest registration round-trip must complete on its
  # own (no attacker help). Without this the active-attacker verdicts are unsound.
  proverif "${LIBS_REG[@]}" "$EX/sanity_reg_passive.pv"    > "$LOG_SAN"  2>&1
  proverif "${LIBS_REG[@]}" "$EX/analysis_reg_dh_msg1.pv"  > "$LOG_RDH"  2>&1
  proverif "${LIBS_REG[@]}" "$EX/analysis_reg_sig_msg1.pv" > "$LOG_RSIG" 2>&1
  # Post-quantum forward secrecy + authenticity: the quantum apocalypse (all
  # classical DH/sig keys derivable) hits in phase 1, strictly after the phase-0
  # session. A session accepted before the apocalypse keeps both secrecy (ML-KEM
  # anchor) and authenticity (timestamp form: only a compromise preceding the
  # acceptance can forge — contrast R2a, false, where the break is during the run).
  LOG_PQ=$(mktemp)
  [ -f "$EX/analysis_reg_dh_pq.pv" ] && \
    proverif "${LIBS_REG[@]}" "$EX/analysis_reg_dh_pq.pv" > "$LOG_PQ" 2>&1
  # Session passive sanity: the full two-message handshake + into_session honest run
  # must reach InitSessDH and RespSessDH on its own (response/session non-vacuity).
  [ -f "$EX/sanity_session_passive.pv" ] && \
    proverif "${LIBS_REG[@]}" "$EX/sanity_session_passive.pv" > "$LOG_SSAN" 2>&1
  [ -f "$EX/sanity_session_sig_passive.pv" ] && \
    proverif "${LIBS_REG[@]}" "$EX/sanity_session_sig_passive.pv" > "$LOG_SSSAN" 2>&1
  NERR=$(( $(grep -c '^Error:' "$LOG_SAN") + $(grep -c '^Error:' "$LOG_RDH") + $(grep -c '^Error:' "$LOG_RSIG") ))
  if [ "$NERR" -ne 0 ]; then echo "REG LOAD FAILED ($NERR errors):"; grep '^Error:' "$LOG_SAN" "$LOG_RDH" "$LOG_RSIG" | head; REG_OK=0; fi
  EXP_SAN="false false false false true true "   # InitReg*/RespReg* reachable + K_1 agreement
  EXP_RDH="false false true false true true false false "
  EXP_RSIG="false true false true true false false "
  EXP_SSAN="false false "                        # InitSessDH + RespSessDH reachable
  GOT_SAN=$(verdicts "$LOG_SAN"); GOT_RDH=$(verdicts "$LOG_RDH"); GOT_RSIG=$(verdicts "$LOG_RSIG")
  echo "  reg passive sanity got: $GOT_SAN"
  echo "                     exp: $EXP_SAN"
  echo "  reg dh msg1 got: $GOT_RDH"
  echo "              exp: $EXP_RDH"
  echo "  reg sig msg1 got: $GOT_RSIG"
  echo "              exp: $EXP_RSIG"
  [ "$GOT_SAN" = "$EXP_SAN" ] && [ "$GOT_RDH" = "$EXP_RDH" ] && [ "$GOT_RSIG" = "$EXP_RSIG" ] || REG_OK=0
  if [ -f "$EX/sanity_session_passive.pv" ]; then
    GOT_SSAN=$(verdicts "$LOG_SSAN")
    echo "  session passive sanity got: $GOT_SSAN"
    echo "                         exp: $EXP_SSAN"
    [ "$GOT_SSAN" = "$EXP_SSAN" ] || REG_OK=0
  fi
  if [ -f "$EX/sanity_session_sig_passive.pv" ]; then
    GOT_SSSAN=$(verdicts "$LOG_SSSAN")
    echo "  session sig passive sanity got: $GOT_SSSAN"
    echo "                             exp: $EXP_SSAN"   # InitSessSig + RespSessSig reachable
    [ "$GOT_SSSAN" = "$EXP_SSAN" ] || REG_OK=0
  fi
  if [ -f "$EX/analysis_reg_dh_pq.pv" ]; then
    if [ "$(grep -c '^Error:' "$LOG_PQ")" -ne 0 ]; then echo "PQ LOAD FAILED:"; grep '^Error:' "$LOG_PQ" | head; REG_OK=0; fi
    EXP_PQ="true true true true false "   # FA1, FA2(timestamp), FS1, FS2, NV
    GOT_PQ=$(verdicts "$LOG_PQ")
    echo "  pq fwd-sec/auth (FA1 FA2 FS1 FS2 NV) got: $GOT_PQ"
    echo "                                       exp: $EXP_PQ"
    [ "$GOT_PQ" = "$EXP_PQ" ] || REG_OK=0
  fi
  # Collect the backgrounded DH session secrecy (R5 InitSessDH, R5 RespSessDH, R7, R8).
  if [ -n "${DHSESS_PID:-}" ]; then
    echo "  (waiting on DH session secrecy R5/R7/R8 ...)"
    wait "$DHSESS_PID"
    if [ "$(grep -c '^Error:' "$LOG_DHSESS")" -ne 0 ]; then
      echo "  DH SESSION LOAD FAILED:"; grep '^Error:' "$LOG_DHSESS" | head; REG_OK=0
    fi
    EXP_DHSESS="true true true true "   # R5(Init), R5(Resp), R7, R8
    GOT_DHSESS=$(verdicts "$LOG_DHSESS")
    echo "  dh session R5/R7/R8 got: $GOT_DHSESS"
    echo "                      exp: $EXP_DHSESS"
    [ "$GOT_DHSESS" = "$EXP_DHSESS" ] || REG_OK=0
  fi
  # Collect the backgrounded sig session (R3 + R3i responder auth, R5x2, R7, R8).
  if [ -n "${SIGSESS_PID:-}" ]; then
    echo "  (waiting on sig session R3/R3i/R5/R7/R8 ...)"
    wait "$SIGSESS_PID"
    if [ "$(grep -c '^Error:' "$LOG_SIGSESS")" -ne 0 ]; then
      echo "  SIG SESSION LOAD FAILED:"; grep '^Error:' "$LOG_SIGSESS" | head; REG_OK=0
    fi
    EXP_SIGSESS="true true true true true true "   # R3, R3i, R5(Init), R5(Resp), R7, R8
    GOT_SIGSESS=$(verdicts "$LOG_SIGSESS")
    echo "  sig session R3/R3i/R5/R7/R8 got: $GOT_SIGSESS"
    echo "                              exp: $EXP_SIGSESS"
    [ "$GOT_SIGSESS" = "$EXP_SIGSESS" ] || REG_OK=0
  fi
fi

if [ "$QUERY_OK" = 1 ] && [ "$REG_OK" = 1 ]; then
  echo "CHECK PASSED (query 14/14 + reg msg1 R1/R2a/R2b/R2c/R4/R6/R9 + PQ fwd-sec/auth + DH session R1/R5/R7/R8 + sig session R1/R3/R3i/R5/R7/R8: DH R2a false / sig R2a true; sig responder-auth + post-quantum fwd-secret/authentic)"
else
  echo "CHECK FAILED"; exit 1
fi
