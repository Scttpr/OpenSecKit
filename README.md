<p align="center">
  <h1 align="center">OpenSecKit</h1>
  <p align="center">
    <strong>Security as Code for AI Coding Assistants</strong>
  </p>
</p>

<p align="center">
  <a href="#installation">Installation</a> •
  <a href="#quickstart">Quickstart</a> •
  <a href="#commands">Commands</a> •
  <a href="#the-7-principles">Principles</a>
</p>

---

## What is OpenSecKit?

OpenSecKit brings structured security workflows to AI coding assistants. Instead of ad-hoc security advice, it provides:

- **System Model** — Understand your codebase before securing it
- **Shift-Left Security** — Threat modeling, risk analysis, security specs
- **Compliance Automation** — RGPD/GDPR and RGS assessments with full audit trail

Works with **Claude Code**, **GitHub Copilot**, **Cursor**, and **Gemini**.

## Installation

```bash
# Install CLI
cargo install osk

# Initialize in your project
cd my-project
osk init
```

The CLI auto-detects available AI agents and generates the appropriate configuration files.

## Quickstart

```bash
# Start your AI assistant (e.g., Claude Code)
claude

# 1. Build the system model
/osk-discover

# 2. Run compliance assessment
/osk-comply rgpd

# 3. Secure a feature
/osk-secure-specify "authentication"
```

## Workflow

```
                    ┌─────────────────┐
                    │  /osk-discover  │
                    │  System Model   │
                    └────────┬────────┘
                             │
              ┌──────────────┴──────────────┐
              ▼                              ▼
    ┌─────────────────┐            ┌─────────────────┐
    │  /osk-secure-*  │            │  /osk-comply    │
    │  Security Specs │            │  Compliance     │
    └─────────────────┘            └─────────────────┘
```

## Commands

### Discover

Build a system model from your codebase.

| Command | Description |
|---------|-------------|
| `/osk-discover` | Analyze codebase, build system model, generate docs |
| `/osk-discover-validate` | Validate model integrity |

**Output:** `.osk/system-model/*.yaml` + `docs/*.md`

### Comply

Autonomous compliance assessment workflows.

| Command | Description |
|---------|-------------|
| `/osk-comply rgpd` | RGPD/GDPR assessment (5-phase workflow) |
| `/osk-comply rgs` | RGS assessment with homologation dossier |
| `/osk-comply-status` | Multi-framework dashboard |
| `/osk-comply-list` | List available frameworks |

**Output:** `.osk/comply/<framework>/`

### Secure

Shift-left security for features.

| Command | Description |
|---------|-------------|
| `/osk-secure-specify <feature>` | Threats, requirements, test cases |
| `/osk-secure-clarify <feature>` | Clarify ambiguities via Q&A |
| `/osk-secure-plan <feature>` | Implementation plan |
| `/osk-secure-tasks <feature>` | Ordered tasks with dependencies |
| `/osk-secure-implement <feature>` | Execute tasks, update risk register |

**Output:** `.osk/specs/<feature>/`

## The 7 Principles

| # | Principle | Focus |
|---|-----------|-------|
| I | Threat Modeling | STRIDE analysis, attack trees |
| II | Risk Analysis | Scoring, prioritization |
| III | Security by Design | Requirements from day one |
| IV | Security Testing | SAST, DAST, SCA |
| V | Secrets Management | No secrets in code |
| VI | Audit Logging | Immutable, centralized |
| VII | Patch Management | Strict SLAs |

## Project Structure

```
.osk/
├── system-model/          # Hub: System documentation
│   ├── index.yaml         # Start here (~200 lines)
│   ├── architecture.yaml
│   ├── data.yaml
│   └── ...
├── comply/                # Compliance outputs
│   ├── rgpd/
│   └── rgs/
└── specs/                 # Security specifications
    └── <feature>/
```

## CLI Utilities

```bash
osk init                    # Initialize project
osk init --agent copilot    # Specific agent
osk init --all-agents       # All agents
osk scan --json             # Scan files
osk validate system-model   # Validate model
```

## Contributing

See [CONTRIBUTING.md](.github/CONTRIBUTING.md)

## License

MIT
