#!/usr/bin/env python3
"""Shared OBSERVED-SIDE scanner for the trust-annotation ledger (V7 reconciler).

Pure Python, no F*/proxy/`make` dependency — safe to run in CI after `extract`.
This is the single shared scanner the trust campaign's plan calls for: it computes
the *observed* trust surface directly from build artifacts, so the ledger can never
be shrunk by a stale or optimistic source marker (see the plan's V7 section).

Four observed planes, one function each:

  plane 1  fstar       unproven obligations in the extracted .fst/.fsti + hand-written
                       companions (admit ()/magic ()/assume/assume val/--admit_smt_queries
                       true). Reproduces the `fstar_admits` MCP tool WITHOUT the proxy.
  plane 2  extraction  the set of extracted F* modules (coverage). A module that stops
                       extracting (hax `-i` filter, deleted .fst) drops out of this set.
  plane 3  makefile    SLOW_MODULES / ADMIT_MODULES declared in the F* Makefile — the
                       modules verified-on-cadence or admitted-in-default-build (plane
                       that fstar_admits alone cannot see).
  plane 4  patches     post-extraction *.patch files under proofs/fstar (count + digest).

`generate_verification_status.py` reuses the plane-1 scanner and the Makefile parser
here so there is exactly one obligation-scanning implementation in the tree.
"""

import hashlib
import os
import re

# ---------------------------------------------------------------------------
# Directories never scanned for F* obligations (caches, VCS, build output).
# ---------------------------------------------------------------------------
_SKIP_DIRS = {".fstar-cache", ".git", "target", "cache", "hints", "node_modules"}


# ===========================================================================
# Plane 1 — F* obligations (reproduces the fstar_admits MCP tool, proxy-free)
# ===========================================================================

def mask_fstar_comments(text):
    """Replace F* comments with spaces (newlines preserved so line numbers hold).

    Handles NESTED block comments `(* ... (* ... *) ... *)` and `//` line comments.
    String literals are intentionally NOT masked — mirrors fstar_admits, where a
    marker inside a string literal can still be reported."""
    out = []
    i, n = 0, len(text)
    while i < n:
        two = text[i:i + 2]
        if two == "(*":
            depth = 1
            out.append("  ")
            i += 2
            while i < n and depth > 0:
                t2 = text[i:i + 2]
                if t2 == "(*":
                    depth += 1
                    out.append("  ")
                    i += 2
                elif t2 == "*)":
                    depth -= 1
                    out.append("  ")
                    i += 2
                else:
                    out.append("\n" if text[i] == "\n" else " ")
                    i += 1
            continue
        if two == "//":
            while i < n and text[i] != "\n":
                out.append(" ")
                i += 1
            continue
        out.append(text[i])
        i += 1
    return "".join(out)


# ---------------------------------------------------------------------------
# G3 pollution-trap defense: mask the interior of `[@@ "trusted: <reason>"]` tags.
#
# The G3 companion-axiom tags are F* string-literal attributes, and the obligation
# tokenizer above does NOT mask string literals (that is deliberate — it mirrors
# fstar_admits, where a marker inside a string can still be reported). Without this
# pass, a trust REASON that happened to contain the word `assume`, or the text
# `admit ()` / `magic ()` / `assume val` / `admit_smt_queries true`, would be
# miscounted as a real F* obligation and REGRESS the ledger. Reasons are kept
# token-safe by convention (so fstar_admits and this scanner stay in agreement),
# and this pass is the belt-and-suspenders guarantee that a future non-token-safe
# reason can never silently grow the surface. Only strings whose content begins
# with `trusted:` are touched; every other string literal is left intact.
_TRUSTED_STR_RE = re.compile(r'"trusted:[^"\n]*"')


def mask_trusted_reason_strings(text):
    """Blank the interior of every `[@@ "trusted: <reason>"]` tag string, preserving
    the surrounding quotes and the exact length (so obligation line numbers still
    hold). See the comment above for why this is needed. Non-`trusted:` strings are
    left untouched, keeping plane 1 faithful to fstar_admits for real obligations."""
    return _TRUSTED_STR_RE.sub(lambda m: '"' + " " * (len(m.group(0)) - 2) + '"', text)


