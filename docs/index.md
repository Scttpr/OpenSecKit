# OpenSecKit

<div align="center">
<h2>Security as Code, Multi-Agent Ready</h2>
<p><strong>Version 4.0.0</strong></p>
</div>

---

**OpenSecKit** est un framework de sécurité applicative qui structure la sécurité via **7 principes fondamentaux** et des **workflows guidés par IA**.

## Pourquoi OpenSecKit ?

- :shield: **7 principes constitutionnels** pour une sécurité complète
- :robot: **Intégration Claude Code** via slash commands
- :page_facing_up: **Documentation as Code** générée automatiquement
- :scales: **Conformité RGPD/RGS** intégrée

## Quickstart

```bash
# Installer le CLI
cargo install osk

# Initialiser un projet
cd mon-projet/
osk init

# Lancer Claude Code et utiliser les commandes
claude
>>> /osk-configure
>>> /osk-analyze "authentication"
```

## Workflow

```mermaid
graph LR
    A[osk init] --> B[osk-configure]
    B --> C[osk-baseline]
    C --> D[osk-analyze]
    D --> E[osk-specify]
    E --> F[osk-harden]
    F --> G[osk-plan]
    G --> H[osk-tasks]
    H --> I[osk-implement]
    I --> J[osk-dashboard]
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

## Liens rapides

<div class="grid cards" markdown>

-   :material-rocket-launch:{ .lg .middle } **Démarrage**

    ---

    Installation et premiers pas avec OpenSecKit

    [:octicons-arrow-right-24: Guide d'installation](getting-started/installation.md)

-   :material-book-open-variant:{ .lg .middle } **Concepts**

    ---

    Comprendre les principes et le workflow

    [:octicons-arrow-right-24: Les 7 Principes](concepts/principles.md)

-   :material-console:{ .lg .middle } **Commandes**

    ---

    Référence complète des slash commands

    [:octicons-arrow-right-24: Vue d'ensemble](commands/index.md)

-   :material-scale-balance:{ .lg .middle } **Conformité**

    ---

    RGPD, RGS et autres domaines réglementaires

    [:octicons-arrow-right-24: Domaines](domains/index.md)

</div>
