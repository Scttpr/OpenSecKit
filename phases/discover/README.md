# Phase 1: Discover

System discovery and security context extraction.

## Purpose

The Discover phase automatically analyzes your codebase to build a comprehensive **system model** - a structured representation of your application's architecture, data flows, trust boundaries, and existing security controls.

## Commands

| Command | Description |
|---------|-------------|
| `/osk-discover` | Build or update model (adaptive - detects state) |
| `/osk-discover validate` | Validate model (or `--resolve` to fix gaps) |
| `/osk-discover docs` | Generate architecture documentation |

## Output

The discover phase generates `.osk/system-model/`:

```
.osk/system-model/
├── index.yaml          # System overview and metadata
├── business.yaml       # Business context and objectives
├── architecture.yaml   # Components, services, infrastructure
├── data.yaml           # Data inventory and classification
├── actors.yaml         # Users, systems, external entities
├── boundaries.yaml          # Trust boundaries and zones
├── integrations.yaml   # External integrations and APIs
├── controls.yaml       # Existing security controls
├── gaps.yaml           # Identified security gaps
├── team.yaml           # Team structure and responsibilities
└── tooling.yaml        # Development and security tooling
```

## Workflow

```
┌─────────────────────────────────────────────────────────┐
│                    DISCOVER PHASE                        │
├─────────────────────────────────────────────────────────┤
│                                                         │
│   Codebase  ──►  /osk-discover  ──►  system-model       │
│   (adaptive: full discovery or incremental update)      │
│                                                         │
│   Validation ──► /osk-discover validate ──► gap report  │
│                                                         │
│   Export    ──► /osk-discover docs ──► documentation    │
│                                                         │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
              ┌───────────────────────┐
              │  COMPLY / SECURE      │
              │  (consume system-model)│
              └───────────────────────┘
```

## Structure

```
phases/discover/
├── prompts/            # LLM prompts for discovery commands
│   ├── discover.md     # Main command (adaptive: full/incremental/context)
│   ├── validate.md     # Validation or --resolve for gaps
│   └── docs.md         # Documentation generation
├── templates/
│   ├── data/           # YAML generation templates
│   ├── outputs/        # Markdown documentation templates
│   └── reports/        # Terminal output templates
└── README.md
```

## Integration with Other Phases

- **Comply**: Reads system-model to map components to compliance requirements
- **Secure**: Uses system-model for threat analysis context (trust boundaries, data flows)
