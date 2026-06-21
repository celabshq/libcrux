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
stmts=re.split(r'(?<=\.)\n', open(sys.argv[1]).read())
hoist=[]; body=[]
for s in stmts:
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

# Use analysis.pv (the security queries) if present; else a bare load-check.
ANALYSIS="$EX/analysis.pv"
[ -f "$ANALYSIS" ] || { printf 'process\n  0\n' > "$EX/loadcheck.pv"; ANALYSIS="$EX/loadcheck.pv"; }
LOG=$(mktemp)
proverif -lib "$PRIM" -lib "$PVD/psq_crypto.pvl" -lib "$EX/missingdecl.dedup.pvl" \
         -lib "$EX/lib.clean.pvl" "$ANALYSIS" > "$LOG" 2>&1
NERR=$(grep -c '^Error:' "$LOG")
if [ "$NERR" -ne 0 ]; then echo "LOAD FAILED ($NERR errors):"; grep '^Error:' "$LOG" | head; exit 1; fi
echo "MODEL LOADS OK (0 errors)."
grep -E '^RESULT' "$LOG" || echo "(no queries yet — Phase 3)"
