# Phase-Based Architecture Proposal

## Overview

Restructure OpenSecKit from **artifact-type organization** to **phase-based organization**, making the 3-part model (Discover → Comply → Secure) the primary navigation structure.

## Proposed Structure

```
OpenSecKit/
├── phases/
│   ├── discover/           # Part 1: System Discovery
│   │   ├── prompts/
│   │   │   ├── init.md
│   │   │   ├── scan.md
│   │   │   ├── update.md
│   │   │   ├── validate.md
│   │   │   ├── context.md
│   │   │   ├── docs.md
│   │   │   └── resolve.md
│   │   ├── templates/
│   │   │   ├── data/           # YAML generation
│   │   │   │   ├── index.yaml.tera
│   │   │   │   ├── business.yaml.tera
│   │   │   │   ├── architecture.yaml.tera
│   │   │   │   ├── data.yaml.tera
│   │   │   │   ├── actors.yaml.tera
│   │   │   │   ├── boundaries.yaml.tera
│   │   │   │   ├── integrations.yaml.tera
│   │   │   │   ├── controls.yaml.tera
│   │   │   │   ├── gaps.yaml.tera
│   │   │   │   ├── team.yaml.tera
│   │   │   │   └── tooling.yaml.tera
│   │   │   ├── outputs/        # (prompt-driven, no templates needed)
│   │   │   └── reports/        # Terminal output
│   │   │       └── summary.tera
│   │   └── README.md           # Phase documentation
│   │
│   ├── comply/             # Part 2: Compliance Assessment
│   │   ├── prompts/
│   │   │   ├── assess.md       # Base compliance prompt
│   │   │   ├── rgpd.md
│   │   │   ├── rgs.md
│   │   │   ├── nis2.md
│   │   │   ├── iso27001.md
│   │   │   ├── soc2.md
│   │   │   ├── list.md
│   │   │   └── status.md
│   │   ├── frameworks/         # Compliance framework definitions
│   │   │   ├── _template/
│   │   │   │   └── framework.yaml.example
│   │   │   ├── _schema/
│   │   │   │   └── framework-schema.yaml
│   │   │   ├── rgpd/
│   │   │   │   ├── framework.yaml
│   │   │   │   ├── skeleton.yaml
│   │   │   │   ├── templates/
│   │   │   │   └── knowledge/  # RGPD-specific guidance
│   │   │   ├── rgs/
│   │   │   │   ├── framework.yaml
│   │   │   │   ├── templates/
│   │   │   │   └── knowledge/  # RGS-specific guidance (ANSSI, etc.)
│   │   │   ├── nis2/
│   │   │   ├── iso27001/
│   │   │   └── soc2/
│   │   ├── templates/
│   │   │   ├── data/
│   │   │   │   └── assessment.yaml.tera
│   │   │   ├── outputs/
│   │   │   │   ├── assessment-report.md.tera
│   │   │   │   └── gap-analysis.md.tera
│   │   │   └── reports/
│   │   │       └── summary.tera
│   │   └── README.md
│   │
│   └── secure/             # Part 3: Security Specification
│       ├── prompts/
│       │   ├── specify.md
│       │   ├── clarify.md
│       │   ├── plan.md
│       │   ├── tasks.md
│       │   └── implement.md
│       ├── templates/
│       │   ├── data/
│       │   │   └── risks.yaml.tera
│       │   ├── outputs/
│       │   │   ├── security-spec.md.tera
│       │   │   └── security-plan.md.tera
│       │   └── reports/
│       │       └── summary.tera
│       ├── knowledge/          # Security methodologies
│       │   ├── methodologies/
│       │   │   ├── threat-modeling/
│       │   │   ├── risk-analysis/
│       │   │   ├── security-requirements/
│       │   │   ├── security-testing/
│       │   │   ├── secrets-management/
│       │   │   ├── audit-logging/
│       │   │   └── patch-management/
│       │   ├── libraries/
│       │   │   ├── threats/
│       │   │   ├── controls/
│       │   │   └── patterns/
│       │   └── examples/
│       │       ├── ecommerce/
│       │       ├── saas/
│       │       └── api/
│       └── README.md
│
├── shared/                 # Cross-phase resources
│   ├── schemas/            # YAML data contracts
│   │   ├── system-model-index.yaml
│   │   ├── system-model-full.yaml
│   │   ├── compliance-assessment.yaml
│   │   ├── security-spec.yaml
│   │   ├── risks.yaml
│   │   └── threat-library-entry.yaml
│   ├── agents/             # Agent configuration templates
│   │   ├── AGENTS.md.tera
│   │   ├── claude-code.tera
│   │   ├── copilot.tera
│   │   ├── cursor.tera
│   │   └── gemini.tera
│   └── legacy/             # V3 templates (deprecation path)
│       ├── context.md.tmpl
│       ├── dashboard.md.tmpl
│       └── ...
│
├── config/                 # Configuration hub
│   ├── registry.toml       # Command registry
│   └── agents.toml         # Agent definitions
│
├── cli/                    # Rust CLI tool
│   └── src/
│
├── docs/                   # Documentation (unchanged)
│   ├── getting-started/
│   ├── commands/
│   ├── development/
│   └── ...
│
├── scripts/                # Build/dev utilities
└── .specify/               # SpecKit integration
```

