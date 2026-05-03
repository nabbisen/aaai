# aaai

[![crates.io](https://img.shields.io/crates/v/aaai?label=rust)](https://crates.io/crates/aaai)
[![License](https://img.shields.io/github/license/nabbisen/aaai)](https://github.com/nabbisen/aaai/blob/main/LICENSE)
[![Rust Documentation](https://docs.rs/aaai/badge.svg?version=latest)](https://docs.rs/aaai)
[![Dependency Status](https://deps.rs/crate/aaai/latest/status.svg)](https://deps.rs/crate/aaai)

**audit for asset integrity** — folder diff auditor with mandatory human-readable justification.

## Overview

aaai compares two directory trees, detects what changed, and audits the
differences against a YAML definition of expected changes. Every accepted
change requires a reason, making audit decisions traceable and explainable.

## Why aaai?

When you need to answer "is this change expected, and *why*?" — not just
"what changed?" — aaai turns raw diff output into a reviewable, storable
audit record.

**Use cases:** release artifact verification, config change auditing,
build output consistency checks, CI/CD gating.

## Quick Start

```sh
# Generate a definition template from the current diff
aaai snap --left ./before --right ./after --out audit.yaml

# Edit audit.yaml: fill in 'reason' for each entry

# Run the audit
aaai audit --left ./before --right ./after --config audit.yaml

# Generate a report
aaai report --left ./before --right ./after --config audit.yaml --out report.md
```

## Features / Design Notes

- **Mandatory reason**: a diff cannot be approved without a human-readable justification.
- **Content strategies**: None, Checksum, LineMatch, Regex, Exact.
- **CLI + GUI**: same judgement engine; GUI adds an interactive approval workflow.
- **Accessible by default**: color + icon + text for all status indicators.

For full documentation, see [docs/](docs/).
