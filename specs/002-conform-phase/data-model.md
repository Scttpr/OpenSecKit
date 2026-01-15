# Data Model: Conformity Phase v4

**Date**: 2026-01-15
**Feature**: 002-conform-phase

## Entity Relationship Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              INPUT (from Discover)                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  .osk/system-model/                                                          │
│  ├── index.yaml ─────────────┐                                              │
│  ├── data.yaml ──────────────┤                                              │
│  ├── integrations.yaml ──────┼──────▶ [System Context]                      │
│  ├── tooling.yaml ───────────┤         (Full assessment scope)              │
│  ├── architecture.yaml ──────┤                                              │
│  ├── actors.yaml ────────────┤                                              │
│  └── security.yaml ──────────┘                                              │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
                                       │
                                       ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                           FRAMEWORK DEFINITIONS                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  domaines/<framework>/framework.yaml                                         │
│  ┌─────────────────────┐                                                    │
│  │   FrameworkDefinition│                                                   │
│  ├─────────────────────┤                                                    │
│  │ framework_id        │───────────┐                                        │
│  │ name                │           │                                        │
│  │ version             │           │                                        │
│  │ authority           │           │                                        │
│  └─────────────────────┘           │                                        │
│           │                        │                                        │
│           │ 1:N                    │                                        │
│           ▼                        │                                        │
│  ┌─────────────────────┐           │                                        │
│  │     Category        │           │                                        │
│  ├─────────────────────┤           │                                        │
│  │ id                  │◀──────────┼─────────┐                              │
│  │ name                │           │         │                              │
│  │ articles/annexes    │           │         │                              │
│  └─────────────────────┘           │         │                              │
│           │                        │         │                              │
│           │ 1:N                    │         │                              │
│           ▼                        │         │                              │
│  ┌─────────────────────┐           │         │                              │
│  │      Control        │           │         │                              │
│  ├─────────────────────┤           │         │                              │
│  │ id ─────────────────┼───────────┘         │                              │
│  │ name                │                     │                              │
│  │ description         │                     │                              │
│  │ category ───────────┼─────────────────────┘                              │
│  │ criticality         │  (must|should|may)                                 │
│  │ evidence_types[]    │  (maps to system model)                            │
│  │ applies_when        │  (conditional)                                     │
│  │ rgs_levels[]        │  (RGS-specific)                                    │
│  └─────────────────────┘                                                    │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
                                       │
                                       ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                          OUTPUT (Compliance Assessment)                     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  .osk/compliance/assessment-<framework>.yaml                                 │
│                                                                              │
│  ┌─────────────────────┐      ┌─────────────────────┐                       │
│  │ ComplianceAssessment│      │   AssessmentScope   │                       │
│  ├─────────────────────┤      ├─────────────────────┤                       │
│  │ framework_id        │      │ components_included │                       │
│  │ assessment_date     │◀────▶│ components_excluded │                       │
│  │ overall_score       │      │ exclusion_justif.   │                       │
│  │ status              │      │ data_included       │                       │
│  │ system_model_hash   │      │ integrations_incl.  │                       │
│  └─────────────────────┘      │ tooling_included    │                       │
│           │                   └─────────────────────┘                       │
│           │ 1:N                                                              │
│           ▼                                                                  │
│  ┌─────────────────────┐      ┌─────────────────────┐                       │
│  │  ControlMapping     │      │      Evidence       │                       │
│  ├─────────────────────┤      ├─────────────────────┤                       │
│  │ id                  │      │ id                  │                       │
│  │ framework_ref       │      │ type (auto|manual)  │                       │
│  │ status (compliant   │      │ description         │                       │
│  │   |partial|gap|n/a) │◀────▶│ path                │                       │
│  │ score               │      │ collected_date      │                       │
│  │ assessed_date       │      │ confidence          │                       │
│  │ evidence[]          │      └─────────────────────┘                       │
│  │ gap_detail          │                                                    │
│  │ gap_severity        │                                                    │
│  └─────────────────────┘                                                    │
│           │                                                                  │
│           │ 1:N (if gap)                                                    │
│           ▼                                                                  │
│  ┌─────────────────────┐                                                    │
│  │    ActionItem       │                                                    │
│  ├─────────────────────┤                                                    │
│  │ id                  │                                                    │
│  │ title               │                                                    │
│  │ priority (P0-P3)    │                                                    │
│  │ category            │                                                    │
│  │ addresses_controls[]│                                                    │
│  │ owner               │                                                    │
│  │ due_date            │                                                    │
│  │ status              │                                                    │
│  └─────────────────────┘                                                    │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Entity Definitions

### 1. FrameworkDefinition

