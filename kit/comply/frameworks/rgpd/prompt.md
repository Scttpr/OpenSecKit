---
description: RGPD/GDPR compliance - Main orchestrator prompt
part: comply
framework: rgpd
version: "5.0.0"
model_sections: [index, data, actors, integrations, tooling, architecture, controls, business, boundaries, team]
sub_prompts:
  - prompts/01-inventory.md
  - prompts/02-aipd.md
  - prompts/03-assess.md
  - prompts/04-gaps.md
  - prompts/05-generate.md
---

# RGPD Compliance Framework v5.0

This framework provides structured RGPD (GDPR) compliance workflow following CNIL methodology for processing inventory, impact assessment, control evaluation, gap analysis, and documentation generation.

## Quick Start

```bash
# Full workflow (recommended)
/osk-comply rgpd                  # Start/resume 5-phase workflow

# Individual phases
/osk-comply rgpd inventory        # Phase 1: Processing inventory
/osk-comply rgpd aipd             # Phase 2: AIPD/DPIA (if required)
/osk-comply rgpd assess           # Phase 3: Control assessment
/osk-comply rgpd gaps             # Phase 4: Gap analysis
/osk-comply rgpd generate         # Phase 5: Documentation

# Utilities
/osk-comply rgpd status           # View current workflow state
/osk-comply rgpd resume           # Resume from last phase
```

## 5-Phase Workflow Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    RGPD COMPLIANCE WORKFLOW v5.0                         │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │ PHASE 1: PROCESSING INVENTORY                                    │   │
│  │ ─────────────────────────────────────────────────────────────── │   │
│  │ • Discover all processing activities                            │   │
│  │ • Map data flows, recipients, transfers                         │   │
│  │ • Identify legal bases (Art. 6)                                 │   │
│  │ • Determine AIPD requirement per processing                     │   │
│  │ Output: .osk/comply/rgpd/processing-inventory.yaml              │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                              │                                          │
│                              ▼                                          │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │ PHASE 2: AIPD/DPIA (Conditional)                                 │   │
│  │ ─────────────────────────────────────────────────────────────── │   │
│  │ Required if: CNIL mandatory list OR 2+ CEPD criteria met        │   │
│  │                                                                  │   │
│  │ CNIL PIA Methodology:                                           │   │
│  │   Step 1: Context study (scope, data, processes, supports)      │   │
│  │   Step 2: Fundamental principles evaluation                     │   │
│  │   Step 3: Risk analysis (3 scenarios)                           │   │
│  │   Step 4: Validation (DPO opinion, action plan)                 │   │
│  │                                                                  │   │
│  │ Output: .osk/comply/rgpd/aipd/{processing-name}.yaml            │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                              │                                          │
│                              ▼                                          │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │ PHASE 3: CONTROL ASSESSMENT                                      │   │
│  │ ─────────────────────────────────────────────────────────────── │   │
│  │ • Evaluate Articles 5-50 compliance                             │   │
│  │ • Transfer mechanisms verification (SCCs, BCRs)                 │   │
│  │ • Processor compliance (Art. 28)                                │   │
│  │ • Security measures (CNIL 25 fiches)                            │   │
│  │ Output: .osk/comply/rgpd/control-assessment.yaml                │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                              │                                          │
│                              ▼                                          │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │ PHASE 4: GAP ANALYSIS                                            │   │
│  │ ─────────────────────────────────────────────────────────────── │   │
│  │ • Gap categorization (organizational, technical, legal)         │   │
│  │ • Priority matrix (BLOCKER, QUICK_WIN, HIGH, MEDIUM, LOW)       │   │
│  │ • Remediation roadmap                                           │   │
│  │ • Quick wins identification                                     │   │
│  │ Output: .osk/comply/rgpd/gaps-analysis.yaml                     │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                              │                                          │
│                              ▼                                          │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │ PHASE 5: DOCUMENTATION                                           │   │
│  │ ─────────────────────────────────────────────────────────────── │   │
│  │ Required documents:                                             │   │
│  │   • Registre de traitement (Art. 30)                           │   │
│  │   • Mesures de sécurité (Art. 32)                               │   │
│  │   • Procédure violation (Art. 33-34)                            │   │
│  │   • Procédure droits (Art. 12-22)                               │   │
│  │   • Politique confidentialité (Art. 13-14)                      │   │
│  │ Conditional documents:                                          │   │
│  │   • AIPD report (if Phase 2 completed)                         │   │
│  │   • LIA (if legitimate interest used)                          │   │
│  │   • TIA (if transfers outside EU)                              │   │
│  │ Contracts:                                                      │   │
│  │   • Clauses sous-traitant (Art. 28)                            │   │
│  │ Output: .osk/comply/rgpd/documents/                             │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

