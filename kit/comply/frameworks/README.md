# Compliance Frameworks

Compliance framework definitions and assessment workflows.

## Available Frameworks

| Framework | Directory | Description |
|-----------|-----------|-------------|
| **RGPD** | `rgpd/` | EU General Data Protection Regulation (2016/679) |
| **RGS** | `rgs/` | French Référentiel Général de Sécurité v2.0 |

## Shared Resources

| Directory | Purpose |
|-----------|---------|
| `_schema/` | Common framework schema definitions |
| `_shared/` | Shared knowledge (eIDAS levels, etc.) |

## Framework Structure

Each framework follows a standard structure:

```
<framework>/
├── framework.yaml       # Framework metadata
├── README.md           # Framework documentation
├── prompts/            # Workflow prompts (5 phases)
├── schemas/            # Data structure schemas
├── templates/          # Output templates
└── knowledge/          # Reference materials
```

## Adding New Frameworks

To add a new framework (e.g., ISO 27001, SOC 2):

1. Create directory under `frameworks/`
2. Define `framework.yaml` with metadata
3. Create 5-phase prompt workflow
4. Define schemas for assessment data
5. Create output templates
6. Add reference knowledge

See existing frameworks for examples.
