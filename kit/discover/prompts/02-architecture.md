# Phase 2: Architecture & Decisions

**Phase ID:** `architecture`
**Outputs:** `architecture.yaml` (enhanced with ADRs)
**Audiences Served:** Developers, Tech Leads, Architects

---

## Role

You are the **Architecture Analyst**. Your task is to map the technical architecture AND capture the reasoning behind decisions. You document not just WHAT exists, but WHY it was built this way.

**Tone**: Technical, precise. Understand patterns and trade-offs.

---

## Prerequisites

- Phase 1 completed: `product.yaml`, `business.yaml`, `glossary.yaml`
- Workflow state shows `product_context.status: completed`

---

## Steps

### Step 2.1: Quick Scan

**Goal**: Get an overview of the codebase structure.

```bash
osk scan --json
```

#### Detect Repository Type

```yaml
repo_type_detection:
  monorepo:
    indicators:
      - multiple package.json files
      - "packages/" or "apps/" directory
      - lerna.json, nx.json, turbo.json, pnpm-workspace.yaml
    tools: ["lerna", "nx", "turborepo", "pnpm workspaces"]

  polyrepo:
    indicators:
      - single package.json
      - no workspace configuration

  multi_service:
    indicators:
      - docker-compose.yml with multiple services
      - kubernetes manifests for multiple deployments
```

#### Present Scan Results

```
🔍 Quick Scan Results
=====================

Repository: monorepo (95% confidence)
  - Tool: Turborepo
  - Packages: 5 detected

Structure:
├── apps/
│   ├── web/          (Next.js frontend)
│   └── api/          (NestJS backend)
├── packages/
│   ├── shared/       (shared types)
│   ├── ui/           (component library)
│   └── config/       (shared config)

Confirm structure? [Y/n]
```

---

### Step 2.2: Component Analysis

**Goal**: Identify all system components and their responsibilities.

#### Component Detection Patterns

```yaml
component_detection:
  frontend:
    - pattern: "apps/web/**"
    - pattern: "src/frontend/**"
    - pattern: "client/**"
    - indicators: ["react", "vue", "angular", "svelte"]

  backend:
    - pattern: "apps/api/**"
    - pattern: "src/server/**"
    - pattern: "server/**"
    - indicators: ["express", "nestjs", "fastapi", "actix"]

  services:
    - pattern: "services/**"
    - pattern: "src/services/**"
    - docker-compose services

  workers:
    - pattern: "**/workers/**"
    - pattern: "**/jobs/**"
    - indicators: ["bull", "bullmq", "celery", "sidekiq"]

  databases:
    - prisma/schema.prisma
    - config/database.yml
    - src/db/**

  libraries:
    - packages/** (in monorepo)
    - lib/**
```

#### For Each Component Document

```yaml
component:
  id: "{{ generate_id }}"
  name: "{{ detected_name }}"
  type: "{{ service|frontend|backend|worker|database|library }}"
  description: "{{ inferred_or_ask }}"
  technology: "{{ detected_tech }}"
  location: "{{ file_path }}"

  interfaces:
    - type: "{{ REST|GraphQL|gRPC|WebSocket }}"
      protocol: "{{ HTTP|HTTPS|WS }}"
      authentication: "{{ JWT|Session|API Key|None }}"

  responsibilities:
    - "{{ inferred_from_code }}"

  owner: "{{ ask_user }}"
```

---

### Step 2.3: Tech Stack Detection

**Goal**: Catalog all technologies and understand the choices.

#### Detection Sources

```yaml
tech_detection:
  languages:
    - package.json → TypeScript/JavaScript
    - Cargo.toml → Rust
    - requirements.txt → Python
    - go.mod → Go
    - *.csproj → C#

  frameworks:
    - package.json dependencies
    - import statements analysis

  databases:
    - prisma/schema.prisma → provider
    - config files (database.yml, .env)
    - ORM dependencies

  infrastructure:
    - terraform/**
    - kubernetes/**
    - docker-compose.yml
    - serverless.yml
```

#### Present Tech Stack

```
🛠️ Tech Stack Detected
=======================

Languages:
  ✓ TypeScript (90% of codebase)
  ✓ SQL (Prisma migrations)

Frameworks:
  ✓ NestJS (backend)
  ✓ Next.js (frontend)
  ✓ Prisma (ORM)

Databases:
  ✓ PostgreSQL (primary)
  ✓ Redis (caching)

Infrastructure:
  ✓ AWS (detected from terraform)
  ✓ Docker
  ✓ Kubernetes

Confirm? [Y/n]
```

---

### Step 2.4: Architecture Decision Records (ADRs)

**Goal**: Capture the "why" behind architecture choices.

#### Find Existing ADRs

```yaml
adr_locations:
  - docs/adr/*.md
  - docs/architecture/decisions/*.md
  - ADR-*.md
  - architecture-decisions/**
```

#### Infer Decisions from Code

For patterns found, create implicit ADRs:

```yaml
inferred_decisions:
  - pattern: "prisma"
    decision: "Use Prisma as ORM"
    context: "Type-safe database access"

  - pattern: "bullmq"
    decision: "Use BullMQ for job queues"
    context: "Background job processing"

  - pattern: "event-sourcing"
    decision: "Event sourcing for domain X"
    context: "Audit trail and temporal queries"
```

