---
title: "Guide d'intégration d'un gestionnaire de secrets"
template_version: "1.0.0"
constitutional_principle: "V - Gestion des secrets"
ssdlc_phase: "implementation"
difficulty: "intermediate"
estimated_time: "2-3 hours"
tags: ["secrets", "vault", "key-vault", "secrets-manager", "security"]
---

# Guide d'intégration d'un gestionnaire de secrets

## Objectif

Éliminer tous les secrets en clair du code source et centraliser leur gestion dans un gestionnaire de secrets dédié.

## Contexte

**Problème** : Les secrets (clés API, mots de passe, certificats, tokens) commitées dans le code sont la cause de 73% des fuites de données.

**Solution** : Utiliser un gestionnaire de secrets qui :
- Stocke les secrets chiffrés
- Contrôle les accès
- Logue tous les accès
- Permet la rotation automatique

## Prérequis

- [ ] Accès administrateur au projet
- [ ] Choix du gestionnaire de secrets effectué
- [ ] Inventaire des secrets existants réalisé
- [ ] Scan du code existant pour détecter les secrets en clair

## Étape 1 : Choisir le gestionnaire de secrets

### Option A : HashiCorp Vault (self-hosted)

**Avantages** :
- Open source, contrôle total
- Multi-cloud, multi-provider
- Rotation automatique avancée
- Audit logging complet

**Inconvénients** :
- Complexité opérationnelle (haute disponibilité, backup)
- Infrastructure à maintenir

**Quand choisir** : Multi-cloud, exigences de souveraineté, grosse équipe

### Option B : Azure Key Vault (cloud Azure)

**Avantages** :
- Fully managed, pas d'ops
- Intégration native Azure
- HSM hardware disponible
- Tarification au secret

**Inconvénients** :
- Vendor lock-in Microsoft
- Moins flexible que Vault

**Quand choisir** : Infrastructure Azure, équipe .NET/Azure

### Option C : AWS Secrets Manager (cloud AWS)

**Avantages** :
- Fully managed
- Intégration native AWS (RDS, Lambda)
- Rotation automatique pour RDS/Redshift
- Tarification usage-based

**Inconvénients** :
- Vendor lock-in AWS
- Coût élevé à grande échelle

**Quand choisir** : Infrastructure AWS, rotation RDS importante

### Option D : GCP Secret Manager (cloud GCP)

**Avantages** :
- Fully managed
- Intégration native GCP
- Versioning automatique
- Tarification compétitive

**Inconvénients** :
- Vendor lock-in Google
- Moins mature que concurrents

**Quand choisir** : Infrastructure GCP, petite/moyenne échelle

### Option E : Doppler (SaaS multi-cloud)

