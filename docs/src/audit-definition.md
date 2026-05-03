# Audit Definition File

YAML document that stores expected differences.

```yaml
version: "1"
meta:
  description: "Release v2.3.0 audit"
entries:
  - path: "config/server.toml"
    diff_type: Modified
    reason: "Port changed — INF-42"
    strategy:
      type: LineMatch
      rules:
        - action: Removed
          line: "port = 80"
        - action: Added
          line: "port = 8080"
    enabled: true
```
