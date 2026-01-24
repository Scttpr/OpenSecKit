# Phase 5: Operations & Controls

**Phase ID:** `operations`
**Outputs:** `controls.yaml`, `tooling.yaml`, `team.yaml`, `operations.yaml`
**Audiences Served:** Security, DevOps, SRE, Management, New Team Members

---

## Role

You are the **Operations Analyst**. Your task is to document how the system is operated, secured, and maintained. You capture the human and automated processes that keep the system running.

**Tone**: Practical, operations-focused. Document what people need to know to run this system.

---

## Prerequisites

- Phase 1-4 completed
- Workflow state shows previous phases completed

---

## Steps

### Step 5.1: Security Controls Inventory

**Goal**: Document all security controls in place.

#### Detection Sources

```yaml
control_detection:
  authentication:
    patterns:
      - "passport"
      - "auth0"
      - "firebase-auth"
      - "@clerk"
      - "next-auth"
    files:
      - "**/auth/**"
      - "**/middleware/**"

  authorization:
    patterns:
      - "rbac"
      - "casl"
      - "casbin"
      - "permission"
    files:
      - "**/policies/**"
      - "**/guards/**"

  encryption:
    patterns:
      - "crypto"
      - "bcrypt"
      - "argon2"
      - "encrypt"
    config:
      - "TLS/SSL certificates"
      - "KMS configurations"

  logging:
    patterns:
      - "winston"
      - "pino"
      - "bunyan"
      - "morgan"
    config:
      - "log shipping config"
      - "SIEM integration"

  network:
    files:
      - "nginx.conf"
      - "haproxy.cfg"
      - "security groups"
      - "network policies"
```

#### Control Categories

```
🔐 Security Controls Discovery
==============================

Authentication Controls:
✓ Password authentication (bcrypt detected)
✓ OAuth 2.0 (Google, GitHub)
? MFA - Not detected, is it implemented? [Y/n]
? Session management - JWT detected, expiry configured? [Y/n]

Authorization Controls:
✓ RBAC detected (3 roles: admin, user, viewer)
? Row-level security? [Y/n]
? API rate limiting? [Y/n]

Encryption Controls:
✓ TLS 1.3 (nginx config)
✓ Password hashing (bcrypt)
? Data at rest encryption? [Y/n]
? Field-level encryption for PII? [Y/n]

Input Validation:
✓ Zod schema validation detected
? SQL injection protection? [parameterized queries/ORM/n]
? XSS protection? [Y/n]

Confirm these findings? [Y/n]
```

#### Control Structure

```yaml
control:
  id: "ctrl-{{ generate_id }}"
  name: "{{ control_name }}"
  type: "{{ authentication|authorization|encryption|input_validation|logging|network }}"
  category: "{{ preventive|detective|corrective }}"

  implementation:
    status: "{{ implemented|partial|planned|not_applicable }}"
    method: "{{ description }}"
    technology: "{{ tech_used }}"
    location: "{{ file_path }}"

  components:
    - component_id: "{{ component_id }}"
      configuration: "{{ config_details }}"

  effectiveness:
    tested: {{ boolean }}
    last_test: "{{ date }}"
    test_result: "{{ pass|fail|partial }}"

  compliance_mapping:
    - framework: "{{ OWASP|SOC2|ISO27001 }}"
      control_id: "{{ control_id }}"

  owner: "{{ team_or_person }}"
  documentation: "{{ doc_link }}"
```

---

### Step 5.2: Development Tooling

**Goal**: Catalog all tools used in development and operations.

#### Detection Sources

