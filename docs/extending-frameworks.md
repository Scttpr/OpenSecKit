# Extending OpenSecKit with New Compliance Frameworks

This guide explains how to add new compliance frameworks to OpenSecKit without modifying any code.

## Overview

OpenSecKit uses a declarative framework definition approach. To add a new framework (e.g., SOC 2, ISO 27001, NIS2), you only need to:

1. Create a `framework.yaml` file following the schema
2. Optionally create framework-specific output templates
3. The `/osk-comply <framework>` command will automatically detect and use it

## Quick Start

```bash
# 1. Create framework directory
mkdir -p frameworks/your-framework/templates

# 2. Copy the template
cp frameworks/_template/framework.yaml.example frameworks/your-framework/framework.yaml

# 3. Edit framework.yaml with your controls
# 4. Test with: /osk-comply your-framework
```

## Framework Schema

All frameworks must conform to the schema at `frameworks/_schema/framework-schema.yaml`.

### Required Structure

```yaml
# frameworks/<framework-id>/framework.yaml

# Framework Metadata (required)
framework:
  id: "framework-id"           # Unique identifier (lowercase, hyphens)
  name: "Framework Name"       # Human-readable name
  version: "1.0"              # Framework version
  source: "Authority Name"     # Issuing authority
  description: "Brief description of the framework"
  official_url: "https://..."  # Official documentation URL

  # Applicability (optional but recommended)
  applicability:
    regions: ["EU", "FR"]      # Geographic applicability
    sectors: ["public", "healthcare"]  # Industry sectors
    data_types: ["personal_data", "health_data"]  # Relevant data types

# Security Levels (optional - omit if framework has single level)
security_levels:
  - id: "level-1"
    name: "Basic"
    description: "Minimum requirements"
    minimum_score: 70
  - id: "level-2"
    name: "Enhanced"
    description: "Enhanced requirements"
    minimum_score: 85

# Categories/Domains (required)
categories:
  - id: "cat-1"
    name: "Category Name"
    description: "What this category covers"
    reference: "Section 1"      # Reference in official document

# Controls (required)
controls:
  - id: "CTRL-001"
    category: "cat-1"          # Must match a category id
    name: "Control Name"
    description: "What this control requires"

    # Level applicability (if security_levels defined)
    levels: ["level-1", "level-2"]

    # Evidence requirements
    evidence_types:
      - type: "code"           # code, config, document, process
        paths: ["src/auth/**"]  # Glob patterns to check
        description: "Authentication implementation"
      - type: "config"
        paths: [".env*", "config/**"]
        description: "Security configuration"

    # Assessment guidance
    verification:
      automated: ["Check for password hashing in auth modules"]
      manual: ["Verify password policy documentation"]

    # Remediation guidance
    remediation:
      guidance: "Implement password hashing using bcrypt or argon2"
      references: ["https://..."]

    # Cross-framework mapping (optional)
    mappings:
      iso27001: ["A.9.4.3"]
      nist: ["IA-5"]

# Scoring Configuration (required)
scoring:
  method: "weighted"           # weighted, simple, or custom
  weights:
    compliant: 1.0
    partial: 0.5
    gap: 0.0
    not_applicable: null       # Excluded from calculation

  thresholds:
    compliant: 85              # Score >= 85 = compliant
    partial: 60                # Score >= 60 = partial compliance
    # Below 60 = non-compliant

# Cross-Framework Mapping (optional)
cross_framework_mapping:
  iso27001:
    description: "ISO 27001:2022 mapping"
    url: "https://..."
  gdpr:
    description: "GDPR article mapping"
```

## Evidence Types

OpenSecKit supports these evidence types for automatic detection:

| Type | Description | Example Paths |
|------|-------------|---------------|
| `code` | Source code implementation | `src/auth/**/*.ts` |
| `config` | Configuration files | `.env*`, `config/*.yaml` |
| `document` | Documentation files | `docs/security/*.md` |
| `process` | Process documentation | `docs/procedures/*.md` |
| `infrastructure` | Infrastructure as code | `terraform/**`, `k8s/**` |
| `tooling` | From tooling.yaml | Auto-detected |
| `integration` | From integrations.yaml | Auto-detected |

