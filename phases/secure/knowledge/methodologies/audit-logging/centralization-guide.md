---
title: "Exigences de centralisation des logs"
template_version: "1.0.0"
constitutional_principle: "VI - Journalisation et audit"
ssdlc_phase: "design"
difficulty: "beginner"
estimated_time: "1-2 hours"
tags: ["logging", "centralization", "requirements", "architecture"]
---

# Exigences de centralisation des logs

## Objectif

Définir les exigences pour centraliser tous les logs (application, infrastructure, sécurité) dans un système unique permettant la recherche, l'analyse et l'archivage.

**Principe** : Des logs éparpillés sont des logs inutiles. La centralisation est le prérequis à toute analyse de sécurité efficace.

---

## Pourquoi centraliser les logs

### Problème des logs distribués

Dans une architecture moderne, les logs sont répartis sur :
- Multiples serveurs d'application
- Bases de données
- Load balancers
- Firewalls et équipements réseau
- Services cloud (Lambda, containers)
- CDN et proxies

**Impact** :
- ❌ Investigation d'incident = connexion manuelle à 10+ systèmes
- ❌ Corrélation impossible entre événements
- ❌ Recherche lente et fastidieuse
- ❌ Logs perdus lors de redémarrage/crash
- ❌ Pas de vue d'ensemble

### Bénéfices de la centralisation

- ✅ **Recherche unifiée** : Un seul point d'entrée pour toutes les requêtes
- ✅ **Corrélation** : Lier des événements de sources différentes
- ✅ **Rétention garantie** : Logs persistés même si la source disparaît
- ✅ **Analyse à grande échelle** : Requêtes sur millions d'événements
- ✅ **Alerting temps réel** : Détection rapide d'anomalies
- ✅ **Conformité** : Archivage centralisé pour audits

---

## Architecture de référence

```
┌─────────────────┐
│   Sources de    │
│      logs       │
└────────┬────────┘
         │
         │ (1) Collection
         ▼
┌─────────────────┐
│   Collecteur    │  ← Agent léger sur chaque source
│   (Agent)       │     (Filebeat, Fluentd, Vector, etc.)
└────────┬────────┘
         │
         │ (2) Transport
         ▼
┌─────────────────┐
│   Agrégateur    │  ← Parsing, enrichissement, routage
│  (optionnel)    │     (Logstash, Fluentd, etc.)
└────────┬────────┘
         │
         │ (3) Stockage
         ▼
┌─────────────────┐
│  Destination    │  ← Stockage centralisé
│   Centralisée   │     (Elasticsearch, S3, Splunk, etc.)
└────────┬────────┘
         │
         │ (4) Visualisation
         ▼
┌─────────────────┐
│   Interface     │  ← Recherche et dashboards
│   Utilisateur   │     (Kibana, Grafana, Splunk UI, etc.)
└─────────────────┘
```

---

## Exigences fonctionnelles

### 1. Collection des logs

**Exigences** :
- [ ] Collecte depuis toutes les sources (app, infra, sécurité)
- [ ] Support de multiples formats (JSON, syslog, plain text)
- [ ] Collecte temps réel (latence < 5 secondes)
- [ ] Résilience : buffer local en cas de coupure réseau
- [ ] Faible empreinte ressources (< 100MB RAM par source)

**Sources typiques** :
- Logs applicatifs (fichiers, stdout)
- Logs système (syslog, journald)
- Logs web servers (nginx, Apache)
- Logs bases de données (PostgreSQL, MySQL)
- Logs cloud (CloudWatch, Stackdriver)
- Logs réseau (firewalls, load balancers)

---

### 2. Transport des logs

**Exigences** :
- [ ] Protocole sécurisé (TLS/SSL)
- [ ] Compression pour réduire la bande passante
- [ ] Buffering et retry automatique
- [ ] Back-pressure pour éviter la perte de logs
- [ ] Support multi-destination (routage conditionnel)

**Considérations** :
- Bande passante disponible (estimation : 1-10 GB/jour par serveur)
- Latence acceptable (temps réel vs batch)
- Fiabilité requise (at-least-once vs exactly-once)

---

### 3. Parsing et enrichissement

**Exigences** :
- [ ] Parsing de formats courants (JSON, syslog, Apache logs)
- [ ] Extraction de champs structurés
- [ ] Enrichissement contextuel (geo-IP, user lookup)
- [ ] Normalisation des timestamps
- [ ] Tagging et classification automatique

**Exemple de transformation** :
```
Input (raw) :
192.168.1.10 - - [15/Jan/2025:14:32:10 +0000] "GET /api/users HTTP/1.1" 200 1234

Output (structuré) :
{
  "timestamp": "2025-01-15T14:32:10Z",
  "source_ip": "192.168.1.10",
  "geoip": { "country": "FR", "city": "Paris" },
  "http_method": "GET",
  "http_path": "/api/users",
  "http_status": 200,
  "bytes_sent": 1234,
  "event_type": "http_access"
}
```