## Workflow State Management

The workflow tracks state in `.osk/comply/rgpd/workflow-state.yaml`:

```yaml
workflow:
  version: "5.0.0"
  started_at: "2026-01-22T10:00:00Z"
  current_phase: inventory        # Current active phase
  current_step: null              # Step within phase (for AIPD)

  phases:
    inventory:
      status: completed           # pending | in_progress | completed | skipped
      output: processing-inventory.yaml
      result:
        total_processing: 5
        aipd_required: 2

    aipd:
      status: in_progress
      required: true              # Based on Phase 1 determination
      processing_activities:
        - id: analytics
          status: completed
        - id: profiling
          status: in_progress

    control_assessment:
      status: pending

    gaps_analysis:
      status: pending

    documentation:
      status: pending
```

## Phase Dependencies

```
Phase 1 (Inventory)
    │
    ├──► Phase 2 (AIPD) ─────► if required by Phase 1
    │         │
    │         └── Can be skipped if no high-risk processing
    │
    ▼
Phase 3 (Control Assessment)
    │
    ├──► Uses Phase 1 processing inventory
    │
    ├──► Uses Phase 2 AIPD results (if available)
    │
    ▼
Phase 4 (Gap Analysis)
    │
    ├──► Consolidates Phase 3 assessment results
    │
    ▼
Phase 5 (Documentation)
    │
    ├──► Uses all previous phases
    │
    └──► Generates conditional docs based on Phase 1-4
```

## Commands Reference

### `/osk-comply rgpd` (Default)

Runs the full workflow, resuming from last incomplete phase.

**Behavior:**
1. Check workflow state
2. If no state, start Phase 1
3. If state exists, resume from `current_phase`
4. Progress through phases sequentially

### `/osk-comply rgpd inventory`

**Phase 1: Processing Inventory**

Discovers and documents all processing activities.

**Output:**
- `.osk/comply/rgpd/processing-inventory.yaml`

**Key activities:**
- Data category discovery
- Legal basis determination
- Recipient/transfer mapping
- AIPD trigger detection

See: `prompts/01-inventory.md`

### `/osk-comply rgpd aipd`

**Phase 2: AIPD/DPIA**

Conducts Data Protection Impact Assessment using CNIL methodology.

**Triggers (CNIL mandatory list):**
- Large-scale health data
- Systematic employee monitoring
- HR profiling
- Biometric identification
- Vulnerable persons data
- Location data at scale
- Cross-referencing datasets
- Innovative technology

**Output:**
- `.osk/comply/rgpd/aipd/{processing-name}.yaml`

**CNIL PIA Steps:**
1. Context study
2. Principles evaluation
3. Risk analysis (3 scenarios)
4. Validation

See: `prompts/02-aipd.md`

### `/osk-comply rgpd assess`

**Phase 3: Control Assessment**

Evaluates compliance against RGPD Articles 5-50.

**Output:**
- `.osk/comply/rgpd/control-assessment.yaml`

**Evaluation categories:**
- Principles (Art. 5-11)
- Rights (Art. 12-22)
- Controller/Processor (Art. 24-43)
- Transfers (Art. 44-49)

See: `prompts/03-assess.md`

### `/osk-comply rgpd gaps`

**Phase 4: Gap Analysis**

Analyzes gaps and creates remediation roadmap.

**Output:**
- `.osk/comply/rgpd/gaps-analysis.yaml`

**Gap categories:**
- Organizational (policies, procedures)
- Technical (security measures)
- Legal (contracts, mechanisms)
- Evidence (documentation)

