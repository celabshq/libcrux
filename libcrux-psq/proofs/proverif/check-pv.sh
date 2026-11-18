#!/usr/bin/env bash
# Re-extract the libcrux-psq PSQ handshake to ProVerif via the hax ProVerif
# backend, compose it with the symbolic crypto model (psq_crypto.pvl) and the
# de-duplicated `missingdecl`, and run the ProVerif analyses in extraction/.
set -uo pipefail
# Modes:
#   (default)      re-extract libcrux-psq via the hax backend, then verify;
#   NO_EXTRACT=1   skip extraction, verify the extraction/ already on disk;
#   EXTRACT_ONLY=1 re-extract and post-process (lib.clean.pvl,
#                  missingdecl.dedup.pvl), but do not run ProVerif.
# Env:
#   HAX_PROVERIF_DIR : a hax checkout @ cargo-hax-v0.5.0 (for the ProVerif
#                      proof-libs). cargo-hax 0.5.0, its hax-rust-engine and
#                      the hax-engine must be first on PATH (or in the
#                      hax-proverif opam switch). REQUIRED.
#   LIST=1           : list the analysis names and exit.
#   ONLY=a,b         : run only these analyses (names or globs, e.g.
#                      ONLY='reg-*-msg1' or ONLY=reg-dh-session).
#   SKIP=a,b         : skip these analyses (names or globs).
#   SKIP_SLOW=1      : skip the two registration session analyses (the slowest).
#   PV_TIMEOUT       : wall-clock limit per ProVerif run, in seconds (default
#                      14400). A run that hits it is reported as a timeout, a
#                      tool limitation with no verdict.
#   PV_MEM_LIMIT     : memory cap per ProVerif run (systemd syntax, e.g. 12G).
#                      A run over it is killed and reported as having no
#                      verdict; the other analyses carry on. Even without a cap,
#                      each run is in its own systemd scope, so a kernel OOM kill
#                      takes down only that run.
#   PV_SERIAL=1      : run the slow analyses one after another rather than in
#                      parallel (lower peak memory).
#   PV_LOG_DIR       : where per-analysis logs + summary.txt go (default
#                      pv-logs/<timestamp>/ next to this script).
#   PV_RECHECK=1     : do not run ProVerif; judge the logs already in PV_LOG_DIR
#                      (e.g. after changing the expected verdicts).
# Exit status: 0 all analyses matched; 1 a failure (wrong verdict, "cannot be
# proved", load error); 2 no failures, but at least one analysis timed out or
# was killed (out of memory).
SELF="$(cd "$(dirname "$0")" && pwd)"
# script lives at libcrux-psq/proofs/proverif/check-pv.sh
WS="$(cd "$SELF/../../.." && pwd)"
if [ "${NO_EXTRACT:-0}" = 1 ] && [ "${EXTRACT_ONLY:-0}" = 1 ]; then
  echo "NO_EXTRACT=1 and EXTRACT_ONLY=1 together would do nothing"; exit 1
fi
[ -n "${HAX_PROVERIF_DIR:-}" ] || { echo "set HAX_PROVERIF_DIR to a hax checkout"; exit 1; }
ENG="$(cd "$HAX_PROVERIF_DIR" && pwd)" || { echo "HAX_PROVERIF_DIR not found: $HAX_PROVERIF_DIR"; exit 1; }
PRIM="$ENG/hax-lib/proof-libs/proverif/primitives.pvl"
[ -f "$PRIM" ] || { echo "no $PRIM — is HAX_PROVERIF_DIR a hax checkout?"; exit 1; }
# The Result model for extraction without `--result-errors` (hax >= 0.5.0).
RES="$ENG/hax-lib/proof-libs/proverif/result.pvl"
[ -f "$RES" ] || { echo "no $RES — hax 0.5.0 or later is required"; exit 1; }
PVD="$SELF"
EX="$SELF/extraction"
eval "$(opam env --switch=hax-proverif 2>/dev/null)" 2>/dev/null || true

