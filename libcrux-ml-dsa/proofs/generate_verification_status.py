#!/usr/bin/env python3
"""Generate per-crate verification_status.md from Rust source annotations and the F* Makefile.

Reusable across libcrux-ml-kem, libcrux-ml-dsa, libcrux-sha3 via a JSON config file
(verification_status.config.json) that sits next to the per-crate Makefile.

Classification (highest tier wins; in order from worst to best):
  - lax: module in ADMIT_MODULES, OR fn has #[hax_lib::fstar::verification_status(lax)],
         OR fn has #[hax_lib::fstar::options("--admit_smt_queries true")] pragma,
         OR fn body has an inline admit at non-terminal position.
  - panic_free: not lax, AND (verification_status(panic_free)
                              OR no #[ensures(...)] annotation
                              OR a single TERMINAL inline admit).
  - correct: not lax, has a non-trivial ensures (matches range/spec patterns), and no body admit.

CLI:
  generate_verification_status.py [--root PATH] [--config PATH] [--output PATH]
                                  [--diff PREV CURR] [--all-body-admits-lax]
"""

import argparse
import json
import os
import re
import sys

DEFAULT_ML_KEM_CONFIG = {
    "crate_name": "ML-KEM",
    "src_dir": "src",
    "makefile": "proofs/fstar/extraction/Makefile",
    "extraction_dir": "proofs/fstar/extraction",
    "output": "proofs/verification_status.md",
    "admit_module_prefix": "Libcrux_ml_kem.",
    "spec_patterns": [
        r"Hacspec_ml_kem\.",
        r"Spec\.Utils\.v_G", r"Spec\.Utils\.v_H", r"Spec\.Utils\.v_PRF",
        r"to_spec_poly_t", r"to_spec_vector_t", r"to_spec_matrix_t",
        r"Traits\.Spec\.\w+_post",
    ],
    "range_patterns": [
        r"is_i16b", r"is_bounded_poly", r"is_bounded_vector",
        r"\bbounded\b", r"is_intb", r"is_i32b",
    ],
    "modules": [
        {"category": "Generic", "display": "constant_time_ops", "paths": ["constant_time_ops"]},
        {"category": "Generic", "display": "hash_functions", "paths": ["hash_functions"]},
        {"category": "Generic", "display": "ind_cpa", "paths": ["ind_cpa"]},
        {"category": "Generic", "display": "ind_cca", "paths": ["ind_cca"]},
        {"category": "Generic", "display": "instantiations",
         "paths": ["ind_cca/instantiations", "ind_cca/instantiations/avx2"]},
        {"category": "Generic", "display": "multiplexing",
         "paths": ["ind_cca/multiplexing", "ind_cca/incremental/multiplexing"]},
        {"category": "Generic", "display": "polynomial", "paths": ["polynomial"]},
        {"category": "Generic", "display": "invert_ntt", "paths": ["invert_ntt"]},
        {"category": "Generic", "display": "ntt", "paths": ["ntt"]},
        {"category": "Generic", "display": "mlkem*",
         "paths": ["mlkem", "mlkem512", "mlkem768", "mlkem1024"]},
        {"category": "Generic", "display": "matrix", "paths": ["matrix"]},
        {"category": "Generic", "display": "serialize", "paths": ["serialize"]},
        {"category": "Generic", "display": "sampling", "paths": ["sampling"]},
        {"category": "Portable", "display": "arithmetic", "paths": ["vector/portable/arithmetic"]},
        {"category": "Portable", "display": "ntt", "paths": ["vector/portable/ntt"]},
        {"category": "Portable", "display": "serialize", "paths": ["vector/portable/serialize"]},
        {"category": "Portable", "display": "compress", "paths": ["vector/portable/compress"]},
        {"category": "Portable", "display": "sampling", "paths": ["vector/portable/sampling"]},
        {"category": "Portable", "display": "vector", "paths": ["vector/portable"]},
        {"category": "Avx2", "display": "arithmetic", "paths": ["vector/avx2/arithmetic"]},
        {"category": "Avx2", "display": "ntt", "paths": ["vector/avx2/ntt"]},
        {"category": "Avx2", "display": "serialize", "paths": ["vector/avx2/serialize"]},
        {"category": "Avx2", "display": "compress", "paths": ["vector/avx2/compress"]},
        {"category": "Avx2", "display": "sampling", "paths": ["vector/avx2/sampling"]},
        {"category": "Avx2", "display": "vector", "paths": ["vector/avx2"]},
        {"category": "Neon", "display": "arithmetic", "paths": ["vector/neon/arithmetic"]},
        {"category": "Neon", "display": "ntt", "paths": ["vector/neon/ntt"]},
        {"category": "Neon", "display": "compress", "paths": ["vector/neon/compress"]},
        {"category": "Neon", "display": "serialize", "paths": ["vector/neon/serialize"]},
        {"category": "Neon", "display": "sampling", "paths": ["vector/neon/sampling"]},
    ],
}


