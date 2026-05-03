# aaai — audit for asset integrity

[![crates.io](https://img.shields.io/crates/v/aaai?label=rust)](https://crates.io/crates/aaai)
[![License](https://img.shields.io/github/license/nabbisen/aaai)](https://github.com/nabbisen/aaai/blob/main/LICENSE)
[![Rust Documentation](https://docs.rs/aaai/badge.svg?version=latest)](https://docs.rs/aaai)
[![Dependency Status](https://deps.rs/crate/aaai/latest/status.svg)](https://deps.rs/crate/aaai)
[![CI](https://github.com/nabbisen/aaai/actions/workflows/ci.yaml/badge.svg)](/.github/workflows/ci.yaml)

**aaai** is a folder-diff auditor that requires every detected change to carry a human-readable reason before it can be marked as accepted. It provides a CLI and a desktop GUI built with [iced](https://github.com/iced-rs/iced).

## Overview

| Feature | Description |
|---|---|
| Folder diff | Parallel comparison of two directory trees |
| Audit definition | YAML file listing expected changes with reasons |
| Content strategies | None / Checksum / LineMatch / Regex / Exact |
| Reports | Markdown, JSON, HTML, SARIF (GitHub Actions) |
| GUI | 3-pane resizable desktop UI with dark/light theme |
| CLI | 15 commands — audit, snap, lint, diff, merge, watch, … |
| CI/CD | Granular exit codes (0 = PASSED … 4 = CONFIG_ERROR) |

## Quick Start

```sh
# Generate a definition template from the current diff
aaai snap --left ./before --right ./after --out audit.yaml

# Fill in the "reason" fields, then audit
aaai audit --left ./before --right ./after --config audit.yaml
```

## Crates

| Crate | Description |
|---|---|
| [`aaai-core`](crates/aaai-core) | Core engine — diff, audit, report, masking |
| [`aaai-cli`](crates/aaai-cli) | `aaai` command-line binary |
| [`aaai-gui`](crates/aaai-gui) | `aaai-gui` desktop application |

## Installation

```sh
# Build from source (requires Rust 1.81+)
cargo build --release -p aaai-cli -p aaai-gui
```

## Documentation

- [Getting Started](docs/src/getting-started.md)
- [CLI Reference](docs/src/cli.md)
- [Content Audit Strategies](docs/src/strategies.md)
- [GUI Guide](docs/src/gui.md)
- [CI/CD Integration](docs/src/ci-integration.md)
- [FAQ](docs/src/faq.md)

## License

Apache-2.0 — see [LICENSE](LICENSE) and [NOTICE](NOTICE).
