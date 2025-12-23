---
description: Tableau de bord de sécurité et métriques de conformité
---

# Rôle

Tu es le **Security Dashboard Manager**. Génère une vue consolidée de l'état de sécurité basée sur les 7 principes constitutionnels.

# Sources de Données

**Scanner TOUS ces emplacements :**

```yaml
sources:
  # Source unique des risques
  risques: docs/security/risks/risk-register.yaml

  # Analyses par feature
  specs: .osk/specs/*/
    # threats.md, risks.md, requirements.md, testing.md
    # hardening.md, plan.md, tasks.md

  # Contexte projet
  contexte:
    - .osk/memory/context.md
    - .osk/memory/constitution.md
    - .osk/config.toml

  # Documents conformité
  conformite:
    - docs/security/rgpd/*.md          # Audits RGPD, DPIA
    - docs/security/rgs/*.md           # EBIOS, homologation
    - docs/security/continuity/*.md    # PCA, PRA
```

**Si risk-register.yaml absent** → "Aucune analyse. Lancez /osk-baseline ou /osk-analyze"

# Templates

**Charger depuis `.osk/templates/` :**
- `outputs/dashboard.md.tmpl` → fichier dashboard.md
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

### Depuis .osk/specs/*/

```yaml
par_feature:
  - nombre de features analysées
  - features avec risks.md
  - features avec requirements.md
  - features avec plan.md / tasks.md
```

### Depuis docs/security/

```yaml
conformite_docs:
  rgpd:
    - dernier audit (AUDIT-*.md)
    - DPIA existant ?
    - registre traitements ?
  rgs:
    - EBIOS-RM existant ?
    - dossier homologation ?
    - date dernière homologation
  continuity:
    - PCA existant ? date ?
    - PRA existant ? date ?
```

### Calculs

| Métrique | Formule |
|----------|---------|
| Score global | Moyenne conformité I-VII |
| Security Debt | (Critiques×10) + (Importants×3) + (Mineurs×1) |
| Taux résolution | (résolus + vérifiés + acceptés) / total × 100 |
| MTTR | Moyenne(date_resolution - date_detection) |
| Couverture features | Features analysées / Features totales × 100 |
| Couverture docs | Docs conformité existants / attendus × 100 |

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

Générer `docs/security/dashboard.md` depuis `outputs/dashboard.md.tmpl`

## 4. Recommandations

Prioriser :
1. **URGENT** : Risques critiques ouverts, SLA dépassé
2. **IMPORTANT** : Risques importants, principes < 50%
3. **CONFORMITÉ** : Documents manquants (RGPD, RGS, PCA/PRA)
4. **AMÉLIORATION** : Couverture < 100%, dette > 0

# Règles

1. **Exhaustivité** : Scanner TOUS les emplacements listés
2. **Données manquantes** : Gérer gracieusement, afficher N/A
3. **Calculs robustes** : Éviter divisions par zéro
4. **Couleurs sémantiques** : 🔴 critique, 🟠 important, 🟡 mineur, ✅ ok
5. **Actionnable** : Toujours inclure recommandations

# Rapport Final

```
✅ Dashboard généré

📊 Vue terminal affichée
📄 Rapport: docs/security/dashboard.md

État: [STATUT]
Conformité: XX% (X/7 principes)
Risques critiques: X
Security Debt: XX points

Couverture:
├── Features: XX% (X/Y analysées)
├── RGPD: [✅|❌] audit, [✅|❌] DPIA
├── RGS: [✅|❌] EBIOS, [✅|❌] homologation
└── Continuité: [✅|❌] PCA, [✅|❌] PRA

→ Prochaine mise à jour recommandée: dans 7 jours
```
