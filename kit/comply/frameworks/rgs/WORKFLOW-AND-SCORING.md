# RGS Assessment Workflow and Scoring Algorithm

This document describes the end-to-end workflow for RGS compliance assessment and the scoring algorithms used.

## End-to-End Workflow

### Phase Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        /osk-comply rgs Workflow                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  1. INITIALIZATION                                                          │
│     ├── Load system model from .osk/system-model/                          │
│     ├── Load framework definition from framework.yaml                       │
│     ├── Determine RGS level (user input or compliance_hints)               │
│     └── Initialize assessment context                                       │
│                                                                             │
│  2. SCOPE CONFIRMATION                                                      │
│     ├── Present system perimeter from system model                         │
│     ├── Identify components, integrations, data categories                 │
│     ├── Confirm or adjust scope with user                                  │
│     └── Record exclusions with justifications                              │
│                                                                             │
│  3. CONTROL ASSESSMENT (26 controls)                                       │
│     ├── For each control in framework.yaml:                                │
│     │   ├── Check system model for relevant data                           │
│     │   ├── Auto-detect evidence from codebase                             │
│     │   ├── Validate against knowledge base requirements                   │
│     │   ├── Prompt user for missing information                            │
│     │   └── Assign status and score                                        │
│     └── Generate control mappings                                          │
│                                                                             │
│  4. CRYPTOGRAPHIC VALIDATION                                               │
│     ├── Scan codebase for crypto patterns                                  │
│     ├── Validate against ANSSI requirements (CRYPTO-REFERENCE.md)          │
│     ├── Check TLS configuration                                            │
│     └── Flag non-compliant algorithms                                      │
│                                                                             │
│  5. TOOL CERTIFICATION CHECK                                               │
│     ├── Extract tools from tooling.yaml and integrations.yaml              │
│     ├── Check certifications against RGS level requirements                │
│     ├── Identify blockers (SecNumCloud for RGS***)                         │
│     └── Generate certification report                                      │
│                                                                             │
│  6. DICP ASSESSMENT                                                        │
│     ├── Evaluate each criterion (D, I, C, P)                               │
│     ├── Calculate achieved levels from controls                            │
│     ├── Compare against required levels for RGS level                      │
│     └── Generate DICP status                                               │
│                                                                             │
│  7. GAP ANALYSIS                                                           │
│     ├── Identify all non-compliant and partial controls                    │
│     ├── Categorize by source (codebase, tooling, infra, process)          │
│     ├── Prioritize by severity and RGS level requirements                  │
│     └── Generate action items                                              │
│                                                                             │
│  8. HOMOLOGATION READINESS                                                 │
│     ├── Check for blocking issues                                          │
│     ├── Verify documentation requirements                                  │
│     ├── Assess EBIOS RM completion                                         │
│     └── Generate homologation checklist                                    │
│                                                                             │
│  9. OUTPUT GENERATION                                                       │
│     ├── Render assessment.yaml from template                               │
│     ├── Render compliance-summary.tera to terminal                         │
│     ├── Generate gap-report.md                                             │
│     └── Store assessment in .osk/comply/rgs/                               │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Detailed Phase Descriptions

#### Phase 1: Initialization

```yaml
# Load order
1. system_model = load(".osk/system-model/index.yaml")
2. framework = load("kit/comply/frameworks/rgs/framework.yaml")
3. knowledge = load_all("kit/comply/frameworks/rgs/knowledge/*.md")
4. schemas = load_all("kit/comply/frameworks/rgs/schemas/*.yaml")

# Determine RGS level
rgs_level =
  user_input ||
  system_model.compliance_hints.rgs.level ||
  prompt_user("Select RGS level: *, **, or ***")
```

#### Phase 2: Scope Confirmation

The scope is derived from the system model and confirmed with the user:

```yaml
scope:
  components: system_model.architecture.components
  integrations: system_model.integrations
  data_categories: system_model.data.inventory
  infrastructure: system_model.architecture.infrastructure

# User confirms or adjusts
# Exclusions must have justifications
exclusions:
  - id: "COMP-LEGACY"
    justification: "Out of scope - scheduled for deprecation Q2"
```

