# Research: Conformity Phase v4

**Date**: 2026-01-15
**Feature**: 002-conform-phase

## Research Questions

1. RGS framework structure for framework.yaml
2. Best practices for interactive compliance assessment workflows
3. Evidence auto-detection patterns from system model
4. Output template requirements for GDPR and RGS

---

## 1. RGS Framework Structure

### Decision: Create framework.yaml following existing pattern

**Rationale**: The codebase has extensive RGS infrastructure (skeleton.yaml, 10 templates, prompts) but lacks a unified `framework.yaml` file like RGPD, ISO 27001, NIS2, and SOC2 have.

**Alternatives Considered**:
- Use skeleton.yaml directly: Rejected - it's V3 legacy format, not compatible with compliance assessment pipeline
- Create from scratch: Rejected - leverage existing detailed templates

### RGS Framework Structure

**Six Core Domains** (Annexes B2-B7):

| Annexe | Domain | Controls |
|--------|--------|----------|
| B2 | Authentication & Identification | RGS-B2.1 to RGS-B2.5 |
| B3 | Data Integrity | RGS-B3.1 to RGS-B3.4 |
| B4 | Confidentiality | RGS-B4.1 to RGS-B4.4 |
| B5 | Traceability | RGS-B5.1 to RGS-B5.5 |
| B6 | Timestamping | RGS-B6.1 to RGS-B6.2 |
| B7 | Electronic Signature | RGS-B7.1 to RGS-B7.2 |

**Three Security Levels**:

| Level | Description | Min Score | Key Requirements |
|-------|-------------|-----------|------------------|
| RGS* (Standard) | Basic government services | 70% | MFA, AES-128, basic logging |
| RGS** (Renforcé) | Sensitive data, critical services | 85% | Certificates, AES-256, SIEM, HSM |
| RGS*** (Élevé) | Defense classified, OIV | 95% | Qualified signatures, TSA, WORM logs |

**Control Applicability by Level**:
- Some controls apply to all levels (e.g., RGS-B4.2 TLS encryption)
- Some controls only apply at higher levels (e.g., RGS-B3.2 qualified signatures at RGS**/RGS***)
- `rgs_levels` field in framework.yaml specifies which levels each control applies to

### DICP Security Model (vs. CIA)

RGS uses DICP instead of standard CIA triad:
- **D**isponibilité (Availability): 0-4 scale
- **I**ntégrité (Integrity): 0-4 scale
- **C**onfidentialité (Confidentiality): 0-4 scale
- **P**reuve (Proof/Traceability): 0-4 scale - unique to RGS

### Homologation Process

Four-phase certification:
1. **Étude**: Threat modeling (EBIOS RM), requirements
2. **Dossier**: Complete security documentation package
3. **Décision**: Authority approval, signed certification
4. **MCS**: Continuous maintenance (quarterly reviews, annual pentest)

### Existing Assets to Leverage

- `domaines/gouvernement-rgs/skeleton.yaml` - Data model reference
- `domaines/gouvernement-rgs/templates/` - 10 detailed templates (340 KB)
- `prompts/osk-comply-rgs.md` - Assessment workflow structure
- `docs/domains/rgs.md` - Documentation

---

## 2. Interactive Assessment Workflow

### Decision: Chapter/Domain-grouped, control-by-control flow with scope confirmation

