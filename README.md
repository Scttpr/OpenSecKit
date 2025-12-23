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
- 🤖 **Multi-Agent** : Claude Code, GitHub Copilot, Cursor, Gemini
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
osk init → /osk-configure → /osk-baseline (optionnel)
                ↓
        /osk-analyze → /osk-specify → /osk-harden
                ↓
        /osk-plan → /osk-tasks → /osk-implement
                ↓
        /osk-rgpd | /osk-rgs → /osk-dashboard
```

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

### Configuration
- `osk init` - Initialise le projet
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
- `/osk-rgpd` - RGPD (Registre, DPIA)
- `/osk-rgs` - RGS (EBIOS RM, Homologation)

### Monitoring
- `/osk-dashboard` - Tableau de bord sécurité
- `/osk-pca-pra` - Plans de continuité

## Documentation

📚 [Documentation complète](https://scttpr.github.io/OpenSecKit)

## Contribuer

Voir [CONTRIBUTING](docs/development/contributing.md)

## License

MIT