#### Phase 3: Control Assessment

Each of the 26 controls is assessed:

```
For each control in framework.controls:
  1. Locate relevant system model data (see SYSTEM-MODEL-REQUIREMENTS.md)
  2. Search codebase for evidence patterns
  3. Validate against knowledge base requirements
  4. If data missing: prompt user or mark not_assessed
  5. Calculate control score
  6. Record evidence and assessment details
```

#### Phase 4: Cryptographic Validation

Uses patterns from `CRYPTO-REFERENCE.md`:

```
1. Grep codebase for compliant_patterns
2. Grep codebase for non_compliant_patterns
3. Check TLS configuration files
4. Validate certificate configuration
5. Generate crypto_validation results
```

#### Phase 5: Tool Certification Check

Uses `schemas/tool-certifications.yaml`:

```
For each tool in (tooling + integrations):
  1. Identify tool category
  2. Look up RGS level requirements for category
  3. Check claimed certifications against requirements
  4. Flag blockers if missing required certs
  5. Generate remediation suggestions
```

#### Phase 6: DICP Assessment

See [DICP Scoring Algorithm](#dicp-scoring-algorithm) below.

#### Phase 7: Gap Analysis

```yaml
gaps_by_severity:
  critical: []  # Blockers for homologation
  high: []      # Must fix for target RGS level
  medium: []    # Should fix
  low: []       # Nice to have

gaps_by_source:
  codebase: []       # Code changes needed
  tooling: []        # Tool configuration/replacement
  infrastructure: [] # Infra changes
  process: []        # Policy/procedure gaps
```

#### Phase 8: Homologation Readiness

```yaml
homologation_ready:
  blockers.length == 0 &&
  overall_score >= threshold_for_level &&
  dicp_all_criteria_met &&
  ebios_rm_completed

blockers:
  - type: "control"
    id: "RGS-CONF.1"
    reason: "Encryption at rest not implemented"
  - type: "certification"
    tool: "AWS"
    reason: "SecNumCloud required for RGS***"
```

#### Phase 9: Output Generation

```
1. Collect all assessment data into context
2. Render assessment.yaml.tera → .osk/comply/rgs/assessment.yaml
3. Render gap-report.md.tera → .osk/comply/rgs/gap-report.md
4. Render compliance-summary.tera → terminal output
5. Update audit_trail with new entry
```

---

## Scoring Algorithms

### Overall Score Calculation

```python
def calculate_overall_score(mappings, rgs_level):
    """
    Calculate overall compliance score as weighted percentage.

    Args:
        mappings: List of control assessment results
        rgs_level: "rgs-star", "rgs-star-star", or "rgs-star-star-star"

    Returns:
        score: 0-100 percentage
    """
    # Get controls applicable to this RGS level
    applicable_controls = [m for m in mappings
                          if m.status != "not_applicable"
                          and is_required_for_level(m.id, rgs_level)]

    if not applicable_controls:
        return 0

    total_weight = 0
    weighted_score = 0

    for control in applicable_controls:
        weight = get_control_weight(control.id, rgs_level)
        total_weight += weight
        weighted_score += control.score * weight

    return round(weighted_score / total_weight)
```

### Control Weight Matrix

| Domain | Base Weight | `RGS*` Modifier | `RGS**` Modifier | `RGS***` Modifier |
|--------|-------------|-----------------|------------------|-------------------|
| AUTH (Authentification) | 1.0 | 1.0 | 1.2 | 1.5 |
| INT (Intégrité) | 1.0 | 0.8 | 1.0 | 1.2 |
| CONF (Confidentialité) | 1.0 | 1.0 | 1.3 | 1.5 |
| TRAC (Traçabilité) | 1.0 | 0.8 | 1.0 | 1.3 |
| HORO (Horodatage) | 0.5 | 0.5 | 0.8 | 1.0 |
| SIG (Signature) | 0.5 | 0.5 | 0.8 | 1.0 |

### Individual Control Scoring

```python
def calculate_control_score(control_id, evidence, system_model):
    """
    Calculate score for a single control (0-100).

    Scoring tiers:
    - 100: Fully compliant with all requirements
    - 75-99: Compliant with minor gaps
    - 50-74: Partially compliant
    - 25-49: Major gaps
    - 0-24: Non-compliant
    """
    requirements = get_requirements(control_id)

    met_requirements = 0
    total_requirements = len(requirements)

    for req in requirements:
        status = evaluate_requirement(req, evidence, system_model)
        if status == "met":
            met_requirements += 1
        elif status == "partial":
            met_requirements += 0.5

    base_score = (met_requirements / total_requirements) * 100

    # Apply confidence modifier
    confidence_avg = average([e.confidence for e in evidence])
    confidence_modifier = {
        "high": 1.0,
        "medium": 0.95,
        "low": 0.85
    }[confidence_avg]

    return round(base_score * confidence_modifier)
```

### Status Determination

```python
def determine_status(score):
    """Map score to status label."""
    if score >= 90:
        return "compliant"
    elif score >= 50:
        return "partial"
    elif score > 0:
        return "gap"
    else:
        return "not_assessed"

def determine_overall_status(overall_score, rgs_level, has_blockers):
    """Determine overall assessment status."""
    if has_blockers:
        return "non_compliant"

    thresholds = {
        "rgs-star": 70,
        "rgs-star-star": 85,
        "rgs-star-star-star": 95
    }

    if overall_score >= thresholds[rgs_level]:
        return "compliant"
    elif overall_score >= thresholds[rgs_level] - 15:
        return "partial"
    else:
        return "non_compliant"
```

---

## DICP Scoring Algorithm

DICP (Disponibilité, Intégrité, Confidentialité, Preuve) scoring maps control assessments to the four security criteria.

### Control-to-DICP Mapping

```yaml
dicp_control_mapping:
  disponibilite:  # Availability
    primary_controls:
      - "RGS-AUTH.1"  # Authentication availability
      - "RGS-AUTH.5"  # Account lifecycle
      - "RGS-INT.4"  # Backup integrity
    contributing_controls:
      - "RGS-TRAC.1"  # Logging (for incident response)

  integrite:  # Integrity
    primary_controls:
      - "RGS-INT.1"  # Data integrity mechanisms
      - "RGS-INT.2"  # Signature verification
      - "RGS-INT.3"  # Change detection
      - "RGS-INT.4"  # Backup integrity
    contributing_controls:
      - "RGS-CONF.3"  # Key management
      - "RGS-SIG.1"  # Digital signatures

  confidentialite:  # Confidentiality
    primary_controls:
      - "RGS-CONF.1"  # Encryption at rest
      - "RGS-CONF.2"  # Encryption in transit
      - "RGS-CONF.3"  # Key management
      - "RGS-CONF.4"  # Data classification
    contributing_controls:
      - "RGS-AUTH.1"  # Authentication
      - "RGS-AUTH.4"  # Privileged accounts

  preuve:  # Proof/Traceability
    primary_controls:
      - "RGS-TRAC.1"  # Logging enabled
      - "RGS-TRAC.2"  # Events logged
      - "RGS-TRAC.3"  # Log retention
      - "RGS-TRAC.4"  # Log protection
      - "RGS-HORO.1"  # Timestamping
    contributing_controls:
      - "RGS-SIG.1"  # Digital signatures
      - "RGS-SIG.2"  # Qualified signatures
```

### DICP Level Calculation

```python
def calculate_dicp_level(criterion, control_scores):
    """
    Calculate achieved DICP level (0-4) for a criterion.

    Levels:
    0 - Not addressed
    1 - Basic (score >= 25%)
    2 - Standard (score >= 50%)
    3 - Enhanced (score >= 75%)
    4 - Advanced (score >= 90%)
    """
    mapping = dicp_control_mapping[criterion]

    # Primary controls have 70% weight, contributing have 30%
    primary_scores = [control_scores[c] for c in mapping.primary_controls
                      if c in control_scores]
    contributing_scores = [control_scores[c] for c in mapping.contributing_controls
                           if c in control_scores]

    if not primary_scores:
        return 0

    primary_avg = sum(primary_scores) / len(primary_scores)
    contributing_avg = sum(contributing_scores) / len(contributing_scores) if contributing_scores else 0

    weighted_score = (primary_avg * 0.7) + (contributing_avg * 0.3)

    # Map to level
    if weighted_score >= 90:
        return 4
    elif weighted_score >= 75:
        return 3
    elif weighted_score >= 50:
        return 2
    elif weighted_score >= 25:
        return 1
    else:
        return 0
```

### Required DICP Levels by RGS Level

```yaml
required_dicp_levels:
  rgs-star:
    disponibilite: 1
    integrite: 2
    confidentialite: 2
    preuve: 1

  rgs-star-star:
    disponibilite: 2
    integrite: 3
    confidentialite: 3
    preuve: 2

  rgs-star-star-star:
    disponibilite: 3
    integrite: 4
    confidentialite: 4
    preuve: 3
```

### DICP Status Determination

```python
def get_dicp_status(achieved_level, required_level):
    """Determine status for a DICP criterion."""
    if achieved_level >= required_level:
        return "compliant"
    elif achieved_level >= required_level - 1:
        return "partial"
    else:
        return "non_compliant"

def is_dicp_compliant(achieved_levels, rgs_level):
    """Check if all DICP criteria are met for the RGS level."""
    required = required_dicp_levels[rgs_level]

    for criterion in ["disponibilite", "integrite", "confidentialite", "preuve"]:
        if achieved_levels[criterion] < required[criterion]:
            return False
    return True
```

---

## Gap Severity Classification

```yaml
severity_rules:
  critical:
    - condition: "is_blocker_for_homologation"
    - condition: "control.score == 0 AND control.required_for_level"
    - condition: "non_compliant_crypto_in_use"
    - condition: "missing_required_certification"

  high:
    - condition: "control.score < 50 AND control.required_for_level"
    - condition: "dicp_criterion_not_met"

  medium:
    - condition: "control.score < 75 AND control.required_for_level"
    - condition: "partial_crypto_compliance"

  low:
    - condition: "control.score < 90"
    - condition: "missing_recommended_certification"
```

---

## Action Item Prioritization

```python
def prioritize_action(gap, rgs_level):
    """
    Assign priority P0-P3 to action items.

    P0: Critical blockers - must fix immediately
    P1: High priority - required for target level
    P2: Medium priority - significant improvement
    P3: Low priority - nice to have
    """
    if gap.severity == "critical":
        return "P0"
    elif gap.severity == "high":
        return "P1"
    elif gap.severity == "medium":
        # Bump to P1 if deadline approaching
        if homologation_deadline_within_30_days:
            return "P1"
        return "P2"
    else:
        return "P3"
```

---

## Assessment Storage

```
.osk/comply/rgs/
├── assessment.yaml          # Full structured assessment
├── assessment-history/      # Previous assessments
│   ├── assessment-2025-01-15.yaml
│   └── assessment-2025-01-10.yaml
├── gap-report.md            # Human-readable gap analysis
├── evidence/                # Collected evidence files
│   ├── crypto-scan.json
│   └── tool-certs.json
└── homologation/            # Homologation dossier (when ready)
    ├── dossier.md
    └── annexes/
```

---

## Cross-Framework Synergies

When multiple frameworks are assessed, shared controls reduce effort:

```yaml
synergy_mapping:
  "RGS-CONF.1":  # Encryption at rest
    rgpd: ["RGPD-32.1"]  # Security of processing
    iso27001: ["A.10.1.1"]

  "RGS-TRAC.1":  # Logging
    rgpd: ["RGPD-30"]  # Records of processing
    iso27001: ["A.12.4.1"]
```

When a control is assessed for one framework, the result can be reused:

```python
def check_synergy(control_id, previous_assessments):
    """Check if control was already assessed in another framework."""
    for assessment in previous_assessments:
        if control_id in synergy_mapping:
            mapped = synergy_mapping[control_id]
            for framework, controls in mapped.items():
                if framework == assessment.framework:
                    for mapped_control in controls:
                        if mapped_control in assessment.mappings:
                            return assessment.mappings[mapped_control]
    return None
```