# ============================================================================
# Text scanning utilities
# ============================================================================

def find_matching_bracket(text, start, open_ch="[", close_ch="]"):
    """From position `start` (which should point AT the open bracket),
    find the position after the matching close bracket.
    Skips raw strings r#"..."# and regular strings "...".
    Returns (end_pos, substring) or (-1, "") if not found."""
    depth = 0
    i = start
    while i < len(text):
        if text[i:i+3] == 'r#"':
            end = text.find('"#', i + 3)
            if end == -1:
                return -1, ""
            i = end + 2
            continue
        if text[i] == '"':
            i += 1
            while i < len(text) and text[i] != '"':
                if text[i] == '\\':
                    i += 1
                i += 1
            i += 1
            continue
        if text[i:i+2] == '//' and (i < 2 or text[i-1] != '*'):
            nl = text.find('\n', i)
            if nl == -1:
                break
            i = nl + 1
            continue
        if text[i:i+2] == '/*':
            end = text.find('*/', i + 2)
            if end == -1:
                break
            i = end + 2
            continue
        if text[i] == open_ch:
            depth += 1
        elif text[i] == close_ch:
            depth -= 1
            if depth == 0:
                return i + 1, text[start:i + 1]
        i += 1
    return -1, ""


# ============================================================================
# Config loading
# ============================================================================

def load_config(config_path, root):
    """Load JSON config from disk. Falls back to root/proofs/verification_status.config.json,
    then to the baked-in ML-KEM default."""
    if config_path:
        with open(config_path) as f:
            return json.load(f)
    fallback = os.path.join(root, "proofs", "verification_status.config.json")
    if os.path.isfile(fallback):
        with open(fallback) as f:
            return json.load(f)
    return DEFAULT_ML_KEM_CONFIG


# ============================================================================
# ADMIT_MODULES parsing
# ============================================================================

def _parse_makefile_var(makefile_path, var_name, prefix):
    """Generic helper: read a Makefile variable that lists `prefix`-prefixed
    module names (one per line, possibly continued with `\\`), and return them
    as a set of `src/<path>.rs` paths.

    Mirrors `list_extracted_modules`'s name-mangling treatment: strips a single
    trailing `_` from each segment (hax appends it to F* identifiers ending in
    a digit)."""
    if not os.path.isfile(makefile_path):
        return set()
    paths = set()
    in_var = False
    with open(makefile_path) as f:
        for line in f:
            if line.startswith(var_name):
                in_var = True
            if in_var:
                for token in line.split():
                    if token.startswith(prefix):
                        mod = token[len(prefix):]
                        mod = mod.removesuffix(".fst").removesuffix(".fsti")
                        segments = [s.rstrip('_') for s in mod.split('.')]
                        path = '/'.join(s.lower() for s in segments)
                        paths.add(f"src/{path}.rs")
                if not line.rstrip().endswith("\\"):
                    in_var = False
    return paths


def parse_admit_modules(makefile_path, prefix, extracted_paths=None):
    """Read the F* extraction Makefile and return the set of source paths
    classified as admitted.

    Two Makefile patterns are supported:
      1. Explicit `ADMIT_MODULES = ...` list (e.g. ml-kem). Direct read.
      2. Inverted list: `VERIFIED_MODULES = ...` plus
         `ADMIT_MODULES = $(filter-out ${VERIFIED_OR_SLOW_MODULES}, $(wildcard *.fst))`
         (e.g. ml-dsa). Admit = `extracted_paths` − `VERIFIED_MODULES` − `SLOW_MODULES`.
         Caller must supply `extracted_paths` for this case to work.
    """
    explicit = _parse_makefile_var(makefile_path, "ADMIT_MODULES", prefix)
    verified = _parse_makefile_var(makefile_path, "VERIFIED_MODULES", prefix)
    if verified and extracted_paths is not None:
        slow = _parse_makefile_var(makefile_path, "SLOW_MODULES", prefix)
        return extracted_paths - verified - slow
    return explicit


