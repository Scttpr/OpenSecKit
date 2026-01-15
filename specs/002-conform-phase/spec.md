# Feature Specification: Conformity Phase v4

**Feature Branch**: `002-conform-phase`
**Created**: 2026-01-15
**Status**: Draft
**Input**: User description: "Renew the conformity phase for v4. It should be easy to add new frameworks. It should embed defined output templates for conformity according to framework guidelines. It should embed source regulations rules as references. It should prompt user to guide him through the process. It should rely on existing context done in first phase. Start with GDPR and RGS only."

## Assessment Scope *(mandatory)*

The compliance assessment covers the **entire system context** captured during the Discover phase, not just the codebase. Any component, tool, or service that processes, stores, or transmits data relevant to the framework is in scope.

### System Model Sources

| Source File | Content | Compliance Relevance |
|-------------|---------|---------------------|
| `data.yaml` | Personal data categories, PII fields, storage locations | What data is processed, retention, sensitivity |
| `integrations.yaml` | External APIs, third-party services | Direct sub-processors, data transfers |
| `tooling.yaml` | CI/CD, documentation (Notion), collaboration (Slack), security tools | Indirect sub-processors, data handling in operations |
| `architecture.yaml` | Cloud providers, hosting, infrastructure | Infrastructure compliance, data residency |
| `actors.yaml` | Users, team members, external parties | Data subjects, access controls, roles |
| `security.yaml` | Detected security controls in code | Technical measures evidence |
| `trust.yaml` | Trust zones, boundaries | Security perimeter, classification |
| `business.yaml` | Domain, processes, criticality | Processing purposes, business context |

### Scope Principle

**If it touches data, it's in scope.** Examples:
- Notion used for documentation → assessed as sub-processor (GDPR Art. 28), checked for data residency
- GitHub Actions for CI/CD → assessed for secrets handling, log retention
- AWS hosting → assessed for data residency, certifications, encryption
- Slack for team communication → assessed if it contains personal data discussions

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Guided GDPR Compliance Assessment (Priority: P1)

As a project owner or DPO, I want to run `/osk-comply rgpd` to perform a guided GDPR compliance assessment based on my system model so that I can identify compliance gaps and generate actionable remediation plans.

**Why this priority**: GDPR is the most widely applicable regulation for projects handling personal data. A guided assessment reduces the expertise barrier and ensures comprehensive coverage.

**Independent Test**: Can be fully tested by running `/osk-comply rgpd` on a project with a completed system model and verifying the agent guides through the assessment, references actual regulation articles, and generates a compliance report.

**Acceptance Scenarios**:

*Phase 1: Prerequisites Check*

1. **Given** a project without a system model, **When** I run `/osk-comply rgpd`, **Then** the agent displays an error: "No system model found. Run `/osk-discover init` first."
2. **Given** an incomplete system model (missing data.yaml), **When** I run `/osk-comply rgpd`, **Then** the agent warns about missing sections and asks to proceed with limited assessment or run `/osk-discover init` first

*Phase 2: Scope Definition (Full System Context)*

3. **Given** a complete system model, **When** I run `/osk-comply rgpd`, **Then** the agent presents a summary of the entire data landscape: personal data categories (from data.yaml), sub-processors (from integrations.yaml), operational tools handling data (from tooling.yaml), and infrastructure (from architecture.yaml)
4. **Given** detected scope, **When** the agent presents tooling like Notion or Slack, **Then** it explains why each tool is in scope (e.g., "Notion stores project documentation that may contain personal data references")
5. **Given** detected data and processors, **When** the agent presents findings, **Then** the user can confirm, add, or exclude items from the assessment scope with justification required for exclusions
6. **Given** scope confirmation, **When** the user confirms, **Then** the agent records the assessment scope including all system model sources in the output file

*Phase 3: Interactive Assessment*

