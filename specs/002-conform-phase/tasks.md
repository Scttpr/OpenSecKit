# Tasks: Conformity Phase v4

**Input**: Design documents from `/specs/002-conform-phase/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Not explicitly requested - manual testing via agent prompts

**Organization**: Tasks grouped by user story. This is a prompt + template feature (no new CLI code).

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1-US6)
- Paths are relative to repository root

---

## Phase 1: Setup (Directory Structure)

**Purpose**: Create directories and copy schemas

- [X] T001 Create directory structure: `domaines/rgs/templates/`, `domaines/_schema/`, `templates/data/comply/`, `templates/outputs/comply/`
- [X] T002 [P] Copy framework schema from `specs/002-conform-phase/contracts/framework-schema.yaml` to `domaines/_schema/framework-schema.yaml`
- [X] T003 [P] Copy assessment scope contract from `specs/002-conform-phase/contracts/assessment-scope.yaml` to `templates/schemas/assessment-scope.yaml`

---

## Phase 2: Foundational (Shared Templates & RGS Framework)

**Purpose**: Create shared infrastructure that ALL user stories depend on

**⚠️ CRITICAL**: US1 and US2 both need the RGS framework.yaml and shared templates

### Shared Output Templates

- [X] T004 [P] Create assessment YAML template in `templates/data/comply/assessment.yaml.tera`
- [X] T005 [P] Create assessment report template in `templates/outputs/comply/assessment-report.md.tera`
- [X] T006 [P] Create gap analysis template in `templates/outputs/comply/gap-analysis.md.tera`
- [X] T007 [P] Create terminal summary template in `templates/reports/compliance-summary.tera`

### RGS Framework Definition

- [X] T008 Create RGS framework.yaml with metadata and security levels in `domaines/rgs/framework.yaml`
- [X] T009 Add RGS categories (B2-B7: Auth, Integrity, Confidentiality, Traceability, Timestamping, Signature) to `domaines/rgs/framework.yaml`
- [X] T010 Add RGS controls for Authentication domain (RGS-B2.1 to RGS-B2.5) to `domaines/rgs/framework.yaml`
- [X] T011 Add RGS controls for Integrity domain (RGS-B3.1 to RGS-B3.4) to `domaines/rgs/framework.yaml`
- [X] T012 Add RGS controls for Confidentiality domain (RGS-B4.1 to RGS-B4.4) to `domaines/rgs/framework.yaml`
- [X] T013 Add RGS controls for Traceability domain (RGS-B5.1 to RGS-B5.5) to `domaines/rgs/framework.yaml`
- [X] T014 Add RGS controls for Timestamping domain (RGS-B6.1 to RGS-B6.2) to `domaines/rgs/framework.yaml`
- [X] T015 Add RGS controls for Signature domain (RGS-B7.1 to RGS-B7.2) to `domaines/rgs/framework.yaml`
- [X] T016 Add scoring thresholds and cross_framework_mapping to `domaines/rgs/framework.yaml`

**Checkpoint**: Foundation ready - RGS framework complete, shared templates in place

---

## Phase 3: User Story 1 - Guided GDPR Assessment (Priority: P1) 🎯 MVP

**Goal**: Run `/osk-comply rgpd` for interactive GDPR compliance assessment based on full system context

**Independent Test**: Run `/osk-comply rgpd` on a project with completed system model, verify agent guides through assessment with article references and generates compliance report

### GDPR Output Templates

- [X] T017 [P] [US1] Create GDPR assessment summary template in `domaines/rgpd/templates/assessment-summary.md.tera`
- [X] T018 [P] [US1] Create sub-processor register template in `domaines/rgpd/templates/sub-processor-register.md.tera`
- [X] T019 [P] [US1] Create GDPR gap report template in `domaines/rgpd/templates/gap-report.md.tera`

### GDPR Agent Prompt

- [X] T020 [US1] Update prerequisites check section in `prompts/osk-comply-rgpd.md` (check for system model, warn on missing sections)
- [X] T021 [US1] Update scope definition section in `prompts/osk-comply-rgpd.md` (present data.yaml + integrations.yaml + tooling.yaml + architecture.yaml)
- [X] T022 [US1] Add evidence auto-detection logic in `prompts/osk-comply-rgpd.md` (map evidence_types to system model paths)
- [X] T023 [US1] Update control assessment workflow in `prompts/osk-comply-rgpd.md` (chapter-grouped, display article text, allow confirm/reject/skip)
- [X] T024 [US1] Add international transfer detection in `prompts/osk-comply-rgpd.md` (flag non-EU in integrations, tooling, architecture)
- [X] T025 [US1] Add sub-processor assessment in `prompts/osk-comply-rgpd.md` (Art. 28 - list from integrations AND tooling)
- [X] T026 [US1] Update gap analysis and report generation in `prompts/osk-comply-rgpd.md` (score, gaps grouped by codebase/tooling/infra, action items)
- [X] T027 [US1] Add output file generation in `prompts/osk-comply-rgpd.md` (assessment-rgpd.yaml, assessment-rgpd.md, gap-report-rgpd.md, sub-processor-register.md)

**Checkpoint**: `/osk-comply rgpd` fully functional - can assess GDPR compliance against full system context

---

## Phase 4: User Story 2 - Guided RGS Assessment (Priority: P1)

**Goal**: Run `/osk-comply rgs` for interactive RGS compliance assessment with homologation readiness

**Independent Test**: Run `/osk-comply rgs` on a project with completed system model, verify agent guides through RGS domains and generates homologation-ready documentation

### RGS Output Templates

- [X] T028 [P] [US2] Create RGS assessment summary template in `domaines/rgs/templates/assessment-summary.md.tera`
- [X] T029 [P] [US2] Create homologation checklist template in `domaines/rgs/templates/homologation-checklist.md.tera`
- [X] T030 [P] [US2] Create system perimeter template in `domaines/rgs/templates/system-perimeter.md.tera`

### RGS Agent Prompt

- [X] T031 [US2] Update prerequisites check section in `prompts/osk-comply-rgs.md` (check for system model, warn on missing sections)
- [X] T032 [US2] Add RGS level selection in `prompts/osk-comply-rgs.md` (ask for *, **, *** if not defined)
- [X] T033 [US2] Update scope definition section in `prompts/osk-comply-rgs.md` (present full system perimeter including tooling)
- [X] T034 [US2] Add tool certification check in `prompts/osk-comply-rgs.md` (SecNumCloud, HDS, ISO 27001 for each service)
- [X] T035 [US2] Add evidence auto-detection logic in `prompts/osk-comply-rgs.md` (map evidence_types to system model paths)
- [X] T036 [US2] Update domain-by-domain assessment in `prompts/osk-comply-rgs.md` (B2-B7, display annexe text, ANSSI recommendations)
- [X] T037 [US2] Add cryptographic validation in `prompts/osk-comply-rgs.md` (check algorithms against ANSSI approved list)
- [X] T038 [US2] Add traceability assessment in `prompts/osk-comply-rgs.md` (app logs + infra logs + third-party audit logs)
- [X] T039 [US2] Update homologation readiness in `prompts/osk-comply-rgs.md` (blockers vs recommendations, documentation checklist)
- [X] T040 [US2] Add output file generation in `prompts/osk-comply-rgs.md` (assessment-rgs.yaml, assessment-rgs.md, homologation-checklist.md, system-perimeter.md)

**Checkpoint**: `/osk-comply rgs` fully functional - can assess RGS compliance with homologation guidance

---

## Phase 5: User Story 3 - Framework Extension (Priority: P2)

**Goal**: Enable adding new compliance frameworks by creating framework.yaml only (no code changes)

**Independent Test**: Create a minimal test framework.yaml, run `/osk-comply <test-framework>`, verify agent loads and uses it

### Framework Extension Support

- [ ] T041 [US3] Create framework extension guide in `docs/extending-frameworks.md` (schema reference, example, validation)
- [ ] T042 [US3] Add framework loading logic to base prompt pattern (detect framework from `domaines/<name>/framework.yaml`)
- [ ] T043 [US3] Add schema validation guidance in `prompts/osk-comply-rgpd.md` (reference for other prompts)
- [ ] T044 [US3] Create framework template in `domaines/_template/framework.yaml.example`

**Checkpoint**: Contributors can add new frameworks without code changes

---

## Phase 6: User Story 4 - Compliance Status Dashboard (Priority: P2)

**Goal**: Run `/osk-comply status` to see multi-framework compliance summary

**Independent Test**: Run GDPR and RGS assessments, then `/osk-comply status`, verify unified view with scores and gaps

### Status Dashboard

- [ ] T045 [US4] Create status dashboard prompt in `prompts/osk-comply-status.md` (load all assessments from .osk/compliance/)
- [ ] T046 [US4] Add assessment summary display in `prompts/osk-comply-status.md` (framework, date, score, gaps, status)
- [ ] T047 [US4] Add cross-framework analysis in `prompts/osk-comply-status.md` (shared controls, cross-framework gaps)
- [ ] T048 [US4] Add framework selection in `prompts/osk-comply-status.md` (offer update or detailed report)

### List Frameworks

- [ ] T049 [P] [US4] Create list frameworks prompt in `prompts/osk-comply-list.md` (scan domaines/ for framework.yaml files)

**Checkpoint**: `/osk-comply status` shows multi-framework dashboard

---

## Phase 7: User Story 5 - Assessment Update (Priority: P2)

**Goal**: Run `/osk-comply rgpd --update` to incrementally update assessment based on system model changes

**Independent Test**: Run initial assessment, modify system model, run update, verify only changed controls re-assessed

### Update Logic

- [ ] T050 [US5] Add system model hash comparison in `prompts/osk-comply-rgpd.md` (detect changes since last assessment)
- [ ] T051 [US5] Add affected controls identification in `prompts/osk-comply-rgpd.md` (map changes to controls)
- [ ] T052 [US5] Add diff-style view in `prompts/osk-comply-rgpd.md` (new gaps, closed gaps, unchanged)
- [ ] T053 [US5] Add audit trail update in `prompts/osk-comply-rgpd.md` (record change in assessment file)
- [ ] T054 [US5] Add `--update` flag handling to `prompts/osk-comply-rgs.md` (same logic as GDPR)

### Resume Logic

- [ ] T055 [US5] Add partial assessment save in `prompts/osk-comply-rgpd.md` (save progress on interrupt)
- [ ] T056 [US5] Add `--resume` flag handling in `prompts/osk-comply-rgpd.md` (restore state)
- [ ] T057 [US5] Add `--resume` flag handling in `prompts/osk-comply-rgs.md` (same logic)

**Checkpoint**: `--update` and `--resume` flags work for both GDPR and RGS

---

## Phase 8: User Story 6 - Export Documentation (Priority: P3)

**Goal**: Run `/osk-comply rgpd --export md` to generate formatted compliance reports

**Independent Test**: Run assessment, then `--export md`, verify formatted document follows framework guidelines

### Export Logic

- [ ] T058 [US6] Add `--export md` flag handling in `prompts/osk-comply-rgpd.md` (use framework templates)
- [ ] T059 [US6] Add GDPR record-of-processing format in `domaines/rgpd/templates/export-report.md.tera`
- [ ] T060 [US6] Add `--export md` flag handling in `prompts/osk-comply-rgs.md`
- [ ] T061 [US6] Add RGS dossier d'homologation format in `domaines/rgs/templates/export-dossier.md.tera`
- [ ] T062 [US6] Add draft watermark logic for assessments with critical gaps

**Checkpoint**: `--export md` generates audit-ready documentation

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, validation, and cleanup

- [ ] T063 [P] Update CLAUDE.md with conform phase commands in `CLAUDE.md`
- [ ] T064 [P] Add conform phase to agents.toml in `agents.toml`
- [ ] T065 Validate all templates render correctly with sample data
- [ ] T066 Run quickstart.md scenarios for manual validation
- [ ] T067 Update README.md with conform phase usage

---

## Dependencies & Execution Order

### Phase Dependencies

```
Phase 1 (Setup)
    │
    ▼