def list_extracted_modules(extraction_dir, prefix, src_dir=None):
    """Scan the F* extraction directory and return the set of `src/<path>.rs`
    paths covered by an extracted F* module (.fst or .fsti).

    Coverage rules:
      * Direct: `Libcrux_<crate>.Foo.Bar.fst` → `src/foo/bar.rs`.
      * Trailing-underscore mangling: hax appends `_` to F* segments whose
        Rust identifier ends in a digit (e.g. `Ml_dsa_44_`). We strip a
        single trailing `_` from each segment when reverse-mapping.
      * Ancestor coverage: if `Libcrux_<crate>.Foo.Bar.Baz.fst` exists and
        `src/foo/bar.rs` is a real file (e.g. parent module that uses a
        hax macro to generate per-variant submodules), `src/foo/bar.rs` is
        also marked extracted. Requires `src_dir` to be passed.

    A Rust module that ends up NOT in this set was filtered out of the
    extraction via `-i -<module>::**` in hax.py and is unverified."""
    if not os.path.isdir(extraction_dir):
        return set()
    extracted = set()
    for fname in os.listdir(extraction_dir):
        if not fname.startswith(prefix):
            continue
        if not (fname.endswith('.fst') or fname.endswith('.fsti')):
            continue
        mod = fname[len(prefix):]
        mod = mod.removesuffix('.fsti').removesuffix('.fst')
        # Strip trailing-underscore mangling per-segment.
        segments = [s.rstrip('_') for s in mod.split('.')]
        # Direct mapping
        leaf = '/'.join(s.lower() for s in segments)
        extracted.add(f"src/{leaf}.rs")
        # Ancestor coverage: walk up the segment list, register parent
        # `.rs` files that actually exist on disk.
        if src_dir is not None:
            for n in range(len(segments) - 1, 0, -1):
                anc = '/'.join(s.lower() for s in segments[:n])
                anc_path = os.path.join(src_dir, f"{anc}.rs")
                if os.path.isfile(anc_path):
                    extracted.add(f"src/{anc}.rs")
    return extracted


# ============================================================================
# Per-file parser
# ============================================================================

FN_RE = re.compile(
    r"^\s*(pub(\([a-z]+\))?\s+)?(const\s+)?(async\s+)?(unsafe\s+)?fn\s"
)
# Non-fn item declarations — cause us to clear pending attributes so they
# don't drift onto a fn that appears later. We do NOT apply per-fn attributes
# (verification_status / opaque / options-pragma) to these.
NON_FN_ITEM_RE = re.compile(
    r"^\s*(pub(\([a-z]+\))?\s+)?"
    r"(unsafe\s+)?"
    r"(struct|enum|union|type|impl|trait|mod|static|const)\s"
)
VSTATUS_RE = re.compile(r"verification_status\((lax|panic_free)\)")
ADMIT_PRAGMA_RE = re.compile(r"--admit_smt_queries\s+true")
INLINE_ADMIT_RE = re.compile(r"\badmit\s*\(\s*\)|admit_smt_queries\s+true")
# `#[hax_lib::opaque]` (without args, no `_to_smt` suffix) — applied only to fns.
# `\b` word-boundary won't match inside `opaque_to_smt` because `_` is a word char,
# so this regex correctly excludes that variant.
OPAQUE_ATTR_RE = re.compile(r"\bhax_lib::opaque\b")


def classify_ensures(text, spec_re, range_re):
    """Return the highest proof tier reached by an ensures annotation:
       - 'hacspec': ensures cites the high-level mathematical spec (Spec.MLKEM/MLDSA/...).
       - 'bounds':  ensures uses range/interval predicates (is_i16b, is_bounded_*, ...).
       - 'math':    ensures present but doesn't match spec or bounds patterns
                    (proves SOME non-trivial property, but neither a bound nor spec equivalence).
    """
    if spec_re.search(text):
        return "hacspec"
    if range_re.search(text):
        return "bounds"
    return "math"


def _strip_fstar_comments(content):
    """Remove F* line/block comments from a fstar!() macro's content so we don't
    false-match `admit ()` text inside an explanatory comment."""
    # F* line comments: // ...
    content = re.sub(r'//[^\n]*', '', content)
    # F* block comments: (* ... *) — non-greedy, multi-line.
    content = re.sub(r'\(\*.*?\*\)', '', content, flags=re.DOTALL)
    return content


def has_body_admit(text, body_open_pos, body_close_pos):
    """Return True if the function body contains an inline admit inside a
    hax_lib::fstar! block. F* comments inside the macro are stripped first."""
    macro_re = re.compile(r"\b(?:hax_lib::)?fstar\s*!\s*\(")
    for m in macro_re.finditer(text, body_open_pos, body_close_pos):
        paren_pos = m.end() - 1  # position of `(`
        end_paren, content = find_matching_bracket(text, paren_pos, '(', ')')
        if end_paren <= 0:
            continue
        cleaned = _strip_fstar_comments(content)
        if INLINE_ADMIT_RE.search(cleaned):
            return True
    return False


def _find_fn_body_brace(text, fn_start):
    """Find the position of the function-body opening `{` after `fn_start`.
    Skips angle-brackets `<>` and parens `()` for generics and params.
    Returns position of `{`, or None if a `;` (bare signature) or EOF is reached first."""
    angle_depth = 0
    paren_depth = 0
    i = fn_start
    while i < len(text):
        if text[i:i+3] == 'r#"':
            end = text.find('"#', i + 3)
            if end == -1:
                return None
            i = end + 2
            continue
        if text[i] == '"':
            i += 1
            while i < len(text) and text[i] != '"':
                if text[i] == '\\':
                    i += 1
                i += 1
            i += 1
            continue
        if text[i:i+2] == '//':
            nl = text.find('\n', i)
            if nl == -1:
                return None
            i = nl + 1
            continue
        if text[i:i+2] == '/*':
            end = text.find('*/', i + 2)
            if end == -1:
                return None
            i = end + 2
            continue
        c = text[i]
        if c == '<':
            angle_depth += 1
        elif c == '>':
            if angle_depth > 0:
                angle_depth -= 1
        elif c == '(':
            paren_depth += 1
        elif c == ')':
            if paren_depth > 0:
                paren_depth -= 1
        elif c == '{' and angle_depth == 0 and paren_depth == 0:
            return i
        elif c == ';' and angle_depth == 0 and paren_depth == 0:
            return None  # bare signature, no body
        i += 1
    return None


