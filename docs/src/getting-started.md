# Getting Started

## Installation

```sh
# Build from source — needs Rust 1.91 or later
cargo build --release -p aaai-cli -p aaai-gui

# Copy the binaries somewhere on your PATH
cp target/release/aaai ~/.local/bin/
cp target/release/aaai-gui ~/.local/bin/
```

---

## First-time setup (recommended: `aaai init`)

For a brand-new project, `aaai init` is the easiest starting point.

```sh
cd /your/project
aaai init
```

The interactive wizard asks for:

- The Before / After folder paths
- Where to put the audit definition
- Your approver name
- Whether to generate an initial snapshot

For CI or scripts, use `--non-interactive`:

```sh
aaai init --non-interactive --dir /path/to/project
```

---

## Manual setup (step by step)

If you'd rather skip the wizard and drive the four core commands by
hand, here's the canonical sequence.

### 1. Generate a snapshot

```sh
aaai snap --left ./before --right ./after --out audit.yaml
```

This produces an `audit.yaml` with one entry per detected change.
**Fill in the `reason` field on each entry** — entries with an empty
reason will be flagged Pending in the next step.

### 2. Run the audit

```sh
aaai audit --left ./before --right ./after --config audit.yaml
```

The verdict lands as one of:

- **PASSED** — every entry is allowed and the rules match
- **FAILED** — at least one entry's content doesn't match its rules
- **PENDING** — at least one entry still has an empty `reason`
  (use `--allow-pending` if you want Pending to count as success)

After the run, `aaai audit` prints a short "Next steps:" hint
pointing to whatever action is most useful given the verdict.

### 3. Inspect and fix problems

```sh
# Look at the raw diff in detail
aaai diff --left ./before --right ./after --content

# Best-practice lint checks on the definition
aaai lint audit.yaml
```

### 4. Generate a report

```sh
# Markdown report
aaai report --left ./before --right ./after --config audit.yaml --out report.md

# HTML report (browser-friendly)
aaai report --left ./before --right ./after --config audit.yaml \
            --format html --out report.html
```

For CI/CD pipelines, SARIF output integrates directly with GitHub
Pull Request reviews — see [CI/CD Integration](ci-integration.md).

---

## Using the GUI

```sh
aaai-gui
```

On the Opening screen, drag folders onto the cards (or use the
pickers), then click **Start audit**. The
[GUI Guide](gui.md) walks through the 3-pane workspace.

---

## Setting defaults with `.aaai.yaml`

Drop a `.aaai.yaml` at the project root to skip the most-repeated
flags:

```yaml
version: "1"
default_definition: "audit/audit.yaml"
default_ignore: "audit/.aaaiignore"
approver_name: "your-name"
mask_secrets: true
```

To scaffold one:

```sh
aaai config --init
```

---

## Shell completions

```sh
# Zsh
aaai completions zsh > ~/.zfunc/_aaai
echo 'fpath=(~/.zfunc $fpath)' >> ~/.zshrc
source ~/.zshrc
```

Bash, Fish, and PowerShell are also supported — see
[Setup & Tooling Commands](cli-setup.md).

---

## What to read next

- [CLI Reference](cli.md) — every subcommand explained
- [Audit Definition File](audit-definition.md) — the YAML schema
- [Content Audit Strategies](strategies.md) — None / Checksum /
  LineMatch / Regex / Exact
- [CI/CD Integration](ci-integration.md) — GitHub Actions workflow
- [GUI Guide](gui.md) — the desktop 3-pane workspace
- [Compatibility Policy](compatibility.md) — what's stable in v1.x