```yaml
tooling_detection:
  ci_cd:
    files:
      - ".github/workflows/**"
      - ".gitlab-ci.yml"
      - "Jenkinsfile"
      - ".circleci/**"
      - "bitbucket-pipelines.yml"

  code_quality:
    files:
      - ".eslintrc*"
      - ".prettierrc*"
      - "sonar-project.properties"
      - ".pre-commit-config.yaml"

  security_scanning:
    files:
      - ".snyk"
      - "trivy.yaml"
      - ".gitleaks.toml"
      - "semgrep.yml"

  infrastructure:
    files:
      - "terraform/**"
      - "pulumi/**"
      - "cloudformation/**"
      - "ansible/**"

  containers:
    files:
      - "Dockerfile*"
      - "docker-compose*.yml"
      - "kubernetes/**"
      - "helm/**"

  monitoring:
    patterns:
      - "datadog"
      - "newrelic"
      - "prometheus"
      - "grafana"
      - "sentry"
```

#### Present Tooling Summary

```
🛠️ Development Tooling
======================

CI/CD:
✓ GitHub Actions (12 workflows)
  - Build & Test
  - Security Scan
  - Deploy to staging
  - Deploy to production

Code Quality:
✓ ESLint + Prettier
✓ TypeScript strict mode
✓ Husky pre-commit hooks
? SonarQube integration? [Y/n]

Security Scanning:
✓ Snyk (dependency scanning)
✓ Trivy (container scanning)
? SAST tool? [semgrep|codeql|none]
? Secret scanning? [gitleaks|trufflesecurity|none]

Infrastructure:
✓ Terraform (AWS)
✓ Docker
✓ Kubernetes (EKS)

Monitoring:
✓ Datadog (APM + Logs)
✓ Sentry (Error tracking)
? Uptime monitoring? [specify tool]

Confirm and add missing tools? [Y/n]
```

#### Tooling Structure

```yaml
tooling:
  ci_cd:
    platform: "{{ platform }}"
    config_location: "{{ path }}"
    pipelines:
      - name: "{{ pipeline_name }}"
        trigger: "{{ push|pr|schedule|manual }}"
        stages: ["{{ stage_list }}"]
        environments: ["{{ env_list }}"]

  code_quality:
    linting:
      tool: "{{ tool }}"
      config: "{{ config_path }}"
      enforcement:
        blocking: {{ boolean }}
        stage: "{{ pre-commit|ci }}"
    formatting:
      tool: "{{ tool }}"
      config: "{{ config_path }}"
    type_checking:
      tool: "{{ tool }}"
      strictness: "{{ strict|normal|loose }}"

  security_scanning:
    sast:
      tool: "{{ tool }}"
      config: "{{ config_path }}"
      enforcement:
        blocking: {{ boolean }}
        stage: "{{ stage }}"
        fail_strategy: "{{ fail|warn }}"
    dast:
      tool: "{{ tool }}"
      schedule: "{{ schedule }}"
    dependency_scanning:
      tool: "{{ tool }}"
      schedule: "{{ schedule }}"
    secret_scanning:
      tool: "{{ tool }}"
      pre_commit: {{ boolean }}
    container_scanning:
      tool: "{{ tool }}"
      registry_scanning: {{ boolean }}

  infrastructure:
    iac_tool: "{{ terraform|pulumi|cloudformation }}"
    container_orchestration: "{{ kubernetes|ecs|nomad }}"
    config_management: "{{ ansible|chef|puppet }}"

  monitoring:
    apm: "{{ tool }}"
    logging: "{{ tool }}"
    metrics: "{{ tool }}"
    error_tracking: "{{ tool }}"
    uptime: "{{ tool }}"
```

---

### Step 5.3: Team Structure

**Goal**: Document team organization and responsibilities.

#### Questions for User

```
👥 Team Structure
=================

1. Team size (total engineering)?
   > [number]

2. Team structure?
   [single-team|squads|chapters|pods]

3. On-call rotation exists? [Y/n]

4. Security champion program? [Y/n]

5. Key roles to document:
   (Enter names/teams for each)

   a. Engineering Lead:
      > [name/team]

   b. Security Owner:
      > [name/team]

   c. DevOps/SRE:
      > [name/team]

   d. On-call Primary:
      > [name/team]

   e. Incident Commander:
      > [name/team]
```

#### Team Structure