def parse_file(filepath, spec_re, range_re):
    """Parse a Rust source file, returning a list of per-function dicts:
       { 'line': int, 'vstatus': 'lax'|'panic_free'|None,
         'ensures_level': 'spec'|'range'|'panic_free'|None,
         'body_admit': bool }
    Body admits (options-pragma OR inline `admit ()`) all classify as lax."""
    with open(filepath) as f:
        text = f.read()
    lines = text.split('\n')
    line_offsets = []
    offset = 0
    for line in lines:
        line_offsets.append(offset)
        offset += len(line) + 1

    functions = []
    pending_vstatus = None
    pending_options_admit = False
    pending_opaque = False
    ensures_text = ""
    skip_until = 0

    def reset_pending():
        nonlocal pending_vstatus, pending_options_admit, pending_opaque, ensures_text
        pending_vstatus = None
        pending_options_admit = False
        pending_opaque = False
        ensures_text = ""

    for lineno, line in enumerate(lines):
        line_start = line_offsets[lineno]
        stripped = line.rstrip()

        if line_start < skip_until:
            continue

        # Skip fstar::before(...) and fstar::after(...) blocks (their content
        # may contain admits or ensures-keywords inside F* lemma bodies).
        skipped = False
        for skip_pat in ['fstar::before(', 'fstar::after(']:
            idx = stripped.find(skip_pat)
            if idx >= 0:
                paren_pos = line_start + idx + len(skip_pat) - 1
                end_pos, _ = find_matching_bracket(text, paren_pos, '(', ')')
                if end_pos > 0:
                    skip_until = end_pos
                skipped = True
                break
        if skipped:
            continue

        # `#[hax_lib::opaque]` attribute (function-only — see NON_FN_ITEM_RE handling).
        # Detected separately because it's a standalone-bracket attribute with no args.
        opaque_match = re.search(r'#\[hax_lib::opaque\]', stripped)
        if opaque_match:
            pending_opaque = True
            continue

        # verification_status attribute
        m = VSTATUS_RE.search(stripped)
        if m:
            pending_vstatus = m.group(1)
            continue

        # options pragma — check for --admit_smt_queries true
        if 'fstar::options' in stripped:
            attr_start = stripped.find('#[')
            if attr_start >= 0:
                attr_bracket_pos = line_start + attr_start + 1
                end_pos, attr_text = find_matching_bracket(text, attr_bracket_pos, '[', ']')
                if end_pos > 0:
                    if ADMIT_PRAGMA_RE.search(attr_text):
                        pending_options_admit = True
                    skip_until = end_pos
            continue

        # ensures attribute
        ensures_match = re.search(r'#\[(hax_lib::)?ensures\(', stripped)
        if ensures_match:
            if re.match(r"\s*//", stripped):
                continue
            hash_bracket = stripped.find('#[', ensures_match.start())
            if hash_bracket == -1:
                hash_bracket = ensures_match.start()
            attr_bracket_pos = line_start + hash_bracket + 1
            end_pos, attr_text = find_matching_bracket(text, attr_bracket_pos, '[', ']')
            if end_pos > 0:
                ensures_text = attr_text
                skip_until = end_pos
            continue

        # function definition — check BEFORE NON_FN_ITEM_RE so `const fn` /
        # `async fn` aren't mistaken for `const` / non-fn items.
        if FN_RE.match(stripped):
            ensures_level = classify_ensures(ensures_text, spec_re, range_re) if ensures_text else None

            body_admit = False
            if pending_options_admit:
                body_admit = True
            else:
                fn_brace = _find_fn_body_brace(text, line_start)
                if fn_brace is not None:
                    body_close, _ = find_matching_bracket(text, fn_brace, '{', '}')
                    if body_close > 0:
                        body_admit = has_body_admit(text, fn_brace, body_close)

            # `#[hax_lib::opaque]` is a function-only marker meaning the body
            # is intentionally hidden from F* — equivalent to lax.
            functions.append({
                'line': lineno + 1,
                'vstatus': pending_vstatus,
                'ensures_level': ensures_level,
                'body_admit': body_admit,
                'opaque': pending_opaque,
            })

            reset_pending()
            continue

        # Non-fn item — clear pending fn-only attributes so they don't drift
        # onto the next fn we encounter.
        if NON_FN_ITEM_RE.match(stripped):
            reset_pending()

    return functions


# ============================================================================
# Accounting
# ============================================================================