**Priority matrix:**
- P0 BLOCKER: Compliance-blocking issues
- QUICK WIN: High impact, low effort
- P1 HIGH: Address within 30 days
- P2 MEDIUM: Address within 90 days
- P3 LOW: Improvement opportunities

See: `prompts/04-gaps.md`

### `/osk-comply rgpd generate`

**Phase 5: Documentation**

Generates compliance documents from templates.

**Output:**
```
.osk/comply/rgpd/documents/
├── core/
│   ├── registre-traitement.md    # Art. 30
│   ├── mesures-securite.md       # Art. 32
│   ├── aipd-{processing}.md      # Art. 35 (if required)
│   └── lia-{processing}.md       # Art. 6(1)(f) (if needed)
├── contracts/
│   └── clause-{processor}.md     # Art. 28
├── procedures/
│   ├── violation-donnees.md      # Art. 33-34
│   └── droits-personnes.md       # Art. 12-22
└── public/
    └── politique-confidentialite.md  # Art. 13-14
```

See: `prompts/05-generate.md`

### `/osk-comply rgpd status`

Display current workflow state without running assessment.

```
RGPD Compliance Workflow Status
───────────────────────────────
Started: 2026-01-22
Last updated: 2026-01-22 14:30

Phase 1: Processing Inventory  ✓ Completed
         5 processing activities identified
         2 require AIPD

Phase 2: AIPD/DPIA             ⧗ In Progress
         analytics: ✓ completed
         profiling: ⧗ in progress (step 2/4)

Phase 3: Control Assessment    ○ Pending

Phase 4: Gap Analysis          ○ Pending

Phase 5: Documentation         ○ Pending

Resume with: /osk-comply rgpd
```

## Knowledge Base

```
knowledge/
├── core/                           # Essential guides
│   ├── guide-securite.md           # CNIL 25 security fiches
│   ├── guide-sous-traitant.md      # Processor obligations
│   ├── aipd-modeles.md             # CNIL PIA methodology
│   ├── aipd-liste-obligatoire.md   # CNIL mandatory AIPD list
│   ├── interet-legitime.md         # Legitimate interest
│   ├── violations-donnees.md       # Breach notification
│   ├── breach-notification.md      # Breach procedures
│   └── registre-traitements.md     # ROPA guidance
├── reference/                      # Reference documents
│   ├── rgpd-complet.md             # Full RGPD text
│   ├── edpb-droit-acces.md         # Access right guidelines
│   ├── edpb-breach-examples.md     # Breach case studies
│   └── sccs-2021.md                # Standard Contractual Clauses
├── optional/                       # Additional resources
│   ├── guide-dpo.md                # DPO guide
│   └── cookies-guidelines.md       # Cookie compliance
└── french_law_78-17_1978_complete.md  # Loi Informatique et Libertés
```

## Framework Definition

See `framework.yaml` for:
- All RGPD controls (Articles 5-50)
- Control categories and criticality
- Evidence types expected
- Scoring methodology

## Schemas

```
schemas/
├── workflow-state.yaml           # Workflow state tracking
├── processing-inventory.yaml     # Phase 1 output
├── aipd.yaml                     # Phase 2 output (CNIL PIA)
├── control-assessment.yaml       # Phase 3 output
└── gaps-analysis.yaml            # Phase 4 output
```

## Output Directory Structure

```
.osk/comply/rgpd/
├── workflow-state.yaml           # Workflow progress
├── processing-inventory.yaml     # Phase 1
├── aipd/                         # Phase 2
│   ├── analytics.yaml
│   └── profiling.yaml
├── control-assessment.yaml       # Phase 3
├── gaps-analysis.yaml            # Phase 4
└── documents/                    # Phase 5
    ├── core/
    ├── contracts/
    ├── procedures/
    └── public/
```

## Dependencies

- **Discover phase** must be completed first
- System model files in `.osk/system-model/`
- Minimum: `index.yaml` and `data.yaml`

## Related Frameworks

- `rgs` - French government security standard (RGS/EBIOS RM)
- Coming: `nis2`, `iso27001`
