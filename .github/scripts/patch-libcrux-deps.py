#!/usr/bin/env python3
# /// script
# requires-python = ">=3.11"
# dependencies = ["tomlkit>=0.12"]
# ///
"""Point a downstream Cargo workspace at a local libcrux checkout.

Every published crate of the libcrux workspace is added to `[patch.crates-io]` as a path
dependency. Since libcrux crates are `0.0.x`, a downstream requirement such as
`libcrux-ml-kem = "0.0.10"` only matches exactly 0.0.10, so a patch alone would
be ignored. Requirements on libcrux crates in the downstream manifests are
therefore rewritten to the exact version of the local checkout.

Features, `default-features`, `optional` etc. of the downstream dependencies
are kept. Existing `[patch.crates-io]` entries for libcrux crates (e.g. a git
rev pinned downstream) are replaced.

Usage:
    patch-libcrux-deps.py --libcrux PATH --downstream PATH
"""

import argparse
import json
import os
import subprocess
import sys

import tomllib

import tomlkit

DEP_TABLES = ("dependencies", "dev-dependencies", "build-dependencies")


def cargo_metadata(manifest_dir):
    out = subprocess.run(
        [
            "cargo",
            "metadata",
            "--no-deps",
            "--format-version",
            "1",
            "--manifest-path",
            os.path.join(manifest_dir, "Cargo.toml"),
        ],
        check=True,
        capture_output=True,
        text=True,
    ).stdout
    return json.loads(out)


def dep_tables(doc):
    """Yield every dependency table of a manifest, including target-specific
    and workspace ones."""
    for name in DEP_TABLES:
        if name in doc:
            yield doc[name]
    for target in doc.get("target", {}).values():
        for name in DEP_TABLES:
            if name in target:
                yield target[name]
    workspace = doc.get("workspace", {})
    if "dependencies" in workspace:
        yield workspace["dependencies"]


def rewrite_requirements(doc, local, changes, manifest):
    for table in dep_tables(doc):
        for key in list(table.keys()):
            spec = table[key]
            crate = key
            if isinstance(spec, dict) and "package" in spec:
                crate = str(spec["package"])
            if crate not in local:
                continue
            req = "=" + local[crate]["version"]
            if isinstance(spec, str):
                table[key] = req
            elif isinstance(spec, dict):
                if spec.get("workspace"):
                    continue  # handled in [workspace.dependencies]
                for src in ("git", "rev", "tag", "branch", "path", "registry"):
                    spec.pop(src, None)
                spec["version"] = req
            else:
                continue
            changes.append(f"{manifest}: {key} -> {req}")


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--libcrux", required=True, help="libcrux checkout")
    parser.add_argument("--downstream", required=True, help="downstream workspace root")
    args = parser.parse_args()

    libcrux = os.path.abspath(args.libcrux)
    downstream = os.path.abspath(args.downstream)

    local = {
        p["name"]: {
            "version": p["version"],
            "path": os.path.dirname(p["manifest_path"]),
        }
        for p in cargo_metadata(libcrux)["packages"]
        # Skip `publish = false` crates (fuzzers, benches, xtask, ...).
        if p.get("publish") != []
    }

    changes = []
    meta = cargo_metadata(downstream)
    root_manifest = os.path.join(meta["workspace_root"], "Cargo.toml")
    lockfile = os.path.join(meta["workspace_root"], "Cargo.lock")
    used = None
    if os.path.exists(lockfile):
        with open(lockfile, "rb") as f:
            used = {p["name"] for p in tomllib.load(f).get("package", [])}
    manifests = {root_manifest} | {p["manifest_path"] for p in meta["packages"]}

    for manifest in sorted(manifests):
        with open(manifest) as f:
            doc = tomlkit.parse(f.read())
        rewrite_requirements(doc, local, changes, manifest)

        if manifest == root_manifest:
            patch = doc.setdefault("patch", tomlkit.table(is_super_table=True))
            crates_io = patch.setdefault("crates-io", tomlkit.table())
            for name, info in sorted(local.items()):
                if used is not None and name not in used:
                    continue  # avoid "patch was not used" noise
                entry = tomlkit.inline_table()
                entry["path"] = info["path"]
                crates_io[name] = entry
            changes.append(f"{manifest}: [patch.crates-io] {len(crates_io)} entries")

        with open(manifest, "w") as f:
            f.write(tomlkit.dumps(doc))

    print("\n".join(changes))
    if not any("->" in c for c in changes):
        sys.exit("error: no libcrux dependency found downstream")


if __name__ == "__main__":
    main()
