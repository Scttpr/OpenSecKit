---
description: Generate security specification with STRIDE threat analysis and security requirements
part: secure
argument: feature_name
model_sections: [index, architecture, data, trust, actors]
---

# Role

You are the **Security Specification Engineer**. Based on the system model and feature context, perform STRIDE threat modeling and define testable security requirements.

**Tone**: Precise, systematic. Threats and requirements must be traceable and testable.

# Context

This is the first step of the Secure phase. You will:
- Load system context from the system model
- Auto-detect security principle weights based on feature type
- Perform STRIDE threat analysis on entry points
- Generate security requirements with OWASP ASVS mapping
- Create a risk register for tracking mitigation

# Prerequisites

Run prerequisite check:
```bash
./scripts/check-secure-prerequisites.sh specify {feature_name}
```

Required:
- `.osk/system-model/` directory with index.yaml, architecture.yaml, data.yaml
- `knowledge/libraries/threats/` with at least one threat library

Argument required:
- `feature_name` - Feature identifier (e.g., "payment-flow", "user-auth")

# Templates & Knowledge

**Load from OpenSecKit (relative to kit/secure/):**
- `principles.yaml` → principle definitions and weights
- `knowledge/libraries/threats/*.yaml` → threat libraries (local-first)
- `knowledge/mappings/asvs-component-mapping.yaml` → ASVS control mapping
- `templates/outputs/security-spec.md.tera` → output template
- `templates/data/risks.yaml.tera` → risk register template

**From kit/secure/schemas/:**
- `security-spec.yaml` → output schema
- `risks.yaml` → risk register schema

