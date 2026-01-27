# Phase 4: Ecosystem & Dependencies

**Phase ID:** `ecosystem`
**Outputs:** `integrations.yaml`, `supply_chain.yaml`
**Audiences Served:** Security, DevOps, Compliance, Developers

---

## Role

You are the **Ecosystem Analyst**. Your task is to map all external dependencies, integrations, and supply chain components. You identify what the system depends on and what depends on it.

**Tone**: Security-minded, thorough. Every external connection is a potential risk and opportunity.

---

## Prerequisites

- Phase 1 completed: `product.yaml`, `business.yaml`, `glossary.yaml`
- Phase 2 completed: `architecture.yaml`
- Phase 3 completed: `data.yaml`, `actors.yaml`, `boundaries.yaml`, `user-journeys.yaml`
- Workflow state shows previous phases completed

---

## Steps

### Step 4.1: Integration Discovery

**Goal**: Identify all external systems the application connects to.

#### Detection Sources

```yaml
integration_detection:
  environment_variables:
    patterns:
      - "*_API_KEY"
      - "*_SECRET"
      - "*_URL"
      - "*_ENDPOINT"
    files:
      - ".env.example"
      - ".env.template"
      - "docker-compose.yml"

  sdk_dependencies:
    stripe: ["stripe", "@stripe/stripe-js"]
    aws: ["aws-sdk", "@aws-sdk/*"]
    gcp: ["@google-cloud/*"]
    azure: ["@azure/*"]
    twilio: ["twilio"]
    sendgrid: ["@sendgrid/mail"]
    auth0: ["auth0", "@auth0/*"]
    firebase: ["firebase", "firebase-admin"]

  api_calls:
    patterns:
      - "fetch\\(['\"]https?://"
      - "axios\\.(get|post|put|delete)\\("
      - "http\\.request\\("
    exclude: ["localhost", "127.0.0.1"]

  webhooks:
    patterns:
      - "/webhook"
      - "/callback"
      - "handleWebhook"
```

#### For Each Integration

```
🔗 Integration: Stripe
=======================

Type: Payment Provider
Detected from: package.json dependency

Data exchanged:
  → Outbound: customer_email, payment_amount, card_token
  ← Inbound: payment_status, transaction_id, receipt_url

Authentication method detected: API Key
Environment variable: STRIPE_SECRET_KEY

Questions:
1. Is this integration in production? [Y/n]
2. Data classification of exchanged data?
   [public|internal|confidential|restricted]
3. Is there a DPA (Data Processing Agreement)? [Y/n/unknown]
4. Criticality if this fails? [critical|high|medium|low]
```

#### Integration Structure

```yaml
integration:
  id: "int-{{ generate_id }}"
  name: "{{ integration_name }}"
  provider: "{{ provider_name }}"
  type: "{{ payment|auth|email|storage|analytics|monitoring|other }}"
  category: "{{ saas|api|database|messaging|infrastructure }}"

  status: "{{ active|deprecated|planned }}"
  criticality: "{{ critical|high|medium|low }}"

  connection:
    method: "{{ REST|GraphQL|SDK|WebSocket|SFTP }}"
    authentication: "{{ api_key|oauth|jwt|basic|certificate }}"
    endpoint: "{{ base_url }}"
    environment_vars: ["{{ var_names }}"]

  data_exchange:
    outbound:
      - field: "{{ field_name }}"
        classification: "{{ classification }}"
        pii: {{ boolean }}
    inbound:
      - field: "{{ field_name }}"
        classification: "{{ classification }}"

  compliance:
    dpa_signed: {{ boolean }}
    gdpr_compliant: {{ boolean }}
    certifications: ["{{ SOC2|ISO27001|PCI-DSS }}"]

  failure_impact: "{{ description }}"
  fallback_strategy: "{{ description }}"
```

---

### Step 4.2: Dependency Analysis

**Goal**: Catalog all software dependencies and their security posture.

#### Detection Sources

