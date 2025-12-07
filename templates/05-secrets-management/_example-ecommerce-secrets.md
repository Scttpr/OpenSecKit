---
title: "Exemple : Gestion des secrets pour une plateforme e-commerce"
template_version: "1.0.0"
constitutional_principle: "V - Gestion des secrets"
project: "ShopSecure - Plateforme e-commerce"
ssdlc_phase: "implementation"
tags: ["example", "ecommerce", "secrets", "vault", "environment-variables"]
---

# Exemple : Gestion des secrets pour ShopSecure

**Projet** : ShopSecure - Plateforme e-commerce B2C
**Équipe** : 5 développeurs
**Stack** : Node.js, PostgreSQL, Redis
**Infrastructure** : Kubernetes (AWS)

---

## Contexte

ShopSecure avait initialement tous ses secrets stockés dans des fichiers `.env` commitées dans git, avec des clés API et mots de passe en clair. Après un scan de sécurité, l'équipe a décidé de migrer vers une solution centralisée.

---

## 1. Inventaire des secrets

### Secrets identifiés

| Secret | Type | Criticité | Usage | Fréquence d'accès |
|--------|------|-----------|-------|-------------------|
| `STRIPE_SECRET_KEY` | API Key | Critique | Paiements | Haute |
| `DATABASE_URL` | Credentials | Critique | Connexion DB | Haute |
| `JWT_SECRET` | Signing Key | Critique | Auth utilisateur | Très haute |
| `SENDGRID_API_KEY` | API Key | Élevée | Emails transactionnels | Moyenne |
| `REDIS_PASSWORD` | Password | Élevée | Cache/sessions | Haute |
| `ENCRYPTION_KEY` | Crypto Key | Critique | Chiffrement PII | Moyenne |

**Total** : 6 secrets critiques, 0 secrets en clair après migration

---

## 2. Décisions d'architecture

### Choix du gestionnaire

**Option retenue** : HashiCorp Vault

**Raisons** :
- Infrastructure multi-cloud (potentielle migration AWS → GCP)
- Rotation automatique des credentials DB
- Audit logging natif
- Pas de vendor lock-in
- Open source (budget limité)

**Alternatives considérées** :
- ❌ AWS Secrets Manager : vendor lock-in, coût élevé
- ❌ Azure Key Vault : pas sur Azure
- ❌ Doppler : équipe veut du self-hosted

---

### Organisation des secrets dans Vault

**Structure** :
```
secret/
  └── shopsecure/
      ├── stripe          (API keys)
      ├── database        (credentials)
      ├── jwt             (signing key)
      ├── sendgrid        (API key)
      ├── redis           (password)
      └── encryption      (crypto key)
```

**Politique d'accès** :
- Application backend : lecture seule sur `secret/shopsecure/*`
- Ops team : lecture/écriture complète
- Rotation automatique : DB uniquement (pour commencer)

---

## 3. Stratégie de rotation

| Secret | Méthode | Fréquence | Automatisé |
|--------|---------|-----------|------------|
| Stripe keys | Manuel | 90 jours | ❌ |
| Database credentials | Vault dynamic secrets | 24h (auto-renew) | ✅ |
| JWT secret | Manuel | 180 jours | ❌ |
| SendGrid API | Manuel | 90 jours | ❌ |
| Redis password | Manuel | 90 jours | ❌ |
| Encryption key | Manuel | 1 an | ❌ |

**Note** : Rotation manuelle = procédure documentée, alertes automatiques avant expiration

---

## 4. Intégration développement

### Environnement local

**Approche** : `.env` local (non committé) avec secrets de développement

**Exemple `.env.development`** :
```bash
STRIPE_SECRET_KEY=sk_test_xxxxxxxx  # Clé de test Stripe
DATABASE_URL=postgres://localhost:5432/shopsecure_dev
JWT_SECRET=dev_not_secure_secret
```

