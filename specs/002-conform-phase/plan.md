# Implementation Plan: Conformity Phase v4

**Branch**: `002-conform-phase` | **Date**: 2026-01-15 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/002-conform-phase/spec.md`

## Summary

Implement the Conformity (Comply) phase for OpenSecKit v4, enabling guided compliance assessments against GDPR and RGS frameworks based on the system model from the Discover phase. The phase provides interactive agent prompts that walk users through control-by-control assessment, auto-detecting evidence from the full system context (codebase, tooling, integrations, infrastructure), and generating audit-ready documentation.

**Technical Approach**: Extend existing infrastructure with new agent prompts for compliance workflows. Leverage existing YAML framework definitions in `domaines/`. Use Tera templates for generating assessment reports. Agent prompts drive the interactive workflow; no new CLI commands required (reuses `osk validate` for schema validation).

## CLI vs Agent Architecture

Following the pattern established in the Discover phase, the Conform phase is **100% agent-driven** with no new CLI commands.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           User Workflow                                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Prerequisites (from Discover)          Conform Phase Commands              │
│  ──────────────────────────────          ─────────────────────              │
│                                                                              │
│  $ osk init                             /osk-comply rgpd                     │
│  /osk-discover init                     /osk-comply rgs                      │
│                                         /osk-comply status                   │
│  Produces:                              /osk-comply rgpd --update            │
│  - .osk/system-model/                   /osk-comply rgpd --export md         │
│                                         /osk-comply --list                   │
│                                                                              │
│                                         Requires: .osk/system-model/ exists │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Reused CLI Utilities** (from Discover phase):

| Command | Purpose in Conform | Usage |
|---------|-------------------|-------|
| `osk validate` | Validate assessment YAML | After generating assessment output |

**No new CLI commands needed**: All compliance logic is agent-driven (control iteration, evidence mapping, user interaction, scoring).

## Technical Context

**Language/Version**: Rust 2021 edition (v1.75+) - for CLI utilities only (minimal)
**Primary Dependencies**: tera 1.20 (templates), serde_yaml 0.9 (YAML), existing framework.yaml parsing
**Storage**: Local filesystem (`.osk/compliance/` directory with YAML + Markdown files)
**Testing**: Manual testing via agent prompts; YAML schema validation via `osk validate`
**Target Platform**: Cross-platform (Linux, macOS, Windows) - agent prompts are platform-agnostic
**Project Type**: Single CLI project + agent prompts
**Performance Goals**: Full assessment <45min (GDPR), <30min (RGS); Update <5min
**Constraints**: Must work with any AI agent (Claude Code, Copilot, Cursor, Gemini)
**Scale/Scope**: ~25 GDPR controls, ~20 RGS controls; projects with up to 50 sub-processors

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Evidence |
|-----------|--------|----------|
| I. AI-Agent Best Practices | ✅ PASS | Prompts use explicit control-by-control workflow. YAML schemas machine-parseable. Templates use clear delimiters. Evidence mapping deterministic. |
| II. Absolute Truthfulness | ✅ PASS | Auto-detected evidence clearly labeled. Manual evidence flagged for auditor review. Unknown status recorded as "not_assessed", never assumed. |
| III. Open Source Excellence | ✅ PASS | MIT license. GDPR/RGS requirements from public EU regulations. No proprietary dependencies. |
| IV. Semantic Versioning | ✅ PASS | Feature follows conventional commits. Changes will be MINOR bump to CLI version. |
| V. Atomic Feature Development | ✅ PASS | Single feature branch (002-conform-phase). 6 user stories, each independently testable. P1 stories (GDPR, RGS) can ship without P2/P3. |
| VI. User-Driven Architecture | ✅ PASS | Interactive assessment asks for confirmation on each control. User can skip, reject auto-evidence, or add manual evidence. User confirms scope before assessment begins. |

**Gate Status**: ✅ PASSED - All principles satisfied. Proceed to Phase 0.

## Project Structure

### Documentation (this feature)

```text
specs/002-conform-phase/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output (YAML schemas)
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
prompts/
├── osk-comply-rgpd.md       # EXISTS - needs major update for v4 workflow
├── osk-comply-rgs.md        # EXISTS - needs major update for v4 workflow
├── osk-comply-status.md     # NEW - multi-framework dashboard
└── osk-comply-list.md       # NEW - list available frameworks

