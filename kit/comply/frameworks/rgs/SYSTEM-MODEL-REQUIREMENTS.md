# RGS System Model Requirements

This document specifies what system model data is required for RGS compliance assessment, with examples and fallback behavior.

## Overview

RGS assessment requires a populated system model at `.osk/system-model/`. The assessment uses data from multiple sections to evaluate compliance across the 26 controls.

## Required vs Optional Sections

| Section | File | Required | Used For |
|---------|------|----------|----------|
| Index | `index.yaml` | **Required** | Quick stats, compliance hints, section references |
| Architecture | `architecture.yaml` | **Required** | Infrastructure, components, data flows |
| Security Controls | `security.yaml` or in `architecture.yaml` | **Required** | Authentication, encryption, logging |
| Data | `data.yaml` | Recommended | Data classification, sensitivity, retention |
| Integrations | `integrations.yaml` | Recommended | Third-party services, certifications |
| Tooling | `tooling.yaml` | Recommended | CI/CD, security tools, cloud consoles |
| Actors | `actors.yaml` | Recommended | Users, roles, privileged accounts |
| Business | `business.yaml` | Optional | Business context, processes |
| Trust | `trust.yaml` | Optional | Trust zones, boundaries |

## Minimum Viable System Model

For RGS assessment to run, you need at minimum:

```yaml
# .osk/system-model/index.yaml
metadata:
  schema_version: "4.0.0"
  created_at: "2025-01-18"
  confidence: 0.5

stats:
  components: 1
  integrations: 0

compliance_hints:
  rgs:
    applicable: true
    level: "rgs-star"  # or rgs-star-star, rgs-star-star-star

sections:
  architecture: "architecture.yaml"
```

```yaml
# .osk/system-model/architecture.yaml
architecture:
  style: "monolith"
  components:
    - id: "COMP-APP"
      name: "Application"
      type: "service"
      technology: "Node.js"

  infrastructure:
    provider: "aws"
    regions: ["eu-west-1"]

security_controls:
  encryption:
    in_transit:
      enabled: true
      protocol: "TLS 1.3"
  authentication:
    session:
      storage: "jwt"
  logging:
    enabled: true
```

## Section-by-Control Mapping

### Authentification (AUTH) Controls

| Control | Required Data | Source Section | Fallback |
|---------|--------------|----------------|----------|
| RGS-AUTH.1 | Authentication method, MFA status | `actors.users[].authentication` | Ask user |
| RGS-AUTH.2 | Session management | `security_controls.authentication.session` | Assume non-compliant |
| RGS-AUTH.3 | Password policy | `security_controls.authentication.password_policy` | Ask user |
| RGS-AUTH.4 | Privileged accounts inventory | `actors.users[].privileged` | Ask user |
| RGS-AUTH.5 | Account lifecycle | `actors.users[]`, `team.access_management` | Ask user |

**Example - Authentication Data:**
```yaml
# actors.yaml
actors:
  users:
    - id: "end-user"
      type: "external"
      authentication:
        method: "password"
        provider: "internal"
        mfa_required: false
      privileged: false

    - id: "admin"
      type: "internal"
      authentication:
        method: "sso"
        provider: "okta"
        mfa_required: true
        mfa_methods: ["totp", "webauthn"]
      privileged: true
```

### Intégrité (INT) Controls

| Control | Required Data | Source Section | Fallback |
|---------|--------------|----------------|----------|
| RGS-INT.1 | Data integrity mechanisms | `security_controls.encryption`, `data.inventory[].fields[].hashed` | Ask user |
| RGS-INT.2 | Signature verification | `integrations[].api_type`, code analysis | Ask user |
| RGS-INT.3 | Change detection | `tooling.security_tooling`, `security_controls` | Ask user |
| RGS-INT.4 | Backup integrity | `tooling.infrastructure`, code analysis | Ask user |

### Confidentialité (CONF) Controls

| Control | Required Data | Source Section | Fallback |
|---------|--------------|----------------|----------|
| RGS-CONF.1 | Encryption at rest | `security_controls.encryption.at_rest` | Assume non-compliant |
| RGS-CONF.2 | Encryption in transit | `security_controls.encryption.in_transit` | Check for TLS |
| RGS-CONF.3 | Key management | `security_controls.encryption.at_rest.key_management` | Ask user |
| RGS-CONF.4 | Data classification | `data.inventory[].classification` | Ask user |

**Example - Encryption Data:**
```yaml
# In architecture.yaml or security.yaml
security_controls:
  encryption:
    at_rest:
      enabled: true
      algorithm: "AES-256-GCM"
      key_management: "aws-kms"
      scope: ["COMP-DB", "COMP-CACHE"]
    in_transit:
      enabled: true
      protocol: "TLS 1.3"
      certificate_management: "acm"
```

