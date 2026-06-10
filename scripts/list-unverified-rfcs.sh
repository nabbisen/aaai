#!/usr/bin/env bash
# scripts/list-unverified-rfcs.sh
#
# List RFCs in rfcs/done/ that do not yet carry a Visual Verification card.
#
# Background: RFC 017 (Visual Verification Harness) requires every shipped RFC
# whose implementation has a visible surface (UI, CLI output, screen layout)
# to carry a "## Visual Verification" section. That section is added by a human
# who has run the relevant build and confirmed it matches the design document
# `aaai_uiux_design.pdf`. This script reports which RFCs still lack that card,
# so contributors can see at a glance what remains to verify.
#
# RFC 000 (the lifecycle policy itself) has no UI surface and is exempt.
# RFCs 001–006 are P3 in RFC 017 §2.4 and are listed but not enforced.
#
# Usage:
#   scripts/list-unverified-rfcs.sh             # report; always exit 0
#   scripts/list-unverified-rfcs.sh --strict    # exit 1 if any unverified
#   scripts/list-unverified-rfcs.sh --quiet     # print only the summary line
#   scripts/list-unverified-rfcs.sh -h|--help   # show this header
#
# Output format (one RFC per line, then a summary):
#   015-opening-screen-redesign: UNVERIFIED
#   016-i18n-repair: UNVERIFIED
#   ...
#
#   Visual verification: N / M RFC(s) unverified.

set -euo pipefail

strict=0
quiet=0

print_help() {
    sed -n '2,/^$/p' "$0" | sed -e 's/^# \{0,1\}//'
}

for arg in "$@"; do
    case "$arg" in
        --strict) strict=1 ;;
        --quiet)  quiet=1 ;;
        -h|--help)
            print_help
            exit 0
            ;;
        *)
            echo "unknown argument: $arg" >&2
            echo "see --help" >&2
            exit 2
            ;;
    esac
done

# cd to repository root (parent of scripts/)
cd "$(dirname "$0")/.."

if [ ! -d rfcs/done ]; then
    echo "rfcs/done/ not found (run from repository root)" >&2
    exit 2
fi

unverified=0
total=0
unverified_list=()

shopt -s nullglob
for f in rfcs/done/[0-9]*.md; do
    total=$((total + 1))
    slug=$(basename "$f" .md)

    # Exempt the lifecycle-policy RFC itself; it has no UI to verify.
    if [[ "$slug" == 000-* ]]; then
        continue
    fi

    if ! grep -q "^## Visual Verification" "$f"; then
        unverified_list+=("$slug")
        unverified=$((unverified + 1))
    fi
done
shopt -u nullglob

if [ "$quiet" -eq 0 ] && [ "$unverified" -gt 0 ]; then
    for slug in "${unverified_list[@]}"; do
        echo "$slug: UNVERIFIED"
    done
    echo
fi

echo "Visual verification: $unverified / $total RFC(s) unverified (RFC 000 excluded)."

if [ "$strict" -eq 1 ] && [ "$unverified" -gt 0 ]; then
    exit 1
fi
exit 0
