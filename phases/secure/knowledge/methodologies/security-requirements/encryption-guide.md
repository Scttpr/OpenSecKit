---
title: "Modèle d'exigences de chiffrement - Phase de conception"
template_version: "1.0.0"
constitutional_principle: "III - sécurité par conception"
ssdlc_phase: "conception"
last_updated: "2025-01-15"
reviewers:
  - security-champion-team
description: "Exigences de chiffrement couvrant données au repos, données en transit, gestion clés et algorithmes cryptographiques."
tags:
  - encryption
  - cryptography
  - tls
  - key-management
difficulty: "avancé"
estimated_time: "1-2 heures"
prerequisites:
  - "Classification données complétée"
  - "Modèle menaces identifiant données sensibles"
related_templates:
  - "../05-secrets-management/vault-integration-guide.md"
  - "owasp-asvs-checklist-design.md"
compliance_frameworks:
  - "NIST SP 800-52"
  - "NIST SP 800-57"
  - "PCI-DSS"
  - "HIPAA"
---

# Modèle d'exigences de chiffrement - Phase de conception

## 1. Classification données et besoins chiffrement

| Type données | Classification | Chiffrement au repos | Chiffrement en transit | Rotation clés |
|--------------|---------------|---------------------|----------------------|---------------|
| Mots de passe utilisateur | Secret | ✓ (hachés, pas chiffrés) | ✓ (TLS 1.3) | N/A (hachés) |
| Clés API | Secret | ✓ (AES-256-GCM) | ✓ (TLS 1.3) | 90 jours |
| PII utilisateur (email, nom) | Confidentiel | ✓ (chiffrement BD) | ✓ (TLS 1.3) | Annuel |
| Infos paiement | Secret | ✓ (Tokenisées, coffre PCI) | ✓ (TLS 1.3) | N/A (tokenisé) |
| Jetons session | Confidentiel | ✓ (chiffrement Redis) | ✓ (TLS 1.3) | Par session |
| Logs audit | Interne | ✓ (chiffrement BD) | ✓ (TLS 1.3) | Annuel |
| Contenu public | Public | ✗ | ✓ (TLS 1.3) | N/A |

---

## 2. Chiffrement au repos

### Chiffrement base de données

**Méthode** : ☑ Transparent Data Encryption (TDE) ☐ Chiffrement niveau application ☐ Chiffrement niveau colonne

**Base de données** : PostgreSQL avec extension pgcrypto

**Algorithme** : AES-256-GCM

**Gestion clés** :
- Clé de chiffrement maître stockée dans AWS KMS / HashiCorp Vault
- Clés de chiffrement base de données (DEK) chiffrées par clé maître (envelope encryption)
- Rotation clés : annuelle pour clé maître base de données

**Tables chiffrées** :
- `users` (colonnes PII)
- `api_keys`
- `payment_methods` (tokenisées, référence coffre uniquement)
- `audit_logs`

### Chiffrement stockage fichiers

**Stockage** : ☑ AWS S3 ☐ Azure Blob ☐ GCP Cloud Storage ☐ Sur site

**Chiffrement** : ☑ Server-Side Encryption (SSE-KMS) ☐ Chiffrement côté client

**Configuration** :
```
Chiffrement bucket S3: SSE-KMS
Clé KMS: Clé personnalisée gérée client
Rotation clés: Activée (rotation annuelle automatique)
Politique bucket: Requérir uploads chiffrés
```

### Chiffrement niveau application

**Cas d'usage** :
- Champs sensibles spécifiques nécessitant recherchabilité (ex : email - hash pour recherche, chiffrement pour stockage)
- Données nécessitant déchiffrement côté client

**Bibliothèque** : Module Node.js `crypto` / bibliothèque Python `cryptography`

**Algorithme** : AES-256-GCM (chiffrement authentifié)

**Stockage clés** : Récupérées de HashiCorp Vault au démarrage application

---

## 3. Chiffrement en transit

### Configuration TLS

**Protocole** : ☑ TLS 1.3 (préféré) ☑ TLS 1.2 (minimum) ☐ TLS 1.1 (déprécié) ☐ SSL (non sécurisé)

