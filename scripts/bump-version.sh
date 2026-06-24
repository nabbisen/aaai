#!/usr/bin/env bash
set -euo pipefail
NEW=$1
[[ "$NEW" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || { echo "Usage: bump-version.sh X.Y.Z"; exit 1; }
# Workspace root — both the workspace version and the internal aaai dep
sed -i "s/^version = \"[^\"]*\"/version = \"$NEW\"/" Cargo.toml
sed -i "s/aaai      = { version = \"[^\"]*\"/aaai      = { version = \"$NEW\"/" Cargo.toml
# Per-crate Cargo.toml files
for c in crates/aaai crates/aaai-cli crates/aaai-gui; do
  sed -i "s/^version = \"[^\"]*\"/version = \"$NEW\"/" "$c/Cargo.toml"
done
echo "Bumped to $NEW"
