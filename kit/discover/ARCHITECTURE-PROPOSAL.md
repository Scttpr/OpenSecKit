# Discover Workflow Architecture Proposal

## Overview

Transform `/osk-discover` into a 6-phase orchestrated workflow that captures **deep product understanding** for multiple audiences:

- **Product Managers** - Business context, user journeys, feature map, KPIs
- **Developers** - Architecture decisions, setup guides, coding conventions, debugging
- **Security** - Controls, boundaries, compliance, risks
- **Operations** - Runbooks, monitoring, incidents, on-call

This goes beyond security documentation to create a **complete product knowledge base**.

---

## Design Principles

1. **Product-First** - Understand the product before analyzing security
2. **Multi-Audience** - Same source data, different documentation views
3. **Living Documentation** - Auto-updates with code changes
4. **Onboarding-Ready** - New team members can understand the system quickly

---

## File Structure

```
kit/discover/
├── prompts/
│   ├── discover.md                    # Main orchestrator
│   ├── 01-product-context.md          # Phase 1: Product & Business
│   ├── 02-architecture.md             # Phase 2: Architecture & Decisions
│   ├── 03-domain-model.md             # Phase 3: Data, Actors, Domain
│   ├── 04-ecosystem.md                # Phase 4: Integrations, Dependencies
│   ├── 05-operations.md               # Phase 5: Controls, Tooling, Team
│   └── 06-synthesis.md                # Phase 6: Gaps & Documentation
│
├── schemas/
│   ├── workflow-state.yaml            # Workflow state schema
│   ├── index.yaml                     # Index schema
│   │
│   │ # Product Understanding (NEW)
│   ├── product.yaml                   # Product vision, features, roadmap
│   ├── glossary.yaml                  # Domain terminology
│   ├── user-journeys.yaml             # User flows and personas
│   │
│   │ # Existing (Enhanced)
│   ├── business.yaml                  # Business context + KPIs
│   ├── architecture.yaml              # Architecture + ADRs
│   ├── data.yaml                      # Data inventory
│   ├── actors.yaml                    # Actors + personas
│   ├── boundaries.yaml                # Trust boundaries
│   ├── integrations.yaml              # External services
│   ├── supply_chain.yaml              # SBOM, dependencies
│   ├── controls.yaml                  # Security controls
│   ├── tooling.yaml                   # CI/CD, collaboration
│   ├── team.yaml                      # Team + onboarding
│   ├── operations.yaml                # Runbooks, monitoring (NEW)
│   └── gaps.yaml                      # Gaps analysis
│
├── knowledge/
│   ├── detection-patterns.md          # Auto-detection rules
│   ├── architecture-styles.md         # Architecture pattern reference
│   ├── domain-modeling.md             # DDD patterns (NEW)
│   ├── user-journey-templates.md      # Journey mapping guide (NEW)
│   ├── adr-templates.md               # ADR format reference (NEW)
│   ├── data-classification.md         # Data classification guidance
│   ├── control-frameworks.md          # Security control reference
│   └── sbom-formats.md                # SBOM format reference
│
└── templates/
    ├── data/                          # YAML templates
    │   ├── product.yaml.tera          # NEW
    │   ├── glossary.yaml.tera         # NEW
    │   ├── user-journeys.yaml.tera    # NEW
    │   ├── operations.yaml.tera       # NEW
    │   └── ... (existing templates)
    │
    └── docs/                          # Documentation templates (NEW)
        ├── pm-onboarding.md.tera      # PM onboarding guide
        ├── dev-onboarding.md.tera     # Developer onboarding
        ├── security-overview.md.tera  # Security documentation
        ├── ops-runbook.md.tera        # Operations runbook
        └── architecture-guide.md.tera # Architecture documentation
```

---

## New System Model Files

### product.yaml - Product Vision & Features

