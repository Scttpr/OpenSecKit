# Phase 1: Discover

System discovery and security context extraction.

## Purpose

The Discover phase automatically analyzes your codebase to build a comprehensive **system model** - a structured representation of your application's architecture, data flows, trust boundaries, and existing security controls.

## Commands

| Command | Description |
|---------|-------------|
| `/osk-discover` | Build or update model (adaptive - detects state) |
| `/osk-discover validate` | Validate model (or `--resolve` to fix gaps) |

Documentation is generated automatically in Phase 6 (Synthesis) of the discovery workflow.

## Output

The discover phase generates `.osk/system-model/` (16 YAML files + documentation):

```
.osk/system-model/
├── index.yaml          # System overview and metadata
├── product.yaml        # Product identity, features, KPIs
├── business.yaml       # Business context and objectives
├── glossary.yaml       # Domain terms and acronyms
├── architecture.yaml   # Components, APIs, data flows, ADRs
├── data.yaml           # Data inventory and classification
├── actors.yaml         # Users, roles, service accounts
├── user-journeys.yaml  # Personas and user journeys
├── boundaries.yaml     # Trust zones and boundaries
├── integrations.yaml   # External integrations
├── supply_chain.yaml   # SBOM and dependency policies
├── controls.yaml       # Security controls
├── tooling.yaml        # CI/CD and security tooling
├── team.yaml           # Team structure and contacts
├── operations.yaml     # Environments, alerts, runbooks
├── gaps.yaml           # Identified gaps
└── docs/               # Generated documentation
    ├── product.md      # For Product Managers
    ├── developer.md    # For Developers
    ├── architecture.md # For Architects
    ├── security.md     # For Security Engineers
    ├── operations.md   # For DevOps/SRE
    └── onboarding.md   # For New Team Members
```

## Workflow

```
┌─────────────────────────────────────────────────────────┐
│                    DISCOVER PHASE                        │
├─────────────────────────────────────────────────────────┤
│                                                         │
│   Codebase  ──►  /osk-discover  ──►  system-model       │
│   (adaptive: full discovery or incremental update)      │
│   (Phase 6 generates documentation automatically)       │
│                                                         │
│   Validation ──► /osk-discover validate ──► gap report  │
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
kit/discover/
├── prompts/            # LLM prompts for discovery commands
│   ├── discover.md     # Main orchestrator (adaptive: full/incremental/context)
│   ├── validate.md     # Validation or --resolve for gaps
│   ├── 01-product-context.md   # Phase 1: Product & business context
│   ├── 02-architecture.md      # Phase 2: Architecture & decisions
│   ├── 03-domain-model.md      # Phase 3: Data, actors, boundaries
│   ├── 04-ecosystem.md         # Phase 4: Integrations & supply chain
│   ├── 05-operations.md        # Phase 5: Controls, tooling, operations
│   └── 06-synthesis.md         # Phase 6: Gaps, validation, documentation
├── templates/
│   ├── data/           # YAML generation templates (16 files)
│   └── outputs/        # Markdown documentation templates (6 files)
├── schemas/            # Workflow state schema
└── README.md
```

## Integration with Other Phases

- **Comply**: Reads system-model to map components to compliance requirements
- **Secure**: Uses system-model for threat analysis context (trust boundaries, data flows)