def compute_module_stats(funcs, in_admit_module, is_unverified):
    """Per-module classification accounting.

    Each function is classified at exactly ONE proof tier (highest wins):
      - lax        : admitted (vstatus=lax, body admit, options-pragma admit, opaque)
      - unverified : Rust module has no F* extraction (filtered out by hax)
      - hacspec    : ensures matches spec_patterns (cites high-level mathematical spec)
      - bounds     : ensures matches range_patterns (bounds/interval predicates only)
      - math       : ensures present but matches neither pattern (some non-trivial property)
      - panic_free : no ensures at all, or vstatus=panic_free explicitly

    The `panic_safe` aggregate = panic_free + math + bounds + hacspec
    (everything proven free of panics).
    """
    base = {
        'total': len(funcs),
        'lax': 0, 'unverified': 0,
        'panic_free': 0, 'math': 0, 'bounds': 0, 'hacspec': 0,
        'body_admit_sites': [],
    }
    if is_unverified:
        base['unverified'] = len(funcs)
        return base
    if in_admit_module:
        base['lax'] = len(funcs)
        return base

    body_admit_sites = []

    for fn in funcs:
        is_lax = fn['vstatus'] == 'lax' or fn['body_admit'] or fn.get('opaque', False)
        if is_lax:
            base['lax'] += 1
            if fn['body_admit']:
                body_admit_sites.append(fn['line'])
            continue

        # vstatus=panic_free or no ensures → panic_free (lowest verified tier)
        if fn['vstatus'] == 'panic_free' or fn['ensures_level'] is None:
            base['panic_free'] += 1
            continue

        # has ensures — classify by tier (hacspec > bounds > math)
        lvl = fn['ensures_level']
        if lvl == 'hacspec':
            base['hacspec'] += 1
        elif lvl == 'bounds':
            base['bounds'] += 1
        else:  # 'math' or unknown
            base['math'] += 1

    base['body_admit_sites'] = body_admit_sites
    return base


# ============================================================================
# Output: status Markdown
# ============================================================================

PREAMBLE = """# {crate} Verification Status

This file is auto-generated by `proofs/generate_verification_status.py`.

Each function is classified at exactly one proof tier (highest wins):

- **Lax**: module in `ADMIT_MODULES`, OR fn has `#[hax_lib::fstar::verification_status(lax)]`,
  OR `#[hax_lib::fstar::options("--admit_smt_queries true")]`, OR `#[hax_lib::opaque]`
  (body hidden from F\\*; distinct from F\\*'s `opaque_to_smt`), OR an inline `admit ()`
  in the body.
- **Unverified**: Rust module not extracted to F\\* at all (filtered out by hax via
  `-i -<module>::**`). Worse than lax — no proof of any kind.
- **Panic-free**: proven free of panics (and obeying preconditions), no further proof:
  fn has `verification_status(panic_free)` or has no `#[ensures(...)]` annotation.
- **Math**: has an `#[ensures(...)]` annotation that proves SOME non-trivial property,
  but doesn't match the bounds or hacspec patterns.
- **Bounds**: ensures uses range/interval predicates (e.g. `is_i16b`, `is_bounded_*`).
- **Hacspec**: ensures cites the high-level mathematical specification (e.g. `Spec.MLKEM.*`).

The "Panic-safe" aggregate (sometimes useful for headline numbers) = Panic-free + Math
+ Bounds + Hacspec — i.e., total minus lax minus unverified.

"""


