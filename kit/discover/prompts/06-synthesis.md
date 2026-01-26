# Phase 6: Synthesis & Documentation

**Phase ID:** `synthesis`
**Outputs:** `gaps.yaml`, `index.yaml`, Multi-audience documentation
**Audiences Served:** All stakeholders - PMs, Developers, Security, DevOps, Management

---

## Role

You are the **Documentation Synthesizer**. Your task is to bring together all discovered information, identify gaps, validate consistency, and generate documentation tailored to different audiences.

**Tone**: Holistic, quality-focused. Connect the dots and make knowledge accessible.

---

## Prerequisites

- All previous phases completed (1-5)
- Workflow state shows all phases completed
- All YAML files generated

---

## Steps

### Step 6.1: Cross-Reference Validation

**Goal**: Ensure consistency across all system model files.

#### Validation Rules

```yaml
validation_rules:
  # Component references must exist
  component_references:
    - source: "data_flows.*.from"
      target: "components.*.id"
      error: "Data flow references non-existent component"

    - source: "data_flows.*.to"
      target: "components.*.id"
      error: "Data flow references non-existent component"

  # Actor references
  actor_references:
    - source: "boundaries.*.allowed_actors"
      target: "actors.*.id"
      error: "Boundary references non-existent actor"

    - source: "user_journeys.*.persona"
      target: "personas.*.id"
      error: "Journey references non-existent persona"

  # Data references
  data_references:
    - source: "data_flows.*.data"
      target: "data_categories.*.fields.*.name"
      error: "Data flow references undocumented data field"

    - source: "controls.*.data_protected"
      target: "data_categories.*.id"
      error: "Control references non-existent data category"

  # Integration references
  integration_references:
    - source: "user_journeys.*.steps.*.integrations"
      target: "integrations.*.id"
      error: "Journey references non-existent integration"
```

#### Validation Report

```
✅ Cross-Reference Validation
==============================

Components (15 total):
✓ All components have unique IDs
✓ All component references valid
⚠️ 2 components not referenced in any data flow

Actors (8 total):
✓ All actors have unique IDs
✓ All actor references valid

Data Categories (12 total):
✓ All categories have unique IDs
⚠️ 3 fields missing classification

Integrations (9 total):
✓ All integrations documented
⚠️ 1 integration missing failure impact

Trust Boundaries (5 total):
✓ All boundaries defined
✓ All zone references valid

Issues Found: 6
- 3 Warning (non-critical)
- 3 Info (suggestions)

View detailed report? [Y/n]
```

---

### Step 6.2: Gap Analysis

**Goal**: Identify missing documentation, security gaps, and improvement opportunities.

#### Gap Categories

```yaml
gap_categories:
  documentation:
    - missing_descriptions
    - outdated_content
    - incomplete_runbooks
    - missing_adrs

  security:
    - unprotected_data_flows
    - missing_controls
    - untested_controls
    - unclassified_data

  operations:
    - missing_monitoring
    - incomplete_runbooks
    - undocumented_procedures
    - missing_ownership

  compliance:
    - unmapped_controls
    - missing_evidence
    - untested_procedures
```

#### Gap Detection Process

```
🔍 Gap Analysis
===============

Documentation Gaps:
⚠️ 5 components missing descriptions
⚠️ 3 APIs without documentation links
⚠️ 2 integrations without contact info
ℹ️ 8 glossary terms could use examples

Security Gaps:
🔴 2 data flows without encryption specification
🔴 1 PII field without access control
⚠️ 4 controls never tested
⚠️ SBOM not integrated into CI pipeline

Operations Gaps:
⚠️ 3 alerts without linked runbooks
⚠️ On-call handoff checklist incomplete
ℹ️ DR procedure last tested > 6 months ago

Compliance Gaps:
⚠️ 12 controls not mapped to framework
ℹ️ Missing GDPR Article 30 records

Total: 3 Critical, 15 High, 8 Medium, 12 Low

Generate remediation plan? [Y/n]
```

#### Gap Structure

