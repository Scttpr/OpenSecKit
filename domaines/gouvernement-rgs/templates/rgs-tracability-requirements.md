---
title: "Exigences de Traçabilité RGS (Fonction 4)"
template_version: "1.0.0"
domain: "government-rgs"
constitutional_principles:
  - "VI - Audit Logging"
  - "II - Risk Analysis"
regulatory_references:
  - "RGS v2.0 - Annexe B5 (Traçabilité)"
  - "ANSSI - Recommandations de sécurité pour la journalisation (2024)"
  - "RGPD - Article 30 (Registre des activités de traitement)"
  - "Code pénal - Article 323-1 et suivants (infractions informatiques)"
ssdlc_phase: "VI - Operations"
difficulty: "advanced"
estimated_time: "40-60 heures (implémentation initiale), 8-16 heures (revue annuelle)"
---

# Exigences de Traçabilité RGS (Fonction 4)

## Objectif

Ce template définit les **exigences de traçabilité obligatoires** pour les systèmes d'information soumis au RGS, conformément à l'**Annexe B5 du RGS v2.0** et aux recommandations ANSSI.

**Contexte Réglementaire** :
- **RGS v2.0 Annexe B5** : La traçabilité garantit l'imputabilité des actions et la capacité d'investigation en cas d'incident
- **ANSSI - Journalisation** : Recommandations pour la collecte, le stockage et l'analyse des logs de sécurité
- **RGPD Article 30** : Obligation de tracer les accès aux données personnelles

**Alignement Constitutionnel** :
- **Principe VI (Audit Logging)** : Journalisation structurée des événements critiques
- **Principe II (Risk Analysis)** : La traçabilité permet l'investigation post-incident

**Applicabilité** :
- ✅ Systèmes d'information de l'administration française
- ✅ Opérateurs d'importance vitale (OIV)
- ✅ Systèmes traitant des données sensibles (santé, fiscalité, social)
- ✅ Services FranceConnect (traçabilité des authentifications)

---

## 1. Exigences de Conservation RGS

### 1.1 Durées de Rétention Obligatoires

| Type d'événement | Durée minimale | Base légale | Justification |
|------------------|----------------|-------------|---------------|
| **Authentifications** (succès/échecs) | **3 ans** | RGS Annexe B5 | Investigation fraude, usurpation identité |
| **Accès aux données personnelles** | **3 ans** | RGPD Art. 30 | Droit d'accès, réclamations CNIL |
| **Actions administratives** | **5 ans** | Code pénal | Prescription délits informatiques |
| **Modifications de configuration** | **3 ans** | RGS Annexe B5 | Audit de conformité, investigation |
| **Transactions financières** | **10 ans** | Code de commerce | Obligations comptables |
| **Accès aux données de santé** | **20 ans** | Code de la santé publique | Dossier médical |

**Recommandation** : Appliquer **3 ans minimum** pour tous les événements de sécurité (simplifie la gestion).

### 1.2 Niveaux de Traçabilité par Classification RGS

| Niveau RGS | Événements à journaliser | Granularité | Stockage |
|------------|-------------------------|-------------|----------|
| **RGS*** | Authentifications, erreurs critiques | Basique | Logs locaux + backup |
| **RGS**** | + Accès données sensibles, modifications | Détaillé | SIEM centralisé |
| **RGS***** | + Toutes actions utilisateurs, flux réseau | Exhaustif | SIEM + SOC 24/7 |

---

## 2. Événements à Journaliser (Catalogue RGS)

### 2.1 Événements d'Authentification (Obligatoires)

| ID | Événement | Données à capturer | Criticité |
|----|-----------|-------------------|-----------|
| **AUTH-001** | Authentification réussie | user_id, timestamp, IP, user_agent, méthode (FC, MFA) | Info |
| **AUTH-002** | Authentification échouée | user_id (si connu), timestamp, IP, raison échec | Warning |
| **AUTH-003** | Verrouillage de compte | user_id, timestamp, IP, nombre tentatives | Alert |
| **AUTH-004** | Déverrouillage de compte | user_id, admin_id, timestamp, méthode | Info |
| **AUTH-005** | Déconnexion | user_id, timestamp, durée session | Info |
| **AUTH-006** | Expiration de session | user_id, timestamp, raison (idle/absolute) | Info |
| **AUTH-007** | Changement de mot de passe | user_id, timestamp, IP, succès/échec | Info |
| **AUTH-008** | Réinitialisation MFA | user_id, admin_id, timestamp | Alert |