```yaml
# .osk/system-model/product.yaml
# Purpose: Core product understanding for PMs and stakeholders

metadata:
  section: "product"
  parent: "index.yaml"

product:
  name: "My Product"
  tagline: "One-line description"
  vision: "What problem does this solve? Why does it exist?"

  value_proposition:
    for_users: "What value do end users get?"
    for_business: "What value does the business get?"
    differentiators: ["What makes this unique?"]

  target_audience:
    primary: "Who is the main user?"
    secondary: ["Other user segments"]
    anti_personas: ["Who is this NOT for?"]

features:
  - id: "feat-001"
    name: "Feature Name"
    description: "What it does"
    status: "live"                    # live | beta | planned | deprecated
    introduced: "2024-Q1"
    owner: "Team/Person"

    user_value: "Why users care"
    business_value: "Why business cares"

    entry_points:                     # How users access this
      - path: "/dashboard"
        method: "UI"
      - path: "/api/v1/resource"
        method: "API"

    dependencies:                     # What it needs
      components: ["auth-service", "database"]
      integrations: ["stripe", "sendgrid"]

    metrics:                          # How we measure success
      - name: "conversion_rate"
        target: "5%"
      - name: "latency_p99"
        target: "<200ms"

kpis:
  business:
    - name: "Monthly Active Users"
      current: "10,000"
      target: "50,000"
      tracking: "analytics.mau"
    - name: "Revenue"
      current: "$100k MRR"
      target: "$500k MRR"

  technical:
    - name: "Uptime"
      current: "99.9%"
      target: "99.95%"
      tracking: "monitoring.uptime"
    - name: "Deployment Frequency"
      current: "weekly"
      target: "daily"

roadmap:
  current_quarter:
    theme: "Growth & Stability"
    priorities:
      - "Scale infrastructure"
      - "Improve onboarding"

  future:
    - quarter: "Q2 2026"
      items: ["Mobile app", "API v2"]
    - quarter: "Q3 2026"
      items: ["Enterprise features"]
```

### glossary.yaml - Domain Terminology

```yaml
# .osk/system-model/glossary.yaml
# Purpose: Shared vocabulary for team alignment

metadata:
  section: "glossary"
  parent: "index.yaml"

terms:
  - term: "Order"
    definition: "A customer's request to purchase items"
    context: "Created when user completes checkout"
    aliases: ["Purchase", "Transaction"]
    related_terms: ["Cart", "Payment", "Fulfillment"]
    code_references:
      - file: "src/models/order.ts"
        entity: "Order"
      - file: "src/api/orders.ts"
        entity: "OrderController"

  - term: "Tenant"
    definition: "A customer organization in multi-tenant setup"
    context: "Each tenant has isolated data"
    technical_notes: "Implemented via schema-per-tenant"
    code_references:
      - file: "src/middleware/tenant.ts"

acronyms:
  - acronym: "MRR"
    expansion: "Monthly Recurring Revenue"
    context: "Key business metric"

  - acronym: "CQRS"
    expansion: "Command Query Responsibility Segregation"
    context: "Architecture pattern used in order service"

domain_concepts:
  bounded_contexts:
    - name: "Orders"
      description: "Everything related to order lifecycle"
      entities: ["Order", "OrderItem", "OrderStatus"]

    - name: "Payments"
      description: "Payment processing and billing"
      entities: ["Payment", "Invoice", "Subscription"]
```

### user-journeys.yaml - User Flows & Personas

```yaml
# .osk/system-model/user-journeys.yaml
# Purpose: Understand how users interact with the product

metadata:
  section: "user_journeys"
  parent: "index.yaml"

personas:
  - id: "persona-admin"
    name: "Admin Alice"
    role: "System Administrator"
    goals:
      - "Manage users and permissions"
      - "Monitor system health"
      - "Configure integrations"
    pain_points:
      - "Complex permission model"
      - "Limited audit visibility"
    technical_level: "high"

  - id: "persona-user"
    name: "End User Emma"
    role: "Regular User"
    goals:
      - "Complete tasks quickly"
      - "Track progress"
    pain_points:
      - "Slow page loads"
      - "Confusing navigation"
    technical_level: "low"

journeys:
  - id: "journey-signup"
    name: "User Signup"
    persona: "persona-user"
    description: "New user creates account and completes onboarding"

    trigger: "User clicks 'Sign Up' from landing page"
    outcome: "User has active account and completed initial setup"

    steps:
      - step: 1
        action: "Enter email and password"
        component: "auth-service"
        api: "POST /api/auth/register"
        data_created: ["user"]

      - step: 2
        action: "Verify email"
        component: "notification-service"
        integration: "sendgrid"

      - step: 3
        action: "Complete profile"
        component: "user-service"
        api: "PUT /api/users/me"
        data_created: ["user_profile"]

      - step: 4
        action: "Select plan"
        component: "billing-service"
        integration: "stripe"
        data_created: ["subscription"]

    success_metrics:
      - "signup_completion_rate"
      - "time_to_first_value"

    failure_points:
      - step: 2
        issue: "Email not received"
        mitigation: "Resend option, check spam"

  - id: "journey-checkout"
    name: "Purchase Flow"
    persona: "persona-user"
    # ... similar structure
```

### operations.yaml - Runbooks & Monitoring

