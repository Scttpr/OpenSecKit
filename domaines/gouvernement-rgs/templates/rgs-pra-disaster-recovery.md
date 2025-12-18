---
title: "Plan de Reprise d'Activité (PRA) - Conforme RGS"
template_version: "1.0.0"
domain: "government-rgs"
constitutional_principle: "I - Threat Modeling, VII - Patch Management"
ssdlc_phase: "operations"
last_updated: "2025-01-15"
description: "Modèle de Plan de Reprise d'Activité technique pour systèmes gouvernementaux. Procédures détaillées de restauration après sinistre."
tags:
  - pra
  - disaster-recovery
  - rgs
  - backup
  - restore
  - runbook
difficulty: "advanced"
estimated_time: "4-8 heures"
prerequisites:
  - "PCA validé"
  - "Architecture système documentée"
  - "Infrastructure as Code disponible"
auto_fill_sources:
  - ".osk/rgs-context.yaml"
  - "docs/context/meta.md"
  - "docker-compose.yml"
  - "terraform/"
  - ".github/workflows/"
---

# Plan de Reprise d'Activité (PRA)

<!--
============================================================================
INSTRUCTIONS DE PRÉ-REMPLISSAGE AUTOMATIQUE

Ce template peut être partiellement pré-rempli par OSK depuis :
- .osk/rgs-context.yaml : RTO/RPO, organisation, fournisseurs
- docs/context/meta.md : Stack technique, services
- docker-compose.yml : Services et dépendances
- terraform/ : Infrastructure déclarative
- .github/workflows/ : Pipelines de déploiement

Les sections marquées [AUTO] peuvent être extraites automatiquement.
Les sections marquées [MANUEL] nécessitent une saisie humaine.
Les sections marquées [SCRIPT] contiennent des commandes à adapter.
============================================================================
-->

## Informations Documentaires

| Champ | Valeur |
|-------|--------|
| **Système** | [AUTO: systeme.nom depuis rgs-context.yaml] |
| **Version du document** | 1.0 |
| **Date de création** | [DATE] |
| **Document lié** | PCA - Plan de Continuité d'Activité |
| **Classification** | [AUTO: classification.classification_donnees] |

---

## 1. Objectifs de Reprise

### 1.1 Métriques Cibles

<!-- [AUTO: Extraire de besoins_securite.disponibilite dans rgs-context.yaml] -->

| Métrique | Valeur | Définition |
|----------|--------|------------|
| **RTO** | [AUTO: rto_heures] h | Délai maximum pour restaurer le service |
| **RPO** | [AUTO: rpo_heures] h | Perte de données maximum tolérée |
| **MTTR** | [MANUEL] h | Temps moyen de réparation cible |

### 1.2 Priorité de Restauration

<!-- [AUTO: Déduire de meta.md et docker-compose.yml] -->

| Priorité | Composant | RTO Spécifique | Dépendances |
|----------|-----------|----------------|-------------|
| 1 | Base de données | 1h | Stockage |
| 2 | Backend API | 2h | BDD, Cache |
| 3 | Frontend | 2h | API |
| 4 | Services annexes | 4h | Selon service |

---

## 2. Inventaire Technique

### 2.1 Architecture du Système

<!-- [AUTO: Extraire de meta.md] -->