### Traçabilité (TRAC) Controls

| Control | Required Data | Source Section | Fallback |
|---------|--------------|----------------|----------|
| RGS-TRAC.1 | Logging enabled | `security_controls.logging.enabled` | Check codebase |
| RGS-TRAC.2 | Events logged | `security_controls.logging.events_logged` | Ask user |
| RGS-TRAC.3 | Log retention | `security_controls.logging.retention` | Ask user |
| RGS-TRAC.4 | Log protection | `security_controls.logging.immutable` | Ask user |
| RGS-TRAC.5 | Access to logs | `tooling.observability.logging` | Ask user |

**Example - Logging Data:**
```yaml
security_controls:
  logging:
    enabled: true
    events_logged: ["auth", "data_access", "admin_actions", "errors"]
    pii_scrubbing: true
    retention: "1 year"  # Minimum for RGS
    destination: "cloudwatch"
    immutable: true
```

### Horodatage (HORO) Controls

| Control | Required Data | Source Section | Fallback |
|---------|--------------|----------------|----------|
| RGS-HORO.1 | Timestamp source | `security_controls`, code analysis | Ask user |
| RGS-HORO.2 | Qualified timestamping | `integrations[]` (qualified TSA) | Ask user |

### Signature (SIG) Controls

| Control | Required Data | Source Section | Fallback |
|---------|--------------|----------------|----------|
| RGS-SIG.1 | Signature implementation | Code analysis, `integrations[]` | Ask user |
| RGS-SIG.2 | Qualified signatures | `integrations[]` (qualified provider) | Ask user |

## Tool Certification Requirements

For `RGS**` and `RGS***`, external services must have appropriate certifications:

| RGS Level | Cloud Hosting | Security Tools | Identity Provider |
|-----------|--------------|----------------|-------------------|
| `RGS*` | ISO 27001 recommended | Any | Any |
| `RGS**` | SecNumCloud recommended | SOC2/ISO 27001 | SOC2/ISO 27001 |
| `RGS***` | SecNumCloud **required** | Qualified (ANSSI) | Qualified (ANSSI) |

**Example - Tooling with Certifications:**
```yaml
# tooling.yaml
tooling:
  source_control:
    platform: "github"
    certifications: ["SOC2", "ISO27001"]

  ci_cd:
    platform: "github-actions"
    certifications: ["SOC2"]

  infrastructure:
    cloud_consoles:
      - provider: "ovh"
        certifications: ["SecNumCloud", "ISO27001", "HDS"]
        sso_enabled: true
        mfa_required: true

  observability:
    apm:
      provider: "datadog"
      certifications: ["SOC2", "ISO27001"]
```

## Fallback Behavior

When system model data is missing, the assessment will:

1. **Required fields missing** → Block assessment, prompt user to run `/osk-discover`
2. **Recommended fields missing** → Continue with warnings, ask user during assessment
3. **Optional fields missing** → Skip related controls or mark as "not_assessed"

### Auto-Detection from Codebase

If system model is incomplete, some data can be auto-detected:

| Data Type | Detection Method | Confidence |
|-----------|-----------------|------------|
| TLS usage | Check for HTTPS URLs, TLS config | High |
| Password hashing | Search for bcrypt/argon2 imports | High |
| JWT usage | Search for jsonwebtoken imports | High |
| Logging | Search for winston/pino/log4j | Medium |
| Encryption | Search for crypto imports | Medium |

### User Prompts

For data that cannot be auto-detected, the assessment will prompt:

```
┌─────────────────────────────────────────────────────────────────┐
│ Data not found in system model                                  │
│                                                                 │
│ Control: RGS-CONF.3 - Key Management                             │
│ Question: How are encryption keys managed?                      │
│                                                                 │
│ 1. AWS KMS                                                      │
│ 2. HashiCorp Vault                                              │
│ 3. Azure Key Vault                                              │
│ 4. Manual management                                            │
│ 5. Not implemented                                              │
└─────────────────────────────────────────────────────────────────┘
```

## Complete Example

See `.osk/system-model/` in the repository for a complete example of a populated system model suitable for RGS assessment.

## Validation

Run `/osk-discover validate` to check if your system model has sufficient data for RGS assessment:

```bash
$ osk discover validate --framework rgs

RGS System Model Validation
============================
✓ index.yaml present
✓ architecture.yaml present
✓ security_controls defined
⚠ data.yaml missing (recommended for CONF controls)
⚠ actors.yaml missing (recommended for AUTH controls)
✓ tooling.yaml present

Coverage: 75% (sufficient for RGS* assessment)
Missing for RGS**: data classification, actor authentication details
```
