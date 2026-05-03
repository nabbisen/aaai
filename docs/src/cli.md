# CLI Reference

## audit

Compare two folders against an audit definition.

```
aaai audit --left <PATH> --right <PATH> --config <FILE>
```

Exit codes: `0` = all OK, `1` = Failed / Pending / Error present.

## snap

Generate an audit definition template.

```
aaai snap --left <PATH> --right <PATH> --out <FILE> [--merge]
```

## report

Output a Markdown or JSON audit report.

```
aaai report --left <PATH> --right <PATH> --config <FILE> --out <FILE> [--format markdown|json]
```