# Query-mode entry points: the query initiator + the (unified) responder,
# transitively. The crypto boundary is annotated in src/** with
# `#[hax_lib::proverif::replace_body(...)]` (DH / KDF / AEAD) and modeled in
# psq_crypto.pvl; serialization is bypassed there.
# derive_k1_{dh,sig} are named explicitly: their function-local K1* structs are
# otherwise not extracted, and hax >= 0.5 rejects record literals of a struct
# whose definition (field order) it has not seen.
INC='-** +~libcrux_psq::handshake::initiator::** +~libcrux_psq::handshake::responder::**'
INC+=' +~libcrux_psq::handshake::derive_k1_dh +~libcrux_psq::handshake::derive_k1_sig'
if [ "${NO_EXTRACT:-0}" != 1 ]; then
  if ! command -v cargo-hax >/dev/null 2>&1; then export PATH="$ENG/target/release:$PATH"; fi
  # Keep the Rust engine in lockstep with whichever cargo-hax was picked up.
  # (The Nix cargo-hax wrapper puts its own engine on PATH instead.)
  ENGINE_NEXT_TO_HAX="$(dirname "$(command -v cargo-hax)")/hax-rust-engine"
  if [ -z "${HAX_RUST_ENGINE_BINARY:-}" ] && [ -x "$ENGINE_NEXT_TO_HAX" ]; then
    export HAX_RUST_ENGINE_BINARY="$ENGINE_NEXT_TO_HAX"
  fi
  echo "using $(cargo hax --version 2>&1 | grep version) from $(command -v cargo-hax)"
  # cargo-hax exits non-zero on HAX diagnostics (e.g. a tls_codec serialization
  # method in a call-graph cycle is opacified to an uninterpreted fun) but
  # still writes lib.pvl, so judge success by the output files instead. hax
  # skips rewriting an unchanged file, so remove the previous (gitignored)
  # outputs first and require them to reappear.
  rm -f "$EX/lib.pvl" "$EX/missingdecl.pvl"
  ( cd "$WS" && cargo hax -C -p libcrux-psq ';' into -i "$INC" proverif )
  for f in lib.pvl missingdecl.pvl; do
    [ -s "$EX/$f" ] || { echo "EXTRACTION FAILED: $EX/$f was not written"; exit 1; }
  done
else
  echo "NO_EXTRACT=1 — verifying the extraction/ already on disk (no re-extract)."
fi

