# Secure Phase

> Shift-left security: embed security into feature development from the start.

## Overview

The Secure phase applies the **7 Security Principles** to feature development through a structured workflow. It transforms security from an afterthought into a first-class requirement.

```
┌─────────────────────────────────────────────────────────────────┐
│                        SECURE WORKFLOW                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   specify ──► clarify ──► plan ──► tasks ──► implement          │
│      │                                                          │
│      ▼                                                          │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │  7 PRINCIPLES (applied via weight system)               │   │
│   │                                                         │   │
│   │  I.   Threat Modeling      V.   Secrets Management      │   │
│   │  II.  Risk Analysis        VI.  Audit Logging           │   │
│   │  III. Security by Design   VII. Patch Management        │   │
│   │  IV.  Security Testing                                  │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Commands

| Command | Role | Output |
|---------|------|--------|
| `osk-secure specify <feature>` | Security Architect | `security-spec.md`, `risks.yaml` |
| `osk-secure clarify <feature>` | Security Architect | Updated spec with clarifications |
| `osk-secure plan <feature>` | Security Engineer | `security-plan.md` |
| `osk-secure tasks <feature>` | Security Engineer | `tasks.md` |
| `osk-secure implement <feature>` | Security Engineer | Implementation |

## The 7 Principles

### I. Threat Modeling

Every feature undergoes STRIDE threat analysis before development.

- **Methodology**: STRIDE (Spoofing, Tampering, Repudiation, Info Disclosure, DoS, Elevation)
- **Deliverables**: DFD with trust boundaries, STRIDE analysis, Attack trees
- **Knowledge**: `knowledge/principles/I-threat-modeling/`

### II. Risk Analysis

Each risk is scored and tracked in a central register.

- **Methodology**: Score = Impact × Probability × Exposure (scale 1-5, max 125)
- **Thresholds**: Critical ≥75 (48h SLA), Important 25-74 (7d SLA), Minor <25 (30d SLA)
- **Knowledge**: `knowledge/principles/II-risk-analysis/`

### III. Security by Design

Security controls are first-class requirements.

- **Fundamentals**: Least privilege, Defense in depth, Fail secure, Input validation
- **Methodology**: OWASP ASVS mapping
- **Knowledge**: `knowledge/principles/III-security-design/`

### IV. Security Testing

Automated tests, blocking for deployment.

- **Types**: SAST (every commit), DAST (staging), SCA (daily)
- **Rule**: No CRITICAL or HIGH vulnerabilities in production
- **Knowledge**: `knowledge/principles/IV-security-testing/`

### V. Secrets Management

No secrets in code. Ever.

- **Infrastructure**: Vault/KMS, Pre-commit hooks, Access logging
- **Rotation**: Maximum 90 days
- **Knowledge**: `knowledge/principles/V-secrets-management/`

### VI. Audit Logging

All sensitive access must be immutably traced.

- **Events**: Auth, Data modifications, Config changes, Secret access, Admin actions
- **Retention**: Minimum 1 year (GDPR)
- **Knowledge**: `knowledge/principles/VI-audit-logging/`

### VII. Patch Management

Known vulnerabilities have strict correction SLAs.

| CVSS | Severity | SLA |
|------|----------|-----|
| 9.0-10.0 | Critical | 48h |
| 7.0-8.9 | High | 7 days |
| 4.0-6.9 | Medium | 30 days |
| 0.1-3.9 | Low | Backlog |

- **Knowledge**: `knowledge/principles/VII-patch-management/`

## Directory Structure

```
secure/
├── README.md                 # This file
├── principles.yaml           # Machine-readable principle definitions
├── prompts/                  # LLM prompts for each command
│   ├── osk-secure-specify.md
│   ├── osk-secure-clarify.md
│   ├── osk-secure-plan.md
│   ├── osk-secure-tasks.md
│   └── osk-secure-implement.md
├── knowledge/
│   ├── principles/           # Guides organized by principle
│   │   ├── I-threat-modeling/
│   │   ├── II-risk-analysis/
│   │   ├── III-security-design/
│   │   ├── IV-security-testing/
│   │   ├── V-secrets-management/
│   │   ├── VI-audit-logging/
│   │   └── VII-patch-management/
│   ├── libraries/            # Reusable reference data
│   │   ├── threats/          # STRIDE, API, auth, data threats
│   │   └── controls/         # Secure defaults
│   └── mappings/             # Control mappings (ASVS)
└── templates/                # Output templates
    ├── data/                 # YAML output schemas
    ├── outputs/              # Markdown outputs
    └── reports/              # Terminal reports
```

## Principle Weights

Principles are weighted per feature based on characteristics:

| Feature Indicator | Principles Impacted |
|-------------------|---------------------|
| Handles authentication | I: HIGH, III: CRITICAL |
| Processes PII | II: HIGH, VI: CRITICAL |
| Handles payments | V: CRITICAL, III: CRITICAL |
| External integrations | VII: HIGH, IV: HIGH |
| Admin functions | III: HIGH, VI: HIGH |
| File uploads | I: HIGH, IV: HIGH |
| API endpoints | I: HIGH, IV: HIGH |

Default weights are defined in `principles.yaml` and can be overridden in `.osk/config.toml`.

## Prerequisites

The Secure phase requires:

1. **System Model** (from Discover phase):
   - `.osk/system-model/index.yaml`
   - `.osk/system-model/architecture.yaml`
   - `.osk/system-model/data.yaml`

2. **Threat Libraries** (at least one):
   - `knowledge/libraries/threats/*.yaml`

## Integration with Other Phases

```
┌──────────────┐
│   DISCOVER   │ ──────► System Model (.osk/system-model/)
└──────────────┘                │
                                │ feeds
                                ▼
                        ┌──────────────┐
                        │    SECURE    │ ◄── You are here
                        └──────────────┘
                                │
                                │ references
                                ▼
                        ┌──────────────┐
                        │    COMPLY    │ ──────► Compliance validation
                        └──────────────┘
```

## References

- [OWASP ASVS](https://owasp.org/www-project-application-security-verification-standard/)
- [STRIDE Methodology](https://docs.microsoft.com/en-us/azure/security/develop/threat-modeling-tool-threats)
- [CVSS Calculator](https://www.first.org/cvss/calculator/3.1)
