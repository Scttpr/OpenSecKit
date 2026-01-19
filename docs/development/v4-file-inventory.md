# OpenSecKit V4 - File Inventory

Complete list of templates, prompts, and scripts needed for V4.

**Legend:**
- ✅ EXISTS - Already exists, may need minor updates
- 🔄 REFACTOR - Exists but needs significant changes for V4
- 🆕 NEW - Needs to be created
- 📦 MOVE - Exists but needs to move to new location

**Reference:** See `v4-template-architecture.md` for the knowledge/ vs templates/ separation.

---

## 0. NEW DIRECTORY STRUCTURE

```
OpenSecKit/
├── knowledge/                    # NEW: Static reference material
│   ├── methodologies/            # How-to guides (moved from templates/01-07)
│   ├── libraries/                # Threat/control libraries
│   └── examples/                 # Concrete examples
│
├── templates/                    # RESTRUCTURED: Generative only
│   ├── schemas/                  # YAML data contracts
│   ├── outputs/                  # Markdown templates
│   ├── data/                     # NEW: YAML output templates
│   ├── reports/                  # Terminal reports
│   └── agents/                   # Tera agent transforms
│
├── prompts/                      # Agent prompts
├── frameworks/                     # Compliance domains
├── cli/                          # Rust CLI
└── docs/                         # Documentation
```

---

## 1. KNOWLEDGE (`knowledge/`) - NEW

### Methodologies (move from templates/01-07)
| File | Status | Source |
|------|--------|--------|
| `methodologies/threat-modeling/stride-guide.md` | 📦 MOVE | `templates/01-threat-modeling/stride-threat-model-template-planning.md` |
| `methodologies/threat-modeling/attack-trees-guide.md` | 📦 MOVE | `templates/01-threat-modeling/attack-tree-template-planning.md` |
| `methodologies/threat-modeling/dfd-guide.md` | 📦 MOVE | `templates/01-threat-modeling/data-flow-diagram-template-design.md` |
| `methodologies/risk-analysis/scoring-guide.md` | 📦 MOVE | `templates/02-risk-analysis/risk-scoring-template-planning.md` |
| `methodologies/risk-analysis/register-guide.md` | 📦 MOVE | `templates/02-risk-analysis/risk-register-template-all.md` |
| `methodologies/security-requirements/asvs-guide.md` | 📦 MOVE | `templates/03-security-requirements/owasp-asvs-checklist-design.md` |
| `methodologies/security-requirements/auth-guide.md` | 📦 MOVE | `templates/03-security-requirements/authentication-requirements-template-design.md` |
| `methodologies/security-requirements/authz-guide.md` | 📦 MOVE | `templates/03-security-requirements/authorization-requirements-template-design.md` |
| `methodologies/security-requirements/encryption-guide.md` | 📦 MOVE | `templates/03-security-requirements/encryption-requirements-template-design.md` |
| `methodologies/security-testing/sast-guide.md` | 📦 MOVE | `templates/04-security-testing/sast-integration-guide-implementation.md` |
| `methodologies/security-testing/dast-guide.md` | 📦 MOVE | `templates/04-security-testing/dast-integration-guide-implementation.md` |
| `methodologies/security-testing/sca-guide.md` | 📦 MOVE | `templates/04-security-testing/sca-dependency-scanning.md` |
| `methodologies/secrets-management/rotation-guide.md` | 📦 MOVE | `templates/05-secrets-management/secrets-rotation-policy-template.md` |
| `methodologies/secrets-management/detection-guide.md` | 📦 MOVE | `templates/05-secrets-management/secrets-detection-setup.md` |
| `methodologies/secrets-management/vault-guide.md` | 📦 MOVE | `templates/05-secrets-management/vault-integration-guide.md` |
| `methodologies/audit-logging/logging-guide.md` | 📦 MOVE | `templates/06-audit-logging/logging-requirements-template-design.md` |
| `methodologies/audit-logging/siem-guide.md` | 📦 MOVE | `templates/06-audit-logging/siem-selection-guide.md` |
| `methodologies/audit-logging/alerts-guide.md` | 📦 MOVE | `templates/06-audit-logging/security-alert-rules-template.md` |
| `methodologies/patch-management/sla-guide.md` | 📦 MOVE | `templates/07-patch-management/patch-sla-policy-template.md` |
| `methodologies/patch-management/scanning-guide.md` | 📦 MOVE | `templates/07-patch-management/dependency-scanning-guide-all.md` |
| `methodologies/patch-management/emergency-guide.md` | 📦 MOVE | `templates/07-patch-management/emergency-patching-procedure.md` |
| `methodologies/patch-management/prioritization-guide.md` | 📦 MOVE | `templates/07-patch-management/vulnerability-prioritization-matrix.md` |

