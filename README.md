<p align="center">
  <h1 align="center">OpenSecKit</h1>
  <p align="center">
    <strong>Security as Code, Multi-Agent Ready</strong>
  </p>
  <p align="center">
    Framework de sécurité applicative multi-agent (Claude Code, Copilot, Cursor, Gemini)
  </p>
</p>

<p align="center">
  <a href="https://scttpr.github.io/OpenSecKit">Documentation</a> •
  <a href="#installation">Installation</a> •
  <a href="#quickstart">Quickstart</a> •
  <a href="#les-7-principes">Principes</a>
</p>

---

## Pourquoi OpenSecKit ?

- 🛡️ **7 principes constitutionnels** pour une sécurité complète
- 🤖 **Multi-Agent** : Claude Code, GitHub Copilot, Cursor, Gemini + `AGENTS.md` universel
- 🔌 **Extensible** : Ajouter un agent = 1 fichier config + 1 template (zéro code)
- 📄 **Documentation as Code** générée automatiquement
- ⚖️ **Conformité RGPD/RGS** intégrée

## Installation

```bash
# Installer le CLI
cargo install osk

# Initialiser un projet (sélection interactive d'agent)
cd mon-projet/
osk init

# Ou spécifier l'agent directement
osk init --agent copilot
osk init --agent cursor
osk init --all-agents
```

## Quickstart

```bash
# Lancer Claude Code
claude

# Configurer le projet
>>> /osk-configure

# Analyser une feature
>>> /osk-analyze "authentication"

# Définir les exigences
>>> /osk-specify "authentication"

# Durcissement
>>> /osk-harden "authentication"

# Planification
>>> /osk-plan "authentication"
>>> /osk-tasks "authentication"

# Implémentation
>>> /osk-implement "authentication"

# Dashboard
>>> /osk-dashboard
```

## Workflow

```
osk init → /osk-discover init → System Model
                ↓
        /osk-configure → /osk-baseline (optionnel)
                ↓
        /osk-analyze → /osk-specify → /osk-harden
                ↓
        /osk-plan → /osk-tasks → /osk-implement
                ↓
        /osk-rgpd | /osk-rgs → /osk-dashboard
```

## Discover Phase

Le Discover Phase permet de construire un modèle système structuré avant l'analyse de sécurité.

```bash
# 1. Initialiser le projet
osk init

# 2. Lancer la découverte interactive (dans Claude)
>>> /osk-discover init

# 3. Après modifications du code
>>> /osk-discover update

# 4. Valider le modèle
>>> /osk-discover validate

# 5. Générer la documentation
>>> /osk-discover docs
```

### CLI Utilities

```bash
osk scan --json          # Scanner les fichiers (respecte .gitignore)
osk id src/api/users.rs  # Générer un ID de composant
osk changes --json       # Lister les fichiers modifiés
osk validate system-model # Valider le modèle système
```

### Structure du Modèle

```
.osk/system-model/
├── index.yaml        # Résumé + références croisées
├── business.yaml     # Domaine, processus, parcours
├── architecture.yaml # Composants, flux de données
├── data.yaml         # Catégories de données, PII
├── actors.yaml       # Utilisateurs, rôles
├── trust.yaml        # Zones et frontières de confiance
├── integrations.yaml # Services externes
├── security.yaml     # Contrôles, outils
└── gaps.yaml         # Lacunes identifiées
```

## Conform Phase (Compliance)

Le Conform Phase permet d'évaluer la conformité réglementaire basée sur le modèle système complet.

```bash
# 1. Évaluation RGPD/GDPR
>>> /osk-comply rgpd

# 2. Évaluation RGS (secteur public français)
>>> /osk-comply rgs

# 3. Tableau de bord multi-framework
>>> /osk-comply status

# 4. Lister les frameworks disponibles
>>> /osk-comply list
```

### Flags d'évaluation

```bash
# Mise à jour incrémentale (après changements)
>>> /osk-comply rgpd --update

# Reprendre une évaluation interrompue
>>> /osk-comply rgpd --resume

# Exporter la documentation d'audit
>>> /osk-comply rgpd --export md
```

### Structure des Sorties

```
.osk/compliance/
├── assessment-rgpd.yaml       # Données d'évaluation RGPD
├── assessment-rgpd.md         # Rapport synthétique
├── gap-report-rgpd.md         # Analyse des écarts
├── sub-processor-register.md  # Registre Art. 28
├── assessment-rgs.yaml        # Données d'évaluation RGS
├── assessment-rgs.md          # Rapport synthétique
├── homologation-checklist.md  # Checklist homologation
├── system-perimeter.md        # Périmètre système RGS
└── exports/                   # Documents d'audit
    ├── rgpd-compliance-report-*.md
    └── dossier-homologation-rgs-*.md
```

### Extension de Framework

Ajoutez de nouveaux frameworks de conformité sans modification de code :

```bash
# Créer un nouveau framework
mkdir -p frameworks/mon-framework/templates
cp frameworks/_template/framework.yaml.example frameworks/mon-framework/framework.yaml

# Utiliser le nouveau framework
>>> /osk-comply mon-framework
```

Voir `docs/extending-frameworks.md` pour plus de détails.

## Les 7 Principes

| # | Principe | Description |
|---|----------|-------------|
| I | Threat Modeling | Analyse proactive des menaces (STRIDE) |
| II | Risk Analysis | Scoring et priorisation des risques |
| III | Security by Design | Exigences de sécurité dès la conception |
| IV | Security Testing | Tests SAST/DAST/SCA automatisés |
| V | Secrets Management | Aucun secret dans le code |
| VI | Audit Logging | Logs immuables et centralisés |
| VII | Patch Management | SLA stricts de mise à jour |

## Commandes

### Discovery
- `osk init` - Initialise le projet avec `.osk/`
- `/osk-discover init` - Découverte système interactive (3 phases)
- `/osk-discover update` - Mise à jour incrémentale (git-diff)
- `/osk-discover context` - Mise à jour contexte (hosting, équipe)
- `/osk-discover validate` - Validation du modèle
- `/osk-discover docs` - Génération documentation

### Configuration
- `/osk-configure` - Configure les principes
- `/osk-baseline` - État des lieux (projets existants)

### Analyse par Feature
- `/osk-analyze` - Menaces et risques (I, II)
- `/osk-specify` - Exigences et tests (III, IV)
- `/osk-harden` - Durcissement (V, VI, VII)

### Planification
- `/osk-plan` - Plan consolidé
- `/osk-tasks` - Tâches ordonnées
- `/osk-implement` - Exécution avec mise à jour risk-register

### Conformité
- `/osk-comply rgpd` - Évaluation RGPD interactive
- `/osk-comply rgs` - Évaluation RGS avec homologation
- `/osk-comply status` - Dashboard multi-framework
- `/osk-comply list` - Lister les frameworks disponibles

### Monitoring
- `/osk-dashboard` - Tableau de bord sécurité
- `/osk-pca-pra` - Plans de continuité

## Documentation

📚 [Documentation complète](https://scttpr.github.io/OpenSecKit)

## Contribuer

Voir [CONTRIBUTING](docs/development/contributing.md)

## License

MIT