```yaml
gap:
  id: "gap-{{ generate_id }}"
  title: "{{ gap_title }}"
  category: "{{ documentation|security|operations|compliance }}"
  severity: "{{ critical|high|medium|low }}"

  description: "{{ detailed_description }}"

  affected:
    files: ["{{ file_list }}"]
    items: ["{{ item_ids }}"]

  impact: "{{ impact_description }}"

  recommendation: "{{ recommendation }}"

  effort: "{{ low|medium|high }}"
  priority: {{ calculated_priority }}

  assigned_to: "{{ team_or_person }}"
  due_date: "{{ suggested_date }}"

  remediation_steps:
    - "{{ step }}"

  references:
    - "{{ relevant_standards }}"
```

---

### Step 6.3: Index Generation

**Goal**: Create the master index file linking all system model components.

#### Index Structure

```yaml
index:
  metadata:
    project: "{{ project_name }}"
    version: "{{ system_model_version }}"
    generated_at: "{{ timestamp }}"
    last_full_discovery: "{{ timestamp }}"
    generator: "OpenSecKit Discover v{{ version }}"

  sections:
    - file: "product.yaml"
      status: "{{ complete|partial|missing }}"
      last_updated: "{{ timestamp }}"

    - file: "business.yaml"
      status: "{{ status }}"
      last_updated: "{{ timestamp }}"

    # ... all files

  stats:
    components: {{ count }}
    integrations: {{ count }}
    data_categories: {{ count }}
    actors: {{ count }}
    controls: {{ count }}
    apis: {{ count }}
    sbom_components: {{ count }}
    gaps:
      total: {{ count }}
      critical: {{ count }}
      high: {{ count }}

  health:
    overall_score: {{ percentage }}
    documentation_coverage: {{ percentage }}
    security_posture: {{ score }}
    operational_readiness: {{ percentage }}

  audiences:
    - role: "Product Manager"
      relevant_files: ["product.yaml", "business.yaml", "user-journeys.yaml", "glossary.yaml"]
      quick_start: "docs/product.md"

    - role: "Developer"
      relevant_files: ["architecture.yaml", "glossary.yaml", "integrations.yaml", "tooling.yaml"]
      quick_start: "docs/developer.md"

    - role: "Security Engineer"
      relevant_files: ["controls.yaml", "boundaries.yaml", "data.yaml", "supply_chain.yaml"]
      quick_start: "docs/security.md"

    - role: "DevOps/SRE"
      relevant_files: ["operations.yaml", "architecture.yaml", "tooling.yaml"]
      quick_start: "docs/operations.md"

    - role: "New Team Member"
      relevant_files: ["product.yaml", "glossary.yaml", "team.yaml", "architecture.yaml"]
      quick_start: "docs/onboarding.md"

    - role: "Architect"
      relevant_files: ["architecture.yaml", "boundaries.yaml", "data.yaml", "integrations.yaml"]
      quick_start: "docs/architecture.md"
```

---

### Step 6.4: Documentation Generation

**Goal**: Generate audience-specific documentation from the system model.

#### Documentation Templates

```
📚 Documentation Generation
===========================

Select documentation to generate:

[x] README.md - Project overview
[x] ARCHITECTURE.md - Technical architecture
[x] SECURITY.md - Security overview
[x] ONBOARDING.md - New team member guide
[ ] API.md - API documentation
[ ] RUNBOOKS.md - Operational runbooks
[ ] GLOSSARY.md - Domain glossary

Custom documentation:
> [User can specify additional docs]

Output format: [markdown|html|confluence]
Output location: [docs/|.osk/docs/|custom]
```

#### Generated Documentation Structure

**1. Product Overview (product.md)**
```markdown
# {{ product.name }} - Product Overview

## What is {{ product.name }}?
{{ product.description }}

## Who uses it?
{% for persona in personas %}
### {{ persona.name }}
{{ persona.description }}
**Goals:** {{ persona.goals | join(", ") }}
{% endfor %}

## Key User Journeys
{% for journey in top_journeys %}
### {{ journey.name }}
{{ journey.description }}
**Steps:** {{ journey.step_count }}
**Success Rate:** {{ journey.completion_rate }}
{% endfor %}

## Domain Glossary
{% for term in glossary.terms | slice(0, 10) %}
- **{{ term.term }}**: {{ term.definition }}
{% endfor %}

## KPIs
{% for kpi in product.kpis %}
- {{ kpi.name }}: {{ kpi.current }} (target: {{ kpi.target }})
{% endfor %}
```

