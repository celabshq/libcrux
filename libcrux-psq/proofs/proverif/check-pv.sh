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
INC='-** +~libcrux_psq::handshake::initiator::query::** +~libcrux_psq::handshake::responder::**'
cargo hax -C -p libcrux-psq --features hax-pv ';' into -i "$INC" proverif || exit 1

# (a) Neutralize + hoist self-recursive serialization stubs. hax renders
#     unresolvable tls_codec trait methods as `f(..) = f(..)`; the engine
#     converts the bare form to an opaque `fun`, this pass catches the
#     let-wrapped form too and hoists ALL opaque stub funs to the top (they are
#     dependency-free; this fixes the mutual-recursion forward references).
python3 - "$EX/lib.pvl" > "$EX/lib.clean.pvl" <<'PY'
import re,sys
# Constructors re-declared in psq_crypto.pvl (so the pv_deserialize bridge can
# reference them before lib.pvl loads); strip their `fun … [data].` decls here
# to avoid a duplicate definition. Their projector reducs + use sites stay and
# resolve against the psq_crypto declaration (loaded earlier).
STRIP={'libcrux_psq__handshake__initiator__InitiatorOuterPayloadOut__Query',
       'libcrux_psq__handshake__responder__InitiatorOuterPayload__Query'}
stmts=re.split(r'(?<=\.)\n', open(sys.argv[1]).read())
hoist=[]; body=[]
for s in stmts:
    ms=re.match(r'\s*fun\s+([A-Za-z0-9_]+)\s*\(\s*bitstring\s*\)\s*:\s*bitstring\s*\[data\]\.\s*$', s, re.S)
    if ms and ms.group(1) in STRIP: continue
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
verdicts () { grep '^RESULT' "$1" | grep -oE 'is (true|false)' | awk '{print $2}' | tr '\n' ' '; }

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
if [ "$GOT_MAIN" = "$EXP_MAIN" ] && [ "$GOT_AUTH" = "$EXP_AUTH" ]; then
  echo "CHECK PASSED (14/14 query-mode verdicts match psq_query.pv)"
else
  echo "CHECK FAILED"; exit 1
fi