**Rationale**: Users need context (which regulation chapter they're in), ability to confirm/reject auto-detected evidence, and clear progress indication.

**Alternatives Considered**:
- Flat list of all controls: Rejected - overwhelming, no context
- Fully automated (no interaction): Rejected - compliance requires human judgment
- Single-question-at-a-time: Rejected - too slow, no batching

### Assessment Workflow Pattern

```
Phase 1: Prerequisites Check
├── Verify .osk/system-model/ exists
├── Verify required sections (data.yaml for GDPR, security.yaml for RGS)
└── Error with actionable message if missing

Phase 2: Scope Definition
├── Load ALL system model sections (full context)
├── Present summary: data categories, integrations, tooling, infrastructure
├── Explain why each component is in scope
├── User confirms/adjusts scope
└── Record justification for any exclusions

Phase 3: Interactive Assessment
├── Group controls by chapter/domain
├── For each control:
│   ├── Display: article/annexe, official text, plain-language explanation
│   ├── Show auto-detected evidence (if any)
│   ├── User: confirm, reject, add evidence, or skip
│   └── Record: status, evidence sources, user responses
└── Progress: "Control 15/47 - Chapter III: Rights"

Phase 4: Gap Analysis & Report
├── Calculate overall score
├── Group gaps by severity and category (codebase/tooling/infrastructure)
├── Generate prioritized action items
├── User confirms
└── Generate output files
```

### User Interaction Points

| Point | Required? | Options |
|-------|-----------|---------|
| Scope confirmation | Yes | Confirm / Add / Exclude (with justification) |
| Auto-detected evidence | Yes | Confirm / Reject / Add more |
| Control assessment | No (can skip) | Compliant / Partial / Gap / N/A (with justification) / Skip |
| Final summary | Yes | Generate / Review specific control / Cancel |

---

## 3. Evidence Auto-Detection Patterns

### Decision: Map control evidence_types to system model sections

**Rationale**: Framework.yaml controls define `evidence_types` (e.g., `["encryption_evidence", "access_controls"]`). These map to specific paths in system model YAML files.

**Alternatives Considered**:
- Free-text search: Rejected - too unpredictable, false positives
- No auto-detection: Rejected - misses 60%+ of available evidence

### Evidence Mapping Table

| Evidence Type | System Model Source | Detection Logic |
|---------------|---------------------|-----------------|
| `encryption_evidence` | security.yaml → encryption | Check if TLS/AES configs exist |
| `access_controls` | security.yaml → access | Check RBAC/ACL presence |
| `processing_records` | data.yaml → processing_activities | List exists with entries |
| `retention_policy` | data.yaml → retention | Retention periods defined |
| `dpa_contracts` | integrations.yaml → all | Check `dpa_signed: true` |
| `privacy_notices` | business.yaml → privacy | Privacy policy referenced |
| `auth_procedures` | security.yaml → authentication | Auth method documented |
| `log_samples` | tooling.yaml → logging | SIEM/logging tool configured |
| `tls_config` | architecture.yaml → tls | TLS version/ciphers specified |

### Auto-Detection Confidence Levels

| Confidence | Description | Action |
|------------|-------------|--------|
| High | Direct match in system model (e.g., `tls_version: "1.3"`) | Present as evidence, user confirms |
| Medium | Indirect indicator (e.g., `logging_tool: "Datadog"`) | Present with explanation, user validates |
| Low | Absence may indicate gap | Flag as potential gap, user clarifies |

---

## 4. Output Template Requirements

### Decision: Framework-specific templates in domaines/<framework>/templates/

**Rationale**: GDPR and RGS have different output format requirements. GDPR needs record-of-processing format; RGS needs dossier d'homologation structure.

**Alternatives Considered**:
- Single generic template: Rejected - doesn't meet regulatory requirements
- Templates in central location: Rejected - harder to extend with new frameworks

### GDPR Output Templates

| Template | Purpose | Based On |
|----------|---------|----------|
| `assessment-summary.md.tera` | Executive summary with score | GDPR Art. 30 format |
| `sub-processor-register.md.tera` | List of all sub-processors | GDPR Art. 28 requirement |
| `gap-report.md.tera` | Detailed gaps with remediation | Internal format |
| `dpia-requirement.md.tera` | DPIA necessity assessment | GDPR Art. 35 criteria |

### RGS Output Templates

| Template | Purpose | Based On |
|----------|---------|----------|
| `assessment-summary.md.tera` | Domain scores, overall status | RGS homologation format |
| `homologation-checklist.md.tera` | Pre-homologation requirements | ANSSI guidelines |
| `system-perimeter.md.tera` | Full system boundary definition | RGS dossier Section 5 |
| `dicp-assessment.md.tera` | DICP scores with justification | RGS dossier Section 3 |

### Common Output (all frameworks)

| Template | Purpose | Location |
|----------|---------|----------|
| `assessment.yaml.tera` | Machine-readable assessment | templates/data/comply/ |
| `assessment-report.md.tera` | Human-readable full report | templates/outputs/comply/ |
| `compliance-summary.tera` | Terminal output | templates/reports/ |

---

## 5. Framework Schema Validation

### Decision: Create JSON Schema for framework.yaml validation

**Rationale**: New frameworks should be validated before use. Contributors need clear feedback on schema errors.

### Framework.yaml Required Fields

```yaml
metadata:
  framework_id: string (required, unique)
  name: string (required)
  version: string (required)
  authority: string (required)

categories:
  - id: string (required)
    name: string (required)

controls:
  - id: string (required, must start with framework_id)
    name: string (required)
    description: string (required)
    category: string (required, must match categories.id)
    criticality: enum [must, should, may] (required)
    evidence_types: array of strings (required)
    applies_when: string (optional, conditional applicability)

scoring:
  method: enum [weighted, binary] (required)
  thresholds:
    compliant: number (required)
    partial: number (required)
```

---

## Summary

| Topic | Decision | Key Insight |
|-------|----------|-------------|
| RGS Framework | Create framework.yaml with 22 controls in 6 domains | DICP model, 3 security levels |
| Assessment Workflow | Chapter-grouped, interactive with scope confirmation | Full system context in scope |
| Evidence Detection | Map evidence_types to system model paths | High/Medium/Low confidence levels |
| Output Templates | Framework-specific in domaines/<framework>/templates/ | GDPR vs RGS format requirements differ |
| Schema Validation | JSON Schema for framework.yaml | Required for extensibility |
