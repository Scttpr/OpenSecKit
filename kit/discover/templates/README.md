# Discover Templates

Tera templates for generating system model and documentation.

## Structure

```
templates/
├── data/           # YAML data templates (16 files)
│   ├── index.yaml.tera
│   ├── business.yaml.tera
│   ├── product.yaml.tera
│   ├── glossary.yaml.tera
│   ├── architecture.yaml.tera
│   ├── data.yaml.tera
│   ├── actors.yaml.tera
│   ├── user-journeys.yaml.tera
│   ├── boundaries.yaml.tera
│   ├── integrations.yaml.tera
│   ├── controls.yaml.tera
│   ├── tooling.yaml.tera
│   ├── team.yaml.tera
│   ├── operations.yaml.tera
│   ├── supply_chain.yaml.tera
│   └── gaps.yaml.tera
└── outputs/        # Markdown documentation templates (6 files)
    ├── product.md.tera         # Product overview (for PMs, stakeholders)
    ├── developer.md.tera       # Developer guide (for developers)
    ├── architecture.md.tera    # Architecture guide (for architects)
    ├── security.md.tera        # Security overview (for security team)
    ├── operations.md.tera      # Operations handbook (for DevOps/SRE)
    └── onboarding.md.tera      # New team member guide
```

## Output Locations

### System Model (`.osk/system-model/`)

YAML files describing the discovered system:

| Template | Output | Purpose |
|----------|--------|---------|
| `index.yaml.tera` | `index.yaml` | Overview, metadata, statistics |
| `business.yaml.tera` | `business.yaml` | Business context, objectives |
| `product.yaml.tera` | `product.yaml` | Product identity, features, KPIs |
| `glossary.yaml.tera` | `glossary.yaml` | Domain terms, acronyms |
| `architecture.yaml.tera` | `architecture.yaml` | Components, APIs, data flows |
| `data.yaml.tera` | `data.yaml` | Data inventory, classification |
| `actors.yaml.tera` | `actors.yaml` | Users, roles, service accounts |
| `user-journeys.yaml.tera` | `user-journeys.yaml` | Personas, journeys, touchpoints |
| `boundaries.yaml.tera` | `boundaries.yaml` | Trust zones, boundaries |
| `integrations.yaml.tera` | `integrations.yaml` | External services, DPA status |
| `controls.yaml.tera` | `controls.yaml` | Security controls |
| `tooling.yaml.tera` | `tooling.yaml` | CI/CD, security tools |
| `team.yaml.tera` | `team.yaml` | Team structure, contacts |
| `operations.yaml.tera` | `operations.yaml` | Environments, runbooks, alerts |
| `supply_chain.yaml.tera` | `supply_chain.yaml` | SBOM, dependency policies |
| `gaps.yaml.tera` | `gaps.yaml` | Identified gaps |

### Documentation (`docs/`)

Markdown documentation for different audiences:

| Template | Output | Audience |
|----------|--------|----------|
| `product.md.tera` | `product.md` | Product Managers, stakeholders |
| `developer.md.tera` | `developer.md` | Developers |
| `architecture.md.tera` | `architecture.md` | Architects, senior developers |
| `security.md.tera` | `security.md` | Security team |
| `operations.md.tera` | `operations.md` | DevOps, SRE |
| `onboarding.md.tera` | `onboarding.md` | New team members |

Each document includes:
- **Dashboard** - Key metrics summary at the top
- **Complete content** - All relevant data from YAML files

## Template Syntax

See [Tera documentation](https://keats.github.io/tera/) for template syntax.

### Common Patterns

```tera
{# Iteration #}
{% for item in items %}
- {{ item.name }}
{% endfor %}

{# Conditionals #}
{% if value %}Yes{% else %}No{% endif %}

{# Default values #}
{{ field | default(value="Not specified") }}

{# Filters #}
{{ list | join(sep=", ") }}
{{ text | truncate(length=50) }}
{{ date | date(format="%Y-%m-%d") }}
```