7. **Given** scope is defined, **When** the agent begins assessment, **Then** it presents GDPR articles grouped by chapter (Principles, Rights, Controller/Processor, Transfers)
8. **Given** each control, **When** the agent assesses it, **Then** it displays: the article number, the official requirement text (from framework.yaml), a plain-language explanation, and evidence detected from all relevant system model sections
9. **Given** Art. 28 (Processor agreements), **When** the agent assesses it, **Then** it lists all sub-processors from integrations.yaml AND tooling.yaml (e.g., "Notion - US-based, requires DPA and SCCs")
10. **Given** Art. 44-49 (International transfers), **When** the agent assesses it, **Then** it flags all non-EU services from integrations.yaml, tooling.yaml, and architecture.yaml
11. **Given** auto-detected evidence, **When** the agent presents it, **Then** the user can confirm, reject, or add additional evidence
12. **Given** a control with no auto-detected evidence, **When** the agent presents it, **Then** it asks targeted questions to determine compliance status
13. **Given** user responses, **When** the agent determines status, **Then** it assigns: compliant, partial, gap, or not-applicable (with justification required for N/A)

*Phase 4: Gap Analysis & Report*

14. **Given** all controls assessed, **When** the agent summarizes, **Then** it displays: overall score, gaps by severity (grouped by: codebase, tooling, infrastructure), and priority action items
15. **Given** the summary, **When** the user confirms, **Then** the agent generates compliance artifacts in `.osk/compliance/`

---

### User Story 2 - Guided RGS Compliance Assessment (Priority: P1)

As a government project owner or RSSI, I want to run `/osk-comply rgs` to perform a guided RGS compliance assessment so that I can prepare for homologation.

**Why this priority**: RGS is mandatory for French government systems. A guided assessment aligned with ANSSI requirements accelerates the homologation process.

**Independent Test**: Can be fully tested by running `/osk-comply rgs` on a project with a completed system model and verifying the agent guides through RGS domains and generates homologation-ready documentation.

**Acceptance Scenarios**:

*Phase 1: Prerequisites Check*

1. **Given** a project without a system model, **When** I run `/osk-comply rgs`, **Then** the agent displays an error: "No system model found. Run `/osk-discover init` first."
2. **Given** an incomplete system model, **When** I run `/osk-comply rgs`, **Then** the agent warns about missing sections required for RGS assessment

*Phase 2: Security Classification & Scope (Full System Context)*

3. **Given** a complete system model, **When** I run `/osk-comply rgs`, **Then** the agent asks about the target RGS level (*, **, ***) if not already defined
4. **Given** classification level, **When** the agent proceeds, **Then** it presents the full system perimeter: codebase components, external services (integrations.yaml), operational tools (tooling.yaml), and infrastructure (architecture.yaml)
5. **Given** system perimeter, **When** the agent presents tools like Notion or GitHub, **Then** it evaluates if they are approved for the target RGS level (e.g., "Notion is not SecNumCloud certified - blocker for RGS**")
6. **Given** classification, **When** the agent presents requirements, **Then** it references ANSSI recommendations for each security function

*Phase 3: Domain-by-Domain Assessment (All Components)*

7. **Given** RGS level is set, **When** the agent begins assessment, **Then** it presents controls grouped by domain: Cryptography, Authentication, Timestamping, Electronic Signature, Confidentiality, Traceability
8. **Given** each control, **When** the agent assesses it, **Then** it displays: the control ID, the official requirement from RGS annexes, ANSSI technical recommendations, and evidence from all system model sections
9. **Given** Confidentiality domain, **When** the agent assesses data storage, **Then** it checks encryption and access controls across: database (architecture.yaml), file storage, AND operational tools (tooling.yaml)
10. **Given** Traceability domain, **When** the agent assesses logging, **Then** it verifies log retention and integrity for: application logs, infrastructure logs, AND third-party tool audit logs
11. **Given** cryptographic controls, **When** the agent assesses them, **Then** it validates against ANSSI algorithm recommendations (key sizes, approved algorithms) for all components including CI/CD secrets management
12. **Given** user responses, **When** the agent determines status, **Then** it provides homologation-specific guidance (blocker vs. recommendation)

