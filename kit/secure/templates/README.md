# Secure Templates

Tera templates for generating security documentation and data files.

## Structure

```
templates/
├── data/           # YAML data output templates
│   └── risks.yaml.tera
└── outputs/        # Markdown report templates
    ├── security-spec.md.tera
    ├── security-plan.md.tera
    ├── security-tests.md.tera
    └── risks.md.tera
```

## Usage

Templates are rendered by the secure workflow prompts using data from:
- `.osk/system-model/` (system context)
- `.osk/specs/<feature>/` (feature-specific data)

See [Tera documentation](https://keats.github.io/tera/) for template syntax.