# One record kind per pattern. `assume val` is matched before bare `assume`
# (the bare pattern uses a negative lookahead so the two never double-count).
_ASSUME_VAL_RE = re.compile(r"\bassume\s+val\b")
_ASSUME_RE = re.compile(r"\bassume\b(?!\s+val\b)")
_ADMIT_RE = re.compile(r"\badmit\s*\(\s*\)")
_MAGIC_RE = re.compile(r"\bmagic\s*\(\s*\)")
_ADMIT_SMT_RE = re.compile(r"admit_smt_queries\s+true")

_KIND_PATTERNS = [
    ("assume_val", _ASSUME_VAL_RE),
    ("assume", _ASSUME_RE),
    ("admit", _ADMIT_RE),
    ("magic", _MAGIC_RE),
    ("admit_smt_queries", _ADMIT_SMT_RE),
]


def _line_of(text, pos):
    return text.count("\n", 0, pos) + 1


def module_name_of(path):
    """`.../Libcrux_ml_dsa.Matrix.fst` -> `Libcrux_ml_dsa.Matrix`."""
    base = os.path.basename(path)
    return base.removesuffix(".fsti").removesuffix(".fst")


def scan_file_obligations(path):
    """Return a list of {file, module, line, kind} obligation records for one
    .fst/.fsti file, matching the fstar_admits tokenizer (comment-masked)."""
    with open(path, encoding="utf-8", errors="replace") as f:
        text = f.read()
    masked = mask_trusted_reason_strings(mask_fstar_comments(text))
    module = module_name_of(path)
    records = []
    for kind, rx in _KIND_PATTERNS:
        for m in rx.finditer(masked):
            records.append({
                "file": path,
                "module": module,
                "line": _line_of(masked, m.start()),
                "kind": kind,
            })
    return records


def scan_obligations(root):
    """Walk `root` for .fst/.fsti (skipping caches) and collect all obligation
    records. Returns dict with total, by_kind, by_file (module->count), records,
    and scanned_files — the same shape the fstar_admits summary exposes."""
    records = []
    scanned = 0
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = [d for d in dirnames if d not in _SKIP_DIRS]
        for fn in filenames:
            if fn.endswith(".fst") or fn.endswith(".fsti"):
                scanned += 1
                records.extend(scan_file_obligations(os.path.join(dirpath, fn)))
    by_kind, by_file = {}, {}
    for r in records:
        by_kind[r["kind"]] = by_kind.get(r["kind"], 0) + 1
        by_file[r["module"]] = by_file.get(r["module"], 0) + 1
    return {
        "total": len(records),
        "scanned_files": scanned,
        "by_kind": by_kind,
        "by_file": by_file,
        "records": records,
    }


# G3 companion-axiom tags (the CLAIMS side of the F* plane). A hand-written
# companion axiom carries a `[@@ "trusted: <category>: <reason>"]` tag above the
# decl; `<reason>` (everything after `trusted:`) is validated with reason_ok, the
# same category vocabulary as the Rust G1/G2 markers. Matched anywhere in an
# attribute set (`[@@ "opaque_to_smt"; "trusted: …"]` works too).
_TRUSTED_TAG_RE = re.compile(r'"trusted:\s*([^"\n]*)"')


def scan_fstar_trusted_tags(path):
    """Return [{file, module, line, reason}] for every `[@@ "trusted: <reason>"]`
    tag in an .fst/.fsti file (comment-masked, so a commented-out tag is ignored).
    `reason` is the text after `trusted:` — validate it with reason_ok (lint V4)."""
    with open(path, encoding="utf-8", errors="replace") as f:
        text = f.read()
    masked = mask_fstar_comments(text)
    module = module_name_of(path)
    out = []
    for m in _TRUSTED_TAG_RE.finditer(masked):
        out.append({
            "file": path,
            "module": module,
            "line": _line_of(masked, m.start()),
            "reason": m.group(1).strip(),
        })
    return out


# ===========================================================================
# Plane 2 — extraction coverage (the set of extracted F* modules)
# ===========================================================================

