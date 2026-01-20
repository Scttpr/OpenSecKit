# RGS Cryptographic Validation Reference

Based on **RGS v2.0 Annexe B1** - Mécanismes cryptographiques (Version 2.03, February 2014)

## Quick Reference Table

### Symmetric Encryption

| Algorithm | Key Size | Block Size | Status | Notes |
|-----------|----------|------------|--------|-------|
| **AES-128** | 128 bits | 128 bits | **Compliant** | Recommended |
| **AES-192** | 192 bits | 128 bits | **Compliant** | Recommended |
| **AES-256** | 256 bits | 128 bits | **Compliant** | Recommended |
| Triple DES (3 keys) | 168 bits | 64 bits | Compliant until 2020 | Block size too small for long-term |
| Triple DES (2 keys) | 112 bits | 64 bits | **Not recommended** | Vulnerable to known-plaintext attacks |
| DES | 56 bits | 64 bits | **Non-compliant** | Key too small |

**Minimum Requirements:**
- Key size: 128 bits (mandatory after 2020)
- Block size: 128 bits (mandatory after 2020)

### Encryption Modes

| Mode | Status | Notes |
|------|--------|-------|
| **CBC** | **Compliant** | With random IV per message |
| **GCM** | **Compliant** | Authenticated encryption, recommended |
| **CTR** | Compliant | With unique nonce |
| **CCM** | Compliant | Authenticated encryption |
| ECB | **Non-compliant** | No semantic security |

### Hash Functions

| Algorithm | Output Size | Status | Notes |
|-----------|-------------|--------|-------|
| **SHA-256** | 256 bits | **Compliant** | Recommended |
| **SHA-384** | 384 bits | **Compliant** | Recommended |
| **SHA-512** | 512 bits | **Compliant** | Recommended |
| SHA-224 | 224 bits | Compliant | Minimum for post-2020 |
| SHA-1 | 160 bits | **Non-compliant** | Collision attacks known (2^63) |
| MD5 | 128 bits | **Non-compliant** | Broken |

**Minimum Requirements:**
- Output size: 256 bits (mandatory after 2020)
- No known collision attacks below 2^(h/2) operations

### Message Authentication (MAC)

| Algorithm | Status | Notes |
|-----------|--------|-------|
| **HMAC-SHA-256** | **Compliant** | Recommended |
| **HMAC-SHA-384** | **Compliant** | Recommended |
| **HMAC-SHA-512** | **Compliant** | Recommended |
| CBC-MAC (AES, retail) | Compliant | With two distinct keys, no IV |
| HMAC-SHA-1 | **Non-compliant** | SHA-1 not compliant |
| CBC-MAC (DES) | **Non-compliant** | DES not compliant |

### Asymmetric Encryption

| Algorithm | Key Size | Status | Notes |
|-----------|----------|--------|-------|
| **RSA-OAEP** | 2048 bits | **Compliant** | Until 2030 |
| **RSA-OAEP** | 3072 bits | **Compliant** | Recommended, required after 2030 |
| **RSA-OAEP** | 4096 bits | **Compliant** | Best long-term security |
| RSA PKCS#1 v1.5 | Any | **Non-compliant** | Padding oracle attacks |
| RSA | 1024 bits | **Non-compliant** | Key too small |

**RSA Requirements:**
- Modulus: ≥2048 bits (≥3072 recommended)
- Public exponent: >65536
- Private exponent: same size as modulus

### Digital Signatures

| Algorithm | Parameters | Status | Notes |
|-----------|------------|--------|-------|
| **RSA-PSS** | 2048+ bits | **Compliant** | Recommended |
| **RSA-PSS** | 3072+ bits | **Compliant** | Required after 2030 |
| **ECDSA (P-256)** | 256-bit curve | **Compliant** | NIST curve |
| **ECDSA (P-384)** | 384-bit curve | **Compliant** | NIST curve |
| **ECDSA (P-521)** | 521-bit curve | **Compliant** | NIST curve |
| **ECDSA (FRP256v1)** | 256-bit curve | **Compliant** | French national curve |
| RSA PKCS#1 v1.5 | Any | **Non-compliant** | Signature forgery possible |
| ECDSA (secp256k1) | 256-bit curve | Not listed | Bitcoin curve, verify |

### Elliptic Curves

| Curve | Field | Order | Status | Notes |
|-------|-------|-------|--------|-------|
| **P-256 (secp256r1)** | GF(p) | 256 bits | **Compliant** | NIST, widely supported |
| **P-384 (secp384r1)** | GF(p) | 384 bits | **Compliant** | NIST |
| **P-521 (secp521r1)** | GF(p) | 521 bits | **Compliant** | NIST |
| **FRP256v1** | GF(p) | 256 bits | **Compliant** | ANSSI French curve |
| B-283 | GF(2^n) | 283 bits | Compliant | Binary curve |
| B-409 | GF(2^n) | 409 bits | Compliant | Binary curve |
| B-571 | GF(2^n) | 571 bits | Compliant | Binary curve |
| Curve25519 | GF(p) | 256 bits | Not listed | Modern, verify |

