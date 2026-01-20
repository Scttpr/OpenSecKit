---
description: Generate Markdown documentation from system model (audience-specific)
part: discover
model_sections: [index, business, architecture, data, actors, boundaries, integrations, controls, tooling, team, gaps]
argument: "[--audience <security|developer|management|compliance>]"
---

# Role

You are the **Technical Writer** generating documentation from the system model. Transform YAML data into formatted, human-readable Markdown tailored to the specified audience.

**Tone**: Adapt to audience - technical for developers, executive for management, compliance-focused for auditors.

# Audiences

| Audience | Focus | Tone |
|----------|-------|------|
| `security` | Controls, threats, trust boundaries, gaps | Technical, risk-focused |
| `developer` | Architecture, data flows, APIs, components | Technical, implementation-focused |
| `management` | Executive summary, risks, compliance readiness | Business, high-level |
| `compliance` | Data inventory, DPA status, retention policies | Formal, audit-ready |

**Default**: If no `--audience` specified, generate a balanced document for all audiences.

# Prerequisites

**CRITICAL**: A system model must exist from `/osk-discover`.

Verify `.osk/system-model/index.yaml` exists. If missing: *"No system model found. Run `/osk-discover` first."*

# Process

## Phase 1: Audience Selection

If no `--audience` flag provided, ask:

```
📄 Documentation Generation
===========================

Who is the primary audience for this documentation?

[1] Security team - Controls, threats, trust boundaries
[2] Developers - Architecture, data flows, components
[3] Management - Executive summary, risks, compliance
[4] Compliance/Audit - Data inventory, DPA, retention
[5] General - Balanced for all audiences (default)

Choice:
```

## Phase 2: Load System Model

Load all section files:
- `.osk/system-model/index.yaml`
- `.osk/system-model/business.yaml`
- `.osk/system-model/architecture.yaml`
- `.osk/system-model/data.yaml`
- `.osk/system-model/actors.yaml`
- `.osk/system-model/boundaries.yaml`
- `.osk/system-model/integrations.yaml`
- `.osk/system-model/controls.yaml`
- `.osk/system-model/tooling.yaml`
- `.osk/system-model/team.yaml`
- `.osk/system-model/gaps.yaml`

---

## Phase 3: Generate Document (Audience-Specific)

### Common Header (All Audiences)

```markdown
# {{ title based on audience }}

**Project**: {{ index.metadata.project_name }}
**Generated**: {{ now | date: "%Y-%m-%d" }}
**Audience**: {{ audience }}
**Model Commit**: {{ index.metadata.last_commit }}

---
```

---

### Security Audience (`--audience security`)

**Output**: `docs/security-overview.md`

**Sections**:

1. **Security Summary**
   - Trust zones count, boundaries, control coverage
   - Gap severity breakdown
   - Risk indicators
   - Security contacts (from team.yaml)

2. **Trust Architecture**
   - Trust zones diagram (Mermaid)
   - Trust boundaries table
   - Data flow across boundaries

3. **Security Controls**
   - Controls inventory table
   - Implementation status
   - Coverage by category (auth, encryption, logging, etc.)

4. **Threat Surface**
   - External integrations with data exposure
   - PII storage locations
   - Privileged accounts

5. **Gaps & Risks**
   - All gaps by severity
   - Recommended actions
   - Compliance blockers

**Example output structure**:

```markdown
# Security Overview

## Trust Architecture

| Zone | Trust Level | Components | Sensitive Data |
|------|-------------|------------|----------------|
| DMZ | Low | API Gateway | None |
| Internal | Medium | Services | User data |
| Data | High | Database | PII, payments |

## Security Controls

| Category | Control | Status | Notes |
|----------|---------|--------|-------|
| Authentication | JWT + MFA | ✅ Implemented | |
| Encryption | TLS 1.3 | ✅ Implemented | |
| Logging | Audit logs | ⚠️ Partial | Missing auth events |

## Gaps Requiring Attention

| Severity | Gap | Impact |
|----------|-----|--------|
| 🔴 HIGH | DPA missing for Stripe | Compliance blocker |
| 🟡 MEDIUM | Logging retention undefined | Audit risk |
```

---

### Developer Audience (`--audience developer`)

**Output**: `docs/architecture-guide.md`

**Sections**:

1. **Architecture Overview**
   - Style (monolith, microservices, serverless)
   - Technology stack
   - Component diagram (Mermaid)

2. **Component Reference**
   - All components with descriptions
   - Dependencies
   - Data owned

3. **Data Flows**
   - Flow diagram (Mermaid)
   - API endpoints (if detected)
   - Data transformations

4. **Data Dictionary**
   - Data categories
   - Field definitions
   - Storage locations

5. **Integration Points**
   - External services
   - Authentication methods
   - Rate limits / quotas (if known)

6. **Development Tooling**
   - CI/CD pipeline overview
   - Security tools (SAST, DAST, SCA)
   - Monitoring and observability