**Avantages** :
- Très facile à démarrer
- Multi-environnement (dev/staging/prod)
- Synchronisation automatique
- Freemium (gratuit jusqu'à 5 utilisateurs)

**Inconvénients** :
- SaaS tiers (confiance requise)
- Moins de contrôle

**Quand choisir** : Startup, MVP, multi-cloud, équipe < 10

---

## Étape 2 : Déployer le gestionnaire

### HashiCorp Vault (Docker - dev/test uniquement)

```bash
# Lancer Vault en mode dev (NON PRODUCTION)
docker run --cap-add=IPC_LOCK \
  -e 'VAULT_DEV_ROOT_TOKEN_ID=myroot' \
  -p 8200:8200 \
  vault:latest

# Exporter les variables d'environnement
export VAULT_ADDR='http://127.0.0.1:8200'
export VAULT_TOKEN='myroot'

# Activer le moteur KV v2
vault secrets enable -path=secret kv-v2
```

**⚠️ Production** : Suivre le [guide officiel](https://learn.hashicorp.com/tutorials/vault/getting-started-deploy) avec Raft storage + TLS

### Azure Key Vault (Azure CLI)

```bash
# Créer un resource group
az group create --name rg-secrets --location westeurope

# Créer un Key Vault
az keyvault create \
  --name kv-mon-projet \
  --resource-group rg-secrets \
  --location westeurope

# Donner les permissions à votre identité
az keyvault set-policy \
  --name kv-mon-projet \
  --upn user@example.com \
  --secret-permissions get list set delete
```

### AWS Secrets Manager (AWS CLI)

```bash
# Créer un secret
aws secretsmanager create-secret \
  --name mon-projet/api-key \
  --secret-string '{"api_key":"valeur-secrete"}' \
  --region eu-west-1
```

---

## Étape 3 : Stocker les secrets

### HashiCorp Vault

```bash
# Stocker un secret
vault kv put secret/mon-projet/database \
  username="admin" \
  password="SuperSecret123!"

# Lire un secret
vault kv get secret/mon-projet/database

# Lire en JSON (pour parsing)
vault kv get -format=json secret/mon-projet/database
```

### Azure Key Vault

```bash
# Stocker un secret
az keyvault secret set \
  --vault-name kv-mon-projet \
  --name database-password \
  --value "SuperSecret123!"

# Lire un secret
az keyvault secret show \
  --vault-name kv-mon-projet \
  --name database-password \
  --query value -o tsv
```

### AWS Secrets Manager

```bash
# Stocker un secret
aws secretsmanager put-secret-value \
  --secret-id mon-projet/database \
  --secret-string '{"username":"admin","password":"SuperSecret123!"}'

# Lire un secret
aws secretsmanager get-secret-value \
  --secret-id mon-projet/database \
  --query SecretString --output text
```

---

## Étape 4 : Intégrer dans le code

### Librairies client recommandées

**Node.js** :
- HashiCorp Vault : `node-vault`
- Azure Key Vault : `@azure/keyvault-secrets` + `@azure/identity`
- AWS Secrets Manager : `@aws-sdk/client-secrets-manager`

**Python** :
- HashiCorp Vault : `hvac`
- Azure Key Vault : `azure-keyvault-secrets` + `azure-identity`
- AWS Secrets Manager : `boto3`

**Go** :
- HashiCorp Vault : `github.com/hashicorp/vault/api`
- Azure Key Vault : `github.com/Azure/azure-sdk-for-go/sdk/keyvault/azsecrets`
- AWS Secrets Manager : `github.com/aws/aws-sdk-go-v2/service/secretsmanager`

### Bonnes pratiques d'intégration

1. **Charger les secrets au démarrage** de l'application, pas à chaque requête
2. **Mettre en cache** les secrets avec un TTL approprié
3. **Implémenter un mécanisme de rafraîchissement** automatique
4. **Ne jamais logger** les valeurs des secrets
5. **Gérer les erreurs** de connexion au gestionnaire de secrets

---

## Étape 5 : Configurer l'authentification

### Vault : AppRole (recommandé pour applications)

```bash
# Activer AppRole
vault auth enable approle

# Créer un rôle
vault write auth/approle/role/mon-app \
  token_ttl=1h \
  token_max_ttl=4h \
  secret_id_ttl=24h \
  policies="mon-app-policy"

# Récupérer role_id (à stocker en config)
vault read auth/approle/role/mon-app/role-id

# Récupérer secret_id (à injecter au déploiement)
vault write -f auth/approle/role/mon-app/secret-id
```

**Intégration** : Utiliser `role_id` et `secret_id` pour s'authentifier via l'API Vault.

### Azure : Managed Identity (recommandé)

```bash
# Activer Managed Identity sur l'App Service
az webapp identity assign \
  --name mon-app \
  --resource-group rg-secrets

# Donner les permissions au Key Vault
az keyvault set-policy \
  --name kv-mon-projet \
  --object-id <managed-identity-object-id> \
  --secret-permissions get list
```

**Intégration** : Les SDKs Azure utilisent automatiquement la Managed Identity (pas besoin de credentials).

### AWS : IAM Role (recommandé)

```bash
# Créer une policy IAM
aws iam create-policy \
  --policy-name SecretsReadPolicy \
  --policy-document file://policy.json

# Attacher à un rôle EC2/Lambda
aws iam attach-role-policy \
  --role-name mon-app-role \
  --policy-arn arn:aws:iam::xxx:policy/SecretsReadPolicy
```

**Intégration** : Les SDKs AWS utilisent automatiquement le rôle IAM de l'instance.

---

## Étape 6 : Configurer la rotation automatique

### Vault : Database credentials rotation

```bash
# Configurer la connexion DB
vault write database/config/postgresql \
  plugin_name=postgresql-database-plugin \
  connection_url="postgresql://{{username}}:{{password}}@localhost:5432/mydb" \
  allowed_roles="readonly" \
  username="vault" \
  password="vault-password"

# Créer un rôle avec rotation
vault write database/roles/readonly \
  db_name=postgresql \
  creation_statements="CREATE ROLE \"{{name}}\" WITH LOGIN PASSWORD '{{password}}' VALID UNTIL '{{expiration}}';" \
  default_ttl="1h" \
  max_ttl="24h"
```

### AWS Secrets Manager : Rotation RDS

```bash
# Activer la rotation automatique (tous les 30 jours)
aws secretsmanager rotate-secret \
  --secret-id mon-projet/rds-password \
  --rotation-lambda-arn arn:aws:lambda:... \
  --rotation-rules AutomaticallyAfterDays=30
```

### Azure Key Vault : Rotation avec Azure Functions

Créer une Azure Function déclenchée tous les 90 jours qui :
1. Génère un nouveau mot de passe
2. Met à jour la base de données
3. Met à jour le secret dans Key Vault

---

## Étape 7 : Migrer les secrets existants

### Inventaire

```bash
# Scanner le code pour détecter les secrets
gitleaks detect --source . --verbose

# Lister tous les fichiers suspects
find . -name ".env*" -o -name "config*.yml" -o -name "credentials*"
```

### Processus de migration

Pour chaque secret trouvé :

1. **Stocker dans le gestionnaire** :
   ```bash
   vault kv put secret/mon-projet/api-github token="ghp_xxx"
   ```

2. **Remplacer dans le code** :
   - Remplacer les constantes codées en dur par des appels au gestionnaire de secrets
   - Utiliser les librairies client appropriées

3. **Supprimer de l'historique git** (si déjà committé) :
   ```bash
   # Utiliser BFG Repo-Cleaner
   java -jar bfg.jar --delete-files .env
   git reflog expire --expire=now --all
   git gc --prune=now --aggressive
   ```

4. **Révoquer l'ancien secret** : régénérer la clé API côté provider

---

## Étape 8 : Configurer les audits

### Vault : Activer audit logging

```bash
# Logger tous les accès
vault audit enable file file_path=/var/log/vault/audit.log

# Vérifier les logs
tail -f /var/log/vault/audit.log | jq
```

### Azure : Diagnostic logs

```bash
# Activer les logs
az monitor diagnostic-settings create \
  --resource /subscriptions/.../providers/Microsoft.KeyVault/vaults/kv-mon-projet \
  --name audit-logs \
  --logs '[{"category": "AuditEvent", "enabled": true}]' \
  --workspace /subscriptions/.../resourceGroups/.../providers/Microsoft.OperationalInsights/workspaces/...
```

### AWS : CloudTrail

Tous les appels à Secrets Manager sont automatiquement loggés dans CloudTrail.

---

## Checklist de validation

- [ ] Aucun secret en clair dans le code source
- [ ] Tous les secrets dans le gestionnaire centralisé
- [ ] Authentification configurée (AppRole, Managed Identity, IAM Role)
- [ ] Rotation automatique activée pour les secrets critiques
- [ ] Audit logging activé
- [ ] Alertes configurées sur accès suspects
- [ ] Documentation à jour (comment accéder aux secrets)
- [ ] Équipe formée aux bonnes pratiques

---

## Métriques de succès

- **Taux de migration** : 100% des secrets en clair migrés
- **Temps de révocation** : < 1h en cas de compromission
- **Accès tracés** : 100% des accès loggés
- **Rotation** : Secrets critiques < 90 jours d'âge

---

## Dépannage

### Problème : "Permission denied" lors de l'accès à un secret

**Solution** :
- Vérifier que l'application a les bonnes permissions (policy Vault, IAM role AWS, Key Vault policy Azure)
- Vérifier que l'authentification fonctionne (token valide, managed identity activée)

### Problème : Rotation automatique échoue

**Solution** :
- Vérifier les logs de rotation
- Tester manuellement la connexion du service de rotation à la DB
- Vérifier que les credentials Vault/rotation ont les permissions nécessaires

### Problème : Performance dégradée

**Solution** :
- Mettre en cache les secrets en mémoire (avec TTL)
- Ne pas faire un appel au gestionnaire à chaque requête HTTP
- Utiliser le renouvellement de lease Vault

---

## Ressources

- [HashiCorp Vault Documentation](https://www.vaultproject.io/docs)
- [Azure Key Vault Best Practices](https://docs.microsoft.com/azure/key-vault/general/best-practices)
- [AWS Secrets Manager User Guide](https://docs.aws.amazon.com/secretsmanager/)
- [OWASP Secrets Management Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Secrets_Management_Cheat_Sheet.html)

---

**Prochaine étape** : Configurer la détection pré-commit avec [secrets-detection-setup.md](secrets-detection-setup.md)