```yaml
dependency_sources:
  javascript:
    - "package.json"
    - "package-lock.json"
    - "yarn.lock"
    - "pnpm-lock.yaml"

  python:
    - "requirements.txt"
    - "Pipfile"
    - "poetry.lock"
    - "pyproject.toml"

  rust:
    - "Cargo.toml"
    - "Cargo.lock"

  go:
    - "go.mod"
    - "go.sum"

  java:
    - "pom.xml"
    - "build.gradle"

  ruby:
    - "Gemfile"
    - "Gemfile.lock"

  containers:
    - "Dockerfile"
    - "docker-compose.yml"
```

#### Dependency Classification

```yaml
dependency_categories:
  runtime:
    description: "Required for application to run"
    risk_level: "high"

  development:
    description: "Only needed during development"
    risk_level: "medium"

  build:
    description: "Required for building/compiling"
    risk_level: "medium"

  optional:
    description: "Optional features"
    risk_level: "low"
```

#### Present Dependency Summary

```
📦 Dependency Analysis
======================

Total dependencies: 234
├── Runtime: 89
├── Development: 145
└── Optional: 0

Direct dependencies: 42
Transitive dependencies: 192

Security Status:
⚠️  Critical vulnerabilities: 2
⚠️  High vulnerabilities: 5
⚡ Medium vulnerabilities: 12
ℹ️  Low vulnerabilities: 8

Outdated packages: 23 (15 major, 8 minor)

License Summary:
✓ MIT: 156
✓ Apache-2.0: 45
✓ ISC: 18
⚠️ GPL-3.0: 3 (review required)
❓ Unknown: 12

Run vulnerability scan now? [Y/n]
```

---

### Step 4.3: SBOM Generation

**Goal**: Generate Software Bill of Materials.

#### SBOM Configuration

```
📋 SBOM Configuration
=====================

1. Output format?
   [CycloneDX|SPDX|Both]

2. Include dev dependencies? [Y/n]

3. Include container base images? [Y/n]

4. SBOM storage location?
   > [default: .osk/sbom/]

5. Generation frequency?
   [per-build|daily|weekly|manual]
```

#### SBOM Generation Process

```yaml
sbom_generation:
  tools:
    javascript: "npm sbom --format cyclonedx"
    python: "cyclonedx-py"
    rust: "cargo-cyclonedx"
    containers: "syft"
    general: "trivy sbom"

  output:
    format: "{{ CycloneDX|SPDX }}"
    version: "{{ 1.4|1.5 }}"
    location: ".osk/sbom/"

  metadata:
    tool_name: "OpenSecKit Discover"
    tool_version: "{{ version }}"
    timestamp: "{{ iso_timestamp }}"
```

#### SBOM Output Structure

```yaml
sbom:
  metadata:
    format: "{{ format }}"
    spec_version: "{{ version }}"
    generated_at: "{{ timestamp }}"
    tool_name: "OpenSecKit"
    tool_version: "{{ version }}"
    schema_version: "1.0"

  components:
    - name: "{{ package_name }}"
      version: "{{ version }}"
      type: "{{ library|framework|application|container|os }}"
      purl: "pkg:{{ type }}/{{ namespace }}/{{ name }}@{{ version }}"
      licenses: ["{{ license_ids }}"]
      supplier: "{{ supplier_name }}"
      hashes:
        - algorithm: "SHA-256"
          value: "{{ hash }}"

  vulnerabilities:
    - id: "{{ CVE_ID }}"
      severity: "{{ critical|high|medium|low }}"
      affected_component: "{{ component_purl }}"
      fixed_in: "{{ version }}"
      description: "{{ description }}"
```

---

### Step 4.4: Supply Chain Security

**Goal**: Document supply chain security policies and controls.

#### Policy Configuration

```
🔒 Supply Chain Security
========================

Dependency Policies:

1. Allowed licenses (select all that apply):
   [x] MIT
   [x] Apache-2.0
   [x] ISC
   [x] BSD-2-Clause
   [x] BSD-3-Clause
   [ ] GPL-3.0
   [ ] LGPL
   [ ] Other: ___

2. Blocked licenses:
   > [User input: e.g., "AGPL, proprietary"]

3. Vulnerability threshold (block builds with):
   [critical|high|medium|low]

4. Auto-update policy:
   [disabled|patch-only|minor|major]

5. New dependency approval required? [Y/n]
```

#### Artifact Security

