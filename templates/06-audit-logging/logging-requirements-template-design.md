---
title: "Exigences de journalisation d'audit - Phase de conception"
template_version: "1.0.0"
constitutional_principle: "VI - Journalisation et audit"
ssdlc_phase: "design"
last_updated: "2025-01-15"
reviewers:
  - security-champion-team
description: "Exigences de journalisation d'audit pour les événements de sécurité, les actions utilisateur et la conformité. Définit quoi journaliser, comment protéger les journaux et les politiques de rétention."
tags:
  - audit-logging
  - journalisation
  - surveillance
  - conformité
difficulty: "intermediate"
estimated_time: "1-2 hours"
prerequisites:
  - "Compréhension des opérations sensibles en sécurité"
  - "Exigences de conformité (RGPD, HIPAA, SOC 2)"
related_templates:
  - "siem-integration-guide.md"
compliance_frameworks:
  - "PCI-DSS Requirement 10"
  - "HIPAA § 164.312(b)"
  - "SOC 2 CC7.2"
---

# Exigences de journalisation d'audit - Phase de conception

## 1. Événements à journaliser

### Événements d'authentification

| Événement | Niveau de log | Champs |
|-----------|--------------|--------|
| Connexion réussie | INFO | ID utilisateur, email, timestamp, IP, user agent |
| Échec de connexion | WARNING | Email (tenté), IP, timestamp, raison (mauvais mot de passe, compte verrouillé) |
| Déconnexion | INFO | ID utilisateur, timestamp |
| Demande de réinitialisation mot de passe | INFO | Email, timestamp, IP |
| Mot de passe modifié | INFO | ID utilisateur, timestamp, IP |
| MFA activé/désactivé | INFO | ID utilisateur, timestamp, IP |
| Expiration de session | INFO | ID utilisateur, durée de session |

### Événements d'autorisation

| Événement | Niveau de log | Champs |
|-----------|--------------|--------|
| Accès accordé | DEBUG | ID utilisateur, ressource, action, timestamp |
| Accès refusé | WARNING | ID utilisateur, ressource tentée, action, timestamp, raison |
| Changement de permission | INFO | ID utilisateur modifié, anciennes permissions, nouvelles permissions, ID admin, timestamp |
| Changement de rôle | INFO | ID utilisateur, ancien rôle, nouveau rôle, ID admin, timestamp |

### Événements d'accès aux données

| Événement | Niveau de log | Champs |
|-----------|--------------|--------|
| Accès PII | INFO | ID utilisateur, ID propriétaire PII, type de données, timestamp |
| Export de données en masse | WARNING | ID utilisateur, nombre d'enregistrements, timestamp |
| Suppression de données | INFO | ID utilisateur, ID enregistrement, timestamp |
| Modification de données | DEBUG | ID utilisateur, ID enregistrement, champ modifié, timestamp |

### Événements administratifs

| Événement | Niveau de log | Champs |
|-----------|--------------|--------|
| Utilisateur créé | INFO | Nouvel ID utilisateur, créé par, timestamp |
| Utilisateur supprimé | INFO | ID utilisateur supprimé, supprimé par, timestamp |
| Usurpation d'identité admin | WARNING | ID admin, ID utilisateur usurpé, durée, timestamp |
| Changement de configuration système | WARNING | Clé config, ancienne valeur, nouvelle valeur, ID admin, timestamp |

---

## 2. Format des logs

**Journalisation structurée (JSON)** :

```json
{
  "timestamp": "2025-01-15T14:32:10.123Z",
  "level": "INFO",
  "event_type": "authentication.login.success",
  "user_id": "uuid-12345",
  "email": "user@example.com",
  "ip_address": "203.0.113.45",
  "user_agent": "Mozilla/5.0...",
  "session_id": "hash(session-id)", // haché pour la confidentialité
  "result": "success"
}
```

---

## 3. Ce qu'il NE faut PAS journaliser

❌ **Ne jamais journaliser** :
- Mots de passe (en clair ou hachés)
- Tokens de session
- Clés API
- Numéros de cartes de crédit (PCI-DSS)
- Numéros de sécurité sociale complets (HIPAA)
- Clés de chiffrement privées

---

## 4. Stockage et rétention des logs

| Type de log | Période de rétention | Emplacement de stockage | Chiffrement |
|-------------|---------------------|------------------------|-------------|
| Logs de sécurité (auth, accès) | 90 jours | SIEM (Elasticsearch/Splunk) | AES-256 au repos |
| Logs d'application | 30 jours | CloudWatch/Stackdriver | AES-256 au repos |
| Logs d'audit (conformité) | 7 ans | S3 Glacier / Azure Archive | AES-256 au repos |

---

## 5. Protection des logs

- ☑ **Immuabilité** : Stockage en écriture unique (SIEM, S3 Object Lock)
- ☑ **Intégrité** : Signature ou hachage des logs pour détecter les altérations
- ☑ **Contrôle d'accès** : Accès réservé aux administrateurs pour les logs de sécurité
- ☑ **Chiffrement** : Au repos (AES-256) et en transit (TLS)

---

## 6. Surveillance et alertes

**Alerter sur** :
- Multiples échecs de connexion (> 5 en 15 min) → Brute force potentiel
- Échecs d'autorisation répétés → Tentative d'escalade de privilèges potentielle
- Export de données en masse (> 1000 enregistrements) → Exfiltration de données potentielle
- Usurpation d'identité admin → Audit des actions admin
- Changements de permissions → S'assurer qu'ils sont autorisés

**Intégration SIEM** : Envoyer les logs vers Splunk / ELK / Azure Sentinel

---

## 7. Conformité

- **PCI-DSS Req 10** : Pistes d'audit pour tous les accès aux données de cartes
- **HIPAA § 164.312(b)** : Contrôles d'audit pour l'accès aux ePHI
- **SOC 2 CC7.2** : Surveillance des opérations système
- **RGPD Article 30** : Registre des activités de traitement

---

## 8. Checklist d'implémentation

- [ ] Implémenter la journalisation structurée (format JSON)
- [ ] Journaliser tous les événements d'authentification et d'autorisation
- [ ] Ajouter la journalisation des accès PII
- [ ] Envoyer les logs vers le SIEM
- [ ] Configurer la rétention des logs (90 jours sécurité, 7 ans audit)
- [ ] Activer le chiffrement des logs au repos
- [ ] Configurer les alertes pour les activités suspectes
- [ ] Tester l'intégrité des logs (vérifier l'absence d'altération)
- [ ] Documenter le processus de revue des logs (hebdomadaire par l'équipe sécurité)

---

## Ressources associées

- [Guide d'intégration SIEM](siem-integration-guide.md)
- [OWASP Logging Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Logging_Cheat_Sheet.html)