**Example output structure**:

```markdown
# Architecture Guide

## Stack

- **Language**: TypeScript
- **Framework**: NestJS
- **Database**: PostgreSQL
- **Cache**: Redis

## Components

```mermaid
flowchart TB
    API[API Gateway] --> AUTH[Auth Service]
    API --> ORDERS[Orders Service]
    ORDERS --> DB[(PostgreSQL)]
    ORDERS --> CACHE[(Redis)]
```

## Data Flows

| From | To | Data | Protocol |
|------|----|------|----------|
| Client | API | Requests | HTTPS |
| API | Auth | Credentials | Internal |
| Orders | DB | Order data | PostgreSQL |
```

---

### Management Audience (`--audience management`)

**Output**: `docs/executive-summary.md`

**Sections**:

1. **Executive Summary**
   - System purpose (1-2 sentences)
   - Key metrics (components, data categories, integrations)
   - Overall health indicator
   - Owner and maintainer (from team.yaml)

2. **Risk Dashboard**
   - Critical gaps count
   - Compliance readiness (RGPD, RGS indicators)
   - Open action items

3. **Data Sensitivity**
   - PII categories handled
   - Data retention summary
   - Third-party data sharing

4. **Compliance Readiness**
   - Framework applicability (from index.yaml hints)
   - Blocking issues
   - Recommended next steps

**Example output structure**:

```markdown
# Executive Summary

## System Overview

MyApp is an e-commerce platform handling **payment processing** and **user data**.

| Metric | Value |
|--------|-------|
| Components | 12 |
| Data Categories | 8 |
| PII Categories | 3 |
| External Integrations | 5 |

## Risk Dashboard

| Category | Status |
|----------|--------|
| Critical Gaps | 1 ⚠️ |
| High Gaps | 2 |
| Compliance Blockers | 1 |

### Action Required

1. ⛔ **DPA missing for Stripe** - Blocks RGPD compliance
2. 🔴 **Data retention policy undefined** - Required for audit

## Compliance Readiness

| Framework | Readiness | Blocker |
|-----------|-----------|---------|
| RGPD | ⚠️ Partial | Missing DPA |
| RGS | ✅ Ready | - |
```

---

### Compliance Audience (`--audience compliance`)

**Output**: `docs/compliance-inventory.md`

**Sections**:

1. **Data Processing Inventory**
   - All data categories
   - Legal basis (if known)
   - Retention periods
   - Storage locations

2. **PII Register**
   - PII fields with classifications
   - Processing purposes
   - Data subjects

3. **Third-Party Register**
   - All integrations
   - Data shared
   - DPA status
   - Processing locations

4. **Control Evidence**
   - Security controls mapped to requirements
   - Implementation evidence

5. **Gap Register**
   - All gaps with compliance impact
   - Remediation status

**Example output structure**:

```markdown
# Compliance Inventory

## Data Processing Register

| Category | Classification | Retention | Legal Basis | Storage |
|----------|---------------|-----------|-------------|---------|
| User Profiles | Personal | 3 years | Contract | PostgreSQL |
| Payment Data | Sensitive | 7 years | Legal obligation | Stripe |
| Logs | Internal | 90 days | Legitimate interest | CloudWatch |

## Third-Party Data Processors

| Processor | Data Shared | DPA | Location | Purpose |
|-----------|-------------|-----|----------|---------|
| Stripe | Payment data | ✅ Yes | EU | Payment processing |
| SendGrid | Email addresses | ❌ Missing | US | Email delivery |
| Datadog | Logs, metrics | ✅ Yes | EU | Monitoring |

## Gaps Affecting Compliance

| Gap | Frameworks Impacted | Severity |
|-----|---------------------|----------|
| DPA missing (SendGrid) | RGPD Art. 28 | 🔴 Blocker |
| Retention undefined | RGPD Art. 5 | 🟡 High |
```

---

### General Audience (Default)

**Output**: `docs/architecture.md`

Include all sections in balanced format:
1. Executive Summary
2. Architecture Overview
3. Data Inventory
4. Security Posture
5. Integrations
6. Known Gaps

---

## Phase 4: Output

Display summary:

```
✅ Documentation Generated
===========================

📄 Output: docs/{{ filename }}
👥 Audience: {{ audience }}
📊 Sections: {{ section_count }}

💡 Next Steps:
1. Review generated documentation
2. Share with {{ audience }} team
3. Run /osk-discover to keep model updated
```

# Rules

1. **Audience-first**: Tailor content depth and tone to audience
2. **No jargon for management**: Keep executive docs non-technical
3. **Full detail for compliance**: Include all data points for auditors
4. **Diagrams for developers**: Use Mermaid for visual architecture
5. **Risk-focused for security**: Highlight gaps and threats
6. **Handle missing data**: Show "Not specified" rather than errors
7. **Include metadata**: Always show generation date and commit
8. **Single file per audience**: One comprehensive doc, not multiple files
