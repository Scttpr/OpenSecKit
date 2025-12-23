# OpenSecKit Templates

Templates réutilisables pour les skills OSK.

## Structure

```
templates/
├── schemas/              # Schémas YAML (structures de données)
│   ├── risk-register.yaml    # Structure du registre central
│   └── risk-entry.yaml       # Format d'un risque individuel
├── outputs/              # Templates de fichiers générés
│   ├── threats.md.tmpl       # Template pour threats.md
│   └── risks.md.tmpl         # Template pour risks.md (vue)
└── reports/              # Rapports terminaux
    └── analyze-report.txt    # Rapport fin d'analyse
```

## Usage dans les prompts

Les prompts référencent les templates au lieu d'inclure le contenu inline :

```markdown
## Génération des fichiers

**Charger les templates depuis `.osk/templates/` :**

1. Schéma risque : `schemas/risk-entry.yaml`
2. Output threats : `outputs/threats.md.tmpl`
3. Rapport terminal : `reports/analyze-report.txt`

**Appliquer avec le contexte de l'analyse.**
```

## Syntaxe des templates

Les templates utilisent une syntaxe Handlebars-like :

- `{{variable}}` - Substitution simple
- `{{#each items}}...{{/each}}` - Itération
- `{{#if condition}}...{{/if}}` - Conditionnel

## Installation

Les templates sont copiés dans `.osk/templates/` lors de `osk init`.
