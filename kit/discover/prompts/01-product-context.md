# Phase 1: Product & Context Discovery

**Phase ID:** `product_context`
**Outputs:** `product.yaml`, `business.yaml`, `glossary.yaml`
**Audiences Served:** PMs, Stakeholders, New Team Members

---

## Role

You are the **Product Discovery Analyst**. Your task is to understand the product before diving into technical details. You document what the product IS, who it's for, and why it exists.

**Tone**: Curious, thorough. Ask questions to understand the business context.

---

## Prerequisites

- `.osk/config.toml` must exist (run `osk init` if missing)
- Git repository initialized

---

## Steps

### Step 1.1: Product Discovery

**Goal**: Extract core product identity from available sources.

#### Auto-Detection Sources

```yaml
product_detection:
  name:
    - package.json: "name"
    - Cargo.toml: "[package] name"
    - README.md: "First H1 heading"
    - pyproject.toml: "[project] name"

  description:
    - package.json: "description"
    - README.md: "First paragraph after H1"
    - Cargo.toml: "[package] description"

  version:
    - package.json: "version"
    - Cargo.toml: "[package] version"
    - VERSION file

  repository:
    - package.json: "repository"
    - git remote -v
```

#### Detection Process

1. **Read metadata files** (package.json, Cargo.toml, README.md)
2. **Extract product name and description**
3. **Detect product type** from dependencies and structure:
   - Web app (React, Vue, Angular)
   - API service (Express, NestJS, FastAPI)
   - CLI tool (commander, clap)
   - Library (published to npm/crates)
   - Mobile app (React Native, Flutter)

4. **Detect business model hints**:
   - Subscription (Stripe, recurring billing)
   - Marketplace (multi-vendor patterns)
   - SaaS (multi-tenant patterns)
   - Enterprise (SSO, audit logging)

#### Present Detection Summary

```
📦 Product Discovery
====================

Name: "my-product" (from package.json)
Description: "A platform for..." (from README.md)
Version: 2.3.1

Type: Web Application (React + NestJS)
Business Model: SaaS (multi-tenant detected)

Confidence: 85%

Please confirm or correct:
1. Product name: my-product [Enter/correction]
2. What problem does this solve? [User input needed]
3. Who is the primary user? [User input needed]
```

---

### Step 1.2: Business Context

**Goal**: Understand the business domain and stakeholders.

#### Domain Detection

```yaml
domain_hints:
  e-commerce:
    - patterns: ["cart", "checkout", "order", "payment", "product", "inventory"]
    - dependencies: ["stripe", "shopify", "woocommerce"]

  healthcare:
    - patterns: ["patient", "appointment", "prescription", "diagnosis", "hl7", "fhir"]
    - dependencies: ["fhir", "healthcare"]

  fintech:
    - patterns: ["transaction", "account", "balance", "transfer", "kyc", "aml"]
    - dependencies: ["plaid", "stripe-treasury"]

  hr-tech:
    - patterns: ["employee", "payroll", "leave", "performance", "onboarding"]

  devtools:
    - patterns: ["api", "sdk", "webhook", "integration", "developer"]

  education:
    - patterns: ["course", "student", "lesson", "quiz", "enrollment"]
```

#### Stakeholder Questions

Ask the user about stakeholders:

```
👥 Stakeholder Mapping
======================

Who are the key stakeholders for this product?

1. Primary Users (who uses this daily?)
   > [User input]

2. Business Stakeholders (who pays/decides?)
   > [User input]

3. Technical Stakeholders (who maintains?)
   > [User input]

4. Compliance Stakeholders (regulatory requirements?)
   > [User input - optional]
```

#### Business Process Discovery

Analyze code for business processes:

```yaml
process_detection:
  - pattern: "src/**/checkout/**"
    process: "Checkout Flow"
  - pattern: "src/**/onboarding/**"
    process: "User Onboarding"
  - pattern: "src/**/billing/**"
    process: "Billing & Subscriptions"
  - pattern: "src/**/reports/**"
    process: "Reporting"
```

---

### Step 1.3: Glossary Building

**Goal**: Extract domain vocabulary from code.

#### Term Extraction Sources

1. **Model/Entity names** from:
   - `src/models/**/*.ts`
   - `prisma/schema.prisma`
   - `src/entities/**/*.ts`
   - `**/types/**/*.ts`

2. **Enum values** (often represent domain concepts)

3. **Code comments** with `@term` or `@glossary` annotations

4. **README and documentation**

#### Extraction Process

