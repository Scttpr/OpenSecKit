---
description: RGS (French Government Security) compliance - Main orchestrator prompt
part: comply
framework: rgs
version: "4.1.0"
model_sections: [index, architecture, controls, data, integrations, tooling, actors, business, boundaries]
sub_prompts:
  - prompts/assess.md
---

# RGS Compliance Framework

This framework provides interactive RGS (Référentiel Général de Sécurité) compliance assessment for French government information systems.

## Quick Start

```bash
# Full workflow
/osk-comply rgs              # Interactive assessment

# With flags
/osk-comply rgs --update     # Re-assess changed controls only
/osk-comply rgs --resume     # Continue interrupted assessment
/osk-comply rgs --export md  # Generate homologation dossier
```

## Workflow Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    RGS COMPLIANCE WORKFLOW                   │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐   │
│  │   DISCOVER   │───▶│    ASSESS    │───▶│    EXPORT    │   │
│  │              │    │              │    │              │   │
│  │ system-model │    │ compliance   │    │ homologation │   │
│  │ *.yaml       │    │ evaluation   │    │ dossier      │   │
│  └──────────────┘    └──────────────┘    └──────────────┘   │
│                             │                    │           │
│                             ▼                    ▼           │
│                    assessment-rgs.yaml    dossier.md         │
│                    homologation.md        perimeter.md       │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Commands

### `/osk-comply rgs` (Default)

Runs interactive RGS compliance assessment:
1. **Level Selection** - Choose RGS*, RGS**, or RGS***
2. **Scope Definition** - Define full system perimeter including tooling
3. **Domain Assessment** - Evaluate 22 controls across 6 domains
4. **Homologation Check** - Identify blockers for certification

**Output:**
- `.osk/compliance/assessment-rgs.yaml`
- `.osk/compliance/assessment-rgs.md`
- `.osk/compliance/homologation-checklist.md`
- `.osk/compliance/system-perimeter.md`

See: `prompts/assess.md`

### `/osk-comply rgs status`

Display current compliance status without running assessment.

```
RGS Compliance Status
──────────────────────
Target Level: RGS**
Last assessment: 2026-01-15
Score: 72% (requires 85%)
Homologation: NOT READY
Blockers: 2 critical
```

## RGS Levels

| Level | Threshold | Use Case |
|-------|-----------|----------|
| RGS* | 70% | Basic government services |
| RGS** | 85% | Sensitive data, SecNumCloud recommended |
| RGS*** | 95% | Critical infrastructure (OIV), SecNumCloud mandatory |

## Control Domains

| Domain | Controls | Focus |
|--------|----------|-------|
| AUTH | 5 | Authentication mechanisms |
| INT | 4 | Data integrity |
| CONF | 4 | Confidentiality, encryption |
| TRAC | 5 | Traceability, logging |
| HORO | 2 | Timestamping |
| SIG | 2 | Electronic signatures |

## Templates

| Template | Purpose |
|----------|---------|
| `assessment-summary.md.tera` | Human-readable assessment report |
| `homologation-checklist.md.tera` | Pre-certification checklist |
| `system-perimeter.md.tera` | Full system boundary definition |
| `export-dossier.md.tera` | ANSSI-compliant homologation dossier |

## Knowledge Base

```
knowledge/
├── rgs-v2-annexe-a1-certificats-electroniques.md
├── rgs-v2-annexe-b1-mecanismes-cryptographiques.md
├── rgs-v2-annexe-b2-gestion-cles.md
├── rgs-v2-annexe-b3-authentification.md
├── guide-homologation-securite.md
├── guide-hygiene-informatique.md
└── ebios-risk-manager.md
```

## Framework Definition

See `framework.yaml` for:
- All 22 RGS controls with requirements per level
- DICP security model mapping
- Certification requirements (SecNumCloud, HDS)
- ANSSI cryptographic standards
- Cross-framework mapping (ISO 27001, RGPD)

## Interactive Features

The assessment prompt is **interactive**:

1. **Level selection** - User chooses target RGS level
2. **Scope confirmation** - User validates system perimeter
3. **Evidence review** - Auto-detected evidence presented for confirmation
4. **Gap-filling** - Missing information requested with clear options
5. **Certification check** - Tool certifications validated per level
6. **Homologation status** - Clear blockers identification

## Output Schema

Assessment output follows the schema at:
- `phases/comply/frameworks/_schema/assessment.yaml` (base)
- `phases/comply/frameworks/rgs/schemas/assessment-extension.yaml` (RGS-specific)

## Examples

### Standard assessment
```bash
/osk-comply rgs
```

### Update after changes
```bash
/osk-comply rgs --update
```

### Resume interrupted assessment
```bash
/osk-comply rgs --resume
```

### Generate homologation dossier
```bash
/osk-comply rgs --export md
```

## Dependencies

- **Discover phase** must be completed first
- System model files in `.osk/system-model/`
- Minimum: `index.yaml`, `architecture.yaml`, `controls.yaml`

## Related Frameworks

- `rgpd` - GDPR data protection compliance
- Coming: `nis2`, `iso27001`

## Reference Documentation

- `WORKFLOW-AND-SCORING.md` - Scoring algorithm and DICP calculation
- `CRYPTO-REFERENCE.md` - ANSSI-approved cryptographic algorithms
- `SYSTEM-MODEL-REQUIREMENTS.md` - Required system model data by control