def list_extracted_module_names(extraction_dir, prefix):
    """Return the sorted set of extracted F* module names under `extraction_dir`
    whose name starts with `prefix` (e.g. `Libcrux_ml_dsa.`). A module dropping
    out of this set means it stopped extracting (hax `-i` filter / deletion)."""
    if not os.path.isdir(extraction_dir):
        return []
    names = set()
    for fn in os.listdir(extraction_dir):
        if (fn.endswith(".fst") or fn.endswith(".fsti")) and fn.startswith(prefix):
            names.add(module_name_of(fn))
    return sorted(names)


# ===========================================================================
# Plane 3 — Makefile SLOW_MODULES / ADMIT_MODULES (build-admitted plane)
# ===========================================================================

def parse_makefile_module_list(makefile_path, var_name):
    """Read a Makefile variable that lists F* module names (`+=`/`=`, one per line,
    `\\`-continued) and return them as a sorted list of module names. Tokens that
    aren't module-shaped (`$(...)`, `filter-out`, operators) are ignored."""
    if not os.path.isfile(makefile_path):
        return []
    mods = set()
    in_var = False
    with open(makefile_path) as f:
        for line in f:
            head = line.lstrip()
            if head.startswith(var_name) and (
                head[len(var_name):len(var_name) + 1] in (" ", "\t", "+", "=", ":")
            ):
                in_var = True
                line = line.split("=", 1)[1] if "=" in line else ""
            if in_var:
                # Ignore a trailing `#`-comment so a `# trusted-module:` reason (V5)
                # or any prose can never inject a phantom module token (e.g. a dotted
                # word like `e.g.` reading as a module name).
                code = line.split("#", 1)[0]
                for token in code.split():
                    tok = token.removesuffix(".fst").removesuffix(".fsti")
                    if re.fullmatch(r"[A-Za-z0-9_.]+", tok) and "." in tok:
                        mods.add(tok)
                if not code.rstrip().endswith("\\"):
                    in_var = False
    return sorted(mods)


# ===========================================================================
# G3 module/config mirrors — the CLAIMS side of planes 2 & 3
#
# `# trusted-module: <name> : <category>: <reason>` comments mirror the module-level
# trust surfaces the same way the Rust G1/G2 markers and the F* G3 tags mirror the
# obligation surface: <name> is a Makefile SLOW/ADMIT module (V5) or a hax `-i`
# extraction-exclusion token (V6), and <reason> carries a category prefix (reason_ok).
# ===========================================================================

# `# trusted-module: <name> : <reason>` — split name/reason on the FIRST ` : `
# (spaced colon) so a module path's own bare `::` is never the separator.
_TRUSTED_MODULE_RE = re.compile(r"#\s*trusted-module:\s*(.+?)\s*$", re.M)


def scan_trusted_module_annotations(text):
    """Parse `# trusted-module: <name> : <reason>` comment lines from a Makefile or a
    hax extraction script. Returns [{name, reason, line}] (both stripped). Reasons are
    validated with reason_ok by the V5/V6 lints."""
    out = []
    for m in _TRUSTED_MODULE_RE.finditer(text):
        payload = m.group(1).strip()
        name, sep, reason = payload.partition(" : ")
        out.append({
            "name": name.strip(),
            "reason": reason.strip() if sep else "",
            "line": text.count("\n", 0, m.start()) + 1,
        })
    return out


# A hax `-i` MODULE-EXCLUSION token: `-<path>` with >= 1 `::` segment (wildcards ok).
# Requiring a `::` avoids matching bare flags (`-i`, `-C`, `--features`, `--z3rlimit`).
_HAX_EXCL_TOKEN_RE = re.compile(
    r"-(?:\*\*|[A-Za-z_][A-Za-z0-9_]*)(?:::(?:\*\*?|[A-Za-z_][A-Za-z0-9_]*))+"
)


