# Phase 3: Domain Model & Actors

**Phase ID:** `domain_model`
**Outputs:** `data.yaml`, `actors.yaml`, `boundaries.yaml`, `user-journeys.yaml`
**Audiences Served:** PMs, Developers, Analysts, Security

---

## Role

You are the **Domain Analyst**. Your task is to deeply understand the data model, who interacts with the system, and how users accomplish their goals. You bridge business concepts with technical implementation.

**Tone**: Analytical, user-focused. Connect code to real-world behavior.

---

## Prerequisites

- Phase 1 completed: `product.yaml`, `business.yaml`, `glossary.yaml`
- Phase 2 completed: `architecture.yaml`
- Workflow state shows previous phases completed

---

## Steps

### Step 3.1: Entity Extraction

**Goal**: Discover all domain entities from code.

#### Detection Sources

```yaml
entity_sources:
  prisma:
    file: "prisma/schema.prisma"
    pattern: "model (\\w+)"

  typeorm:
    files: "src/entities/**/*.ts"
    pattern: "@Entity()"

  sequelize:
    files: "src/models/**/*.js"
    pattern: "sequelize.define"

  mongoose:
    files: "src/models/**/*.ts"
    pattern: "mongoose.Schema"

  typescript:
    files: "src/types/**/*.ts"
    pattern: "interface (\\w+)"
```

#### For Each Entity

```yaml
entity:
  name: "{{ entity_name }}"
  description: "{{ from_comments_or_ask }}"
  source_file: "{{ file_path }}"

  fields:
    - name: "{{ field_name }}"
      type: "{{ field_type }}"
      required: {{ boolean }}
      unique: {{ boolean }}

  relationships:
    - type: "{{ one-to-many|many-to-many|one-to-one }}"
      target: "{{ other_entity }}"
      field: "{{ field_name }}"

  # Map to glossary
  glossary_term: "{{ term_id }}"
```

---

### Step 3.2: Data Classification

**Goal**: Classify data sensitivity and compliance requirements.

#### PII Detection

```yaml
pii_patterns:
  high_sensitivity:
    field_names: ["ssn", "social_security", "tax_id", "passport", "national_id"]
    annotations: ["@HighlyConfidential", "@Restricted"]

  personal:
    field_names: ["email", "phone", "address", "name", "dob", "date_of_birth"]
    annotations: ["@PII", "@Personal"]

  financial:
    field_names: ["credit_card", "bank_account", "salary", "income"]
    annotations: ["@Financial", "@PCI"]

  health:
    field_names: ["diagnosis", "prescription", "medical", "health"]
    annotations: ["@PHI", "@Health"]
```

#### Classification Process

```
🔒 Data Classification
=======================

Entity: User
Fields analyzed: 12

High Sensitivity (1):
  ⚠️ ssn - Social Security Number
     Recommendation: Encrypt at rest, mask in logs

Personal Data (4):
  📧 email - User email address
  📱 phone - Phone number
  🏠 address - Physical address
  👤 name - Full name

Standard (7):
  ✓ id, created_at, updated_at, status, role, preferences, settings

GDPR Applicable: Yes (PII detected)
DPIA Required: Review recommended (SSN handling)

Confirm classification? [Y/n]
```

#### Data Category Output

```yaml
data_categories:
  - id: "cat-user-pii"
    name: "User Personal Information"
    classification: "confidential"

    fields:
      - name: "email"
        type: "string"
        pii: true
        sensitive: false
        encryption_required: true

    retention:
      period: "account_lifetime + 30_days"
      legal_basis: "Contract fulfillment"
      deletion_method: "hard_delete"

    legal_basis: "contract"
    dpia_required: false
    impact_if_breached: "high"
```

---

### Step 3.3: Persona Discovery

**Goal**: Identify user personas from code and analytics.

#### Detection Sources

```yaml
persona_detection:
  auth_roles:
    - pattern: "role.*admin|admin.*role"
    - pattern: "role.*user|user.*role"
    - pattern: "permissions"

  user_types:
    - enum values in auth code
    - user.type field values
    - different login flows

  analytics:
    - segment traits
    - mixpanel user properties
```

#### For Each Persona

```
👤 Persona Discovery
=====================

Detected user types from code:
1. admin (from role enum)
2. user (from role enum)
3. api_consumer (from auth middleware)

For "admin" persona:

1. What is this user's primary goal?
   > [User input: e.g., "Manage platform and users"]

2. What are their main pain points?
   > [User input]

3. Technical level? [high|medium|low]
   > [User selection]

4. Usage frequency? [daily|weekly|monthly]
   > [User selection]
```

---

### Step 3.4: User Journey Mapping

**Goal**: Document how users accomplish their goals.

#### Detection Process

1. **Identify entry points** from architecture.yaml
2. **Trace API calls** for each user action
3. **Map data touched** at each step
4. **Identify failure points**

#### Journey Detection