# (a) Neutralize + hoist self-recursive serialization stubs. hax renders
#     unresolvable tls_codec trait methods as `f(..) = f(..)`; the engine
#     converts the bare form to an opaque `fun`, this pass catches the
#     let-wrapped form too and hoists ALL opaque stub funs to the top (they are
#     dependency-free; this fixes the mutual-recursion forward references).
python3 - "$EX/lib.pvl" > "$EX/lib.clean.pvl" <<'PY'
import re,sys
# Constructors re-declared in psq_crypto.pvl (so the pv_deserialize bridge and
# the signature model can reference them before lib.pvl loads); strip their
# `fun … [data].` decls here to avoid a duplicate definition. Their projector
# reducs + use sites stay and resolve against the psq_crypto declaration (loaded
# earlier).
STRIP={'libcrux_psq__handshake__initiator__InitiatorOuterPayloadOut__Query',
       'libcrux_psq__handshake__responder__InitiatorOuterPayload__Query',
       'libcrux_psq__handshake__initiator__InitiatorOuterPayloadOut__Registration',
       'libcrux_psq__handshake__responder__InitiatorOuterPayload__Registration',
       'libcrux_psq__handshake__InnerMessageOut__InnerMessageOut',
       'libcrux_psq__handshake__InnerMessage__InnerMessage',
       'libcrux_psq__handshake__AuthMessageOut__Dh',
       'libcrux_psq__handshake__AuthMessageOut__Sig',
       'libcrux_psq__handshake__AuthMessage__Dh',
       'libcrux_psq__handshake__AuthMessage__Sig',
       'libcrux_psq__handshake__initiator__InitiatorInnerPayloadOut__InitiatorInnerPayloadOut',
       'libcrux_psq__handshake__initiator__InitiatorInnerPayload__InitiatorInnerPayload',
       'libcrux_psq__handshake__responder__ResponderRegistrationPayloadOut__ResponderRegistrationPayloadOut',
       'libcrux_psq__handshake__responder__ResponderRegistrationPayload__ResponderRegistrationPayload',
       'tls_codec__quic_vec__VLByteSlice__VLByteSlice',
       'libcrux_psq__handshake__ciphersuite__initiator__SigningKeyPair__Ed25519',
       'libcrux_psq__handshake__ciphersuite__initiator__SigningKeyPair__MlDsa65',
       'libcrux_psq__handshake__ciphersuite__types__Signature__Ed25519',
       'libcrux_psq__handshake__ciphersuite__types__Signature__MlDsa65',
       'libcrux_psq__handshake__ciphersuite__types__SignatureVerificationKey__Ed25519',
       'libcrux_psq__handshake__ciphersuite__types__SignatureVerificationKey__MlDsa65',
       'libcrux_psq__handshake__ciphersuite__initiator__Auth__DH',
       'libcrux_psq__handshake__ciphersuite__initiator__Auth__Sig',
       'libcrux_psq__handshake__dhkem__DHKeyPair__DHKeyPair',
       'libcrux_psq__handshake__ciphersuite__types__Authenticator__Dh',
       'libcrux_psq__handshake__ciphersuite__types__Authenticator__Sig',
       'libcrux_psq__handshake__ciphersuite__initiator__PqKemPublicKey__MlKem',
       'libcrux_psq__handshake__ciphersuite__types__PQEncapsulationKey__MlKem',
       'libcrux_psq__handshake__ciphersuite__initiator__InitiatorCiphersuite__InitiatorCiphersuite',
       'libcrux_psq__handshake__ciphersuite__responder__ResponderCiphersuite__ResponderCiphersuite'}
# Letfuns re-defined in psq_crypto.pvl (overriding a mis-resolved extraction);
# strip the extracted definition here so the psq_crypto one is the sole def.
STRIP_LETFUN={'libcrux_psq__handshake__ciphersuite__types__Impl_2__from',
              'libcrux_psq__aead__Impl_1__handshake_encrypt'}
stmts=re.split(r'(?<=\.)\n', open(sys.argv[1]).read())
hoist=[]; body=[]
for s in stmts:
    # The engine now prepends Rust doc comments `(* ... *)` before items. Strip
    # any leading comment block when CLASSIFYING the statement (fun/letfun/const)
    # so the statement-start regexes still match; keep `s` intact for output.
    sc=re.sub(r'^\s*(?:\(\*.*?\*\)\s*)+', '', s, flags=re.S)
    ms=re.match(r'\s*fun\s+([A-Za-z0-9_]+)\s*\(.*?\)\s*:\s*bitstring\s*\[data\]\.\s*$', sc, re.S)
    if ms and ms.group(1) in STRIP: continue
    ml=re.match(r'\s*letfun\s+([A-Za-z0-9_]+)\s*\(', sc)
    if ml and ml.group(1) in STRIP_LETFUN: continue
    m=re.match(r'\s*letfun\s+([A-Za-z0-9_]+)\s*\((.*?)\)\s*=', sc, re.S)
    if m and re.search(r'\b'+re.escape(m.group(1))+r'\b', sc[m.end():]):
        ar=m.group(2).count(':') if m.group(2).strip() else 0
        hoist.append(f'fun {m.group(1)}({", ".join(["bitstring"]*ar)}): bitstring.'); continue
    m2=re.search(r'(?m)^fun\s+([A-Za-z0-9_]+)\s*\(([^)]*)\)\s*:\s*bitstring\.\s*$', sc)
    if m2 and '[data]' not in sc and sc.strip().startswith('fun'):
        hoist.append(f'fun {m2.group(1)}({m2.group(2)}): bitstring.'); continue
    body.append(s)
seen=set(); H=[]
for h in hoist:
    n=re.match(r'fun\s+([A-Za-z0-9_]+)',h).group(1)
    if n not in seen: seen.add(n); H.append(h)
