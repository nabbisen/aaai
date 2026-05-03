# Overview

**aaai** (audit for asset integrity) is a folder diff auditor.

It compares two directory trees and audits the differences against a YAML
definition of expected changes.  Every accepted change requires a human-readable
reason, making audit decisions traceable and explainable.

## Key concepts

| Term | Meaning |
|------|---------|
| Before | The source / reference folder |
| After | The target / current folder |
| Audit definition | YAML file describing expected differences |
| Reason | Mandatory justification for each expected change |
| Strategy | Content-audit method (None / Checksum / LineMatch / Regex / Exact) |