### Libraries (new)
| File | Status | Purpose |
|------|--------|---------|
| `libraries/threats/stride-common.yaml` | 🆕 NEW | Common STRIDE threats |
| `libraries/threats/api-threats.yaml` | 🆕 NEW | API-specific threats |
| `libraries/threats/auth-threats.yaml` | 🆕 NEW | Authentication threats |
| `libraries/threats/data-threats.yaml` | 🆕 NEW | Data handling threats |
| `libraries/threats/injection-threats.yaml` | 🆕 NEW | Injection attack threats |
| `libraries/controls/owasp-asvs-v4.yaml` | 🆕 NEW | OWASP ASVS v4 controls |
| `libraries/controls/cis-controls-v8.yaml` | 🆕 NEW | CIS Controls v8 |
| `libraries/controls/common-controls.yaml` | 🆕 NEW | Common security controls |
| `libraries/patterns/secure-defaults.yaml` | 🆕 NEW | Secure default patterns |
| `libraries/patterns/anti-patterns.yaml` | 🆕 NEW | Common security mistakes |

### Examples (move from templates)
| File | Status | Source |
|------|--------|--------|
| `examples/ecommerce/stride-analysis.md` | 📦 MOVE | `templates/01-threat-modeling/_exemple-ecommerce-stride.md` |
| `examples/ecommerce/risk-register.md` | 📦 MOVE | `templates/02-risk-analysis/_exemple-ecommerce-risque.md` |
| `examples/ecommerce/requirements.md` | 📦 MOVE | `templates/03-security-requirements/_example-ecommerce-requirements.md` |
| `examples/ecommerce/testing.md` | 📦 MOVE | `templates/04-security-testing/_example-ecommerce-testing.md` |
| `examples/ecommerce/secrets.md` | 📦 MOVE | `templates/05-secrets-management/_example-ecommerce-secrets.md` |
| `examples/ecommerce/logging.md` | 📦 MOVE | `templates/06-audit-logging/_example-ecommerce-logging.md` |
| `examples/ecommerce/patching.md` | 📦 MOVE | `templates/07-patch-management/_example-ecommerce-patching.md` |
| `examples/saas/full-example.md` | 🆕 NEW | SaaS security example |
| `examples/api/full-example.md` | 🆕 NEW | API security example |

---

## 2. SCHEMAS (`templates/schemas/`)

### Existing (keep)
| File | Status | Notes |
|------|--------|-------|
| `risk-entry.yaml` | ✅ EXISTS | Add `model_sections` field |
| `risk-register.yaml` | ✅ EXISTS | Add compliance section |
| `requirement-entry.yaml` | ✅ EXISTS | Keep as-is |
| `task-entry.yaml` | ✅ EXISTS | Keep as-is |
| `test-strategy.yaml` | ✅ EXISTS | Keep as-is |
| `secret-entry.yaml` | ✅ EXISTS | Keep as-is |
| `logging-event.yaml` | ✅ EXISTS | Keep as-is |
| `patch-entry.yaml` | ✅ EXISTS | Keep as-is |
| `plan-action.yaml` | ✅ EXISTS | Keep as-is |
| `feature-entry.yaml` | ✅ EXISTS | Keep as-is |
| `rgpd-treatment.yaml` | ✅ EXISTS | Keep as-is |

