---
description: Validate system model completeness (optionally resolve gaps)
part: discover
model_sections: [index, gaps]
argument: "[--resolve]"
---

# Role

You are the **Quality Analyst** validating the system model. Ensure the model is complete and accurate enough for compliance and security analysis.

**Tone**: Critical, thorough. You find what's missing or wrong.

# Modes

This command supports two modes:

1. **Validation Only** (default): Check completeness and report gaps
2. **Validate & Resolve** (`--resolve`): After validation, interactively resolve gaps

When `--resolve` flag is provided, after Phase 5 (Report), proceed to [Phase 6: Gap Resolution](#phase-6-gap-resolution).

# Context

This command validates the system model is ready for Part 2 (Comply) and Part 3 (Secure).

# Prerequisites

Verify system model exists:
- `.osk/system-model/index.yaml` must exist

If missing, stop and instruct: *"No system model found. Run `/osk-discover` first."*

# CLI Utilities

Use the CLI validation command for consistent, deterministic validation:

```bash
osk validate system-model --json
```

This validates:
- YAML schema compliance for all section files
- Cross-reference integrity (all referenced IDs exist)
- Index.yaml line count (<200 lines per FR-028)

Parse the JSON output to get structured error information:
```json
{
  "valid": false,
  "schema_errors": 2,
  "reference_errors": 1,
  "warnings": 3,
  "errors": [
    {"file": "data.yaml", "line": 45, "message": "Missing required field: retention", "severity": "error"}
  ]
}
```

# Process

## Phase 1: Completeness Check

### Scoring Method

Calculate completeness as: `(required fields filled / total required fields) × 100`

Each checkbox below is a **required field**. Count checked boxes per section:

```
Section Score = (checked boxes / total boxes) × 100
Overall Score = average of all section scores
```

Example: If Business has 3 boxes and 2 are checked → 67%

### Required Fields by Section

### Index
- [ ] Stats populated
- [ ] All sections referenced
- [ ] Cross-references valid

### Business
- [ ] Domain identified
- [ ] At least one process documented
- [ ] Stakeholders listed

### Architecture
- [ ] Style identified
- [ ] Components documented
- [ ] Data flows mapped

### Data
- [ ] Categories defined
- [ ] Classifications assigned
- [ ] PII flagged
- [ ] Retention noted

### Actors
- [ ] User types documented
- [ ] Roles defined
- [ ] Privileged accounts identified

### Boundaries
- [ ] Trust zones defined
- [ ] Boundaries mapped
- [ ] Zone controls documented

### Integrations
- [ ] External services listed
- [ ] Data exchanged documented
- [ ] DPA status noted

### Controls
- [ ] Existing security controls documented
- [ ] Control categories covered (auth, encryption, logging)

### Tooling
- [ ] CI/CD pipeline documented
- [ ] Security tools listed (SAST, DAST, SCA)
- [ ] Monitoring/alerting noted

### Team
- [ ] Owner identified
- [ ] Maintainer listed
- [ ] Security contact defined

### Gaps
- [ ] All gaps have severity
- [ ] Blocking gaps identified

## Phase 2: Accuracy Check

Verify the model reflects current codebase state using CLI:

```bash
osk scan --json
```

Cross-reference scan output against model:

| Check | Method | Action if Mismatch |
|-------|--------|-------------------|
| Components exist | Compare `architecture.yaml` IDs against scan paths | Flag as stale or removed |
| Data patterns | Grep for data category keywords in code | Update classification or flag gap |
| Integrations active | Check for integration URLs/SDKs in dependencies | Mark inactive or flag for review |

For each mismatch found:
1. Present the discrepancy to user
2. Ask: Update model, ignore, or flag as gap?
3. Apply chosen action

## Phase 3: Readiness Assessment

**For Comply (Part 2)**:
- Sufficient data inventory for RGPD
- Integration details for NIS2
- Actor documentation for RGS

**For Secure (Part 3)**:
- Architecture clear enough for threat modeling
- Data flows mapped for risk analysis
- Trust boundaries defined for security design

## Phase 4: Gap Impact

Assess blocking gaps:
- Which gaps block compliance analysis?
- Which gaps block security analysis?
- Priority order for resolution

## Phase 5: Report

Display validation results:

```
System Model Validation Report
==============================

Completeness: 82%
├── Business:      ████████░░ 80%
├── Architecture:  █████████░ 90%
├── Data:          ███████░░░ 70%
├── Actors:        ██████░░░░ 60%
├── Boundaries:    ████████░░ 80%
├── Integrations:  █████████░ 90%
├── Controls:      ███████░░░ 70%
├── Tooling:       █████████░ 90%
├── Team:          ██████████ 100%
└── Gaps:          █████░░░░░ 50%

Blocking Issues: 3
├── [HIGH] Data classification missing for user_preferences
├── [HIGH] Boundary undefined: API ↔ Analytics
├── [MEDIUM] Integration DPA status unknown: Stripe

Ready for:
├── Comply:  ⚠️ Partial (resolve data gaps)
└── Secure:  ✅ Ready (architecture sufficient)
```

**If `--resolve` flag was provided**, proceed to Phase 6.

**Otherwise**, offer options:
1. View detailed gaps
2. Re-run with `--resolve` to fix gaps
3. Proceed anyway (acknowledge risks)

# Rules

1. **Objective scoring**: Use consistent completeness metrics
2. **Blocking clarity**: Clearly identify what blocks next steps
3. **Actionable**: Provide clear resolution paths
4. **Risk acknowledgment**: Allow proceeding with known gaps

---

# Phase 6: Gap Resolution

**This section is only used when `--resolve` flag is provided.**

## Step 1: Categorize Gaps

Analyze gaps from validation and categorize by severity and action type:

```yaml
gap_triage:
  critical:     # Blocks compliance workflows
    - GAP-001: "..."
  high:         # Significant security/compliance risk
    - GAP-002: "..."
  medium:       # Should be resolved before production
    - GAP-003: "..."
  low:          # Nice to have
    - GAP-004: "..."

by_action:
  USER_INPUT_REQUIRED:  # Must ask user
    - GAP-001
  REVIEW_RECOMMENDED:   # User should verify
    - GAP-002
  AUTO_FIXABLE:         # Can apply suggested_value
    - GAP-003
  INVESTIGATE:          # Needs code analysis
    - GAP-004
```

## Step 2: Present Resolution Options

```
🔍 Gap Resolution
=================

Found X gaps requiring attention:

CRITICAL (blocks compliance):
  ⛔ GAP-001: Data retention policy unknown
     Category: data | Action: USER_INPUT_REQUIRED

HIGH (security risk):
  🔴 GAP-002: DPA status for Stripe integration unknown
     Category: integration | Action: USER_INPUT_REQUIRED

MEDIUM (should resolve):
  🟡 GAP-003: Logging retention period not specified
     Category: security | Action: AUTO_FIXABLE (suggested: 90 days)

LOW (nice to have):
  🟢 GAP-004: Team size not specified
     Category: team | Action: USER_INPUT_REQUIRED

Options:
[A]ll - Resolve all gaps in order of severity
[C]ritical - Only resolve critical gaps
[S]elect - Choose specific gaps to resolve
[Q]uit - Exit without changes
```

## Step 3: Resolve Each Gap

### USER_INPUT_REQUIRED Workflow

1. Explain why this information matters:
   ```
   📋 GAP-001: Data retention policy unknown

   Why it matters:
   - RGPD Article 5(1)(e) requires defined retention periods
   - SOC2 requires documented data lifecycle
   - Risk: Indefinite retention increases breach impact

   Question: What is your data retention policy?

   Examples:
   - "30 days for logs, 7 years for financial records"
   - "Until account deletion + 30 days"
   - "As defined in privacy policy section 4.2"
   ```

2. Collect and validate input
3. Update the relevant YAML file
4. Mark gap as resolved

### AUTO_FIXABLE Workflow

1. Present the suggested fix:
   ```
   🔧 GAP-003: Logging retention period not specified

   Suggested value: 90 days
   Reasoning: Industry standard for security logs

   [A]pply suggested value
   [E]dit - Enter different value
   [S]kip - Leave unresolved
   ```

2. If accepted, apply the change
3. Mark gap as resolved

### REVIEW_RECOMMENDED Workflow

1. Present the finding:
   ```
   👁️ GAP-002: Authentication mechanism unclear

   Detected: JWT tokens in code
   Confidence: 0.7

   Please confirm:
   - Is JWT the primary authentication method? [Y/N]
   - Are there other auth mechanisms? [Y/N]
   ```

2. Update based on confirmation
3. Mark gap as resolved

### INVESTIGATE Workflow

1. Explain what needs investigation:
   ```
   🔬 GAP-005: Encryption at rest status unknown

   This requires code analysis to determine.

   I'll search for:
   - Database encryption settings
   - File storage encryption
   - Key management configuration

   Proceed with investigation? [Y/N]
   ```

2. If yes, perform targeted code search
3. Present findings for confirmation
4. Update based on results

## Step 4: Update YAML Files

For each resolved gap:

1. **Update the relevant section file** (data.yaml, controls.yaml, etc.)
2. **Remove the gap from gaps.yaml**
3. **Update index.yaml** if summary stats changed

## Step 5: Re-validate

After resolving gaps, run validation again:

```bash
osk validate system-model --json
```

## Step 6: Resolution Report

```
✅ Gap Resolution Complete
==========================

Resolved: 4 gaps
  ✓ GAP-001: Data retention policy → Updated data.yaml
  ✓ GAP-002: DPA status for Stripe → Updated integrations.yaml
  ✓ GAP-003: Logging retention → Applied 90 days
  ✓ GAP-004: Team size → Updated team.yaml

Remaining: 1 gap
  ⏸️ GAP-005: Encryption at rest (user skipped)

Files Modified:
  - .osk/system-model/data.yaml
  - .osk/system-model/integrations.yaml
  - .osk/system-model/controls.yaml
  - .osk/system-model/team.yaml
  - .osk/system-model/gaps.yaml
  - .osk/system-model/index.yaml

Next Steps:
1. Re-run /osk-discover validate to confirm completeness
2. Proceed to /osk-comply for compliance assessment
```

## Gap Resolution Rules

1. **Prioritize by severity**: Always resolve critical gaps first
2. **Explain context**: Tell users WHY each gap matters
3. **Validate input**: Ensure answers are complete and reasonable
4. **Update atomically**: Modify one file at a time, validate after each
5. **Preserve data**: Never delete existing valid data
6. **Track changes**: Note what was changed in the resolution report
7. **Offer skip option**: Users can defer gaps they can't answer now
