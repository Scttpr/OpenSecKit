# Les 7 Principes de Sécurité

Ces principes forment le socle de sécurité de tout projet utilisant OpenSecKit.

## Vue d'Ensemble

| # | Principe | Commandes | Livrables |
|---|----------|-----------|-----------|
| I | Threat Modeling | `/osk-analyze` | `threats.md` |
| II | Risk Analysis | `/osk-analyze` | `risks.md`, `risk-register.yaml` |
| III | Security by Design | `/osk-specify` | `requirements.md` |
| IV | Security Testing | `/osk-specify` | `testing.md` |
| V | Secrets Management | `/osk-harden` | `hardening.md` |
| VI | Audit Logging | `/osk-harden` | `hardening.md` |
| VII | Patch Management | `/osk-harden` | `hardening.md` |

---

## I. Modélisation des Menaces

!!! info "Règle"
    Toute feature doit faire l'objet d'une analyse des menaces **AVANT** développement.

### Méthodologie STRIDE

| Catégorie | Question | Exemples |
|-----------|----------|----------|
| **S**poofing | Qui peut usurper une identité ? | Token forgery, session hijacking |
| **T**ampering | Quoi peut être altéré ? | Data modification, MITM |
| **R**epudiation | Quoi peut être nié ? | Missing logs |
| **I**nfo Disclosure | Quoi peut fuiter ? | Error messages, side channels |
| **D**enial of Service | Quoi peut être bloqué ? | Resource exhaustion |
| **E**levation | Qui peut escalader ? | IDOR, injection |

### Livrables

- Diagramme de flux (DFD) avec trust boundaries
- Analyse STRIDE par asset critique
- Attack trees pour les menaces principales

---

## II. Analyse de Risques

!!! info "Règle"
    Chaque risque doit être scoré et tracé dans le registre central.

### Scoring

```
Score = Impact × Probabilité × Exposition

Échelle 1-5 pour chaque facteur
Score max = 125
```

### Seuils et SLA

| Sévérité | Score | Priorité | SLA |
|----------|-------|----------|-----|
| CRITIQUE | ≥ 80 | P0 | 48h |
| IMPORTANT | 49-79 | P1 | 7 jours |
| MODÉRÉ | 25-48 | P2 | 30 jours |
| MINEUR | < 25 | P3-P4 | 90 jours |

### Livrables

- `risks.md` par feature (vue générée)
- `risk-register.yaml` (source unique)

---

## III. Sécurité dès la Conception

!!! info "Règle"
    Les contrôles de sécurité sont des exigences de première classe.

### Principes Fondamentaux

- **Moindre privilège** : droits strictement nécessaires
- **Défense en profondeur** : plusieurs couches de contrôles
- **Échec sécurisé** : refuser par défaut en cas d'erreur
- **Validation systématique** : ne jamais faire confiance aux entrées

### Checklist

- [x] Authentification robuste (MFA pour accès critiques)
- [x] Autorisation granulaire (RBAC)
- [x] Validation de toutes les entrées
- [x] Chiffrement transit (TLS 1.3) et repos (AES-256)
- [x] Protection OWASP Top 10

---

## IV. Tests de Sécurité

!!! info "Règle"
    Tests automatisés, bloquants pour le déploiement.

### Types de Tests

| Type | Quoi | Quand | Outils |
|------|------|-------|--------|
| SAST | Code source | Chaque commit | SonarQube, Semgrep, CodeQL |
| DAST | Application runtime | Staging | OWASP ZAP, Burp Suite |
| SCA | Dépendances | Quotidien | Dependabot, Snyk |

!!! warning "Règle de Blocage"
    Aucune vulnérabilité CRITIQUE ou HAUTE non résolue en production.

---

## V. Gestion des Secrets

!!! danger "Règle"
    Aucun secret dans le code. **Jamais.**

### À faire / À ne pas faire

| ❌ Interdit | ✅ Requis |
|-------------|-----------|
| Secrets en clair dans le code | Gestionnaire de secrets (Vault, KMS) |
| Secrets dans `.env` versionné | Rotation automatique (max 90 jours) |
| Clés API hardcodées | Scan pre-commit (gitleaks, trufflehog) |

### Infrastructure Requise

1. Gestionnaire de secrets (HashiCorp Vault, AWS Secrets Manager, Azure Key Vault)
2. Hooks pre-commit pour détection
3. Logs d'accès aux secrets

---

## VI. Traçabilité et Audit

!!! info "Règle"
    Tous les accès sensibles doivent être tracés de manière immuable.

### Événements à Logger

- Authentification (succès ET échecs)
- Modifications de données sensibles
- Changements de configuration
- Accès aux secrets
- Actions administratives

### Format Structuré

```json
{
  "timestamp": "2025-01-15T14:32:01Z",
  "trace_id": "abc-123",
  "event_type": "authentication_failure",
  "user_id": "user@example.com",
  "ip_address": "192.168.1.100",
  "severity": "warning"
}
```

### Rétention

- Minimum 1 an (RGPD)
- Stockage WORM ou SIEM centralisé

---

## VII. Patch Management

!!! info "Règle"
    Vulnérabilités connues = SLA de correction stricts.

### SLA Non Négociables

| CVSS | Sévérité | SLA |
|------|----------|-----|
| 9.0-10.0 | CRITIQUE | **48h** |
| 7.0-8.9 | HAUTE | **7 jours** |
| 4.0-6.9 | MOYENNE | **30 jours** |
| 0.1-3.9 | FAIBLE | Backlog |

### Processus

1. Scan quotidien automatique (npm audit, pip-audit)
2. Priorisation selon CVSS + contexte
3. Test de non-régression
4. Déploiement selon SLA

---

## Pondération par Projet

`/osk-configure` adapte le poids de chaque principe selon le contexte :

```toml
# .osk/config.toml
[principles]
I_threat_modeling = "high"      # Toujours critique
II_risk_analysis = "high"       # Toujours critique
III_security_design = "medium"  # Selon sensibilité données
IV_security_testing = "high"    # Bloquant pour prod
V_secrets_management = "high"   # Toujours critique
VI_audit_logging = "medium"     # Selon réglementations
VII_patch_management = "high"   # Toujours critique
```

---

## Validation Constitutionnelle

Avant chaque mise en production :

- [ ] Threat model documenté (Principe I)
- [ ] Risques scorés et validés (Principe II)
- [ ] Contrôles de sécurité implémentés (Principe III)
- [ ] SAST/DAST/SCA passés (Principe IV)
- [ ] Aucun secret dans le code (Principe V)
- [ ] Logs de sécurité actifs (Principe VI)
- [ ] Dépendances à jour (Principe VII)

!!! tip "Commande"
    Utilisez `/osk-dashboard` pour suivre la conformité aux 7 principes.

---

## Références

- [OWASP Top 10](https://owasp.org/www-project-top-ten/) - Vulnérabilités web
- [STRIDE](https://docs.microsoft.com/en-us/azure/security/develop/threat-modeling-tool-threats) - Modélisation des menaces
- [CVSS](https://www.first.org/cvss/) - Scoring des vulnérabilités
- [ISO 27001](https://www.iso.org/isoiec-27001-information-security.html) - Management de la sécurité