```yaml
# .osk/system-model/operations.yaml
# Purpose: Operations knowledge for on-call and DevOps

metadata:
  section: "operations"
  parent: "index.yaml"

environments:
  - name: "production"
    url: "https://app.example.com"
    region: "eu-west-1"
    provider: "AWS"
    access: "VPN + SSO required"

  - name: "staging"
    url: "https://staging.example.com"
    region: "eu-west-1"
    provider: "AWS"
    access: "Team members only"

monitoring:
  dashboards:
    - name: "Main Dashboard"
      url: "https://grafana.example.com/d/main"
      purpose: "Overall system health"

    - name: "API Performance"
      url: "https://grafana.example.com/d/api"
      purpose: "API latency and errors"

  alerts:
    - name: "High Error Rate"
      condition: "error_rate > 1%"
      severity: "critical"
      runbook: "runbook-high-errors"
      escalation: "page on-call"

    - name: "Database CPU High"
      condition: "db_cpu > 80%"
      severity: "warning"
      runbook: "runbook-db-performance"
      escalation: "slack #alerts"

runbooks:
  - id: "runbook-high-errors"
    name: "High Error Rate Response"
    trigger: "Error rate alert fires"

    diagnosis:
      - step: 1
        action: "Check error logs"
        command: "kubectl logs -l app=api --tail=100"

      - step: 2
        action: "Identify error pattern"
        check: "Is it one endpoint or widespread?"

      - step: 3
        action: "Check recent deployments"
        command: "kubectl rollout history deployment/api"

    resolution:
      - scenario: "Bad deployment"
        action: "Rollback"
        command: "kubectl rollout undo deployment/api"

      - scenario: "External service down"
        action: "Enable circuit breaker"
        command: "kubectl set env deployment/api STRIPE_CIRCUIT=open"

      - scenario: "Database overload"
        action: "Scale read replicas"
        ref: "runbook-db-performance"

    post_incident:
      - "Create incident ticket"
      - "Schedule postmortem if P1/P2"
      - "Update runbook if new pattern"

  - id: "runbook-deployment"
    name: "Production Deployment"
    steps:
      - step: 1
        action: "Verify staging tests passed"
        check: "CI green on main branch"

      - step: 2
        action: "Announce in #deployments"
        template: "Deploying v{version} to production"

      - step: 3
        action: "Deploy"
        command: "gh workflow run deploy.yml -f env=production"

      - step: 4
        action: "Monitor for 15 minutes"
        dashboard: "API Performance"

      - step: 5
        action: "Verify smoke tests"
        command: "curl https://app.example.com/health"

on_call:
  rotation: "PagerDuty schedule 'Backend On-Call'"
  escalation_policy:
    - level: 1
      wait: "5 minutes"
      notify: "Primary on-call"
    - level: 2
      wait: "10 minutes"
      notify: "Secondary on-call"
    - level: 3
      wait: "15 minutes"
      notify: "Engineering Manager"

  handoff_checklist:
    - "Review open incidents"
    - "Check deployment schedule"
    - "Review recent changes"
```

---

## Workflow State Schema

```yaml
# .osk/system-model/workflow-state.yaml

workflow:
  version: "1.0.0"
  started_at: "2026-01-24T10:00:00Z"
  last_updated: "2026-01-24T12:30:00Z"
  discovery_mode: "full"              # full | incremental | context
  base_commit: "abc123def456"         # Git commit at start
  current_commit: "def456abc789"      # Current git commit

  current_phase: "architecture"       # Current phase being executed
  current_step: "component-analysis"  # Current step within phase

phases:
  context:
    status: "completed"               # pending | in_progress | completed | skipped
    started_at: "2026-01-24T10:00:00Z"
    completed_at: "2026-01-24T10:15:00Z"
    output: "business.yaml"
    result:
      domain: "e-commerce"
      stakeholder_count: 4
      process_count: 6

  architecture:
    status: "in_progress"
    started_at: "2026-01-24T10:15:00Z"
    output: "architecture.yaml"
    steps:
      - id: "quick-scan"
        status: "completed"
      - id: "component-analysis"
        status: "in_progress"
      - id: "data-flow-mapping"
        status: "pending"
      - id: "api-inventory"
        status: "pending"
      - id: "resilience-analysis"
        status: "pending"
    result:
      component_count: 0              # Updated as discovered
      detected_stack: ["TypeScript", "NestJS", "PostgreSQL"]

  data_actors:
    status: "pending"
    output:
      - "data.yaml"
      - "actors.yaml"
      - "boundaries.yaml"

  integrations:
    status: "pending"
    output:
      - "integrations.yaml"
      - "supply_chain.yaml"

  controls_tooling:
    status: "pending"
    output:
      - "controls.yaml"
      - "tooling.yaml"
      - "team.yaml"

  gaps_docs:
    status: "pending"
    output:
      - "gaps.yaml"
      - "index.yaml"
      - "documents/"

# Incremental update tracking
incremental:
  last_full_discovery: "2026-01-20T09:00:00Z"
  change_detection:
    enabled: true
    patterns_to_phases:
      # File patterns → affected phases
      "src/models/**": ["data_actors"]
      "src/api/**": ["architecture"]
      "src/auth/**": ["data_actors", "controls_tooling"]
      "package*.json": ["integrations"]
      "Cargo.toml": ["integrations"]
      "terraform/**": ["architecture", "data_actors"]
      ".github/workflows/**": ["controls_tooling"]

  pending_updates:
    - phase: "architecture"
      reason: "src/api/orders.ts modified"
      files_changed: 3
    - phase: "integrations"
      reason: "package.json modified"
      files_changed: 1
```