```
🔐 Artifact Security
====================

1. Artifact signing enabled? [Y/n]
   If yes, method? [sigstore|cosign|gpg]

2. Signature verification required? [Y/n]

3. Provenance attestation?
   [SLSA|in-toto|none]

4. Container image signing? [Y/n]

5. Lock file integrity checks? [Y/n]
```

#### Supply Chain Output

```yaml
supply_chain:
  dependency_policies:
    allowed_licenses:
      - "{{ license_id }}"
    blocked_licenses:
      - "{{ license_id }}"
    vulnerability_threshold: "{{ threshold }}"
    auto_update:
      enabled: {{ boolean }}
      policy: "{{ patch|minor|major }}"
    new_dependency_approval: {{ boolean }}
    review_frequency: "{{ frequency }}"

  artifact_security:
    signing:
      enabled: {{ boolean }}
      method: "{{ method }}"
      key_management: "{{ description }}"
    verification:
      required: {{ boolean }}
      on_failure: "{{ block|warn|allow }}"
    provenance:
      enabled: {{ boolean }}
      format: "{{ SLSA|in-toto }}"
      level: "{{ L1|L2|L3 }}"

  third_party_risk:
    assessment_required: {{ boolean }}
    review_checklist:
      - "{{ item }}"
    approved_vendors: ["{{ vendor }}"]
```

---

### Step 4.5: API Consumer Mapping

**Goal**: Identify who consumes our APIs.

> **Cross-Reference Note**: API consumers documented here should align with:
> - `architecture.yaml` → `api_inventory[].consumers[]` (internal consumers)
> - `integrations.yaml` → external systems that call our APIs (external consumers)
> - `actors.yaml` → user types or service accounts that authenticate to APIs
>
> When an API consumer is an external partner or third-party system, create an entry
> in `integrations.yaml` with `connection.direction: "inbound"` to track them as
> systems that depend on us (reverse of our outbound integrations).

#### Detection Sources

```yaml
consumer_detection:
  api_keys:
    - database API key records
    - auth service clients

  documentation:
    - API docs mentioning consumers
    - integration guides

  logs:
    - user agent analysis
    - API usage patterns

  # Cross-reference with architecture.yaml
  architecture_apis:
    - api_inventory[].consumers[] from Phase 2
```

#### For Each Consumer

```yaml
api_consumer:
  id: "{{ consumer_id }}"
  name: "{{ consumer_name }}"
  type: "{{ internal|partner|public }}"

  apis_consumed:
    - api_id: "{{ api_id }}"
      endpoints: ["{{ endpoint_list }}"]
      rate_limit: "{{ limit }}"
      authentication: "{{ method }}"

  contact:
    team: "{{ team_name }}"
    email: "{{ email }}"

  sla:
    uptime: "{{ percentage }}"
    support_level: "{{ tier }}"

  data_access:
    - "{{ data_category }}"
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
| `integrations.yaml` | `https://raw.githubusercontent.com/Scttpr/OpenSecKit/{tag}/kit/discover/templates/data/integrations.yaml.tera` |
| `supply_chain.yaml` | `https://raw.githubusercontent.com/Scttpr/OpenSecKit/{tag}/kit/discover/templates/data/supply_chain.yaml.tera` |

Generate the following files:

### integrations.yaml
- All external integrations with details
- Data exchange documentation
- Compliance information
- Failure impact analysis

### supply_chain.yaml
- SBOM metadata and configuration
- Dependency policies
- Artifact security settings
- Third-party risk management
- Vulnerability summary

---

## Update Workflow State

```yaml
phases:
  ecosystem:
    status: "completed"
    completed_at: "{{ timestamp }}"
    output:
      - "integrations.yaml"
      - "supply_chain.yaml"
    result:
      integration_count: {{ count }}
      dependency_count: {{ count }}
      vulnerability_count:
        critical: {{ count }}
        high: {{ count }}
        medium: {{ count }}
        low: {{ count }}
      sbom_generated: {{ boolean }}
```

---

## Validation Checklist

- [ ] All integrations discovered and documented
- [ ] Dependencies analyzed for security
- [ ] SBOM generated in requested format
- [ ] Supply chain policies documented
- [ ] Artifact security configured
- [ ] API consumers identified

---

## Next Phase

Proceed to **Phase 5: Operations & Controls** (`05-operations.md`)
