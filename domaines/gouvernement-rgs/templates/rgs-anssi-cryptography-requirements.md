---
title: "Exigences Cryptographiques RGS ANSSI"
template_version: "1.0.0"
domain: "government-rgs"
constitutional_principles:
  - "III - Security by Design"
  - "IV - Defense in Depth"
  - "II - Risk Analysis"
regulatory_references:
  - "RGS v2.0 - Annexe B4 (Confidentialité)"
  - "ANSSI - Mécanismes cryptographiques (v2.08, 2024)"
  - "ANSSI - Recommandations de sécurité relatives à TLS (v1.3, 2024)"
  - "eIDAS Regulation (EU) 910/2014 - Qualified Trust Services"
ssdlc_phase: "III - Design"
difficulty: "advanced"
estimated_time: "60-80 heures (implémentation initiale), 16-24 heures (revue annuelle)"
---

# Exigences Cryptographiques RGS ANSSI

## 📋 Objectif

Ce modèle fournit les **exigences cryptographiques obligatoires** pour les systèmes et applications du gouvernement français traitant des données sensibles, basées sur les recommandations **ANSSI** (Agence nationale de la sécurité des systèmes d'information) et l'**Annexe B4 du RGS v2.0** (Confidentialité).

**Contexte Réglementaire** :
- **RGS v2.0 Annexe B4** : Les mécanismes cryptographiques doivent respecter des niveaux de sécurité proportionnels à la classification des données (niveaux DCP, DR, NP)
- **ANSSI Mécanismes cryptographiques (v2.08, 2024)** : Liste autoritaire des algorithmes approuvés, tailles de clés et implémentations
- **Règlement eIDAS** : Les signatures et cachets électroniques qualifiés nécessitent une cryptographie approuvée ANSSI

**Alignement Constitutionnel** :
- **Principe III (Security by Design)** : La cryptographie est un contrôle fondamental intégré dans l'architecture système
- **Principe IV (Defense in Depth)** : Multiples couches cryptographiques (data-at-rest, data-in-transit, authentification)
- **Principe II (Risk Analysis)** : Sélection d'algorithmes basée sur modèle de menaces et sensibilité des données

**Applicabilité** :
- ✅ Systèmes administration publique française (ministères, collectivités locales, agences publiques)
- ✅ Entreprises privées fournissant services au gouvernement français par délégation
- ✅ Opérateurs infrastructures critiques soumis à LPM (Loi de programmation militaire)
- ✅ Systèmes secteur santé traitant données de santé (certification HDS)
- ⚠️ Secteur privé (recommandations applicables, non obligatoires sauf obligation contractuelle)

---

## 1. Niveaux de Sécurité RGS et Exigences Cryptographiques

### 1.1 Classification des Données (RGS Annexe B4)

Les données du gouvernement français sont classées en trois niveaux de confidentialité :

| Niveau | Nom Complet | Définition | Exemples | Taille Clé Min. |
|--------|-------------|------------|----------|-----------------|
| **DCP** | Diffusion de Contrôle Public | Information accessible publiquement | Open data, rapports publiés | **128-bit** (AES-128) |
| **DR** | Diffusion Restreinte | Distribution restreinte, non-public mais non classifié | Mémos internes, données personnelles citoyens | **192-bit** (AES-192) ou **256-bit** (AES-256) |
| **NP** | Non Protégé (mais sensible) | Opérations gouvernementales sensibles | Documents prédécisionnels, évaluations marchés publics | **256-bit** (AES-256) |
| **CD** | Confidentiel Défense | Information classifiée défense | **Régime séparé** - requiert approbation IGI 1300 (non couvert par ce modèle) |

**Recommandation par Défaut** : Utiliser **AES-256** pour toutes les données DR et NP (simplifie la gestion des clés, pérenne).

### 1.2 Correspondance Robustesse Cryptographique

| Niveau de Sécurité | Taille Clé Symétrique | Taille Clé Asymétrique (RSA) | Taille Clé Asymétrique (ECC) | Fonction de Hachage |
|-------------------|----------------------|------------------------------|------------------------------|---------------------|
| **DCP** (Faible) | AES-128 (128-bit) | RSA-2048 | ECDSA P-256 (256-bit) | SHA-256 |
| **DR** (Moyenne) | AES-192 ou AES-256 | RSA-3072 ou RSA-4096 | ECDSA P-384 (384-bit) | SHA-384 |
| **NP** (Élevée) | AES-256 | RSA-4096 | ECDSA P-521 (521-bit) | SHA-512 |

**Recommandation ANSSI** : Pour les nouveaux systèmes déployés en 2025+, utiliser **AES-256, RSA-4096, ECDSA P-384** indépendamment du niveau de classification pour tenir compte des avancées cryptographiques et menaces informatique quantique.

---

## 2. Algorithmes Cryptographiques Approuvés (ANSSI v2.08, 2024)

### 2.1 Chiffrement Symétrique (Confidentialité)

**✅ Algorithmes APPROUVÉS** :

| Algorithme | Mode | Taille Clé | Cas d'Usage | Statut ANSSI | Date Limite Migration |
|------------|------|------------|-------------|--------------|----------------------|
| **AES** (Advanced Encryption Standard) | GCM, CCM, CBC+HMAC | 128, 192, **256-bit** | Data-at-rest, data-in-transit | ✅ **Recommandé** | Aucune (utiliser jusqu'en 2050+) |
| **ChaCha20-Poly1305** | AEAD | 256-bit | Mobile/IoT (meilleures performances qu'AES sur CPUs sans AES-NI) | ✅ **Autorisé** | Aucune |

**⚠️ Algorithmes DÉPRÉCIÉS** (à migrer) :

| Algorithme | Raison | Date Limite Migration |
|------------|--------|----------------------|
| **3DES** (Triple DES) | Force clé effective 112-bit (insuffisante) | ❌ **Interdit** depuis 2024 |
| **RC4** | Biais dans flux de clés (complexité attaque <2^40) | ❌ **Interdit** depuis 2015 |
| **DES** | Clé 56-bit (force brute possible) | ❌ **Interdit** depuis 2010 |

**Modes de Chiffrement** :

| Mode | Authentification | Cas d'Usage | Statut ANSSI |
|------|------------------|-------------|--------------|
| **GCM** (Galois/Counter Mode) | ✅ Authentifié (AEAD) | **Préféré** - TLS 1.3, chiffrement disque, chiffrement API | ✅ **Recommandé** |
| **CCM** (Counter with CBC-MAC) | ✅ Authentifié (AEAD) | IoT, appareils à ressources limitées | ✅ **Autorisé** |
| **CBC** (Cipher Block Chaining) | ❌ Pas d'authentification (requiert HMAC séparé) | Systèmes legacy uniquement | ⚠️ **Autorisé** (doit utiliser CBC+HMAC-SHA256, éviter attaques padding oracle) |
| **ECB** (Electronic Codebook) | ❌ Pas d'authentification, pas d'IV (déterministe) | ❌ **Interdit** (révèle motifs dans plaintext) | ❌ **Ne jamais utiliser** |

**Example (Python - AES-256-GCM)**:
```python
from cryptography.hazmat.primitives.ciphers.aead import AESGCM
import os

# Generate 256-bit key (store in HSM or secrets manager)
key = AESGCM.generate_key(bit_length=256)
aesgcm = AESGCM(key)

# Encrypt data
nonce = os.urandom(12)  # 96-bit nonce (GCM recommendation)
plaintext = b"Sensitive government data"
ciphertext = aesgcm.encrypt(nonce, plaintext, associated_data=None)

# Decrypt data
decrypted = aesgcm.decrypt(nonce, ciphertext, associated_data=None)
```

### 2.2 Asymmetric Encryption (Chiffrement asymétrique)

**✅ APPROVED Algorithms**:

| Algorithm | Key Size | Use Case | ANSSI Status | Quantum Resistance |
|-----------|----------|----------|--------------|-------------------|
| **RSA** | 2048-bit (minimum), **4096-bit (recommended)** | TLS, code signing, key exchange | ✅ **Recommended** | ❌ Vulnerable to quantum (Shor's algorithm) |
| **ECDH** (Elliptic Curve Diffie-Hellman) | P-256, **P-384**, P-521 | Key exchange (TLS 1.3) | ✅ **Recommended** | ❌ Vulnerable to quantum |
| **X25519** (Curve25519) | 256-bit | Modern key exchange (TLS 1.3, SSH) | ✅ **Allowed** | ❌ Vulnerable to quantum |

**⚠️ MIGRATION TIMELINE**:
- **RSA-1024**: ❌ **Prohibited** since 2020 (factorizable with current computing power)
- **RSA-2048**: ⚠️ **Allowed** until 2030, then deprecated (migrate to RSA-4096 or post-quantum)
- **RSA-4096**: ✅ **Recommended** until quantum computers become viable (estimated 2035-2040)

**Post-Quantum Cryptography (PQC)**:
- **NIST PQC Standards (2024)**: CRYSTALS-Kyber (key encapsulation), CRYSTALS-Dilithium (digital signatures)
- **ANSSI Recommendation**: Begin planning migration to PQC for systems with >10-year lifespan
- **Timeline**: Hybrid TLS (classical + PQC) by 2028, full PQC by 2035

### 2.3 Digital Signatures

**✅ APPROVED Algorithms**:

| Algorithm | Key Size | Use Case | ANSSI Status |
|-----------|----------|----------|--------------|
| **RSA-PSS** (Probabilistic Signature Scheme) | 2048-bit (min), **4096-bit (recommended)** | Code signing, document signing | ✅ **Recommended** (preferred over PKCS#1 v1.5) |
| **ECDSA** (Elliptic Curve DSA) | P-256, **P-384**, P-521 | TLS certificates, JWT signing | ✅ **Recommended** |
| **EdDSA** (Ed25519, Ed448) | 256-bit (Ed25519), 448-bit (Ed448) | SSH keys, Git commit signing | ✅ **Allowed** |

**⚠️ DEPRECATED**:
- **RSA-PKCS#1 v1.5**: ⚠️ Allowed but discouraged (vulnerable to padding oracle attacks - use RSA-PSS instead)
- **DSA**: ❌ **Prohibited** (weak random number generation leads to private key recovery)

**Example (Python - RSA-PSS Signature)**:
```python
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.asymmetric import rsa, padding

# Generate RSA-4096 key pair
private_key = rsa.generate_private_key(
    public_exponent=65537,
    key_size=4096
)
public_key = private_key.public_key()

# Sign document
message = b"Official government decree"
signature = private_key.sign(
    message,
    padding.PSS(
        mgf=padding.MGF1(hashes.SHA384()),
        salt_length=padding.PSS.MAX_LENGTH
    ),
    hashes.SHA384()
)

# Verify signature
public_key.verify(
    signature,
    message,
    padding.PSS(
        mgf=padding.MGF1(hashes.SHA384()),
        salt_length=padding.PSS.MAX_LENGTH
    ),
    hashes.SHA384()
)
```

### 2.4 Hash Functions

**✅ APPROVED Algorithms**:

| Algorithm | Output Size | Use Case | ANSSI Status | Collision Resistance |
|-----------|-------------|----------|--------------|---------------------|
| **SHA-256** | 256-bit | File integrity, password hashing (with salt+pepper), HMAC | ✅ **Recommended** | ✅ Strong (2^128 attack complexity) |
| **SHA-384** | 384-bit | High-security signatures, TLS 1.3 | ✅ **Recommended** | ✅ Very strong (2^192) |
| **SHA-512** | 512-bit | NP-level data, long-term archival | ✅ **Recommended** | ✅ Very strong (2^256) |
| **SHA-3** (Keccak) | 256, 384, 512-bit | Alternative to SHA-2 (different construction) | ✅ **Allowed** | ✅ Strong |
| **BLAKE2** | Variable (up to 512-bit) | High-performance hashing (faster than SHA-2) | ✅ **Allowed** | ✅ Strong |

**⚠️ PROHIBITED Algorithms**:

| Algorithm | Reason | Migration Deadline |
|-----------|--------|-------------------|
| **MD5** | Collision attacks (2^20 complexity) | ❌ **Prohibited** since 2010 |
| **SHA-1** | Collision attacks (SHAttered attack, 2017) | ❌ **Prohibited** since 2020 |

**Password Hashing** (special case):
- ❌ **Do NOT use SHA-256 alone** for password hashing (too fast, vulnerable to GPU brute-force)
- ✅ **Use password hashing algorithms**: **Argon2id** (recommended), bcrypt, scrypt, PBKDF2-HMAC-SHA256

**Example (Python - Argon2id)**:
```python
from argon2 import PasswordHasher

ph = PasswordHasher(
    time_cost=3,        # 3 iterations
    memory_cost=65536,  # 64 MB memory
    parallelism=4,      # 4 threads
    hash_len=32,        # 256-bit output
    salt_len=16         # 128-bit salt
)

# Hash password
password_hash = ph.hash("user_password")

# Verify password
try:
    ph.verify(password_hash, "user_password")
    print("Password valid")
except:
    print("Invalid password")
```

---

## 3. TLS/SSL Configuration (ANSSI TLS v1.3, 2024)

### 3.1 Minimum TLS Version

| Protocol | ANSSI Status | Use Case | Migration Deadline |
|----------|--------------|----------|-------------------|
| **TLS 1.3** | ✅ **Recommended** | All new systems (2024+) | Use by default |
| **TLS 1.2** | ✅ **Allowed** | Legacy client support | Migrate to TLS 1.3 by 2028 |
| **TLS 1.1** | ❌ **Prohibited** | N/A | Disabled since 2020 |
| **TLS 1.0** | ❌ **Prohibited** | N/A | Disabled since 2020 |
| **SSL 3.0, SSL 2.0** | ❌ **Prohibited** | N/A | Disabled since 2015 (POODLE attack) |

**Configuration (Nginx)**:
```nginx
ssl_protocols TLSv1.2 TLSv1.3;
ssl_prefer_server_ciphers on;
```

### 3.2 Approved Cipher Suites (TLS 1.2)

**✅ RECOMMENDED Cipher Suites** (in order of preference):

```
# TLS 1.3 (simplified cipher suite negotiation)
TLS_AES_256_GCM_SHA384                     # AES-256-GCM with SHA-384
TLS_CHACHA20_POLY1305_SHA256               # ChaCha20-Poly1305 (mobile-friendly)
TLS_AES_128_GCM_SHA256                     # AES-128-GCM (acceptable for DCP level)

# TLS 1.2 (legacy support)
ECDHE-RSA-AES256-GCM-SHA384                # Forward secrecy + AEAD
ECDHE-ECDSA-AES256-GCM-SHA384
ECDHE-RSA-AES128-GCM-SHA256
ECDHE-ECDSA-AES128-GCM-SHA256
```

**🔒 Key Requirements**:
- ✅ **Perfect Forward Secrecy (PFS)**: Use ECDHE or DHE key exchange (ephemeral keys)
- ✅ **Authenticated Encryption (AEAD)**: Use GCM or CHACHA20-POLY1305 modes
- ❌ **No CBC mode**: Vulnerable to BEAST, Lucky13, and padding oracle attacks
- ❌ **No static DH**: Does not provide forward secrecy

**❌ PROHIBITED Cipher Suites**:
```
# Weak encryption
DES-CBC3-SHA                               # 3DES (weak)
RC4-SHA                                    # RC4 (biased keystream)
NULL                                       # No encryption

# No forward secrecy
AES256-GCM-SHA384                          # Static RSA key exchange (no PFS)

# Weak authentication
aNULL                                      # No authentication (MITM vulnerable)
eNULL                                      # No encryption
```

**Example (Nginx TLS 1.3 Configuration)**:
```nginx
server {
    listen 443 ssl http2;
    server_name example.gouv.fr;

    # TLS versions
    ssl_protocols TLSv1.2 TLSv1.3;

    # TLS 1.3 cipher suites (automatically used if TLSv1.3 negotiated)
    ssl_ciphers 'ECDHE-RSA-AES256-GCM-SHA384:ECDHE-RSA-AES128-GCM-SHA256';
    ssl_prefer_server_ciphers on;

    # Certificates (ANSSI-approved CA)
    ssl_certificate /etc/ssl/certs/example.gouv.fr.crt;
    ssl_certificate_key /etc/ssl/private/example.gouv.fr.key;

    # OCSP stapling (certificate revocation check)
    ssl_stapling on;
    ssl_stapling_verify on;
    ssl_trusted_certificate /etc/ssl/certs/ca-chain.crt;

    # HSTS (force HTTPS for 2 years)
    add_header Strict-Transport-Security "max-age=63072000; includeSubDomains; preload" always;

    # DH parameters (for DHE cipher suites)
    ssl_dhparam /etc/ssl/certs/dhparam4096.pem;

    # Session caching (performance optimization)
    ssl_session_cache shared:SSL:50m;
    ssl_session_timeout 1d;
    ssl_session_tickets off;  # Disable for perfect forward secrecy
}
```

**Generate DH Parameters (4096-bit)**:
```bash
openssl dhparam -out /etc/ssl/certs/dhparam4096.pem 4096
```

### 3.3 Certificate Requirements

**Certificate Authority (CA) Requirements**:
- ✅ Use certificates from **ANSSI-approved CAs**:
  - **ANTS (Agence Nationale des Titres Sécurisés)** - French government PKI
  - **ChamberSign France** (for professionals)
  - **Certinomis** (for businesses)
  - **Let's Encrypt** (allowed for DCP-level public websites, but not for DR/NP)
- ❌ Self-signed certificates prohibited in production (except internal development)

**Certificate Properties**:

| Property | Requirement | Example |
|----------|-------------|---------|
| **Key Algorithm** | RSA-4096 or ECDSA P-384 | `Public-Key: (4096 bit)` |
| **Signature Algorithm** | SHA-256, SHA-384, or SHA-512 | `Signature Algorithm: sha384WithRSAEncryption` |
| **Validity Period** | Maximum **397 days** (13 months) per CA/Browser Forum Baseline Requirements | Not before: 2025-01-01, Not after: 2026-02-01 |
| **Subject Alternative Names (SAN)** | Include all domain variants (www, non-www) | `DNS:example.gouv.fr, DNS:www.example.gouv.fr` |
| **Certificate Transparency (CT)** | Embedded SCT (Signed Certificate Timestamp) | Required for Chrome/Firefox acceptance |

**Example (Generate CSR for RSA-4096)**:
```bash
# Generate private key (store in HSM in production)
openssl genrsa -out example.gouv.fr.key 4096

# Generate Certificate Signing Request (CSR)
openssl req -new -key example.gouv.fr.key -out example.gouv.fr.csr -sha384 \
  -subj "/C=FR/ST=Ile-de-France/L=Paris/O=Ministere de l'Exemple/CN=example.gouv.fr"

# Submit CSR to ANSSI-approved CA (e.g., ChamberSign France)
```

**Certificate Revocation Checking**:
- ✅ Implement **OCSP Stapling** (Online Certificate Status Protocol) - server fetches revocation status and includes in TLS handshake
- ✅ Fallback to **CRL** (Certificate Revocation List) if OCSP unavailable
- ⚠️ **Do NOT disable revocation checking** (leaves you vulnerable to compromised certificates)

---

## 4. Key Management (RGS Annexe B4)

### 4.1 Key Generation

**Entropy Requirements**:
- ✅ Use **cryptographically secure random number generators (CSRNG)**:
  - Linux: `/dev/urandom` (not `/dev/random` - unnecessary blocking)
  - Windows: `CryptGenRandom()` API
  - Hardware: **TRNG** (True Random Number Generator) in HSM
- ❌ **Never use**: `rand()`, `Math.random()`, `mt_rand()`, predictable seeds (timestamps, PIDs)

**Example (Python)**:
```python
import secrets

# Generate 256-bit AES key
aes_key = secrets.token_bytes(32)  # 32 bytes = 256 bits

# Generate 128-bit random nonce/IV
nonce = secrets.token_bytes(16)
```

**Key Generation Locations**:

| Key Type | Generation Location | Storage Location |
|----------|---------------------|------------------|
| **Long-term keys** (TLS certificates, code signing) | **HSM** (Hardware Security Module) | HSM (never export) |
| **Session keys** (TLS session, ephemeral ECDHE) | Application server (in-memory) | In-memory (destroy after use) |
| **Data encryption keys (DEK)** | Application or HSM | Encrypted by KEK, stored in database |
| **Key encryption keys (KEK)** | **HSM** | HSM (never export) |

### 4.2 Key Storage

**Storage Mechanisms** (in order of security):

| Mechanism | Security Level | Use Case | ANSSI Compliance |
|-----------|----------------|----------|------------------|
| **HSM** (FIPS 140-2 Level 3+) | ⭐⭐⭐⭐⭐ Highest | Long-term keys (TLS, code signing, KEKs) | ✅ **Required** for NP-level data |
| **TPM** (Trusted Platform Module) | ⭐⭐⭐⭐ High | Device-bound keys (disk encryption, device attestation) | ✅ **Recommended** for endpoint security |
| **Cloud KMS** (AWS KMS, Azure Key Vault, GCP KMS) | ⭐⭐⭐ Medium-High | Cloud-native applications (DR-level data) | ✅ **Allowed** (if EU-hosted, GDPR-compliant) |
| **Encrypted file** (AES-256-GCM, key derived from passphrase) | ⭐⭐ Medium | Development/testing only | ⚠️ **Not recommended** for production |
| **Environment variables** | ⭐ Low | Legacy systems migration only | ❌ **Discouraged** (visible in process list, logs) |
| **Hardcoded in source code** | ☠️ None | ❌ **Never acceptable** | ❌ **Prohibited** |

**HSM Recommendations**:
- **ANSSI-Qualified HSMs**: Thales nShield, Utimaco SecurityServer
- **Certification**: FIPS 140-2 Level 3 or Common Criteria EAL4+
- **Deployment**: On-premises HSM for NP-level data; Cloud HSM (AWS CloudHSM, Azure Dedicated HSM) acceptable for DR-level

**Example (AWS KMS - Envelope Encryption)**:
```python
import boto3
import base64
from cryptography.hazmat.primitives.ciphers.aead import AESGCM

kms = boto3.client('kms', region_name='eu-west-3')  # Paris region (GDPR-compliant)

# Generate data encryption key (DEK) from KMS
response = kms.generate_data_key(
    KeyId='arn:aws:kms:eu-west-3:123456789012:key/...',
    KeySpec='AES_256'
)

plaintext_key = response['Plaintext']        # Use to encrypt data
encrypted_key = response['CiphertextBlob']   # Store alongside encrypted data

# Encrypt data with DEK
aesgcm = AESGCM(plaintext_key)
nonce = os.urandom(12)
ciphertext = aesgcm.encrypt(nonce, b"Sensitive data", None)

# Store: encrypted_key + nonce + ciphertext (DEK never stored in plaintext)

# Decrypt: First decrypt DEK with KMS, then decrypt data
dek_plaintext = kms.decrypt(CiphertextBlob=encrypted_key)['Plaintext']
aesgcm = AESGCM(dek_plaintext)
plaintext = aesgcm.decrypt(nonce, ciphertext, None)
```

### 4.3 Key Rotation

**Rotation Frequency** (ANSSI recommendations):

| Key Type | Rotation Frequency | Rationale |
|----------|-------------------|-----------|
| **TLS Certificates** | **365 days** (13 months max per CA/B Forum) | Industry standard, limits exposure window |
| **SSH Host Keys** | **730 days** (2 years) | Balance between security and operational overhead |
| **API Keys / OAuth Client Secrets** | **90 days** | High-value targets, frequently exposed in logs |
| **Data Encryption Keys (DEK)** | **30 days** (for DR/NP data) | Limit amount of data encrypted with single key |
| **Master Key Encryption Keys (KEK)** | **365 days** | Rarely used directly, protected by HSM |
| **JWT Signing Keys** | **90-180 days** | Limits impact of key compromise |

**Automated Rotation** (example with Kubernetes + cert-manager):
```yaml
apiVersion: cert-manager.io/v1
kind: Certificate
metadata:
  name: example-gouv-fr-tls
spec:
  secretName: example-gouv-fr-tls
  duration: 8760h  # 365 days
  renewBefore: 720h  # Renew 30 days before expiry
  commonName: example.gouv.fr
  dnsNames:
    - example.gouv.fr
    - www.example.gouv.fr
  issuerRef:
    name: letsencrypt-prod
    kind: ClusterIssuer
```

### 4.4 Key Destruction

**Secure Deletion** (when key is no longer needed):

| Storage Medium | Destruction Method | ANSSI Compliance |
|----------------|-------------------|------------------|
| **HSM** | Use HSM `zeroize` command (FIPS 140-2 requirement) | ✅ Compliant |
| **RAM** | Overwrite memory with zeros, invoke garbage collector | ✅ Compliant |
| **SSD/Flash** | **Cryptographic erasure** (destroy KEK, rendering encrypted data unrecoverable) | ✅ **Recommended** (physical overwrite ineffective on SSDs) |
| **HDD** | DoD 5220.22-M (7-pass overwrite) or physical destruction | ✅ Compliant |
| **Backup Tapes** | Physical destruction (shredding, incineration) | ✅ Compliant |

**Example (Secure Memory Wipe in Python)**:
```python
import ctypes

def secure_zero(data: bytearray):
    """Securely zero out memory (prevent compiler optimization)"""
    # Use ctypes to prevent Python from optimizing away the memset
    ctypes.memset(id(data), 0, len(data))

# Usage
secret_key = bytearray(b"my_secret_key_12345678901234567890")
# ... use key ...
secure_zero(secret_key)  # Zero out before garbage collection
del secret_key
```

---

## 5. Data-at-Rest Encryption

### 5.1 Database Encryption

**Encryption Layers**:

| Layer | Technology | Use Case | ANSSI Compliance |
|-------|------------|----------|------------------|
| **Application-Level (Field Encryption)** | AES-256-GCM per field | Encrypt specific sensitive columns (e.g., SSN, credit card) | ✅ **Recommended** for PII |
| **Transparent Data Encryption (TDE)** | Database-native (PostgreSQL pgcrypto, MySQL TDE) | Encrypt entire database files | ✅ **Recommended** |
| **Filesystem-Level (LUKS, dm-crypt)** | OS-level encryption (Linux LUKS, Windows BitLocker) | Encrypt entire disk/partition | ✅ **Required** for laptops, removable media |

**Example (PostgreSQL - Field-Level Encryption with pgcrypto)**:
```sql
-- Enable pgcrypto extension
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Create table with encrypted field
CREATE TABLE citizens (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255),
    ssn_encrypted BYTEA,  -- Encrypted Social Security Number
    created_at TIMESTAMP DEFAULT NOW()
);

-- Insert with encryption (using AES-256-GCM)
-- Key should be fetched from HSM/KMS, not hardcoded
INSERT INTO citizens (name, ssn_encrypted)
VALUES (
    'Jean Dupont',
    pgp_sym_encrypt('1234567890123', 'encryption_key_from_hsm', 'cipher-algo=aes256')
);

-- Query with decryption
SELECT
    name,
    pgp_sym_decrypt(ssn_encrypted, 'encryption_key_from_hsm') AS ssn
FROM citizens
WHERE id = 1;
```

**Key Management for TDE**:
- ✅ Store master key in **HSM** or **Cloud KMS** (not on same server as database)
- ✅ Use **envelope encryption**: TDE uses database encryption key (DEK), DEK is encrypted by master key (KEK) in HSM
- ⚠️ **Backup encryption**: Encrypted backups must use different keys (prevent backup compromise from leaking prod data)

### 5.2 Filesystem Encryption (LUKS)

**Linux Unified Key Setup (LUKS)** - recommended for French government servers:

```bash
# Create encrypted partition (AES-256-XTS)
cryptsetup luksFormat /dev/sdb1 --cipher aes-xts-plain64 --key-size 512 --hash sha256

# Open encrypted partition
cryptsetup luksOpen /dev/sdb1 encrypted_data

# Create filesystem
mkfs.ext4 /dev/mapper/encrypted_data

# Mount
mount /dev/mapper/encrypted_data /mnt/secure_data
```

**LUKS Configuration (ANSSI-compliant)**:

| Parameter | Value | Rationale |
|-----------|-------|-----------|
| **Cipher** | `aes-xts-plain64` | XTS mode prevents manipulation attacks on block storage |
| **Key Size** | `512` (AES-256-XTS uses 512-bit key = 2x256-bit keys) | ANSSI recommendation for DR/NP data |
| **Hash** | `sha256` | PBKDF2 password stretching |
| **Iteration Count** | ≥100,000 | Slow down brute-force attacks (adjust based on hardware) |

**Key Escrow** (for government systems):
- ✅ Store LUKS key escrow in **hardware-encrypted USB token** (YubiKey, Nitrokey) kept in physical safe
- ✅ Use **multi-party key splitting** (Shamir's Secret Sharing): Require M-of-N keyholders to decrypt (e.g., 3-of-5 administrators)
- ⚠️ **Document key escrow procedures** in incident response plan (for emergency decryption)

---

## 6. Code Signing and Software Integrity

### 6.1 Code Signing Requirements

**When Code Signing is Mandatory** (RGS + eIDAS):
- ✅ Government software distributed to citizens (desktop apps, mobile apps)
- ✅ Firmware updates for IoT devices / critical infrastructure
- ✅ Software packages for government procurement (compliance verification)
- ✅ Docker images deployed to production (supply chain security)

**Signature Algorithm**:
- ✅ **RSA-4096 with SHA-384** (recommended for long-term validity, e.g., 5-year certificates)
- ✅ **ECDSA P-384 with SHA-384** (acceptable for short-term signatures, e.g., CI/CD builds)

**Example (Sign Java JAR with jarsigner)**:
```bash
# Generate code signing certificate (from ANSSI-approved CA)
# Certificate must have "Code Signing" extended key usage

# Sign JAR file
jarsigner -keystore code-signing.p12 \
          -storepass <password> \
          -digestalg SHA-384 \
          -sigalg SHA384withRSA \
          application.jar \
          code-signing-cert

# Verify signature
jarsigner -verify -verbose -certs application.jar
```

**Example (Sign Docker Image with Docker Content Trust)**:
```bash
# Enable Docker Content Trust (uses Notary with ECDSA P-256)
export DOCKER_CONTENT_TRUST=1

# Sign and push image
docker trust sign registry.example.gouv.fr/app:v1.0.0

# Verify signature on pull
docker pull registry.example.gouv.fr/app:v1.0.0
# (will fail if signature invalid or image tampered)
```

### 6.2 Software Bill of Materials (SBOM)

**ANSSI Recommendation** (2024): Include **SBOM** with all signed software to enable vulnerability tracking.

**Example (Generate SBOM with Syft)**:
```bash
# Install Syft
curl -sSfL https://raw.githubusercontent.com/anchore/syft/main/install.sh | sh

# Generate SBOM in CycloneDX format (OWASP standard)
syft packages dir:. -o cyclonedx-json > sbom.json

# Sign SBOM
gpg --detach-sign --armor sbom.json
# Produces sbom.json.asc
```

---

## 7. Quantum-Resistant Cryptography (PQC)

### 7.1 Post-Quantum Threat Timeline

**ANSSI Quantum Threat Assessment** (2024):

| Year | Quantum Computing Capability | Cryptographic Impact | Recommendation |
|------|------------------------------|---------------------|----------------|
| **2025-2030** | Small-scale quantum computers (<100 qubits) | No immediate threat to RSA-4096/AES-256 | ✅ Continue using classical crypto |
| **2030-2035** | Medium-scale quantum (1000+ qubits) | **RSA-2048 at risk** (Shor's algorithm) | ⚠️ Migrate RSA-2048 → RSA-4096 or PQC |
| **2035-2040** | Large-scale fault-tolerant quantum | **All RSA/ECC at risk**, AES-128 weakened | ❌ Classical asymmetric crypto broken, migrate to PQC |

**Harvest Now, Decrypt Later (HNDL)** Attack:
- **Threat**: Adversaries collect encrypted traffic today, decrypt with quantum computer in 2035+
- **Risk**: Long-term secrets (government archives, health records, defense plans) vulnerable
- **Mitigation**: Use PQC **now** for data with >10-year confidentiality requirement

### 7.2 NIST Post-Quantum Standards (2024)

**Approved PQC Algorithms** (NIST FIPS 203/204/205):

| Algorithm | Type | Use Case | ANSSI Status |
|-----------|------|----------|--------------|
| **CRYSTALS-Kyber** (FIPS 203) | Key Encapsulation Mechanism (KEM) | TLS key exchange, hybrid with ECDH | ✅ **Recommended** for planning |
| **CRYSTALS-Dilithium** (FIPS 204) | Digital Signature | Code signing, document signing | ✅ **Recommended** for planning |
| **SPHINCS+** (FIPS 205) | Stateless hash-based signature | Long-term archival signatures | ✅ **Allowed** |

**Hybrid Cryptography** (Classical + PQC):
```
# Example: TLS 1.3 with hybrid key exchange
# Key = ECDH_P384_shared_secret || Kyber768_shared_secret

Client Hello:
  Supported Groups: x25519_kyber768, P-384, kyber768

Server Hello:
  Selected Group: x25519_kyber768 (hybrid)

Key Derivation:
  shared_secret = X25519_KDF(ecdh_share) || Kyber768_Encaps(kyber_share)
  master_secret = HKDF(shared_secret)
```

**Implementation Readiness**:
- **OpenSSL 3.2+**: Experimental PQC support via OQS provider (Open Quantum Safe)
- **Cloudflare TLS**: Hybrid PQC in production since 2023 (X25519Kyber768)
- **Chrome/Firefox**: Hybrid PQC TLS enabled by default in 2024

**ANSSI Recommendation for French Government**:
- **2025-2028**: Begin PQC pilot projects (test hybrid TLS, PQC code signing)
- **2028-2030**: Mandate hybrid PQC for new DR/NP systems
- **2030-2035**: Full migration to PQC (deprecate classical RSA/ECC)

---

## 8. Compliance Testing and Validation

### 8.1 Automated Cryptographic Testing

**Tools for Validating ANSSI Compliance**:

| Tool | Purpose | Usage |
|------|---------|-------|
| **testssl.sh** | TLS configuration scanner | `./testssl.sh --severity MEDIUM https://example.gouv.fr` |
| **Mozilla Observatory** | Web security scanner (includes TLS) | https://observatory.mozilla.org |
| **Nmap with ssl-enum-ciphers** | Enumerate TLS cipher suites | `nmap --script ssl-enum-ciphers -p 443 example.gouv.fr` |
| **ANSSI Crypto Checker** | Validate cryptographic algorithms | (Internal ANSSI tool for government use) |

**Example (testssl.sh Output)**:
```bash
./testssl.sh https://example.gouv.fr

# Expected output (ANSSI-compliant):
✅ TLS 1.3: Yes
✅ TLS 1.2: Yes
❌ TLS 1.1: No (disabled)
❌ TLS 1.0: No (disabled)
✅ Forward Secrecy: Yes (ECDHE)
✅ Cipher Suites: TLS_AES_256_GCM_SHA384 (RECOMMENDED)
```

### 8.2 Penetration Testing Requirements

**ANSSI Penetration Testing Scope** (for RGS certification):

- [ ] **Cryptographic Protocol Testing**: Test for downgrade attacks (POODLE, BEAST, CRIME)
- [ ] **Certificate Validation**: Verify certificate chain, revocation checking (OCSP/CRL)
- [ ] **Key Management**: Attempt to extract keys from memory, logs, backups
- [ ] **Random Number Generation**: Test entropy sources (NIST Statistical Test Suite)
- [ ] **Side-Channel Attacks**: Timing attacks on RSA/ECDSA, cache-timing attacks on AES

**Recommended Penetration Testing Firms** (ANSSI-qualified):
- **Excellium Services** (Paris)
- **Devoteam Secure**
- **Wavestone** (Cybersecurity division)

---

## 9. Incident Response for Cryptographic Failures

### 9.1 Compromise Scenarios

| Scenario | Impact | Immediate Response | Long-Term Mitigation |
|----------|--------|-------------------|---------------------|
| **Private Key Compromise** (TLS cert) | ⚠️ High - MITM attacks possible | 1. Revoke certificate (OCSP/CRL)<br>2. Generate new key pair<br>3. Re-issue certificate<br>4. Update servers (< 4 hours) | Migrate to HSM-stored keys |
| **Algorithm Weakness Discovered** (e.g., SHA-256 collision) | ⚠️ Medium-High - Depends on usage | 1. Assess exposure (code signatures, TLS, etc.)<br>2. Begin migration to stronger algorithm<br>3. Notify stakeholders | Implement crypto-agility (parameterize algorithms) |
| **Quantum Computer Breakthrough** | 🔴 Critical - All RSA/ECC broken | 1. Activate PQC contingency plan<br>2. Deploy hybrid PQC immediately<br>3. Re-encrypt archived data | Maintain PQC readiness |

### 9.2 Crypto-Agility Architecture

**Design for Algorithm Substitution**:

```python
# Bad: Hardcoded algorithm
def encrypt_data(data):
    cipher = AES.new(KEY, AES.MODE_GCM)  # Hardcoded AES-GCM
    return cipher.encrypt(data)

# Good: Parameterized algorithm (crypto-agility)
CRYPTO_CONFIG = {
    "algorithm": "AES-256-GCM",
    "key_size": 256,
    "hash": "SHA-384"
}

def encrypt_data(data, config=CRYPTO_CONFIG):
    if config["algorithm"] == "AES-256-GCM":
        cipher = AES.new(KEY, AES.MODE_GCM)
    elif config["algorithm"] == "ChaCha20-Poly1305":
        cipher = ChaCha20_Poly1305.new(key=KEY)
    # Easy to add new algorithms (e.g., post-quantum)
    return cipher.encrypt(data)
```

**Version Encrypted Data**:
```json
{
  "version": 1,
  "algorithm": "AES-256-GCM",
  "ciphertext": "base64_encrypted_data",
  "nonce": "base64_nonce",
  "tag": "base64_auth_tag"
}
```
- When migrating algorithms, increment version
- Old data can be bulk re-encrypted or decrypted on-the-fly

---

## 10. Compliance Checklist

### 10.1 Pre-Production Validation

**Data-at-Rest**:
- [ ] Database uses **TDE** or field-level encryption (AES-256-GCM)
- [ ] Disk encryption enabled on all servers (LUKS/BitLocker with AES-256-XTS)
- [ ] Master keys stored in **HSM** or **Cloud KMS** (not on application server)
- [ ] Key rotation automated (DEKs rotated every 30 days for DR/NP data)

**Data-in-Transit**:
- [ ] TLS 1.2+ enforced (TLS 1.0/1.1 disabled)
- [ ] Cipher suites limited to **ECDHE-RSA-AES256-GCM-SHA384** or stronger
- [ ] Perfect Forward Secrecy (PFS) enabled
- [ ] Certificates from **ANSSI-approved CA** (or documented exception for Let's Encrypt)
- [ ] OCSP stapling enabled (certificate revocation checking)

**Password Hashing**:
- [ ] **Argon2id** or bcrypt used (not plain SHA-256)
- [ ] Iteration count: ≥100,000 (PBKDF2) or time_cost=3 (Argon2id)
- [ ] Unique salt per password (≥128-bit)

**Code Signing**:
- [ ] All production software signed with **RSA-4096-SHA384** or **ECDSA-P384**
- [ ] SBOM (Software Bill of Materials) generated and signed
- [ ] Code signing certificate stored in **HSM**

**Random Number Generation**:
- [ ] Use **CSRNG** (`/dev/urandom`, `secrets` module, `CryptGenRandom`)
- [ ] Entropy validated with NIST Statistical Test Suite

**Post-Quantum Readiness** (for systems with >10-year lifespan):
- [ ] Cryptographic algorithms parameterized (crypto-agility)
- [ ] PQC migration plan documented (target: hybrid PQC by 2028)
- [ ] Long-term archived data re-encrypted with PQC-safe KEKs

### 10.2 Annual Compliance Review

- [ ] **Algorithm Review**: Verify no deprecated algorithms in use (check ANSSI v2.08 updates)
- [ ] **TLS Configuration Scan**: Run `testssl.sh` on all public endpoints
- [ ] **Key Rotation Audit**: Verify all keys rotated per policy (TLS certs, API keys, DEKs)
- [ ] **Penetration Test**: Annual test by ANSSI-qualified firm
- [ ] **Incident Review**: Analyze any cryptographic incidents (key compromise, algorithm weakness)

---

## 11. Constitutional Principles Alignment

| Principle | How This Template Supports It |
|-----------|------------------------------|
| **III - Security by Design** | Cryptography is embedded in architecture from design phase (TLS, database encryption, code signing); default-secure configurations (TLS 1.3, AES-256-GCM) |
| **IV - Defense in Depth** | Multiple cryptographic layers: TLS (transport), TDE (database), LUKS (filesystem), field-level encryption (application) |
| **II - Risk Analysis** | Algorithm selection based on data classification (DCP/DR/NP levels); threat modeling informs PQC migration timeline |

**Cross-References**:
- Integrate with **Secure Development Guidelines**: Mandate cryptographic library usage (no custom crypto implementations)
- Integrate with **Key Management Policy**: Define key lifecycle (generation, storage, rotation, destruction)
- Integrate with **Incident Response Plan**: Define procedures for key compromise, algorithm deprecation

---

## Version Control

**Template Version**: 1.0.0
**Last Updated**: 2025-11-19
**Next Review Date**: 2026-11-19 (or upon ANSSI guidance update)
**Owner**: Chief Information Security Officer (CISO)
**Approved By**: ANSSI Qualified Security Auditor

**Change Log**:
- **v1.0.0** (2025-11-19): Initial template based on ANSSI Mécanismes cryptographiques v2.08 (2024), RGS v2.0 Annexe B4, ANSSI TLS recommendations v1.3 (2024)

**References**:
- **RGS v2.0**: https://cyber.gouv.fr/le-referentiel-general-de-securite-rgs
- **Guides et publications ANSSI**: https://cyber.gouv.fr/publications
- **NIST Post-Quantum Cryptography**: https://csrc.nist.gov/projects/post-quantum-cryptography
- **eIDAS Regulation**: https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=uriserv:OJ.L_.2014.257.01.0073.01.ENG
