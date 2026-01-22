---
description: RGS (French Government Security) compliance - Full Homologation Workflow Orchestrator
part: comply
framework: rgs
version: "5.0.0"
model_sections: [index, architecture, controls, data, integrations, tooling, actors, business, boundaries]
sub_prompts:
  - prompts/01-level-assessment.md
  - prompts/02-ebios-rm.md
  - prompts/03-assess.md
  - prompts/04-gaps.md
  - prompts/05-dossier.md
---

# RGS Full Homologation Workflow

This framework provides a complete RGS (Referentiel General de Securite) homologation workflow for French government information systems, following the ANSSI 9-step homologation process.

## Workflow Overview

```
                        RGS HOMOLOGATION WORKFLOW
    =========================================================================

    ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
    │   PHASE 1   │     │   PHASE 2   │     │   PHASE 3   │
    │   LEVEL     │────▶│  EBIOS RM   │────▶│   CONTROL   │
    │ ASSESSMENT  │     │ (Conditional)│     │ ASSESSMENT  │
    └─────────────┘     └─────────────┘     └─────────────┘
          │                   │                   │
          ▼                   ▼                   ▼
    level-assessment    ebios-rm/            assessment-rgs
        .yaml           *.yaml                  .yaml
                              │
                              │
    ┌─────────────┐     ┌─────────────┐
    │   PHASE 4   │     │   PHASE 5   │
    │    GAPS     │────▶│   DOSSIER   │
    │  ANALYSIS   │     │ GENERATION  │
    └─────────────┘     └─────────────┘
          │                   │
          ▼                   ▼
    gaps-analysis.yaml  dossier-homologation/
    remediation-        ├── main.md
    roadmap.md          └── annexes/
```

## Command

```bash
/osk-comply rgs [--update] [--export <md|pdf>]
```

The workflow is **autonomous** - it automatically determines which phase to execute based on workflow state and progresses through all phases without manual intervention.

## Workflow State Management

The orchestrator maintains workflow state in `.osk/comply/rgs/workflow-state.yaml`:

```yaml
workflow:
  version: "5.0.0"
  started_at: "2025-01-21T10:00:00Z"
  last_updated: "2025-01-21T14:30:00Z"
  current_phase: "ebios-rm"
  current_step: "workshop-3"

  phases:
    level_assessment:
      status: completed
      output: "level-assessment.yaml"
      completed_at: "2025-01-21T10:30:00Z"
      result:
        determined_level: "RGS**"

    ebios_rm:
      status: in_progress
      required: true
      workshops:
        - id: 1
          name: "Cadrage et socle de securite"
          status: completed
        - id: 2
          name: "Sources de risque"
          status: completed
        - id: 3
          name: "Scenarios strategiques"
          status: in_progress
        - id: 4
          name: "Scenarios operationnels"
          status: pending
        - id: 5
          name: "Traitement du risque"
          status: pending

    control_assessment:
      status: pending

    gaps_analysis:
      status: pending

    dossier_generation:
      status: pending
```

---

# Orchestration Logic

## Phase 1: Level Assessment

**Sub-prompt**: `prompts/01-level-assessment.md`

**Purpose**: Determine the appropriate RGS security level based on system characteristics.

**Entry conditions**:
- System model exists (`.osk/system-model/index.yaml`)
- No existing level assessment OR user requests reassessment

**Decision tree**:
```
System characteristics:
├── OIV / Critical infrastructure? ───────────────▶ RGS***
├── Sensitive data (health, legal, financial)? ───▶ RGS**
├── Public-facing government service? ────────────▶ RGS** (recommended)
└── Internal administrative system? ──────────────▶ RGS*
```

**Output**: `.osk/comply/rgs/level-assessment.yaml`

**Next phase determination**:
```yaml
if level == "RGS***":
  next: "ebios_rm"  # Mandatory
  ebios_required: true

elif level == "RGS**":
  next: "ebios_rm"  # Strongly recommended
  ebios_required: true
  # User can skip with justification

elif level == "RGS*":
  # Ask user
  prompt: "EBIOS RM risk analysis is recommended but optional for RGS*. Proceed with EBIOS RM?"
  if yes: next: "ebios_rm"
  else: next: "control_assessment"
```

---

## Phase 2: EBIOS RM (Conditional)

**Sub-prompt**: `prompts/02-ebios-rm.md`

**Purpose**: Conduct formal risk analysis following ANSSI EBIOS Risk Manager methodology.

**Entry conditions**:
- Level assessment completed
- EBIOS RM required OR user opted in

**5 Workshops**:

