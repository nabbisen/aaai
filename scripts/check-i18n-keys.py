#!/usr/bin/env python3
"""
scripts/check-i18n-keys.py

Verify that every i18n key used by t!() in crates/aaai-gui/src is present
in BOTH crates/aaai-gui/locales/en.yaml and ja.yaml.

Background. RFC 016 traced a class of bugs where t!("opening.title") would
render the literal string "opening.title" because the key was absent from
the loaded YAML. This script catches that statically, before the GUI is
ever launched. It complements RFC 017 (visual verification): static
checks find structural omissions, visual verification finds wiring bugs
and rendering surprises.

Reports four categories:

  MISSING    Key referenced in code, not present in a locale YAML.
             This is always a bug (the user will see a literal key).

  UNUSED     Key present in YAML, never referenced by t!() in code.
             Dead translation. Informational by default; treated as
             an error under --strict.

  DIVERGENT  Key present in one locale's YAML but not the other.
             Implies inconsistent translation coverage.

  SKIPPED    A dynamic t!(var) call site whose key cannot be resolved
             statically. Listed for awareness only.

Exit codes:
  0  all dotted keys are present in both locales (UNUSED-only is OK)
  1  at least one MISSING or DIVERGENT key
  2  setup error (wrong cwd, PyYAML unavailable, etc.)

Usage:
  scripts/check-i18n-keys.py           # full report
  scripts/check-i18n-keys.py --quiet   # summary line only
  scripts/check-i18n-keys.py --strict  # also fail on UNUSED entries

The script is intentionally pure-Python (no grep -P, no external tools)
so it runs identically on every platform supported by aaai.
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
LOCALES_DIR = REPO_ROOT / "crates" / "aaai-gui" / "locales"
SRC_DIR = REPO_ROOT / "crates" / "aaai-gui" / "src"

# Match `t!("dotted.namespace.key")` calls.
#
# `(?<![A-Za-z0-9_])` is a negative lookbehind that rejects `format!("...")`,
# `print!("...")`, `panic!("...")` and similar — anything where an identifier
# character precedes the `t!`. Without it, the substring "t!(" inside
# `format!(` would match and produce false positives.
T_BANG_RE = re.compile(r'(?<![A-Za-z0-9_])t!\("([^"]+)"')

# A key is considered structurally valid if it's a dotted-lowercase namespace,
# e.g. "opening.title" or "diff.legend_added". Everything else (raw doc-comment
# examples, freeform fallback strings) is treated as SKIPPED rather than
# MISSING — those strings legitimately need not appear in YAML.
DOTTED_KEY_RE = re.compile(r"^[a-z][a-z0-9_]*(\.[a-z0-9_]+)+$")

# A line starting with `//` (after leading whitespace) is a Rust line comment;
# we ignore matches found on such lines so example usages in rustdoc don't
# pollute the key set.
COMMENT_PREFIX_RE = re.compile(r"^\s*//")


def die(msg: str, code: int = 2):
    print(f"check-i18n-keys: {msg}", file=sys.stderr)
    sys.exit(code)


def extract_keys_from_code(src_dir: Path) -> tuple[set[str], set[str], list[str]]:
    """Return (dotted_keys, non_dotted_first_args, dynamic_call_locations).

    Scans each `.rs` file holistically (not line-by-line) so multi-line
    `t!(\\n    "key",\\n    arg = value\\n)` calls — which `rustfmt` may
    produce for long argument lists — are detected correctly. Line comments
    starting with `//` are stripped before regex matching so example usages
    in rustdoc don't pollute the key set.
    """
    dotted: set[str] = set()
    non_dotted: set[str] = set()
    dynamic: list[str] = []

    for rs_file in sorted(src_dir.rglob("*.rs")):
        raw = rs_file.read_text(encoding="utf-8")
        # Strip per-line `//` comments while keeping line numbers intact, so
        # the file's offset-to-line mapping for `dynamic` reports is correct.
        stripped_lines = []
        for line in raw.splitlines():
            m = COMMENT_PREFIX_RE.match(line)
            if m:
                stripped_lines.append("")
            else:
                # Strip mid-line `// ...` comments too (cheap heuristic: the
                # first `//` outside a string literal). Rust strings can
                # contain `//`; we tolerate that by being conservative.
                idx = line.find("//")
                if idx >= 0 and line.count('"', 0, idx) % 2 == 0:
                    stripped_lines.append(line[:idx])
                else:
                    stripped_lines.append(line)
        content = "\n".join(stripped_lines)

        # Match t!() calls across line boundaries. The `re.DOTALL` flag lets
        # `\s*` match newlines between `t!(` and the opening quote.
        for m in re.finditer(
            r'(?<![A-Za-z0-9_])t!\(\s*"([^"]+)"',
            content,
            re.DOTALL,
        ):
            key = m.group(1)
            if DOTTED_KEY_RE.match(key):
                dotted.add(key)
            else:
                non_dotted.add(key)

        # Detect dynamic calls: `t!(` followed by something that isn't a
        # string literal (and isn't whitespace before a string literal).
        file_has_dynamic = False
        for m in re.finditer(
            r'(?<![A-Za-z0-9_])t!\(\s*([^"\s])',
            content,
            re.DOTALL,
        ):
            first_char = m.group(1)
            if first_char != '"':
                # Approximate the line number from the match offset.
                line_no = content.count("\n", 0, m.start()) + 1
                rel = rs_file.relative_to(REPO_ROOT)
                dynamic.append(f"{rel}:{line_no}")
                file_has_dynamic = True

        # If this file contains a dynamic t!() call (e.g.
        # `make_btn("filter.all", ...)` → `t!(label)`), the runtime
        # resolution can read any dotted-key string literal in the
        # same file. Conservatively collect every dotted-key-shaped
        # string literal so we don't falsely flag those keys as UNUSED.
        # This errs on the side of "less false positives in unused",
        # which is the right trade-off: missing a dead key only leaves
        # a stale YAML entry, while false-positive-unused causes us
        # to delete a key that's actually translated.
        if file_has_dynamic:
            for m in re.finditer(r'"([a-z][a-z0-9_]*(?:\.[a-z0-9_]+)+)"', content):
                key = m.group(1)
                if DOTTED_KEY_RE.match(key):
                    dotted.add(key)

    return dotted, non_dotted, dynamic


def flatten_yaml(path: Path) -> set[str]:
    """Return the set of dotted leaf keys in a YAML mapping file."""
    try:
        import yaml  # type: ignore[import]
    except ImportError:
        die("PyYAML is required (`pip install pyyaml` or apt install python3-yaml)")

    data = yaml.safe_load(path.read_text(encoding="utf-8")) or {}
    if not isinstance(data, dict):
        die(f"{path}: top-level YAML must be a mapping, got {type(data).__name__}")

    leaves: set[str] = set()

    def walk(node: object, prefix: str) -> None:
        if isinstance(node, dict):
            for k, v in node.items():
                p = f"{prefix}.{k}" if prefix else str(k)
                walk(v, p)
        else:
            leaves.add(prefix)

    walk(data, "")
    return leaves


def section(title: str, items: set[str] | list[str], quiet: bool) -> None:
    if not items or quiet:
        return
    print(f"=== {title} ({len(items)}) ===")
    for item in sorted(items):
        print(f"  {item}")
    print()


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Cross-check t!() i18n keys against locale YAML files.",
    )
    parser.add_argument("--quiet", action="store_true", help="print only the summary line")
    parser.add_argument("--strict", action="store_true", help="exit non-zero if any UNUSED entries exist")
    args = parser.parse_args()

    if not LOCALES_DIR.is_dir():
        die(f"{LOCALES_DIR} not found (run from repository root)")
    if not SRC_DIR.is_dir():
        die(f"{SRC_DIR} not found (run from repository root)")

    en_path = LOCALES_DIR / "en.yaml"
    ja_path = LOCALES_DIR / "ja.yaml"
    if not en_path.exists() or not ja_path.exists():
        die(f"expected {en_path} and {ja_path}")

    dotted_keys, non_dotted, dynamic = extract_keys_from_code(SRC_DIR)
    if not dotted_keys:
        die("no dotted-namespace t!() keys found — regex change or empty source?")

    en_keys = flatten_yaml(en_path)
    ja_keys = flatten_yaml(ja_path)

    missing_en = dotted_keys - en_keys
    missing_ja = dotted_keys - ja_keys
    unused_en = en_keys - dotted_keys
    unused_ja = ja_keys - dotted_keys
    only_en = en_keys - ja_keys
    only_ja = ja_keys - en_keys

    section("MISSING in en.yaml (referenced in code, absent from YAML)", missing_en, args.quiet)
    section("MISSING in ja.yaml", missing_ja, args.quiet)
    section("DIVERGENT — present in en.yaml only", only_en, args.quiet)
    section("DIVERGENT — present in ja.yaml only", only_ja, args.quiet)
    section("UNUSED in en.yaml (key in YAML, never called)", unused_en, args.quiet)
    section("UNUSED in ja.yaml", unused_ja, args.quiet)

    if non_dotted and not args.quiet:
        print(f"=== SKIPPED non-dotted t!() first-args ({len(non_dotted)}) ===")
        print("These are either documentation examples or freeform fallback strings;")
        print("they do not require YAML entries.")
        for s in sorted(non_dotted):
            print(f"  {s!r}")
        print()

    if dynamic and not args.quiet:
        print(f"=== SKIPPED dynamic t!() call sites ({len(dynamic)}) ===")
        print("These resolve the key at runtime and cannot be checked statically:")
        for loc in dynamic:
            print(f"  {loc}")
        print()

    missing_count = len(missing_en) + len(missing_ja)
    divergent_count = len(only_en) + len(only_ja)
    unused_count = len(unused_en) + len(unused_ja)

    print(
        f"i18n key audit: "
        f"{missing_count} missing, "
        f"{divergent_count} divergent, "
        f"{unused_count} unused. "
        f"(code: {len(dotted_keys)} keys; en: {len(en_keys)}; ja: {len(ja_keys)})"
    )

    if missing_count > 0:
        return 1
    if divergent_count > 0:
        return 1
    if args.strict and unused_count > 0:
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
