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
    documentation_language: "{{ language_code }}"  # en|fr|es|de

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
      quick_start: ".osk/docs/product.md"

    - role: "Developer"
      relevant_files: ["architecture.yaml", "glossary.yaml", "integrations.yaml", "tooling.yaml"]
      quick_start: ".osk/docs/developer.md"

    - role: "Security Engineer"
      relevant_files: ["controls.yaml", "boundaries.yaml", "data.yaml", "supply_chain.yaml"]
      quick_start: ".osk/docs/security.md"

    - role: "DevOps/SRE"
      relevant_files: ["operations.yaml", "architecture.yaml", "tooling.yaml"]
      quick_start: ".osk/docs/operations.md"

    - role: "New Team Member"
      relevant_files: ["product.yaml", "glossary.yaml", "team.yaml", "architecture.yaml"]
      quick_start: ".osk/docs/onboarding.md"

    - role: "Architect"
      relevant_files: ["architecture.yaml", "boundaries.yaml", "data.yaml", "integrations.yaml"]
      quick_start: ".osk/docs/architecture.md"
```

---

### Step 6.4: Language Selection

**Goal**: Allow the user to choose the language for generated documentation.

#### Language Selection Prompt

Before generating documentation, present the language options to the user:

```
🌐 Documentation Language Selection
===================================

Please select the language for generated documentation:

  1. English (en) - Default
  2. Français (fr)
  3. Español (es)
  4. Deutsch (de)

Your choice [1-4, default: 1]: ___
```

#### Supported Languages

| Code | Language | Template Suffix |
|------|----------|-----------------|
| `en` | English | (default, no suffix) |
| `fr` | Français | `.fr` |
| `es` | Español | `.es` |
| `de` | Deutsch | `.de` |

#### Store Language Preference

Store the selected language in the workflow state:

```yaml
workflow:
  documentation_language: "{{ selected_language_code }}"  # en|fr|es|de
```

---

### Step 6.5: Documentation Generation

**Goal**: Generate audience-specific documentation from the system model using official templates.

#### Template Repository

**CRITICAL**: Before generating each documentation file, fetch the corresponding Tera template from the latest OpenSecKit release.

**Step 1**: Get latest release tag:
```bash
gh api repos/Scttpr/OpenSecKit/releases/latest --jq '.tag_name'
```

**Step 2**: Fetch templates using the tag and selected language:

For the selected language code (`{lang}`), use this template URL pattern:
- English (`en`): `{template}.md.tera` (default)
- Other languages: `{template}.{lang}.md.tera`

| Output File | Template URL (English) | Template URL (Other: replace `{lang}`) |
|-------------|------------------------|----------------------------------------|
| `docs/product.md` | `.../outputs/product.md.tera` | `.../outputs/product.{lang}.md.tera` |
| `docs/architecture.md` | `.../outputs/architecture.md.tera` | `.../outputs/architecture.{lang}.md.tera` |
| `docs/developer.md` | `.../outputs/developer.md.tera` | `.../outputs/developer.{lang}.md.tera` |
| `docs/security.md` | `.../outputs/security.md.tera` | `.../outputs/security.{lang}.md.tera` |
| `docs/operations.md` | `.../outputs/operations.md.tera` | `.../outputs/operations.{lang}.md.tera` |
| `docs/onboarding.md` | `.../outputs/onboarding.md.tera` | `.../outputs/onboarding.{lang}.md.tera` |

Base URL: `https://raw.githubusercontent.com/Scttpr/OpenSecKit/{tag}/kit/discover/templates/outputs/`

**Fallback**: If a language-specific template is not found (404), fall back to the English template and translate the content during generation.

#### Template Rendering Process

**You MUST create each documentation file.** For each documentation file:

1. **Fetch the template** from the URL above using WebFetch
2. **Read the template header** to understand ownership rules (what it OWNS vs REFERENCES)
3. **Load all generated YAML files** as context data from `.osk/system-model/`:
   - `product.yaml`, `business.yaml`, `glossary.yaml`
   - `architecture.yaml`, `data.yaml`, `actors.yaml`
   - `boundaries.yaml`, `user-journeys.yaml`, `integrations.yaml`
   - `supply_chain.yaml`, `controls.yaml`, `tooling.yaml`
   - `team.yaml`, `operations.yaml`, `gaps.yaml`, `index.yaml`
4. **Render the template** by replacing Tera variables with actual data
5. **Generate Mermaid diagrams** where the template specifies them
6. **Include all dashboard metrics** as defined in the template
7. **Ensure cross-references** link to the correct related documents
8. **Write the rendered content** to `.osk/docs/{filename}.md` using the Write tool

#### Template Variable Mapping

Templates use Tera syntax. Map YAML data to template variables:

