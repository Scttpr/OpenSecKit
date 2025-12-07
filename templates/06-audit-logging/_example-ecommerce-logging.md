---
title: "Exemple : Audit logging pour une plateforme e-commerce"
template_version: "1.0.0"
constitutional_principle: "VI - Journalisation et audit"
project: "ShopSecure - Plateforme e-commerce"
ssdlc_phase: "implementation"
tags: ["example", "ecommerce", "logging", "siem", "elk"]
---

# Exemple : Audit logging pour ShopSecure

**Projet** : ShopSecure - Plateforme e-commerce B2C
**Équipe** : 5 développeurs + 1 SRE
**Stack** : Node.js, PostgreSQL, Redis
**Volume** : 50 000 transactions/mois, 100 000 visiteurs/mois

---

## Contexte

ShopSecure avait initialement des logs basiques (console.log) sans centralisation ni structure. Après un incident de fraude non détecté pendant 3 jours, l'équipe a décidé d'implémenter une stratégie d'audit logging complète.

**Problèmes rencontrés** :
- ❌ Logs éparpillés sur 5 serveurs
- ❌ Format texte non structuré, impossible à analyser
- ❌ Aucune alerte sur activité suspecte
- ❌ Investigation manuelle = SSH sur chaque serveur
- ❌ Perte de logs lors des redémarrages

---

## 1. Décisions d'architecture

### Choix de la stack de logging

**Stack retenue** : Winston → Filebeat → Elasticsearch → Kibana (ELK)

**Raisons** :
- Open source (budget limité)
- Bonne performance pour le volume actuel
- Équipe déjà familière avec JavaScript
- Pas besoin de corrélation avancée (pas encore de SIEM complet)

**Alternatives considérées** :
- ❌ Splunk : trop coûteux ($5k+/an)
- ❌ Datadog : préférence self-hosted pour contrôle des données
- ❌ Loki : équipe préfère Elasticsearch (connu)

---

### Organisation des logs

**Catégories définies** :

| Catégorie | Niveau | Volume/jour | Rétention |
|-----------|--------|-------------|-----------|
| Sécurité (auth, accès) | INFO/WARN | 5000 événements | 365 jours |
| Transactions (commandes, paiements) | INFO | 2000 événements | 365 jours |
| Application (erreurs, perf) | INFO/ERROR | 10000 événements | 30 jours |
| Accès HTTP | DEBUG | 100000 événements | 7 jours |

**Index Elasticsearch** :
- `shopsecure-security-*` : Événements de sécurité
- `shopsecure-transactions-*` : Commandes et paiements
- `shopsecure-app-*` : Logs applicatifs
- `shopsecure-access-*` : Logs HTTP (nginx)

---

## 2. Événements de sécurité journalisés

### Authentification

**Événements** :
- Connexion réussie (INFO)
- Échec de connexion (WARNING)
- Déconnexion (INFO)
- Réinitialisation mot de passe (INFO)
- Changement de mot de passe (INFO)
- Activation/désactivation MFA (INFO)

**Champs capturés** :
- User ID, email
- IP source, user agent
- Résultat (succès/échec)
- Raison d'échec (si applicable)
- Timestamp

---

### Transactions

**Événements** :
- Création de commande (INFO)
- Paiement traité (INFO)
- Paiement échoué (WARNING)
- Remboursement (INFO)

**Champs capturés** :
- User ID, order ID
- Montant, devise
- Méthode de paiement (type seulement, pas le numéro)
- Statut
- Timestamp

**⚠️ Ce qui N'est PAS journalisé** :
- ❌ Numéros de carte complets (PCI-DSS)
- ❌ CVV
- ❌ Tokens de paiement Stripe

---

### Actions administratives

**Événements** :
- Création/suppression utilisateur (WARNING)
- Changement de rôle (WARNING)
- Modification de produit (INFO)
- Export de données (WARNING)

**Champs capturés** :
- Admin ID
- Action effectuée
- Ressource cible (user ID, product ID)
- Anciennes/nouvelles valeurs (pour changements)
- Timestamp

---

## 3. Centralisation et agrégation

### Infrastructure

**Architecture déployée** :
```
Application (5 serveurs) → Filebeat → Logstash → Elasticsearch (3 nœuds) → Kibana
                                          ↓
                                    S3 (archive)
```

**Configuration** :
- Winston : journalisation structurée JSON dans fichiers locaux
- Filebeat : lecture des fichiers, envoi vers Logstash
- Logstash : parsing, enrichissement (geo-IP), routage vers index
- Elasticsearch : stockage, indexation
- Kibana : visualisation, recherche, alertes

**Volumes** :
- ~17 000 événements/jour
- ~500 MB/jour (compressé)
- ~180 GB/an de stockage

---

## 4. Alertes configurées

### Alerte 1 : Brute force login

**Déclencheur** : ≥ 5 échecs de connexion en 5 minutes (même IP ou même email)

**Action** :
- Notification Slack #security
- Blocage IP temporaire (30 min)
- Si > 10 tentatives : ticket Jira automatique

**Résultats** :
- 2-3 alertes/semaine (tentatives légitimes : utilisateurs oubliant leur mot de passe)
- 1 vrai brute force détecté et bloqué en 3 mois

---

### Alerte 2 : Transaction anormalement élevée

**Déclencheur** : Paiement > 2000€ (seuil inhabituel pour le site)

**Action** :
- Email équipe fraude
- Validation manuelle avant expédition

**Résultats** :
- ~5 alertes/mois
- 1 fraude détectée (carte volée)