**Suites cipher** (ordre priorité) :
1. `TLS_AES_256_GCM_SHA384` (TLS 1.3)
2. `TLS_CHACHA20_POLY1305_SHA256` (TLS 1.3)
3. `TLS_AES_128_GCM_SHA256` (TLS 1.3)
4. `TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384` (TLS 1.2)
5. `TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256` (TLS 1.2)

**Suites cipher désactivées** :
- Tous ciphers mode CBC (attaques BEAST, Lucky 13)
- Tous ciphers RC4
- Tous ciphers DES/3DES
- Tous ciphers export-grade

**Certificat** :
- Fournisseur : Let's Encrypt / AWS Certificate Manager
- Type clé : RSA 2048-bit minimum (RSA 4096-bit ou ECDSA P-256 préféré)
- Validité : 90 jours (renouvellement automatique)
- Certificate transparency : activée

**HSTS (HTTP Strict Transport Security)** :
```
Strict-Transport-Security: max-age=31536000; includeSubDomains; preload
```

**Certificate pinning** : ☐ Implémenté ☑ Non requis (confiance CA)

### Communication interne service-à-service

**Microservices** : ☑ Mutual TLS (mTLS) ☐ Non Chiffré (réseau fiable)

**Configuration** :
- Service mesh : Istio / Linkerd pour mTLS automatique
- Émission certificats : CA interne (backend PKI Vault)
- Rotation certificats : toutes les 24 heures (automatique)
- Vérification : stricte (rejeter connexions sans certificat client valide)

---

## 4. Gestion clés

### Hiérarchie clés

```
Clé maître (HSM / KMS)
    ↓
Clés de chiffrement données (DEK) - par base de données, par service
    ↓
Éléments données individuels (chiffrés avec DEKs)
```

**Envelope encryption** : DEKs chiffrées par clé maître

### Stockage clés

**Gestionnaire secrets** : ☑ HashiCorp Vault ☐ AWS Secrets Manager ☐ Azure Key Vault ☐ GCP Secret Manager

**Contrôle d'accès** :
- Comptes service application uniquement
- Politiques IAM moindre privilège
- Journalisation audit pour tous accès clés

**Sauvegarde clés** :
- Données Vault chiffrées et sauvegardées quotidiennement
- Clés de chiffrement sauvegarde stockées hors ligne (réparties entre plusieurs dépositaires)

### Rotation clés

| Type clé | Fréquence rotation | Méthode |
|----------|-------------------|---------|
| Clé maître base de données | Annuelle | Automatisée via KMS |
| Clés chiffrement API | 90 jours | Déclenchement manuel, redémarrage app |
| Certificats TLS | 90 jours | Automatisée via Let's Encrypt |
| Certificats mTLS service | Quotidienne | Automatisée via Vault PKI |
| Clés signature (JWT) | 180 jours | Automatisée via Vault |

**Processus rotation** :
1. Générer nouvelle version clé
2. Chiffrer nouvelles données avec nouvelle clé
3. Re-chiffrer anciennes données avec nouvelle clé (tâche arrière-plan)
4. Retirer ancienne clé après période grâce (30 jours)
5. Logger événement rotation

---

## 5. Algorithmes cryptographiques

### Algorithmes approuvés

| Cas d'usage | Algorithme | Taille clé | Mode |
|-------------|-----------|------------|------|
| **Chiffrement symétrique** | AES | 256-bit | GCM (authentifié) |
| **Chiffrement asymétrique** | RSA | 4096-bit | Padding OAEP |
| **Échange clés** | ECDH | P-256, P-384 | - |
| **Signatures numériques** | ECDSA | P-256 | - |
| **Hachage** | SHA-2 | 256-bit, 512-bit | - |
| **Hachage mots de passe** | bcrypt | - | Facteur travail 12 |
| **HMAC** | HMAC-SHA256 | Clé 256-bit | - |
| **Génération nombres aléatoires** | CSPRNG | - | crypto.randomBytes() |

### Algorithmes interdits

- ❌ MD5 (cryptographiquement cassé)
- ❌ SHA-1 (déprécié, attaques collision)
- ❌ DES, 3DES (longueur clé insuffisante)
- ❌ RC4 (vulnérabilités cipher flux)
- ❌ Mode ECB (fuites motifs)
- ❌ Crypto personnalisé/propriétaire