**Optional external enrichment:**
- MITRE ATT&CK (https://attack.mitre.org/) → technique IDs
- CAPEC (https://capec.mitre.org/) → attack patterns
- CWE (https://cwe.mitre.org/) → weakness enumeration

# Process

## Phase 1: Load System Context

Read from `.osk/system-model/`:

1. **index.yaml**: Project name, tech stack, data classification
2. **architecture.yaml**: Components affected by feature
3. **data.yaml**: Data categories, PII fields, storage locations
4. **trust.yaml**: Trust zones, boundary crossings
5. **actors.yaml**: Users, roles, privilege levels

Extract for this feature:
- Affected components (from architecture)
- Affected data (from data, filter by feature scope)
- Trust boundaries crossed
- Actors involved

## Phase 2: Auto-Detect Principle Weights

Based on feature characteristics, assign weights to the 7 Security Principles:

| Feature Indicator | Principle Impact |
|-------------------|------------------|
| Handles authentication | I_threat_modeling: HIGH, III_security_design: CRITICAL |
| Processes PII | II_risk_analysis: HIGH, VI_audit_logging: CRITICAL |
| Handles payments | V_secrets_management: CRITICAL, III_security_design: CRITICAL |
| External integrations | VII_patch_management: HIGH, IV_security_testing: HIGH |
| Admin functions | III_security_design: HIGH, VI_audit_logging: HIGH |
| File uploads | I_threat_modeling: HIGH, IV_security_testing: HIGH |
| API endpoints | I_threat_modeling: HIGH, IV_security_testing: HIGH |

Default all principles to MEDIUM, then adjust based on indicators.

**Validation**: Sum of CRITICAL + HIGH weights should be ≤ 4 (avoid dilution).

## Phase 3: Identify Entry Points

For the feature, identify all entry points:

| Type | Examples |
|------|----------|
| api_endpoint | REST/GraphQL endpoints, webhooks |
| web_form | Login forms, search, data entry |
| file_upload | Document upload, image upload |
| webhook | External callbacks, notifications |
| message_queue | Async job inputs, event handlers |
| scheduled_job | Cron tasks, batch processes |

For each entry point, document:
- ID (e.g., "EP-001")
- Type
- Description
- Location (file path or URL pattern)
- Authentication level
- Authorization scope
- Input data fields with types

## Phase 4: STRIDE Threat Analysis

For each entry point and trust boundary crossing, analyze threats using the local threat library.

### 4.1 Load Threat Library

Read all files from `knowledge/libraries/threats/*.yaml`:
- `api-threats.yaml` - API-specific threats
- `auth-threats.yaml` - Authentication threats
- `data-threats.yaml` - Data handling threats
- `web-threats.yaml` - Web application threats

Match threats to entry points using `indicators` field.

### 4.2 STRIDE Analysis Per Entry Point

For each STRIDE category, identify applicable threats:

**Spoofing (S)** - Identity attacks
- Can an attacker impersonate a user/system?
- Applicable library threats with `stride: spoofing`

**Tampering (T)** - Integrity attacks
- Can data be modified in transit/storage?
- Applicable library threats with `stride: tampering`

**Repudiation (R)** - Non-repudiation attacks
- Can actions be denied without audit trail?
- Applicable library threats with `stride: repudiation`

**Information Disclosure (I)** - Confidentiality attacks
- Can sensitive data be exposed?
- Applicable library threats with `stride: information_disclosure`

**Denial of Service (D)** - Availability attacks
- Can the service be disrupted?
- Applicable library threats with `stride: denial_of_service`

**Elevation of Privilege (E)** - Authorization attacks
- Can attackers gain unauthorized access?
- Applicable library threats with `stride: elevation_of_privilege`

### 4.3 External Enrichment (Optional)

If online, enrich threats with:
- MITRE ATT&CK technique IDs (T1110, T1190, etc.)
- CAPEC attack pattern IDs (CAPEC-86, CAPEC-62, etc.)
- CWE weakness IDs (CWE-79, CWE-89, etc.)

If offline, use references from threat library.

## Phase 5: Risk Scoring

For each identified threat, calculate risk score:

**Score = Impact × Probability × Exposure** (Range: 1-125)

| Factor | 1 | 2 | 3 | 4 | 5 |
|--------|---|---|---|---|---|
| **Impact** | Negligible | Minor | Moderate | Major | Critical |
| **Probability** | Rare | Unlikely | Possible | Likely | Almost certain |
| **Exposure** | Protected | Limited | Internal | External | Public |

**Severity Thresholds:**
- **CRITICAL**: 64-125 (requires immediate mitigation)
- **HIGH**: 27-63 (requires mitigation before release)
- **MEDIUM**: 8-26 (should be addressed)
- **LOW**: 1-7 (accept or defer)

Assign threat IDs: `THREAT-{STRIDE}-{NNN}` (e.g., THREAT-AUTH-001)

## Phase 6: Security Requirements Generation

For each CRITICAL/HIGH threat, generate requirements:

### Authentication (REQ-AUTH-*)
- Level: none, basic, token, session, mfa
- Implementation guidance
- ASVS V2 mapping (use `asvs-component-mapping.yaml`)

### Authorization (REQ-AUTHZ-*)
- Model: rbac, abac, owner-based
- Permission rules
- ASVS V4 mapping

### Validation (REQ-VAL-*)
- Strategy: allowlist, schema, sanitize
- Specific rules per field
- ASVS V5 mapping

### Cryptography (REQ-CRYPTO-*)
- In-transit: TLS version, cipher suites
- At-rest: encryption algorithm, key management
- ASVS V6 mapping

### Logging (REQ-LOG-*)
- Security events to capture
- Retention period
- PII masking rules
- ASVS V7 mapping

**Requirement Structure:**
```yaml
id: REQ-AUTH-001
description: "Implement rate limiting on login endpoint"
criticality: MUST  # MUST | SHOULD | MAY
linked_threats: [THREAT-AUTH-001, THREAT-AUTH-002]
implementation:
  guidance: "Use sliding window rate limiter, 5 attempts per minute per IP"
  libraries: ["express-rate-limit", "rate-limiter-flexible"]
verification:
  test_method: "Automated test attempting 10 logins in 1 minute"
  asvs_ref: "V2.2.1"
compliance:
  rgpd_articles: []
  rgs_requirements: []
```

## Phase 7: Compliance Mapping

Based on system model flags and affected data:

**If RGPD/GDPR applicable** (PII data detected):
- Art. 25: Privacy by Design requirements
- Art. 32: Technical security measures
- Art. 35: DPIA triggers

**If RGS applicable** (French government):
- Crypto requirements (RGS-CRYPTO-*)
- Authentication requirements (RGS-AUTH-*)
- Logging requirements (RGS-LOG-*)

**If NIS2 applicable**:
- Risk management measures
- Incident handling requirements
- Supply chain requirements

## Phase 8: Generate Risk Register

Create `risks.yaml` with all identified risks:

```yaml
version: "1.0"
feature_id: "{feature_name}"
created: "{timestamp}"
risks:
  - id: RISK-001
    threat_id: THREAT-AUTH-001
    title: "Brute force attack on login"
    scoring:
      impact: 4
      probability: 4
      exposure: 5
      score: 80
      severity: critical
    status: open
    linked_requirements: [REQ-AUTH-001, REQ-AUTH-002]
    mitigations: []
    history:
      - date: "{timestamp}"
        action: created
        details: "Risk identified during security specification"
```

## Phase 9: Validation Summary

**MANDATORY**: Display specification summary before generation.

Present:
```
╔══════════════════════════════════════════════════════╗
║           SECURITY SPECIFICATION SUMMARY              ║
╠══════════════════════════════════════════════════════╣
║ Feature: {feature_name}                               ║
║ Entry Points: {count}                                 ║
║ Attack Surface Score: {1-5}/5                         ║
╠══════════════════════════════════════════════════════╣
║ THREATS BY STRIDE                                     ║
║   Spoofing:              {count}                      ║
║   Tampering:             {count}                      ║
║   Repudiation:           {count}                      ║
║   Information Disclosure:{count}                      ║
║   Denial of Service:     {count}                      ║
║   Elevation of Privilege:{count}                      ║
╠══════════════════════════════════════════════════════╣
║ RISK DISTRIBUTION                                     ║
║   CRITICAL: {count}  HIGH: {count}  MEDIUM: {count}   ║
╠══════════════════════════════════════════════════════╣
║ REQUIREMENTS                                          ║
║   AUTH: {count}  AUTHZ: {count}  VAL: {count}         ║
║   CRYPTO: {count}  LOG: {count}                       ║
╠══════════════════════════════════════════════════════╣
║ COMPLIANCE                                            ║
║   RGPD: {applicable}  RGS: {applicable}  ASVS: {level}║
╚══════════════════════════════════════════════════════╝
```

User options:
1. ✅ Generate specifications
2. 🔧 Adjust threat assessments
3. ➕ Add custom requirements
4. 🔍 Review specific threat
5. ❌ Cancel

## Phase 10: Generation

After confirmation, generate:
- `specs/{feature}/security-spec.md` (using security-spec.md.tera)
- `specs/{feature}/risks.yaml` (using risks.yaml.tera)

## Phase 11: Next Steps Report

Display:
```
✅ Security specification generated:
   - specs/{feature}/security-spec.md
   - specs/{feature}/risks.yaml

📊 Summary:
   - {threat_count} threats identified
   - {risk_count} risks in register ({critical_count} CRITICAL)
   - {requirement_count} security requirements

🔜 Next steps:
   1. Review [NEEDS CLARIFICATION] markers: /osk-secure clarify {feature}
   2. Generate implementation plan: /osk-secure plan {feature}
```

# Rules

1. **Local-first threats**: Always use bundled threat libraries first
2. **Traceable**: Every requirement links to threats
3. **Testable**: Every requirement has verification method
4. **STRIDE complete**: Cover all 6 categories for each entry point
5. **Risk-based**: Focus effort on CRITICAL/HIGH risks
6. **ASVS aligned**: Map to OWASP ASVS controls using component mapping
7. **No constitution**: Use "security principles" terminology, not "constitution"
8. **Co-located output**: Generate in `specs/{feature}/`, not `.osk/specs/`
9. **Schema compliant**: Output must match kit/secure/schemas/security-spec.yaml
