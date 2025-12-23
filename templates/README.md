# OpenSecKit Templates

Templates réutilisables pour les skills OSK. Les prompts sont légers (~100 lignes)
et référencent ces templates au lieu d'inclure le contenu inline.

## Structure

```
templates/
├── schemas/              # Schémas YAML (structures de données)
│   ├── risk-register.yaml    # Structure du registre central
│   ├── risk-entry.yaml       # Format d'un risque individuel
│   ├── feature-entry.yaml    # Format d'une feature
│   ├── requirement-entry.yaml # Format d'une exigence
│   ├── test-strategy.yaml    # Format stratégie de tests
│   ├── secret-entry.yaml     # Format inventaire secrets
│   ├── logging-event.yaml    # Format événement logging
│   ├── patch-entry.yaml      # Format dépendance à patcher
│   ├── plan-action.yaml      # Format action du plan
│   ├── task-entry.yaml       # Format tâche atomique
│   ├── resolution-entry.yaml # Format résolution risque
│   └── rgpd-treatment.yaml   # Format traitement RGPD
│
├── outputs/              # Templates de fichiers générés
│   ├── threats.md.tmpl       # Template pour threats.md
│   ├── risks.md.tmpl         # Template pour risks.md (vue)
│   ├── stride-system.md.tmpl # Template STRIDE système
│   ├── requirements.md.tmpl  # Template exigences
│   ├── testing.md.tmpl       # Template tests
│   ├── hardening.md.tmpl     # Template durcissement
│   ├── plan.md.tmpl          # Template plan
│   ├── tasks.md.tmpl         # Template tâches
│   ├── dashboard.md.tmpl     # Template dashboard
│   ├── context.md.tmpl       # Template contexte
│   ├── constitution.md.tmpl  # Template constitution
│   └── features.yaml.tmpl    # Template liste features
│
└── reports/              # Rapports terminaux
    ├── analyze-report.txt    # Rapport fin d'analyse
    ├── baseline-report.txt   # Rapport baseline
    ├── specify-report.txt    # Rapport spécifications
    ├── harden-report.txt     # Rapport durcissement
    ├── plan-report.txt       # Rapport plan
    ├── tasks-report.txt      # Rapport tâches
    ├── resolve-report.txt    # Rapport résolution
    ├── dashboard-report.txt  # Rapport dashboard
    ├── configure-report.txt  # Rapport configuration
    ├── pca-pra-report.txt    # Rapport PCA/PRA
    ├── rgpd-audit-report.txt # Rapport audit RGPD
    └── rgs-report.txt        # Rapport RGS
```

## Usage dans les prompts

Les prompts référencent les templates au lieu d'inclure le contenu inline :

```markdown
# Templates

**Charger depuis `.osk/templates/` :**
- `schemas/risk-entry.yaml` → format de chaque risque
- `outputs/threats.md.tmpl` → structure du fichier threats.md
- `reports/analyze-report.txt` → rapport terminal final

**Appliquer avec le contexte de l'analyse.**
```

## Syntaxe des templates

Les templates utilisent une syntaxe Handlebars-like :

- `{{variable}}` - Substitution simple
- `{{#each items}}...{{/each}}` - Itération
- `{{#if condition}}...{{/if}}` - Conditionnel
- `{{@index}}` - Index dans une boucle

### Exemple

```handlebars
# Risques - {{feature}}

{{#each risks}}
## {{id}} - {{title}}

| Attribut | Valeur |
|----------|--------|
| Sévérité | {{severity}} |
| Score | {{score}} |

{{/each}}
```

## Installation

Les templates sont copiés dans `.osk/templates/` lors de `osk init`.

## Ajouter un nouveau template

1. Créer le fichier dans le bon dossier (schemas/, outputs/, reports/)
2. Référencer dans le prompt correspondant
3. Tester avec `./scripts/test-local.sh`

## Principes de design

1. **Léger** : Prompts < 150 lignes, templates séparés
2. **Réutilisable** : Un schéma peut servir à plusieurs prompts
3. **Maintenable** : Modifier un format = modifier un seul fichier
4. **Traçable** : Chaque template documenté avec sa version