---

### 4. Stockage centralisé

**Exigences** :
- [ ] Capacité suffisante (estimer volume × rétention)
- [ ] Performance de recherche (< 1s pour requêtes courantes)
- [ ] Indexation automatique des champs
- [ ] Compression des données anciennes
- [ ] Rétention automatique (rotation, archivage)

**Politiques de rétention recommandées** :
| Type de log | Rétention chaude | Rétention froide | Total |
|-------------|------------------|------------------|-------|
| Sécurité | 30 jours | 335 jours | 1 an |
| Application | 7 jours | 23 jours | 30 jours |
| Audit/Conformité | 90 jours | 6.75 ans | 7 ans |

---

### 5. Recherche et visualisation

**Exigences** :
- [ ] Interface de recherche intuitive
- [ ] Langage de requête puissant
- [ ] Filtres facettés (par source, niveau, période)
- [ ] Dashboards personnalisables
- [ ] Export de résultats (CSV, JSON)
- [ ] Partage de requêtes et dashboards

---

## Critères de choix d'une solution

### Par taille d'organisation

**Petite équipe (< 10 personnes)** :
- Solution SaaS (Datadog, Logtail, Papertrail)
- Peu de maintenance
- Coût acceptable (< $100/mois)

**Équipe moyenne (10-50 personnes)** :
- ELK Stack managé (Elastic Cloud, AWS OpenSearch)
- Ou solution SaaS premium
- Budget : $500-2000/mois

**Grande organisation (> 50 personnes)** :
- ELK Stack self-hosted
- Ou Splunk Enterprise
- Équipe dédiée pour opérations
- Budget : $5000+/mois

---

### Par volume de logs

| Volume quotidien | Solution recommandée | Coût estimé |
|------------------|---------------------|-------------|
| < 1 GB | SaaS (Datadog, Logtail) | $50-200/mois |
| 1-10 GB | ELK Cloud ou Splunk Cloud | $500-2000/mois |
| 10-100 GB | ELK self-hosted | $1000-5000/mois (infra) |
| > 100 GB | ELK ou Splunk Enterprise | $10000+/mois |

---

### Par expertise technique

**Faible expertise** :
- ✅ Solution SaaS tout-en-un
- ✅ Configuration minimale
- ❌ Éviter self-hosted complexe

**Expertise moyenne** :
- ✅ ELK managé (Elastic Cloud)
- ✅ Ou solution open source simple (Vector + Loki)
- ⚠️ ELK self-hosted possible avec support

**Expertise élevée** :
- ✅ N'importe quelle solution
- ✅ Optimisation fine possible
- ✅ Multi-cloud, hybrid setup

---

## Checklist d'implémentation

### Phase de conception
- [ ] Inventaire des sources de logs
- [ ] Estimation du volume quotidien
- [ ] Définition des exigences de rétention
- [ ] Choix de l'architecture (agents, agrégateurs, destination)
- [ ] Sélection des outils
- [ ] Dimensionnement infrastructure
- [ ] Budget validé

### Phase de déploiement
- [ ] Installation des agents sur toutes les sources
- [ ] Configuration du transport sécurisé (TLS)
- [ ] Mise en place de l'agrégateur (si nécessaire)
- [ ] Configuration du stockage centralisé
- [ ] Parsing et structuration des logs
- [ ] Tests de bout en bout
- [ ] Configuration de la rétention

### Phase de validation
- [ ] Vérifier que tous les logs arrivent (aucune perte)
- [ ] Latence acceptable (< 30 secondes)
- [ ] Performance de recherche acceptable
- [ ] Dashboards opérationnels
- [ ] Alertes configurées
- [ ] Documentation à jour
- [ ] Formation de l'équipe

---

## Métriques de succès

| Métrique | Cible | Mesure |
|----------|-------|--------|
| **Couverture** | 100% des sources | % sources loggant dans le système central |
| **Latence** | < 30s | Temps entre génération et disponibilité |
| **Perte de logs** | 0% | Logs perdus / Logs générés |
| **Performance recherche** | < 2s | Temps de réponse requêtes courantes |
| **Disponibilité** | > 99.5% | Uptime du système de logs |
| **Coût** | < budget | Coût mensuel vs budget alloué |

---

## Anti-patterns à éviter

❌ **Logs non structurés** : Impossible à parser automatiquement
❌ **Pas de buffer local** : Perte de logs si réseau coupé
❌ **Logs en clair** : Risque de fuite de données sensibles
❌ **Pas de rotation** : Saturation du stockage
❌ **Configuration manuelle partout** : Non reproductible, erreurs
❌ **Logs sensibles non masqués** : Violation RGPD/conformité

---

## Ressources associées

- [Exigences de journalisation](logging-requirements-template-design.md)
- [Guide de sélection SIEM](siem-selection-guide.md)
- [Template règles d'alerte](security-alert-rules-template.md)