### 2.2 Événements d'Autorisation (Obligatoires)

| ID | Événement | Données à capturer | Criticité |
|----|-----------|-------------------|-----------|
| **AUTHZ-001** | Accès autorisé à ressource | user_id, resource_id, action, timestamp | Info |
| **AUTHZ-002** | Accès refusé à ressource | user_id, resource_id, action, raison, timestamp | Warning |
| **AUTHZ-003** | Élévation de privilèges | user_id, ancien_role, nouveau_role, admin_id | Alert |
| **AUTHZ-004** | Tentative d'accès non autorisé | user_id, resource_id, IP, timestamp | Alert |

### 2.3 Événements sur Données Sensibles (Obligatoires RGPD)

| ID | Événement | Données à capturer | Criticité |
|----|-----------|-------------------|-----------|
| **DATA-001** | Lecture de données personnelles | user_id, data_subject_id, champs accédés, finalité | Info |
| **DATA-002** | Modification de données personnelles | user_id, data_subject_id, champs modifiés, avant/après | Info |
| **DATA-003** | Suppression de données personnelles | user_id, data_subject_id, raison (droit oubli, etc.) | Info |
| **DATA-004** | Export de données (masse) | user_id, nombre enregistrements, format, destination | Alert |
| **DATA-005** | Accès aux données de santé | user_id, patient_id, motif accès, timestamp | Info |

### 2.4 Événements d'Administration (Obligatoires)

| ID | Événement | Données à capturer | Criticité |
|----|-----------|-------------------|-----------|
| **ADMIN-001** | Création d'utilisateur | admin_id, new_user_id, rôles attribués | Info |
| **ADMIN-002** | Suppression d'utilisateur | admin_id, deleted_user_id, raison | Alert |
| **ADMIN-003** | Modification de rôles | admin_id, target_user_id, ancien/nouveau rôle | Alert |
| **ADMIN-004** | Modification de configuration | admin_id, paramètre, ancienne/nouvelle valeur | Alert |
| **ADMIN-005** | Accès console admin | admin_id, IP, timestamp, actions effectuées | Info |
| **ADMIN-006** | Déploiement applicatif | admin_id, version, environnement, timestamp | Info |

### 2.5 Événements de Sécurité (Obligatoires)

| ID | Événement | Données à capturer | Criticité |
|----|-----------|-------------------|-----------|
| **SEC-001** | Détection d'intrusion (IDS/IPS) | source_ip, signature, action (block/alert) | Alert |
| **SEC-002** | Tentative d'injection (SQL, XSS) | user_id, IP, payload (sanitisé), endpoint | Alert |
| **SEC-003** | Scan de vulnérabilité détecté | source_ip, ports scannés, timestamp | Warning |
| **SEC-004** | Certificat expiré/invalide | service, certificat, date expiration | Alert |
| **SEC-005** | Accès aux secrets (Vault, KMS) | user_id, secret_path, action, timestamp | Info |
| **SEC-006** | Rotation de clé cryptographique | admin_id, key_id, ancien/nouveau, raison | Info |

### 2.6 Événements Système (Recommandés)

| ID | Événement | Données à capturer | Criticité |
|----|-----------|-------------------|-----------|
| **SYS-001** | Démarrage/arrêt service | service_name, timestamp, raison | Info |
| **SYS-002** | Erreur applicative critique | service, error_code, stack_trace (sanitisé) | Error |
| **SYS-003** | Saturation ressources (CPU, RAM, disque) | service, métrique, seuil dépassé | Warning |
| **SYS-004** | Backup effectué | type, taille, durée, destination | Info |
| **SYS-005** | Restauration de backup | admin_id, source, raison, timestamp | Alert |

---

## 3. Format de Log Structuré (JSON)

### 3.1 Schéma de Log RGS

Tous les logs doivent être au format **JSON structuré** pour faciliter l'ingestion SIEM.

```json
{
  "timestamp": "2025-01-15T14:32:45.123Z",
  "version": "1.0",
  "level": "INFO",
  "event": {
    "id": "AUTH-001",
    "category": "authentication",
    "action": "login_success",
    "outcome": "success"
  },
  "actor": {
    "user_id": "usr_abc123def456",
    "user_type": "citizen",
    "session_id": "sess_xyz789",
    "ip_address": "203.0.113.42",
    "user_agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64)...",
    "geo": {
      "country": "FR",
      "city": "Paris"
    }
  },
  "resource": {
    "type": "account",
    "id": "usr_abc123def456",
    "name": "User Account"
  },
  "context": {
    "application": "portail-citoyen",
    "environment": "production",
    "service": "auth-service",
    "trace_id": "trace_abc123xyz789",
    "span_id": "span_def456"
  },
  "details": {
    "auth_method": "franceconnect",
    "idp": "impots.gouv.fr",
    "eidas_level": "substantial",
    "mfa_used": true
  },
  "compliance": {
    "rgs_level": "RGS**",
    "data_classification": "DR",
    "retention_days": 1095
  }
}
```