*Phase 4: Homologation Readiness*

13. **Given** all domains assessed, **When** the agent summarizes, **Then** it displays: homologation readiness status, blocking gaps (grouped by: codebase, tooling, infrastructure), required documentation checklist
14. **Given** the summary, **When** the user confirms, **Then** the agent generates RGS compliance artifacts and homologation preparation documents

---

### User Story 3 - Framework Extension (Priority: P2)

As an OpenSecKit contributor, I want to add a new compliance framework by creating a framework.yaml file following a documented structure so that I can extend OpenSecKit without modifying core code.

**Why this priority**: Extensibility ensures OpenSecKit can grow to support NIS2, ISO 27001, SOC2, and other frameworks without architectural changes.

**Independent Test**: Can be fully tested by creating a new framework.yaml following the documented schema and verifying `/osk-comply <framework>` works without code changes.

**Acceptance Scenarios**:

1. **Given** the framework schema documentation, **When** I create a new `domaines/<framework>/framework.yaml`, **Then** the file passes schema validation
2. **Given** a valid framework.yaml, **When** I run `/osk-comply <framework>`, **Then** the agent loads the framework and conducts assessment using its controls
3. **Given** a framework with output templates, **When** I add templates to `domaines/<framework>/templates/`, **Then** the agent uses framework-specific templates for output
4. **Given** a framework with cross-references to other frameworks, **When** I define mappings in framework.yaml, **Then** the agent shows relationships during assessment
5. **Given** an invalid framework.yaml, **When** I run `/osk-comply <framework>`, **Then** the agent displays schema validation errors with specific line numbers

---

### User Story 4 - Compliance Status Dashboard (Priority: P2)

As a project owner, I want to run `/osk-comply status` to see a summary of all compliance assessments for my project so that I can track multi-framework compliance at a glance.

**Why this priority**: Projects often need to comply with multiple frameworks. A unified dashboard reduces context-switching and identifies shared gaps.

**Independent Test**: Can be fully tested by running multiple framework assessments, then running `/osk-comply status` and verifying it shows all assessments in a unified view.

**Acceptance Scenarios**:

1. **Given** no compliance assessments exist, **When** I run `/osk-comply status`, **Then** the agent displays "No compliance assessments found" and suggests running `/osk-comply <framework>`
2. **Given** one or more assessments exist, **When** I run `/osk-comply status`, **Then** the agent displays: framework name, last assessment date, overall score, gap count, and status (compliant/partial/gap)
3. **Given** multiple assessments, **When** the agent displays status, **Then** it highlights shared controls and cross-framework gaps
4. **Given** the status view, **When** the user selects a framework, **Then** the agent offers to run an update assessment or show detailed report

---

### User Story 5 - Assessment Update (Priority: P2)

As a project owner, I want to run `/osk-comply rgpd --update` to update an existing assessment based on system model changes so that I can track compliance drift without starting from scratch.

**Why this priority**: Systems evolve over time. Incremental updates save time and provide change tracking.

**Independent Test**: Can be fully tested by running an initial assessment, modifying the system model, running update, and verifying only changed controls are re-assessed.

**Acceptance Scenarios**:

1. **Given** an existing assessment and unchanged system model, **When** I run `/osk-comply rgpd --update`, **Then** the agent reports "No changes detected since last assessment"
2. **Given** an existing assessment and changed system model, **When** I run `/osk-comply rgpd --update`, **Then** the agent identifies affected controls and re-assesses only those
3. **Given** an update, **When** the agent identifies changes, **Then** it shows a diff-style view: new gaps, closed gaps, unchanged controls
4. **Given** an update, **When** completed, **Then** the agent updates the assessment file and records the change in audit_trail