## Migration Mapping

### Current → Proposed

| Current Location | Proposed Location |
|-----------------|-------------------|
| `prompts/osk-discover-*.md` | `phases/discover/prompts/*.md` |
| `prompts/osk-comply-*.md` | `phases/comply/prompts/*.md` |
| `prompts/osk-secure-*.md` | `phases/secure/prompts/*.md` |
| `frameworks/rgpd/` | `phases/comply/frameworks/rgpd/` |
| `frameworks/rgs/` | `phases/comply/frameworks/rgs/` |
| `templates/data/discover/` | `phases/discover/templates/data/` |
| `templates/data/comply/` | `phases/comply/templates/data/` |
| `templates/outputs/discover/` | `phases/discover/templates/outputs/` |
| `templates/outputs/comply/` | `phases/comply/templates/outputs/` |
| `templates/secure/` | `phases/secure/templates/outputs/` |
| `templates/reports/discovery-*.tera` | `phases/discover/templates/reports/` |
| `templates/reports/compliance-*.tera` | `phases/comply/templates/reports/` |
| `templates/schemas/` | `shared/schemas/` |
| `templates/agents/` | `shared/agents/` |
| `knowledge/methodologies/` | `phases/secure/knowledge/methodologies/` |
| `knowledge/libraries/` | `phases/secure/knowledge/libraries/` |
| `knowledge/examples/` | `phases/secure/knowledge/examples/` |
| `knowledge/frameworks/rgs/` | `phases/comply/frameworks/rgs/knowledge/` |
| `templates/outputs/*.tmpl` | `shared/legacy/` |

## Benefits

### 1. Cohesion
Everything needed for a phase is in one folder:
```bash
# To understand the comply phase:
ls phases/comply/
# → prompts/ frameworks/ templates/ README.md
```

### 2. Frameworks Live Where They Belong
Frameworks are compliance-specific, so they're under `comply/`:
```
phases/comply/frameworks/
├── rgpd/
│   ├── framework.yaml
│   ├── templates/
│   └── knowledge/      # RGPD-specific guidance here
└── rgs/
    ├── framework.yaml
    ├── templates/
    └── knowledge/      # ANSSI crypto, homologation guides here
```

### 3. Knowledge is Contextualized
- Security methodologies → `phases/secure/knowledge/`
- Framework-specific knowledge → `phases/comply/frameworks/{name}/knowledge/`
- No more top-level `knowledge/` that's really only for secure phase

### 4. Cleaner Prompt Names
```
# Current (redundant prefixes)
prompts/osk-discover-init.md
prompts/osk-comply-rgpd.md

# Proposed (context from folder)
phases/discover/prompts/init.md
phases/comply/prompts/rgpd.md
```

### 5. Extensibility
Adding a new phase (e.g., "operate" for runtime security):
```bash
mkdir -p phases/operate/{prompts,templates,knowledge}
```

### 6. Clear Shared vs Phase-Specific
- `shared/` = truly cross-cutting (schemas, agent configs)
- `phases/*/` = phase-specific everything

## Trade-offs

### Cons to Consider

1. **CLI Path Updates**: Registry and CLI code need to reference new paths
2. **Deeper Nesting**: `phases/comply/frameworks/rgpd/templates/` vs `frameworks/rgpd/templates/`
3. **Migration Effort**: One-time restructure + testing
4. **Tooling Updates**: Any scripts referencing old paths need updates

### Mitigations

1. **Symlinks During Transition**: Keep old paths as symlinks temporarily
2. **Registry-Driven**: CLI already uses registry.toml - just update paths there
3. **Single PR**: Do the restructure atomically

## CLI Registry Update

```toml
# config/registry.toml

[paths]
phases = "phases"
shared = "shared"

[phases.discover]
prompts = "phases/discover/prompts"
templates = "phases/discover/templates"

[phases.comply]
prompts = "phases/comply/prompts"
frameworks = "phases/comply/frameworks"
templates = "phases/comply/templates"

[phases.secure]
prompts = "phases/secure/prompts"
templates = "phases/secure/templates"
knowledge = "phases/secure/knowledge"

[shared]
schemas = "shared/schemas"
agents = "shared/agents"
```

## Decision

**Recommendation**: Proceed with phase-based restructure because:
1. The 3-phase model is the core product concept
2. Current structure is already hybrid (phase prefixes everywhere)
3. Frameworks at top-level alongside knowledge is awkward
4. Makes onboarding clearer ("want to understand comply? look at phases/comply/")

**Next Steps**:
1. [ ] Review and approve this proposal
2. [ ] Update CLI to use registry-driven paths
3. [ ] Execute migration (single PR)
4. [ ] Update documentation
5. [ ] Remove legacy symlinks after one release cycle