### New for V4
| File | Status | Purpose |
|------|--------|---------|
| `system-model-index.yaml` | ✅ DONE | Lightweight index (~200 lines) |
| `system-model-full.yaml` | ✅ DONE | Full schema reference |
| `system-model-business.yaml` | 🆕 NEW | Business section schema |
| `system-model-architecture.yaml` | 🆕 NEW | Architecture section schema |
| `system-model-data.yaml` | 🆕 NEW | Data inventory section schema |
| `system-model-actors.yaml` | 🆕 NEW | Actors section schema |
| `system-model-boundaries.yaml` | 🆕 NEW | Trust boundaries section schema |
| `system-model-integrations.yaml` | 🆕 NEW | Integrations section schema |
| `system-model-controls.yaml` | 🆕 NEW | Security controls section schema |
| `system-model-gaps.yaml` | 🆕 NEW | Discovery gaps section schema |
| `compliance-assessment.yaml` | ✅ DONE | Part 2 output schema |
| `feature-constitution.yaml` | ❌ REMOVED | Abandoned - use security-spec.yaml instead |

---

## 2. PROMPTS (`prompts/`)

### Part 1: Discover
| File | Status | Purpose |
|------|--------|---------|
| `osk-discover-init.md` | 🆕 NEW | Full discovery, generates system-model/ |
| `osk-discover-scan.md` | 🆕 NEW | Focused scan (--architecture, --data, etc.) |
| `osk-discover-update.md` | 🆕 NEW | Incremental update, detect drift |
| `osk-discover-validate.md` | 🆕 NEW | Validate model against code |

### Part 2: Comply
| File | Status | Purpose |
|------|--------|---------|
| `osk-comply.md` | 🆕 NEW | Base compliance prompt |
| `osk-comply-rgpd.md` | 🔄 REFACTOR | From `osk-rgpd.md`, consume system-model |
| `osk-comply-rgs.md` | 🔄 REFACTOR | From `osk-rgs.md`, consume system-model |
| `osk-comply-nis2.md` | 🆕 NEW | NIS2 compliance |
| `osk-comply-iso27001.md` | 🆕 NEW | ISO 27001 compliance |
| `osk-comply-soc2.md` | 🆕 NEW | SOC 2 compliance |
| `osk-comply-gap-analysis.md` | 🆕 NEW | Cross-framework gap analysis |
| `osk-comply-evidence.md` | 🆕 NEW | Evidence collection |

### Part 3: Secure
| File | Status | Purpose |
|------|--------|---------|
| `osk-secure-constitute.md` | ❌ REMOVED | Abandoned - principles integrated into specify |
| `osk-secure-specify.md` | 🔄 REFACTOR | Merge analyze+specify+harden |
| `osk-secure-clarify.md` | 🆕 NEW | Clarification workflow |
| `osk-secure-plan.md` | 🔄 REFACTOR | From `osk-plan.md` |
| `osk-secure-tasks.md` | 🔄 REFACTOR | From `osk-tasks.md` |
| `osk-secure-implement.md` | 🔄 REFACTOR | From `osk-implement.md` |

### Utilities (keep with updates)
| File | Status | Purpose |
|------|--------|---------|
| `osk-configure.md` | 🔄 REFACTOR | Integrate with discover |
| `osk-baseline.md` | 🔄 REFACTOR | Integrate with discover |
| `osk-dashboard.md` | 🔄 REFACTOR | Show system-model + compliance |
| `osk-incident.md` | ✅ EXISTS | Keep, minor updates |
| `osk-pca-pra.md` | ✅ EXISTS | Keep, minor updates |

### Deprecated (V3 compatibility aliases)
| File | Status | Notes |
|------|--------|-------|
| `osk-analyze.md` | 🔄 ALIAS | Redirect to osk-secure-specify |
| `osk-specify.md` | 🔄 ALIAS | Redirect to osk-secure-specify |
| `osk-harden.md` | 🔄 ALIAS | Redirect to osk-secure-specify |
| `osk-plan.md` | 🔄 ALIAS | Redirect to osk-secure-plan |
| `osk-tasks.md` | 🔄 ALIAS | Redirect to osk-secure-tasks |
| `osk-implement.md` | 🔄 ALIAS | Redirect to osk-secure-implement |
| `osk-rgpd.md` | 🔄 ALIAS | Redirect to osk-comply-rgpd |
| `osk-rgs.md` | 🔄 ALIAS | Redirect to osk-comply-rgs |

