# Content Audit Strategies

aaai provides five strategies for content-level auditing.
The strategy is chosen per entry in the audit definition.

---

## None

Checks only that the expected diff type occurred. No content inspection.

```yaml
strategy:
  type: None
```

**When to use:** File additions/deletions where content doesn't need to be
verified; or as a placeholder while you define more specific rules.

**Note:** Consider replacing with a more specific strategy for important
configuration files.

---

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

---

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
Whitespace (leading/trailing) is significant.

---

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

**`target` options:**

| Value | Description |
|---|---|
| `AddedLines` | Pattern applied only to lines in the *after* file (default) |
| `RemovedLines` | Pattern applied only to lines in the *before* file |
| `AllChangedLines` | Pattern applied to all changed lines |

---

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

**Caution:** Avoid for files larger than a few KB or files that change
frequently — maintenance becomes difficult.

---

## Choosing a Strategy

| File type | Recommended |
|---|---|
| Binary, image, archive | Checksum |
| Config file (key = value changes) | LineMatch |
| Version numbers, dates | Regex |
| Addition / removal (content irrelevant) | None |
| Small, stable text file | Exact |

---

## Large File Warning

Applying **Exact** or **LineMatch** to a file larger than 1 MB triggers
an `AuditWarning::LargeFileStrategy` advisory.
Consider **Checksum** for large files instead.