---

## Phase Definitions (Enhanced for Deep Product Understanding)

### Phase 1: Product & Context (`01-product-context.md`)

**Purpose:** Understand the product before diving into technical details

**Audiences Served:** PMs, Stakeholders, New Team Members

**Steps:**
1. **Product Discovery**
   - Extract product name, description from README/package.json
   - Identify value proposition and target audience
   - Detect feature flags and experiments

2. **Business Context**
   - Domain identification (e-commerce, healthcare, fintech, etc.)
   - Stakeholder mapping (who uses, who maintains, who pays)
   - Business process discovery
   - Compliance context (RGPD/RGS hints)

3. **Glossary Building**
   - Extract domain terms from code (models, types, comments)
   - Identify bounded contexts (DDD)
   - Map code entities to business concepts

4. **KPI Detection**
   - Find analytics/tracking code
   - Identify metrics dashboards
   - Document success criteria

**Inputs:**
- README.md, package.json, Cargo.toml
- Code comments and documentation
- User interview

**Outputs:**
- `product.yaml` - Product vision, features, KPIs
- `business.yaml` - Business context, stakeholders
- `glossary.yaml` - Domain terminology

**Detection Patterns:**
```yaml
product_detection:
  name: ["package.json:name", "Cargo.toml:package.name", "README.md:h1"]
  description: ["package.json:description", "README.md:first_paragraph"]

feature_detection:
  - feature_flags: ["launchdarkly", "flagsmith", "unleash"]
  - experiments: ["growthbook", "optimizely"]

domain_terms:
  - models: "src/models/**/*.ts"
  - types: "src/types/**/*.ts"
  - entities: "prisma/schema.prisma"
```

---

### Phase 2: Architecture & Decisions (`02-architecture.md`)

**Purpose:** Map technical architecture AND capture the "why" behind decisions

**Audiences Served:** Developers, Tech Leads, Architects

**Steps:**
1. **Quick Scan**
   - Run `osk scan --json`
   - Detect repository structure (mono/multi-repo)

2. **Component Identification**
   - Services, APIs, frontends, databases
   - Component responsibilities and ownership
   - Entry points and public interfaces

3. **Tech Stack Analysis**
   - Languages, frameworks, libraries
   - Why these choices? (look for ADRs)
   - Known limitations and trade-offs

4. **Architecture Decision Records (ADRs)**
   - Find existing ADRs in docs/adr/
   - Infer decisions from code patterns
   - Document implicit decisions

5. **Data Flow Mapping**
   - How data moves between components
   - Synchronous vs. async patterns
   - Message queues and event buses

6. **API Inventory**
   - Public APIs with versioning
   - Internal APIs
   - GraphQL schemas, OpenAPI specs

7. **Resilience Analysis**
   - DR strategy, backups, HA configuration
   - Circuit breakers, retries, timeouts

**Inputs:**
- Phase 1: `product.yaml`, `business.yaml`
- Codebase scan results

**Outputs:**
- `architecture.yaml` - Components, data flows, API inventory, resilience

**ADR Schema (embedded in architecture.yaml):**
```yaml
architecture_decisions:
  - id: "ADR-001"
    title: "Use PostgreSQL for primary database"
    status: "accepted"                # proposed | accepted | deprecated | superseded
    date: "2024-01-15"
    context: "Need ACID transactions for order processing"
    decision: "PostgreSQL with read replicas"
    consequences:
      positive: ["Strong consistency", "Rich ecosystem"]
      negative: ["Scaling limitations", "Operational complexity"]
    alternatives_considered:
      - option: "MongoDB"
        reason_rejected: "Need strong transactions"
    supersedes: null
    superseded_by: null

  - id: "ADR-002"
    title: "Event-driven architecture for order processing"
    status: "accepted"
    # ... etc
```

