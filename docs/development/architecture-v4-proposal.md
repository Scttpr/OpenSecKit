# OpenSecKit V4 Architecture Proposal

## Overview

This document proposes a restructured architecture for OpenSecKit V4, organizing the framework into three distinct parts with a **hub-and-spokes** model.

```
                    ┌─────────────────────────────┐
                    │      PART 1: DISCOVER       │
                    │      (Reality Model Hub)    │
                    │                             │
                    │  system-model.yaml          │
                    │  - Business context         │
                    │  - Architecture             │
                    │  - Data inventory           │
                    │  - Actors & permissions     │
                    │  - Trust boundaries         │
                    │  - External integrations    │
                    └──────────────┬──────────────┘
                                   │
                    ┌──────────────┴──────────────┐
                    │                             │
           ┌────────▼────────┐          ┌────────▼────────┐
           │   PART 2:       │          │   PART 3:       │
           │   COMPLY        │          │   SECURE        │
           │                 │          │                 │
           │ Consumes model  │          │ Consumes model  │
           │ Maps to:        │          │ Extends with:   │
           │ - RGPD          │          │ - Threat specs  │
           │ - RGS           │          │ - Requirements  │
           │ - NIS2          │          │ - Test strategy │
           │ - ISO 27001     │          │ - Risk register │
           │ - SOC 2         │          │                 │
           │ - ...           │          │                 │
           └─────────────────┘          └─────────────────┘
```

## Design Principles

1. **Hub and Spokes**: Part 1 (Discover) is the central "reality model" that Parts 2 and 3 consume independently
2. **Full Context**: Part 1 captures business logic, architecture, data classification, actor mapping, and trust boundaries
3. **Framework Agnostic Compliance**: Part 2 can map to ANY compliance framework
4. **SpecKit-Style Workflow**: Part 3 follows a structured workflow inspired by SpecKit
5. **Agent Agnostic**: All prompts work across multiple AI agents (Claude Code, Copilot, Cursor, Gemini)
6. **CLI + Agent Prompts**: CLI handles scaffolding/discovery, agents execute security workflows

---

## Multi-Agent Architecture

V4 maintains the data-driven multi-agent architecture from V3.

### Execution Model

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              CLI (Rust)                                     │
│                                                                             │
│  • osk init          → Scaffolds project, detects agents                    │
│  • osk discover init → Runs discovery, generates system-model.yaml          │
│  • osk scaffold      → Generates agent-specific prompt files                │
│  • osk update        → Syncs prompts from registry                          │
│  • osk validate      → Validates schemas and references                     │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         Agent Prompt Files                                  │
│                                                                             │
│  Claude Code    → .claude/commands/osk-*.md                                 │
│  Copilot        → .github/copilot-instructions.md                           │
│  Cursor         → .cursor/rules/osk-*.mdc                                   │
│  Gemini         → .gemini/settings.json + instructions                      │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         AI Agent Execution                                  │
│                                                                             │
│  User invokes: /osk-discover init, /osk-comply rgpd, /osk-secure specify    │
│  Agent reads prompts, executes workflow, generates artifacts                │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Agent Configuration (agents.toml)

```toml
[meta]
version = "4.0.0"
default_agent = "claude-code"

[agents.claude-code]
name = "Claude Code"
format = "slash-command"           # One file per command
output_dir = ".claude/commands"
file_pattern = "osk-{command}.md"
template = "claude-code.tera"
enabled = true
capabilities = ["file-read", "file-write", "bash", "search"]

[agents.copilot]
name = "GitHub Copilot"
format = "single-file"             # All commands in one file
output_file = ".github/copilot-instructions.md"
template = "copilot.tera"
enabled = true
capabilities = ["file-read", "file-write"]

[agents.cursor]
name = "Cursor"
format = "rules-dir"               # One rule per command
output_dir = ".cursor/rules"
file_pattern = "osk-{command}.mdc"
template = "cursor.tera"
enabled = true
capabilities = ["file-read", "file-write", "bash"]

[agents.gemini]
name = "Google Gemini"
format = "single-file"
output_file = ".gemini/instructions.md"
template = "gemini.tera"
enabled = true
capabilities = ["file-read", "file-write"]

[universal]
enabled = true
output_file = "AGENTS.md"
template = "AGENTS.md.tera"
description = "Universal agent instructions for any AI assistant"
```