**Minimum Requirements:**
- Prime order subgroup: ≥256 bits (mandatory after 2020)

### Key Exchange

| Protocol | Parameters | Status | Notes |
|----------|------------|--------|-------|
| **ECDH (P-256+)** | 256+ bit curves | **Compliant** | Recommended |
| **DH** | 2048+ bit modulus | **Compliant** | Use safe primes |
| **DH** | 3072+ bit modulus | **Compliant** | Recommended after 2030 |
| DH | 1024 bit modulus | **Non-compliant** | Modulus too small |

**Perfect Forward Secrecy (PFS):** Recommended for key establishment

### TLS Configuration

| Version | Status | Cipher Suites |
|---------|--------|---------------|
| **TLS 1.3** | **Compliant** | All default suites |
| **TLS 1.2** | **Compliant** | With compliant cipher suites only |
| TLS 1.1 | **Non-compliant** | Deprecated |
| TLS 1.0 | **Non-compliant** | Deprecated |
| SSL 3.0 | **Non-compliant** | Broken (POODLE) |

**Compliant TLS 1.2 Cipher Suites:**
```
TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384
TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256
TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384
TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256
TLS_DHE_RSA_WITH_AES_256_GCM_SHA384
TLS_DHE_RSA_WITH_AES_128_GCM_SHA256
```

**Non-compliant cipher suites (avoid):**
- Any with RC4, DES, 3DES
- Any with MD5, SHA-1 for signature
- Any with RSA key exchange (no PFS)
- Any with NULL encryption
- Any with EXPORT ciphers

### Password Hashing

| Algorithm | Status | Parameters |
|-----------|--------|------------|
| **Argon2id** | **Recommended** | memory=64MB, iterations=3, parallelism=4 |
| **bcrypt** | **Compliant** | cost≥12 |
| **scrypt** | **Compliant** | N=2^14, r=8, p=1 minimum |
| **PBKDF2-SHA256** | Compliant | iterations≥100,000 |
| PBKDF2-SHA1 | **Non-compliant** | SHA-1 not compliant |
| MD5 | **Non-compliant** | Not a KDF, broken |
| SHA-256 (plain) | **Non-compliant** | No salt, no iterations |

## Validation Logic

### Auto-Detection Patterns

```yaml
# Patterns to search in codebase for crypto validation

compliant_patterns:
  encryption:
    - "aes-256-gcm"
    - "aes-128-gcm"
    - "AES/GCM"
    - "crypto.createCipheriv('aes"

  hashing:
    - "sha256"
    - "sha384"
    - "sha512"
    - "SHA-256"
    - "crypto.createHash('sha256"

  password_hashing:
    - "bcrypt"
    - "argon2"
    - "scrypt"
    - "pbkdf2.*sha256"

  tls:
    - "TLSv1.3"
    - "TLSv1.2"
    - "minVersion.*TLSv1.2"

non_compliant_patterns:
  encryption:
    - "des-"
    - "rc4"
    - "DES/"
    - "RC4"
    - "aes-.*-ecb"

  hashing:
    - "md5"
    - "sha1"
    - "MD5"
    - "SHA-1"
    - "createHash('md5"
    - "createHash('sha1"

  password_hashing:
    - "md5("
    - "sha1("
    - "hashpw.*md5"

  tls:
    - "TLSv1.0"
    - "TLSv1.1"
    - "SSLv3"
    - "ssl3"
```

### Assessment Scoring

| Finding | Impact on Score |
|---------|-----------------|
| Non-compliant algorithm in use | -20% per instance |
| SHA-1 for signatures | -15% |
| RSA < 2048 bits | -20% |
| No PFS in TLS | -10% |
| Weak password hashing | -15% |
| Compliant with recommendations | +5% bonus |

## RGS Level Requirements

| Requirement | `RGS*` | `RGS**` | `RGS***` |
|-------------|--------|---------|----------|
| AES-128+ | Required | Required | Required |
| SHA-256+ | Required | Required | Required |
| RSA 2048+ | Required | Required | RSA 3072+ |
| TLS 1.2+ | Required | TLS 1.2+ with PFS | TLS 1.3 preferred |
| Qualified certificates | - | Recommended | Required |
| HSM for key storage | - | Recommended | Required |
| ANSSI-qualified products | - | Recommended | Required |

## References

- [RGS v2.0 Annexe B1](https://cyber.gouv.fr/le-referentiel-general-de-securite-rgs) - Full cryptographic rules
- [ANSSI Crypto Guide](https://cyber.gouv.fr/publications/mecanismes-cryptographiques) - Updated recommendations
- [FRP256v1 Parameters](https://www.legifrance.gouv.fr/jorf/id/JORFTEXT000024668816) - French elliptic curve

---

*Note: This reference is based on RGS v2.0 Annexe B1 (2014). ANSSI publishes periodic updates to cryptographic recommendations. Always verify against the latest ANSSI guidance for production systems.*