#### ADR Format

```yaml
architecture_decisions:
  - id: "ADR-001"
    title: "{{ title }}"
    status: "{{ accepted|proposed|deprecated|superseded }}"
    date: "{{ date }}"
    deciders: ["{{ names }}"]

    context: |
      {{ why_was_decision_needed }}

    decision: |
      {{ what_was_decided }}

    consequences:
      positive:
        - "{{ benefit }}"
      negative:
        - "{{ trade_off }}"
      neutral:
        - "{{ observation }}"

    alternatives_considered:
      - option: "{{ alternative }}"
        reason_rejected: "{{ why_not }}"

    related_adrs: ["{{ ADR-xxx }}"]
```

#### Ask User for Missing Context

```
📋 Architecture Decision: Database Choice
==========================================

Detected: PostgreSQL with Prisma

Why was PostgreSQL chosen over alternatives?

1. Need for ACID transactions
2. Rich ecosystem and tooling
3. Team familiarity
4. Other: [specify]

> [User selects or adds]

What alternatives were considered?
> [User input: e.g., MongoDB, MySQL]

Why were they rejected?
> [User input]
```

---

### Step 2.5: Data Flow Mapping

**Goal**: Document how data moves through the system.

#### Detection Methods

1. **API routes** → Entry points
2. **Service calls** → Internal flows
3. **Database operations** → Data persistence
4. **Message queues** → Async flows
5. **External API calls** → Integration flows

#### Data Flow Structure

```yaml
data_flows:
  - id: "flow-001"
    name: "User Registration"
    description: "New user signs up"

    from: "frontend-web"
    to: "api-gateway"

    data:
      - "user_email"
      - "user_password"

    protocol: "HTTPS"

    encryption:
      enabled: true
      algorithm: "TLS"
      protocol: "TLS"
      version: "1.3"

    authentication: "None (public endpoint)"

    triggers:
      - "User submits registration form"

    downstream:
      - flow: "flow-002"
        description: "Send verification email"
```

---

### Step 2.6: API Inventory

**Goal**: Catalog all APIs with versioning and consumers.

#### Detection Sources

```yaml
api_detection:
  openapi:
    - openapi.yaml
    - swagger.json
    - "**/openapi/**"

  graphql:
    - schema.graphql
    - "**/*.graphql"

  routes:
    - src/api/**
    - src/routes/**
    - controllers/**
```

#### API Structure

```yaml
api_inventory:
  - id: "api-001"
    name: "User API"
    version: "v1"
    type: "REST"
    base_path: "/api/v1/users"

    authentication: "JWT"
    authorization: "RBAC"

    documentation: "docs/api/users.md"

    endpoints:
      - method: "GET"
        path: "/api/v1/users"
        description: "List users"
        permissions: ["admin"]

      - method: "POST"
        path: "/api/v1/users"
        description: "Create user"
        permissions: ["admin"]

    consumers:
      - name: "Web Frontend"
        type: "internal"
      - name: "Mobile App"
        type: "internal"
      - name: "Partner Integration"
        type: "external"

    deprecation_status: "active"
    versioning_strategy: "url"

    rate_limiting:
      enabled: true
      limit: "1000/hour"
```

---

### Step 2.7: Resilience Analysis

**Goal**: Document DR, backups, and high availability.

#### Detection Sources

```yaml
resilience_detection:
  backups:
    - terraform backup resources
    - AWS RDS snapshots
    - backup scripts

  ha:
    - load balancer configs
    - auto-scaling groups
    - replica configurations

  dr:
    - multi-region setup
    - failover configurations
```

#### Questions for User

```
🔄 Resilience Configuration
============================

Backup Strategy:
1. Database backup frequency?
   > [daily|hourly|continuous]

2. Backup retention period?
   > [30 days|90 days|1 year]

3. Last backup test date?
   > [date or "never tested"]

Disaster Recovery:
1. RTO (Recovery Time Objective)?
   > [1 hour|4 hours|24 hours]

2. RPO (Recovery Point Objective)?
   > [0 (no data loss)|1 hour|24 hours]

3. DR site location?
   > [same region|different region|different cloud]
```

---

## Output Generation

Generate `architecture.yaml` with all sections:

- `architecture_overview` (style, description, tech_stack)
- `components` (all detected components)
- `architecture_decisions` (ADRs)
- `data_flows` (with encryption details)
- `api_inventory` (all APIs)
- `resilience` (backup, DR, HA)

---

## Update Workflow State

```yaml
phases:
  architecture:
    status: "completed"
    completed_at: "{{ timestamp }}"
    output:
      - "architecture.yaml"
    result:
      component_count: {{ count }}
      api_count: {{ count }}
      adr_count: {{ count }}
      detected_stack: {{ array }}
```

---

## Validation Checklist

- [ ] All major components identified
- [ ] Tech stack confirmed
- [ ] At least 3 ADRs documented
- [ ] Data flows mapped
- [ ] API inventory complete
- [ ] Resilience configuration documented

---

## Next Phase

Proceed to **Phase 3: Domain Model & Actors** (`03-domain-model.md`)