### Prompt Structure for V4

Each prompt in `prompts/` is agent-agnostic. The Tera templates in `templates/agents/` transform them for each agent.

```markdown
# prompts/osk-secure-specify.md

---
description: Security specification for a feature
argument: feature_name
part: 3
requires:
  - "system-model.yaml"
generates:
  - ".osk/specs/{feature}/threats.md"
  - ".osk/specs/{feature}/requirements.md"

# System model sections to load (keeps context small)
model_sections:
  - index           # Always loaded (~200 lines)
  - architecture    # Only components relevant to feature
  - data            # Only data categories for feature
---

# Role

You are the **Security Specification Engineer**...

# Process

## Phase 1: Load Context
1. Read `.osk/system-model/index.yaml` (always)
2. Read relevant sections based on feature
...
```

### Context Budget Management

| Section | Typical Size | When to Load |
|---------|--------------|--------------|
| `index.yaml` | ~200 lines | **Always** |
| `architecture.yaml` | ~500-2000 lines | Component analysis |
| `data.yaml` | ~300-1500 lines | Data-related features, RGPD |
| `actors.yaml` | ~200-800 lines | Auth features |
| `boundaries.yaml` | ~100-400 lines | Network/boundary analysis |
| `integrations.yaml` | ~200-600 lines | Third-party features, RGPD |
| `controls.yaml` | ~200-500 lines | Hardening, gap analysis |
| `gaps.yaml` | ~100-300 lines | Planning, remediation |

### Agent-Specific Adaptations

| Aspect | Claude Code | Copilot | Cursor | Gemini |
|--------|-------------|---------|--------|--------|
| **Invocation** | `/osk-secure specify auth` | Natural language | `/osk-secure specify auth` | Natural language |
| **File access** | Direct | Via suggestions | Direct | Via suggestions |
| **Bash execution** | Yes | No | Yes | Limited |
| **Multi-step** | Yes (agentic) | Limited | Yes | Limited |

For agents with limited capabilities (Copilot, Gemini), prompts include manual steps:
```markdown
# For agents without bash access:
> **Manual step**: Run `npm audit` and paste the output below.
```

## Current vs V4 Mapping

| Current (V3) | Part | V4 Structure |
|--------------|------|--------------|
| `/osk-configure` | 1 | `/osk-discover init` |
| `/osk-baseline` | 1 | `/osk-discover scan` |
| - | 1 | `/osk-discover update` |
| - | 1 | `/osk-discover validate` |
| `/osk-rgpd` | 2 | `/osk-comply rgpd` |
| `/osk-rgs` | 2 | `/osk-comply rgs` |
| - | 2 | `/osk-comply <framework>` |
| - | 2 | `/osk-comply gap-analysis` |
| `/osk-analyze` | 3 | `/osk-secure constitute` + `/osk-secure specify` |
| `/osk-specify` | 3 | `/osk-secure specify` |
| `/osk-harden` | 3 | `/osk-secure specify` |
| `/osk-plan` | 3 | `/osk-secure plan` |
| `/osk-tasks` | 3 | `/osk-secure tasks` |
| `/osk-implement` | 3 | `/osk-secure implement` |

---

# Part 1: DISCOVER

## Purpose

Creates and maintains `system-model.yaml` - the comprehensive "what exists" documentation that feeds both compliance and shift-left workflows.

## Commands

```
/osk-discover
├── /osk-discover init        # First-time full analysis
├── /osk-discover scan        # Deep scan of specific aspect
│   ├── --architecture        # Components, services, infra
│   ├── --data                # Data inventory & classification
│   ├── --actors              # Users, roles, permissions
│   ├── --business            # Business processes & flows
│   └── --integrations        # External dependencies
├── /osk-discover update      # Incremental update (detect drift)
└── /osk-discover validate    # Verify model matches reality
```

## `/osk-discover init` Workflow

### Phase 1: Codebase Analysis

- Detect stack (Node, Python, Go, Rust...)
- Identify entry points (routes, handlers, main)
- Map file structure → components
- Extract dependencies → integrations

### Phase 2: Architecture Extraction

- Identify services/modules
- Detect data stores (DB, cache, queues)
- Map service-to-service calls
- Identify external APIs
- Generate component diagram (Mermaid)

