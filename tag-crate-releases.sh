#!/usr/bin/env bash
#
# tag-crate-releases.sh — tag cargo workspace crates and draft GitHub releases.
#
# For each crate name given, looks up its version via `cargo metadata`
# (this correctly resolves `version.workspace = true` inheritance), then:
#
#   1. Creates an annotated git tag (if it doesn't exist yet):
#        tag:     <crate>-v<version>
#        message: Release: <crate> v<version>
#   2. Pushes that tag to `origin`.
#   3. Creates a draft GitHub release for the tag (if one doesn't exist yet):
#        title: <crate> v<version>
#        notes: Please see [CHANGELOG.md](<link>) for the most important
#               changes since the last release.
#      where <link> is a permalink (pinned to the tagged commit's SHA) to
#      the CHANGELOG.md located next to the crate's Cargo.toml.
#
# The script is idempotent: rerunning it after a partial failure (e.g. a
# failed push or release creation) resumes where it left off instead of
# bailing out on already-created tags.
#
# Usage:
#   ./tag-crate-releases.sh <crate-name> [<crate-name> ...]
#
# Requirements: cargo, git, jq, and the GitHub CLI (`gh` >= 2.15,
# authenticated). Must be run from within the cargo workspace (or a
# subdirectory of it) inside the corresponding git repository, with
# `origin` pointing at the GitHub repo.

set -euo pipefail
set -x

usage() {
  echo "Usage: $0 <crate-name> [<crate-name> ...]" >&2
  echo "Creates annotated git tags '\$crate-v\$version' and draft GitHub" >&2
  echo "releases for each given crate in the current cargo workspace." >&2
  exit 1
}

if [[ $# -eq 0 ]]; then
  usage
fi

for bin in cargo jq git gh; do
  if ! command -v "$bin" >/dev/null 2>&1; then
    echo "Error: '$bin' not found in PATH" >&2
    exit 1
  fi
done

if ! git rev-parse --git-dir >/dev/null 2>&1; then
  echo "Error: not inside a git repository" >&2
  exit 1
fi

repo_root=$(git rev-parse --show-toplevel)

# Determine the GitHub repo from the 'origin' remote (the same remote we
# push tags to), rather than gh's "default repo", which may point elsewhere.
# Handles: https://github.com/owner/repo(.git), git@github.com:owner/repo(.git),
# ssh://git@github.com/owner/repo(.git)
if ! origin_url=$(git remote get-url origin 2>/dev/null); then
  echo "Error: git remote 'origin' is not configured" >&2
  exit 1
fi

normalized=${origin_url%/}
normalized=${normalized%.git}
if [[ $normalized =~ ^https?://([^@/]+@)?([^/:]+)/(.+)$ ]]; then
  gh_host=${BASH_REMATCH[2]}
  gh_path=${BASH_REMATCH[3]}
elif [[ $normalized =~ ^ssh://([^@/]+@)?([^/:]+)(:[0-9]+)?/(.+)$ ]]; then
  gh_host=${BASH_REMATCH[2]}
  gh_path=${BASH_REMATCH[4]}
elif [[ $normalized != *"://"* && $normalized =~ ^([^@/]+@)?([^/:]+):(.+)$ ]]; then
  # scp-style syntax, e.g. git@github.com:owner/repo
  gh_host=${BASH_REMATCH[2]}
  gh_path=${BASH_REMATCH[3]}
else
  echo "Error: cannot parse 'origin' remote URL: ${origin_url}" >&2
  exit 1
fi

repo_web_url="https://${gh_host}/${gh_path}"
gh_repo="${gh_host}/${gh_path}"   # HOST/OWNER/REPO form accepted by gh -R

# Fetch workspace metadata once. --no-deps restricts the package list to
# workspace members only, so external dependencies with clashing names
# won't be matched.
if ! metadata=$(cargo metadata --no-deps --format-version 1); then
  echo "Error: 'cargo metadata' failed" >&2
  exit 1
fi

workspace_root=$(jq -r '.workspace_root' <<<"$metadata")

exit_code=0

for crate in "$@"; do
  # Version and manifest path in a single jq pass; \u001f (unit separator)
  # avoids ambiguity with any printable character in paths.
  pkg_info=$(jq -r --arg name "$crate" \
    'first(.packages[] | select(.name == $name))
     | [.version, .manifest_path] | join("\u001f")' <<<"$metadata" || true)

  if [[ -z "$pkg_info" || "$pkg_info" == "null" ]]; then
    echo "Error: crate '$crate' not found in workspace" >&2
    exit_code=1
    continue
  fi

  version=${pkg_info%%$'\x1f'*}
  manifest_path=${pkg_info#*$'\x1f'}
  crate_dir=$(dirname "$manifest_path")

  # The version was read from the working tree, but the tag will point at
  # HEAD. Refuse if the manifests that determine the version have
  # uncommitted changes, since the tag/permalink would then be wrong.
  if [[ -n "$(git status --porcelain -- "$manifest_path" "${workspace_root}/Cargo.toml")" ]]; then
    echo "Error: '$crate': uncommitted changes in Cargo.toml — commit the" >&2
    echo "  version bump first so the tag matches the tagged commit" >&2
    exit_code=1
    continue
  fi

  # Path of the crate's CHANGELOG.md relative to the repo root, for the link.
  case "$crate_dir" in
    "$repo_root")   changelog_rel_path="CHANGELOG.md" ;;
    "$repo_root"/*) changelog_rel_path="${crate_dir#"$repo_root"/}/CHANGELOG.md" ;;
    *)
      echo "Error: '$crate': crate directory '$crate_dir' is outside the git repository" >&2
      exit_code=1
      continue
      ;;
  esac

  if [[ ! -f "${crate_dir}/CHANGELOG.md" ]]; then
    echo "Warning: ${changelog_rel_path} does not exist locally (linking anyway)" >&2
  elif [[ -n "$(git status --porcelain -- "${crate_dir}/CHANGELOG.md")" ]]; then
    echo "Warning: ${changelog_rel_path} has uncommitted changes; the permalink" >&2
    echo "  will show the committed version" >&2
  fi

  tag="${crate}-v${version}"
  message="Release: ${crate} v${version}"

  if git rev-parse -q --verify "refs/tags/${tag}" >/dev/null; then
    echo "Note: tag '${tag}' already exists locally, reusing it"
  else
    git tag -a "$tag" -m "$message"
    echo "Created tag: $tag ($message)"
  fi

  # Idempotent: succeeds (up to date) if the remote already has this tag at
  # the same commit; fails if the remote has it at a different commit.
  if ! git push origin "refs/tags/${tag}"; then
    echo "Error: failed to push tag '${tag}' to origin" >&2
    exit_code=1
    continue
  fi

  if gh release view "$tag" -R "$gh_repo" >/dev/null 2>&1; then
    echo "Note: GitHub release for '${tag}' already exists, skipping"
    continue
  fi

  # Pin the CHANGELOG.md permalink to the exact commit the tag points at.
  sha=$(git rev-parse "${tag}^{commit}")
  changelog_link="${repo_web_url}/blob/${sha}/${changelog_rel_path}"

  release_title="${crate} v${version}"
  release_notes="Please see [CHANGELOG.md](${changelog_link}) for the most important changes since the last release."

  if gh release create "$tag" \
      -R "$gh_repo" \
      --verify-tag \
      --title "$release_title" \
      --notes "$release_notes" \
      --draft; then
    echo "Created draft GitHub release for tag: $tag"
  else
    echo "Error: failed to create GitHub release for tag '${tag}'" >&2
    exit_code=1
  fi
done

exit "$exit_code"