sys.stdout.write('(* hoisted opaque serialization stubs *)\n'+'\n'.join(H)+'\n\n'+'\n'.join(body))
PY

# hax >= 0.5.0 renders these trait-method calls as opaque `__unextracted_impl`
# stubs. Map each back to the function the earlier extraction resolved it to;
# `name` and `Impl_2__from` are the overrides in psq_crypto.pvl.
sed -i.bak \
  -e 's/libcrux_psq__handshake__ciphersuite__traits__CiphersuiteBase__name__unextracted_impl(/libcrux_psq__handshake__ciphersuite__traits__CiphersuiteBase__name(/g' \
  -e 's/core__cmp__PartialEq__eq__unextracted_impl(/core__cmp__PartialEq__eq(/g' \
  -e 's/core__convert__From__from__unextracted_impl(/libcrux_psq__handshake__ciphersuite__types__Impl_2__from(/g' \
  -e 's/core__convert__Into__into__unextracted_impl(/core__convert__Into__into(/g' \
  "$EX/lib.clean.pvl" && rm -f "$EX/lib.clean.pvl.bak"

# (b) De-dup the auto-declared `missingdecl` against the real defs
#     (primitives + psq_crypto + the cleaned lib) and mark survivors `[data]`.
python3 - "$PRIM" "$RES" "$PVD/psq_crypto.pvl" "$EX/lib.clean.pvl" "$EX/missingdecl.pvl" > "$EX/missingdecl.dedup.pvl" <<'PY'
import re,sys
defs=set()
for f in sys.argv[1:-1]:
    try: t=open(f).read()
    except: continue
    for m in re.finditer(r'^(?:fun|letfun|const)\s+([A-Za-z0-9_]+)', t, re.M): defs.add(m.group(1))
    for m in re.finditer(r';\s*([A-Za-z0-9_]+)\s*\(', t.replace('\n',' ')): defs.add(m.group(1))
out=[]
for l in open(sys.argv[-1]):
    m=re.match(r'^(fun|const)\s+([A-Za-z0-9_]+)', l)
    if m and m.group(2) in defs: continue
    if l.startswith('fun ') and l.rstrip().endswith(': bitstring.'): l=l.rstrip()[:-1]+' [data].\n'
    out.append(l)
sys.stdout.write(''.join(out))
PY

if [ "${EXTRACT_ONLY:-0}" = 1 ]; then
  echo "EXTRACT_ONLY=1 — extracted and post-processed into $EX/ (lib.pvl,"
  echo "  missingdecl.pvl, lib.clean.pvl, missingdecl.dedup.pvl); ProVerif not run."
  exit 0
fi

# ---------------------------------------------------------------------------
# Run the two query-mode analyses against the composed model and assert the
# query verdicts from psq-design/models/psq_query.pv. The shared setup/roles/
# aliases/nounif live in psq_query_lib.pvl; the queries split into:
#   analysis.pv      full responder (drives read_message_contents): sanity
#                    checks + every property except responder authentication
#   analysis_auth.pv auth-core responder (drives decrypt_outer_message):
#                    ResponderAuth, ResponseInjAgreement
# ResponderAuth/ResponseInjAgreement are responder-authentication
# correspondences; the DH-commutativity model
# does not terminate through the full read_message_contents bookkeeping (rate
# limiter / ciphersuite coercion / state machine), so they run against the
# security-equivalent auth core.
LIBS=(-lib "$PRIM" -lib "$RES" -lib "$PVD/psq_crypto.pvl" -lib "$EX/missingdecl.dedup.pvl"
      -lib "$EX/lib.clean.pvl" -lib "$EX/psq_query_lib.pvl")