```
Template Variable          →  YAML Source
─────────────────────────────────────────────────────
{{ project.name }}         →  index.yaml → project.name
{{ metadata.generated_at }} →  Current timestamp
{{ product.* }}            →  product.yaml
{{ features }}             →  product.yaml → features[]
{{ architecture.* }}       →  architecture.yaml
{{ data.* }}               →  data.yaml
{{ actors.* }}             →  actors.yaml
{{ boundaries.* }}         →  boundaries.yaml
{{ user_journeys.* }}      →  user-journeys.yaml
{{ integrations }}         →  integrations.yaml → integrations[]
{{ supply_chain.* }}       →  supply_chain.yaml
{{ controls.* }}           →  controls.yaml
{{ tooling.* }}            →  tooling.yaml
{{ team.* }}               →  team.yaml
{{ operations.* }}         →  operations.yaml
{{ gaps.* }}               →  gaps.yaml
{{ glossary.* }}           →  glossary.yaml
{{ stats.* }}              →  index.yaml → stats
{{ compliance_hints.* }}   →  index.yaml → compliance_hints
{{ kpis.* }}               →  product.yaml → kpis
{{ roadmap.* }}            →  product.yaml → roadmap
{{ business_context.* }}   →  business.yaml → business_context
{{ business_processes }}   →  business.yaml → business_processes
{{ business_rules }}       →  business.yaml → business_rules
```

#### Generation Checklist

For each document:

- [ ] Template fetched from GitHub
- [ ] Template header ownership rules read
- [ ] All YAML context files loaded
- [ ] All template sections rendered
- [ ] Dashboard metrics included
- [ ] Mermaid diagrams generated (where applicable)
- [ ] Cross-references to other docs are correct
- [ ] Links to YAML files use relative paths (`../system-model/*.yaml`)
- [ ] **File written to `.osk/docs/` using Write tool**

#### Documentation Menu

```
📚 Documentation Generation
===========================

Selected language: {{ language_name }} ({{ language_code }})
Fetching templates from OpenSecKit repository...

Generating documentation:
[x] .osk/docs/product.md      ← product{{ lang_suffix }}.md.tera
[x] .osk/docs/architecture.md ← architecture{{ lang_suffix }}.md.tera
[x] .osk/docs/developer.md    ← developer{{ lang_suffix }}.md.tera
[x] .osk/docs/security.md     ← security{{ lang_suffix }}.md.tera
[x] .osk/docs/operations.md   ← operations{{ lang_suffix }}.md.tera
[x] .osk/docs/onboarding.md   ← onboarding{{ lang_suffix }}.md.tera

Output location: .osk/docs/
Language: {{ language_code }}
```

Where `{{ lang_suffix }}` is empty for English, or `.{lang}` for other languages (e.g., `.fr` for French).

**IMPORTANT**: After displaying this menu, you MUST actually create each file using the Write tool.

---

### Step 6.6: Final Summary

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
├── .osk/docs/product.md ✓
├── .osk/docs/developer.md ✓
├── .osk/docs/security.md ✓
├── .osk/docs/operations.md ✓
├── .osk/docs/onboarding.md ✓
└── .osk/docs/architecture.md ✓

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

### YAML Templates

**Fetch YAML templates from latest release before generating outputs.**

| Output | Template URL |
|--------|--------------|
| `gaps.yaml` | `https://raw.githubusercontent.com/Scttpr/OpenSecKit/{tag}/kit/discover/templates/data/gaps.yaml.tera` |
| `index.yaml` | `https://raw.githubusercontent.com/Scttpr/OpenSecKit/{tag}/kit/discover/templates/data/index.yaml.tera` |

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

### Documentation files (in .osk/docs/)

**CRITICAL**: You MUST create each documentation file. For each file:

1. **Fetch the template** from GitHub using WebFetch
2. **Load context** from all generated YAML files in `.osk/system-model/`
3. **Render the template** by replacing Tera variables with actual data
4. **Write the file** to `.osk/docs/{filename}.md` using the Write tool

| Template | Output File |
|----------|-------------|
| product.md.tera | `.osk/docs/product.md` |
| developer.md.tera | `.osk/docs/developer.md` |
| security.md.tera | `.osk/docs/security.md` |
| operations.md.tera | `.osk/docs/operations.md` |
| onboarding.md.tera | `.osk/docs/onboarding.md` |
| architecture.md.tera | `.osk/docs/architecture.md` |

**DO NOT skip file creation.** Each documentation file must be written to disk.

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
      - ".osk/docs/product.md"
      - ".osk/docs/developer.md"
      - ".osk/docs/security.md"
      - ".osk/docs/operations.md"
      - ".osk/docs/onboarding.md"
      - ".osk/docs/architecture.md"
    result:
      gap_count: {{ count }}
      health_score: {{ score }}
      documentation_generated: {{ count }}
      documentation_language: "{{ language_code }}"

workflow:
  status: "completed"
  completed_at: "{{ timestamp }}"
  total_duration: "{{ duration }}"
  files_generated: {{ count }}
  documentation_language: "{{ language_code }}"
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
