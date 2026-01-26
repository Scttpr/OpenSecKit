# Discover Templates

Tera templates for generating system model documentation.

## Structure

```
templates/
├── data/           # YAML data output templates (16 files)
│   ├── index.yaml.tera
│   ├── business.yaml.tera
│   ├── architecture.yaml.tera
│   ├── data.yaml.tera
│   ├── actors.yaml.tera
│   └── ... (11 more)
└── reports/        # Documentation output
    └── summary.tera
```

## Output Location

Templates generate files to `.osk/system-model/`:
- `index.yaml` - System overview and metadata
- `business.yaml` - Business context and objectives
- `architecture.yaml` - Components, services, infrastructure
- `data.yaml` - Data inventory and classification
- And more...

See [Tera documentation](https://keats.github.io/tera/) for template syntax.