def scan_hax_exclusion_tokens(text, crate_snake):
    """Return the sorted set of `-<crate_snake>::…` module-exclusion tokens in a hax
    extraction script — the parts of THIS crate dropped from F* extraction (a trust
    surface: an absent module is worse than an admitted one). Skips `--interfaces` /
    `interface_include` lines (those suppress only the `.fsti`, not the proof) and the
    `# trusted-module:` annotation lines (so an annotation naming a token is not itself
    counted as a usage)."""
    prefix = "-" + crate_snake + "::"
    toks = set()
    for line in text.split("\n"):
        if line.lstrip().startswith("#"):
            continue
        if "interface" in line:
            continue
        for m in _HAX_EXCL_TOKEN_RE.finditer(line):
            if m.group(0).startswith(prefix):
                toks.add(m.group(0))
    return sorted(toks)


# ===========================================================================
# Plane 4 — post-extraction patches (count + digest)
# ===========================================================================

def list_fstar_patches(crate_root):
    """Return sorted [{path, sha256}] for every *.patch under <crate>/proofs/fstar.
    These are the manual post-extraction edits applied to the F* tree."""
    fstar_root = os.path.join(crate_root, "proofs", "fstar")
    out = []
    for dirpath, dirnames, filenames in os.walk(fstar_root):
        dirnames[:] = [d for d in dirnames if d not in _SKIP_DIRS]
        for fn in sorted(filenames):
            if fn.endswith(".patch"):
                p = os.path.join(dirpath, fn)
                with open(p, "rb") as fh:
                    digest = hashlib.sha256(fh.read()).hexdigest()
                out.append({
                    "path": os.path.relpath(p, crate_root),
                    "sha256": digest,
                })
    return sorted(out, key=lambda d: d["path"])


# ===========================================================================
# CLAIMS side — Rust trust markers (G1+)
#
# The observed side above is ground truth; the markers below are CLAIMS about
# WHY an obligation is trusted. This scanner is the shared source of truth for
# the trust-marker lints (V2/V2b/V3 in annotation_lint.py) and the ledger's
# marker-direction reconciliation (reconcile_markers in trust_ledger.py).
# ===========================================================================

# Category prefixes for a trust reason (plan's unified vocabulary). `pending-proof`
# carries a `(<ref>)` before the colon. Reason format: "<category>: <one-line>".
TRUST_CATEGORIES = (
    "unprovable-termination",
    "hax-limitation",
    "trusted-extern",
    "validated-axiom",
    "nospec",  # uninterpreted intrinsic symbol: no int-vec model/lift yet
    "slow-proof",
    "pending-proof",  # requires a (<ref>) suffix — see _REASON_RE
)
_REASON_RE = re.compile(
    r"^(?:unprovable-termination|hax-limitation|trusted-extern|validated-axiom"
    r"|nospec|slow-proof|pending-proof\([^)]+\)):\s"
)

# The G1 body wrappers and the fn-level summary label.
_TRUSTED_ADMIT_RE = re.compile(r"\btrusted_admit!\s*\(")
_TRUSTED_ASSUME_RE = re.compile(r"\btrusted_assume!\s*\(")
_TRUSTED_LABEL_RE = re.compile(
    r"#\[\s*libcrux_macros::trusted\s*\(\s*(inline-admit|inline-assume)\s*\)\s*\]"
)
# The G2 whole-function attribute wrappers: `#[libcrux_macros::trusted(<kind>, "<reason>")]`
# where <kind> emits the corresponding hax mechanism (see crates/utils/macros). The
# `\b` (not a required comma) lets the scanner also catch a reason-less `#[trusted(opaque)]`
# so V2 can flag the missing reason instead of silently ignoring it.
#
# `replace` is a PURE marker (like inline-admit): it emits no mechanism, it sits
# alongside a sibling `#[hax_lib::fstar::replace(...)]` attribute purely as the
# declaration the V8 replace-bijection lint counts. It carries a `"<category>: <reason>"`
# 2nd arg (validated by reason_ok), so it lives in the `attr` bucket with the G2 kinds
# rather than the reason-less `labels` bucket.
_TRUSTED_ATTR_KINDS = ("lax", "panic_free", "opaque", "exclude", "replace")
_TRUSTED_ATTR_RE = re.compile(
    r"#\[\s*libcrux_macros::trusted\s*\(\s*(lax|panic_free|opaque|exclude|replace)\b"
)
# `#[hax_lib::fstar::replace(...)]` / `#[fstar::replace_body(...)]` SITES — the OBSERVED
# side of the replace trust surface. Match the attribute HEAD only (on comment-masked
# text), so a `fstar::replace` mention inside a replacement string or a comment is never
# miscounted. `replace_body` (body-only substitution) is the same class of trust surface
# — F* verifies hand-written text, not the extracted body — and is counted the same.
_FSTAR_REPLACE_SITE_RE = re.compile(r"#\[\s*(?:hax_lib::)?fstar::replace(?:_body)?\b")
# Raw obligation-producing mechanisms that MUST now be wrapped (V3 ban).
_RAW_ADMIT_RE = re.compile(r'\bproof!\s*\(\s*"admit \(\)"\s*\)')
_RAW_ASSUME_RE = re.compile(r'\bproof!\s*\(\s*r?#*"?\s*assume\b')
# Rust fn definition (captures the name). Covers pub/pub(crate)/const/unsafe/async.
_RUST_FN_RE = re.compile(
    r"^\s*(?:pub\s*(?:\([^)]*\)\s*)?)?(?:default\s+)?(?:const\s+)?(?:async\s+)?"
    r"(?:unsafe\s+)?(?:extern\s+\"[^\"]*\"\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)"
)

