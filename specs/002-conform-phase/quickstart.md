# Quickstart: Conformity Phase v4

## Prerequisites

Before running compliance assessments, ensure:

1. **OSK initialized**: Run `osk init` if not done
2. **System model complete**: Run `/osk-discover init` to build system model
3. **System model validated**: Run `/osk-discover validate` to check completeness

```bash
# Check prerequisites
ls .osk/system-model/

# Expected files:
# index.yaml, data.yaml, integrations.yaml, tooling.yaml,
# architecture.yaml, actors.yaml, security.yaml, ...
```

---

## GDPR Compliance Assessment

### Run Full Assessment

```
/osk-comply rgpd
```

The agent will:
1. Check prerequisites (system model exists)
2. Present scope summary (data, sub-processors, tooling, infrastructure)
3. Ask you to confirm scope
4. Walk through GDPR articles by chapter
5. Show auto-detected evidence for each control
6. Ask you to confirm, reject, or add evidence
7. Generate compliance report

### Assessment with Options

```
# Resume interrupted assessment
/osk-comply rgpd --resume

# Update existing assessment (only changed controls)
/osk-comply rgpd --update

# Export to Markdown
/osk-comply rgpd --export md
```

### Output Files

After assessment, find outputs in `.osk/compliance/`:

```
.osk/compliance/
├── assessment-rgpd.yaml      # Machine-readable assessment
├── assessment-rgpd.md        # Human-readable report
├── gap-report-rgpd.md        # Gaps with remediation
└── sub-processor-register.md # Art. 28 processor list
```

---

## RGS Compliance Assessment

### Run Full Assessment

```
/osk-comply rgs
```

The agent will:
1. Check prerequisites
2. Ask for target RGS level (*, **, ***) if not defined
3. Present full system perimeter (including tooling)
4. Check tool certifications (SecNumCloud, etc.)
5. Walk through RGS domains (B2-B7)
6. Validate against ANSSI recommendations
7. Generate homologation-ready documentation

### Assessment with Options

```
# Resume interrupted assessment
/osk-comply rgs --resume

# Update existing assessment
/osk-comply rgs --update

# Export to Markdown
/osk-comply rgs --export md
```

### Output Files

```
.osk/compliance/
├── assessment-rgs.yaml        # Machine-readable assessment
├── assessment-rgs.md          # Human-readable report
├── homologation-checklist.md  # Pre-certification checklist
└── system-perimeter.md        # Full system boundary
```

---

## Multi-Framework Dashboard

View all compliance assessments at a glance:

```
/osk-comply status
```

Output:
```
┌─────────────────────────────────────────────────────────────┐
│ Framework   │ Last Assessment │ Score │ Gaps │ Status      │
├─────────────┼─────────────────┼───────┼──────┼─────────────┤
│ RGPD        │ 2026-01-15      │ 78%   │ 5    │ partial     │
│ RGS**       │ 2026-01-14      │ 92%   │ 2    │ compliant   │
└─────────────────────────────────────────────────────────────┘

Cross-framework: 3 shared controls, 1 gap affects both frameworks
```

---

## Available Frameworks

List installed frameworks:

```
/osk-comply --list
```

Output:
```
Available compliance frameworks:
  rgpd     - Règlement Général sur la Protection des Données (EU 2016/679)
  rgs      - Référentiel Général de Sécurité (ANSSI v2.0)

Coming soon:
  iso27001 - ISO/IEC 27001:2022
  nis2     - NIS2 Directive
  soc2     - SOC 2 Type II
```

---

## Adding New Frameworks

To add a new compliance framework:

1. Create framework directory:
   ```
   domaines/<framework-id>/
   ├── framework.yaml    # Required: controls, scoring
   └── templates/        # Optional: custom output templates
   ```

2. Follow the schema in `specs/002-conform-phase/contracts/framework-schema.yaml`

3. Validate your framework:
   ```
   osk validate --schema framework domaines/<framework-id>/framework.yaml
   ```

4. Run assessment:
   ```
   /osk-comply <framework-id>
   ```

---

## Understanding Assessment Scope

The compliance assessment covers **all components** from your system model:

| Source | What's Assessed | Example |
|--------|----------------|---------|
| `data.yaml` | Personal data categories, processing activities | User profiles, health data |
| `integrations.yaml` | External APIs and services | Stripe, Sendgrid |
| `tooling.yaml` | Operational tools | Notion, Slack, GitHub Actions |
| `architecture.yaml` | Infrastructure providers | AWS, OVH, Cloudflare |
| `actors.yaml` | Users and access rights | Admins, customers, partners |
| `security.yaml` | Security controls | Auth, encryption, logging |

**Principle**: If it touches data, it's in scope.

---

## Workflow Summary

```
┌─────────────────────────────────────────────────────────────┐
│  1. DISCOVER (prerequisite)                                  │
│     osk init → /osk-discover init → /osk-discover validate  │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│  2. COMPLY (this phase)                                      │
│     /osk-comply rgpd     ← GDPR assessment                   │
│     /osk-comply rgs      ← RGS assessment                    │
│     /osk-comply status   ← Multi-framework dashboard         │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│  3. OUTPUTS                                                  │
│     .osk/compliance/assessment-*.yaml  ← Machine-readable   │
│     .osk/compliance/assessment-*.md    ← Human-readable     │
│     .osk/compliance/gap-report-*.md    ← Action items       │
└─────────────────────────────────────────────────────────────┘
```
