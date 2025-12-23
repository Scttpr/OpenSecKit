# /osk-dashboard

Tableau de bord de sécurité.

## Synopsis

```bash
>>> /osk-dashboard
```

## Description

`/osk-dashboard` affiche une vue consolidée de la posture sécurité du projet.

## Données Scannées

- `docs/security/risks/risk-register.yaml`
- `.osk/specs/*/`
- `.osk/memory/`
- `docs/security/rgpd/`
- `docs/security/rgs/`
- `docs/security/continuity/`

## Fichiers Générés

| Fichier | Description |
|---------|-------------|
| `docs/security/dashboard.md` | Rapport Markdown |

## Métriques Affichées

### Conformité Constitutionnelle

Score de conformité aux 7 principes avec détail par principe.

### Registre des Risques

| Métrique | Description |
|----------|-------------|
| Ouverts | Risques non traités |
| En cours | Corrections en cours |
| Résolus | Corrections appliquées |
| Vérifiés | Tests validés |
| Acceptés | Risques acceptés |

### Couverture

- Features analysées
- Documents RGPD
- Documents RGS
- Plans de continuité

### Métriques Clés

| Métrique | Description |
|----------|-------------|
| Score Risque Résiduel | Somme des scores après mitigation |
| Security Debt | Points de dette sécurité |
| MTTR | Mean Time To Resolve |
| Taux de Résolution | % risques résolus |

## Exemple de sortie

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                OPENSECKIT SECURITY DASHBOARD
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Projet: mon-projet
Statut Production: 🟡 ATTENTION

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                CONFORMITÉ CONSTITUTIONNELLE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

                    ████████████░░░░░░░░ 65%

┌───────────────────────────────────────────────────────────┐
│ PRINCIPE                      │ STATUT  │ COUVERTURE     │
├───────────────────────────────┼─────────┼────────────────┤
│ I   Threat Modeling           │ ✅      │ 80%            │
│ II  Risk Analysis             │ ✅      │ 75%            │
│ III Security by Design        │ 🟡      │ 60%            │
│ IV  Security Testing          │ 🟡      │ 50%            │
│ V   Secrets Management        │ ✅      │ 90%            │
│ VI  Audit Logging             │ 🔴      │ 30%            │
│ VII Patch Management          │ ✅      │ 70%            │
└───────────────────────────────────────────────────────────┘

🔴 URGENT (< 48h)
  1. [RISK-AUTH-001] - Brute force sur login

🟠 IMPORTANT (< 7j)
  1. Compléter les tests de sécurité
  2. Activer logging centralisé

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## Prérequis

- `docs/security/risks/risk-register.yaml` présent
- `.osk/config.toml` présent