# proof_macros.rs is the DEFINITION site of the wrappers (macro_rules! + doc
# comments), never a call site — exclude it so the macro defs / doc examples
# don't read as usages.
_MARKER_SKIP_FILES = {"proof_macros.rs"}


def mask_rust_comments(text):
    """Blank Rust comments (`//`-to-EOL and NESTED `/* ... */`) with spaces,
    preserving newlines and string literals. String literals (`"..."`, byte and
    raw `r#"..."#`) are skipped over so a `//` or `/*` inside a string — or inside
    a trust reason — is NOT treated as a comment."""
    out = []
    i, n = 0, len(text)
    while i < n:
        c = text[i]
        two = text[i:i + 2]
        # raw string: r"...", r#"..."#, br##"..."## etc.
        mraw = re.match(r'b?r(#*)"', text[i:i + 8])
        if mraw:
            hashes = mraw.group(1)
            close = '"' + hashes
            j = text.find(close, i + mraw.end())
            end = (j + len(close)) if j != -1 else n
            out.append(text[i:end])
            i = end
            continue
        if c == '"':  # normal / byte string with \-escapes
            out.append(c)
            i += 1
            while i < n:
                if text[i] == "\\" and i + 1 < n:
                    out.append(text[i:i + 2])
                    i += 2
                    continue
                out.append(text[i])
                if text[i] == '"':
                    i += 1
                    break
                i += 1
            continue
        if two == "//":
            while i < n and text[i] != "\n":
                out.append(" ")
                i += 1
            continue
        if two == "/*":
            depth = 1
            out.append("  ")
            i += 2
            while i < n and depth > 0:
                t2 = text[i:i + 2]
                if t2 == "/*":
                    depth += 1
                    out.append("  ")
                    i += 2
                elif t2 == "*/":
                    depth -= 1
                    out.append("  ")
                    i += 2
                else:
                    out.append("\n" if text[i] == "\n" else " ")
                    i += 1
            continue
        out.append(c)
        i += 1
    return "".join(out)


def _first_string_arg(text, open_paren_pos):
    """Read the first string-literal argument after a macro's `(`, returning its
    content (used for the reason). Handles normal `"..."` (with `\\`-escapes and
    `\\`-newline continuations) and raw `r#"..."#` strings."""
    i, n = open_paren_pos + 1, len(text)
    while i < n and text[i] in " \t\r\n":
        i += 1
    mraw = re.match(r'r(#*)"', text[i:i + 8])
    if mraw:
        hashes = mraw.group(1)
        start = i + mraw.end()
        close = '"' + hashes
        j = text.find(close, start)
        return text[start:j] if j != -1 else text[start:]
    if i < n and text[i] == '"':
        i += 1
        buf = []
        while i < n:
            if text[i] == "\\" and i + 1 < n:
                # \-newline continuation: drop it (Rust eats newline + leading ws)
                if text[i + 1] == "\n":
                    i += 2
                    while i < n and text[i] in " \t":
                        i += 1
                    continue
                buf.append(text[i + 1])
                i += 2
                continue
            if text[i] == '"':
                break
            buf.append(text[i])
            i += 1
        return "".join(buf)
    return ""