### Path Resolution

Evidence paths are resolved against the system model:

```yaml
evidence_types:
  # Check in codebase
  - type: "code"
    paths: ["src/auth/**"]

  # Check in system model
  - type: "tooling"
    paths: [".osk/system-model/tooling.yaml"]
    filter: "category == 'security'"

  # Check infrastructure
  - type: "infrastructure"
    paths: [".osk/system-model/architecture.yaml"]
    filter: "components.*.encryption"
```

## Framework-Specific Templates

You can create custom output templates in `frameworks/<framework>/templates/`:

| Template | Purpose | Required? |
|----------|---------|-----------|
| `assessment-summary.md.tera` | Assessment overview | Recommended |
| `gap-report.md.tera` | Detailed gap analysis | Optional |
| `export-report.md.tera` | Export format | Optional |

Templates use [Tera](https://tera.netlify.app/) syntax and receive the assessment context:

```jinja2
# {{ framework.name }} Assessment

**Date**: {{ assessment_date }}
**Score**: {{ overall_score }}%

## Controls

{% for control in controls %}
### {{ control.id }}: {{ control.name }}

- Status: {{ control.status }}
- Evidence: {{ control.evidence | join(sep=", ") }}
{% endfor %}
```

### Available Template Variables

```yaml
# Framework metadata
framework:
  id, name, version, source, description

# Assessment context
assessment_date: "2024-01-15"
overall_score: 85
overall_status: "compliant"

# System context
system_name: "My Application"
components:
  application: 5
  integrations: 3
  tooling: 8
  infrastructure: 2

# Controls
controls:
  - id: "CTRL-001"
    name: "Control Name"
    status: "compliant"
    evidence: ["Evidence 1", "Evidence 2"]
    gaps: []

# Gaps by category
gaps:
  - control_id: "CTRL-002"
    severity: "high"
    source: "codebase"
    description: "Missing encryption"

# Action items
action_items:
  - priority: "P0"
    title: "Implement encryption"
    domain: "Confidentiality"
```

## Validation

Before using your framework, validate it:

```bash
# Validate schema
osk validate framework frameworks/your-framework/framework.yaml

# Test assessment (dry run)
/osk-comply your-framework --dry-run
```

### Common Validation Errors

| Error | Cause | Fix |
|-------|-------|-----|
| `Unknown category` | Control references undefined category | Add category to `categories` |
| `Invalid level` | Control references undefined level | Add level to `security_levels` |
| `Missing required field` | Required field not provided | Check schema for required fields |
| `Invalid evidence type` | Unknown evidence type | Use: code, config, document, process, infrastructure, tooling, integration |

## Examples

### Minimal Framework

```yaml
framework:
  id: "minimal"
  name: "Minimal Framework"
  version: "1.0"
  source: "Example"
  description: "Minimal example framework"

categories:
  - id: "security"
    name: "Security"
    description: "Security controls"

controls:
  - id: "SEC-001"
    category: "security"
    name: "Access Control"
    description: "Implement access control"
    evidence_types:
      - type: "code"
        paths: ["src/**/*auth*"]

scoring:
  method: "simple"
  thresholds:
    compliant: 80
    partial: 50
```

### Multi-Level Framework (like RGS)

See `frameworks/rgs/framework.yaml` for a complete example with:
- Multiple security levels
- DICP security model
- Cryptographic requirements
- Cross-framework mappings
- Homologation process

### Data Privacy Framework (like GDPR)

See `frameworks/rgpd/framework.yaml` for:
- Article-based controls
- Data subject rights
- International transfers
- Sub-processor requirements

## Best Practices

1. **Start simple**: Begin with core controls, add detail iteratively
2. **Use mappings**: Map to ISO 27001 or NIST for cross-framework analysis
3. **Specific evidence paths**: More specific paths = better auto-detection
4. **Clear remediation**: Provide actionable remediation guidance
5. **Test incrementally**: Validate after each major addition

## Getting Help

- Schema reference: `frameworks/_schema/framework-schema.yaml`
- Example frameworks: `frameworks/rgs/`, `frameworks/rgpd/`
- Template syntax: [Tera documentation](https://tera.netlify.app/docs/)