# analysis.pv additionally loads the query properties it shares verbatim with
# psq-design/models/psq_query.pv (the queries over QInitSend/QRespRecv and the
# payloads -- see analysis.pv's header); analysis_auth.pv has nothing in common
# with that file (ResponderAuth/ResponseInjAgreement both carry the per-run `er` field), so it stays on the plain LIBS above.
LIBS_MAIN=("${LIBS[@]}" -lib "$PVD/psq_query_shared_queries.pvl")
# Primary verdict per query. Skip ProVerif's secondary `RESULT (but event(...)
# is true.)` annotation that follows a FALSE injective-correspondence query
# (e.g. NoInitiatorInjAgreement, a replay): it is not a separate query verdict.
# A query ProVerif can neither prove nor refute shows up as `cbp` (cannot be
# proved), so it is a visible mismatch instead of silently shifting positions.
verdicts () { grep '^RESULT' "$1" | grep -v '(but' | sed -E 's/.* is (true|false)\..*/\1/; s/.*cannot be proved.*/cbp/' | tr '\n' ' '; }

# If the queries aren't present yet, fall back to a bare load-check.
if [ ! -f "$EX/analysis.pv" ]; then
  printf 'process\n  0\n' > "$EX/loadcheck.pv"
  LOG=$(mktemp); proverif "${LIBS[@]}" "$EX/loadcheck.pv" > "$LOG" 2>&1
  NERR=$(grep -c '^Error:' "$LOG")
  [ "$NERR" -eq 0 ] && echo "MODEL LOADS OK (0 errors)." || { echo "LOAD FAILED"; grep '^Error:' "$LOG"|head; exit 1; }
  echo "(no queries yet)"; exit 0
fi


# ---- Running and judging the analyses ------------------------------------
# Every ProVerif run gets a wall-clock limit of PV_TIMEOUT seconds. A run that
# hits it has no verdict: it is reported as a TOOL LIMITATION (ProVerif did not
# terminate), separately from failures. Every analysis runs with trace
# reconstruction on, and a query ProVerif can neither prove nor refute (`cbp`,
# "cannot be proved") always counts as a failure.
#
# Isolation: each ProVerif run gets its OWN transient systemd scope (optionally
# capped at PV_MEM_LIMIT). If one run exhausts memory, the OOM kill and the
# scope teardown that follows hit only that run, not this script or the
# terminal, so the remaining analyses still run and report. Each verdict is
# printed as soon as its run ends, and every log is kept in PV_LOG_DIR.
PV_TIMEOUT="${PV_TIMEOUT:-14400}"
PV_LOG_DIR="${PV_LOG_DIR:-$SELF/pv-logs/$(date +%Y%m%d-%H%M%S)}"
FAILED=(); NO_VERDICT=()

LIBS_REG=(-lib "$PRIM" -lib "$RES" -lib "$PVD/psq_crypto.pvl" -lib "$EX/missingdecl.dedup.pvl"
          -lib "$EX/lib.clean.pvl" -lib "$EX/psq_reg_lib.pvl")

# ---- The analyses --------------------------------------------------------
# analysis <name> <libs array> <file in extraction/> <expected verdicts> [slow]
# The expected verdicts are `Property=true|false` pairs, one per query, in the
# order ProVerif reports them (the order of the queries in the libs and the
# file). The property names are those of the `(* --- Name. ... *)` comment on
# each query, which match the psq-design specification models.
# "slow" analyses run in the background, in parallel with the others (unless
# PV_SERIAL=1), and are skipped by SKIP_SLOW=1.
NAMES=(); declare -A A_LIBS A_FILE A_EXP A_SLOW
analysis () { NAMES+=("$1"); A_LIBS[$1]=$2; A_FILE[$1]=$3; A_EXP[$1]=$4; A_SLOW[$1]=${5:-}; }

# Query mode. Expected verdicts as in psq_query.pv. analysis.pv's query order
# is: the shared queries (from psq_query_shared_queries.pvl, loaded before it),
# then its own local er-carrying ones; sanity checks first in each file.
analysis query-main LIBS_MAIN analysis.pv "
  SanityQInitSendReachable=false SanityQRespRecvReachable=false
  SanityK0Leaks=false NoInitiatorAuth=false K0Secrecy=true
  QueryConfidentiality=true ResponseConfidentiality=true
  NoQueryForwardSecrecy=false
  SanityQRespSendReachable=false SanityQInitRecvReachable=false
  SanityK2Leaks=false SanitySecretResponseSent=false SanityResponseLeaks=false
  K2ForwardSecrecy=true ResponseForwardSecrecy=true NoInitiatorAuthK2=false"