### 3.2 Champs Obligatoires RGS

| Champ | Type | Description | Obligatoire |
|-------|------|-------------|-------------|
| `timestamp` | ISO 8601 | Horodatage UTC avec millisecondes | ✅ Oui |
| `event.id` | String | Identifiant unique du type d'événement | ✅ Oui |
| `event.category` | Enum | Catégorie (authentication, authorization, data, admin, security) | ✅ Oui |
| `event.action` | String | Action effectuée | ✅ Oui |
| `event.outcome` | Enum | Résultat (success, failure, unknown) | ✅ Oui |
| `actor.user_id` | String | Identifiant de l'utilisateur | ✅ Oui (si applicable) |
| `actor.ip_address` | IP | Adresse IP source | ✅ Oui |
| `context.trace_id` | String | Identifiant de corrélation distribuée | ✅ Oui |
| `context.application` | String | Nom de l'application | ✅ Oui |
| `context.environment` | Enum | Environnement (production, staging, dev) | ✅ Oui |

### 3.3 Données à NE PAS Journaliser

**Données sensibles interdites dans les logs** :

| Donnée | Raison | Alternative |
|--------|--------|-------------|
| **Mots de passe** (même hashés) | Risque de fuite | Journaliser uniquement "password changed" |
| **Tokens d'accès** | Utilisables pour usurpation | Journaliser uniquement les 8 derniers caractères |
| **Numéros de carte bancaire** | PCI-DSS | Masquer : `****-****-****-1234` |
| **NIR (Numéro de sécurité sociale)** | Donnée très sensible | Pseudonymiser : `hash(NIR)` |
| **Données médicales** | Secret médical | Journaliser uniquement l'accès, pas le contenu |
| **Contenus de messages** | Vie privée | Journaliser uniquement les métadonnées |

**Exemple de masquage** :
```python
def sanitize_log(data):
    """Masque les données sensibles avant journalisation"""
    sanitized = data.copy()

    # Masquer tokens
    if 'access_token' in sanitized:
        sanitized['access_token'] = f"...{sanitized['access_token'][-8:]}"

    # Masquer emails partiellement
    if 'email' in sanitized:
        email = sanitized['email']
        parts = email.split('@')
        sanitized['email'] = f"{parts[0][:2]}***@{parts[1]}"

    return sanitized
```

---

## 4. Architecture de Collecte

### 4.1 Architecture SIEM Recommandée

```
┌─────────────────────────────────────────────────────────────────┐
│                    SOURCES DE LOGS                               │
├─────────────────────────────────────────────────────────────────┤
│  Applications     Serveurs Web     BDD          Infra            │
│  (JSON stdout)    (access.log)     (audit)      (syslog)         │
└────────┬──────────────┬─────────────┬────────────┬──────────────┘
         │              │             │            │
         ▼              ▼             ▼            ▼
┌─────────────────────────────────────────────────────────────────┐
│                    COLLECTE (Agents)                             │
├─────────────────────────────────────────────────────────────────┤
│  Filebeat         Fluentd        Vector        rsyslog           │
│  (conteneurs)     (K8s)          (edge)        (legacy)          │
└────────────────────────┬────────────────────────────────────────┘
                         │ TLS 1.3 (chiffré en transit)
                         ▼
┌─────────────────────────────────────────────────────────────────┐
│                    INGESTION & PARSING                           │
├─────────────────────────────────────────────────────────────────┤
│  Logstash / Kafka                                                │
│  - Parsing JSON                                                  │
│  - Enrichissement (GeoIP, threat intel)                         │
│  - Normalisation ECS (Elastic Common Schema)                    │
└────────────────────────┬────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────────┐
│                    STOCKAGE (SIEM)                               │
├─────────────────────────────────────────────────────────────────┤
│  Elasticsearch / Splunk / Azure Sentinel                         │
│  - Index par jour (rollover automatique)                        │
│  - Rétention : 3 ans (hot: 30j, warm: 1an, cold: 2ans)         │
│  - Chiffrement at-rest (AES-256)                                │
│  - Réplication 3 nœuds minimum                                  │
└────────────────────────┬────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────────┐
│                    ANALYSE & ALERTING                            │
├─────────────────────────────────────────────────────────────────┤
│  Kibana / Splunk UI / Grafana                                    │
│  - Dashboards temps réel                                         │
│  - Règles de corrélation (SIGMA)                                │
│  - Alertes PagerDuty / OpsGenie                                 │
│  - Rapports de conformité automatisés                           │
└─────────────────────────────────────────────────────────────────┘
```