**Avantages** :
- Développeurs autonomes
- Pas besoin d'accès Vault en local
- Secrets de test, pas de risque

---

### Environnement staging/production

**Approche** : Récupération depuis Vault au démarrage de l'application

**Flux** :
1. Pod Kubernetes démarre
2. Application s'authentifie avec Vault (AppRole)
3. Récupère tous les secrets nécessaires
4. Stocke en mémoire (jamais sur disque)
5. Token renouvelé toutes les 50 minutes

---

## 5. Sécurisation du code

### Détection pré-commit

**Outil** : gitleaks

**Configuration** :
- Bloque tout commit contenant `sk_live_` (Stripe production)
- Détecte les mots de passe codés en dur
- Vérifie les fichiers `.env*` ne sont pas commitées
- Allowlist pour secrets de test (`sk_test_`)

**Résultat** :
- 3 secrets détectés et bloqués lors des 2 premières semaines
- 0 détection depuis (équipe habituée)

---

## 6. Migration réalisée

### Processus suivi

1. **Audit initial** : scan gitleaks sur tout l'historique git
   - 12 secrets trouvés dans l'historique
   - 6 secrets encore actifs dans le code

2. **Stockage dans Vault** : migration de tous les secrets

3. **Mise à jour du code** : remplacement des constantes par appels Vault

4. **Nettoyage git** : suppression des secrets de l'historique avec BFG

5. **Révocation** : régénération de toutes les clés API compromises

**Durée totale** : 2 jours

---

## 7. Métriques de succès

### Avant migration

| Métrique | Valeur |
|----------|--------|
| Secrets en clair dans le code | 6 |
| Secrets dans l'historique git | 12 |
| Rotation des secrets | Jamais |
| Audit des accès | Impossible |
| Temps de révocation | Indéterminé |

### Après migration

| Métrique | Valeur |
|----------|--------|
| Secrets en clair dans le code | 0 ✅ |
| Secrets dans l'historique git | 0 ✅ (nettoyé) |
| Rotation des secrets DB | Automatique (24h) ✅ |
| Audit des accès | 100% tracé ✅ |
| Temps de révocation | < 1h ✅ |
| Détection pré-commit | Active ✅ |

---

## 8. Leçons apprises

### Ce qui a bien fonctionné ✅

- **AppRole Vault** : authentification simple et sécurisée
- **Rotation automatique DB** : plus de crainte d'oublier
- **Pre-commit hook** : évite les erreurs humaines
- **Documentation claire** : onboarding rapide des nouveaux devs

### Défis rencontrés ⚠️

- **Courbe d'apprentissage Vault** : 1 semaine pour maîtriser
- **Performance initiale** : appels Vault ralentissaient le démarrage
  - Solution : cache en mémoire avec TTL
- **Gestion des secrets de test** : confusion dev/prod au début
  - Solution : préfixes clairs (`sk_test_` vs `sk_live_`)

---

## 9. Prochaines étapes

- [ ] Rotation automatique des API keys (Stripe, SendGrid)
- [ ] Migration encryption key vers HSM
- [ ] Alerting sur accès suspects dans Vault
- [ ] Formation avancée équipe sur Vault policies
- [ ] Audit externe de la configuration Vault

---

## 10. Recommandations

**Pour une équipe similaire** :

1. **Commencer simple** : migrer d'abord les secrets les plus critiques
2. **Former l'équipe** : prévoir 1-2 jours de formation Vault
3. **Automatiser progressivement** : rotation manuelle d'abord, puis auto
4. **Ne pas négliger la doc** : guide d'onboarding = gain de temps énorme
5. **Tester la révocation** : simuler une compromission pour valider les procédures

---

**Ressources utilisées** :
- [Guide d'intégration Vault](vault-integration-guide.md)
- [Setup détection secrets](secrets-detection-setup.md)
- [Politique de rotation](secrets-rotation-policy-template.md)