**Detection Patterns:**
```yaml
adr_detection:
  locations:
    - "docs/adr/*.md"
    - "docs/architecture/decisions/*.md"
    - "ADR-*.md"

component_detection:
  - pattern: "src/services/**"
    type: "service"
  - pattern: "apps/*"
    type: "application"
  - pattern: "packages/*"
    type: "library"

architecture_patterns:
  - cqrs: ["@nestjs/cqrs", "pattern: CommandHandler"]
  - event_sourcing: ["eventstore", "event-sourcing"]
  - microservices: ["kubernetes", "docker-compose services > 3"]
```

---

### Phase 3: Domain Model & Actors (`03-domain-model.md`)

**Purpose:** Deep understanding of domain, data, users, and their journeys

**Audiences Served:** PMs, Developers, Analysts

**Steps:**
1. **Domain Modeling**
   - Extract entities from code (models, schemas)
   - Identify aggregates and value objects
   - Map relationships between entities

2. **Data Category Discovery**
   - From models, schemas, migrations
   - Classify sensitivity (PII, financial, health)
   - Retention requirements

3. **User Personas**
   - From auth/roles code
   - From analytics/tracking
   - User interview

4. **User Journey Mapping**
   - Trace user flows through code
   - Entry points → APIs → Data
   - Error paths and edge cases

5. **Actor Identification**
   - Human users (roles, permissions)
   - Service accounts
   - External systems

6. **Trust Boundaries**
   - Network segments
   - Security zones
   - Data isolation

**Inputs:**
- Phase 1: `product.yaml`, `glossary.yaml`
- Phase 2: `architecture.yaml`

**Outputs:**
- `data.yaml` - Data inventory with domain context
- `actors.yaml` - Users, roles, personas
- `boundaries.yaml` - Trust zones
- `user-journeys.yaml` - User flows and touchpoints

**User Journey Detection:**
```yaml
journey_detection:
  auth_flows:
    - "src/auth/**" → signup, login, password-reset
  checkout_flows:
    - "src/checkout/**" → cart, payment, confirmation
  api_flows:
    - trace: "controller → service → repository"
```

---

### Phase 4: Ecosystem & Dependencies (`04-ecosystem.md`)

**Purpose:** Map all external dependencies and their role in the product

**Audiences Served:** Developers, DevOps, Security

**Steps:**
1. **External Service Detection**
   - SDKs and API calls
   - SaaS integrations (Stripe, SendGrid, etc.)
   - Cloud services (AWS, GCP, Azure)

2. **Integration Documentation**
   - Purpose of each integration
   - Data exchanged
   - Configuration and credentials

3. **DPA/Compliance Status**
   - Data Processing Agreements
   - Data residency
   - Certifications

4. **SBOM Generation**
   - Direct dependencies
   - Transitive dependencies
   - License analysis

5. **Dependency Risk Assessment**
   - Critical dependencies
   - Single points of failure
   - Alternative options

**Inputs:**
- Phase 2: `architecture.yaml`
- Phase 3: `data.yaml`

**Outputs:**
- `integrations.yaml` - External services with context
- `supply_chain.yaml` - SBOM, dependencies, risks

**Integration Context Schema:**
```yaml
integrations:
  - id: "stripe"
    name: "Stripe"
    category: "payment"
    purpose: "Process customer payments"

    # Product context (NEW)
    used_in_features: ["checkout", "subscriptions"]
    user_journeys: ["journey-checkout", "journey-upgrade"]

    # Why this choice?
    decision:
      date: "2023-06"
      reason: "Best-in-class payment UX, PCI compliance"
      alternatives_considered: ["Adyen", "PayPal"]

    # Operational context
    criticality: "high"
    fallback: "Queue failed payments for retry"
    sla: "99.99%"

    # Technical details
    connection: { ... }
    data_exchanged: { ... }
    compliance: { ... }
```

---

### Phase 5: Operations & Team (`05-operations.md`)

**Purpose:** Capture operational knowledge for running and maintaining the system

**Audiences Served:** DevOps, On-Call Engineers, New Developers

**Steps:**
1. **Environment Documentation**
   - Production, staging, development
   - Access requirements
   - Configuration differences

2. **Monitoring & Alerting**
   - Dashboards and their purpose
   - Alert conditions and runbooks
   - Log locations and formats

3. **Runbook Creation**
   - Common incidents and responses
   - Deployment procedures
   - Rollback procedures

4. **CI/CD Pipeline Analysis**
   - Build stages and gates
   - Deployment strategies
   - Security tooling

