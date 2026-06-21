#!/usr/bin/env bash
# Re-extract the libcrux-psq PSQ handshake to ProVerif via the hax
# proverif-rust backend, compose it with the symbolic crypto model
# (psq_crypto.pvl) and the de-duplicated `missingdecl`, and run ProVerif.
#
# Phase 2 status: this validates that the QUERY-MODE model (initiator + responder)
# LOADS and type-checks (no parse/type error). Security queries (Phase 3) are
# added in proofs/proverif/extraction/analysis.pv once present.
#
#   HAX_PROVERIF_DIR : a hax checkout @ proverif-rust-backend with target/release
#                      built (or the hax-proverif opam switch installed).
set -uo pipefail
# script lives at libcrux-psq/proofs/proverif/check-pv.sh; run from workspace root
cd "$(dirname "$0")/../../.."
ENG="${HAX_PROVERIF_DIR:?set HAX_PROVERIF_DIR to a hax checkout}"
PRIM="$ENG/hax-lib/proof-libs/proverif/primitives.pvl"
PVD=libcrux-psq/proofs/proverif
EX="$PVD/extraction"
eval "$(opam env --switch=hax-proverif 2>/dev/null)" 2>/dev/null || true
if ! command -v cargo-hax >/dev/null 2>&1; then export PATH="$ENG/target/release:$PATH"; fi
export HAX_RUST_ENGINE_BINARY="${HAX_RUST_ENGINE_BINARY:-$ENG/target/release/hax-rust-engine}"

# Query-mode entry points: the query initiator + the (unified) responder,
# transitively. The crypto boundary is annotated in src/** with
# `#[hax_lib::proverif::replace_body(...)]` (DH / KDF / AEAD) and modeled in
# psq_crypto.pvl; serialization is bypassed there.
INC='-** +~libcrux_psq::handshake::initiator::** +~libcrux_psq::handshake::responder::**'
cargo hax -C -p libcrux-psq --features hax-pv ';' into -i "$INC" proverif || exit 1

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
    ms=re.match(r'\s*fun\s+([A-Za-z0-9_]+)\s*\(.*?\)\s*:\s*bitstring\s*\[data\]\.\s*$', s, re.S)
    if ms and ms.group(1) in STRIP: continue
    ml=re.match(r'\s*letfun\s+([A-Za-z0-9_]+)\s*\(', s)
    if ml and ml.group(1) in STRIP_LETFUN: continue
    m=re.match(r'\s*letfun\s+([A-Za-z0-9_]+)\s*\((.*?)\)\s*=', s, re.S)
    if m and re.search(r'\b'+re.escape(m.group(1))+r'\b', s[m.end():]):
        ar=m.group(2).count(':') if m.group(2).strip() else 0
        hoist.append(f'fun {m.group(1)}({", ".join(["bitstring"]*ar)}): bitstring.'); continue
    m2=re.search(r'(?m)^fun\s+([A-Za-z0-9_]+)\s*\(([^)]*)\)\s*:\s*bitstring\.\s*$', s)
    if m2 and '[data]' not in s and s.strip().startswith(('fun','(* self-recursive')):
        hoist.append(f'fun {m2.group(1)}({m2.group(2)}): bitstring.'); continue
    body.append(s)
seen=set(); H=[]
for h in hoist:
    n=re.match(r'fun\s+([A-Za-z0-9_]+)',h).group(1)
    if n not in seen: seen.add(n); H.append(h)
sys.stdout.write('(* hoisted opaque serialization stubs *)\n'+'\n'.join(H)+'\n\n'+'\n'.join(body))
PY

# (b) De-dup the auto-declared `missingdecl` against the real defs
#     (primitives + psq_crypto + the cleaned lib) and mark survivors `[data]`.
python3 - "$PRIM" "$PVD/psq_crypto.pvl" "$EX/lib.clean.pvl" "$EX/missingdecl.pvl" > "$EX/missingdecl.dedup.pvl" <<'PY'
import re,sys
defs=set()
for f in sys.argv[1:4]:
    try: t=open(f).read()
    except: continue
    for m in re.finditer(r'^(?:fun|letfun|const)\s+([A-Za-z0-9_]+)', t, re.M): defs.add(m.group(1))
    for m in re.finditer(r';\s*([A-Za-z0-9_]+)\s*\(', t.replace('\n',' ')): defs.add(m.group(1))
out=[]
for l in open(sys.argv[4]):
    m=re.match(r'^(fun|const)\s+([A-Za-z0-9_]+)', l)
    if m and m.group(2) in defs: continue
    if l.startswith('fun ') and l.rstrip().endswith(': bitstring.'): l=l.rstrip()[:-1]+' [data].\n'
    out.append(l)