### 4.2 Politique de Rétention (Tiering)

| Tier | Durée | Stockage | Coût | Accès |
|------|-------|----------|------|-------|
| **Hot** | 0-30 jours | SSD haute performance | $$$ | Temps réel (<1s) |
| **Warm** | 30 jours - 1 an | HDD standard | $$ | Rapide (<10s) |
| **Cold** | 1-3 ans | Object storage (S3/GCS) | $ | Différé (<1min) |
| **Archive** | >3 ans (si requis) | Glacier/Archive | ¢ | Heures |

**Configuration Elasticsearch ILM (Index Lifecycle Management)** :
```json
{
  "policy": {
    "phases": {
      "hot": {
        "min_age": "0ms",
        "actions": {
          "rollover": {
            "max_size": "50gb",
            "max_age": "1d"
          }
        }
      },
      "warm": {
        "min_age": "30d",
        "actions": {
          "shrink": { "number_of_shards": 1 },
          "forcemerge": { "max_num_segments": 1 }
        }
      },
      "cold": {
        "min_age": "365d",
        "actions": {
          "searchable_snapshot": {
            "snapshot_repository": "rgs-archive"
          }
        }
      },
      "delete": {
        "min_age": "1095d",
        "actions": {
          "delete": {}
        }
      }
    }
  }
}
```

---

## 5. Horodatage Qualifié (RFC 3161)

### 5.1 Exigences RGS pour l'Horodatage

L'horodatage qualifié RGS garantit la **preuve de l'existence d'un événement** à un instant donné.

**Quand utiliser l'horodatage qualifié** :

| Cas d'usage | Horodatage qualifié | Horodatage simple |
|-------------|---------------------|-------------------|
| Logs d'authentification courants | ❌ | ✅ NTP synchronisé |
| Signature de documents officiels | ✅ Obligatoire | ❌ |
| Preuve pour contentieux juridique | ✅ Obligatoire | ❌ |
| Scellement de journaux d'audit | ✅ Recommandé | ⚠️ Acceptable |
| Horodatage de transactions financières | ✅ Obligatoire | ❌ |

### 5.2 Autorités d'Horodatage Qualifiées (TSA)

**Prestataires qualifiés RGS** (liste ANSSI) :

| Prestataire | URL TSA | Certification |
|-------------|---------|---------------|
| **Certinomis** | `https://timestamp.certinomis.com` | RGS**, eIDAS |
| **Universign** | `https://timestamp.universign.eu` | RGS**, eIDAS |
| **DocuSign** | `https://timestamp.docusign.com` | eIDAS |
| **ChamberSign** | `https://timestamp.chambersign.fr` | RGS** |

### 5.3 Implémentation de l'Horodatage

**Exemple (Python - RFC 3161)** :
```python
import hashlib
import requests
from asn1crypto import tsp, core

def timestamp_data(data: bytes, tsa_url: str) -> bytes:
    """Obtient un jeton d'horodatage qualifié RFC 3161"""

    # 1. Calculer le hash du document/log
    digest = hashlib.sha256(data).digest()

    # 2. Construire la requête TSP
    message_imprint = tsp.MessageImprint({
        'hash_algorithm': {'algorithm': 'sha256'},
        'hashed_message': digest
    })

    ts_request = tsp.TimeStampReq({
        'version': 1,
        'message_imprint': message_imprint,
        'cert_req': True,  # Inclure certificat TSA dans réponse
        'nonce': core.Integer(random.getrandbits(64))
    })

    # 3. Envoyer à l'autorité d'horodatage
    response = requests.post(
        tsa_url,
        data=ts_request.dump(),
        headers={'Content-Type': 'application/timestamp-query'}
    )

    # 4. Valider et retourner le jeton
    ts_response = tsp.TimeStampResp.load(response.content)
    if ts_response['status']['status'].native != 'granted':
        raise Exception(f"TSA error: {ts_response['status']}")

    return ts_response['time_stamp_token'].dump()

# Usage pour sceller un lot de logs
def seal_audit_logs(log_batch: list, tsa_url: str):
    """Scelle un lot de logs avec horodatage qualifié"""

    # Concaténer les logs et hasher
    batch_content = '\n'.join(json.dumps(log) for log in log_batch)
    batch_hash = hashlib.sha256(batch_content.encode()).hexdigest()

    # Obtenir jeton d'horodatage
    timestamp_token = timestamp_data(batch_content.encode(), tsa_url)

    # Stocker le scellement
    seal = {
        'batch_id': str(uuid.uuid4()),
        'timestamp': datetime.utcnow().isoformat(),
        'log_count': len(log_batch),
        'batch_hash_sha256': batch_hash,
        'timestamp_token_base64': base64.b64encode(timestamp_token).decode(),
        'tsa_url': tsa_url
    }

    return seal
```