### Phase 3: Data Discovery

- Scan models/schemas (Prisma, SQLAlchemy, etc.)
- Identify PII fields (email, phone, address...)
- Classify sensitivity (personal/sensitive/confidential)
- Map data-to-storage locations
- Detect data flows between components
- Identify retention patterns

### Phase 4: Actor Mapping

- Identify user types (from auth code)
- Extract roles/permissions
- Map actor-to-data access
- Identify privileged accounts

### Phase 5: Trust Boundaries

- Identify network boundaries (from infra code)
- Map authentication gates
- Identify encryption boundaries
- Generate trust boundary diagram

### Phase 6: Business Context (AI-assisted)

- Infer business processes from code paths
- Map user journeys from routes/flows
- Identify critical business operations
- **[ASK USER]** Validate/complete business context

## Output: `system-model/` (Split Architecture)

To avoid context window issues with large codebases, the system model is split into separate files:

```
.osk/system-model/
├── index.yaml              # Metadata + references to sections
├── business.yaml           # Business context (processes, journeys)
├── architecture.yaml       # Components, data flows, infrastructure
├── data.yaml               # Data inventory, classifications, flows
├── actors.yaml             # Users, roles, permissions
├── boundaries.yaml              # Trust zones and boundaries
├── integrations.yaml       # External dependencies
├── controls.yaml           # Detected security controls
└── gaps.yaml               # Discovery gaps requiring attention
```

### Loading Strategy

Each prompt specifies which sections it needs:

```yaml
# In prompt frontmatter
requires_model_sections:
  - index         # Always loaded (small, ~100 lines)
  - data          # Only if analyzing data
  - architecture  # Only if analyzing components
```

**Context budget per command:**

| Command | Sections Loaded | Estimated Size |
|---------|-----------------|----------------|
| `/osk-discover init` | All (writes) | N/A |
| `/osk-comply rgpd` | index, data, integrations | ~2K lines |
| `/osk-secure constitute` | index, architecture, data, trust | ~3K lines |
| `/osk-secure specify` | index + relevant component | ~1K lines |

### Index File

The `index.yaml` is always loaded and provides:
- Metadata and version
- Quick stats (component count, data count, etc.)
- Section summaries (one-line per item)
- Cross-references

