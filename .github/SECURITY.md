# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 3.x.x   | :white_check_mark: |
| < 3.0   | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability in OpenSecKit, please report it responsibly.

### How to Report

1. **Do NOT open a public issue**
2. Open a [private security advisory](https://github.com/Scttpr/OpenSecKit/security/advisories/new) on GitHub
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

### What to Expect

- **Acknowledgment**: Within 48 hours
- **Initial Assessment**: Within 7 days
- **Resolution Timeline**: Depends on severity
  - Critical: 7 days
  - High: 14 days
  - Medium: 30 days
  - Low: 90 days

### Scope

This policy applies to:
- The `osk` CLI binary
- Prompt templates and security content
- GitHub Actions workflows

### Out of Scope

- Third-party dependencies (report to upstream)
- Social engineering attacks
- Denial of service attacks

## Security Measures

OpenSecKit implements:
- Weekly dependency audits (`cargo-audit`, `cargo-deny`)
- SAST scanning (Semgrep)
- Secret detection (Gitleaks)
- Multi-platform testing

## Acknowledgments

We thank security researchers who responsibly disclose vulnerabilities.