### 5.4 Synchronisation NTP (Horodatage Simple)

Pour les logs courants, une synchronisation NTP suffit :

```bash
# Configuration NTP (chrony recommandé)
# /etc/chrony/chrony.conf

# Serveurs NTP français (RENATER)
server ntp.univ-paris1.fr iburst
server ntp.obspm.fr iburst
server ntp.unistra.fr iburst

# Serveur NTP gouvernemental
server time.anfr.fr iburst

# Précision maximale
maxupdateskew 100.0
rtcsync

# Log des écarts
driftfile /var/lib/chrony/drift
logdir /var/log/chrony
log tracking measurements statistics
```

**Vérification** :
```bash
chronyc tracking
# Reference ID    : 5.196.XX.XX (ntp.obspm.fr)
# Stratum         : 2
# System time     : 0.000000123 seconds fast of NTP time
# ✅ Écart < 100ms acceptable pour logs RGS
```

---

## 6. Protection des Logs

### 6.1 Intégrité des Logs (Anti-Tampering)

**Exigence RGS** : Les logs doivent être protégés contre toute modification ou suppression non autorisée.

**Mécanismes de protection** :

| Mécanisme | Description | Niveau de protection |
|-----------|-------------|---------------------|
| **WORM Storage** | Write-Once-Read-Many (S3 Object Lock, Vault) | ⭐⭐⭐⭐⭐ |
| **Chaînage cryptographique** | Hash chain (similar blockchain) | ⭐⭐⭐⭐ |
| **Signature des lots** | Signature RSA/ECDSA périodique | ⭐⭐⭐ |
| **Copie vers système isolé** | Log forwarding vers SIEM séparé | ⭐⭐⭐ |
| **Permissions restrictives** | Comptes de service read-only | ⭐⭐ |

**Implémentation du chaînage cryptographique** :
```python
import hashlib
import json
from datetime import datetime

class AuditLogChain:
    """Chaîne de logs avec intégrité cryptographique"""

    def __init__(self):
        self.chain = []
        self.previous_hash = "0" * 64  # Genesis block

    def add_log(self, log_entry: dict) -> dict:
        """Ajoute un log avec hash de chaînage"""

        block = {
            'index': len(self.chain),
            'timestamp': datetime.utcnow().isoformat(),
            'log': log_entry,
            'previous_hash': self.previous_hash
        }

        # Calculer le hash du bloc
        block_string = json.dumps(block, sort_keys=True)
        block['hash'] = hashlib.sha256(block_string.encode()).hexdigest()

        # Mettre à jour la chaîne
        self.chain.append(block)
        self.previous_hash = block['hash']

        return block

    def verify_chain(self) -> bool:
        """Vérifie l'intégrité de toute la chaîne"""

        for i, block in enumerate(self.chain):
            # Vérifier le hash
            block_copy = {k: v for k, v in block.items() if k != 'hash'}
            expected_hash = hashlib.sha256(
                json.dumps(block_copy, sort_keys=True).encode()
            ).hexdigest()

            if block['hash'] != expected_hash:
                return False  # Bloc altéré

            # Vérifier le chaînage
            if i > 0 and block['previous_hash'] != self.chain[i-1]['hash']:
                return False  # Chaîne rompue

        return True
```

### 6.2 Confidentialité des Logs

**Chiffrement des logs sensibles** :