---

## 3. AGENT TEMPLATES (`templates/agents/`)

| File | Status | Purpose |
|------|--------|---------|
| `claude-code.tera` | 🔄 REFACTOR | Group by part, add model_sections |
| `copilot.tera` | 🔄 REFACTOR | Update for V4 structure |
| `cursor.tera` | 🔄 REFACTOR | Update for V4 structure |
| `gemini.tera` | 🔄 REFACTOR | Update for V4 structure |
| `AGENTS.md.tera` | 🔄 REFACTOR | Document 3-part workflow |

---

## 4. DATA TEMPLATES (`templates/data/`) - NEW

YAML output templates for structured data generation.

### Discover (system-model sections)
| File | Status | Purpose |
|------|--------|---------|
| `data/discover/index.yaml` | 🆕 NEW | System model index template |
| `data/discover/business.yaml` | 🆕 NEW | Business context template |
| `data/discover/architecture.yaml` | 🆕 NEW | Architecture template |
| `data/discover/data.yaml` | 🆕 NEW | Data inventory template |
| `data/discover/actors.yaml` | 🆕 NEW | Actors template |
| `data/discover/boundaries.yaml` | 🆕 NEW | Trust boundaries template |
| `data/discover/integrations.yaml` | 🆕 NEW | Integrations template |
| `data/discover/controls.yaml` | 🆕 NEW | Security controls template |
| `data/discover/gaps.yaml` | 🆕 NEW | Discovery gaps template |

### Comply (assessments)
| File | Status | Purpose |
|------|--------|---------|
| `data/comply/assessment.yaml` | 🆕 NEW | Compliance assessment template |
| `data/comply/gap-report.yaml` | 🆕 NEW | Gap analysis data template |
| `data/comply/evidence.yaml` | 🆕 NEW | Evidence collection template |

### Secure (specs)
| File | Status | Purpose |
|------|--------|---------|
| `data/secure/constitution.yaml` | ❌ REMOVED | Abandoned - use security-spec templates |
| `data/secure/threats.yaml` | 🆕 NEW | Threats data template |
| `data/secure/risks.yaml` | 🆕 NEW | Risks data template |
| `data/secure/requirements.yaml` | 🆕 NEW | Requirements data template |
| `data/secure/tasks.yaml` | 🆕 NEW | Tasks data template |

### Core
| File | Status | Purpose |
|------|--------|---------|
| `data/risk-register.yaml` | 🆕 NEW | Risk register template |

---

## 5. OUTPUT TEMPLATES (`templates/outputs/`)

Markdown templates for human-readable documentation.

### Existing (keep)
| File | Status |
|------|--------|
| `threats.md.tmpl` | ✅ EXISTS |
| `risks.md.tmpl` | ✅ EXISTS |
| `requirements.md.tmpl` | ✅ EXISTS |
| `testing.md.tmpl` | ✅ EXISTS |
| `hardening.md.tmpl` | ✅ EXISTS |
| `plan.md.tmpl` | ✅ EXISTS |
| `tasks.md.tmpl` | ✅ EXISTS |
| `context.md.tmpl` | ✅ EXISTS |
| `constitution.md.tmpl` | ❌ REMOVED | V3 legacy, deleted |
| `dashboard.md.tmpl` | 🔄 REFACTOR |
| `features.yaml.tmpl` | ✅ EXISTS |
| `stride-system.md.tmpl` | ✅ EXISTS |

