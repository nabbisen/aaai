#!/usr/bin/env bash
#
# Bump the workspace version across every place it is declared.
#
# Direct hand-editing of Cargo.toml is discouraged (see RELEASING.md) — use
# this script so no declaration is missed.
#
# The substitutions below are deliberately tolerant of the column alignment
# used throughout these manifests (DEC-006 keeps them hand-aligned, and the
# columns move when the longest key changes). An earlier revision matched a
# fixed run of six spaces before `= {`, which silently stopped matching when
# the alignment drifted to sixteen. It still printed "Bumped to X" and exited
# 0, leaving the internal `aaai` dependency pinned to the previous version —
# which builds fine locally through the `path` dependency and only fails at
# `cargo publish -p aaai-cli`, where `^0.40.0` does not match a freshly
# published 0.41.0. Hence both the tolerant patterns and the verification at
# the end: this script must fail loudly rather than report a success it did
# not achieve.

set -euo pipefail

NEW=${1:-}
[[ "$NEW" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || { echo "Usage: bump-version.sh X.Y.Z" >&2; exit 1; }

cd "$(dirname "$0")/.."

# Workspace root: the workspace version, and the internal path dependency
# that carries a version for publishing.
sed -i -E "s/^(version[[:space:]]*=[[:space:]]*)\"[^\"]*\"/\1\"$NEW\"/" Cargo.toml
sed -i -E "s/^(aaai[[:space:]]*=[[:space:]]*\{[[:space:]]*version[[:space:]]*=[[:space:]]*)\"[^\"]*\"/\1\"$NEW\"/" Cargo.toml

# Per-crate manifests. These currently use `version.workspace = true`, so
# there is normally nothing to do — kept so a crate that opts out of the
# workspace version is still bumped rather than silently skipped.
for c in crates/aaai crates/aaai-cli crates/aaai-gui; do
  sed -i -E "s/^(version[[:space:]]*=[[:space:]]*)\"[^\"]*\"/\1\"$NEW\"/" "$c/Cargo.toml"
done

# Verify. Every declaration that carries a literal version must now be $NEW.
#
# Two forms deliberately carry no version literal and must not be flagged, or
# the check becomes noise and the next person disables it:
#   version.workspace = true        (per-crate, inherits the workspace version)
#   aaai = { workspace = true }     (inherits the whole dependency spec)
# Neither is matched below — the first because `version.` is not `version =`,
# the second because workspace-inherited lines are excluded explicitly.
fail=0
for f in Cargo.toml crates/aaai/Cargo.toml crates/aaai-cli/Cargo.toml crates/aaai-gui/Cargo.toml; do
  while IFS= read -r line; do
    echo "  $f: $line" >&2
    fail=1
  done < <(grep -nE "^(version|aaai)[[:space:]]*=" "$f" \
             | grep -vE "workspace[[:space:]]*=[[:space:]]*true" \
             | grep -v "\"$NEW\"" || true)
done

if [ "$fail" -ne 0 ]; then
  echo "bump-version: FAILED — the declarations above were not updated to $NEW." >&2
  echo "bump-version: the tree is partially modified; revert with 'git checkout -- Cargo.toml crates/*/Cargo.toml'." >&2
  exit 1
fi

echo "Bumped to $NEW"
grep -nE "^(version|aaai)[[:space:]]*=" Cargo.toml
