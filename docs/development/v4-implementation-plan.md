# OpenSecKit V4 Implementation Plan

## Overview

This document provides the implementation roadmap for OpenSecKit V4, transitioning from the current mixed architecture to the three-part hub-and-spokes model.

**Target Version**: 4.0.0
**Reference**: [architecture-v4-proposal.md](./architecture-v4-proposal.md)

---

## Implementation Phases

```
Phase 1: Foundation          Phase 2: Part 1 (Discover)     Phase 3: Part 2 (Comply)
──────────────────           ────────────────────────       ────────────────────────
Schema finalization          CLI commands                   Refactor existing domains
Registry updates             Prompt creation                New framework support
Breaking change prep         Integration                    Cross-framework mapping

Phase 4: Part 3 (Secure)     Phase 5: Integration           Phase 6: Migration & Docs
────────────────────────     ──────────────────             ────────────────────────
SpecKit-style workflow       Connect all parts              V3 → V4 migration
Security specification       Dashboard updates              Documentation
Refactor existing            Testing & validation           Release
```

---

## Phase 1: Foundation

### Objective
Prepare the codebase for V4 changes without breaking V3 functionality.

### Tasks

#### 1.1 Schema Finalization

```yaml
task: Finalize system-model.yaml schema
status: done
files:
  - templates/schemas/system-model.yaml
acceptance_criteria:
  - Schema covers all 7 sections (business, architecture, data, actors, trust, integrations, controls)
  - Schema is valid YAML
  - Field descriptions are clear and complete
```

```yaml
task: Finalize compliance-assessment.yaml schema
status: done
files:
  - templates/schemas/compliance-assessment.yaml
acceptance_criteria:
  - Schema supports multiple frameworks
  - Cross-framework mapping structure defined
  - Evidence collection structure defined
```

```yaml
task: Finalize security-spec.yaml schema
status: done
files:
  - templates/schemas/security-spec.yaml
acceptance_criteria:
  - Threat analysis structure defined
  - Requirements structure defined
  - Testing strategy structure defined
```

#### 1.2 Registry Updates

```yaml
task: Create V4 registry structure
status: pending
files:
  - registry-v4.toml
dependencies: []
acceptance_criteria:
  - New command namespaces defined (discover, comply, secure)
  - Backward compatibility markers for V3 commands
  - Version field updated to 4.0.0
```

```yaml
task: Define command deprecation strategy
status: pending
files:
  - docs/development/v3-deprecation.md
dependencies: []
acceptance_criteria:
  - V3 → V4 command mapping documented
  - Deprecation warnings defined
  - Sunset timeline established
```

#### 1.3 CLI Preparation

```yaml
task: Add V4 command structure to CLI
status: pending
files:
  - cli/src/commands/mod.rs
  - cli/src/commands/discover.rs
  - cli/src/commands/comply.rs
  - cli/src/commands/secure.rs
dependencies:
  - registry-v4.toml
acceptance_criteria:
  - discover subcommand module created
  - comply subcommand module created
  - secure subcommand module created
  - Help text reflects new structure
```

#### 1.4 Multi-Agent Support

```yaml
task: Update agents.toml for V4
status: pending
files:
  - agents.toml
acceptance_criteria:
  - Version bumped to 4.0.0
  - New commands registered (discover, comply, secure)
  - Capabilities field added per agent
  - Part field added for command grouping
```

```yaml
task: Update Claude Code template for V4
status: pending
files:
  - templates/agents/claude-code.tera
dependencies:
  - agents.toml
acceptance_criteria:
  - Supports new command structure
  - Groups commands by part (discover/comply/secure)
  - Includes system-model references
```

```yaml
task: Update Copilot template for V4
status: pending
files:
  - templates/agents/copilot.tera
dependencies:
  - agents.toml
acceptance_criteria:
  - Handles limited capabilities gracefully
  - Includes manual step instructions where needed
```

```yaml
task: Update Cursor template for V4
status: pending
files:
  - templates/agents/cursor.tera
dependencies:
  - agents.toml
acceptance_criteria:
  - Supports .mdc rule format
  - Groups commands by part
```

```yaml
task: Update Gemini template for V4
status: pending
files:
  - templates/agents/gemini.tera
dependencies:
  - agents.toml
acceptance_criteria:
  - Handles limited capabilities
  - Single-file format maintained
```