### New for V4
| File | Status | Purpose |
|------|--------|---------|
| `system-model-index.yaml.tmpl` | 🆕 NEW | Index file template |
| `system-model-business.yaml.tmpl` | 🆕 NEW | Business section template |
| `system-model-architecture.yaml.tmpl` | 🆕 NEW | Architecture section template |
| `system-model-data.yaml.tmpl` | 🆕 NEW | Data section template |
| `system-model-actors.yaml.tmpl` | 🆕 NEW | Actors section template |
| `system-model-boundaries.yaml.tmpl` | 🆕 NEW | Trust section template |
| `system-model-integrations.yaml.tmpl` | 🆕 NEW | Integrations section template |
| `system-model-controls.yaml.tmpl` | 🆕 NEW | Security section template |
| `system-model-gaps.yaml.tmpl` | 🆕 NEW | Gaps section template |
| `compliance-assessment.yaml.tmpl` | 🆕 NEW | Compliance output template |
| `feature-constitution.md.tmpl` | ❌ REMOVED | Abandoned - use security-spec templates |
| `feature-constitution.yaml.tmpl` | ❌ REMOVED | Abandoned - use security-spec templates |

---

## 5. TERMINAL REPORTS (`templates/reports/`)

### Existing (keep)
| File | Status |
|------|--------|
| `configure-report.txt` | ✅ EXISTS |
| `baseline-report.txt` | ✅ EXISTS |
| `analyze-report.txt` | ✅ EXISTS |
| `specify-report.txt` | ✅ EXISTS |
| `harden-report.txt` | ✅ EXISTS |
| `plan-report.txt` | ✅ EXISTS |
| `tasks-report.txt` | ✅ EXISTS |
| `implement-report.txt` | ✅ EXISTS |
| `dashboard-report.txt` | 🔄 REFACTOR |
| `rgpd-audit-report.txt` | ✅ EXISTS |
| `rgs-report.txt` | ✅ EXISTS |
| `pca-pra-report.txt` | ✅ EXISTS |

### New for V4
| File | Status | Purpose |
|------|--------|---------|
| `discover-init-report.txt` | 🆕 NEW | Discovery summary |
| `discover-scan-report.txt` | 🆕 NEW | Scan results |
| `discover-update-report.txt` | 🆕 NEW | Drift detection |
| `discover-validate-report.txt` | 🆕 NEW | Validation results |
| `comply-report.txt` | 🆕 NEW | Compliance assessment summary |
| `comply-gap-report.txt` | 🆕 NEW | Gap analysis summary |
| `secure-specify-report.txt` | 🆕 NEW | Security specification summary |

---

## 6. CLI RUST FILES (`cli/src/`)

### Existing (keep/update)
| File | Status | Notes |
|------|--------|-------|
| `main.rs` | 🔄 REFACTOR | Add new subcommands |
| `args.rs` | 🔄 REFACTOR | Add discover/comply/secure args |
| `config.rs` | ✅ EXISTS | Minor updates |
| `stack.rs` | 🔄 REFACTOR | Enhance detection |
| `agents.rs` | ✅ EXISTS | Add capabilities field |
| `prompts.rs` | 🔄 REFACTOR | Add model_sections parsing |
| `github.rs` | ✅ EXISTS | Keep as-is |
| `commands/mod.rs` | 🔄 REFACTOR | Register new commands |
| `commands/init.rs` | ✅ EXISTS | Minor updates |
| `commands/check.rs` | ✅ EXISTS | Keep as-is |
| `commands/scaffold.rs` | ✅ EXISTS | Keep as-is |
| `commands/update.rs` | ✅ EXISTS | Keep as-is |
| `commands/validate.rs` | 🔄 REFACTOR | Add model validation |
| `commands/ingest.rs` | ✅ EXISTS | Keep as-is |
| `utils/mod.rs` | ✅ EXISTS | Keep as-is |
| `utils/git.rs` | ✅ EXISTS | Keep as-is |
| `utils/yaml.rs` | ✅ EXISTS | Keep as-is |
| `utils/template.rs` | ✅ EXISTS | Keep as-is |
| `utils/counter.rs` | ✅ EXISTS | Keep as-is |