| Workshop | Name | Input | Output |
|----------|------|-------|--------|
| 1 | Cadrage et socle de securite | system-model, business.yaml | Scope, feared events, security baseline |
| 2 | Sources de risque | Workshop 1 | Risk sources, threat actors |
| 3 | Scenarios strategiques | Workshops 1-2, integrations.yaml | Ecosystem attack paths |
| 4 | Scenarios operationnels | Workshops 1-3, architecture.yaml | Technical attack scenarios |
| 5 | Traitement du risque | Workshops 1-4, controls.yaml | Risk treatment plan, residual risks |

**Output**: `.osk/comply/rgs/ebios-rm/`
```
ebios-rm/
├── workshop-1-cadrage.yaml
├── workshop-2-sources.yaml
├── workshop-3-strategic.yaml
├── workshop-4-operational.yaml
├── workshop-5-treatment.yaml
├── risk-register.yaml
├── feared-events.yaml
└── summary.md
```

**Skip conditions** (RGS* only):
```yaml
if level == "RGS*" and user_skips:
  record:
    ebios_rm:
      status: "skipped"
      justification: "[user provided]"
      simplified_risk_analysis: true  # Use simplified approach in Phase 3
  next: "control_assessment"
```

---

## Phase 3: Control Assessment

**Sub-prompt**: `prompts/03-assess.md`

**Purpose**: Evaluate 26 RGS controls across 6 domains.

**Entry conditions**:
- Level assessment completed
- EBIOS RM completed OR skipped with justification

**Domains**:
| Domain | Controls | Focus |
|--------|----------|-------|
| AUTH | 5 | Authentication mechanisms |
| INT | 4 | Data integrity |
| CONF | 4 | Confidentiality, encryption |
| TRAC | 5 | Traceability, logging |
| HORO | 2 | Timestamping |
| SIG | 2 | Electronic signatures |

**Input from previous phases**:
```yaml
from_level_assessment:
  - rgs_level
  - dicp_requirements

from_ebios_rm:
  - feared_events (map to controls)
  - risk_treatment_decisions (validate controls address risks)
  - residual_risks (document in assessment)
```

**Output**: `.osk/comply/rgs/assessment-rgs.yaml`

---

## Phase 4: Gap Analysis

**Sub-prompt**: `prompts/04-gaps.md`

**Purpose**: Analyze compliance gaps and generate remediation roadmap.

**Entry conditions**:
- Control assessment completed

**Gap categories**:
```yaml
categories:
  - codebase      # Code-level security issues
  - infrastructure # Hosting, cloud, network
  - tooling       # CI/CD, monitoring, collaboration
  - process       # Procedures, policies, documentation
  - evidence      # Missing security artifacts
```

**Prioritization**:
```
┌────────────┬─────────────────────────────────────────┐
│ Priority   │ Criteria                                │
├────────────┼─────────────────────────────────────────┤
│ P0 BLOCKER │ Prevents homologation                   │
│ P1 HIGH    │ Required for target level, quick fix   │
│ P2 MEDIUM  │ Required for target level, effort >1wk │
│ P3 LOW     │ Recommended improvement                │
└────────────┴─────────────────────────────────────────┘
```

**Output**:
```
.osk/comply/rgs/
├── gaps-analysis.yaml
├── remediation-roadmap.md
└── quick-wins.md
```

---

## Phase 5: Dossier Generation

**Sub-prompt**: `prompts/05-dossier.md`

**Purpose**: Generate formal homologation dossier following ANSSI structure.

**Entry conditions**:
- Gap analysis completed
- OR user explicitly requests draft with blockers

**ANSSI 9-Step Homologation Structure**:

| Step | Dossier Section | Source |
|------|-----------------|--------|
| 1 | Perimetre du systeme | level-assessment.yaml, system-model |
| 2 | Besoins DICP | level-assessment.yaml |
| 3 | Analyse des risques | ebios-rm/ OR simplified |
| 4 | Mesures de securite | assessment-rgs.yaml |
| 5 | Risques residuels | ebios-rm/workshop-5, gaps-analysis |
| 6 | Resultats d'audits | assessment-rgs.yaml (evidence) |
| 7 | Elements de decision | All phases synthesis |
| 8 | Decision d'homologation | [Commission placeholder] |
| 9 | Plan de maintien (MCS) | remediation-roadmap.md |

**Output**:
```
.osk/comply/rgs/dossier-homologation/
├── 00-index.md
├── 01-perimetre.md
├── 02-dicp.md
├── 03-analyse-risques.md
├── 04-mesures-securite.md
├── 05-risques-residuels.md
├── 06-audits.md
├── 07-elements-decision.md
├── 08-decision-placeholder.md
├── 09-plan-mcs.md
└── annexes/
    ├── A-architecture.md
    ├── B-controles-rgs.md
    ├── C-ebios-synthese.md
    ├── D-certifications.md
    └── E-ecarts-remediation.md
```