```yaml
task: Update universal AGENTS.md template
status: pending
files:
  - templates/agents/AGENTS.md.tera
dependencies:
  - agents.toml
acceptance_criteria:
  - Documents V4 three-part structure
  - Explains workflow for any AI agent
```

---

## Phase 2: Part 1 - Discover

### Objective
Implement the "Reality Model" hub that creates and maintains `system-model.yaml`.

### Tasks

#### 2.1 Core Discovery Engine

```yaml
task: Implement stack detection enhancements
status: pending
files:
  - cli/src/stack.rs
dependencies: []
acceptance_criteria:
  - Detects all major frameworks (Node, Python, Go, Rust, Java, PHP, Ruby)
  - Identifies infrastructure (Docker, K8s, Terraform)
  - Returns structured StackInfo with confidence scores
```

```yaml
task: Implement architecture extraction
status: pending
files:
  - cli/src/discover/architecture.rs
dependencies:
  - stack detection
acceptance_criteria:
  - Identifies service components from code structure
  - Detects data stores from config/code
  - Maps data flows from imports/calls
  - Generates Mermaid diagram
```

```yaml
task: Implement data discovery
status: pending
files:
  - cli/src/discover/data.rs
dependencies:
  - architecture extraction
acceptance_criteria:
  - Scans ORM models (Prisma, SQLAlchemy, TypeORM, etc.)
  - Identifies PII fields by name patterns
  - Classifies data sensitivity
  - Maps data to storage components
```

```yaml
task: Implement actor mapping
status: pending
files:
  - cli/src/discover/actors.rs
dependencies:
  - data discovery
acceptance_criteria:
  - Identifies user types from auth code
  - Extracts roles/permissions from RBAC definitions
  - Maps actor-to-data access patterns
```

```yaml
task: Implement trust boundary detection
status: pending
files:
  - cli/src/discover/trust.rs
dependencies:
  - architecture extraction
acceptance_criteria:
  - Identifies network boundaries from infra code
  - Detects authentication gates
  - Maps encryption boundaries
  - Generates trust zone diagram
```

```yaml
task: Implement integration detection
status: pending
files:
  - cli/src/discover/integrations.rs
dependencies:
  - data discovery
acceptance_criteria:
  - Detects third-party services from dependencies
  - Identifies API calls to external services
  - Extracts data shared with integrations
  - Flags missing DPA status
```

#### 2.2 Discovery Commands

```yaml
task: Implement /osk-discover init command
status: pending
files:
  - cli/src/commands/discover/init.rs
  - prompts/osk-discover-init.md
dependencies:
  - All discovery modules
acceptance_criteria:
  - Runs full discovery pipeline
  - Generates system-model/ folder with split files
  - index.yaml always < 200 lines
  - Identifies gaps requiring user input
  - Outputs summary report
```

```yaml
task: Implement /osk-discover scan command
status: pending
files:
  - cli/src/commands/discover/scan.rs
  - prompts/osk-discover-scan.md
dependencies:
  - /osk-discover init
acceptance_criteria:
  - Supports --architecture flag
  - Supports --data flag
  - Supports --actors flag
  - Supports --business flag
  - Supports --integrations flag
  - Updates specific sections of system-model.yaml
```

```yaml
task: Implement /osk-discover update command
status: pending
files:
  - cli/src/commands/discover/update.rs
  - prompts/osk-discover-update.md
dependencies:
  - /osk-discover init
acceptance_criteria:
  - Detects drift from previous scan
  - Incrementally updates system-model.yaml
  - Reports changes since last scan
  - Preserves manual edits
```

```yaml
task: Implement /osk-discover validate command
status: pending
files:
  - cli/src/commands/discover/validate.rs
  - prompts/osk-discover-validate.md
dependencies:
  - /osk-discover init
acceptance_criteria:
  - Validates system-model.yaml against schema
  - Checks references (component IDs, data IDs, etc.)
  - Verifies model matches current code
  - Reports validation errors
```

#### 2.3 Discovery Prompts