```yaml
journey_patterns:
  authentication:
    entry: "/login|/auth|/signin"
    steps: ["enter_credentials", "verify", "create_session"]

  registration:
    entry: "/register|/signup"
    steps: ["enter_info", "verify_email", "complete_profile"]

  checkout:
    entry: "/checkout"
    steps: ["review_cart", "enter_payment", "confirm", "receipt"]

  onboarding:
    entry: "/onboarding|/setup"
    steps: ["welcome", "configure", "invite_team", "complete"]
```

#### Journey Documentation

```
🗺️ Journey: User Registration
==============================

Persona: New User
Trigger: User clicks "Sign Up"
Outcome: User has active account

Steps detected:
1. POST /api/auth/register
   → Creates: User record
   → Integration: None

2. POST /api/email/verify
   → Integration: SendGrid
   → Creates: Verification token

3. GET /api/auth/verify/:token
   → Updates: User.verified = true

4. PUT /api/users/me/profile
   → Updates: User profile

Confirm this journey? [Y/n]

What metrics track success?
> [User input: e.g., "signup_completion_rate"]

Common failure points?
> [User input: e.g., "Email not received"]
```

---

### Step 3.5: Actor Identification

**Goal**: Document all system actors (human and machine).

#### Detection Sources

```yaml
actor_detection:
  human:
    - auth roles and permissions
    - user type enums
    - UI access patterns

  machine:
    - service accounts
    - API keys
    - cron jobs
    - webhooks
```

#### Actor Documentation

```yaml
actors:
  user_types:
    - id: "actor-admin"
      name: "Administrator"
      type: "human"
      description: "Platform administrator"
      trust_level: 4

      authentication:
        method: "password + MFA"
        mfa_required: true
        session_duration: "8h"

      authorization:
        model: "RBAC"
        default_permissions: ["read:*", "write:*", "admin:*"]

      data_access: ["all_user_data", "system_config", "audit_logs"]
      features_accessible: ["admin_dashboard", "user_management"]

  service_accounts:
    - id: "sa-cron"
      name: "Cron Service"
      purpose: "Execute scheduled jobs"
      permissions: ["execute:jobs", "read:data"]
      rotation_policy: "90_days"
```

---

### Step 3.6: Trust Boundary Definition

**Goal**: Map security perimeters.

#### Detection Sources

```yaml
boundary_detection:
  network:
    - kubernetes namespaces
    - docker networks
    - terraform vpc/subnet configs

  application:
    - middleware patterns
    - auth guards
    - rate limiters
```

#### Boundary Structure

```yaml
trust_zones:
  - id: "zone-public"
    name: "Public Internet"
    trust_level: 0
    components: ["cdn", "load-balancer"]

  - id: "zone-dmz"
    name: "DMZ"
    trust_level: 1
    components: ["api-gateway", "web-server"]

  - id: "zone-internal"
    name: "Internal Network"
    trust_level: 3
    components: ["api-service", "worker"]

  - id: "zone-data"
    name: "Data Zone"
    trust_level: 4
    components: ["database", "cache"]

trust_boundaries:
  - id: "boundary-public-dmz"
    from_zone: "zone-public"
    to_zone: "zone-dmz"
    protocol: "HTTPS"
    controls:
      authentication: "none"
      rate_limiting: true
      waf: true
```

---

## Output Generation

### Templates

**Fetch YAML templates before generating outputs** to ensure correct structure:

| Output | Template URL |
|--------|--------------|
| `data.yaml` | `https://raw.githubusercontent.com/Scttpr/OpenSecKit/main/kit/discover/templates/data/data.yaml.tera` |
| `actors.yaml` | `https://raw.githubusercontent.com/Scttpr/OpenSecKit/main/kit/discover/templates/data/actors.yaml.tera` |
| `boundaries.yaml` | `https://raw.githubusercontent.com/Scttpr/OpenSecKit/main/kit/discover/templates/data/boundaries.yaml.tera` |
| `user-journeys.yaml` | `https://raw.githubusercontent.com/Scttpr/OpenSecKit/main/kit/discover/templates/data/user-journeys.yaml.tera` |

Generate the following files:

### data.yaml
- Data categories with classification
- PII/sensitive field mapping
- Retention policies
- GDPR compliance flags

### actors.yaml
- User types with personas reference
- Roles and permissions
- Service accounts

### boundaries.yaml
- Trust zones
- Trust boundaries with controls

### user-journeys.yaml
- Personas (detailed)
- User journeys with steps
- Touchpoints
- Success metrics

---

## Update Workflow State

```yaml
phases:
  domain_model:
    status: "completed"
    completed_at: "{{ timestamp }}"
    output:
      - "data.yaml"
      - "actors.yaml"
      - "boundaries.yaml"
      - "user-journeys.yaml"
    result:
      data_category_count: {{ count }}
      actor_count: {{ count }}
      journey_count: {{ count }}
      pii_detected: {{ boolean }}
```

---

## Validation Checklist

- [ ] All entities discovered and classified
- [ ] PII fields identified and marked
- [ ] At least 2 personas defined
- [ ] At least 3 user journeys mapped
- [ ] Trust zones defined
- [ ] Boundaries between zones documented

---

## Next Phase

Proceed to **Phase 4: Ecosystem & Dependencies** (`04-ecosystem.md`)