```yaml
# .osk/system-model/index.yaml

metadata:
  version: "1.0"
  created_at: "[DATE ISO]"
  last_scan: "[DATE ISO]"
  confidence: 0.85  # How much was auto-detected vs inferred

# ═══════════════════════════════════════════════════════════
# SECTION 1: BUSINESS CONTEXT
# ═══════════════════════════════════════════════════════════
business:
  domain: "[e-commerce|healthcare|finance|...]"
  description: "[Business description]"

  processes:
    - id: "BP-XXX"
      name: "[Process name]"
      description: "[What this process does]"
      trigger: "[What initiates this process]"
      actors: ["[actor-id]", "..."]
      data_created: ["[DATA-ID]", "..."]
      data_read: ["[DATA-ID]", "..."]
      criticality: "[low|medium|high|critical]"

  user_journeys:
    - id: "UJ-XXX"
      name: "[Journey name]"
      steps: ["BP-XXX", "..."]

# ═══════════════════════════════════════════════════════════
# SECTION 2: ARCHITECTURE
# ═══════════════════════════════════════════════════════════
architecture:
  style: "[monolith|microservices|serverless|hybrid]"

  components:
    - id: "COMP-XXX"
      name: "[Component name]"
      type: "[service|datastore|queue|cache|gateway]"
      technology: "[Node.js|Python|PostgreSQL|...]"
      source_paths: ["src/..."]
      ports: [3000]
      trust_zone: "[external|dmz|internal|data]"

  data_flows:
    - id: "DF-XXX"
      from: "COMP-XXX"
      to: "COMP-YYY"
      protocol: "[HTTP/TLS|gRPC|PostgreSQL|...]"
      data: ["DATA-XXX"]
      crosses_trust_boundary: true|false

# ═══════════════════════════════════════════════════════════
# SECTION 3: DATA INVENTORY
# ═══════════════════════════════════════════════════════════
data:
  inventory:
    - id: "DATA-XXX"
      name: "[Data category name]"
      classification: "[public|internal|personal|sensitive|confidential]"
      fields:
        - name: "[field_name]"
          type: "[string|number|object|...]"
          pii: true|false
          sensitivity: "[public|personal|sensitive|confidential]"
      storage: ["COMP-XXX"]
      source_files: ["src/models/..."]
      retention: "[duration or policy]"
      legal_basis: "[contract|consent|legal_obligation|...]"  # For RGPD
      pci_scope: true|false  # For PCI-DSS

  flows:
    - data: "DATA-XXX"
      path: ["COMP-XXX", "COMP-YYY", "..."]
      encrypted_in_transit: true|false
      encrypted_at_rest: true|false

# ═══════════════════════════════════════════════════════════
# SECTION 4: ACTORS
# ═══════════════════════════════════════════════════════════
actors:
  - id: "[actor-id]"
    type: "[external|internal|system]"
    description: "[Actor description]"
    authentication: "[none|password|sso|api-key|...]"
    mfa: true|false
    can_access: ["DATA-XXX:own|all", "..."]
    privileged: true|false

  roles:
    - id: "ROLE-XXX"
      permissions: ["read:resource", "write:resource", "..."]

# ═══════════════════════════════════════════════════════════
# SECTION 5: TRUST BOUNDARIES
# ═══════════════════════════════════════════════════════════
trust_boundaries:
  zones:
    - id: "ZONE-XXX"
      name: "[Zone name]"
      trust_level: 0-3  # 0=untrusted, 3=highly trusted
      components: ["COMP-XXX", "..."]

  boundaries:
    - id: "TB-XXX"
      name: "[Boundary name]"
      from_zone: "ZONE-XXX"
      to_zone: "ZONE-YYY"
      controls: ["WAF", "auth-required", "TLS", "..."]

# ═══════════════════════════════════════════════════════════
# SECTION 6: INTEGRATIONS (External Dependencies)
# ═══════════════════════════════════════════════════════════
integrations:
  - id: "INT-XXX"
    name: "[Service name]"
    type: "[payment-processor|email-service|auth-provider|...]"
    location: "[Country]"
    data_shared: ["DATA-XXX", "..."]
    transfer_mechanism: "[DPF|SCCs|adequacy|...]"  # For RGPD
    dpa_signed: true|false
    security_certifications: ["SOC2", "ISO27001", "..."]

# ═══════════════════════════════════════════════════════════
# SECTION 7: DISCOVERY GAPS
# ═══════════════════════════════════════════════════════════
discovery_gaps:
  - type: "[missing_info|security_concern|ambiguity]"
    field: "[path.to.field]"
    location: "[file:line]"  # If applicable
    message: "[Description]"
    severity: "[info|low|medium|high]"
    action: "[USER_INPUT_REQUIRED|REVIEW_RECOMMENDED|AUTO_FIXABLE]"
```

---

# Part 2: COMPLY

## Purpose

Consumes `system-model.yaml` and maps it to compliance frameworks. Performs gap analysis and generates compliance documentation.

## Commands

```
/osk-comply
├── /osk-comply <framework>       # Assess against specific framework
│   ├── rgpd                      # GDPR / RGPD
│   ├── rgs                       # Référentiel Général de Sécurité
│   ├── nis2                      # NIS2 Directive
│   ├── iso27001                  # ISO 27001
│   ├── soc2                      # SOC 2 Type II
│   ├── pci-dss                   # PCI-DSS v4.0
│   └── nist-csf                  # NIST CSF 2.0
├── /osk-comply gap-analysis      # Cross-framework gap analysis
├── /osk-comply evidence          # Collect evidence for controls
└── /osk-comply audit <framework> # Audit mode for existing compliance
```

## Workflow

```
┌─────────────────────────────────────────────────────────────┐
│  /osk-comply <framework>                                    │
│                                                             │
│  INPUT:  system-model.yaml                                  │
│  OUTPUT: compliance-assessment-<framework>.yaml             │
│          docs/security/<framework>/                         │
│                                                             │
│  1. Load system-model.yaml                                  │
│  2. Load framework requirements (from frameworks/)            │
│  3. Map data inventory → framework requirements             │
│  4. Map controls → framework controls                       │
│  5. Identify gaps                                           │
│  6. Generate compliance documentation                       │
│  7. Calculate compliance score                              │
└─────────────────────────────────────────────────────────────┘
```