---

### Alerte 3 : Export massif de données

**Déclencheur** : Export > 1000 utilisateurs par un admin

**Action** :
- Notification immédiate CISO
- Log haute sévérité pour audit

**Résultats** :
- 0 alerte (fonctionnalité peu utilisée)

---

## 5. Conformité RGPD

### Anonymisation automatique

**Processus** :
- Script quotidien (cron)
- Anonymise email, IP après 90 jours
- Conserve les métadonnées pour statistiques

**Champs anonymisés** :
- `email` → "ANONYMIZED"
- `ip` → "0.0.0.0"
- `user_id` → haché de manière irréversible

**Champs conservés** :
- Timestamp
- Type d'événement
- Résultat (succès/échec)
- Pays (geo-IP anonymisé)

---

## 6. Dashboards créés

### Dashboard Sécurité

**Visualisations** :
- Échecs de connexion par jour (trend)
- Top 10 IPs avec échecs
- Distribution géographique des connexions
- Tentatives d'accès admin non autorisées

**Usage** : Revue hebdomadaire par security champion

---

### Dashboard Transactions

**Visualisations** :
- Volume de commandes par jour
- Montant total par jour
- Taux d'échec de paiement
- Distribution par méthode de paiement

**Usage** : Monitoring quotidien par équipe business

---

### Dashboard Conformité

**Visualisations** :
- Événements de sécurité par type
- Actions admin sur données utilisateurs
- Exports de données
- Accès aux données PII

**Usage** : Rapports mensuels pour audit interne

---

## 7. Métriques de succès

### Avant implémentation

| Métrique | Valeur |
|----------|--------|
| Temps de détection d'incident | 3+ jours |
| Temps d'investigation | 4-6 heures |
| Logs structurés | 0% |
| Alertes automatiques | 0 |
| Couverture événements sécurité | 20% |

### Après implémentation

| Métrique | Valeur |
|----------|--------|
| Temps de détection d'incident | < 15 minutes ✅ |
| Temps d'investigation | 15-30 minutes ✅ |
| Logs structurés | 100% ✅ |
| Alertes automatiques | 3 actives ✅ |
| Couverture événements sécurité | 100% ✅ |
| Disponibilité ELK | 99.8% ✅ |

---

## 8. Leçons apprises

### Ce qui a bien fonctionné ✅

- **Logs structurés JSON** : Facilite parsing et recherche
- **Centralisation ELK** : Un seul endroit pour tout
- **Alertes Slack** : Réponse rapide de l'équipe
- **Anonymisation automatique** : Conformité RGPD sans effort manuel
- **Formation équipe** : 1 journée de formation Kibana = équipe autonome

### Défis rencontrés ⚠️

- **Volume de logs HTTP** : Initialement trop verbeux
  - Solution : Réduire niveau DEBUG → INFO, rétention 30j → 7j

- **Faux positifs alertes** : Brute force déclenché par utilisateurs légitimes
  - Solution : Augmenter seuil 5 → 7 tentatives, whitelist IPs bureaux

- **Performance Elasticsearch** : Lenteur sur recherches longues périodes
  - Solution : Index par jour, curator pour archivage automatique

- **Coût infrastructure** : Plus élevé que prévu
  - Estimation initiale : $200/mois
  - Réel : $350/mois (Elasticsearch 3 nœuds + S3)

---

## 9. Incidents détectés grâce aux logs

### Incident 1 : Tentative de brute force (Mois 1)

**Détection** : Alerte Slack - 15 échecs de connexion en 3 minutes
**Investigation** : 2 minutes (recherche Kibana)
**Résolution** : Blocage IP automatique
**Impact** : Aucun compte compromis

---

### Incident 2 : Fraude par carte volée (Mois 2)

**Détection** : Alerte email - Transaction 2500€ (inhabituel)
**Investigation** : 10 minutes (historique utilisateur, pattern)
**Résolution** : Annulation commande, remboursement, blocage compte
**Impact** : Évité perte de 2500€

---

### Incident 3 : Bug causant échecs de paiement (Mois 3)

**Détection** : Dashboard - pic d'échecs paiement (monitoring proactif)
**Investigation** : 20 minutes (logs applicatifs corrélés)
**Résolution** : Rollback déploiement défectueux
**Impact** : 15 minutes d'interruption vs heures sans logs

---

## 10. Prochaines étapes

- [ ] Migration vers SIEM complet (Wazuh) pour corrélation avancée
- [ ] Intégration threat intelligence (abuse.ch, Shodan)
- [ ] Alertes ML pour détection d'anomalies comportementales
- [ ] Archivage long terme optimisé (S3 Glacier pour réduire coûts)
- [ ] Dashboards temps réel pour équipe support

---

## 11. Recommandations

**Pour une équipe similaire** :

1. **Commencer simple** : Logs structurés + centralisation avant SIEM complet
2. **Définir les événements critiques** : Focus sur auth, transactions, admin
3. **Automatiser l'anonymisation** : Éviter oubli RGPD
4. **Former l'équipe tôt** : Kibana prend 1 jour à maîtriser
5. **Monitorer les coûts** : Elasticsearch peut coûter cher, optimiser rapidement
6. **Ajuster les alertes** : Itérer pour réduire faux positifs

---

**Ressources utilisées** :
- [Exigences de journalisation](logging-requirements-template-design.md)
- [Exigences centralisation](log-centralization-requirements.md)
- [Template règles d'alerte](security-alert-rules-template.md)