def _fn_index(masked_lines):
    """[(line_index, fn_name)] for each fn definition (comment-masked lines)."""
    out = []
    for idx, line in enumerate(masked_lines):
        m = _RUST_FN_RE.match(line)
        if m:
            out.append((idx, m.group(1)))
    return out


def _enclosing_fn(fn_defs, line_index):
    """Name of the fn whose definition most closely precedes `line_index`."""
    name = None
    for idx, fn in fn_defs:
        if idx <= line_index:
            name = fn
        else:
            break
    return name


def _following_fn(fn_defs, line_index):
    """Name of the first fn defined at or after `line_index` (the labeled fn)."""
    for idx, fn in fn_defs:
        if idx >= line_index:
            return fn
    return None


def scan_file_trust_markers(path, repo_root):
    """Scan one .rs file for trust markers. Returns dict with lists:
    `body` (trusted_admit!/trusted_assume! calls), `labels` (#[trusted(inline-*)]),
    `attr` (#[trusted(lax|panic_free|opaque|exclude, "reason")] whole-function wrappers),
    `raw_admit`/`raw_assume` (banned bare proof! mechanisms)."""
    with open(path, encoding="utf-8", errors="replace") as f:
        text = f.read()
    masked = mask_rust_comments(text)
    masked_lines = masked.split("\n")
    fn_defs = _fn_index(masked_lines)
    rel = os.path.relpath(path, repo_root)
    line_of = lambda pos: masked.count("\n", 0, pos)  # 0-based line index

    body, labels, attr, raw_admit, raw_assume = [], [], [], [], []

    for kind, rx in (("inline-admit", _TRUSTED_ADMIT_RE),
                     ("inline-assume", _TRUSTED_ASSUME_RE)):
        for m in rx.finditer(masked):
            li = line_of(m.start())
            reason = _first_string_arg(masked, m.end() - 1)
            body.append({
                "file": rel, "line": li + 1, "kind": kind,
                "reason": reason, "fn": _enclosing_fn(fn_defs, li),
            })
    for m in _TRUSTED_LABEL_RE.finditer(masked):
        li = line_of(m.start())
        labels.append({
            "file": rel, "line": li + 1, "kind": m.group(1),
            "fn": _following_fn(fn_defs, li),
        })
    # G2 whole-function attribute wrappers. The reason is the 2nd macro arg (after
    # the kind ident); a reason-less wrapper yields "" so V2 flags it.
    for m in _TRUSTED_ATTR_RE.finditer(masked):
        li = line_of(m.start())
        j = m.end()
        while j < len(masked) and masked[j] in " \t\r\n":
            j += 1
        reason = _first_string_arg(masked, j) if j < len(masked) and masked[j] == "," else ""
        attr.append({
            "file": rel, "line": li + 1, "kind": m.group(1),
            "reason": reason, "item": _following_fn(fn_defs, li),
        })
    for rx, bucket in ((_RAW_ADMIT_RE, raw_admit), (_RAW_ASSUME_RE, raw_assume)):
        for m in rx.finditer(masked):
            bucket.append({"file": rel, "line": line_of(m.start()) + 1})

    return {"body": body, "labels": labels, "attr": attr,
            "raw_admit": raw_admit, "raw_assume": raw_assume}


def scan_file_replace_sites(path, repo_root):
    """Return [{file, line}] for every `#[hax_lib::fstar::replace(...)]` /
    `#[fstar::replace_body(...)]` attribute HEAD in one .rs file (comment-masked, so a
    `fstar::replace` mention in a comment or inside a replacement string is ignored)."""
    with open(path, encoding="utf-8", errors="replace") as f:
        text = f.read()
    masked = mask_rust_comments(text)
    rel = os.path.relpath(path, repo_root)
    return [{"file": rel, "line": masked.count("\n", 0, m.start()) + 1}
            for m in _FSTAR_REPLACE_SITE_RE.finditer(masked)]