5. **Security Controls**
   - Authentication mechanisms
   - Authorization model
   - Encryption and secrets

6. **Team Structure**
   - Roles and responsibilities
   - On-call rotation
   - Escalation paths

7. **Onboarding Guide Generation**
   - Local setup steps
   - Required access and tools
   - First tasks

**Inputs:**
- All previous phases

**Outputs:**
- `controls.yaml` - Security controls
- `tooling.yaml` - CI/CD, collaboration
- `team.yaml` - Team structure, onboarding
- `operations.yaml` - Runbooks, monitoring, environments

**Developer Onboarding Schema (in team.yaml):**
```yaml
onboarding:
  developer:
    estimated_time: "1 week"

    day_1:
      - task: "Get access to GitHub, Slack, Jira"
        owner: "IT"
        time: "2 hours"

      - task: "Clone repository and run setup"
        command: "make setup"
        docs: "docs/getting-started.md"
        time: "1 hour"

      - task: "Review architecture documentation"
        docs: ".osk/system-model/documents/architecture-guide.md"
        time: "2 hours"

    day_2:
      - task: "Pair with mentor on first bug fix"
      - task: "Review coding conventions"
        docs: "CONTRIBUTING.md"

    week_1:
      - task: "Complete first PR"
      - task: "Shadow on-call rotation"
      - task: "Review domain glossary"
        docs: ".osk/system-model/glossary.yaml"

  pm:
    estimated_time: "3 days"
    day_1:
      - task: "Review product documentation"
        docs: ".osk/system-model/documents/pm-onboarding.md"
      - task: "Meet key stakeholders"
```

---

### Phase 6: Synthesis & Documentation (`06-synthesis.md`)

**Purpose:** Consolidate all knowledge into actionable documentation

**Audiences:** Everyone - generates audience-specific docs

**Steps:**
1. **Gap Consolidation**
   - Collect gaps from all phases
   - Prioritize by severity and impact
   - Create remediation roadmap

2. **Index Generation**
   - Statistics and health indicators
   - Model completeness score
   - Compliance readiness

3. **Documentation Generation** (Multi-Audience)
   - PM Onboarding Guide
   - Developer Onboarding Guide
   - Architecture Guide
   - Security Overview
   - Operations Runbook
   - API Reference

4. **Cross-Reference Validation**
   - Ensure all references are valid
   - Check for orphaned entities
   - Validate glossary usage

**Inputs:**
- All previous phase outputs

**Outputs:**
- `gaps.yaml` - Consolidated gaps
- `index.yaml` - Model index with statistics
- `documents/` - Generated documentation

**Documentation Outputs:**

| Document | Audience | Contents |
|----------|----------|----------|
| `pm-onboarding.md` | PMs | Product vision, features, KPIs, stakeholders, roadmap |
| `dev-onboarding.md` | Developers | Setup guide, architecture, ADRs, coding conventions |
| `architecture-guide.md` | Tech Leads | Components, data flows, decisions, trade-offs |
| `security-overview.md` | Security | Controls, boundaries, compliance, risks |
| `ops-runbook.md` | DevOps | Environments, monitoring, alerts, incident response |
| `api-reference.md` | API Consumers | API inventory, authentication, examples |
| `domain-guide.md` | Analysts | Glossary, data model, user journeys |

---

## Orchestrator Logic (`discover.md`)

```
/osk-discover
      │
      ▼
┌─────────────────────────────┐
│ Check workflow-state.yaml   │
└─────────────────────────────┘
      │
      ├── NOT EXISTS ──────────► FULL DISCOVERY
      │                              │
      │                              ▼
      │                         Execute Phases 1-6
      │
      ▼ EXISTS
┌─────────────────────────────┐
│ Check current_phase status  │
└─────────────────────────────┘
      │
      ├── IN_PROGRESS ─────────► RESUME from current_step
      │
      ├── COMPLETED ───────────► Check for changes
      │                              │
      │                              ├── Changes found
      │                              │       │
      │                              │       ▼
      │                              │   INCREMENTAL UPDATE
      │                              │   (phase-aware)
      │                              │
      │                              └── No changes
      │                                      │
      │                                      ▼
      │                                  "Model up to date"
      │                                  Offer: [R]efresh context
      │
      └── User requests FULL ──► Reset state, FULL DISCOVERY
```

---

## Incremental Update Logic

### Change Detection

```bash
# Get changes since last discovery
osk changes --since workflow-state.yaml:base_commit
```

### Phase Mapping

