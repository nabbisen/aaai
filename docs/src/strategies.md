# Content Audit Strategies

aaai provides five strategies for content-level auditing. The strategy is
chosen per entry in the audit definition.

## None

Checks only that the expected diff type occurred. No content inspection.

```yaml
strategy:
  type: None
```

**When to use:** File additions/deletions where content doesn't need to be
verified; or as a placeholder while you define more specific rules.

**Caution:** Do not use for important configuration files without adding a
more specific strategy later.

## Checksum

Verifies the file's SHA-256 digest matches an expected value.

```yaml
strategy:
  type: Checksum
  expected_sha256: "abc123...64hexchars"
```

**When to use:** Binary files (images, archives, compiled assets), or any
file where byte-for-byte identity must be confirmed.

**Getting the hash:**
```sh
sha256sum myfile.bin   # Linux
shasum -a 256 myfile   # macOS
```

## LineMatch

Verifies that specific lines were added and/or removed.

```yaml
strategy:
  type: LineMatch
  rules:
    - action: Removed
      line: "port = 80"
    - action: Added
      line: "port = 8080"
```

**When to use:** Configuration value changes. The most common strategy for
TOML, YAML, `.env`, and INI files.

**Rules:** Each rule checks for one exact line. Order does not matter.
Whitespace is significant (leading/trailing spaces must match).

## Regex

Verifies that changed lines match a regular expression pattern.

```yaml
strategy:
  type: Regex
  pattern: "^version = \"\\d+\\.\\d+\\.\\d+\""
  target: AddedLines   # AddedLines | RemovedLines | AllChangedLines
```

**When to use:** Version numbers, dates, or any value that changes
predictably but cannot be pinned to a single exact value.

**Target options:**
- `AddedLines` — pattern applied to lines only in the *after* file
- `RemovedLines` — pattern applied to lines only in the *before* file
- `AllChangedLines` — pattern applied to all changed lines

## Exact

Verifies that the after-file's full content exactly matches expected text.

```yaml
strategy:
  type: Exact
  expected_content: |
    [server]
    host = "0.0.0.0"
    port = 8080
```

**When to use:** Small, stable files where any deviation is unacceptable.

**Caution:** Avoid for files larger than a few KB. Hard to maintain when the
file changes frequently.