**Source**: `domaines/<framework>/framework.yaml`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| framework_id | string | Yes | Unique identifier (e.g., "rgpd", "rgs") |
| name | string | Yes | Full framework name |
| version | string | Yes | Framework version (e.g., "2016/679") |
| authority | string | Yes | Issuing authority (e.g., "EU", "ANSSI") |
| effective_date | date | No | When framework came into effect |
| scope | string | No | What the framework covers |

**Validation Rules**:
- `framework_id` must be lowercase, alphanumeric, hyphens allowed
- `framework_id` must be unique across all frameworks
- `version` should follow the official versioning of the regulation

---

### 2. Category

**Source**: `domaines/<framework>/framework.yaml` → `categories[]`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| id | string | Yes | Category identifier |
| name | string | Yes | Display name |
| articles | array[string] | No | Article numbers (GDPR) |
| annexe | string | No | Annexe reference (RGS) |

**Validation Rules**:
- `id` must be unique within framework
- All controls must reference a valid category id

**GDPR Categories**:
- `principles` - Data Processing Principles (Art. 5-11)
- `rights` - Data Subject Rights (Art. 12-23)
- `controller_processor` - Obligations (Art. 24-43)
- `transfers` - International Transfers (Art. 44-50)

**RGS Categories**:
- `authentification` - Annexe B2
- `integrite` - Annexe B3
- `confidentialite` - Annexe B4
- `tracabilite` - Annexe B5
- `horodatage` - Annexe B6
- `signature` - Annexe B7

---

### 3. Control

**Source**: `domaines/<framework>/framework.yaml` → `controls[]`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| id | string | Yes | Unique control ID (prefixed with framework) |
| article | string | No | Article reference (GDPR) |
| annexe | string | No | Annexe reference (RGS) |
| name | string | Yes | Short control name |
| description | string | Yes | Full requirement text |
| category | string | Yes | Reference to category.id |
| criticality | enum | Yes | `must` \| `should` \| `may` |
| evidence_types | array[string] | Yes | Types of evidence needed |
| applies_when | string | No | Conditional applicability |
| rgs_levels | array[string] | No | Which RGS levels (RGS only) |

**Validation Rules**:
- `id` must start with framework_id (e.g., "RGPD-5.1.a", "RGS-B2.1")
- `category` must reference existing category.id
- `criticality` determines scoring weight (must=3, should=2, may=1)
- `evidence_types` must be from known vocabulary

**Evidence Type Vocabulary**:
```yaml
# Data evidence
- processing_records
- retention_policy
- data_inventory
- purpose_documentation

# Security evidence
- security_measures
- access_controls
- encryption_evidence
- auth_procedures

# Legal evidence
- privacy_policy
- consent_mechanisms
- dpa_contracts
- legal_basis_documentation

# Operational evidence
- log_samples
- audit_trails
- breach_procedures
```

---

### 4. ComplianceAssessment

**Source**: `.osk/compliance/assessment-<framework>.yaml`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| schema_version | string | Yes | Schema version (4.0.0) |
| framework | string | Yes | Framework ID |
| assessment_date | datetime | Yes | When assessment was run |
| assessor | string | Yes | `/osk-comply <framework>` |
| system_model_hash | string | Yes | SHA256 of system model at assessment |
| overall_score | number | Yes | 0-100 percentage |
| status | enum | Yes | `compliant` \| `partial` \| `non_compliant` \| `not_assessed` |

**Status Determination**:
- `compliant`: score >= 90 AND no critical gaps
- `partial`: score >= 50 OR has critical gaps
- `non_compliant`: score < 50
- `not_assessed`: assessment incomplete

**Validation Rules**:
- `system_model_hash` used to detect changes for `--update`
- `overall_score` calculated from control scores weighted by criticality

---

### 5. AssessmentScope

**Source**: `.osk/compliance/assessment-<framework>.yaml` → `scope`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| description | string | Yes | Scope description |
| components_included | array[string] | Yes | Component IDs from system model |
| components_excluded | array[string] | No | Excluded components |
| exclusion_justification | string | Conditional | Required if exclusions exist |
| data_included | array[string] | Yes | Data category IDs |
| integrations_included | array[string] | Yes | Integration IDs |
| tooling_included | array[string] | Yes | Tooling IDs |
| actors_included | array[string] | Yes | Actor IDs |

**Validation Rules**:
- Every exclusion requires justification
- All IDs must reference valid system model entries

---

### 6. ControlMapping

**Source**: `.osk/compliance/assessment-<framework>.yaml` → `mappings[]`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| id | string | Yes | Internal tracking ID |
| framework_ref | string | Yes | Article/Control ID |
| requirement | string | Yes | Short requirement name |
| status | enum | Yes | `compliant` \| `partial` \| `gap` \| `not_applicable` \| `not_assessed` |
| score | number | Yes | 0-100 for this control |
| assessed_date | datetime | Yes | When this control was assessed |
| assessed_by | enum | Yes | `auto` \| `manual` \| auditor name |
| evidence | array[Evidence] | No | Evidence items |
| gap_detail | string | Conditional | Required if status is gap |
| gap_severity | enum | Conditional | `critical` \| `high` \| `medium` \| `low` |
| remediation | object | Conditional | Remediation plan if gap |

