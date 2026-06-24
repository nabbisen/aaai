# Setup & Tooling Commands

Commands for project initialisation, configuration, and shell integration.

## aaai init

Interactive project setup wizard.

```sh
aaai init [--dir <DIR>] [--non-interactive]
```

Pass `--non-interactive` to skip the prompts and generate `.aaai.yaml`
with default values. Useful in CI/CD scripts where no human input is
available.

---

## aaai config

Inspect or scaffold the project configuration file `.aaai.yaml`.

```sh
aaai config [--init] [--dir <DIR>] [--show]
```

- `--init` writes a starter `.aaai.yaml` (refuses to overwrite an
  existing file).
- `--show` prints the current effective configuration to stdout.

---

## aaai version

Print the version of `aaai`.

```sh
aaai version [--json-output]
```

With `--json-output`, emits a JSON object containing `version` and
`git_commit` fields — easier to parse from CI scripts.

---

## aaai exit-codes

Print the canonical exit-code table.

```sh
aaai exit-codes
```

| Code | Meaning |
|---|---|
| 0 | PASSED — audit complete, all entries within tolerance |
| 1 | FAILED — at least one Failed entry |
| 2 | PENDING — at least one Pending entry requires review |
| 3 | ERROR — a runtime error prevented completion |
| 4 | CONFIG_ERROR — definition or CLI arguments were invalid |

These codes are stable in v1.x — see the
[Compatibility Policy](compatibility.md) for the SemVer
interpretation. CI scripts can rely on the same numeric values
across minor and patch releases.

---

## aaai completions

Generate a shell-completion script.

```sh
aaai completions <bash|zsh|fish|powershell>
```

**Install examples:**

```sh
# Bash
aaai completions bash >> ~/.bash_completion

# Zsh
aaai completions zsh > ~/.zfunc/_aaai
echo 'fpath=(~/.zfunc $fpath)' >> ~/.zshrc

# Fish
aaai completions fish > ~/.config/fish/completions/aaai.fish
```

---

## Discovering next steps

Every aaai subcommand's `--help` output ends with a short "Next
steps:" block pointing to the typical follow-up commands for that
operation. For example:

```sh
$ aaai snap --help
...
Next steps:
  Review the generated audit.yaml and fill in reasons.
  Re-run with `aaai audit` to verify.
```

The top-level `aaai --help` includes a "Getting started:" block that
walks new users through `init → snap → audit → report`.