def scan_rust_trust_markers(src_root, repo_root):
    """Walk `src_root` (a crate's src/) for .rs files and aggregate trust markers.
    Excludes the wrapper-definition file (proof_macros.rs)."""
    agg = {"body": [], "labels": [], "attr": [], "raw_admit": [], "raw_assume": []}
    for dirpath, dirnames, filenames in os.walk(src_root):
        dirnames[:] = [d for d in dirnames if d not in _SKIP_DIRS]
        for fn in sorted(filenames):
            if not fn.endswith(".rs") or fn in _MARKER_SKIP_FILES:
                continue
            got = scan_file_trust_markers(os.path.join(dirpath, fn), repo_root)
            for k in agg:
                agg[k].extend(got[k])
    return agg


def reason_ok(reason):
    """True iff a trust reason starts with a valid category prefix (V2)."""
    return bool(_REASON_RE.match(reason.strip()))


# ===========================================================================
# WS5 — core-models "silent surface" scanners (mk_lift_lemma! + opaque stubs)
#
# Two trust surfaces that a naive `assume`/`admit` grep MISSES because they say
# neither word in Rust, yet each becomes an F* axiom on extraction:
#
#   * `mk_lift_lemma!(NAME(...) == ...)` expands (see core_arch/*/interpretations.rs)
#     to an opaque `#[hax_lib::lemma]` carrying `[@@ LIFT_LEMMA]`, which hax extracts
#     as a `[@@ v_LIFT_LEMMA] assume val NAME_interp : ... -> Lemma (...)` — an
#     UNPROVEN (differentially mk!-tested) axiom relating the opaque hardware symbol
#     to its int-vec model. ~150 of these, invisible to the F* `assume`-token plane
#     until post-extraction (and even then they read as ordinary `assume val`s with
#     no back-link to the Rust site).
#
#   * `#[hax_lib::opaque] fn NAME(..) { unimplemented!()/todo!() }` extracts to an
#     UNINTERPRETED F* `val NAME : ty` (no body). If NO `mk_lift_lemma!` names it,
#     F* knows *nothing* about it — a "no-spec" hardware symbol (e.g. x86 AES/CLMUL).
#
# These run on the git-tracked core-models Rust source, so they are visible on a
# plain checkout even though the generated `.fst` tree is gitignored.
# ===========================================================================

# A `mk_lift_lemma!(NAME ...)` INVOCATION (not the `macro_rules! mk_lift_lemma {`
# definition, which is followed by `{` not `!(`). Captures the intrinsic name.
_MK_LIFT_LEMMA_RE = re.compile(r"\bmk_lift_lemma!\s*\(\s*([A-Za-z_][A-Za-z0-9_]*)")
# The two source forms of an opaque hardware-symbol tag:
#   (a) the bare `#[hax_lib::opaque]` attribute head, and
#   (b) the LOUD `#[libcrux_macros::trusted(opaque, "…")]` marker introduced by the
#       trust-hardening sweep, which emits the SAME `#[cfg_attr(hax, hax_lib::opaque)]`
#       mechanism (byte-identical F* extraction) but is greppable and reason-tagged.
# Both must be recognised so the enumerator does not lose sight of a symbol once it
# is marked; the `marked` flag records which symbols carry the loud form.
_HAX_OPAQUE_RE = re.compile(r"#\[\s*hax_lib::opaque\s*\]")
_TRUSTED_OPAQUE_RE = re.compile(r"#\[\s*libcrux_macros::trusted\s*\(\s*opaque\b")
# Empty-body markers that make an opaque fn a pure stub (no executable Rust model).
_STUB_BODY_RE = re.compile(r"\b(?:unimplemented|todo)!\s*\(")


def scan_file_mk_lift_lemmas(path, repo_root=None):
    """Return [{file, line, name}] for every `mk_lift_lemma!(NAME ...)` invocation
    in one .rs file (comment-masked, so the `macro_rules!` definition and the `//!`
    guide examples in x86.rs are ignored). `name` is the lifted intrinsic."""
    with open(path, encoding="utf-8", errors="replace") as f:
        text = f.read()
    masked = mask_rust_comments(text)
    rel = os.path.relpath(path, repo_root) if repo_root else path
    out = []
    for m in _MK_LIFT_LEMMA_RE.finditer(masked):
        out.append({
            "file": rel,
            "line": masked.count("\n", 0, m.start()) + 1,
            "name": m.group(1),
        })
    return out