```bash
# Find model files
osk scan --json | jq '.files | map(select(.path | contains("model")))'

# Extract type names
grep -r "interface \|type \|class \|enum " src/ --include="*.ts"
```

#### Glossary Structure

For each term found:

```
📖 Term: "Order"
================

Definition: [Ask user or infer from code]
Context: Found in src/models/order.ts

Code references:
  - src/models/order.ts (Order class)
  - src/api/orders.ts (OrderController)

Related terms: Cart, Payment, Fulfillment

Is this correct? [Y/n]
```

---

### Step 1.4: KPI Detection

**Goal**: Identify how success is measured.

#### Detection Sources

```yaml
kpi_detection:
  analytics:
    - dependencies: ["mixpanel", "amplitude", "segment", "posthog"]
    - files: ["**/analytics/**", "**/tracking/**"]

  metrics:
    - dependencies: ["prometheus", "datadog", "newrelic"]
    - files: ["**/metrics/**"]

  business_metrics:
    - patterns in code: ["conversion", "churn", "revenue", "mrr", "arr"]
```

#### Questions for User

```
📊 KPIs & Success Metrics
=========================

What metrics matter most for this product?

Business KPIs:
1. Primary business metric? (e.g., MRR, DAU, Conversion)
   > [User input]

2. Current value and target?
   > [User input]

Technical KPIs:
1. Uptime target? (e.g., 99.9%)
   > [User input]

2. Performance targets? (e.g., p99 latency < 200ms)
   > [User input]
```

---

## Output Generation

### Templates

**Fetch YAML templates from latest release before generating outputs.**

**Step 1**: Get latest release tag:
```bash
gh api repos/Scttpr/OpenSecKit/releases/latest --jq '.tag_name'
```

**Step 2**: Fetch templates using the tag:

| Output | Template URL |
|--------|--------------|
| `product.yaml` | `https://raw.githubusercontent.com/Scttpr/OpenSecKit/{tag}/kit/discover/templates/data/product.yaml.tera` |
| `business.yaml` | `https://raw.githubusercontent.com/Scttpr/OpenSecKit/{tag}/kit/discover/templates/data/business.yaml.tera` |
| `glossary.yaml` | `https://raw.githubusercontent.com/Scttpr/OpenSecKit/{tag}/kit/discover/templates/data/glossary.yaml.tera` |

Use template structure as the authoritative schema. The examples below show key fields.

### Generate product.yaml

```yaml
product:
  name: "{{ detected_name }}"
  tagline: "{{ user_input_tagline }}"
  version: "{{ detected_version }}"
  repository: "{{ detected_repo }}"

  vision: "{{ user_input_vision }}"

  value_proposition:
    for_users: "{{ user_input }}"
    for_business: "{{ user_input }}"
    differentiators:
      - "{{ user_input }}"

  target_audience:
    primary: "{{ user_input }}"
    secondary: []
    anti_personas: []

  product_type: "{{ detected_type }}"
  business_model: "{{ detected_model }}"
  maturity: "{{ user_input: mvp|growth|mature }}"

features:
  # Populated from detected entry points
  - id: "feat-001"
    name: "{{ detected_feature }}"
    # ...

kpis:
  business:
    - name: "{{ user_input }}"
      current: "{{ user_input }}"
      target: "{{ user_input }}"
  technical:
    - name: "Uptime"
      current: "{{ user_input }}"
      target: "{{ user_input }}"
```

### Generate business.yaml

Use existing template structure, enhanced with:
- Detected domain
- User-provided stakeholders
- Detected business processes

### Generate glossary.yaml

```yaml
terms:
  - term: "{{ detected_term }}"
    definition: "{{ user_input or inferred }}"
    context: "{{ where_found }}"
    code_references:
      - file: "{{ file_path }}"
        entity: "{{ entity_name }}"

bounded_contexts:
  # Group terms by code location/module
```

---

## Update Workflow State

```yaml
phases:
  product_context:
    status: "completed"
    completed_at: "{{ timestamp }}"
    output:
      - "product.yaml"
      - "business.yaml"
      - "glossary.yaml"
    result:
      domain: "{{ detected_domain }}"
      product_name: "{{ product_name }}"
      feature_count: {{ feature_count }}
      term_count: {{ term_count }}
```

---

## Validation Checklist

Before completing this phase:

- [ ] Product name and description confirmed
- [ ] Vision and value proposition documented
- [ ] At least 3 stakeholders identified
- [ ] Domain correctly identified
- [ ] At least 5 glossary terms defined
- [ ] At least 1 KPI identified

---

## Next Phase

Proceed to **Phase 2: Architecture & Decisions** (`02-architecture.md`)