## Output: `compliance-assessment-<framework>.yaml`

```yaml
# .osk/compliance/compliance-assessment-rgpd.yaml

metadata:
  framework: "rgpd"
  framework_version: "2016/679"
  assessment_date: "[DATE ISO]"
  assessor: "[/osk-comply rgpd]"
  system_model_version: "[version from system-model.yaml]"

summary:
  overall_score: 72  # Percentage
  status: "[compliant|partially_compliant|non_compliant]"
  controls_assessed: 45
  controls_compliant: 32
  controls_partial: 8
  controls_gap: 5

mappings:
  - article: "Art. 30"
    requirement: "Records of processing activities"
    description: "Maintain records of all processing activities"
    data_sources:
      - "system-model.yaml#data.inventory"
      - "system-model.yaml#business.processes"
    status: "[compliant|partial|gap]"
    evidence:
      - type: "[auto-detected|manual|document]"
        path: "[path to evidence]"
        description: "[What this proves]"
    gap_detail: "[If gap, explain what's missing]"
    remediation: "[Suggested fix]"
    priority: "[P0|P1|P2|P3]"

  - article: "Art. 35"
    requirement: "Data Protection Impact Assessment"
    # ... etc

cross_references:
  # How this framework maps to others
  - article: "Art. 32"
    maps_to:
      - framework: "iso27001"
        control: "A.8.24"
      - framework: "soc2"
        control: "CC6.1"

action_items:
  - id: "COMPLY-RGPD-001"
    article: "Art. 35"
    action: "Complete DPIA for payment processing"
    priority: "P1"
    assigned_to: null
    due_date: null
```

---

# Part 3: SECURE

## Purpose

Shift-left security workflow for new features. Follows a SpecKit-inspired structured workflow applied to security.

## Commands

```
/osk-secure
├── /osk-secure constitute <feature>  # Define security principles for feature
├── /osk-secure specify <feature>     # Threats, requirements, test strategy
├── /osk-secure clarify <feature>     # Resolve ambiguities (if needed)
├── /osk-secure plan <feature>        # Consolidated implementation plan
├── /osk-secure tasks <feature>       # Ordered tasks with dependencies
└── /osk-secure implement <feature>   # Execute + update risk-register
```

## Workflow

The workflow mirrors SpecKit but focuses on security:

```
SPECKIT WORKFLOW              →    OSK-SECURE WORKFLOW
────────────────────────────       ────────────────────────────

1. Specification              →    /osk-secure specify
   (Feature spec)                  (Threats + Requirements + Tests)

2. Clarification              →    /osk-secure clarify
   (If needed)                     (Resolve ambiguities)

3. Planning                   →    /osk-secure plan
   (Implementation plan)           (Security implementation plan)

4. Tasks                      →    /osk-secure tasks
   (Task breakdown)                (Ordered security tasks)

5. Implement                  →    /osk-secure implement
   (Execute)                       (Execute + update risk-register)
```

> **Note**: The original proposal included a "constitute" step for per-feature principle weighting.
> This was simplified in the final V4 design - the 7 security principles are applied directly
> by `/osk-secure specify` using context from the system-model.

## Step 1: `/osk-secure specify <feature>`

**Purpose**: Generate comprehensive security specifications including threats, requirements, and test strategy, applying the 7 security principles.

**Input**:
- `system-model.yaml` (from Part 1)
- Feature description/name

**Output**:
- `.osk/specs/NNN-<feature>/threats.md`
- `.osk/specs/NNN-<feature>/requirements.md`
- `.osk/specs/NNN-<feature>/testing.md`
- `.osk/specs/NNN-<feature>/hardening.md`
- `docs/security/risks/risk-register.yaml` (updated)

**Process**:
1. **Threat Analysis** (Principle I)
   - STRIDE analysis for each asset
   - Data Flow Diagram with trust boundaries
   - Attack trees for critical threats

2. **Risk Scoring** (Principle II)
   - Score = Impact × Probability × Exposure (1-5 each)
   - Severity classification (CRITICAL ≥80, IMPORTANT 49-79, etc.)
   - Priority assignment (P0-P4)