```yaml
# Map changed files to affected phases
file_patterns:
  # Phase 2: Architecture
  "src/**/*.ts": architecture
  "src/**/*.rs": architecture
  "Dockerfile": architecture

  # Phase 3: Data & Actors
  "src/models/**": data_actors
  "src/auth/**": data_actors
  "prisma/schema.prisma": data_actors

  # Phase 4: Integrations
  "package.json": integrations
  "Cargo.toml": integrations
  "src/integrations/**": integrations

  # Phase 5: Controls & Tooling
  ".github/workflows/**": controls_tooling
  "src/security/**": controls_tooling
  ".eslintrc*": controls_tooling
```

### Update Execution

```
Incremental Update:
  1. Identify changed phases from file patterns
  2. Load existing YAML for unchanged phases
  3. Re-run ONLY affected phases
  4. Update dependencies:
     - If Phase 2 re-runs → re-run Phase 3 (data flows depend on components)
     - If Phase 3 re-runs → re-run Phase 4 (integrations reference data)
     - If Phase 4/5 re-runs → re-run Phase 6 (gaps consolidation)
  5. Update workflow-state.yaml
```

---

## User Interaction Points

### Phase 1 (Context)
- Confirm domain detection
- Add/edit stakeholders
- Validate business processes

### Phase 2 (Architecture)
- Confirm component detection
- Validate tech stack
- Review data flows

### Phase 3 (Data & Actors)
- Confirm PII detection
- Validate user types
- Define trust zones

### Phase 4 (Integrations)
- Confirm external services
- Provide DPA status
- Validate SBOM

### Phase 5 (Controls & Tooling)
- Confirm control detection
- Add team contacts
- Validate tooling

### Phase 6 (Gaps & Docs)
- Review gap prioritization
- Select documentation audiences
- Final validation

---

## CLI Commands

```bash
# Full discovery
osk discover

# Resume interrupted discovery
osk discover --resume

# Incremental update
osk discover --update

# Context refresh only
osk discover --context

# Run specific phase
osk discover --phase architecture

# Skip to phase (assumes previous completed)
osk discover --from-phase data_actors

# Validate existing model
osk discover validate

# Generate documentation
osk discover docs --audience security
```

---

## Migration Path

### Step 1: Create schemas
- Define workflow-state.yaml schema
- Validate existing template schemas

### Step 2: Create phase prompts
- Extract logic from current discover.md
- Create 01-context.md through 06-gaps-docs.md

### Step 3: Rewrite orchestrator
- Update discover.md as orchestrator
- Implement phase routing logic

### Step 4: Add incremental logic
- Implement change detection
- Add phase dependency tracking

### Step 5: Update documentation
- Update skill descriptions
- Add CLI help

---

## Benefits

1. **Resumable** - Can stop and resume at any point
2. **Incremental** - Only re-analyze what changed
3. **Modular** - Each phase is self-contained
4. **Traceable** - Clear audit trail in workflow-state.yaml
5. **Consistent** - Follows same pattern as comply workflows
6. **Extensible** - Easy to add new phases or modify existing

---

## Complete System Model Summary

### Files Generated

| File | Purpose | Primary Audience |
|------|---------|------------------|
| `product.yaml` | Product vision, features, KPIs, roadmap | PMs, Stakeholders |
| `glossary.yaml` | Domain terminology, bounded contexts | Everyone |
| `user-journeys.yaml` | Personas, user flows, touchpoints | PMs, UX, Developers |
| `business.yaml` | Business context, stakeholders, processes | PMs, Compliance |
| `architecture.yaml` | Components, ADRs, data flows, APIs, resilience | Developers, Architects |
| `data.yaml` | Data inventory, classification, GDPR | Developers, Compliance |
| `actors.yaml` | Users, roles, permissions | Developers, Security |
| `boundaries.yaml` | Trust zones, security perimeters | Security, Architects |
| `integrations.yaml` | External services, DPAs | Developers, Compliance |
| `supply_chain.yaml` | SBOM, dependencies, risks | Security, DevOps |
| `controls.yaml` | Security controls in code | Security, Developers |
| `tooling.yaml` | CI/CD, collaboration tools | DevOps, Developers |
| `team.yaml` | Team structure, onboarding, contacts | Everyone |
| `operations.yaml` | Runbooks, monitoring, incidents | DevOps, On-Call |
| `gaps.yaml` | Identified gaps, remediation plan | All |
| `index.yaml` | Model overview, statistics | All |

### Documentation Generated