**Validation Rules**:
- `gap_detail` and `gap_severity` required when `status` is `gap` or `partial`
- `evidence` should have at least one item when `status` is `compliant`

---

### 7. Evidence

**Source**: `.osk/compliance/assessment-<framework>.yaml` → `mappings[].evidence[]`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| id | string | Yes | Evidence ID |
| type | enum | Yes | `auto-detected` \| `config` \| `document` \| `manual` |
| description | string | Yes | What this evidence demonstrates |
| path | string | Yes | Path to evidence (system model path or file) |
| collected_date | datetime | Yes | When evidence was collected |
| confidence | enum | No | `high` \| `medium` \| `low` (for auto-detected) |

**Validation Rules**:
- `path` should be valid system model path (e.g., `data.yaml#processing_activities`)
- `confidence` only applies to `auto-detected` type

---

### 8. ActionItem

**Source**: `.osk/compliance/assessment-<framework>.yaml` → `action_items[]`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| id | string | Yes | Action ID |
| title | string | Yes | Action title |
| description | string | Yes | Detailed description |
| priority | enum | Yes | `P0` \| `P1` \| `P2` \| `P3` |
| category | enum | Yes | `codebase` \| `tooling` \| `infrastructure` \| `process` \| `documentation` |
| addresses_controls | array[string] | Yes | Control IDs this addresses |
| owner | string | No | Team or person responsible |
| due_date | date | No | Target completion date |
| status | enum | Yes | `open` \| `in_progress` \| `blocked` \| `completed` \| `deferred` |

**Priority Guide**:
- `P0`: Critical gap, immediate action required
- `P1`: High priority, address within 30 days
- `P2`: Medium priority, address within 90 days
- `P3`: Low priority, address within 180 days

---

## State Transitions

### Assessment Status

```
                    ┌─────────────┐
                    │ not_assessed│
                    └──────┬──────┘
                           │ Start assessment
                           ▼
                    ┌─────────────┐
                    │  in_progress│ (partial file saved)
                    └──────┬──────┘
                           │ Complete all controls
                           ▼
               ┌───────────┴───────────┐
               │                       │
               ▼                       ▼
        ┌─────────────┐         ┌─────────────┐
        │  compliant  │         │   partial   │
        │  (score≥90) │         │ (50≤score<90)│
        └─────────────┘         └─────────────┘
               │                       │
               │                       ▼
               │                ┌─────────────┐
               │                │non_compliant│
               │                │  (score<50) │
               │                └─────────────┘
               │                       │
               └───────────┬───────────┘
                           │ --update
                           ▼
                    (re-assess changed controls)
```

### Control Status

```
┌─────────────┐
│not_assessed │ (initial state)
└──────┬──────┘
       │ Agent presents control
       ▼
┌──────┴───────────────────────────────────────┐
│                                               │
▼               ▼               ▼               ▼
┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────────┐
│ compliant│  │  partial │  │   gap    │  │not_applicable│
│(evidence │  │(some     │  │(no       │  │(justified    │
│ confirms)│  │ evidence)│  │ evidence)│  │ exclusion)   │
└──────────┘  └──────────┘  └──────────┘  └──────────────┘
```

---

## Evidence Mapping to System Model

### GDPR Evidence Sources

| Control | Primary Source | Fallback Source |
|---------|---------------|-----------------|
| Art. 5 (Principles) | data.yaml, security.yaml | business.yaml |
| Art. 13-14 (Information) | business.yaml#privacy | - |
| Art. 28 (Processors) | integrations.yaml, tooling.yaml | - |
| Art. 30 (Records) | data.yaml#processing_activities | - |
| Art. 32 (Security) | security.yaml | architecture.yaml |
| Art. 44-49 (Transfers) | integrations.yaml#location, tooling.yaml#location | architecture.yaml#hosting |

### RGS Evidence Sources

| Domain | Primary Source | Fallback Source |
|--------|---------------|-----------------|
| B2 (Auth) | security.yaml#authentication | tooling.yaml |
| B3 (Integrity) | security.yaml#encryption | - |
| B4 (Confidentiality) | security.yaml#encryption, architecture.yaml | - |
| B5 (Traceability) | tooling.yaml#logging, security.yaml | - |
| B6 (Timestamping) | integrations.yaml#tsa | - |
| B7 (Signature) | integrations.yaml#signature, security.yaml | - |