def write_status_md(rows, crate_name, output_path,
                    body_admit_sites_by_module, unverified_paths_seen,
                    module_counts_by_display=None):
    """`module_counts_by_display` maps display-name → number of underlying .rs files
    (e.g., 'mlkem*' → 4). Used for per-row 'Modules' count + category subtotals."""
    if module_counts_by_display is None:
        module_counts_by_display = {}

    # Group rows by category to compute subtotals
    grouped = []  # list of (category, [data_rows])
    current_cat = None
    bucket = []
    for row in rows:
        if row is None:
            continue
        cat = row[0]
        if cat:  # new category header inline (the row carries _Category_)
            if bucket:
                grouped.append((current_cat, bucket))
            current_cat = cat.replace('_', '').strip()
            bucket = [row]
        else:
            bucket.append(row)
    if bucket:
        grouped.append((current_cat, bucket))

    with open(output_path, 'w') as f:
        f.write(PREAMBLE.format(crate=crate_name))
        # Columns: Category | File | Mods | Fns | Lax | Unv | PF | Math | Bounds | Hacspec
        f.write(
            f"| {'Category':<10} | {'File':<17} | {'Mods':>4} | {'Fns':>3} "
            f"| {'Lax':>3} | {'Unv':>3} | {'PF':>3} | {'Math':>4} | {'Bounds':>6} | {'Hacspec':>7} |\n"
        )
        f.write(
            f"| {'-'*10} | {'-'*17} | {'-'*4:>4} | {'---':>3} "
            f"| {'---':>3} | {'---':>3} | {'---':>3} | {'-'*4:>4} | {'-'*6:>6} | {'-'*7:>7} |\n"
        )

        cat_totals = []  # for the per-category summary at the end
        for cat_idx, (cat_label, cat_rows) in enumerate(grouped):
            sub_total = sub_lax = sub_unv = sub_pf = sub_math = sub_bounds = sub_hacspec = sub_mods = 0
            for row in cat_rows:
                cat_display, display, total, lax, unv, pf, math, bounds, hacspec = row
                mods = module_counts_by_display.get(display, 1)
                sub_mods += mods
                sub_total += total; sub_lax += lax; sub_unv += unv
                sub_pf += pf; sub_math += math; sub_bounds += bounds; sub_hacspec += hacspec
                f.write(
                    f"| {cat_display:<10} | {display:<17} | {mods:>4} | {total:>3} "
                    f"| {lax:>3} | {unv if unv else '':>3} | {pf:>3} | {math:>4} | {bounds:>6} | {hacspec:>7} |\n"
                )
            # Per-category subtotal row
            f.write(
                f"| {'':10} | {'**'+cat_label+' total**':<17} "
                f"| {'**'+str(sub_mods)+'**':>4} | {'**'+str(sub_total)+'**':>3} "
                f"| {'**'+str(sub_lax)+'**':>3} | {('**'+str(sub_unv)+'**') if sub_unv else '':>3} "
                f"| {'**'+str(sub_pf)+'**':>3} | {'**'+str(sub_math)+'**':>4} "
                f"| {'**'+str(sub_bounds)+'**':>6} | {'**'+str(sub_hacspec)+'**':>7} |\n"
            )
            cat_totals.append((cat_label, sub_mods, sub_total, sub_lax, sub_unv, sub_pf, sub_math, sub_bounds, sub_hacspec))
            if cat_idx < len(grouped) - 1:
                f.write(
                    f"| {'':10} | {'':17} | {'':>4} | {'':>3} "
                    f"| {'':>3} | {'':>3} | {'':>3} | {'':>4} | {'':>6} | {'':>7} |\n"
                )

        total_fns = sum(r[2] for r in rows if r is not None)
        total_lax = sum(r[3] for r in rows if r is not None)
        total_unv = sum(r[4] for r in rows if r is not None)
        total_pf = sum(r[5] for r in rows if r is not None)
        total_math = sum(r[6] for r in rows if r is not None)
        total_bounds = sum(r[7] for r in rows if r is not None)
        total_hacspec = sum(r[8] for r in rows if r is not None)
        total_safe = total_pf + total_math + total_bounds + total_hacspec
        total_mods = sum(c[1] for c in cat_totals)

        f.write("\n## Summary\n\n")
        if total_fns:
            def pct(n):
                return f"({n*100/total_fns:.1f}%)"
            f.write(f"- **Total modules**: {total_mods}\n")
            f.write(f"- **Total functions**: {total_fns}\n")
            f.write(f"- **Lax** (admitted): {total_lax} {pct(total_lax)}\n")
            f.write(f"- **Unverified** (not extracted): {total_unv} {pct(total_unv)}\n")
            f.write(f"- **Panic-safe** (PF + Math + Bounds + Hacspec): {total_safe} {pct(total_safe)}\n")
            f.write(f"  - Panic-free only (no further proof): {total_pf} {pct(total_pf)}\n")
            f.write(f"  - Math (non-trivial ensures, no bounds/spec match): {total_math} {pct(total_math)}\n")
            f.write(f"  - Bounds (range/interval ensures): {total_bounds} {pct(total_bounds)}\n")
            f.write(f"  - Hacspec (cites high-level spec): {total_hacspec} {pct(total_hacspec)}\n")
        else:
            f.write("- (no functions found — check config paths)\n")

        if cat_totals:
            f.write("\n### Modules per category\n\n")
            f.write(f"| {'Category':<12} | {'Modules':>7} | {'Fns':>4} | {'Lax':>3} | {'Unv':>3} | {'PF':>3} | {'Math':>4} | {'Bounds':>6} | {'Hacspec':>7} |\n")
            f.write(f"| {'-'*12} | {'-'*7} | {'-'*4} | {'-'*3} | {'-'*3} | {'-'*3} | {'-'*4} | {'-'*6} | {'-'*7} |\n")
            for label, mods, tot, lax, unv, pf, math, bounds, hacspec in cat_totals:
                f.write(f"| {label:<12} | {mods:>7} | {tot:>4} | {lax:>3} | {unv:>3} | {pf:>3} | {math:>4} | {bounds:>6} | {hacspec:>7} |\n")

        if unverified_paths_seen:
            f.write("\n## Unverified Rust modules (not extracted to F\\*)\n\n")
            f.write("These Rust modules have no corresponding F\\* file in the extraction "
                    "directory — they were filtered out by hax (`-i -<module>::**` in `hax.py`) "
                    "and are unverified at any tier.\n\n")
            f.write(f"| {'Module':<30} | {'Path':<40} | {'Fns':>3} |\n")
            f.write(f"| {'-'*30} | {'-'*40} | {'-'*3} |\n")
            for label, path, n in unverified_paths_seen:
                f.write(f"| {label:<30} | {path:<40} | {n:>3} |\n")

        if body_admit_sites_by_module:
            f.write("\n## Body-admit sites (audit)\n\n")
            f.write("Functions classified as lax due to `admit ()` (or `--admit_smt_queries true`) "
                    "inside their body. Auditable so the script's classification decisions are traceable.\n\n")
            f.write(f"| {'Module':<25} | {'Line':>5} |\n")
            f.write(f"| {'-'*25} | {'-'*5} |\n")
            for module_label, sites in body_admit_sites_by_module:
                for line in sites:
                    f.write(f"| {module_label:<25} | {line:>5} |\n")