---

### User Story 6 - Compliance Documentation Generation (Priority: P3)

As a project owner preparing for audit, I want to run `/osk-comply rgpd --export md` to generate a formatted compliance report so that I can share it with auditors or management.

**Why this priority**: Auditors and stakeholders need human-readable reports. Export capability bridges the gap between machine data and human consumption.

**Independent Test**: Can be fully tested by running export command and verifying a formatted document is generated.

**Acceptance Scenarios**:

1. **Given** an existing assessment, **When** I run `/osk-comply rgpd --export md`, **Then** the agent generates a Markdown compliance report
2. **Given** an export request, **When** the agent generates the report, **Then** it uses framework-specific templates that align with official guidelines
3. **Given** GDPR export, **When** the report is generated, **Then** it follows GDPR record-of-processing format requirements
4. **Given** RGS export, **When** the report is generated, **Then** it follows the dossier d'homologation structure

---

### Edge Cases

**Prerequisites**

- What happens when system model exists but is from an older schema version?
  - System MUST detect version mismatch and warn user, offering to run `/osk-discover validate` first
- What happens when framework.yaml is missing for a requested framework?
  - System MUST display "Framework '<name>' not found. Available frameworks: rgpd, rgs" with list of installed frameworks

**Assessment Process**

- What happens when user wants to skip a control?
  - System MUST allow skipping but record it as "not_assessed" with a note, not as compliant
- What happens when user provides evidence but agent cannot validate it?
  - System MUST accept user-provided evidence with "manual" assessment flag for auditor review
- What happens when assessment is interrupted mid-process?
  - System MUST save progress to a partial assessment file and allow resumption with `/osk-comply rgpd --resume`

**Framework Extension**

- What happens when a framework.yaml has controls referencing non-existent system model sections?
  - System MUST validate control data_sources against available system model sections and warn about unmappable controls
- What happens when two frameworks define the same control ID?
  - System MUST use framework-prefixed IDs internally (e.g., RGPD-5.1.a, RGS-AUTH-01) to prevent collisions

**Export**

- What happens when export is requested but assessment has critical gaps?
  - System MUST generate the report but include a prominent "DRAFT - Critical Gaps Unresolved" watermark

## Requirements *(mandatory)*

### Functional Requirements

**Framework Management (FR-001 to FR-006)**

- **FR-001**: System MUST load framework definitions from `domaines/<framework>/framework.yaml`
- **FR-002**: Framework files MUST follow a standardized schema with: metadata, categories, controls, scoring, and cross_framework_mapping sections
- **FR-003**: System MUST validate framework.yaml against schema before use and report errors with specific line numbers
- **FR-004**: System MUST support adding new frameworks without code changes (configuration-only extension)
- **FR-005**: System MUST provide `/osk-comply --list` command to show available frameworks
- **FR-006**: Framework definitions MUST include official source references (regulation article numbers, standard clause IDs)

**GDPR/RGPD Assessment - Full System Context (FR-007 to FR-017)**

- **FR-007**: System MUST provide `/osk-comply rgpd` command for GDPR compliance assessment
- **FR-008**: RGPD assessment MUST cover all mandatory chapters: Principles (Art. 5-11), Rights (Art. 12-23), Controller/Processor (Art. 24-43), Transfers (Art. 44-50)
- **FR-009**: RGPD assessment MUST detect personal data categories from system model data.yaml
- **FR-010**: RGPD assessment MUST identify data subjects and processing purposes from system model (data.yaml, business.yaml, actors.yaml)
- **FR-011**: RGPD assessment MUST evaluate sub-processor compliance from BOTH integrations.yaml (direct APIs/services) AND tooling.yaml (operational tools like Notion, Slack, Jira)
- **FR-012**: RGPD assessment MUST assess infrastructure providers from architecture.yaml as sub-processors (cloud providers, hosting, CDN)
- **FR-013**: RGPD assessment MUST flag DPIA requirement based on processing characteristics (high risk, large scale, sensitive data, systematic monitoring)
- **FR-014**: RGPD assessment MUST check international transfer mechanisms for ALL non-EU services across: integrations.yaml, tooling.yaml, and architecture.yaml
- **FR-015**: RGPD assessment MUST present scope summary showing all in-scope components grouped by: codebase, integrations, tooling, infrastructure
- **FR-016**: RGPD assessment MUST require justification when user excludes a tool or service from scope
- **FR-017**: RGPD output MUST include: assessment-rgpd.yaml, assessment-rgpd.md, gap-report-rgpd.md, sub-processor-register.md