3. **Requirements Definition** (Principle III)
   - Security requirements (MUST/SHOULD/MAY per RFC 2119)
   - Map to OWASP ASVS where applicable

4. **Test Strategy** (Principle IV)
   - SAST rules/checks
   - DAST test cases
   - SCA requirements

5. **Hardening Needs** (Principles V, VI, VII)
   - Secrets inventory
   - Logging requirements
   - Patch/dependency requirements

## Step 2: `/osk-secure clarify <feature>` (if needed)

**Purpose**: Resolve ambiguities and validate assumptions with the user.

**Input**: Ambiguities identified in specify phase

**Output**: Updated specifications with resolved questions

**When to use**:
- Requirements are ambiguous
- Multiple valid security approaches exist
- User input needed for risk acceptance decisions

## Step 3: `/osk-secure plan <feature>`

**Purpose**: Consolidate all security work into an implementation plan.

**Input**: All specs (threats, requirements, testing, hardening)

**Output**: `.osk/specs/NNN-<feature>/plan.md`

**Process**:
1. Consolidate all security work items
2. Order by risk priority (P0 → P4)
3. Identify dependencies between items
4. Group by implementation phase
5. Estimate complexity (NOT time)

## Step 4: `/osk-secure tasks <feature>`

**Purpose**: Break the plan into atomic, actionable tasks.

**Input**: `plan.md`

**Output**:
- `.osk/specs/NNN-<feature>/tasks.md` (human-readable)
- `.osk/specs/NNN-<feature>/tasks.yaml` (machine-readable)

**Task Schema**:

```yaml
# tasks.yaml
tasks:
  - id: "TASK-001"
    title: "Implement rate limiting on reset endpoint"
    description: "Add rate limiting to prevent brute force attacks"
    addresses_risks: ["RISK-PWD-RESET-001"]
    principle: "III"
    priority: "P0"
    complexity: "medium"
    dependencies: []
    acceptance_criteria:
      - "Rate limit of 5 requests per minute per IP"
      - "Rate limit of 3 requests per hour per email"
      - "Return 429 with Retry-After header"
    files_to_modify:
      - "src/auth/password-reset.ts"
      - "src/middleware/rate-limit.ts"
```

## Step 5: `/osk-secure implement <feature>`

**Purpose**: Execute tasks and maintain the risk register.

**Input**: `tasks.yaml`

**Output**:
- Code changes
- Updated `docs/security/risks/risk-register.yaml`
- Updated `system-model.yaml` (if architecture changed)

**Process**:
1. Execute tasks in dependency order
2. Update risk status: OPEN → IN_PROGRESS → RESOLVED
3. Record resolution details (commit SHA, PR number, controls implemented)
4. Run verification (SAST/DAST if configured)
5. Update system-model if architecture changed

---

# Data Model Relationships

```
┌─────────────────────────────────────────────────────────────┐
│                    PART 1: DISCOVER                         │
│                                                             │
│  .osk/system-model.yaml                                     │
│  └── Single source of truth for "what exists"               │
└─────────────────────────┬───────────────────────────────────┘
                          │
          ┌───────────────┼───────────────┐
          │               │               │
          ▼               │               ▼
┌─────────────────────┐   │   ┌─────────────────────────────┐
│    PART 2: COMPLY   │   │   │      PART 3: SECURE         │
│                     │   │   │                             │
│ .osk/compliance/    │   │   │ .osk/specs/NNN-feature/     │
│ ├── assessment-     │   │   │ ├── security-spec.md         │
│ │   rgpd.yaml       │   │   │ ├── threats.md              │
│ ├── assessment-     │   │   │ ├── requirements.md         │
│ │   rgs.yaml        │   │   │ ├── testing.md              │
│ └── ...             │   │   │ ├── hardening.md            │
│                     │   │   │ ├── plan.md                 │
│ docs/security/      │   │   │ ├── tasks.md                │
│ ├── rgpd/           │   │   │ └── tasks.yaml              │
│ ├── rgs/            │   │   │                             │
│ └── ...             │   │   │ docs/security/risks/        │
│                     │   │   │ └── risk-register.yaml      │
└─────────────────────┘   │   └─────────────────────────────┘
                          │
                          │
                          ▼
              ┌───────────────────────┐
              │   SHARED SCHEMAS      │
              │                       │
              │ templates/schemas/    │
              │ ├── system-model.yaml │
              │ ├── compliance-       │
              │ │   assessment.yaml   │
              │ ├── risk-entry.yaml   │
              │ ├── task-entry.yaml   │
              │ └── ...               │
              └───────────────────────┘
```

