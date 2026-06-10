# Workflow Commands

Commands for managing history, merging definitions, and automating
audit workflows.

## aaai merge

Merge two audit definition files into one.

```sh
aaai merge <BASE> <OVERLAY> [--out <FILE>] [OPTIONS]
```

| Flag | Description |
|---|---|
| `--out <FILE>` | Output path. When omitted, BASE is overwritten in place. |
| `--detect-conflicts` | Detect conflicts only; don't actually merge. |
| `--dry-run` | Print what would change, write nothing. |

Useful when several team members maintain separate slices of an
audit definition and you want to combine them before shipping.

---

## aaai history

Show the recent audit-run history persisted in
`~/.aaai/history.jsonl`.

```sh
aaai history [-n <N>] [--stats] [--json-output]
```

| Flag | Description |
|---|---|
| `-n <N>` | How many entries to show (default: 10) |
| `--stats` | Show pass-rate, average counts, and a short trend summary |
| `--json-output` | Emit history as one JSON object per line |

The on-disk format of `history.jsonl` is part of the v1.x
[Compatibility Policy](compatibility.md#configuration-files):
fields may be added, but the line-oriented shape is fixed.

---

## aaai dashboard

Show a one-screen audit summary as colour-coded cards.

```sh
aaai dashboard --left <BEFORE> --right <AFTER> --config <FILE> [--detail]
```

`--detail` expands each card with the top entries needing attention.

The dashboard output ends with a short "Next steps:" hint pointing
at whichever action will move the audit forward — typically
either filling in Pending reasons, fixing Failed rules, or
exporting a report when everything is clean.

---

## aaai watch

Watch the Before and After folders for file changes and re-run the
audit automatically.

```sh
aaai watch --left <BEFORE> --right <AFTER> --config <FILE> \
           [--debounce-ms <MS>]
```

`--debounce-ms` (default: 250) controls how long the watcher coalesces
rapid file events before triggering a re-run. Raise it for noisy
build directories; lower it for snappier feedback.

Press Ctrl+C to stop the watcher.
