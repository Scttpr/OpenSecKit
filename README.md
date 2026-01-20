<p align="center">
  <h1 align="center">OpenSecKit</h1>
  <p align="center">
    <strong>Security as Code, Multi-Agent Ready</strong>
  </p>
  <p align="center">
    Version 4.0.0 | Multi-agent security framework (Claude Code, Copilot, Cursor, Gemini)
  </p>
</p>

<p align="center">
  <a href="https://scttpr.github.io/OpenSecKit">Documentation</a> •
  <a href="#installation">Installation</a> •
  <a href="#quickstart">Quickstart</a> •
  <a href="#the-7-principles">Principles</a>
</p>

---

## Why OpenSecKit?

- **7 constitutional principles** for comprehensive security
- **Multi-Agent**: Claude Code, GitHub Copilot, Cursor, Gemini + universal `AGENTS.md`
- **Three-phase workflow**: Discover → Secure → Comply
- **Documentation as Code** generated automatically
- **RGPD/RGS compliance** built-in

## Installation

```bash
# Install the CLI
cargo install osk

# Initialize a project (auto-detects available agents)
cd my-project/
osk init

# Or specify agents directly
osk init --agent copilot
osk init --agent cursor
osk init --all-agents
```

## Quickstart

```bash
# Start Claude Code
claude

# 1. Build the system model
>>> /osk-discover

# 2. Secure a feature
>>> /osk-secure-specify "authentication"
>>> /osk-secure-plan "authentication"
>>> /osk-secure-tasks "authentication"
>>> /osk-secure-implement "authentication"

# 3. Compliance assessment
>>> /osk-comply-rgpd
>>> /osk-comply-status
```

## Workflow

```
osk init → /osk-discover → System Model
                ↓
    ┌───────────┴───────────┐
    ↓                       ↓
/osk-secure-*          /osk-comply-*
    ↓                       ↓
Security Specs         Compliance Reports
```

## Phase 1: Discover

Build a structured system model before security analysis.

```bash
# Build or update system model (adaptive)
>>> /osk-discover

# Validate the model
>>> /osk-discover-validate

# Generate documentation
>>> /osk-discover-docs --audience security
```

### System Model Structure

```
.osk/system-model/
├── index.yaml        # Summary + cross-references
├── business.yaml     # Domain, processes, journeys
├── architecture.yaml # Components, data flows
├── data.yaml         # Data categories, PII
├── actors.yaml       # Users, roles
├── boundaries.yaml   # Trust zones and boundaries
├── integrations.yaml # External services
├── controls.yaml     # Security controls
├── tooling.yaml      # Security tools
├── team.yaml         # Team structure
└── gaps.yaml         # Identified gaps
```

## Phase 2: Secure

Shift-left security with the 7 principles.

```bash
# Full security specification (threats, requirements, testing, hardening)
>>> /osk-secure-specify "feature_name"

# Clarify ambiguities
>>> /osk-secure-clarify "feature_name"

# Generate implementation plan
>>> /osk-secure-plan "feature_name"

# Generate ordered tasks
>>> /osk-secure-tasks "feature_name"

# Implement with risk register updates
>>> /osk-secure-implement "feature_name"
```

### Security Outputs

```
.osk/specs/001-authentication/
├── security-spec.md   # Full security specification
├── risks.yaml         # Risk register
├── security-plan.md   # Implementation plan
├── security-tasks.md  # Ordered tasks
└── tasks.yaml         # Machine-readable tasks
```

## Phase 3: Comply

Compliance assessment based on the system model.

```bash
# RGPD/GDPR assessment
>>> /osk-comply-rgpd

# RGS assessment (French public sector)
>>> /osk-comply-rgs

# Multi-framework dashboard
>>> /osk-comply-status

# List available frameworks
>>> /osk-comply-list
```

### Compliance Outputs

```
.osk/compliance/
├── assessment-rgpd.yaml       # RGPD assessment data
├── assessment-rgpd.md         # Summary report
├── gap-report-rgpd.md         # Gap analysis
├── sub-processor-register.md  # Art. 28 register
├── assessment-rgs.yaml        # RGS assessment data
├── assessment-rgs.md          # Summary report
├── homologation-checklist.md  # Homologation checklist
└── system-perimeter.md        # RGS system perimeter
```

## The 7 Principles

| # | Principle | Description |
|---|-----------|-------------|
| I | Threat Modeling | Proactive threat analysis (STRIDE) |
| II | Risk Analysis | Risk scoring and prioritization |
| III | Security by Design | Security requirements from conception |
| IV | Security Testing | Automated SAST/DAST/SCA tests |
| V | Secrets Management | No secrets in code |
| VI | Audit Logging | Immutable, centralized logs |
| VII | Patch Management | Strict update SLAs |

## CLI Utilities

```bash
osk scan --json           # Scan files (respects .gitignore)
osk id src/api/users.rs   # Generate component ID
osk changes --json        # List modified files
osk validate system-model # Validate system model
```

## Commands Reference

### Discover
| Command | Description |
|---------|-------------|
| `/osk-discover` | Build or update system model |
| `/osk-discover-validate` | Validate model (optionally resolve gaps) |
| `/osk-discover-docs` | Generate documentation |

### Secure
| Command | Description |
|---------|-------------|
| `/osk-secure-specify` | Security specification (threats, requirements, testing) |
| `/osk-secure-clarify` | Clarify ambiguities and validate assumptions |
| `/osk-secure-plan` | Consolidated implementation plan |
| `/osk-secure-tasks` | Generate ordered tasks with dependencies |
| `/osk-secure-implement` | Implement tasks and update risk register |

### Comply
| Command | Description |
|---------|-------------|
| `/osk-comply-rgpd` | RGPD/GDPR compliance assessment |
| `/osk-comply-rgs` | RGS compliance with homologation |
| `/osk-comply-status` | Multi-framework dashboard |
| `/osk-comply-list` | List available frameworks |

## V3 Compatibility

V3 commands are aliased to their V4 equivalents:

| V3 Command | V4 Equivalent |
|------------|---------------|
| `/osk-configure` | `/osk-discover` |
| `/osk-analyze` | `/osk-secure-specify` |
| `/osk-specify` | `/osk-secure-specify` |
| `/osk-harden` | `/osk-secure-specify` |
| `/osk-plan` | `/osk-secure-plan` |
| `/osk-tasks` | `/osk-secure-tasks` |
| `/osk-implement` | `/osk-secure-implement` |
| `/osk-rgpd` | `/osk-comply-rgpd` |
| `/osk-rgs` | `/osk-comply-rgs` |

## Documentation

[Full documentation](https://scttpr.github.io/OpenSecKit)

## Contributing

See [CONTRIBUTING](docs/development/contributing.md)

## License

MIT