```
┌─────────────────────────────────────────────────────────────────┐
│                    ARCHITECTURE SYSTÈME                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  [Compléter avec le schéma d'architecture]                      │
│                                                                  │
│  Exemple:                                                       │
│                                                                  │
│    Internet                                                     │
│        │                                                        │
│        ▼                                                        │
│   ┌─────────┐                                                   │
│   │   CDN   │ [AUTO: fournisseur CDN]                          │
│   └────┬────┘                                                   │
│        │                                                        │
│        ▼                                                        │
│   ┌─────────┐     ┌─────────┐                                  │
│   │ Load    │────►│ App     │ [AUTO: stack backend]            │
│   │ Balancer│     │ Server  │                                  │
│   └─────────┘     └────┬────┘                                  │
│                        │                                        │
│              ┌─────────┼─────────┐                             │
│              │         │         │                             │
│              ▼         ▼         ▼                             │
│         ┌───────┐ ┌───────┐ ┌───────┐                         │
│         │ Redis │ │ PgSQL │ │  S3   │                         │
│         │ Cache │ │  DB   │ │Storage│                         │
│         └───────┘ └───────┘ └───────┘                         │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 Composants et Versions

<!-- [AUTO: Extraire de meta.md, package.json, docker-compose.yml] -->

| Composant | Version | Image/Package | Port | Criticité |
|-----------|---------|---------------|------|-----------|
| PostgreSQL | [AUTO] | postgres:15 | 5432 | CRITIQUE |
| Redis | [AUTO] | redis:7-alpine | 6379 | IMPORTANT |
| Backend | [AUTO] | [image] | 8000 | CRITIQUE |
| Frontend | [AUTO] | [image] | 3000 | CRITIQUE |
| Nginx | [AUTO] | nginx:alpine | 80/443 | CRITIQUE |

### 2.3 Services Externes

<!-- [AUTO: Extraire de fournisseurs dans rgs-context.yaml] -->

| Service | Fournisseur | Criticité | Fallback | Contact |
|---------|-------------|-----------|----------|---------|
| Hébergement | [AUTO] | CRITIQUE | Site DR | [Contact] |
| CDN | [AUTO] | IMPORTANT | Direct access | [Contact] |
| Email | [AUTO] | STANDARD | File attente | [Contact] |
| Monitoring | [AUTO] | IMPORTANT | Logs locaux | [Contact] |
| Auth externe | [AUTO] | CRITIQUE | Auth locale | [Contact] |

---

## 3. Stratégie de Sauvegarde

### 3.1 Matrice de Sauvegarde

<!-- [AUTO: Adapter selon l'infrastructure détectée] -->

| Donnée | Méthode | Fréquence | Rétention | Localisation | Chiffrement |
|--------|---------|-----------|-----------|--------------|-------------|
| BDD PostgreSQL | pg_dump + WAL | Continue + Daily | 90 jours | [Backup provider] | AES-256 |
| Fichiers utilisateurs | Sync S3 | Temps réel | 1 an | [Backup provider] | AES-256 |
| Configuration | Git + Secrets | À chaque change | Illimité | [Forge] | - |
| Logs | Ship to SIEM | Temps réel | 3 ans | [SIEM] | TLS |
| Secrets | Vault snapshot | Daily | 30 jours | [Vault backup] | AES-256 |

### 3.2 Procédures de Backup

#### 3.2.1 PostgreSQL - Backup Continu (WAL Archiving)

<!-- [SCRIPT: Adapter selon configuration] -->

**Configuration `postgresql.conf`** :
```ini
# WAL Archiving
wal_level = replica
archive_mode = on
archive_command = 'aws s3 cp %p s3://[BUCKET]/wal/%f --sse AES256'
archive_timeout = 300

# Pour Point-in-Time Recovery
restore_command = 'aws s3 cp s3://[BUCKET]/wal/%f %p'
```

**Script de backup quotidien** :
```bash
#!/bin/bash
# backup-postgres.sh
# [SCRIPT: À adapter selon environnement]

set -euo pipefail

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_BUCKET="s3://[BUCKET]/backups/postgres"
DB_NAME="[AUTO: nom base]"

# Backup avec pg_dump
pg_dump -Fc -v -h localhost -U postgres "$DB_NAME" \
  | gzip \
  | aws s3 cp - "${BACKUP_BUCKET}/${DB_NAME}_${TIMESTAMP}.dump.gz" --sse AES256

# Vérification
aws s3 ls "${BACKUP_BUCKET}/${DB_NAME}_${TIMESTAMP}.dump.gz"

# Nettoyage vieux backups (> 90 jours)
aws s3 ls "$BACKUP_BUCKET/" \
  | awk '{print $4}' \
  | while read file; do
    file_date=$(echo "$file" | grep -oP '\d{8}')
    if [[ $(date -d "$file_date" +%s) -lt $(date -d "90 days ago" +%s) ]]; then
      aws s3 rm "${BACKUP_BUCKET}/${file}"
    fi
  done

echo "Backup completed: ${DB_NAME}_${TIMESTAMP}.dump.gz"
```

#### 3.2.2 Redis - Backup RDB

```bash
#!/bin/bash
# backup-redis.sh

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_BUCKET="s3://[BUCKET]/backups/redis"

# Trigger BGSAVE
redis-cli BGSAVE
sleep 10

# Upload RDB file
aws s3 cp /var/lib/redis/dump.rdb \
  "${BACKUP_BUCKET}/dump_${TIMESTAMP}.rdb" --sse AES256
```

#### 3.2.3 Fichiers / Object Storage

```bash
#!/bin/bash
# sync-files.sh