| Document | Source Files | Audience |
|----------|--------------|----------|
| **pm-onboarding.md** | product, business, glossary, user-journeys | Product Managers |
| **dev-onboarding.md** | architecture, team, tooling, glossary | New Developers |
| **architecture-guide.md** | architecture, data, integrations | Tech Leads |
| **security-overview.md** | controls, boundaries, data, actors | Security Team |
| **ops-runbook.md** | operations, team, tooling | DevOps, On-Call |
| **api-reference.md** | architecture (api_inventory) | API Consumers |
| **domain-guide.md** | glossary, data, user-journeys | Business Analysts |

---

## Template Examples

### PM Onboarding Guide Template

```markdown
# {{ product.name }} - PM Onboarding Guide

## What is {{ product.name }}?

{{ product.vision }}

### Value Proposition
- **For Users:** {{ product.value_proposition.for_users }}
- **For Business:** {{ product.value_proposition.for_business }}

## Key Features

{% for feature in features %}
### {{ feature.name }}
{{ feature.description }}

- **Status:** {{ feature.status }}
- **User Value:** {{ feature.user_value }}
- **Success Metrics:** {{ feature.metrics | join(", ") }}
{% endfor %}

## KPIs We Track

| Metric | Current | Target |
|--------|---------|--------|
{% for kpi in kpis.business %}
| {{ kpi.name }} | {{ kpi.current }} | {{ kpi.target }} |
{% endfor %}

## Stakeholders

{% for stakeholder in business.stakeholders %}
- **{{ stakeholder.role }}**: {{ stakeholder.concerns | join(", ") }}
{% endfor %}

## Glossary

{% for term in glossary.terms %}
- **{{ term.term }}**: {{ term.definition }}
{% endfor %}

## User Journeys

{% for journey in user_journeys.journeys %}
### {{ journey.name }}
{{ journey.description }}

**Trigger:** {{ journey.trigger }}
**Outcome:** {{ journey.outcome }}
{% endfor %}
```

### Developer Onboarding Guide Template

```markdown
# {{ product.name }} - Developer Onboarding

## Quick Start

### Prerequisites
- {{ tooling.development.ide_configs | join(", ") }}
- Access to: GitHub, Slack

### Setup

\`\`\`bash
# Clone the repository
git clone {{ business.repository }}

# Install dependencies
{{ team.onboarding.developer.day_1[1].command }}

# Run locally
make dev
\`\`\`

## Architecture Overview

{{ architecture.architecture_overview.description }}

### Tech Stack
- **Languages:** {{ architecture.tech_stack.languages | join(", ") }}
- **Frameworks:** {{ architecture.tech_stack.frameworks | join(", ") }}
- **Databases:** {{ architecture.tech_stack.databases | join(", ") }}

### Components

{% for comp in architecture.components %}
#### {{ comp.name }}
- **Type:** {{ comp.type }}
- **Technology:** {{ comp.technology }}
- **Location:** {{ comp.location }}
{% endfor %}

## Architecture Decisions (ADRs)

{% for adr in architecture.architecture_decisions %}
### {{ adr.id }}: {{ adr.title }}
- **Status:** {{ adr.status }}
- **Context:** {{ adr.context }}
- **Decision:** {{ adr.decision }}
{% endfor %}

## Domain Glossary

{% for term in glossary.terms %}
### {{ term.term }}
{{ term.definition }}

**Code:** `{{ term.code_references[0].file }}`
{% endfor %}

## Development Workflow

1. Create branch from `{{ tooling.source_control.default_branch }}`
2. Make changes
3. Run tests: `make test`
4. Submit PR
5. Wait for CI: {{ tooling.ci_cd.platform }}

## Getting Help

- **Slack:** #{{ team.communication.slack_channel }}
- **On-Call:** {{ team.on_call.rotation }}
```

---

## Implementation Roadmap

### Phase 1: Foundation (Week 1)
- [ ] Create workflow-state.yaml schema
- [ ] Create product.yaml schema and template
- [ ] Create glossary.yaml schema and template
- [ ] Create user-journeys.yaml schema and template
- [ ] Create operations.yaml schema and template

### Phase 2: Phase Prompts (Week 2)
- [ ] Write 01-product-context.md
- [ ] Write 02-architecture.md (enhanced)
- [ ] Write 03-domain-model.md
- [ ] Write 04-ecosystem.md
- [ ] Write 05-operations.md
- [ ] Write 06-synthesis.md

### Phase 3: Orchestrator (Week 3)
- [ ] Rewrite discover.md as orchestrator
- [ ] Implement phase routing
- [ ] Implement resume capability
- [ ] Implement incremental updates

### Phase 4: Documentation (Week 4)
- [ ] Create documentation templates
- [ ] Implement multi-audience generation
- [ ] Add validation and cross-referencing
- [ ] Testing and refinement
