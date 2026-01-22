---
description: Run autonomous compliance assessment (RGPD or RGS)
part: comply
phase: compliance
version: "5.0.0"
---

# /osk-comply <framework>

> Autonomous multi-phase compliance assessment workflow

## Usage

```
/osk-comply rgpd    # RGPD/GDPR compliance assessment
/osk-comply rgs     # RGS compliance assessment (French public sector)
```

## Available Frameworks

| Framework | Description |
|-----------|-------------|
| `rgpd` | RGPD/GDPR - EU data protection regulation |
| `rgs` | RGS - French government security standard |

## How It Works

Each framework runs an autonomous 5-phase workflow:

1. **Discovery** - Analyze system model for relevant data/controls
2. **Assessment** - Evaluate against framework requirements
3. **Risk Analysis** - Identify gaps and risks
4. **Remediation** - Generate remediation roadmap
5. **Documentation** - Produce compliance artifacts

The workflow maintains state in `.osk/comply/<framework>/workflow-state.yaml` and can be resumed at any point.

## Prerequisites

- System model must exist: `.osk/system-model/index.yaml`
- Run `/osk-discover` first if not present

## Output

```
.osk/comply/<framework>/
├── workflow-state.yaml    # Progress tracking
├── *.yaml                 # Assessment data
└── documents/             # Generated compliance documents
```

## Options

- `--update` - Re-run assessment with updated system model
- `--export <md|pdf>` - Export compliance documentation

---

**Please specify a framework:** `/osk-comply rgpd` or `/osk-comply rgs`