```yaml
task: Create osk-discover-init.md prompt
status: pending
files:
  - prompts/osk-discover-init.md
dependencies:
  - system-model.yaml schema
acceptance_criteria:
  - Defines AI agent role as "Discovery Analyst"
  - Documents 6-phase discovery process
  - References system-model.yaml schema
  - Includes validation and user confirmation steps
```

```yaml
task: Create osk-discover-scan.md prompt
status: pending
files:
  - prompts/osk-discover-scan.md
dependencies:
  - osk-discover-init.md
acceptance_criteria:
  - Supports focused scans per aspect
  - Documents merge strategy with existing model
  - Handles conflicts with existing data
```

```yaml
task: Create osk-discover-update.md prompt
status: pending
files:
  - prompts/osk-discover-update.md
dependencies:
  - osk-discover-init.md
acceptance_criteria:
  - Documents drift detection logic
  - Defines incremental update strategy
  - Preserves manual annotations
```

---

## Phase 3: Part 2 - Comply

### Objective
Implement compliance assessment that consumes the system model and maps to frameworks.

### Tasks

#### 3.1 Compliance Engine

```yaml
task: Create compliance framework loader
status: pending
files:
  - cli/src/comply/frameworks/mod.rs
  - cli/src/comply/frameworks/loader.rs
dependencies: []
acceptance_criteria:
  - Loads framework definitions from frameworks/
  - Parses control requirements
  - Supports versioned frameworks
```

```yaml
task: Implement control mapping engine
status: pending
files:
  - cli/src/comply/mapper.rs
dependencies:
  - framework loader
acceptance_criteria:
  - Maps system-model sections to framework controls
  - Calculates compliance scores
  - Identifies gaps
  - Generates evidence references
```

```yaml
task: Implement cross-framework analysis
status: pending
files:
  - cli/src/comply/cross_framework.rs
dependencies:
  - control mapping engine
acceptance_criteria:
  - Identifies shared controls across frameworks
  - Maps equivalent controls
  - Calculates unified compliance score
```

#### 3.2 Framework Definitions

```yaml
task: Refactor RGPD domain for V4
status: pending
files:
  - frameworks/rgpd/framework.yaml
  - frameworks/rgpd/controls/
dependencies:
  - compliance-assessment.yaml schema
acceptance_criteria:
  - Controls defined in structured format
  - Mapping rules to system-model sections
  - Evidence requirements per control
  - Cross-references to other frameworks
```

```yaml
task: Refactor RGS domain for V4
status: pending
files:
  - frameworks/rgs/framework.yaml
  - frameworks/rgs/controls/
dependencies:
  - compliance-assessment.yaml schema
acceptance_criteria:
  - EBIOS RM integration
  - Homologation requirements structured
  - ANSSI crypto requirements mapped
```

```yaml
task: Complete NIS2 domain
status: pending
files:
  - frameworks/nis2/framework.yaml
  - frameworks/nis2/controls/
dependencies:
  - compliance-assessment.yaml schema
acceptance_criteria:
  - Art. 21 requirements defined
  - Art. 23 incident reporting defined
  - Sector classification logic
  - Cross-references to RGPD, RGS
```

```yaml
task: Add ISO 27001 domain
status: pending
files:
  - frameworks/iso27001/framework.yaml
  - frameworks/iso27001/controls/
  - frameworks/iso27001/README.md
  - frameworks/iso27001/templates/
dependencies:
  - compliance-assessment.yaml schema
acceptance_criteria:
  - All Annex A controls defined
  - Statement of Applicability template
  - Cross-references to other frameworks
  - Evidence requirements per control
```

```yaml
task: Add SOC 2 domain
status: pending
files:
  - frameworks/soc2/framework.yaml
  - frameworks/soc2/controls/
  - frameworks/soc2/README.md
dependencies:
  - compliance-assessment.yaml schema
acceptance_criteria:
  - Trust Services Criteria defined
  - All 5 categories supported (Security, Availability, etc.)
  - Cross-references to ISO 27001
```

#### 3.3 Compliance Commands

```yaml
task: Implement /osk-comply <framework> command
status: pending
files:
  - cli/src/commands/comply/assess.rs
  - prompts/osk-comply.md
dependencies:
  - compliance engine
  - framework definitions
acceptance_criteria:
  - Loads system-model.yaml
  - Runs framework assessment
  - Generates compliance-assessment-<framework>.yaml
  - Generates framework-specific docs
```