```python
from cryptography.fernet import Fernet
import json
import os

class EncryptedLogger:
    """Logger avec chiffrement des données sensibles"""

    def __init__(self, encryption_key: bytes):
        self.cipher = Fernet(encryption_key)

    def log(self, event: dict, sensitive_fields: list = None):
        """Journalise avec chiffrement des champs sensibles"""

        log_entry = event.copy()

        if sensitive_fields:
            encrypted_data = {}
            for field in sensitive_fields:
                if field in log_entry:
                    # Chiffrer le champ sensible
                    value = json.dumps(log_entry.pop(field))
                    encrypted_data[field] = self.cipher.encrypt(
                        value.encode()
                    ).decode()

            if encrypted_data:
                log_entry['_encrypted'] = encrypted_data

        return log_entry

# Usage
logger = EncryptedLogger(key)
log = logger.log(
    {
        'event': 'DATA-001',
        'user_id': 'usr_123',
        'accessed_data': {'ssn': '1234567890123', 'name': 'Jean Dupont'},
        'timestamp': '2025-01-15T10:30:00Z'
    },
    sensitive_fields=['accessed_data']  # Chiffrer ce champ
)
```

### 6.3 Contrôle d'Accès aux Logs

**Matrice RBAC pour accès aux logs** :

| Rôle | Lecture logs applicatifs | Lecture logs sécurité | Lecture logs admin | Export | Suppression |
|------|-------------------------|----------------------|-------------------|--------|-------------|
| **Développeur** | ✅ (env dev/staging) | ❌ | ❌ | ❌ | ❌ |
| **Ops/SRE** | ✅ | ✅ (alertes uniquement) | ❌ | ❌ | ❌ |
| **Analyste SOC** | ✅ | ✅ | ✅ | ✅ (anonymisé) | ❌ |
| **RSSI** | ✅ | ✅ | ✅ | ✅ | ❌ |
| **Auditeur externe** | ✅ (scope défini) | ✅ (scope défini) | ✅ (scope défini) | ✅ | ❌ |
| **Système** | ❌ | ❌ | ❌ | ❌ | ❌ |

**Principe** : Aucun utilisateur ne peut supprimer des logs (WORM).

---

## 7. Alertes et Corrélation

### 7.1 Règles d'Alerte Obligatoires RGS

| ID | Règle | Condition | Sévérité | Délai alerte |
|----|-------|-----------|----------|--------------|
| **ALERT-001** | Brute force authentification | >5 échecs/5min même IP | Critical | Temps réel |
| **ALERT-002** | Connexion depuis pays inhabituel | GeoIP != historique utilisateur | High | <5 min |
| **ALERT-003** | Accès données masse | >1000 enregistrements/requête | High | <5 min |
| **ALERT-004** | Élévation privilèges non planifiée | ADMIN-003 hors change window | Critical | Temps réel |
| **ALERT-005** | Échec authentification admin | AUTH-002 sur compte admin | High | Temps réel |
| **ALERT-006** | Tentative injection SQL/XSS | SEC-002 détecté | Critical | Temps réel |
| **ALERT-007** | Accès hors heures ouvrables | Connexion 22h-6h (hors astreinte) | Medium | <15 min |
| **ALERT-008** | Volume logs anormal | Variation >200% vs moyenne | Medium | <15 min |

### 7.2 Règles SIGMA (Format Standard)

**Exemple de règle SIGMA pour brute force** :
```yaml
title: Brute Force Authentication Attempt
id: rgs-auth-bruteforce-001
status: production
level: critical
description: Détection de tentatives de brute force sur authentification
author: RSSI
date: 2025/01/15
references:
  - https://attack.mitre.org/techniques/T1110/
logsource:
  category: authentication
  product: application
detection:
  selection:
    event.id: 'AUTH-002'
    event.outcome: 'failure'
  timeframe: 5m
  condition: selection | count(actor.ip_address) > 5
falsepositives:
  - Utilisateurs ayant oublié leur mot de passe
  - Tests automatisés mal configurés
tags:
  - attack.credential_access
  - attack.t1110
  - rgs.annexe_b5
```

### 7.3 Dashboards de Monitoring

**KPIs de sécurité à afficher** :

| Métrique | Calcul | Seuil alerte | Fréquence |
|----------|--------|--------------|-----------|
| Taux d'authentification réussie | `AUTH-001 / (AUTH-001 + AUTH-002)` | <95% | Temps réel |
| Nombre d'alertes critiques | `count(ALERT-*)` severity=critical | >0 | Temps réel |
| Temps moyen de détection (MTTD) | `alert_time - event_time` | >15 min | Quotidien |
| Volume de logs/jour | `count(*)` par jour | Variation >50% | Quotidien |
| Couverture de journalisation | `services_loggés / services_total` | <100% | Hebdomadaire |