**RGS Assessment - Full System Context (FR-018 to FR-029)**

- **FR-018**: System MUST provide `/osk-comply rgs` command for RGS compliance assessment
- **FR-019**: RGS assessment MUST cover all six domains: Cryptography, Authentication, Timestamping, Electronic Signature, Confidentiality, Traceability
- **FR-020**: RGS assessment MUST support three security levels (*, **, ***) with level-appropriate control subsets
- **FR-021**: RGS assessment MUST validate cryptographic implementations against ANSSI recommendations for ALL components including CI/CD (tooling.yaml) and infrastructure (architecture.yaml)
- **FR-022**: RGS assessment MUST evaluate authentication mechanisms against RGS authentication levels (1-4) across codebase AND operational tools
- **FR-023**: RGS assessment MUST check logging/traceability against RGS retention requirements for: application logs, infrastructure logs, AND third-party tool audit logs (from tooling.yaml)
- **FR-024**: RGS assessment MUST verify tool certification status (SecNumCloud, HDS, ISO 27001) for all services in integrations.yaml, tooling.yaml, and architecture.yaml
- **FR-025**: RGS assessment MUST flag tools not approved for the target RGS level as blockers (e.g., non-SecNumCloud tools for RGS**)
- **FR-026**: RGS assessment MUST assess data classification enforcement across ALL storage locations including operational tools
- **FR-027**: RGS assessment MUST include homologation readiness checklist with sections for: codebase, tooling, infrastructure
- **FR-028**: RGS output MUST follow dossier d'homologation structure for audit compatibility
- **FR-029**: RGS output MUST include: assessment-rgs.yaml, assessment-rgs.md, homologation-checklist.md, system-perimeter.md

**Interactive Guidance (FR-030 to FR-036)**

- **FR-030**: Assessment agent MUST display official requirement text from regulation source for each control
- **FR-031**: Assessment agent MUST provide plain-language explanation of each requirement
- **FR-032**: Assessment agent MUST show auto-detected evidence from ALL system model sections before asking questions
- **FR-033**: Assessment agent MUST allow user to confirm, reject, or supplement auto-detected evidence
- **FR-034**: Assessment agent MUST ask targeted questions only when evidence is insufficient
- **FR-035**: Assessment agent MUST allow skipping controls with mandatory documentation as "not_assessed"
- **FR-036**: Assessment agent MUST provide progress indication (e.g., "Control 15/47 - Chapter III: Rights")

**Output Generation (FR-037 to FR-044)**

- **FR-037**: System MUST generate compliance assessment in YAML format following `templates/schemas/compliance-assessment.yaml` schema
- **FR-038**: System MUST generate human-readable Markdown summary alongside YAML output
- **FR-039**: System MUST support framework-specific output templates in `domaines/<framework>/templates/`
- **FR-040**: Output MUST include: overall score, control-by-control status, gaps with severity (grouped by codebase/tooling/infrastructure), prioritized action items
- **FR-041**: Output MUST preserve audit trail of assessment changes between runs
- **FR-042**: Output MUST link each control assessment to evidence sources (system model section paths, user confirmations)
- **FR-043**: System MUST support export to Markdown format via `--export md` flag
- **FR-044**: Export templates MUST align with framework-specific documentation requirements (GDPR records format, RGS homologation dossier)

