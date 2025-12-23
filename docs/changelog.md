# Changelog

Historique des versions OpenSecKit.

## Version 3.0.1 - 2025-12-23

### Nouvelles fonctionnalités

- **`/osk-implement`** - Nouvelle commande pour exécuter les tâches
    - Exécute les tâches de `tasks.yaml` une par une
    - Un commit par tâche (traçabilité maximale)
    - Mise à jour automatique du `risk-register.yaml`
    - Options : `--auto`, `--dry-run`, `--task T001`

- **Analyse STRIDE système** dans `/osk-baseline`
    - Principes I et II démarrent à 30%
    - Identification précoce des menaces architecturales

### Améliorations

- **Risk Register - Source unique**
    - `docs/security/risks/risk-register.yaml` = SOURCE UNIQUE
    - Les fichiers `.osk/specs/*/risks.md` sont des vues générées

- **Architecture Templates refactorisée**
    - Prompts réduits de ~800 à ~100 lignes
    - Templates séparés : schemas/, outputs/, reports/
    - Réduction totale : -88%

- **Dashboard étendu**
    - Scanne tous les dossiers `docs/security/*`
    - Inclut conformité RGPD, RGS, continuité

### Changements

- `/osk-resolve` remplacé par `/osk-implement`

---

## Version 3.0.0 - 2025-12-23

### Breaking changes

- **Refonte des commandes**
    - ❌ Supprimées : `/security`, `/audit`
    - ✅ Nouvelles : `/osk-configure`, `/osk-baseline`, `/osk-analyze`, `/osk-specify`, `/osk-harden`, `/osk-plan`, `/osk-tasks`

- **Structure des fichiers**
    - Avant : `docs/security/features/SEC-*.md`
    - Après : `.osk/specs/NNN-feature/` + `docs/security/`

### Nouvelles fonctionnalités

- **Workflow orienté feature**
- **CLI `--default`** pour mode non-interactif
- **Scripts de test** : `test-local.sh`, `check-links.sh`

---

## Version 2.0.0 - 2025-01-15

### Breaking changes

- Réduction des commandes : 6 → 4
- Nouveau Risk Register centralisé

### Nouvelles fonctionnalités

- `/security` - Analyse constitutionnelle complète
- `/audit` - Vérification conformité
- `/dashboard` - Vue consolidée
- Risk Register avec scoring et lifecycle

---

## Liens

- [GitHub Releases](https://github.com/Scttpr/OpenSecKit/releases)
- [Guide de migration](getting-started/installation.md)