---

## 6. Tokenisation

**Données paiement** : ☑ Tokenisées (jamais stocker numéros carte bruts)

**Service tokenisation** : Tierce partie conforme PCI (Stripe, PayPal, Braintree)

**Stockage jetons** :
```json
{
  "payment_method_id": "uuid",
  "token": "tok_xyz123", // Jeton Stripe
  "last_four": "4242",
  "exp_month": 12,
  "exp_year": 2025
}
```

**Avantages** :
- Pas de périmètre PCI DSS pour application
- Tierce partie gère chiffrement et sécurité données carte

---

## 7. Exemple chiffrement niveau application

### Chiffrement niveau champ

```javascript
// Chiffrement
const crypto = require('crypto');
const algorithm = 'aes-256-gcm';

function encrypt(plaintext, key) {
  const iv = crypto.randomBytes(16);
  const cipher = crypto.createCipheriv(algorithm, key, iv);

  let encrypted = cipher.update(plaintext, 'utf8', 'hex');
  encrypted += cipher.final('hex');

  const authTag = cipher.getAuthTag();

  return {
    iv: iv.toString('hex'),
    encryptedData: encrypted,
    authTag: authTag.toString('hex')
  };
}

// Déchiffrement
function decrypt(encryptedObj, key) {
  const decipher = crypto.createDecipheriv(
    algorithm,
    key,
    Buffer.from(encryptedObj.iv, 'hex')
  );

  decipher.setAuthTag(Buffer.from(encryptedObj.authTag, 'hex'));

  let decrypted = decipher.update(encryptedObj.encryptedData, 'hex', 'utf8');
  decrypted += decipher.final('utf8');

  return decrypted;
}
```

---

## 8. Exigences conformité

### PCI-DSS

- ☑ Exigence 3.4 : Rendre PAN illisible (tokenisation)
- ☑ Exigence 3.5 : Documenter procédures gestion clés
- ☑ Exigence 3.6 : Documenter entièrement processus gestion clés
- ☑ Exigence 4.1 : Utiliser cryptographie forte pour transmission sur réseaux ouverts

### HIPAA

- ☑ § 164.312(a)(2)(iv) chiffrement : implémenter chiffrement pour ePHI
- ☑ § 164.312(e)(1) sécurité transmission : implémenter mesures sécurité techniques pour protéger contre accès non autorisé ePHI transmises

### RGPD

- ☑ Article 32(1)(a) : Chiffrement données personnelles comme mesure sécurité

---

## 9. Liste de vérification implémentation

### Développement

- [ ] Implémenter TLS 1.3 pour tous endpoints externes
- [ ] Configurer en-têtes HSTS
- [ ] Activer chiffrement base de données (TDE ou niveau colonne)
- [ ] Implémenter envelope encryption avec KMS/Vault
- [ ] Utiliser AES-256-GCM pour chiffrement niveau application
- [ ] Intégrer HashiCorp Vault pour stockage clés
- [ ] Implémenter rotation automatique clés
- [ ] Utiliser bcrypt pour hachage mots de passe
- [ ] Activer chiffrement bucket S3 (SSE-KMS)
- [ ] Implémenter mTLS pour microservices

### Tests sécurité

- [ ] Scanner configurations TLS faibles (SSL Labs, testssl.sh)
- [ ] Vérifier chiffrement base de données activé
- [ ] Tester processus rotation clés
- [ ] Vérifier aucune donnée sensible transmise non chiffrée
- [ ] Test intrusion faiblesses chiffrement
- [ ] Revue code clés codées en dur (gitleaks, Semgrep)

---

## Ressources liées

- [Guide intégration HashiCorp Vault](../05-secrets-management/vault-integration-guide.md)
- [OWASP Cryptographic Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Cryptographic_Storage_Cheat_Sheet.html)
- [NIST SP 800-52r2 - Directives TLS](https://csrc.nist.gov/publications/detail/sp/800-52/rev-2/final)
- [NIST SP 800-57 - Gestion clés](https://csrc.nist.gov/publications/detail/sp/800-57-part-1/rev-5/final)