analysis query-auth LIBS analysis_auth.pv "
  ResponderAuth=true ResponseInjAgreement=true"

# Registration mode. Message-1 analyses (shared lib psq_reg_lib.pvl) drive the
# real extracted registration initiator/responder (auth core), on a sound
# honest run (sanity_reg_passive.pv: InitReg*+RespReg* reachable under a
# PASSIVE attacker with K_1 agreement — the non-vacuity foundation). The
# headline contrast: InitiatorAuthPQ is FALSE in DH mode (DH initiator auth is
# not post-quantum sound) but TRUE in signature mode. The session analyses
# drive the FULL handshake + into_session and check the specification's sanity
# queries (reachability, non-vacuity) under the active attacker; the passive
# session sanity runs also show that the honest run completes on its own. They
# are the heaviest runs (time and memory).
if [ -f "$EX/analysis_reg_dh_msg1.pv" ]; then
  analysis reg-passive-sanity LIBS_REG sanity_reg_passive.pv "
    SanityInitRegDHReachable=false SanityRespRegDHReachable=false
    SanityInitRegSigReachable=false SanityRespRegSigReachable=false
    SanityPassiveK1AgreementDH=true SanityPassiveK1AgreementSig=true"
  analysis reg-dh-msg1 LIBS_REG analysis_reg_dh_msg1.pv "
    SanityRespRegDHReachable=false InitiatorAuthPQ=false
    InitiatorAuthClassical=true NoInitiatorInjAgreement=false K1Secrecy=true
    SanityK1Leaks=false RegPayloadHNDLSecrecy=true
    NoRegPayloadForwardSecrecy=false SanityRegPayloadLeaks=false"
  analysis reg-sig-msg1 LIBS_REG analysis_reg_sig_msg1.pv "
    SanityRespRegSigReachable=false InitiatorAuthPQ=true
    NoInitiatorInjAgreement=false K1Secrecy=true SanityK1Leaks=false
    RegPayloadHNDLSecrecy=true NoRegPayloadForwardSecrecy=false
    SanityRegPayloadLeaks=false"
  analysis reg-dh-pq LIBS_REG analysis_reg_dh_pq.pv "
    SanityRespRegDHReachable=false ForwardInitiatorAuth=true
    ForwardInitiatorAuthTimed=true RegPayloadHNDLSecrecy=true K1Secrecy=true
    SanityRegPayloadLeaks=false"
  analysis session-dh-passive LIBS_REG sanity_session_passive.pv "
    SanityInitSessDHReachable=false SanityRespSessDHReachable=false"
  analysis session-sig-passive LIBS_REG sanity_session_sig_passive.pv "
    SanityInitSessSigReachable=false SanityRespSessSigReachable=false"
  analysis reg-dh-session LIBS_REG analysis_reg_dh_session.pv "
    SanityInitRegDHReachable=false SanityRespRegDHReachable=false
    SanityInitSessDHReachable=false SanityRespSessDHReachable=false
    SanitySessionKeyLeaksInitiator=false SanitySessionKeyLeaksResponder=false
    SanityResponseLeaks=false
    SessionKeySecrecyInitiator=true SessionKeySecrecyResponder=true
    ResponseHNDLSecrecy=true ResponseForwardSecrecy=true" slow
  analysis reg-sig-session LIBS_REG analysis_reg_sig_session.pv "
    SanityInitSessSigReachable=false SanityRespSessSigReachable=false
    SanitySessionKeyLeaksInitiator=false SanitySessionKeyLeaksResponder=false
    SanityResponseLeaks=false
    ResponderAuth=true ResponderInjAgreement=true
    SessionKeySecrecyInitiator=true SessionKeySecrecyResponder=true
    ResponseHNDLSecrecy=true ResponseForwardSecrecy=true" slow
