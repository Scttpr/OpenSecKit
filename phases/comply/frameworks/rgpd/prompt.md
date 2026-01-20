---
description: RGPD/GDPR compliance - Main orchestrator prompt
part: comply
framework: rgpd
version: "4.1.0"
model_sections: [index, data, actors, integrations, tooling, architecture, controls, business, boundaries, team]
sub_prompts:
  - prompts/assess.md
  - prompts/generate.md
---

# RGPD Compliance Framework

This framework provides interactive RGPD (GDPR) compliance assessment and document generation.

## Quick Start

```bash
# Full workflow
/osk-comply rgpd              # Interactive: assess then generate

# Individual phases
/osk-comply rgpd assess       # Compliance assessment only
/osk-comply rgpd generate     # Document generation only
```

## Workflow Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                    RGPD COMPLIANCE WORKFLOW                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐      │
│  │   DISCOVER   │───▶│    ASSESS    │───▶│   GENERATE   │      │
│  │              │    │              │    │              │      │
│  │ system-model │    │ compliance   │    │ documents    │      │
│  │ *.yaml       │    │ evaluation   │    │ *.md         │      │
│  └──────────────┘    └──────────────┘    └──────────────┘      │
│                             │                    │              │
│                             ▼                    ▼              │
│                    assessment-rgpd.yaml    registre.md          │
│                    gaps-rgpd.yaml          aipd.md              │
│                                            politique.md         │
│                                            ...                  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Commands

### `/osk-comply rgpd` (Default)

Runs the full interactive workflow:
1. **Assess** - Evaluate compliance, identify gaps (interactive)
2. **Generate** - Create required documents (interactive)

### `/osk-comply rgpd assess`

Interactive compliance assessment against RGPD controls.

**What it does:**
- Loads system-model from discover phase
- Evaluates each RGPD article (grouped by chapter)
- **Asks for confirmation** at each step
- Identifies gaps and missing information
- Calculates compliance score

**Output:**
- `.osk/compliance/assessment-rgpd.yaml`
- `.osk/compliance/assessment-rgpd.md`
- `.osk/compliance/gaps-rgpd.yaml`

**Flags:**
- `--update` - Re-assess only changed controls
- `--quick` - Skip confirmations for compliant controls
- `--export` - Also generate audit report

See: `prompts/assess.md`

### `/osk-comply rgpd generate`

Interactive document generation from templates.

**What it does:**
- Loads assessment results and system-model
- Determines which documents are required
- **Asks user to select** which documents to generate
- **Fills information gaps** through dialogue
- Generates completed compliance documents

**Output:**
```
.osk/compliance/documents/rgpd/
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

**Flags:**
- `--doc <name>` - Generate specific document only
- `--all` - Generate all without prompting
- `--update` - Re-generate changed documents
- `--format <md|pdf|html>` - Output format

See: `prompts/generate.md`

### `/osk-comply rgpd status`

Display current compliance status without running assessment.

```
RGPD Compliance Status
──────────────────────
Last assessment: 2026-01-15
Score: 78% (Partial Compliance)
Critical gaps: 2
Documents generated: 5/8
```

## Templates

### Core Documents

| Template | Article | Description |
|----------|---------|-------------|
| `registre-traitement.md.tera` | 30 | Record of Processing Activities (ROPA) |
| `mesures-securite.md.tera` | 32 | Security measures (CNIL 25 fiches) |
| `aipd.md.tera` | 35 | Data Protection Impact Assessment |
| `lia.md.tera` | 6(1)(f) | Legitimate Interest Assessment |

### Contracts

| Template | Article | Description |
|----------|---------|-------------|
| `clause-sous-traitant.md.tera` | 28 | Processor contract clauses |

### Procedures

| Template | Article | Description |
|----------|---------|-------------|
| `violation-donnees.md.tera` | 33-34 | Breach notification procedure |
| `droits-personnes.md.tera` | 12-22 | Data subject rights procedure |

### Public

| Template | Article | Description |
|----------|---------|-------------|
| `politique-confidentialite.md.tera` | 13-14 | Privacy policy |

## Knowledge Base

```
knowledge/
├── core/                           # Essential guides
│   ├── guide-securite.md           # CNIL 25 security fiches
│   ├── guide-sous-traitant.md      # Processor obligations
│   ├── aipd-modeles.md             # DPIA methodology
│   ├── aipd-liste-obligatoire.md   # DPIA mandatory list
│   ├── interet-legitime.md         # Legitimate interest
│   ├── violations-donnees.md       # Breach notification
│   ├── breach-notification.md      # Breach procedures
│   └── registre-traitements.md     # ROPA guidance
├── reference/                      # Reference documents
│   ├── rgpd-complet.md             # Full RGPD text
│   ├── edpb-droit-acces.md         # Access right guidelines
│   ├── edpb-breach-examples.md     # Breach case studies
│   ├── adequacy-list.yaml          # Adequacy decisions
│   └── sccs-2021.md                # Standard Contractual Clauses
└── optional/                       # Additional resources
    ├── guide-dpo.md                # DPO guide
    └── cookies-guidelines.md       # Cookie compliance
```

## Framework Definition

See `framework.yaml` for:
- All RGPD controls (Articles 5-50)
- Control categories and criticality
- Evidence types expected
- Scoring methodology
- Cross-framework mapping (ISO 27001, NIS2)

## Interactive Features

Both assess and generate prompts are **interactive**:

1. **Confirmation steps** - User validates auto-detected findings
2. **Gap-filling questions** - Missing information requested with clear options
3. **Review before action** - Preview before generating documents
4. **Progress saving** - Can pause and resume
5. **Explanation of requirements** - Plain language alongside legal references

## Output Schema

Assessment output follows the schema at:
- `phases/comply/frameworks/_schema/assessment.yaml` (base)
- `phases/comply/frameworks/rgpd/schemas/assessment-extension.yaml` (RGPD-specific)

## Examples

### Quick assessment check
```bash
/osk-comply rgpd assess --quick
```

### Generate only the privacy policy
```bash
/osk-comply rgpd generate --doc politique-confidentialite
```

### Full compliance package
```bash
/osk-comply rgpd generate --all
```

### Update after fixing gaps
```bash
/osk-comply rgpd assess --update
/osk-comply rgpd generate --update
```

## Dependencies

- **Discover phase** must be completed first
- System model files in `.osk/system-model/`
- Minimum: `index.yaml` and `data.yaml`

## Related Frameworks

- `rgs` - French government security standard
- Coming: `nis2`, `iso27001`