# ============================================================================
# Diff mode
# ============================================================================

# Match a status row with 10 columns: cat | display | mods | fns | lax | unv | pf | math | bounds | hacspec
# Subtotal rows wrap their numbers in `**...**` (markdown bold) so `\d+` won't match them.
DIFF_TABLE_RE = re.compile(
    r"^\| \s*([^|]*?)\s*\| \s*([^|]+?)\s*\| \s*(\d+)\s*"
    r"\| \s*(\d+)\s*"
    r"\| \s*(\d+)\s*\| \s*(\d*)\s*\| \s*(\d+)\s*"
    r"\| \s*(\d+)\s*\| \s*(\d+)\s*\| \s*(\d+)\s*\|\s*$",
    re.MULTILINE,
)


def parse_status_md(path):
    """Parse a verification_status.md table.
    Returns dict[(category, display)] = (mods, total, lax, unv, pf, math, bounds, hacspec)."""
    with open(path) as f:
        text = f.read()
    rows = {}
    current_category = ""
    for m in DIFF_TABLE_RE.finditer(text):
        cat = m.group(1).strip()
        display = m.group(2).strip()
        try:
            mods = int(m.group(3))
            total = int(m.group(4))
            lax = int(m.group(5))
            unv = int(m.group(6)) if m.group(6) else 0
            pf = int(m.group(7))
            math = int(m.group(8))
            bounds = int(m.group(9))
            hacspec = int(m.group(10))
        except ValueError:
            continue
        if cat:
            cleaned = cat.replace('_', '').strip()
            if cleaned:
                current_category = cleaned
        if not current_category:
            continue
        rows[(current_category, display)] = (mods, total, lax, unv, pf, math, bounds, hacspec)
    return rows


def write_diff_md(prev_path, curr_path, output_path, prev_label, curr_label):
    prev = parse_status_md(prev_path)
    curr = parse_status_md(curr_path)

    ordered_keys = []
    seen = set()
    for src in (curr, prev):
        for k in src:
            if k not in seen:
                ordered_keys.append(k)
                seen.add(k)

    def delta(a, b):
        d = b - a
        return f"{d:+d}" if d else "  "

    with open(output_path, 'w') as f:
        f.write(f"# Verification Status Diff — `{prev_label}` → `{curr_label}`\n\n")
        f.write(f"Comparison of `{prev_path}` against `{curr_path}`. "
                f"Each per-tier column is shown as `prev→curr (Δ)`.\n\n")
        f.write(f"| {'Category':<10} | {'File':<17} | {'Fns':>9} "
                f"| {'Lax':>9} | {'Unv':>9} | {'PF':>9} | {'Math':>9} | {'Bounds':>9} | {'Hacspec':>9} |\n")
        f.write(f"| {'-'*10} | {'-'*17} | {'-'*9} "
                f"| {'-'*9} | {'-'*9} | {'-'*9} | {'-'*9} | {'-'*9} | {'-'*9} |\n")
        last_cat = ""
        sums = {k: [0, 0] for k in ('total', 'lax', 'unv', 'pf', 'math', 'bounds', 'hacspec')}

        def cell(prev_n, curr_n):
            d = curr_n - prev_n
            sign = f"{d:+d}" if d else "·"
            return f"{prev_n}→{curr_n} {sign}"

        for k in ordered_keys:
            cat, display = k
            p = prev.get(k, (0, 0, 0, 0, 0, 0, 0, 0))
            c = curr.get(k, (0, 0, 0, 0, 0, 0, 0, 0))
            pmods, pt, plax, punv, ppf, pmath, pb, ph = p
            cmods, ct, clax, cunv, cpf, cmath, cb, ch = c
            cat_show = f"_{cat}_" if cat != last_cat else ""
            last_cat = cat
            f.write(f"| {cat_show:<10} | {display:<17} | {cell(pt, ct):>9} "
                    f"| {cell(plax, clax):>9} | {cell(punv, cunv):>9} | {cell(ppf, cpf):>9} "
                    f"| {cell(pmath, cmath):>9} | {cell(pb, cb):>9} | {cell(ph, ch):>9} |\n")
            for key, pv, cv in (
                ('total', pt, ct), ('lax', plax, clax), ('unv', punv, cunv),
                ('pf', ppf, cpf), ('math', pmath, cmath),
                ('bounds', pb, cb), ('hacspec', ph, ch),
            ):
                sums[key][0] += pv
                sums[key][1] += cv

        f.write("\n## Aggregate\n\n")
        for label, key in (
            ('Functions', 'total'), ('Lax', 'lax'), ('Unverified', 'unv'),
            ('Panic-free only', 'pf'), ('Math', 'math'),
            ('Bounds', 'bounds'), ('Hacspec', 'hacspec'),
        ):
            a, b = sums[key]
            f.write(f"- {label}: {a} → {b} ({b - a:+d})\n")
        safe_prev = sums['pf'][0] + sums['math'][0] + sums['bounds'][0] + sums['hacspec'][0]
        safe_curr = sums['pf'][1] + sums['math'][1] + sums['bounds'][1] + sums['hacspec'][1]
        f.write(f"- **Panic-safe (PF+Math+Bounds+Hacspec)**: {safe_prev} → {safe_curr} ({safe_curr-safe_prev:+d})\n")