fi

if [ "${LIST:-0}" = 1 ]; then
  for n in "${NAMES[@]}"; do printf '  %-22s %s%s\n' "$n" "${A_FILE[$n]}" "${A_SLOW[$n]:+  (slow)}"; done
  exit 0
fi

# ONLY / SKIP: comma-separated analysis names or globs (e.g. ONLY='reg-*-msg1').
in_list () { local n=$1 p ps; IFS=, read -ra ps <<< "$2"; for p in "${ps[@]}"; do case "$n" in $p) return 0;; esac; done; return 1; }
selected () {
  [ -n "${ONLY:-}" ] && ! in_list "$1" "$ONLY" && return 1
  [ -n "${SKIP:-}" ] && in_list "$1" "$SKIP" && return 1
  [ "${SKIP_SLOW:-0}" = 1 ] && [ -n "${A_SLOW[$1]}" ] && return 1
  return 0
}
RUN=(); for n in "${NAMES[@]}"; do selected "$n" && RUN+=("$n"); done
if [ -n "${ONLY:-}" ]; then
  IFS=, read -ra PATS <<< "$ONLY"
  for p in "${PATS[@]}"; do
    m=0; for n in "${NAMES[@]}"; do case "$n" in $p) m=1;; esac; done
    [ $m = 1 ] || { echo "ONLY: '$p' matches no analysis (LIST=1 lists them)"; exit 1; }
  done
