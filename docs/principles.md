# Les 7 Principes de Sécurité

> Ces principes forment le socle de sécurité de tout projet utilisant OpenSecKit.

---

## Vue d'Ensemble

| # | Principe | Commandes OSK | Livrables |
|---|----------|---------------|-----------|
| I | Threat Modeling | `/osk-secure specify` | `security-spec.md`, `risks.yaml` |
| II | Risk Analysis | `/osk-secure specify` | `risks.yaml` |
| III | Security by Design | `/osk-secure specify` | `security-spec.md` |
| IV | Security Testing | `/osk-secure specify` | `security-spec.md` |
| V | Secrets Management | `/osk-secure specify` | `security-spec.md` |
| VI | Audit Logging | `/osk-secure specify` | `security-spec.md` |
| VII | Patch Management | `/osk-secure specify` | `security-spec.md` |

---

## I. Modélisation des Menaces

**Toute feature doit faire l'objet d'une analyse des menaces AVANT développement.**

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

**Chaque risque doit être scoré et tracé dans le registre central.**

### Scoring

```
Score = Impact × Probabilité × Exposition

Échelle 1-5 pour chaque facteur
Score max = 125
```

### Seuils

| Sévérité | Score | Action |
|----------|-------|--------|
| CRITIQUE | ≥ 75 | Validation Direction, SLA 48h |
| IMPORTANT | 25-74 | Validation Sécurité, SLA 7j |
| MINEUR | < 25 | Documentation, SLA 30j |

### Livrables

- `risks.yaml` par feature
- Registre de risques consolidé

---

## III. Sécurité dès la Conception

**Les contrôles de sécurité sont des exigences de première classe.**

### Principes Fondamentaux

- **Moindre privilège** : droits strictement nécessaires
- **Défense en profondeur** : plusieurs couches de contrôles
- **Échec sécurisé** : refuser par défaut en cas d'erreur
- **Validation systématique** : ne jamais faire confiance aux entrées

### Checklist

- [ ] Authentification robuste (MFA pour accès critiques)
- [ ] Autorisation granulaire (RBAC)
- [ ] Validation de toutes les entrées
- [ ] Chiffrement transit (TLS 1.3) et repos (AES-256)
- [ ] Protection OWASP Top 10

---

## IV. Tests de Sécurité

**Tests automatisés, bloquants pour le déploiement.**

### Types de Tests

| Type | Quoi | Quand | Outils |
|------|------|-------|--------|
| SAST | Code source | Chaque commit | SonarQube, Semgrep, CodeQL |
| DAST | Application runtime | Staging | OWASP ZAP, Burp Suite |
| SCA | Dépendances | Quotidien | Dependabot, Snyk |

### Règle de Blocage

> Aucune vulnérabilité CRITIQUE ou HAUTE non résolue en production.

---

## V. Gestion des Secrets

**Aucun secret dans le code. Jamais.**

### Règles

- Secrets en clair dans le code
- Secrets dans `.env` versionné
- Gestionnaire de secrets (Vault, KMS)
- Rotation automatique (max 90 jours)
- Scan pre-commit (gitleaks, trufflehog)

### Infrastructure Requise

1. Gestionnaire de secrets (HashiCorp Vault, AWS Secrets Manager, Azure Key Vault)
2. Hooks pre-commit pour détection
3. Logs d'accès aux secrets

---

## VI. Traçabilité et Audit

**Tous les accès sensibles doivent être tracés de manière immuable.**

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

**Vulnérabilités connues = SLA de correction stricts.**

### SLA Non Négociables

| CVSS | Sévérité | SLA |
|------|----------|-----|
| 9.0-10.0 | CRITIQUE | **48h** |
| 7.0-8.9 | HAUTE | **7 jours** |
| 4.0-6.9 | MOYENNE | **30 jours** |
| 0.1-3.9 | FAIBLE | Backlog |

### Processus

1. Scan quotidien automatique (cargo audit, pip-audit)
2. Priorisation selon CVSS + contexte
3. Test de non-régression
4. Déploiement selon SLA

---

## Pondération par Projet

`/osk-discover init` configure le poids de chaque principe selon le contexte :

```toml
# .osk/config.toml
[principles]
threat_modeling = "high"      # Toujours critique
risk_analysis = "high"        # Toujours critique
security_requirements = "medium"  # Selon sensibilité données
security_testing = "high"     # Bloquant pour prod
secrets_management = "high"   # Toujours critique
audit_logging = "medium"      # Selon réglementations
patch_management = "high"     # Toujours critique
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

> Utilisez `/osk-comply status` pour suivre la conformité, `/osk-comply rgpd` ou `/osk-comply rgs` pour les audits par domaine.

---

## Références

- **OWASP Top 10** : vulnérabilités web les plus critiques
- **STRIDE** : méthodologie de modélisation des menaces
- **CVSS** : scoring des vulnérabilités
- **ISO 27001** : management de la sécurité
