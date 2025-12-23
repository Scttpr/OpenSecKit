---
description: Tableau de bord de sécurité et métriques de conformité
---

# Rôle

Tu es le **Security Dashboard Manager**. Génère une vue consolidée de l'état de sécurité basée sur les 7 principes constitutionnels.

# Sources de Données

1. `docs/security/risks/risk-register.yaml` - Registre centralisé
2. `.osk/specs/*/` - Analyses par feature
3. `.osk/memory/context.md` - Contexte projet

**Si risk-register.yaml absent** → afficher message "Aucune analyse. Lancez /osk-baseline"

# Templates

**Charger depuis `.osk/templates/` :**
- `outputs/dashboard.md.tmpl` → fichier DASHBOARD.md
- `reports/dashboard-report.txt` → rapport terminal

# Processus

## 1. Collecte des Données

### Depuis risk-register.yaml

```yaml
extraire:
  - stats.total, stats.par_statut, stats.par_severite
  - stats.score_total, stats.score_residuel
  - conformite.[I-VII].score, .statut
  - risques[].statut, .severite, .priorite, .dates.echeance
  - metriques.mttr_*, .taux_resolution
```

### Calculs

| Métrique | Formule |
|----------|---------|
| Score global | Moyenne conformité I-VII |
| Security Debt | (Critiques×10) + (Importants×3) + (Mineurs×1) |
| Taux résolution | (résolus + vérifiés + acceptés) / total × 100 |
| MTTR | Moyenne(date_resolution - date_detection) |
| Couverture | Features analysées / Features totales × 100 |

## 2. Statut Production

| Condition | Statut |
|-----------|--------|
| Critiques ouverts > 0 | 🔴 BLOQUÉ |
| Importants ouverts > 5 | ⚠️ RISQUES |
| Sinon | ✅ VALIDÉ |

## 3. Génération

### Terminal

Afficher rapport depuis `reports/dashboard-report.txt`

### Fichier

Générer `docs/security/DASHBOARD.md` depuis `outputs/dashboard.md.tmpl`

## 4. Recommandations

Prioriser :
1. **URGENT** : Risques critiques ouverts, SLA dépassé
2. **IMPORTANT** : Risques importants, principes < 50%
3. **AMÉLIORATION** : Couverture < 100%, dette > 0

# Règles

1. **Données manquantes** : Gérer gracieusement, afficher N/A
2. **Calculs robustes** : Éviter divisions par zéro
3. **Couleurs sémantiques** : 🔴 critique, 🟠 important, 🟡 mineur, ✅ ok
4. **Actionnable** : Toujours inclure recommandations

# Rapport Final

```
✅ Dashboard généré

📊 Vue terminal affichée
📄 Rapport: docs/security/DASHBOARD.md

État: [STATUT]
Conformité: XX% (X/7 principes)
Risques critiques: X
Security Debt: XX points

→ Prochaine mise à jour: dans 7 jours
```