---

## 8. Conformité et Audit

### 8.1 Checklist de Conformité Traçabilité RGS

**Collecte** :
- [ ] Tous les événements obligatoires (§2) sont journalisés
- [ ] Format JSON structuré conforme au schéma (§3)
- [ ] Horodatage UTC avec synchronisation NTP (<100ms)
- [ ] Données sensibles masquées ou chiffrées
- [ ] Trace_id pour corrélation distribuée

**Stockage** :
- [ ] Rétention minimum 3 ans configurée
- [ ] Politique de tiering (hot/warm/cold) en place
- [ ] Chiffrement at-rest (AES-256)
- [ ] Réplication sur 3 nœuds minimum
- [ ] Backup quotidien vérifié

**Intégrité** :
- [ ] Protection WORM ou chaînage cryptographique
- [ ] Horodatage qualifié pour logs juridiques
- [ ] Aucune possibilité de suppression utilisateur
- [ ] Vérification d'intégrité périodique

**Accès** :
- [ ] Matrice RBAC documentée et appliquée
- [ ] Accès aux logs journalisé (meta-logging)
- [ ] Séparation des droits (qui écrit ≠ qui lit)

**Alerting** :
- [ ] Règles d'alerte obligatoires configurées
- [ ] Escalade définie (SOC → RSSI → Direction)
- [ ] Tests des alertes mensuels

**Documentation** :
- [ ] Procédure d'investigation documentée
- [ ] Contacts d'astreinte à jour
- [ ] Rapport de conformité trimestriel

### 8.2 Rapport de Conformité Traçabilité

**Structure du rapport (généré par `/osk-rgs renew`)** :

```markdown
# Rapport de Conformité Traçabilité RGS

**Date** : 2025-01-15
**Système** : [Nom du système]
**Niveau RGS** : RGS**
**Auditeur** : OpenSecKit /audit rgs

## Résumé Exécutif

| Critère | Statut | Score |
|---------|--------|-------|
| Collecte des événements | ✅ | 95% |
| Format et qualité | ✅ | 100% |
| Rétention | ✅ | 100% |
| Intégrité | ⚠️ | 80% |
| Alerting | ✅ | 90% |
| **GLOBAL** | ⚠️ | **93%** |

## Détail par Exigence

### Événements manquants
- SEC-005 (Accès aux secrets) : Non implémenté
- Recommandation : Activer audit Vault/KMS

### Points d'amélioration
1. Horodatage qualifié non configuré pour scellement
2. Règle ALERT-007 (accès hors heures) désactivée

## Actions Requises

| Priorité | Action | Échéance |
|----------|--------|----------|
| 🔴 Haute | Implémenter SEC-005 | 30 jours |
| 🟡 Moyenne | Configurer horodatage qualifié | 60 jours |
```

---

## 9. Implémentation de Référence

### 9.1 Logger Python Conforme RGS