```yaml
task: Implement /osk-comply gap-analysis command
status: pending
files:
  - cli/src/commands/comply/gap_analysis.rs
  - prompts/osk-comply-gap-analysis.md
dependencies:
  - /osk-comply <framework>
acceptance_criteria:
  - Compares multiple framework assessments
  - Identifies cross-framework gaps
  - Prioritizes remediation actions
  - Generates consolidated gap report
```

```yaml
task: Implement /osk-comply evidence command
status: pending
files:
  - cli/src/commands/comply/evidence.rs
  - prompts/osk-comply-evidence.md
dependencies:
  - /osk-comply <framework>
acceptance_criteria:
  - Collects evidence for controls
  - Validates evidence freshness
  - Links evidence to controls
  - Generates evidence index
```

#### 3.4 Compliance Prompts

```yaml
task: Create osk-comply.md prompt
status: pending
files:
  - prompts/osk-comply.md
dependencies:
  - compliance-assessment.yaml schema
acceptance_criteria:
  - Defines AI agent role as "Compliance Analyst"
  - Documents assessment workflow
  - References framework-specific templates
  - Includes gap analysis guidance
```

```yaml
task: Refactor osk-rgpd.md for V4
status: pending
files:
  - prompts/osk-rgpd.md → prompts/osk-comply-rgpd.md
dependencies:
  - osk-comply.md
acceptance_criteria:
  - Consumes system-model.yaml
  - Generates compliance-assessment-rgpd.yaml
  - Backward compatible with V3 output locations
```

```yaml
task: Refactor osk-rgs.md for V4
status: pending
files:
  - prompts/osk-rgs.md → prompts/osk-comply-rgs.md
dependencies:
  - osk-comply.md
acceptance_criteria:
  - Consumes system-model.yaml
  - Integrates EBIOS RM from system-model
  - Generates compliance-assessment-rgs.yaml
```

---

## Phase 4: Part 3 - Secure

### Objective
Implement SpecKit-style security workflow for new features.

### Tasks

#### 4.1 Specification Command

```yaml
task: Implement /osk-secure specify command
status: pending
files:
  - cli/src/commands/secure/specify.rs
  - prompts/osk-secure-specify.md
dependencies:
  - system-model.yaml
acceptance_criteria:
  - Consumes system-model for context
  - Generates security-spec.yaml (STRIDE analysis, requirements)
  - Generates threats.md (STRIDE analysis)
  - Generates requirements.md (security requirements)
  - Generates testing.md (test strategy)
  - Generates hardening.md (secrets, logging, patching)
  - Updates risk-register.yaml
```

```yaml
task: Create osk-secure-specify.md prompt
status: pending
files:
  - prompts/osk-secure-specify.md
dependencies:
  - system-model.yaml
acceptance_criteria:
  - Merges current osk-analyze.md threat modeling
  - Merges current osk-specify.md requirements
  - Merges current osk-harden.md hardening
  - References 7 security principles for prioritization
  - Uses system-model for context
```

#### 4.2 Clarification Command

```yaml
task: Implement /osk-secure clarify command
status: pending
files:
  - cli/src/commands/secure/clarify.rs
  - prompts/osk-secure-clarify.md
dependencies:
  - /osk-secure specify
acceptance_criteria:
  - Reads clarifications_needed from security-spec
  - Presents questions to user
  - Updates security-spec with answers
  - Re-runs specify if needed
```

```yaml
task: Create osk-secure-clarify.md prompt
status: pending
files:
  - prompts/osk-secure-clarify.md
dependencies:
  - security-spec.yaml schema
acceptance_criteria:
  - Presents clarification questions clearly
  - Documents security implications of each option
  - Updates specs after clarification
```

#### 4.3 Planning Commands

```yaml
task: Refactor /osk-plan for V4
status: pending
files:
  - prompts/osk-plan.md → prompts/osk-secure-plan.md
dependencies:
  - /osk-secure specify
acceptance_criteria:
  - Consumes specify outputs (threats, requirements, testing, hardening)
  - References security-spec for priorities
  - Generates plan.md
  - Backward compatible with V3
```