def scan_file_opaque_intrinsics(path, repo_root=None):
    """Return [{file, line, name, stub_body, marked}] for every opaque-tagged
    *function* in one .rs file, where "opaque-tagged" means either a bare
    `#[hax_lib::opaque]` OR a `#[libcrux_macros::trusted(opaque, "…")]` marker.
    `stub_body` is True iff the body is an empty `unimplemented!()`/`todo!()` (no
    executable Rust model — the worst no-spec form). `marked` is True iff the loud
    `trusted(opaque, …)` form is used (the trust-hardening sweep's greppable marker).
    Comment-masked, so the `//!`-guide example in x86.rs is ignored. Opaque items
    that are NOT functions (opaque structs/consts) are skipped."""
    with open(path, encoding="utf-8", errors="replace") as f:
        text = f.read()
    masked = mask_rust_comments(text)
    lines = masked.split("\n")
    rel = os.path.relpath(path, repo_root) if repo_root else path
    out = []
    for i, line in enumerate(lines):
        marked = bool(_TRUSTED_OPAQUE_RE.search(line))
        if not (marked or _HAX_OPAQUE_RE.search(line)):
            continue
        # The fn may sit a few attribute/blank lines below the opaque attr.
        for j in range(i + 1, min(i + 30, len(lines))):
            m = _RUST_FN_RE.match(lines[j])
            if m:
                # Body window: from the fn line up to the next attribute block,
                # capped — opaque intrinsic bodies are 1-3 lines.
                window = "\n".join(lines[j:j + 40]).split("\n#")[0][:800]
                out.append({
                    "file": rel,
                    "line": j + 1,
                    "name": m.group(1),
                    "stub_body": bool(_STUB_BODY_RE.search(window)),
                    "marked": marked,
                })
                break
            # A non-fn item (struct/const/type/impl) directly under the attr:
            # this opaque tag is not an intrinsic-fn axiom — stop looking.
            if re.match(r"\s*(?:pub\s*(?:\([^)]*\)\s*)?)?"
                        r"(?:struct|enum|const|static|type|impl|trait|mod)\b", lines[j]):
                break
    return out


def file_sha256(path):
    """Hex sha256 of a file's bytes (for content-hash dedup of byte-identical
    F* modules extracted into more than one crate)."""
    with open(path, "rb") as fh:
        return hashlib.sha256(fh.read()).hexdigest()


def marker_soundness(markers):
    """Cross-check the CLAIMS side for internal soundness (shared by the V2b/V3
    lint and the ledger's marker reconciliation). Returns (missing, stale, raw):

      missing  [(file, fn, kind)]  a body trusted_admit!/trusted_assume! whose fn
                                   lacks the matching-kind #[trusted(inline-*)] label
      stale    [(file, fn, kind)]  a fn-level label with no matching body macro
      raw      [(file, line, kind)] a bare proof!("admit ()")/proof!(assume …) that
                                   bypasses the wrappers (kind in {admit, assume})
    """
    body_kinds, label_kinds = {}, {}
    for b in markers["body"]:
        body_kinds.setdefault((b["file"], b["fn"]), set()).add(b["kind"])
    for lab in markers["labels"]:
        label_kinds.setdefault((lab["file"], lab["fn"]), set()).add(lab["kind"])
    missing, stale = [], []
    for key in sorted(set(body_kinds) | set(label_kinds)):
        bk, lk = body_kinds.get(key, set()), label_kinds.get(key, set())
        for k in sorted(bk - lk):
            missing.append((key[0], key[1], k))
        for k in sorted(lk - bk):
            stale.append((key[0], key[1], k))
    raw = ([(r["file"], r["line"], "admit") for r in markers["raw_admit"]]
           + [(r["file"], r["line"], "assume") for r in markers["raw_assume"]])
    return missing, stale, raw
