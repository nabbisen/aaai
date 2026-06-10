# Compatibility Policy (v1.x)

> **Status.** This document defines the compatibility contract that
> `aaai` guarantees during the v1.x series. It does not apply to the
> pre-1.0 series (v0.x), where breaking changes were normal.

## What "compatibility" means here

`aaai` follows [Semantic Versioning](https://semver.org/) with the
interpretation laid out below. The short version:

- **Patch** (`1.0.X`): bug fixes, no behaviour change for correct
  inputs, no schema change.
- **Minor** (`1.X.0`): new features. Existing CLI invocations, config
  files, GUI shortcuts, and i18n keys keep working unchanged. New
  fields and new commands may appear.
- **Major** (`2.0.0`): no compatibility guarantee. A new major may
  rename, remove, or restructure anything listed below.

Within v1.x — that is, between v1.0.0 and v1.99.x — the surfaces
listed below are stable. Patch and minor releases will not break them.

## CLI

### Subcommand names

These names are stable in v1.x:

| Command | Purpose |
|---|---|
| `aaai audit` | Run an audit |
| `aaai snap` | Generate a definition template from current diffs |
| `aaai report` | Emit a Markdown / JSON report |
| `aaai check` | Validate an audit definition |
| `aaai lint` | Lint an audit definition |
| `aaai diff` | Show raw file differences |
| `aaai merge` | Merge two audit definitions |
| `aaai init` | Scaffold a new audit project |
| `aaai watch` | Re-run audits when files change |
| `aaai history` | Inspect persisted audit runs |
| `aaai config` | Manage user preferences |
| `aaai dashboard` | One-screen project summary |
| `aaai completions` | Emit shell completions |
| `aaai export` | Export an audit run as a portable artifact |
| `aaai version` | Print the version |
| `aaai exit-codes` | List canonical exit codes |

Removing or renaming any of these would be a v2.0.0 change.

### Long options

Long options (e.g. `--reason`, `--strategy`, `--from`, `--to`,
`--format`, `--out`, `--quiet`, `--verbose`, `--json`) are stable.
Adding new long options is a minor change. Removing or renaming an
existing one is a major change.

### Short options

Short options (e.g. `-f`, `-t`, `-q`, `-v`) are **best-effort
stable**. When a short flag conflicts between subcommands, the long
form is canonical and the short form may be reassigned in a minor
release. The CHANGELOG will note any such reassignment.

### Exit codes

These exit codes are fixed and will not change in v1.x:

| Code | Meaning |
|---|---|
| 0 | PASSED — audit complete, all entries within tolerance |
| 1 | FAILED — at least one Failed entry |
| 2 | PENDING — at least one Pending entry that requires review |
| 3 | ERROR — a runtime error prevented completion |
| 4 | CONFIG_ERROR — the definition or CLI arguments were invalid |

`aaai exit-codes` prints this table at any time.

### Help text wording

`--help` wording may change between minor releases. If you script
against help output, parse the structured output instead (every
subcommand supports `--json` where appropriate). Help changes are
called out in the CHANGELOG.

## Configuration files

### `~/.aaai/profiles.yaml`

Stored profiles persist named `(before, after, definition, ignore)`
combinations.

- **Adding a field** to `AuditProfile` is a minor change. New fields
  carry `#[serde(default)]` so older files load cleanly.
- **Removing or renaming** a field is a major change.
- Files written by a newer minor version that include unknown fields
  remain readable by older v1.x versions: the unknown fields are
  silently ignored and preserved on rewrite when feasible.

### `~/.aaai/prefs.yaml`

User preferences (theme, locale, etc.). Same field-policy as
`profiles.yaml`.

### `~/.aaai/history.jsonl`

One JSON object per line per audit run. Readers must tolerate
unknown fields. Adding new fields is a minor change. The line-oriented
format itself is fixed.

### `audit.yaml` (project audit definition)

This file is the central contract between aaai and the project being
audited. Its schema is fully versioned via the `version:` field
(currently `version: 1`). Adding new fields under `version: 1` is a
minor change provided older readers can ignore them safely; any
schema migration that older readers cannot tolerate increments the
`version:` value and is described in the CHANGELOG.

`aaai check` is the canonical validator for this file.

## GUI

### Keyboard shortcuts

These are RFC-documented and stable in v1.x:

| Shortcut | Action |
|---|---|
| Ctrl+S | Save audit definition |
| Ctrl+R | Re-run audit |
| Ctrl+Z | Undo last approval |
| Ctrl+E | Export Markdown report |
| Arrow up / down | Move selection in the file tree |
| Enter | Focus the inspector's reason field |
| Escape | Deselect the current entry |

Adding a new shortcut is a minor change. Removing or remapping an
existing one is a major change.

### Pane structure

The 3-pane main screen (file tree / diff view / inspector) and the
top toolbar / bottom action bar are part of the v1.x contract.
Reordering or removing panes is a major change. Adding optional new
panes (e.g. via a sheet) is a minor change.

### i18n keys

Translation keys under namespaces like `error.*`, `banner.*`,
`empty_state.*`, `relative.*`, `opening.*`, `inspector.*`,
`toolbar.*`, `toast.*`, etc. are stable identifiers. Embedders or
contributors translating into new languages can rely on them.

- **Adding** a new key (e.g. for a new feature or error case) is a
  minor change.
- **Renaming or removing** a key is a major change. If wording needs
  to change, the value behind the key changes — the key itself stays.
- **Wording changes** to translated values are not breaking, but the
  CHANGELOG calls them out when they are non-trivial (e.g. a banner
  message rewording).

### Themes

The Light and Dark themes are stable. Theme rendering details may
change across minor releases (e.g. shade adjustments).

### Drag-and-drop

The Opening screen accepts folder drops in v1.x. Files (non-folders)
display an inline error. Future enhancements may add per-card hit
testing — that's an additive minor change.

## Library API (`aaai-core`)

`aaai-core` is the core audit engine. It is
[published on crates.io](https://crates.io/crates/aaai-core) and
its API documentation lives at
[docs.rs/aaai-core](https://docs.rs/aaai-core).

Inside the workspace, `aaai-cli` and `aaai-gui` are the canonical
consumers. External consumers can depend on `aaai-core` directly
for embedding the audit engine into other tools.

The public API is **best-effort stable** in v1.x — the same SemVer
interpretation used for the CLI and config files applies:

- **Adding** new public items (functions, types, fields with
  `#[serde(default)]`) is a minor change.
- **Renaming or removing** public items, or changing function
  signatures, is a major change.

Internal refactors that don't affect the public API are free.
Where a public-API change is unavoidable within v1.x (for example
to fix a soundness issue), it follows the same opt-in →
deprecation → next-major pipeline as the CLI, and the CHANGELOG
calls it out with a migration note.

## How breaking changes are handled

If a breaking change becomes necessary during v1.x — for example, a
security issue requires renaming an option — the process is:

1. The new behaviour is introduced in a minor release **as opt-in**
   (a flag, a new subcommand, or a new config field).
2. The old behaviour emits a deprecation warning for at least one
   minor version.
3. The CHANGELOG describes the migration.
4. The old behaviour is removed only in the next major release
   (v2.0.0).

This means a strictly forward-only migration path is guaranteed
within v1.x. If you pin to a specific v1.x version and migrate
forward one minor at a time, no flag day should ever surprise you.

## What is *not* covered by this contract

- **Performance.** Patch and minor releases may change runtime or
  memory characteristics. Benchmarks are not part of the contract.
- **Filesystem layout for derived artifacts.** Cache files, scratch
  directories, and intermediate report formats may change shape
  between minor releases.
- **Error message wording.** The structure (message + hint, exit
  code, JSON `error.code`) is stable; the human-readable text may
  improve across releases.
- **GUI pixel-perfect appearance.** Layout, spacing, colours, and
  animations may evolve across minor releases. The contracted parts
  are the structural elements listed under "Pane structure" above.
- **Screen-reader interoperability.** Deferred from v1.0; see the
  ABDD audit sheet for the declared limitation. A future RFC will
  cover this when the underlying GUI toolkit (iced) exposes
  accessibility hooks.
- **Languages beyond en and ja.** v1.0 ships English and Japanese.
  Adding new locales is a minor change.

## Versions before v1.0.0

Versions v0.x do not provide any compatibility guarantees. The v0
series was the development path toward v1.0; the CHANGELOG records
breaking changes that happened along the way. Users upgrading from
v0.19.0 should follow the upgrade notes in the v1.0.0 release entry.