**Watermark logic**:
```yaml
if gaps_analysis.blockers > 0:
  watermark: "PROJET - [N] points bloquants non resolus"
  status: "draft"
else:
  watermark: null
  status: "ready_for_commission"
```

---

# Orchestrator Behavior

## Starting a New Workflow

```
1. Check prerequisites
   ├── System model exists? ──▶ If no: ERROR "Run /osk-discover first"
   └── workflow-state.yaml exists?
       ├── If yes: Prompt "Resume existing workflow or start new?"
       └── If no: Initialize new workflow-state.yaml

2. Execute Phase 1 (Level Assessment)
   └── Save checkpoint after completion

3. Determine Phase 2 path
   ├── EBIOS required? ──▶ Execute Phase 2
   └── EBIOS skipped? ──▶ Record justification, skip to Phase 3

4. Execute remaining phases sequentially
   └── Save checkpoint after each phase

5. Generate final dossier
   └── Display completion summary
```

## Resuming Workflow

```
1. Load workflow-state.yaml
2. Display current state summary
3. Prompt: "Resume from [current_phase].[current_step]?"
4. If yes: Continue from saved state
5. If no: Offer options
   ├── Restart from specific phase
   ├── Start completely new workflow
   └── Cancel
```

## Error Handling

```yaml
on_error:
  - Save current state to workflow-state.yaml
  - Log error details
  - Display recovery options:
    - Retry current step
    - Skip current step (if allowed)
    - Rollback to previous checkpoint
    - Abort workflow
```

---

# Flags

## --update

Re-assess only components that have changed since last run. Compares system-model timestamps with workflow state.

## --export <format>

Export homologation dossier in specified format:
- `md` - Markdown (default)
- `pdf` - PDF (requires pandoc)

---

# Dependencies

## Required System Model Sections

| Section | Required For |
|---------|--------------|
| index.yaml | All phases |
| architecture.yaml | Phases 2, 3, 5 |
| controls.yaml | Phases 3, 4, 5 |
| data.yaml | Phases 1, 2, 3 |
| integrations.yaml | Phases 2, 3 |
| tooling.yaml | Phases 3, 4 |
| actors.yaml | Phases 2, 3 |
| business.yaml | Phases 1, 2 |
| boundaries.yaml | Phases 1, 5 |

## Knowledge Base

All sub-prompts have access to:
- `knowledge/rgs-v2-*.md` - Official RGS v2.0 annexes
- `knowledge/guide-homologation-securite.md` - ANSSI homologation guide
- `knowledge/ebios-risk-manager.md` - EBIOS RM methodology
- `knowledge/guide-hygiene-informatique.md` - 42 security measures

## Schemas

| Schema | Purpose |
|--------|---------|
| `schemas/workflow-state.yaml` | Workflow progress tracking |
| `schemas/level-assessment.yaml` | Phase 1 output |
| `schemas/ebios-rm.yaml` | Phase 2 output |
| `schemas/assessment-extension.yaml` | Phase 3 output (existing) |
| `schemas/gaps-analysis.yaml` | Phase 4 output |
| `schemas/tool-certifications.yaml` | Certification validation |

---

# Output Structure

After complete workflow:

```
.osk/comply/rgs/
├── workflow-state.yaml           # Workflow tracking
├── level-assessment.yaml         # Phase 1 output
├── ebios-rm/                     # Phase 2 output
│   ├── workshop-1-cadrage.yaml
│   ├── workshop-2-sources.yaml
│   ├── workshop-3-strategic.yaml
│   ├── workshop-4-operational.yaml
│   ├── workshop-5-treatment.yaml
│   ├── risk-register.yaml
│   └── summary.md
├── assessment-rgs.yaml           # Phase 3 output
├── assessment-rgs.md             # Human-readable summary
├── gaps-analysis.yaml            # Phase 4 output
├── remediation-roadmap.md        # Prioritized actions
├── quick-wins.md                 # Low-effort high-impact fixes
└── dossier-homologation/         # Phase 5 output
    ├── 00-index.md
    ├── 01-perimetre.md
    ├── 02-dicp.md
    ├── 03-analyse-risques.md
    ├── 04-mesures-securite.md
    ├── 05-risques-residuels.md
    ├── 06-audits.md
    ├── 07-elements-decision.md
    ├── 08-decision-placeholder.md
    ├── 09-plan-mcs.md
    └── annexes/
```

---

# Related Documentation

- `WORKFLOW-AND-SCORING.md` - Scoring algorithms and DICP calculation
- `CRYPTO-REFERENCE.md` - ANSSI-approved cryptographic algorithms
- `SYSTEM-MODEL-REQUIREMENTS.md` - Required system model data by control
- `README.md` - User documentation

# Related Frameworks

- `rgpd` - GDPR data protection compliance
- Coming: `nis2`, `iso27001`
