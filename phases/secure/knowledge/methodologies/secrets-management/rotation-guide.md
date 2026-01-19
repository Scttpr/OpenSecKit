---
title: "Politique de rotation des secrets"
template_version: "1.0.0"
constitutional_principle: "V - Gestion des secrets"
ssdlc_phase: "planning"
difficulty: "beginner"
estimated_time: "1 hour"
tags: ["secrets", "rotation", "policy", "governance"]
---

# Politique de rotation des secrets

## Métadonnées du projet

**Projet** : [Nom du projet]
**Date de création** : [AAAA-MM-JJ]
**Responsable** : [Nom du security champion]
**Dernière révision** : [AAAA-MM-JJ]
**Version** : 1.0.0

---

## 1. Objectif

Définir les règles de rotation automatique des secrets pour minimiser l'impact d'une compromission et respecter les exigences de conformité.

**Principe** : Un secret compromis doit avoir une durée de vie limitée. La rotation proactive réduit la fenêtre d'exposition.

---

## 2. Périmètre

Cette politique s'applique à tous les secrets utilisés par le projet :

- [ ] Clés API (third-party services, internal APIs)
- [ ] Mots de passe (bases de données, admin accounts)
- [ ] Certificats TLS/SSL
- [ ] Tokens d'authentification (JWT signing keys, OAuth tokens)
- [ ] Clés de chiffrement (encryption keys, signing keys)
- [ ] Service accounts credentials
- [ ] SSH keys

---

## 3. Classification des secrets

### Secrets critiques (rotation tous les 30 jours max)

**Définition** : Secrets dont la compromission a un impact catastrophique.

**Exemples** :
- [ ] Mot de passe root base de données production
- [ ] Clés de chiffrement des données utilisateur
- [ ] Tokens d'accès aux systèmes de paiement
- [ ] Certificats TLS des domaines principaux
- [ ] Service accounts avec privilèges admin

**Rotation** : Automatique, tous les 30 jours maximum
**Responsable** : Équipe Ops
**Approbation** : Security champion

---

### Secrets élevés (rotation tous les 90 jours max)

**Définition** : Secrets sensibles mais avec impact limité.

**Exemples** :
- [ ] Clés API services tiers (analytics, monitoring)
- [ ] Mots de passe bases de données staging
- [ ] Tokens CI/CD
- [ ] Service accounts avec privilèges limités
- [ ] SSH keys pour déploiements

**Rotation** : Automatique, tous les 90 jours maximum
**Responsable** : Équipe Ops
**Approbation** : Chef de projet

---

### Secrets moyens (rotation tous les 180 jours)

**Définition** : Secrets peu sensibles, impact faible.

**Exemples** :
- [ ] Clés API services non critiques
- [ ] Mots de passe environnements de développement
- [ ] Tokens de bots (Slack, Discord)
- [ ] Credentials lecture seule

**Rotation** : Manuelle ou automatique, tous les 180 jours
**Responsable** : Développeur assigné
**Approbation** : Non requise

---

## 4. Processus de rotation

### 4.1 Rotation automatique (recommandée)

**Fréquence** :
- Critiques : tous les 30 jours
- Élevés : tous les 90 jours
- Moyens : tous les 180 jours

**Mécanisme** :

1. **Génération** : Le gestionnaire de secrets génère un nouveau secret
2. **Déploiement** : Le nouveau secret est déployé (blue/green si possible)
3. **Période de transition** : Les deux secrets (ancien + nouveau) sont valides pendant 24h
4. **Révocation** : L'ancien secret est révoqué après 24h
5. **Notification** : Alertes envoyées en cas d'échec

**Outils** :
- HashiCorp Vault : Database Secrets Engine
- AWS Secrets Manager : Automatic rotation
- Azure Key Vault : Azure Functions pour rotation custom
- Kubernetes : External Secrets Operator

**Exemple de configuration (Vault)** :

```hcl
# Rotation automatique credentials PostgreSQL
vault write database/roles/myapp \
  db_name=postgresql \
  creation_statements="CREATE ROLE \"{{name}}\" WITH LOGIN PASSWORD '{{password}}' VALID UNTIL '{{expiration}}';" \
  default_ttl="30d" \
  max_ttl="90d"
```

---

### 4.2 Rotation manuelle (si automatique impossible)

**Processus** :

1. **Planification** : Créer un ticket 7 jours avant échéance
2. **Génération** : Générer le nouveau secret
3. **Test en staging** : Valider que l'application fonctionne avec le nouveau secret
4. **Déploiement production** : Mettre à jour le secret en production
5. **Validation** : Vérifier que tous les services fonctionnent
6. **Révocation** : Supprimer l'ancien secret
7. **Documentation** : Logger la rotation dans le registre

**Checklist de rotation manuelle** :

- [ ] Nouveau secret généré (complexité suffisante)
- [ ] Secret testé en staging
- [ ] Fenêtre de maintenance planifiée (si nécessaire)
- [ ] Équipe alertée du changement
- [ ] Ancien secret révoqué
- [ ] Tests post-rotation OK
- [ ] Logs audités

---

## 5. Cas particuliers

### 5.1 Certificats TLS/SSL

**Rotation** : 90 jours avant expiration