### New for V4
| File | Status | Purpose |
|------|--------|---------|
| `commands/discover/mod.rs` | 🆕 NEW | Discover subcommand module |
| `commands/discover/init.rs` | 🆕 NEW | osk discover init |
| `commands/discover/scan.rs` | 🆕 NEW | osk discover scan |
| `commands/discover/update.rs` | 🆕 NEW | osk discover update |
| `commands/discover/validate.rs` | 🆕 NEW | osk discover validate |
| `commands/comply/mod.rs` | 🆕 NEW | Comply subcommand module |
| `commands/comply/assess.rs` | 🆕 NEW | osk comply <framework> |
| `commands/comply/gap_analysis.rs` | 🆕 NEW | osk comply gap-analysis |
| `commands/comply/evidence.rs` | 🆕 NEW | osk comply evidence |
| `commands/secure/mod.rs` | 🆕 NEW | Secure subcommand module |
| `commands/secure/constitute.rs` | 🆕 NEW | osk secure constitute |
| `commands/secure/specify.rs` | 🆕 NEW | osk secure specify |
| `commands/secure/clarify.rs` | 🆕 NEW | osk secure clarify |
| `commands/secure/plan.rs` | 🆕 NEW | osk secure plan |
| `commands/secure/tasks.rs` | 🆕 NEW | osk secure tasks |
| `commands/secure/implement.rs` | 🆕 NEW | osk secure implement |
| `commands/migrate.rs` | 🆕 NEW | V3 → V4 migration |
| `discover/mod.rs` | 🆕 NEW | Discovery engine module |
| `discover/architecture.rs` | 🆕 NEW | Architecture extraction |
| `discover/data.rs` | 🆕 NEW | Data discovery |
| `discover/actors.rs` | 🆕 NEW | Actor mapping |
| `discover/trust.rs` | 🆕 NEW | Trust boundary detection |
| `discover/integrations.rs` | 🆕 NEW | Integration detection |
| `discover/business.rs` | 🆕 NEW | Business context inference |
| `comply/mod.rs` | 🆕 NEW | Compliance engine module |
| `comply/frameworks/mod.rs` | 🆕 NEW | Framework loader |
| `comply/frameworks/loader.rs` | 🆕 NEW | Load framework definitions |
| `comply/mapper.rs` | 🆕 NEW | Control mapping engine |
| `comply/cross_framework.rs` | 🆕 NEW | Cross-framework analysis |
| `comply/system_model_reader.rs` | 🆕 NEW | Read system-model sections |
| `secure/mod.rs` | 🆕 NEW | Secure engine module |
| `secure/system_model_reader.rs` | 🆕 NEW | Read system-model sections |
| `secure/system_model_updater.rs` | 🆕 NEW | Update system-model |

---

## 7. DOMAIN FILES (`frameworks/`)

### RGPD (update)
| File | Status |
|------|--------|
| `rgpd/framework.yaml` | 🆕 NEW |
| `rgpd/controls/*.yaml` | 🆕 NEW |
| `rgpd/skeleton.yaml` | ✅ EXISTS |
| `rgpd/templates/*` | ✅ EXISTS |

### RGS (update)
| File | Status |
|------|--------|
| `rgs/framework.yaml` | 🆕 NEW |
| `rgs/controls/*.yaml` | 🆕 NEW |
| `rgs/skeleton.yaml` | ✅ EXISTS |
| `rgs/templates/*` | ✅ EXISTS |

### NIS2 (complete)
| File | Status |
|------|--------|
| `nis2/framework.yaml` | 🆕 NEW |
| `nis2/controls/*.yaml` | 🆕 NEW |
| `nis2/skeleton.yaml` | 🆕 NEW |
| `nis2/templates/*` | ✅ EXISTS (partial) |

### ISO 27001 (new)
| File | Status |
|------|--------|
| `iso27001/README.md` | 🆕 NEW |
| `iso27001/framework.yaml` | 🆕 NEW |
| `iso27001/controls/*.yaml` | 🆕 NEW |
| `iso27001/skeleton.yaml` | 🆕 NEW |
| `iso27001/templates/soa.md` | 🆕 NEW |
| `iso27001/templates/isms-scope.md` | 🆕 NEW |