domaines/
├── rgpd/
│   ├── framework.yaml       # EXISTS - complete with 25 controls
│   └── templates/           # NEW - GDPR-specific output templates
│       ├── assessment-summary.md.tera
│       ├── sub-processor-register.md.tera
│       └── gap-report.md.tera
├── rgs/                     # NEEDS CREATION
│   ├── framework.yaml       # NEW - RGS controls by domain
│   └── templates/
│       ├── assessment-summary.md.tera
│       ├── homologation-checklist.md.tera
│       └── system-perimeter.md.tera
└── _schema/
    └── framework-schema.yaml  # NEW - JSON Schema for framework.yaml validation

templates/
├── schemas/
│   └── compliance-assessment.yaml  # EXISTS - complete
├── data/comply/                    # NEW - YAML generation templates
│   └── assessment.yaml.tera
├── outputs/comply/                 # NEW - Markdown output templates
│   ├── assessment-report.md.tera
│   └── gap-analysis.md.tera
└── reports/
    └── compliance-summary.tera     # NEW - terminal output template
```

**Structure Decision**: Extend existing structure. Framework definitions go in `domaines/<framework>/`. Agent prompts follow existing naming in `prompts/`. Templates extend existing `templates/` structure. No new CLI source files needed.

## Complexity Tracking

> No violations requiring justification. Design follows constitution principles.

| Aspect | Decision | Rationale |
|--------|----------|-----------|
| Agent-only workflow | No new CLI commands | Compliance requires judgment; deterministic CLI not appropriate |
| Framework-per-directory | `domaines/<framework>/` structure | Clean extensibility, self-contained frameworks |
| Tera templates for output | Reuse existing templating | Consistent with Discover phase, maintainable |
| YAML for assessment data | Machine-parseable, version-controllable | Aligns with AI-agent best practices |
| Full system context scope | All YAML sections in scope | User-driven architecture; user confirms scope |

---

## Constitution Check (Post-Design)

*Re-evaluated after Phase 1 design completion.*

| Principle | Status | Post-Design Evidence |
|-----------|--------|---------------------|
| I. AI-Agent Best Practices | ✅ PASS | Data model defines explicit evidence mapping paths. Contracts use JSON Schema for validation. Templates follow Tera conventions. |
| II. Absolute Truthfulness | ✅ PASS | Evidence has confidence levels (high/medium/low). Manual evidence flagged as `type: manual`. N/A requires `exclusion_justification`. |
| III. Open Source Excellence | ✅ PASS | All schemas publicly documented. Framework structure extensible without code changes. |
| IV. Semantic Versioning | ✅ PASS | Schema versioned (4.0.0). Backward compatibility maintained via schema_version field. |
| V. Atomic Feature Development | ✅ PASS | Design maintains P1/P2/P3 independence. RGS framework.yaml can be added without GDPR changes. |
| VI. User-Driven Architecture | ✅ PASS | AssessmentScope contract requires user confirmation. Every exclusion requires justification. |

**Post-Design Gate Status**: ✅ PASSED - Design aligns with all constitution principles.

---

## Generated Artifacts

| Artifact | Path | Purpose |
|----------|------|---------|
| Research | `specs/002-conform-phase/research.md` | RGS structure, evidence mapping, template requirements |
| Data Model | `specs/002-conform-phase/data-model.md` | Entity definitions, state transitions, evidence sources |
| Framework Schema | `specs/002-conform-phase/contracts/framework-schema.yaml` | JSON Schema for framework.yaml validation |
| Scope Contract | `specs/002-conform-phase/contracts/assessment-scope.yaml` | Structure for scope presentation and confirmation |
| Quickstart | `specs/002-conform-phase/quickstart.md` | User guide for compliance commands |

---

## Next Steps

Run `/speckit.tasks` to generate implementation tasks from this plan.