---

# File Structure (V4)

```
.osk/
├── config.toml                    # Project configuration
├── system-model/                  # [NEW] Part 1 output - Reality Model (split)
│   ├── index.yaml                 # Always loaded (~200 lines max)
│   ├── business.yaml              # Business context
│   ├── architecture.yaml          # Components, flows
│   ├── data.yaml                  # Data inventory
│   ├── actors.yaml                # Users, roles
│   ├── boundaries.yaml                 # Trust boundaries
│   ├── integrations.yaml          # External deps
│   ├── controls.yaml              # Detected controls
│   └── gaps.yaml                  # Discovery gaps
├── compliance/                    # [NEW] Part 2 outputs
│   ├── assessment-rgpd.yaml
│   ├── assessment-rgs.yaml
│   └── assessment-iso27001.yaml
├── memory/                        # [KEPT] Context for agents
│   └── context.md
├── specs/                         # [KEPT] Part 3 outputs (per-feature)
│   └── NNN-feature/
│       ├── security-spec.md       # Security specification
│       ├── threats.md
│       ├── requirements.md
│       ├── testing.md
│       ├── hardening.md
│       ├── plan.md
│       ├── tasks.md
│       └── tasks.yaml
└── templates/                     # [KEPT] Local template copies

docs/security/
├── risks/
│   └── risk-register.yaml         # [KEPT] Central risk register
├── rgpd/                          # [KEPT] RGPD documentation
├── rgs/                           # [KEPT] RGS documentation
├── iso27001/                      # [NEW] ISO 27001 documentation
└── dashboard.md                   # [KEPT] Security posture overview
```

---

# Migration Path

## Phase 1: Add Part 1 (Discover)

1. Create `system-model.yaml` schema
2. Implement `/osk-discover init` command
3. Refactor `/osk-configure` to populate system-model
4. Refactor `/osk-baseline` to enhance system-model

## Phase 2: Refactor Part 2 (Comply)

1. Create compliance assessment schema
2. Refactor `/osk-rgpd` to consume system-model
3. Refactor `/osk-rgs` to consume system-model
4. Add new frameworks (ISO 27001, SOC 2, etc.)
5. Implement `/osk-comply gap-analysis`

## Phase 3: Refactor Part 3 (Secure)

1. Add `/osk-secure constitute` (new)
2. Refactor `/osk-analyze` → `/osk-secure specify`
3. Merge `/osk-specify` and `/osk-harden` into `/osk-secure specify`
4. Add `/osk-secure clarify` (new)
5. Keep `/osk-plan` → `/osk-secure plan`
6. Keep `/osk-tasks` → `/osk-secure tasks`
7. Keep `/osk-implement` → `/osk-secure implement`

## Phase 4: Integration

1. Ensure Part 2 and Part 3 both read from system-model
2. Cross-reference compliance requirements in risk-register
3. Update dashboard to show both compliance and risk posture

---

# Open Questions

1. **Namespace**: Should commands be `/osk-discover`, `/osk-comply`, `/osk-secure` or nested under `/osk` (e.g., `/osk discover init`)?

2. **Backward Compatibility**: How to handle projects using V3 structure? Migration command?

3. **system-model.yaml size**: For large codebases, could become unwieldy. Split into multiple files?

4. **Real-time sync**: How to keep system-model in sync as code changes? Git hooks? CI integration?

5. **Control Library**: Should Part 2 include a standardized control library for cross-framework mapping?

---

# References

- [SpecKit](https://github.com/Scttpr/SpecKit) - Inspiration for Part 3 workflow
- [STRIDE](https://docs.microsoft.com/en-us/azure/security/develop/threat-modeling-tool-threats) - Threat modeling methodology
- [OWASP ASVS](https://owasp.org/www-project-application-security-verification-standard/) - Security requirements mapping
- [EBIOS RM](https://www.ssi.gouv.fr/guide/la-methode-ebios-risk-manager-le-guide/) - Risk analysis for RGS