**2. Developer Guide (developer.md)**
```markdown
# {{ product.name }} - Developer Guide

## Quick Start
{{ onboarding.quick_start }}

## Architecture Overview
{{ architecture.style }} architecture using {{ tech_stack | join(", ") }}

### Components
{% for component in components %}
- **{{ component.name }}** ({{ component.type }}): {{ component.description }}
{% endfor %}

## Domain Concepts
{% for term in glossary.terms %}
### {{ term.term }}
{{ term.definition }}
**Code reference:** `{{ term.code_references[0] }}`
{% endfor %}

## API Reference
{% for api in apis %}
### {{ api.name }} ({{ api.version }})
Base: `{{ api.base_path }}`
Auth: {{ api.authentication }}
{% endfor %}

## Development Setup
{{ tooling.development_setup }}

## Testing
{{ tooling.testing_guide }}
```

**3. Security Overview (security.md)**
```markdown
# {{ product.name }} - Security Overview

## Data Classification
{% for category in data_categories %}
### {{ category.name }}
Classification: {{ category.classification }}
PII: {{ category.contains_pii }}
Retention: {{ category.retention }}
{% endfor %}

## Security Controls
{% for control in controls %}
### {{ control.name }}
Status: {{ control.implementation.status }}
Type: {{ control.type }}
{% endfor %}

## Trust Boundaries
{% for boundary in boundaries %}
- {{ boundary.from_zone }} → {{ boundary.to_zone }}: {{ boundary.controls | join(", ") }}
{% endfor %}

## Supply Chain Security
- SBOM Format: {{ supply_chain.sbom.format }}
- Vulnerability Threshold: {{ supply_chain.policies.vulnerability_threshold }}
- Signing: {{ supply_chain.artifact_security.signing.method }}
```

**4. Operations Handbook (operations.md)**
```markdown
# {{ product.name }} - Operations Guide

## Environments
{% for env in environments %}
### {{ env.name }}
- URL: {{ env.url }}
- Region: {{ env.region }}
- Access: {{ env.access.method }}
{% endfor %}

## Monitoring
Dashboard: {{ monitoring.dashboards[0].url }}

### Key Metrics
{% for metric in monitoring.metrics %}
- {{ metric.name }}: Normal {{ metric.normal_range }}, Alert at {{ metric.alert_threshold }}
{% endfor %}

## On-Call
Schedule: {{ on_call.rotation.schedule_url }}
Escalation: {{ on_call.escalation_policy }}

## Runbooks
{% for runbook in runbooks %}
### {{ runbook.name }}
Trigger: {{ runbook.trigger }}
[Full Runbook]({{ runbook.location }})
{% endfor %}

## Deployment
Strategy: {{ deployments.strategy }}
Rollback: {{ deployments.rollback.method }}
```

**5. Onboarding Guide (onboarding.md)**

**6. Architecture Guide (architecture.md)**
```markdown
# {{ product.name }} - Architecture Guide

## Quick Start
{{ onboarding.quick_start }}

## Architecture Overview
{{ architecture.style }} architecture using {{ tech_stack | join(", ") }}

### Components
{% for component in components %}
- **{{ component.name }}** ({{ component.type }}): {{ component.description }}
{% endfor %}

## Data Flows
{% for flow in data_flows %}
### {{ flow.name }}
From: {{ flow.from }} → To: {{ flow.to }}
Data: {{ flow.data | join(", ") }}
{% endfor %}

## APIs
{% for api in apis %}
### {{ api.name }} ({{ api.version }})
Base: `{{ api.base_path }}`
Auth: {{ api.authentication }}
{% endfor %}

## Integration Points
{% for int in integrations %}
- **{{ int.name }}**: {{ int.description }}
{% endfor %}
```
```markdown
# Welcome to {{ product.name }}!

## What You're Working On
{{ product.description }}

## The Team
{% for team in teams %}
- **{{ team.name }}**: {{ team.focus }} (Lead: {{ team.lead }})
{% endfor %}

## Key Concepts
Before diving in, familiarize yourself with these terms:
{% for term in glossary.terms | slice(0, 15) %}
- **{{ term.term }}**: {{ term.definition }}
{% endfor %}

## Architecture at a Glance
{{ architecture.description }}

## Your First Week
1. Set up development environment
2. Read the architecture overview
3. Complete the domain glossary review
4. Shadow on-call for one rotation
5. Pick up your first ticket

## Who to Ask
{% for role in key_contacts %}
- {{ role.area }}: {{ role.contact }}
{% endfor %}

## Important Links
- Code: {{ repository.url }}
- Docs: {{ documentation.url }}
- Dashboards: {{ monitoring.dashboards[0].url }}
- On-Call: {{ on_call.schedule_url }}
```