sys.stdout.write(''.join(out))
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
# security-equivalent auth core (cf. mandrake's *_core files).
LIBS=(-lib "$PRIM" -lib "$PVD/psq_crypto.pvl" -lib "$EX/missingdecl.dedup.pvl"
      -lib "$EX/lib.clean.pvl" -lib "$EX/psq_query_lib.pvl")
# Primary verdict per query. Skip ProVerif's secondary `RESULT (but event(...)
# is true.)` annotation that follows a FALSE injective-correspondence query
# (e.g. R2c replay): it is not a separate query verdict.
verdicts () { grep '^RESULT' "$1" | grep -v '(but' | grep -oE 'is (true|false)' | awk '{print $2}' | tr '\n' ' '; }

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
# psq_registration_{dh,sig}_msg1.pv EXACTLY, including R2c (replay) and R9 (NOT
# forward-secret), which now resolve correctly (the sound honest run + the wire
# leak fixes — CiphersuiteBase::name field-projection and the AuthMessageOut::Sig
# .into() canon in psq_crypto.pvl — let ProVerif reconstruct the honest K_1, so
# R2c/R9 come out FALSE rather than over-approximating to true):
#   DH  : R1 reachable, R2a FALSE (DH initiator-auth NOT post-quantum sound),
#         R2b true, R2c false, R4 true, R6 true, R9 false, nonvac false
#                                              -> false false true false true true false false
#   sig : R1 reachable, R2a TRUE (signature-based auth IS PQ-sound),
#         R2c false, R4 true, R6 true, R9 false, nonvac false
#                                              -> false true false true true false false
# R9-false reconstructs reg_secret under endpoint compromise (no ephemeral break),
# doubling as the leak / non-vacuity control for the secrecy queries R4/R6.
LIBS_REG=(-lib "$PRIM" -lib "$PVD/psq_crypto.pvl" -lib "$EX/missingdecl.dedup.pvl"
          -lib "$EX/lib.clean.pvl" -lib "$EX/psq_reg_lib.pvl")
REG_OK=1
if [ -f "$EX/analysis_reg_dh_msg1.pv" ]; then
  LOG_SAN=$(mktemp); LOG_RDH=$(mktemp); LOG_RSIG=$(mktemp)
  # Passive-first sanity: the honest registration round-trip must complete on its
  # own (no attacker help). Without this the active-attacker verdicts are unsound.
  proverif "${LIBS_REG[@]}" "$EX/sanity_reg_passive.pv"    > "$LOG_SAN"  2>&1
  proverif "${LIBS_REG[@]}" "$EX/analysis_reg_dh_msg1.pv"  > "$LOG_RDH"  2>&1
  proverif "${LIBS_REG[@]}" "$EX/analysis_reg_sig_msg1.pv" > "$LOG_RSIG" 2>&1
  NERR=$(( $(grep -c '^Error:' "$LOG_SAN") + $(grep -c '^Error:' "$LOG_RDH") + $(grep -c '^Error:' "$LOG_RSIG") ))
  if [ "$NERR" -ne 0 ]; then echo "REG LOAD FAILED ($NERR errors):"; grep '^Error:' "$LOG_SAN" "$LOG_RDH" "$LOG_RSIG" | head; REG_OK=0; fi
  EXP_SAN="false false false false true true "   # InitReg*/RespReg* reachable + K_1 agreement
  EXP_RDH="false false true false true true false false "
  EXP_RSIG="false true false true true false false "
  GOT_SAN=$(verdicts "$LOG_SAN"); GOT_RDH=$(verdicts "$LOG_RDH"); GOT_RSIG=$(verdicts "$LOG_RSIG")
  echo "  reg passive sanity got: $GOT_SAN"
  echo "                     exp: $EXP_SAN"
  echo "  reg dh msg1 got: $GOT_RDH"
  echo "              exp: $EXP_RDH"
  echo "  reg sig msg1 got: $GOT_RSIG"
  echo "              exp: $EXP_RSIG"
  [ "$GOT_SAN" = "$EXP_SAN" ] && [ "$GOT_RDH" = "$EXP_RDH" ] && [ "$GOT_RSIG" = "$EXP_RSIG" ] || REG_OK=0
fi

if [ "$QUERY_OK" = 1 ] && [ "$REG_OK" = 1 ]; then
  echo "CHECK PASSED (query 14/14 + registration msg1 R1/R2a/R2b/R2c/R4/R6/R9: DH R2a false / sig R2a true, replay+fwd-sec both false)"
else
  echo "CHECK FAILED"; exit 1
fi