```python
import json
import logging
import uuid
from datetime import datetime
from typing import Optional, Dict, Any

class RGSLogger:
    """Logger conforme aux exigences RGS Annexe B5"""

    def __init__(
        self,
        application: str,
        environment: str,
        rgs_level: str = "RGS**"
    ):
        self.application = application
        self.environment = environment
        self.rgs_level = rgs_level
        self.logger = logging.getLogger(application)

    def _build_log(
        self,
        event_id: str,
        category: str,
        action: str,
        outcome: str,
        actor: Optional[Dict] = None,
        resource: Optional[Dict] = None,
        details: Optional[Dict] = None,
        level: str = "INFO"
    ) -> Dict[str, Any]:
        """Construit un log conforme au schéma RGS"""

        return {
            "timestamp": datetime.utcnow().isoformat() + "Z",
            "version": "1.0",
            "level": level,
            "event": {
                "id": event_id,
                "category": category,
                "action": action,
                "outcome": outcome
            },
            "actor": actor or {},
            "resource": resource or {},
            "context": {
                "application": self.application,
                "environment": self.environment,
                "trace_id": self._get_trace_id(),
                "span_id": str(uuid.uuid4())[:8]
            },
            "details": details or {},
            "compliance": {
                "rgs_level": self.rgs_level,
                "retention_days": 1095  # 3 ans
            }
        }

    def _get_trace_id(self) -> str:
        """Récupère le trace_id du contexte (OpenTelemetry, etc.)"""
        # Intégration avec OpenTelemetry si disponible
        try:
            from opentelemetry import trace
            span = trace.get_current_span()
            if span:
                return format(span.get_span_context().trace_id, '032x')
        except ImportError:
            pass
        return str(uuid.uuid4())

    def auth_success(
        self,
        user_id: str,
        ip_address: str,
        auth_method: str,
        user_agent: str = None,
        **kwargs
    ):
        """AUTH-001: Authentification réussie"""
        log = self._build_log(
            event_id="AUTH-001",
            category="authentication",
            action="login_success",
            outcome="success",
            actor={
                "user_id": user_id,
                "ip_address": ip_address,
                "user_agent": user_agent
            },
            details={
                "auth_method": auth_method,
                **kwargs
            }
        )
        self.logger.info(json.dumps(log))
        return log

    def auth_failure(
        self,
        user_id: Optional[str],
        ip_address: str,
        reason: str,
        user_agent: str = None
    ):
        """AUTH-002: Authentification échouée"""
        log = self._build_log(
            event_id="AUTH-002",
            category="authentication",
            action="login_failure",
            outcome="failure",
            level="WARNING",
            actor={
                "user_id": user_id,
                "ip_address": ip_address,
                "user_agent": user_agent
            },
            details={
                "failure_reason": reason
            }
        )
        self.logger.warning(json.dumps(log))
        return log

    def data_access(
        self,
        user_id: str,
        resource_type: str,
        resource_id: str,
        action: str,
        fields_accessed: list = None
    ):
        """DATA-001: Accès aux données personnelles"""
        log = self._build_log(
            event_id="DATA-001",
            category="data",
            action=f"data_{action}",
            outcome="success",
            actor={"user_id": user_id},
            resource={
                "type": resource_type,
                "id": resource_id
            },
            details={
                "fields_accessed": fields_accessed or [],
                "action": action
            }
        )
        self.logger.info(json.dumps(log))
        return log

    def admin_action(
        self,
        admin_id: str,
        action: str,
        target: Dict,
        details: Dict = None
    ):
        """ADMIN-*: Actions administratives"""
        log = self._build_log(
            event_id=f"ADMIN-{action.upper()[:3]}",
            category="admin",
            action=action,
            outcome="success",
            level="WARNING",
            actor={"user_id": admin_id, "user_type": "admin"},
            resource=target,
            details=details
        )
        self.logger.warning(json.dumps(log))
        return log

    def security_alert(
        self,
        alert_type: str,
        source_ip: str,
        details: Dict,
        severity: str = "high"
    ):
        """SEC-*: Événements de sécurité"""
        log = self._build_log(
            event_id=f"SEC-{alert_type.upper()[:3]}",
            category="security",
            action=alert_type,
            outcome="detected",
            level="ERROR" if severity == "critical" else "WARNING",
            actor={"ip_address": source_ip},
            details={
                "severity": severity,
                **details
            }
        )
        self.logger.error(json.dumps(log))
        return log

# Usage
rgs_logger = RGSLogger(
    application="portail-citoyen",
    environment="production",
    rgs_level="RGS**"
)

# Authentification réussie
rgs_logger.auth_success(
    user_id="usr_abc123",
    ip_address="203.0.113.42",
    auth_method="franceconnect",
    idp="impots.gouv.fr",
    eidas_level="substantial"
)

# Accès données
rgs_logger.data_access(
    user_id="usr_abc123",
    resource_type="citizen_record",
    resource_id="rec_xyz789",
    action="read",
    fields_accessed=["name", "address", "birthdate"]
)
```

---

## 10. Références

### Documentation ANSSI

- **RGS v2.0 Annexe B5** : https://cyber.gouv.fr/le-referentiel-general-de-securite-rgs
- **Guides et publications** : https://cyber.gouv.fr/publications
- **Prestataires d'horodatage qualifiés** : https://www.lsti-certification.fr/

### Standards

- **RFC 3161** : Time-Stamp Protocol (TSP)
- **RFC 5424** : Syslog Protocol
- **ECS (Elastic Common Schema)** : https://www.elastic.co/guide/en/ecs/current/index.html
- **SIGMA Rules** : https://github.com/SigmaHQ/sigma

### Outils

- **Filebeat** : https://www.elastic.co/beats/filebeat
- **Logstash** : https://www.elastic.co/logstash
- **Vector** : https://vector.dev/

---

**Template Version** : 1.0.0
**Last Updated** : 2025-01-15
**Next Review** : 2026-01-15
**Owner** : RSSI
