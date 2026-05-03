# Getting Started

## Build

```sh
cargo build --release -p aaai-cli
cargo build --release -p aaai-gui
```

## First audit

```sh
aaai snap --left ./before --right ./after --out audit.yaml
# Edit audit.yaml: fill in every 'reason' field
aaai audit --left ./before --right ./after --config audit.yaml
aaai report --left ./before --right ./after --config audit.yaml --out report.md
```