Phase 2 (Foundational) ─── BLOCKS ALL USER STORIES
    │
    ├──────────┬──────────┬──────────┬──────────┬──────────┐
    ▼          ▼          ▼          ▼          ▼          ▼
Phase 3    Phase 4    Phase 5    Phase 6    Phase 7    Phase 8
  (US1)      (US2)      (US3)      (US4)      (US5)      (US6)
  P1 🎯      P1         P2         P2         P2         P3
    │          │          │          │          │          │
    └──────────┴──────────┴──────────┴──────────┴──────────┘
                              │
                              ▼
                        Phase 9 (Polish)
```

### User Story Dependencies

| Story | Depends On | Can Parallel With |
|-------|------------|-------------------|
| US1 (GDPR) | Foundational only | US2, US3, US4 |
| US2 (RGS) | Foundational only | US1, US3, US4 |
| US3 (Extension) | US1 or US2 complete (for reference) | US4 |
| US4 (Status) | US1 and US2 (needs assessments to display) | - |
| US5 (Update) | US1 and US2 (needs initial assessments) | US6 |
| US6 (Export) | US1 and US2 (needs assessments to export) | US5 |

### Parallel Opportunities

**Phase 2 (Foundational)**:
```
T004, T005, T006, T007 (shared templates) - all parallel
T008-T016 (RGS framework) - sequential within file
```

**Phase 3 (US1) + Phase 4 (US2)**:
```
# Can run in parallel after Foundational:
US1 templates (T017, T018, T019) || US2 templates (T028, T029, T030)
US1 prompt updates (T020-T027) || US2 prompt updates (T031-T040)
```

---

## Implementation Strategy

### MVP First (P1 Stories Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (RGS framework + shared templates)
3. Complete Phase 3: US1 - GDPR Assessment
4. **STOP and VALIDATE**: Test `/osk-comply rgpd` end-to-end
5. Complete Phase 4: US2 - RGS Assessment
6. **STOP and VALIDATE**: Test `/osk-comply rgs` end-to-end
7. Ship MVP with GDPR + RGS support

### Incremental Delivery

| Increment | Stories | Value Delivered |
|-----------|---------|-----------------|
| MVP | US1 + US2 | Core compliance assessments |
| +Extension | US3 | Community can add frameworks |
| +Dashboard | US4 | Multi-framework visibility |
| +Update | US5 | Incremental re-assessment |
| +Export | US6 | Audit-ready reports |

---

## Summary

| Metric | Count |
|--------|-------|
| Total Tasks | 67 |
| Setup (Phase 1) | 3 |
| Foundational (Phase 2) | 13 |
| US1 - GDPR (P1) | 11 |
| US2 - RGS (P1) | 13 |
| US3 - Extension (P2) | 4 |
| US4 - Status (P2) | 5 |
| US5 - Update (P2) | 8 |
| US6 - Export (P3) | 5 |
| Polish (Phase 9) | 5 |
| Parallelizable Tasks | 18 |

**MVP Scope**: Phase 1 + Phase 2 + Phase 3 (US1) + Phase 4 (US2) = 40 tasks