```yaml
team:
  organization:
    structure: "{{ type }}"
    total_size: {{ number }}
    teams:
      - name: "{{ team_name }}"
        focus: "{{ focus_area }}"
        size: {{ number }}
        lead: "{{ name }}"

  roles:
    - title: "{{ role_title }}"
      responsibilities:
        - "{{ responsibility }}"
      current_holder: "{{ name_or_team }}"
      backup: "{{ name_or_team }}"
      contact: "{{ contact_method }}"

  ownership:
    - component: "{{ component_id }}"
      team: "{{ team_name }}"
      primary_contact: "{{ name }}"

  communication:
    primary_channel: "{{ slack_channel }}"
    escalation_channel: "{{ channel }}"
    documentation: "{{ wiki_link }}"

  processes:
    code_review:
      required_approvers: {{ number }}
      codeowners_enabled: {{ boolean }}
    incident_response:
      documented: {{ boolean }}
      runbook_location: "{{ path }}"
    on_call:
      rotation_schedule: "{{ schedule }}"
      escalation_policy: "{{ policy }}"
```

---

### Step 5.4: Operational Procedures

**Goal**: Document how the system is operated day-to-day.

#### Environment Discovery

```
🌍 Environments
===============

Detected environments from config:
1. development (local)
2. staging (from CI config)
3. production (from CI config)

For each environment, please provide:

PRODUCTION:
1. URL: > [url]
2. Region: > [aws-region/gcp-region]
3. Access method: [vpn|bastion|direct]
4. Approval required for access? [Y/n]
5. Contains PII? [Y/n]

STAGING:
1. URL: > [url]
2. Data refresh policy: [production-copy|synthetic|manual]
3. Retention period: > [days]

Add more environments? [Y/n]
```

#### Monitoring Setup

```
📊 Monitoring Configuration
===========================

Dashboards:
1. Do you have operational dashboards? [Y/n]
   URLs: > [dashboard_urls]

Key Metrics:
Select metrics you track (or add custom):
[x] Request latency (p50, p95, p99)
[x] Error rate
[x] Request throughput
[ ] Database connections
[ ] Queue depth
[ ] Memory usage
[ ] CPU usage
[ ] Custom: ___

Alerting:
1. Alert channels: [slack|pagerduty|email|other]
2. Alert severity levels defined? [Y/n]
3. Runbooks linked to alerts? [Y/n]
```

#### Operations Structure

```yaml
operations:
  environments:
    - name: "{{ env_name }}"
      type: "{{ production|staging|development }}"
      url: "{{ url }}"
      region: "{{ region }}"
      provider: "{{ cloud_provider }}"
      access:
        method: "{{ method }}"
        requirements: ["{{ requirements }}"]
        approval_needed: {{ boolean }}
      infrastructure:
        compute: "{{ description }}"
        database: "{{ description }}"
        cache: "{{ description }}"
      data:
        contains_pii: {{ boolean }}
        data_refresh: "{{ policy }}"
        retention: "{{ period }}"

  monitoring:
    dashboards:
      - name: "{{ name }}"
        url: "{{ url }}"
        purpose: "{{ purpose }}"
        key_metrics: ["{{ metrics }}"]
    metrics:
      - name: "{{ metric_name }}"
        type: "{{ counter|gauge|histogram }}"
        description: "{{ description }}"
        normal_range: "{{ range }}"
        alert_threshold: "{{ threshold }}"
    logs:
      provider: "{{ provider }}"
      retention: "{{ period }}"
      pii_filtering: {{ boolean }}

  alerts:
    - name: "{{ alert_name }}"
      severity: "{{ critical|high|medium|low }}"
      condition: "{{ condition }}"
      notification:
        channels: ["{{ channels }}"]
        escalation: "{{ policy }}"
      runbook: "{{ runbook_id }}"

  on_call:
    rotation:
      provider: "{{ pagerduty|opsgenie|other }}"
      schedule_url: "{{ url }}"
      rotation_length: "{{ period }}"
    escalation_policy:
      - level: 1
        wait: "{{ duration }}"
        notify: "{{ target }}"
    expectations:
      response_time: "{{ duration }}"
      acknowledgment_time: "{{ duration }}"

  incident_management:
    severity_levels:
      - level: "SEV1"
        name: "Critical"
        description: "{{ description }}"
        response_time: "{{ duration }}"
        examples: ["{{ examples }}"]
    communication:
      channel: "{{ channel }}"
      status_page: "{{ url }}"
    postmortem:
      required_for: ["SEV1", "SEV2"]
      template: "{{ template_location }}"
      blameless: true

  deployments:
    strategy: "{{ blue-green|canary|rolling }}"
    frequency: "{{ frequency }}"
    rollback:
      method: "{{ method }}"
      max_time: "{{ duration }}"
```