```yaml
task: Refactor /osk-tasks for V4
status: pending
files:
  - prompts/osk-tasks.md → prompts/osk-secure-tasks.md
dependencies:
  - /osk-secure plan
acceptance_criteria:
  - Consumes plan.md
  - Generates tasks.md and tasks.yaml
  - Links tasks to risks
  - Backward compatible with V3
```

#### 4.4 Implementation Command

```yaml
task: Refactor /osk-implement for V4
status: pending
files:
  - prompts/osk-implement.md → prompts/osk-secure-implement.md
dependencies:
  - /osk-secure tasks
acceptance_criteria:
  - Consumes tasks.yaml
  - Updates risk-register on completion
  - Updates system-model if architecture changed
  - Backward compatible with V3
```

---

## Phase 5: Integration

### Objective
Connect all three parts and ensure they work together seamlessly.

### Tasks

#### 5.1 Data Flow Integration

```yaml
task: Ensure Part 2 reads system-model correctly
status: pending
files:
  - cli/src/comply/system_model_reader.rs
dependencies:
  - Part 1 complete
  - Part 2 complete
acceptance_criteria:
  - Comply commands load system-model.yaml
  - Data mappings work correctly
  - Missing system-model triggers helpful error
```

```yaml
task: Ensure Part 3 reads system-model correctly
status: pending
files:
  - cli/src/secure/system_model_reader.rs
dependencies:
  - Part 1 complete
  - Part 3 complete
acceptance_criteria:
  - Secure commands load system-model.yaml
  - Context extraction works correctly
  - Security-spec references system-model data
```

```yaml
task: Implement system-model update from Part 3
status: pending
files:
  - cli/src/secure/system_model_updater.rs
dependencies:
  - Part 3 complete
acceptance_criteria:
  - /osk-secure implement can update system-model
  - New components added during implementation reflected
  - Triggers re-assessment recommendation
```

#### 5.2 Dashboard Updates

```yaml
task: Update dashboard for V4 structure
status: pending
files:
  - prompts/osk-dashboard.md
dependencies:
  - All parts complete
acceptance_criteria:
  - Shows system-model summary
  - Shows compliance scores per framework
  - Shows risk-register summary
  - Shows recent changes across all parts
```

```yaml
task: Create unified compliance view
status: pending
files:
  - templates/outputs/compliance-dashboard.md.tmpl
dependencies:
  - Part 2 complete
acceptance_criteria:
  - Aggregates all framework assessments
  - Shows cross-framework compliance
  - Highlights critical gaps
```

#### 5.3 Validation & Testing

```yaml
task: Create integration tests
status: pending
files:
  - cli/tests/integration/
dependencies:
  - All parts complete
acceptance_criteria:
  - Test discover → comply flow
  - Test discover → secure flow
  - Test full workflow (discover → comply → secure → implement)
  - Test system-model updates propagate correctly
```

```yaml
task: Create example project for testing
status: pending
files:
  - examples/v4-demo/
dependencies:
  - All parts complete
acceptance_criteria:
  - Sample Node.js project with known structure
  - Pre-generated system-model for comparison
  - Expected compliance assessment results
  - Expected security specs
```

---

## Phase 6: Migration & Documentation

### Objective
Enable smooth migration from V3 and complete documentation.

### Tasks

#### 6.1 Migration Tools

```yaml
task: Create V3 → V4 migration command
status: pending
files:
  - cli/src/commands/migrate.rs
dependencies:
  - All parts complete
acceptance_criteria:
  - Detects V3 project structure
  - Generates system-model from existing context.md
  - Converts existing specs to V4 format
  - Preserves risk-register data
```

```yaml
task: Create migration guide
status: pending
files:
  - docs/migration/v3-to-v4.md
dependencies:
  - migration command
acceptance_criteria:
  - Step-by-step migration instructions
  - Breaking changes documented
  - Rollback procedure documented
  - FAQ for common issues
```

#### 6.2 Documentation Updates

```yaml
task: Update main documentation
status: pending
files:
  - docs/index.md
  - docs/getting-started/quickstart.md
  - docs/getting-started/installation.md
dependencies:
  - All parts complete
acceptance_criteria:
  - Reflects V4 architecture
  - Updated command references
  - New workflow diagrams
```

