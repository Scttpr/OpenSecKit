# Templates

Architecture des templates OpenSecKit.

## Principe

Les prompts (~100 lignes) référencent des templates externes au lieu d'inclure le contenu inline. Cela permet :

- **Maintenabilité** - Un format à modifier = un fichier
- **Réutilisabilité** - Templates partagés entre commandes
- **Légèreté** - Prompts concis et lisibles

## Structure

```
templates/
├── schemas/              # Schémas YAML
│   ├── risk-register.yaml
│   ├── risk-entry.yaml
│   ├── requirement-entry.yaml
│   ├── task-entry.yaml
│   └── ...
│
├── outputs/              # Templates fichiers générés
│   ├── threats.md.tmpl
│   ├── risks.md.tmpl
│   ├── plan.md.tmpl
│   └── ...
│
└── reports/              # Rapports terminaux
    ├── analyze-report.txt
    ├── dashboard-report.txt
    └── ...
```

## Schemas

Définissent la structure des données YAML :

### risk-entry.yaml

```yaml
id: "RISK-[FEATURE]-[NNN]"
source: "[/osk-baseline|/osk-analyze feature]"
type: "[systeme|feature|vulnerabilite]"
feature: "[nom_feature]"
titre: "[Titre court]"
description: "[Description détaillée]"

# Scoring
impact: 1-5
probabilite: 1-5
exposition: 1-5
score_initial: 1-125

# Workflow
statut: "OUVERT"
dates:
  detection: "[DATE ISO]"
  echeance: "[DATE ISO]"
```

### task-entry.yaml

```yaml
id: "T[NNN]"
titre: "[Titre de la tâche]"
description: "[Description détaillée]"
status: "[todo|in_progress|done]"
effort: "[estimation]"
fichiers: []
risques_adresses: []
depends_on: []
instructions: "[Instructions détaillées]"
criteres_done: []
```

## Outputs

Templates pour les fichiers générés (syntaxe Handlebars) :

### threats.md.tmpl

```handlebars
# Menaces - {{feature}}

> Analyse STRIDE générée par OpenSecKit

## Vue d'Ensemble

{{#each stride_categories}}
### {{category}} - {{name}}

{{#each threats}}
#### {{id}} - {{title}}

| Attribut | Valeur |
|----------|--------|
| Sévérité | {{severity}} |
| Asset | {{asset}} |

{{/each}}
{{/each}}
```

## Reports

Rapports affichés dans le terminal après exécution :

### analyze-report.txt

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
/osk-analyze {{feature}} - Analyse Terminée
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

MENACES IDENTIFIÉES (STRIDE)
├── Spoofing              : {{stride.S}} menaces
├── Tampering             : {{stride.T}} menaces
...
```

## Syntaxe

Les templates utilisent une syntaxe Handlebars :

| Syntaxe | Description |
|---------|-------------|
| `{{variable}}` | Substitution simple |
| `{{#each items}}...{{/each}}` | Itération |
| `{{#if condition}}...{{/if}}` | Conditionnel |
| `{{#unless condition}}...{{/unless}}` | Conditionnel inversé |
| `{{@index}}` | Index dans une boucle |

## Usage dans les Prompts

```markdown
# Templates

**Charger depuis `.osk/templates/` :**
- `schemas/risk-entry.yaml` → format de chaque risque
- `outputs/threats.md.tmpl` → structure du fichier threats.md
- `reports/analyze-report.txt` → rapport terminal final

**Appliquer avec le contexte de l'analyse.**
```

## Ajouter un Template

1. Créer le fichier dans le bon dossier
2. Référencer dans le prompt correspondant
3. Tester avec `./scripts/test-local.sh`