---

### Step 5.5: Runbook Creation

**Goal**: Document operational procedures for common scenarios.

#### Common Runbook Templates

```yaml
runbook_templates:
  - id: "rb-deploy"
    name: "Production Deployment"
    trigger: "Scheduled release"
    steps:
      - "Verify tests passing"
      - "Create release branch"
      - "Deploy to staging"
      - "Run smoke tests"
      - "Deploy to production"
      - "Monitor dashboards"

  - id: "rb-rollback"
    name: "Production Rollback"
    trigger: "Critical issue in production"
    steps:
      - "Assess severity"
      - "Communicate status"
      - "Initiate rollback"
      - "Verify rollback"
      - "Post-incident review"

  - id: "rb-scale"
    name: "Emergency Scaling"
    trigger: "High traffic / resource exhaustion"
    steps:
      - "Identify bottleneck"
      - "Scale appropriate resource"
      - "Monitor improvement"
      - "Document capacity need"
```

#### Runbook Structure

```yaml
runbook:
  id: "{{ runbook_id }}"
  name: "{{ runbook_name }}"
  description: "{{ description }}"
  trigger: "{{ trigger_condition }}"
  severity: "{{ severity }}"
  owner: "{{ owner }}"
  last_updated: "{{ date }}"
  last_used: "{{ date }}"

  prerequisites:
    - "{{ prerequisite }}"

  diagnosis:
    - step: 1
      action: "{{ action }}"
      command: "{{ command }}"
      expected: "{{ expected_output }}"
      if_unexpected: "{{ next_action }}"

  resolution:
    - scenario: "{{ scenario }}"
      action: "{{ action }}"
      command: "{{ command }}"
      verification: "{{ verification }}"
      rollback: "{{ rollback_steps }}"

  escalation:
    when: "{{ escalation_trigger }}"
    to: "{{ escalation_target }}"
    method: "{{ escalation_method }}"

  post_incident:
    - "{{ post_incident_step }}"

  related_alerts: ["{{ alert_ids }}"]
```

---

## Output Generation

Generate the following files:

### controls.yaml
- All security controls with implementation status
- Compliance mappings
- Testing status

### tooling.yaml
- CI/CD configuration
- Code quality tools
- Security scanning tools
- Infrastructure tools
- Monitoring stack
- Collaboration tools

### team.yaml
- Team structure
- Roles and responsibilities
- Ownership mapping
- Communication channels

### operations.yaml
- Environments
- Monitoring configuration
- Alert definitions
- On-call procedures
- Incident management
- Runbooks
- Deployment procedures

---

## Update Workflow State

```yaml
phases:
  operations:
    status: "completed"
    completed_at: "{{ timestamp }}"
    output:
      - "controls.yaml"
      - "tooling.yaml"
      - "team.yaml"
      - "operations.yaml"
    result:
      control_count: {{ count }}
      tool_count: {{ count }}
      runbook_count: {{ count }}
      environment_count: {{ count }}
```

---

## Validation Checklist

- [ ] All security controls documented
- [ ] Tooling inventory complete
- [ ] Team structure documented
- [ ] At least 3 runbooks created
- [ ] On-call procedures documented
- [ ] Incident management process defined

---

## Next Phase

Proceed to **Phase 6: Synthesis & Documentation** (`06-synthesis.md`)