```yaml
task: Create Part 1 documentation
status: pending
files:
  - docs/concepts/discover.md
  - docs/commands/osk-discover-init.md
  - docs/commands/osk-discover-scan.md
  - docs/commands/osk-discover-update.md
  - docs/commands/osk-discover-validate.md
dependencies:
  - Part 1 complete
acceptance_criteria:
  - Explains system-model concept
  - Documents all discover commands
  - Provides examples
```

```yaml
task: Create Part 2 documentation
status: pending
files:
  - docs/concepts/comply.md
  - docs/commands/osk-comply.md
  - docs/domains/index.md (update)
dependencies:
  - Part 2 complete
acceptance_criteria:
  - Explains compliance assessment
  - Documents framework support
  - Documents cross-framework analysis
```

```yaml
task: Create Part 3 documentation
status: pending
files:
  - docs/concepts/secure.md
  - docs/commands/osk-secure-constitute.md
  - docs/commands/osk-secure-specify.md
  - docs/commands/osk-secure-clarify.md
  - docs/commands/osk-secure-plan.md
  - docs/commands/osk-secure-tasks.md
  - docs/commands/osk-secure-implement.md
dependencies:
  - Part 3 complete
acceptance_criteria:
  - Explains SpecKit-style workflow
  - Documents 7 security principles
  - Provides full workflow example
```

#### 6.3 Release Preparation

```yaml
task: Update changelog
status: pending
files:
  - docs/changelog.md
  - CHANGELOG.md
dependencies:
  - All documentation complete
acceptance_criteria:
  - V4.0.0 changes documented
  - Breaking changes highlighted
  - Migration notes included
```

```yaml
task: Update version numbers
status: pending
files:
  - cli/Cargo.toml
  - registry.toml
  - package.json
dependencies:
  - All tasks complete
acceptance_criteria:
  - Version bumped to 4.0.0
  - All version references consistent
```

```yaml
task: Create release notes
status: pending
files:
  - RELEASE-NOTES-v4.md
dependencies:
  - changelog updated
acceptance_criteria:
  - Summary of V4 architecture
  - Key features highlighted
  - Migration path documented
  - Known issues listed
```

---

## Task Summary

### By Phase

| Phase | Total Tasks | Status |
|-------|-------------|--------|
| Phase 1: Foundation | 12 | 3 done, 9 pending |
| Phase 2: Part 1 (Discover) | 13 | 0 done, 13 pending |
| Phase 3: Part 2 (Comply) | 12 | 0 done, 12 pending |
| Phase 4: Part 3 (Secure) | 9 | 0 done, 9 pending |
| Phase 5: Integration | 6 | 0 done, 6 pending |
| Phase 6: Migration & Docs | 8 | 0 done, 8 pending |
| **Total** | **60** | **3 done, 57 pending** |

### By Priority

| Priority | Description | Tasks |
|----------|-------------|-------|
| P0 | Core schemas and CLI structure | 6 |
| P1 | Part 1 (Discover) - Foundation for everything | 13 |
| P2 | Part 3 (Secure) - Core workflow | 9 |
| P3 | Part 2 (Comply) - Framework support | 12 |
| P4 | Integration and documentation | 14 |

### Dependencies Graph

```
Phase 1 (Foundation)
    │
    ├──▶ Phase 2 (Discover)
    │         │
    │         ├──▶ Phase 3 (Comply)
    │         │         │
    │         │         └──▶ Phase 5 (Integration)
    │         │                     │
    │         └──▶ Phase 4 (Secure) │
    │                   │           │
    │                   └───────────┘
    │                         │
    └─────────────────────────┴──▶ Phase 6 (Migration & Docs)
```

---

## Next Steps

1. **Immediate**: Review and approve this implementation plan
2. **Start Phase 1**: Complete registry-v4.toml and CLI command structure
3. **Parallel work**: Begin Part 1 discovery modules while finalizing foundation
4. **Iterative releases**: Consider alpha releases after each phase

---

## Open Questions

1. **Versioning Strategy**: Should V4 be a new major version or a feature flag in V3?
2. **Backward Compatibility**: How long to maintain V3 command aliases?
3. **Framework Priority**: Which compliance frameworks to implement first after RGPD/RGS?
4. **Testing Strategy**: Unit tests vs integration tests vs end-to-end tests priority?