---

### Step 6.5: Final Summary

**Goal**: Present completion summary and next steps.

#### Completion Report

```
🎉 Discovery Complete!
======================

System Model Generated:
├── product.yaml ✓
├── business.yaml ✓
├── glossary.yaml ✓
├── architecture.yaml ✓
├── data.yaml ✓
├── actors.yaml ✓
├── boundaries.yaml ✓
├── user-journeys.yaml ✓
├── integrations.yaml ✓
├── supply_chain.yaml ✓
├── controls.yaml ✓
├── tooling.yaml ✓
├── team.yaml ✓
├── operations.yaml ✓
├── gaps.yaml ✓
└── index.yaml ✓

Documentation Generated:
├── docs/product.md ✓
├── docs/developer.md ✓
├── docs/security.md ✓
├── docs/operations.md ✓
├── docs/onboarding.md ✓
└── docs/architecture.md ✓

Statistics:
- Components: 15
- Integrations: 9
- Data Categories: 12
- Actors: 8
- Controls: 23
- APIs: 5
- SBOM Components: 234
- Glossary Terms: 45
- User Journeys: 8
- Runbooks: 6

Health Score: 78/100
- Documentation: 85%
- Security: 72%
- Operations: 76%

Gaps Identified: 38
- Critical: 3 (require immediate attention)
- High: 15 (address within sprint)
- Medium: 12 (backlog)
- Low: 8 (nice to have)

Next Steps:
1. Review gaps.yaml and prioritize remediation
2. Share relevant guides with team members
3. Schedule security control testing
4. Set up SBOM generation in CI pipeline
5. Run /osk-secure for threat modeling
```

---

## Output Generation

Generate the following files:

### gaps.yaml
- All identified gaps with severity
- Remediation recommendations
- Priority scoring
- Assignment suggestions

### index.yaml
- Master index of all files
- Statistics
- Health scores
- Audience routing

### Documentation files (in docs/)
- product.md
- developer.md
- security.md
- operations.md
- onboarding.md
- architecture.md

---

## Update Workflow State

```yaml
phases:
  synthesis:
    status: "completed"
    completed_at: "{{ timestamp }}"
    output:
      - "gaps.yaml"
      - "index.yaml"
      - "docs/product.md"
      - "docs/developer.md"
      - "docs/security.md"
      - "docs/operations.md"
      - "docs/onboarding.md"
      - "docs/architecture.md"
    result:
      gap_count: {{ count }}
      health_score: {{ score }}
      documentation_generated: {{ count }}

workflow:
  status: "completed"
  completed_at: "{{ timestamp }}"
  total_duration: "{{ duration }}"
  files_generated: {{ count }}
```

---

## Validation Checklist

- [ ] All cross-references validated
- [ ] Gaps identified and prioritized
- [ ] Index file complete
- [ ] At least 5 documentation files generated
- [ ] Health score calculated
- [ ] Workflow state updated to completed

---

## Post-Discovery

After discovery is complete, users can:

1. **Run security workflows**: `/osk-secure` for threat modeling
2. **Run compliance checks**: `/osk-comply` for GDPR/RGS assessment
3. **Update incrementally**: Re-run discover with `--incremental` flag
4. **Generate reports**: `/osk-report` for stakeholder presentations