### SOC 2 (new)
| File | Status |
|------|--------|
| `soc2/README.md` | 🆕 NEW |
| `soc2/framework.yaml` | 🆕 NEW |
| `soc2/controls/*.yaml` | 🆕 NEW |
| `soc2/skeleton.yaml` | 🆕 NEW |
| `soc2/templates/tsc-mapping.md` | 🆕 NEW |

---

## 8. CONFIGURATION FILES

| File | Status | Purpose |
|------|--------|---------|
| `registry.toml` | 🔄 REFACTOR | Add V4 commands |
| `registry-v4.toml` | 🆕 NEW | V4 registry (can replace) |
| `agents.toml` | 🔄 REFACTOR | Add capabilities, part field |

---

## 9. DOCUMENTATION (`docs/`)

| File | Status | Purpose |
|------|--------|---------|
| `development/architecture-v4-proposal.md` | ✅ DONE | V4 architecture |
| `development/v4-implementation-plan.md` | ✅ DONE | Implementation plan |
| `development/v4-tasks.yaml` | ✅ DONE | Task tracking |
| `development/v4-file-inventory.md` | ✅ DONE | This file |
| `development/v3-deprecation.md` | 🆕 NEW | Deprecation guide |
| `migration/v3-to-v4.md` | 🆕 NEW | Migration guide |
| `concepts/discover.md` | 🆕 NEW | Part 1 concepts |
| `concepts/comply.md` | 🆕 NEW | Part 2 concepts |
| `concepts/secure.md` | 🆕 NEW | Part 3 concepts |
| `commands/osk-discover-*.md` | 🆕 NEW | Discover command docs |
| `commands/osk-comply*.md` | 🆕 NEW | Comply command docs |
| `commands/osk-secure-*.md` | 🆕 NEW | Secure command docs |
| `reference/file-structure.md` | 🔄 REFACTOR | Update for V4 |

---

## 10. SCRIPTS (`scripts/`)

| File | Status | Purpose |
|------|--------|---------|
| `test-local.sh` | ✅ EXISTS | Keep, add V4 tests |
| `check-links.sh` | ✅ EXISTS | Keep as-is |
| `generate-v4-schemas.sh` | 🆕 NEW | Generate split schemas |
| `migrate-v3-to-v4.sh` | 🆕 NEW | Migration helper |

---

## SUMMARY

| Category | Existing | Refactor | New | Total |
|----------|----------|----------|-----|-------|
| Schemas | 11 | 0 | 12 | 23 |
| Prompts | 5 | 14 | 12 | 31 |
| Agent Templates | 0 | 5 | 0 | 5 |
| Output Templates | 11 | 1 | 12 | 24 |
| Reports | 11 | 1 | 7 | 19 |
| CLI Rust | 18 | 8 | 35 | 61 |
| Domains | ~20 | 2 | ~25 | ~47 |
| Config | 0 | 2 | 1 | 3 |
| Docs | 4 | 1 | 10 | 15 |
| Scripts | 2 | 0 | 2 | 4 |
| **TOTAL** | **~82** | **~34** | **~116** | **~232** |

---

## PRIORITY ORDER

### Phase 1 (Foundation)
1. `registry-v4.toml`
2. `agents.toml` updates
3. CLI command structure (`commands/discover/`, `commands/comply/`, `commands/secure/`)
4. Agent templates updates

### Phase 2 (Discover)
1. System model schemas (split)
2. Discovery engine modules
3. `osk-discover-*.md` prompts
4. Output templates for system-model

### Phase 3 (Comply)
1. Framework YAML definitions
2. Compliance engine
3. `osk-comply*.md` prompts
4. Domain updates (RGPD, RGS, NIS2)

### Phase 4 (Secure)
1. `osk-secure-constitute.md`
2. `osk-secure-specify.md` (merge)
3. Remaining secure prompts
4. V3 aliases for backward compatibility

### Phase 5-6 (Integration & Migration)
1. Integration testing
2. Migration command
3. Documentation
4. New frameworks (ISO 27001, SOC 2)
