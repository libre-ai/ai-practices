# Security Policy

## Reporting a vulnerability

Report suspected vulnerabilities privately through GitHub Security Advisories ("Report a vulnerability" on this repository). Do not open public issues for security reports.

## Posture

- Fail-closed: security fixes take precedence over features.
- Supported version: `main` (pre-1.0, no LTS branches).
- Supply-chain baseline: CI actions are SHA-pinned; dependencies are watched by Dependabot and `cargo-deny`.