SOURCE_BUCKET="s3://[BUCKET-PROD]/uploads"
BACKUP_BUCKET="s3://[BUCKET-BACKUP]/uploads"

# Sync avec versioning
aws s3 sync "$SOURCE_BUCKET" "$BACKUP_BUCKET" \
  --sse AES256 \
  --delete
```

### 3.3 Vérification des Backups

**Script de vérification automatique** (à exécuter quotidiennement) :

```bash
#!/bin/bash
# verify-backups.sh

ALERT_EMAIL="[AUTO: organisation.rssi.email]"
ERRORS=()

# Vérifier backup PostgreSQL < 24h
LATEST_PG=$(aws s3 ls s3://[BUCKET]/backups/postgres/ | tail -1 | awk '{print $4}')
PG_DATE=$(echo "$LATEST_PG" | grep -oP '\d{8}')
if [[ $(date -d "$PG_DATE" +%s) -lt $(date -d "yesterday" +%s) ]]; then
  ERRORS+=("PostgreSQL backup outdated: $LATEST_PG")
fi

# Vérifier taille backup (pas vide)
PG_SIZE=$(aws s3 ls "s3://[BUCKET]/backups/postgres/$LATEST_PG" | awk '{print $3}')
if [[ $PG_SIZE -lt 1000 ]]; then
  ERRORS+=("PostgreSQL backup suspiciously small: $PG_SIZE bytes")
fi

# Vérifier WAL archiving
WAL_COUNT=$(aws s3 ls s3://[BUCKET]/wal/ | wc -l)
if [[ $WAL_COUNT -lt 10 ]]; then
  ERRORS+=("WAL archiving may be broken: only $WAL_COUNT files")
fi

# Alerter si erreurs
if [[ ${#ERRORS[@]} -gt 0 ]]; then
  echo "BACKUP VERIFICATION FAILED" | mail -s "[ALERT] Backup Issues" "$ALERT_EMAIL"
  for error in "${ERRORS[@]}"; do
    echo "- $error"
  done
  exit 1
fi

echo "All backups verified OK"
```

---

## 4. Procédures de Restauration

### 4.1 Restauration Complète (Full DR)

**Scénario** : Perte totale de l'infrastructure, reconstruction depuis zéro.

**Temps estimé** : [MANUEL] heures (selon RTO: [AUTO: rto_heures]h)

#### Étape 1 : Provisionner Infrastructure

<!-- [AUTO: Adapter selon IaC disponible (Terraform, etc.)] -->

```bash
# Option A : Terraform (si disponible)
cd infrastructure/terraform
terraform init
terraform apply -var="environment=dr" -auto-approve

# Option B : Docker Compose (environnement simple)
docker-compose -f docker-compose.prod.yml up -d

# Option C : Kubernetes (si applicable)
kubectl apply -f k8s/namespace.yaml
kubectl apply -f k8s/
```

**Vérification** :
```bash
# Vérifier que les services sont UP
docker-compose ps  # ou kubectl get pods
```

#### Étape 2 : Restaurer Base de Données

```bash
#!/bin/bash
# restore-postgres.sh
# [SCRIPT: À adapter selon environnement]

set -euo pipefail

BACKUP_BUCKET="s3://[BUCKET]/backups/postgres"
DB_NAME="[AUTO: nom base]"
DB_HOST="[HOST]"
DB_USER="postgres"

# Lister les backups disponibles
echo "Backups disponibles:"
aws s3 ls "$BACKUP_BUCKET/" | tail -10

# Sélectionner le backup (ou passer en paramètre)
read -p "Nom du fichier backup à restaurer: " BACKUP_FILE

# Télécharger et décompresser
aws s3 cp "${BACKUP_BUCKET}/${BACKUP_FILE}" /tmp/restore.dump.gz
gunzip /tmp/restore.dump.gz

# Créer base si nécessaire
createdb -h "$DB_HOST" -U "$DB_USER" "$DB_NAME" || true

# Restaurer
pg_restore -h "$DB_HOST" -U "$DB_USER" -d "$DB_NAME" -v /tmp/restore.dump

# Vérification
psql -h "$DB_HOST" -U "$DB_USER" -d "$DB_NAME" -c "SELECT COUNT(*) FROM [table_principale];"

echo "Restauration PostgreSQL terminée"
```

#### Étape 3 : Restaurer Point-in-Time (si nécessaire)

```bash
#!/bin/bash
# restore-pitr.sh
# Restauration à un point précis dans le temps

TARGET_TIME="2025-01-15 14:30:00"

# Créer recovery.conf (PostgreSQL < 12) ou postgresql.auto.conf (>= 12)
cat > /var/lib/postgresql/data/postgresql.auto.conf << EOF
restore_command = 'aws s3 cp s3://[BUCKET]/wal/%f %p'
recovery_target_time = '$TARGET_TIME'
recovery_target_action = 'promote'
EOF

# Créer signal de recovery
touch /var/lib/postgresql/data/recovery.signal

# Redémarrer PostgreSQL
pg_ctl restart -D /var/lib/postgresql/data
```

#### Étape 4 : Restaurer Fichiers

```bash
#!/bin/bash
# restore-files.sh

BACKUP_BUCKET="s3://[BUCKET-BACKUP]/uploads"
TARGET_DIR="/app/uploads"

# Sync depuis backup
aws s3 sync "$BACKUP_BUCKET" "$TARGET_DIR"

# Vérifier permissions
chown -R app:app "$TARGET_DIR"
chmod -R 755 "$TARGET_DIR"
```

#### Étape 5 : Restaurer Secrets

```bash
#!/bin/bash
# restore-secrets.sh

# Option A : HashiCorp Vault
vault operator unseal [UNSEAL_KEY]
vault login [ROOT_TOKEN]

# Option B : AWS Secrets Manager (secrets toujours disponibles)
aws secretsmanager get-secret-value --secret-id [SECRET_NAME]

# Option C : Fichiers .env chiffrés
gpg --decrypt secrets.env.gpg > .env
```

#### Étape 6 : Déployer Application

```bash
#!/bin/bash
# deploy-app.sh

# Depuis GitHub Actions (déclencher workflow manuellement)
gh workflow run deploy.yml -f environment=production

# Ou déploiement direct
docker-compose pull
docker-compose up -d

# Vérifier santé
curl -f http://localhost/health || exit 1
```

#### Étape 7 : Configurer DNS/LB

```bash
#!/bin/bash
# update-dns.sh

# Option A : Cloudflare
curl -X PUT "https://api.cloudflare.com/client/v4/zones/[ZONE]/dns_records/[RECORD]" \
  -H "Authorization: Bearer [TOKEN]" \
  -H "Content-Type: application/json" \
  --data '{"type":"A","name":"[DOMAIN]","content":"[NEW_IP]","proxied":true}'

# Option B : AWS Route53
aws route53 change-resource-record-sets \
  --hosted-zone-id [ZONE_ID] \
  --change-batch file://dns-change.json
```

#### Étape 8 : Tests de Validation

```bash
#!/bin/bash
# validate-restore.sh

ERRORS=()

# Test 1: Health check
if ! curl -sf http://[URL]/health; then
  ERRORS+=("Health check failed")
fi

# Test 2: Connexion BDD
if ! psql -h [HOST] -U [USER] -d [DB] -c "SELECT 1"; then
  ERRORS+=("Database connection failed")
fi

# Test 3: Authentification
if ! curl -sf -X POST http://[URL]/api/auth/test; then
  ERRORS+=("Auth test failed")
fi

# Test 4: Données présentes
COUNT=$(psql -h [HOST] -U [USER] -d [DB] -t -c "SELECT COUNT(*) FROM users")
if [[ $COUNT -lt 1 ]]; then
  ERRORS+=("Data appears to be missing")
fi

# Résultat
if [[ ${#ERRORS[@]} -gt 0 ]]; then
  echo "VALIDATION FAILED:"
  for e in "${ERRORS[@]}"; do echo "  - $e"; done
  exit 1
fi

echo "✅ All validation tests passed"
```

---

### 4.2 Restauration Partielle (Composant Unique)

#### 4.2.1 Restauration Base de Données Seule

```bash
# Voir Étape 2 de la section 4.1
./restore-postgres.sh
```

#### 4.2.2 Restauration Application Seule

```bash
# Redéploiement sans toucher aux données
docker-compose pull app
docker-compose up -d app
```

#### 4.2.3 Restauration Cache Redis

```bash
#!/bin/bash
# restore-redis.sh

BACKUP_FILE="dump_[TIMESTAMP].rdb"

# Arrêter Redis
redis-cli SHUTDOWN NOSAVE

# Restaurer RDB
aws s3 cp "s3://[BUCKET]/backups/redis/$BACKUP_FILE" /var/lib/redis/dump.rdb

# Redémarrer Redis
systemctl start redis
```

---

### 4.3 Basculement Site de Secours

**Prérequis** : Site DR provisionné et réplication en cours

```bash
#!/bin/bash
# failover-to-dr.sh

DR_SITE="dr.example.com"
PROD_SITE="prod.example.com"

echo "=== BASCULEMENT VERS SITE DR ==="

# 1. Vérifier état site DR
echo "[1/5] Vérification site DR..."
ssh $DR_SITE "docker-compose ps"

# 2. Promouvoir replica PostgreSQL
echo "[2/5] Promotion PostgreSQL replica..."
ssh $DR_SITE "psql -c 'SELECT pg_promote()'"

# 3. Vérifier réplication OK
echo "[3/5] Vérification données..."
ssh $DR_SITE "psql -c 'SELECT COUNT(*) FROM users'"

# 4. Basculer DNS
echo "[4/5] Basculement DNS..."
./update-dns.sh $DR_SITE

# 5. Vérifier
echo "[5/5] Validation..."
sleep 60  # Attendre propagation DNS
curl -sf "https://[DOMAIN]/health"

echo "=== BASCULEMENT TERMINÉ ==="
```

---

## 5. Procédures Post-Restauration

### 5.1 Checklist de Validation

```
CHECKLIST POST-RESTAURATION
Date/Heure : _______________
Opérateur : _______________

INFRASTRUCTURE
[ ] Tous les conteneurs/pods UP
[ ] Networking fonctionnel
[ ] Certificats TLS valides
[ ] DNS résolu correctement

BASE DE DONNÉES
[ ] Connexion OK
[ ] Données présentes (COUNT tables principales)
[ ] Intégrité (pas d'erreurs dans logs)
[ ] Réplication configurée (si applicable)

APPLICATION
[ ] Health check OK
[ ] Authentification fonctionnelle
[ ] Fonctionnalités critiques testées
[ ] Intégrations externes OK

SÉCURITÉ
[ ] Secrets correctement chargés
[ ] Certificats valides
[ ] Logs de sécurité actifs
[ ] Monitoring opérationnel

PERFORMANCE
[ ] Temps de réponse acceptable
[ ] Pas d'erreurs dans logs
[ ] Métriques dans les seuils
```

### 5.2 Communication de Reprise

```bash
# Notification équipe interne
./notify-team.sh "Service [NOM] restauré à [HEURE]. Validation en cours."

# Mise à jour status page
./update-status.sh "operational" "Service restored"

# Si applicable : notification utilisateurs
./notify-users.sh "Le service est de nouveau disponible."
```

### 5.3 Surveillance Renforcée

```yaml
# alerting-post-restore.yaml
# Règles d'alerting renforcées pendant 72h post-restauration

groups:
  - name: post-restore-alerts
    rules:
      - alert: HighErrorRate
        expr: rate(http_requests_total{status=~"5.."}[5m]) > 0.01
        for: 2m  # Réduit de 5m à 2m
        labels:
          severity: critical
        annotations:
          summary: "Error rate elevated post-restore"

      - alert: DatabaseLatency
        expr: pg_stat_activity_max_tx_duration > 30
        for: 1m
        labels:
          severity: warning
        annotations:
          summary: "Database latency elevated"

      - alert: MemoryUsage
        expr: container_memory_usage_bytes / container_spec_memory_limit_bytes > 0.8
        for: 5m
        labels:
          severity: warning
```

---

## 6. Tests du PRA

### 6.1 Plan de Tests

| Test | Fréquence | Scope | Responsable | Durée |
|------|-----------|-------|-------------|-------|
| Restauration backup BDD | Mensuel | Env test | DBA | 2h |
| Restauration complète | Trimestriel | Env DR | DevOps | 4h |
| Basculement DR | Annuel | Production | Équipe + Direction | 8h |
| Test scripts automatisés | Hebdomadaire | CI/CD | DevOps | Auto |

### 6.2 Script de Test Automatisé

```bash
#!/bin/bash
# test-pra.sh
# À exécuter en CI/CD ou manuellement

set -euo pipefail

TEST_ENV="dr-test"
RESULTS=()

echo "=== TEST PRA - $(date) ==="

# Test 1: Restauration backup PostgreSQL
echo "[Test 1] Restauration PostgreSQL..."
if ./restore-postgres.sh --env=$TEST_ENV --latest; then
  RESULTS+=("✅ PostgreSQL restore: OK")
else
  RESULTS+=("❌ PostgreSQL restore: FAILED")
fi

# Test 2: Validation données
echo "[Test 2] Validation données..."
COUNT=$(psql -h $TEST_ENV -c "SELECT COUNT(*) FROM users" -t)
if [[ $COUNT -gt 0 ]]; then
  RESULTS+=("✅ Data validation: OK ($COUNT records)")
else
  RESULTS+=("❌ Data validation: FAILED (0 records)")
fi

# Test 3: Déploiement application
echo "[Test 3] Déploiement application..."
if docker-compose -f docker-compose.test.yml up -d; then
  RESULTS+=("✅ App deployment: OK")
else
  RESULTS+=("❌ App deployment: FAILED")
fi

# Test 4: Health check
echo "[Test 4] Health check..."
if curl -sf http://$TEST_ENV/health; then
  RESULTS+=("✅ Health check: OK")
else
  RESULTS+=("❌ Health check: FAILED")
fi

# Rapport
echo ""
echo "=== RÉSULTATS ==="
for r in "${RESULTS[@]}"; do
  echo "$r"
done

# Nettoyage
docker-compose -f docker-compose.test.yml down -v
```

### 6.3 Historique des Tests

| Date | Type | Résultat | RTO Mesuré | Observations |
|------|------|----------|------------|--------------|
| [DATE] | Backup restore | ✅/❌ | [X]h | [Notes] |
| [DATE] | Full DR | ✅/❌ | [X]h | [Notes] |

---

## 7. Maintenance du PRA

### 7.1 Déclencheurs de Mise à Jour

Le PRA doit être mis à jour lors de :

- [ ] Changement d'infrastructure (nouveau service, migration cloud)
- [ ] Changement de fournisseur (hébergeur, backup, etc.)
- [ ] Modification architecture (nouveau composant critique)
- [ ] Après chaque test PRA (intégrer les leçons)
- [ ] Changement d'équipe (mise à jour contacts)
- [ ] Revue annuelle obligatoire

### 7.2 Revue Annuelle

| Élément | Vérifié | Date | Par |
|---------|---------|------|-----|
| Scripts de backup fonctionnels | [ ] | | |
| Scripts de restore fonctionnels | [ ] | | |
| Contacts à jour | [ ] | | |
| Infrastructure DR opérationnelle | [ ] | | |
| Documentation à jour | [ ] | | |
| RTO/RPO toujours valides | [ ] | | |

---

## Annexes

### Annexe A : Variables d'Environnement

<!-- [AUTO: Extraire de .env.example] -->

```bash
# Variables critiques pour restauration
DATABASE_URL=postgresql://[USER]:[PASS]@[HOST]:5432/[DB]
REDIS_URL=redis://[HOST]:6379
S3_BACKUP_BUCKET=s3://[BUCKET]
AWS_REGION=[REGION]

# Secrets (depuis Vault/Secrets Manager)
SECRET_KEY=[FROM_VAULT]
API_KEYS=[FROM_VAULT]
```

### Annexe B : Commandes Utiles

```bash
# === PostgreSQL ===
# Vérifier état réplication
psql -c "SELECT * FROM pg_stat_replication;"

# Taille base de données
psql -c "SELECT pg_size_pretty(pg_database_size('[DB]'));"

# === Docker ===
# Logs d'un service
docker-compose logs -f [SERVICE]

# Recréer un service
docker-compose up -d --force-recreate [SERVICE]

# === Kubernetes ===
# État des pods
kubectl get pods -n [NAMESPACE]

# Logs d'un pod
kubectl logs -f [POD] -n [NAMESPACE]

# === AWS ===
# Lister backups
aws s3 ls s3://[BUCKET]/backups/ --recursive

# Vérifier taille bucket
aws s3 ls s3://[BUCKET] --summarize --recursive | tail -2
```

### Annexe C : Contacts d'Urgence

<!-- [AUTO: Depuis rgs-context.yaml et fournisseurs] -->

| Rôle | Nom | Téléphone | Email |
|------|-----|-----------|-------|
| DBA Principal | [MANUEL] | [MANUEL] | [MANUEL] |
| DevOps Principal | [MANUEL] | [MANUEL] | [MANUEL] |
| Support Hébergeur | [AUTO] | [MANUEL] | [MANUEL] |
| Support BDD Managée | [AUTO] | [MANUEL] | [MANUEL] |
| RSSI | [AUTO: rssi.nom] | [MANUEL] | [AUTO: rssi.email] |

---

**Document généré par OpenSecKit** | Version 1.0 | [DATE]