**Cross-Framework Features (FR-045 to FR-048)**

- **FR-045**: System MUST provide `/osk-comply status` command for multi-framework dashboard
- **FR-046**: Framework definitions MUST support cross_framework_mapping to show control relationships
- **FR-047**: Status dashboard MUST identify shared controls that satisfy multiple frameworks
- **FR-048**: Status dashboard MUST highlight cross-framework gaps (one fix addresses multiple frameworks)

**Update & Resume (FR-049 to FR-052)**

- **FR-049**: System MUST provide `--update` flag to re-assess only changed controls since last assessment
- **FR-050**: Update MUST detect changes by comparing current system model hash (all sections) with recorded hash
- **FR-051**: System MUST provide `--resume` flag to continue interrupted assessments from last saved progress
- **FR-052**: Resume MUST restore exact state including user responses already provided

### Key Entities

- **System Context**: The complete picture of a system from the Discover phase, encompassing all YAML files in `.osk/system-model/`. This is the primary input for compliance assessment and includes: data categories, integrations, tooling, architecture, actors, trust zones, and security controls
- **Framework Definition**: YAML file defining a compliance framework's metadata, controls, scoring method, and cross-references. Located in `domaines/<framework>/framework.yaml`
- **Control**: A single requirement from a regulation or standard (e.g., GDPR Article 30, RGS AUTH-02). Contains ID, official text, evidence types, and applicability conditions. Controls can draw evidence from multiple system model sections
- **Compliance Assessment**: The output of an assessment run, containing control-by-control status, evidence, gaps, and action items. Stored in `.osk/compliance/assessment-<framework>.yaml`
- **Assessment Scope**: The subset of system context being assessed. By default includes all components but users can exclude items with justification. Recorded in the assessment output
- **Evidence**: Documentation or system configuration that demonstrates compliance with a control. Can be auto-detected from any system model section or user-provided
- **Sub-Processor**: Any third-party service that processes data on behalf of the organization. Includes direct integrations (APIs), operational tools (Notion, Slack), and infrastructure providers (AWS, OVH). All must be assessed for GDPR Art. 28 and RGS certification requirements
- **Gap**: A control that is not satisfied or only partially satisfied. Includes severity rating, remediation guidance, and categorization (codebase/tooling/infrastructure)
- **Action Item**: A prioritized task to address a compliance gap. Linked to controls and assigned priority based on severity
- **Output Template**: Tera template that transforms assessment YAML into framework-specific documentation format

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can complete a full GDPR assessment (all controls) in under 45 minutes for projects with up to 20 data processing activities
- **SC-002**: Users can complete a full RGS assessment (all domains at RGS** level) in under 30 minutes
- **SC-003**: Assessment output passes schema validation 100% of the time
- **SC-004**: A new framework can be added by creating framework.yaml only (no code changes required) and passes validation
- **SC-005**: Assessment correctly auto-detects at least 60% of evidence from system model. Measurement: For a test project with 10 known controls that have evidence in system model, at least 6 controls should have auto-detected evidence
- **SC-006**: Generated compliance reports are immediately usable for audit preparation without manual reformatting
- **SC-007**: Cross-framework status correctly identifies shared controls when frameworks have defined mappings
- **SC-008**: Assessment update completes in under 5 minutes for typical changes (1-5 affected controls)
- **SC-009**: Interrupted assessments can be resumed with 100% state preservation (no lost user responses)

### Assumptions

- System model has been completed via `/osk-discover init` before compliance assessment
- Users have basic familiarity with the compliance framework being assessed (know what GDPR or RGS is)
- Framework definitions (framework.yaml) contain accurate and up-to-date regulation requirements
- AI agents executing prompts can read and parse YAML files from the file system
- Projects subject to GDPR/RGS have documented their data processing in the system model data.yaml section
