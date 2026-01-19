# OpenSecKit V4 - Template Architecture

## Overview

V4 separates **knowledge** (methodology, reference material) from **templates** (generative output).

```
OpenSecKit/
├── knowledge/              # Static reference material (for agents to learn)
│   ├── methodologies/      # How-to guides
│   ├── libraries/          # Reusable threat/control libraries
│   └── examples/           # Concrete examples
│
└── templates/              # Dynamic generation (for producing outputs)
    ├── schemas/            # YAML data contracts
    ├── outputs/            # Markdown document templates
    ├── data/               # YAML data templates
    ├── reports/            # Terminal output templates
    └── agents/             # Tera agent transforms
```

---

## Knowledge Directory (`knowledge/`)

**Purpose**: Reference material that agents read to understand HOW to perform security tasks. Not used for generation.

### Structure

```
knowledge/
├── methodologies/
│   ├── threat-modeling/
│   │   ├── stride-guide.md           # How to do STRIDE analysis
│   │   ├── attack-trees-guide.md     # How to create attack trees
│   │   └── dfd-guide.md              # How to create DFDs
│   ├── risk-analysis/
│   │   ├── scoring-methodology.md    # How to score risks
│   │   └── prioritization-guide.md   # How to prioritize
│   ├── security-requirements/
│   │   ├── asvs-guide.md             # How to use OWASP ASVS
│   │   └── rfc2119-guide.md          # MUST/SHOULD/MAY usage
│   ├── security-testing/
│   │   ├── sast-guide.md             # How to implement SAST
│   │   ├── dast-guide.md             # How to implement DAST
│   │   └── sca-guide.md              # How to implement SCA
│   ├── secrets-management/
│   │   ├── rotation-guide.md         # How to rotate secrets
│   │   └── detection-guide.md        # How to detect secrets in code
│   ├── audit-logging/
│   │   ├── logging-strategy.md       # What to log
│   │   └── siem-integration.md       # How to integrate SIEM
│   └── patch-management/
│       ├── sla-guide.md              # How to set SLAs
│       └── emergency-procedure.md    # Emergency patching
│
├── libraries/
│   ├── threats/
│   │   ├── stride-common.yaml        # Common STRIDE threats
│   │   ├── api-threats.yaml          # API-specific threats
│   │   ├── auth-threats.yaml         # Authentication threats
│   │   └── data-threats.yaml         # Data handling threats
│   ├── controls/
│   │   ├── owasp-asvs-v4.yaml        # OWASP ASVS controls
│   │   ├── cis-controls-v8.yaml      # CIS Controls
│   │   └── nist-csf-controls.yaml    # NIST CSF controls
│   └── patterns/
│       ├── secure-defaults.yaml      # Secure default patterns
│       └── anti-patterns.yaml        # Common security mistakes
│
└── examples/
    ├── ecommerce/
    │   ├── stride-analysis.md        # Complete STRIDE example
    │   ├── risk-register.yaml        # Example risk register
    │   └── requirements.md           # Example requirements
    ├── saas/
    │   └── ...
    └── api/
        └── ...
```

### Usage in Prompts

Prompts reference knowledge files for methodology:

```markdown
# prompts/osk-secure-specify.md

## Phase 2: Threat Modeling

Follow the methodology in `knowledge/methodologies/threat-modeling/stride-guide.md`.

Use the threat library from `knowledge/libraries/threats/stride-common.yaml` as starting point.
```

---

## Templates Directory (`templates/`)

**Purpose**: Generative templates that produce actual output files. Used by agents to create structured documents.

### Structure

```
templates/
├── schemas/                # YAML schemas (data contracts)
│   ├── system-model/       # System model schemas
│   │   ├── index.yaml
│   │   ├── business.yaml
│   │   ├── architecture.yaml
│   │   ├── data.yaml
│   │   ├── actors.yaml
│   │   ├── boundaries.yaml
│   │   ├── integrations.yaml
│   │   ├── controls.yaml
│   │   └── gaps.yaml
│   ├── compliance/
│   │   └── assessment.yaml
│   ├── specs/
│   │   ├── security-spec.yaml
│   │   ├── risk-entry.yaml
│   │   ├── requirement-entry.yaml
│   │   ├── task-entry.yaml
│   │   └── ...
│   └── risk-register.yaml
│
├── outputs/                # Markdown templates (human-readable docs)
│   ├── discover/
│   │   └── system-model-summary.md
│   ├── comply/
│   │   ├── assessment-summary.md
│   │   └── gap-report.md
│   ├── secure/
│   │   ├── security-spec.md
│   │   ├── threats.md
│   │   ├── requirements.md
│   │   ├── testing.md
│   │   ├── hardening.md
│   │   ├── plan.md
│   │   └── tasks.md
│   └── dashboard.md
│
├── data/                   # YAML data templates (structured outputs)
│   ├── discover/
│   │   ├── index.yaml
│   │   ├── business.yaml
│   │   ├── architecture.yaml
│   │   ├── data.yaml
│   │   ├── actors.yaml
│   │   ├── boundaries.yaml
│   │   ├── integrations.yaml
│   │   ├── controls.yaml
│   │   └── gaps.yaml
│   ├── comply/
│   │   └── assessment.yaml
│   ├── secure/
│   │   ├── security-spec.yaml
│   │   ├── tasks.yaml
│   │   └── risks.yaml
│   └── risk-register.yaml
│
├── reports/                # Terminal output templates
│   ├── discover-init.txt
│   ├── discover-scan.txt
│   ├── comply-summary.txt
│   ├── secure-specify.txt
│   └── ...
│
└── agents/                 # Tera templates for agent adaptation
    ├── claude-code.tera
    ├── copilot.tera
    ├── cursor.tera
    ├── gemini.tera
    └── AGENTS.md.tera
```