**Processus** :
1. Générer nouveau certificat
2. Déployer en parallèle (si possible)
3. Basculer le trafic
4. Révoquer ancien certificat

**Automatisation** : Let's Encrypt + Certbot (rotation auto tous les 60 jours)

---

### 5.2 Clés de chiffrement

**Rotation** : Selon réglementations (RGPD: recommandé annuellement)

**Processus** :
1. Générer nouvelle clé
2. Chiffrer nouvelles données avec nouvelle clé
3. Re-chiffrer données existantes (si requis)
4. Archiver ancienne clé (pour déchiffrement historique)

**⚠️ Attention** : Ne jamais supprimer une clé de chiffrement si des données chiffrées existent encore

---

### 5.3 Secrets compromis

**Rotation d'urgence** : < 1 heure

**Processus** :
1. **Détection** : Secret compromis identifié (leak GitHub, log exposé)
2. **Révocation immédiate** : Révoquer le secret dans les 15 minutes
3. **Génération** : Générer un nouveau secret
4. **Déploiement d'urgence** : Déployer le nouveau secret
5. **Investigation** : Analyser l'impact de la compromission
6. **Documentation** : Incident post-mortem

**Responsable** : On-call security champion

---

## 6. Exceptions

**Secrets non soumis à rotation** :

- [ ] Clés de chiffrement de backup (archivage légal)
- [ ] Certificats racine CA (rotation très rare)
- [ ] Secrets hardware (HSM master keys)

**Justification** : [Expliquer pourquoi ces secrets ne peuvent pas être rotés]

**Compensations** : [Contrôles additionnels pour mitiger le risque]

---

## 7. Responsabilités

| Rôle | Responsabilités |
|------|-----------------|
| **Security champion** | Définir la politique, auditer la conformité, approuver les exceptions |
| **Équipe Ops** | Configurer la rotation automatique, exécuter les rotations manuelles critiques |
| **Développeurs** | Implémenter le support de rotation dans le code, tester les rotations |
| **Chef de projet** | Allouer du temps pour implémentation/maintenance rotation |

---

## 8. Monitoring et alerting

### Métriques à suivre

- **Âge moyen des secrets** : < 90 jours
- **Taux de rotation automatique** : > 90%
- **Temps de rotation d'urgence** : < 1h
- **Échecs de rotation** : 0 par mois

### Alertes

**Critique (PagerDuty)** :
- Secret critique expire dans < 7 jours
- Échec de rotation automatique
- Secret compromis détecté

**Warning (Email)** :
- Secret élevé expire dans < 14 jours
- Rotation manuelle planifiée non exécutée
- Âge d'un secret > durée max définie

---

## 9. Audit et conformité

**Fréquence d'audit** : Trimestriel

**Checklist d'audit** :

- [ ] Tous les secrets critiques ont été rotés dans les 30 derniers jours
- [ ] Tous les secrets élevés ont été rotés dans les 90 derniers jours
- [ ] Aucun secret n'a dépassé sa durée de vie maximale
- [ ] Toutes les rotations sont loggées
- [ ] Aucune exception non documentée
- [ ] Tous les échecs de rotation ont été résolus

**Responsable** : Security champion
**Rapport à** : Direction

---

## 10. Formation

**Tous les développeurs doivent** :

- [ ] Comprendre pourquoi la rotation est nécessaire
- [ ] Savoir implémenter le code compatible avec la rotation
- [ ] Connaître la procédure de rotation d'urgence
- [ ] Avoir accès aux runbooks de rotation

**Formation annuelle obligatoire** : 1h sur la gestion et rotation des secrets

---

## 11. Documentation

**Runbooks à maintenir** :

- [ ] Comment roter manuellement chaque type de secret
- [ ] Procédure de rotation d'urgence (compromission)
- [ ] Troubleshooting des échecs de rotation
- [ ] Contacts on-call en cas de problème

**Localisation** : [Lien vers wiki/documentation]

---

## 12. Révision de la politique

**Fréquence** : Annuelle ou après incident de sécurité

**Déclencheurs de révision** :
- Compromission de secret
- Changement réglementaire
- Nouveau type de secret introduit
- Retour d'audit externe

**Processus** :
1. Revue par security champion
2. Validation par équipe Ops
3. Approbation par direction
4. Communication à toutes les équipes

---

## Signatures

**Créé par** : _________________________ Date : __________
**Approuvé par (Security champion)** : _________________________ Date : __________
**Approuvé par (Direction)** : _________________________ Date : __________

---

## Annexe : Registre de rotation

| Secret | Type | Criticité | Dernière rotation | Prochaine rotation | Automatique | Responsable |
|--------|------|-----------|-------------------|--------------------|-----------| ------------|
| `db-prod-password` | DB password | Critique | 2025-11-01 | 2025-12-01 | Oui | Ops |
| `stripe-api-key` | API key | Critique | 2025-10-15 | 2025-11-15 | Oui | Ops |
| `github-token-ci` | Token | Élevé | 2025-09-01 | 2025-12-01 | Non | DevOps |

**Mettre à jour ce registre après chaque rotation.**

---

**Prochaine étape** : Configurer la détection de secrets avec [secrets-detection-setup.md](secrets-detection-setup.md)