# ============================================================================
# Main
# ============================================================================

def main():
    parser = argparse.ArgumentParser(description=__doc__.split('\n')[0])
    parser.add_argument('--root', default=None,
                        help="Crate root (default: parent of script's directory)")
    parser.add_argument('--config', default=None,
                        help='Path to verification_status.config.json (default: <root>/proofs/verification_status.config.json or baked-in ML-KEM)')
    parser.add_argument('--output', default=None,
                        help='Output Markdown path')
    parser.add_argument('--diff', nargs=2, metavar=('PREV', 'CURR'),
                        help='Diff mode: write a comparison table of two verification_status.md files')
    parser.add_argument('--diff-label-prev', default='prev')
    parser.add_argument('--diff-label-curr', default='curr')
    args = parser.parse_args()

    if args.diff:
        prev, curr = args.diff
        out = args.output or 'verification_status_diff.md'
        write_diff_md(prev, curr, out, args.diff_label_prev, args.diff_label_curr)
        print(f"Generated diff at {out}")
        return

    script_dir = os.path.dirname(os.path.abspath(__file__))
    root = os.path.abspath(args.root) if args.root else os.path.dirname(script_dir)
    config = load_config(args.config, root)

    src_dir = os.path.join(root, config['src_dir'])
    makefile = os.path.join(root, config['makefile'])
    extraction_dir = os.path.join(root, config.get('extraction_dir',
                                                   'proofs/fstar/extraction'))
    output = args.output or os.path.join(root, config['output'])

    spec_re = re.compile('|'.join(config['spec_patterns']))
    range_re = re.compile('|'.join(config['range_patterns']))

    extracted_paths = list_extracted_modules(extraction_dir, config['admit_module_prefix'], src_dir)
    admit_paths = parse_admit_modules(makefile, config['admit_module_prefix'], extracted_paths)

    rows = []
    body_admit_sites_by_module = []
    unverified_paths_seen = []
    module_counts_by_display = {}  # display name → number of underlying source files
    prev_category = ""

    for module in config['modules']:
        category = module['category']
        display = module['display']
        paths = module['paths']

        agg = {'total': 0, 'lax': 0, 'unverified': 0,
               'panic_free': 0, 'math': 0, 'bounds': 0, 'hacspec': 0}
        all_body_admits = []
        files_present = 0

        for p in paths:
            filepath = os.path.join(src_dir, f"{p}.rs")
            if not os.path.isfile(filepath):
                continue
            files_present += 1
            rel = f"src/{p}.rs"
            funcs = parse_file(filepath, spec_re, range_re)
            # If we have an extraction dir, a Rust module not present there is unverified.
            is_unverified = bool(extracted_paths) and rel not in extracted_paths
            stats = compute_module_stats(funcs, rel in admit_paths, is_unverified)
            for k in ('total', 'lax', 'unverified', 'panic_free', 'math', 'bounds', 'hacspec'):
                agg[k] += stats[k]
            for line in stats['body_admit_sites']:
                all_body_admits.append((rel, line))
            if is_unverified and stats['total'] > 0:
                unverified_paths_seen.append((f"{category}/{display}", rel, stats['total']))

        cat_display = ""
        if category != prev_category:
            if prev_category:
                rows.append(None)
            cat_display = f"_{category}_"
            prev_category = category

        rows.append((cat_display, display,
                     agg['total'], agg['lax'], agg['unverified'],
                     agg['panic_free'], agg['math'], agg['bounds'], agg['hacspec']))
        module_counts_by_display[display] = files_present

        if all_body_admits:
            label = f"{category}/{display}"
            body_admit_sites_by_module.append((label, [line for (_, line) in all_body_admits]))

    write_status_md(rows, config['crate_name'], output,
                    body_admit_sites_by_module, unverified_paths_seen,
                    module_counts_by_display)
    print(f"Generated {output}")


if __name__ == "__main__":
    main()