---

## Output Format: YAML + Markdown

Each generated artifact has two representations:

### 1. YAML (Structured Data)

Machine-readable, used for:
- Cross-referencing between documents
- Automated validation
- Compliance scoring
- Risk aggregation

```yaml
# .osk/specs/001-auth/security-spec.yaml

metadata:
  feature_id: "001"
  feature_name: "auth"
  created_at: "2026-01-14"

security_objectives:
  - id: "OBJ-001"
    objective: "Prevent credential stuffing"
    priority: "must"

threat_analysis:
  entry_points:
    - id: "EP-001"
      name: "Login endpoint"
      risk_level: "critical"
```

### 2. Markdown (Human Documentation)

Human-readable, used for:
- Review and approval
- Audit documentation
- Team communication
- Git diffs

```markdown
# .osk/specs/001-auth/security-spec.md

# Security Specification: Authentication Feature

## Security Objectives

| ID | Objective | Priority |
|----|-----------|----------|
| OBJ-001 | Prevent credential stuffing | MUST |

## Threat Analysis

| Entry Point | Risk Level | Primary Threats |
|-------------|------------|-----------------|
| Login endpoint | **CRITICAL** | Credential stuffing, brute force |
```

### Generation Flow

```
Agent executes prompt
        │
        ▼
┌───────────────────┐
│   Generate YAML   │  ← Primary output (structured)
│   (data.yaml)     │
└─────────┬─────────┘
          │
          ▼
┌───────────────────┐
│ Generate Markdown │  ← Secondary output (human-readable)
│   (doc.md)        │     Generated FROM YAML
└───────────────────┘
```

### Template Structure

**YAML Template** (`templates/data/secure/security-spec.yaml`):
```yaml
# Template for security specification
# Variables: {{feature_id}}, {{feature_name}}, {{objectives}}, {{threats}}

metadata:
  schema: "security-spec"
  version: "4.0.0"
  feature_id: "{{feature_id}}"
  feature_name: "{{feature_name}}"
  created_at: "{{timestamp}}"
  created_by: "/osk-secure specify"

security_objectives:
{{#each objectives}}
  - id: "{{id}}"
    objective: "{{objective}}"
    rationale: "{{rationale}}"
    priority: "{{priority}}"
{{/each}}

threat_analysis:
  entry_points:
{{#each entry_points}}
    - id: "{{id}}"
      name: "{{name}}"
      risk_level: "{{risk_level}}"
{{/each}}
```

**Markdown Template** (`templates/outputs/secure/security-spec.md`):
```markdown
# Security Specification: {{feature_name}}

> Generated by `/osk-secure specify` on {{timestamp}}

## Security Objectives

| ID | Objective | Rationale | Priority |
|----|-----------|-----------|----------|
{{#each objectives}}
| {{id}} | {{objective}} | {{rationale}} | {{priority}} |
{{/each}}

## Threat Analysis

| Entry Point | Risk Level | Primary Threats |
|-------------|------------|-----------------|
{{#each entry_points}}
| {{name}} | **{{risk_level}}** | {{threats}} |
{{/each}}

---
*Source: `.osk/specs/{{feature_id}}-{{feature_name}}/security-spec.yaml`*
```

---

## Schema vs Template vs Output

| Type | Location | Format | Purpose |
|------|----------|--------|---------|
| **Schema** | `templates/schemas/` | YAML | Defines structure, validates data |
| **Data Template** | `templates/data/` | YAML + Handlebars | Template for generating YAML output |
| **Doc Template** | `templates/outputs/` | Markdown + Handlebars | Template for generating Markdown |
| **Output (YAML)** | `.osk/*/` | YAML | Generated structured data |
| **Output (MD)** | `.osk/*/` or `docs/` | Markdown | Generated human documentation |

---

## Migration from V3

### Files to Move

```
# FROM (V3)                              # TO (V4)
templates/01-threat-modeling/     →      knowledge/methodologies/threat-modeling/
templates/02-risk-analysis/       →      knowledge/methodologies/risk-analysis/
templates/03-security-req/        →      knowledge/methodologies/security-requirements/
templates/04-security-testing/    →      knowledge/methodologies/security-testing/
templates/05-secrets-mgmt/        →      knowledge/methodologies/secrets-management/
templates/06-audit-logging/       →      knowledge/methodologies/audit-logging/
templates/07-patch-mgmt/          →      knowledge/methodologies/patch-management/

templates/schemas/                →      templates/schemas/ (keep)
templates/outputs/                →      templates/outputs/ (restructure)
templates/reports/                →      templates/reports/ (keep)
templates/agents/                 →      templates/agents/ (keep)
```

### New Directories

```
knowledge/libraries/threats/      # Extract from methodology docs
knowledge/libraries/controls/     # New: control libraries
knowledge/examples/               # Move _example-* files here

templates/data/                   # New: YAML output templates
```

---

## Benefits

1. **Clear Separation**: Knowledge (learn) vs Templates (generate)
2. **Dual Output**: YAML for machines, Markdown for humans
3. **Validation**: Schemas validate YAML outputs
4. **Reusability**: Libraries can be shared across features
5. **Maintainability**: Methodology updates don't affect generation
6. **Auditability**: YAML provides traceable data trail