fi
[ ${#RUN[@]} -gt 0 ] || { echo "no analyses selected"; exit 1; }

mkdir -p "$PV_LOG_DIR"
# Everything from here on also goes to $PV_LOG_DIR/summary.txt.
exec > >(tee -a "$PV_LOG_DIR/summary.txt") 2>&1

SCOPE=()
if [ "${PV_NO_SCOPE:-0}" != 1 ] && systemd-run --user --scope -q true 2>/dev/null; then
  SCOPE=(systemd-run --user --scope -q --collect)
  # Without swap capping, a capped run would swap instead of being stopped.
  [ -n "${PV_MEM_LIMIT:-}" ] && SCOPE+=(-p "MemoryMax=$PV_MEM_LIMIT" -p MemorySwapMax=0)
  SCOPE+=(--)
else
  echo "WARNING: no systemd user scope available; an out-of-memory kill may take the whole script down."
  [ -n "${PV_MEM_LIMIT:-}" ] && echo "WARNING: PV_MEM_LIMIT ignored without systemd-run."
fi
echo "Running: ${RUN[*]}"
echo "Logs: $PV_LOG_DIR  (timeout ${PV_TIMEOUT}s${PV_MEM_LIMIT:+, memory limit $PV_MEM_LIMIT per run})"

run_pv () {   # run_pv <name>: run one analysis into $PV_LOG_DIR/<name>.log
  local name=$1 log="$PV_LOG_DIR/$1.log" rc
  local -n libs=${A_LIBS[$1]}
  "${SCOPE[@]}" timeout "$PV_TIMEOUT" proverif "${libs[@]}" "$EX/${A_FILE[$name]}" > "$log" 2>&1
  rc=$?
  case $rc in
    0)   ;;
    124) echo "PV-TIMEOUT" >> "$log" ;;
    137) echo "PV-KILLED" >> "$log" ;;     # SIGKILL: almost always the OOM killer
    *)   echo "PV-EXIT $rc" >> "$log" ;;
  esac
  return 0
}
# show_verdicts <name> <got verdicts>: one line per expected property, marking
# each as ok / MISMATCH / missing. Returns non-zero unless every property is ok
# and ProVerif reported no extra results.
show_verdicts () {
  local name=$1 exps gots i prop want have mark bad=0
  read -d '' -ra exps <<< "${A_EXP[$name]}"; read -d '' -ra gots <<< "$2"
  for i in "${!exps[@]}"; do
    prop=${exps[$i]%%=*}; want=${exps[$i]#*=}; have=${gots[$i]:-}
    if [ -z "$have" ]; then mark="--"; have="(no result)"; bad=1
    elif [ "$have" = "$want" ]; then mark="ok"
    else mark="!!"; bad=1; fi
    [ "$have" = cbp ] && have="cannot be proved"
    printf '    %s %-30s %-6s (expected %s)\n' "$mark" "$prop" "$have" "$want"
  done
  if [ ${#gots[@]} -gt ${#exps[@]} ]; then
    echo "    !! ProVerif reported ${#gots[@]} results, but ${#exps[@]} properties are expected"; bad=1
  fi
  return $bad
}
check () {    # check <name>
  local label=$1 log="$PV_LOG_DIR/$1.log" got
  if grep -q '^Error' "$log"; then
    echo "  $label: LOAD FAILED"; grep '^Error' "$log" | head -3 | sed 's/^/    /'
    FAILED+=("$label"); return
  fi
  got=$(verdicts "$log")
  if grep -qx 'PV-TIMEOUT' "$log"; then
    echo "  $label: TIMEOUT after ${PV_TIMEOUT}s — tool limitation (ProVerif did not terminate), no verdict"
    show_verdicts "$label" "$got"
    NO_VERDICT+=("$label (timeout)"); return
  fi
  if grep -qx 'PV-KILLED' "$log" || grep -qi 'out of memory' "$log"; then
    echo "  $label: KILLED (most likely out of memory${PV_MEM_LIMIT:+, limit $PV_MEM_LIMIT}) — tool limitation, no verdict"
    show_verdicts "$label" "$got"
    NO_VERDICT+=("$label (killed/OOM)"); return
  fi
  if grep -q '^PV-EXIT' "$log"; then
    echo "  $label: ProVerif exited abnormally ($(grep '^PV-EXIT' "$log")); see $log"
    FAILED+=("$label"); return
  fi
  if show_verdicts "$label" "$got" > "$log.verdicts"; then
    echo "  $label: all as expected"
  else
    echo "  $label: MISMATCH (\"cannot be proved\" counts as a failure)"
    FAILED+=("$label")
  fi
  cat "$log.verdicts"; rm -f "$log.verdicts"
}

# PV_RECHECK=1: judge the logs already in PV_LOG_DIR (e.g. after changing the
# expected verdicts) instead of running ProVerif.
if [ "${PV_RECHECK:-0}" = 1 ]; then
  for n in "${RUN[@]}"; do
    if [ -f "$PV_LOG_DIR/$n.log" ]; then check "$n"; else echo "  $n: no log in $PV_LOG_DIR"; FAILED+=("$n"); fi
  done
  RUN=()
fi

# Slow analyses start first, in the background, so they overlap the rest
# (PV_SERIAL=1 runs everything one at a time, to bound peak memory).
declare -A PID
for n in "${RUN[@]}"; do
  [ -n "${A_SLOW[$n]}" ] && [ "${PV_SERIAL:-0}" != 1 ] && { run_pv "$n" & PID[$n]=$!; }
done
for n in "${RUN[@]}"; do
  [ -n "${PID[$n]:-}" ] && continue
  run_pv "$n"; check "$n"
done
for n in "${RUN[@]}"; do
  [ -n "${PID[$n]:-}" ] || continue
  echo "  (waiting on $n ...)"; wait "${PID[$n]}"; check "$n"
done

join () { local IFS=','; echo "$*" | sed 's/,/, /g'; }
echo
echo "Logs: $PV_LOG_DIR"
if [ ${#FAILED[@]} -gt 0 ]; then
  echo "CHECK FAILED: $(join "${FAILED[@]}")"
  [ ${#NO_VERDICT[@]} -gt 0 ] && echo "  also without a verdict (tool limitation): $(join "${NO_VERDICT[@]}")"
  exit 1
fi
if [ ${#NO_VERDICT[@]} -gt 0 ]; then
  echo "CHECK INCOMPLETE: no failures, but without a verdict (tool limitation): $(join "${NO_VERDICT[@]}")"
  exit 2
fi
echo "CHECK PASSED: every selected analysis matched its expected verdicts."
