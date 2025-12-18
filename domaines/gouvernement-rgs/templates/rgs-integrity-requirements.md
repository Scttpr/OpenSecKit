---
title: "Exigences d'Intégrité RGS (Fonction 2)"
template_version: "1.0.0"
domain: "government-rgs"
constitutional_principles:
  - "III - Security by Design"
  - "V - Secrets Management"
regulatory_references:
  - "RGS v2.0 - Annexe B3 (Intégrité)"
  - "eIDAS Regulation (EU) 910/2014 - Signatures électroniques qualifiées"
  - "ANSSI - Prestataires de services de confiance qualifiés"
  - "Code civil - Articles 1366-1368 (Preuve électronique)"
ssdlc_phase: "III - Design"
difficulty: "advanced"
estimated_time: "40-60 heures (implémentation initiale), 8-16 heures (revue annuelle)"
---

# Exigences d'Intégrité RGS (Fonction 2)

## Objectif

Ce template définit les **exigences d'intégrité obligatoires** pour les systèmes d'information soumis au RGS, conformément à l'**Annexe B3 du RGS v2.0** et au règlement **eIDAS**.

**Contexte Réglementaire** :
- **RGS v2.0 Annexe B3** : L'intégrité garantit que les données n'ont pas été altérées de manière non autorisée
- **eIDAS (UE) 910/2014** : Cadre européen pour les signatures électroniques et services de confiance
- **Code civil Art. 1366-1368** : Valeur juridique de la signature électronique

**Alignement Constitutionnel** :
- **Principe III (Security by Design)** : L'intégrité est un contrôle fondamental dès la conception
- **Principe V (Secrets Management)** : Protection des clés de signature

**Applicabilité** :
- ✅ Documents administratifs officiels (arrêtés, décisions, attestations)
- ✅ Contrats dématérialisés avec l'administration
- ✅ Données de santé (ordonnances, compte-rendus)
- ✅ Transactions financières (factures, mandats)
- ✅ Fichiers de configuration et code source critique

---

## 1. Niveaux de Signature Électronique

### 1.1 Classification eIDAS / RGS

| Niveau | Nom | Valeur juridique | Cas d'usage | Exigence technique |
|--------|-----|-----------------|-------------|-------------------|
| **Simple** | Signature électronique simple | Présomption de fiabilité (Art. 1367 C.civ) | Validation interne, logs | Case à cocher, email |
| **Avancée** | Signature électronique avancée (SEA) | Présomption renforcée | Contrats B2B, documents internes | Certificat X.509, clé privée |
| **Qualifiée** | Signature électronique qualifiée (SEQ) | Équivalente manuscrite (Art. 1367 al.2) | Actes authentiques, marchés publics | Certificat qualifié + QSCD |

### 1.2 Correspondance RGS

| Niveau RGS | Signature requise | Cas d'usage |
|------------|-------------------|-------------|
| **RGS*** | SEA minimum | Documents administratifs courants |
| **RGS**** | SEQ recommandée | Données sensibles, actes engageant l'État |
| **RGS***** | SEQ obligatoire | Données classifiées, marchés publics > seuil européen |

### 1.3 Exigences par Type de Signature

#### Signature Électronique Avancée (SEA)

**Exigences (eIDAS Art. 26)** :
- [ ] Liée au signataire de manière univoque
- [ ] Permet d'identifier le signataire
- [ ] Créée avec des données sous contrôle exclusif du signataire
- [ ] Liée aux données signées de telle sorte que toute modification soit détectable

**Implémentation** :
```
Certificat X.509 v3
├── Clé privée stockée de manière sécurisée (fichier chiffré, HSM logiciel)
├── Algorithme : RSA-2048+ ou ECDSA P-256+
├── Hash : SHA-256 minimum
└── Format : PAdES, XAdES, CAdES, JAdES
```

#### Signature Électronique Qualifiée (SEQ)

**Exigences supplémentaires (eIDAS Art. 28-29)** :
- [ ] Certificat qualifié délivré par un PSCE qualifié (liste de confiance eIDAS)
- [ ] Créée avec un QSCD (Qualified Signature Creation Device)
- [ ] QSCD = HSM certifié (Common Criteria EAL4+ ou FIPS 140-2 Level 3+)

**Implémentation** :
```
Certificat qualifié eIDAS
├── Délivré par PSCE qualifié (Certinomis, ChamberSign, Universign, etc.)
├── Clé privée dans QSCD (carte à puce, HSM certifié)
├── Algorithme : RSA-4096 ou ECDSA P-384
├── Hash : SHA-384 ou SHA-512
└── Format : PAdES-B-LTA, XAdES-B-LTA (Long-Term Archive)
```

---

## 2. Prestataires de Services de Confiance Qualifiés (PSCE)

### 2.1 Liste des PSCE Qualifiés en France

| Prestataire | Services qualifiés | URL | Certification |
|-------------|-------------------|-----|---------------|
| **Certinomis** (La Poste) | Signature, Cachet, Horodatage | https://www.certinomis.fr | RGS**, eIDAS |
| **ChamberSign France** | Signature, Cachet | https://www.chambersign.fr | RGS**, eIDAS |
| **Universign** (Signaturit) | Signature, Horodatage | https://www.universign.com | RGS**, eIDAS |
| **DocuSign France** | Signature | https://www.docusign.fr | eIDAS |
| **Yousign** | Signature | https://yousign.com | eIDAS |
| **Certigna** (Dhimyotis) | Signature, Cachet | https://www.certigna.fr | RGS**, eIDAS |

**Liste officielle** : https://webgate.ec.europa.eu/tl-browser/ (EU Trusted List)

### 2.2 Services de Confiance Qualifiés

| Service | Description | Cas d'usage RGS |
|---------|-------------|-----------------|
| **Signature qualifiée** | Signature avec certificat qualifié + QSCD | Documents engageant l'administration |
| **Cachet qualifié** | Signature d'organisation (personne morale) | Factures, attestations automatisées |
| **Horodatage qualifié** | Preuve de date certaine | Scellement de documents, logs juridiques |
| **Validation qualifiée** | Vérification de signatures | Portails de vérification |
| **Conservation qualifiée** | Archivage à valeur probante | Coffre-fort numérique |

---

## 3. Formats de Signature Standardisés

### 3.1 Formats ETSI

| Format | Standard | Documents | Recommandation |
|--------|----------|-----------|----------------|
| **PAdES** | ETSI EN 319 142 | PDF | ✅ **Recommandé** pour documents administratifs |
| **XAdES** | ETSI EN 319 132 | XML | ✅ Flux de données, échanges inter-administrations |
| **CAdES** | ETSI EN 319 122 | Binaire (CMS) | ⚠️ Fichiers arbitraires |
| **JAdES** | ETSI TS 119 182 | JSON | ✅ APIs REST, données structurées |

### 3.2 Niveaux de Signature (Baseline Profiles)

| Niveau | Suffixe | Contenu | Durée de validité |
|--------|---------|---------|-------------------|
| **B** | -B | Signature + certificat | Durée du certificat (~1-3 ans) |
| **T** | -B-T | + Horodatage | Durée du certificat TSA |
| **LT** | -B-LT | + Données de validation (CRL, OCSP) | Long terme (sans dépendance réseau) |
| **LTA** | -B-LTA | + Archive timestamp (renouvellement) | **Illimité** (avec ré-horodatage périodique) |

**Recommandation RGS** : Utiliser **PAdES-B-LTA** ou **XAdES-B-LTA** pour les documents à conserver >5 ans.

### 3.3 Exemple de Signature PAdES (Python)

```python
from endesive.pdf import cms
from cryptography.hazmat.primitives.serialization import pkcs12
from datetime import datetime
import hashlib

def sign_pdf_pades(
    pdf_path: str,
    output_path: str,
    p12_path: str,
    p12_password: str,
    tsa_url: str = "https://timestamp.certinomis.com"
) -> str:
    """Signe un PDF au format PAdES-B-T"""

    # Charger le certificat et la clé privée
    with open(p12_path, 'rb') as f:
        p12_data = f.read()

    private_key, certificate, chain = pkcs12.load_key_and_certificates(
        p12_data,
        p12_password.encode()
    )

    # Charger le PDF
    with open(pdf_path, 'rb') as f:
        pdf_data = f.read()

    # Paramètres de signature
    dct = {
        'sigflags': 3,  # Certify signature
        'contact': 'admin@example.gouv.fr',
        'location': 'Paris, France',
        'signingdate': datetime.utcnow().strftime('%Y%m%d%H%M%S+00\'00\''),
        'reason': 'Document officiel - Administration française',
        'signature': 'Signature Electronique Avancee',
        'signaturebox': (0, 0, 0, 0),  # Invisible signature
    }

    # Créer la signature avec horodatage
    signed_pdf = cms.sign(
        pdf_data,
        dct,
        private_key,
        certificate,
        chain,
        'sha384',  # Hash algorithm
        tsa_url=tsa_url,  # Horodatage qualifié
        tsa_username=None,
        tsa_password=None
    )

    # Sauvegarder le PDF signé
    with open(output_path, 'wb') as f:
        f.write(pdf_data)
        f.write(signed_pdf)

    return output_path

# Usage
sign_pdf_pades(
    pdf_path="arrete_2025-001.pdf",
    output_path="arrete_2025-001_signed.pdf",
    p12_path="certificat_administration.p12",
    p12_password="SecurePassword123!",
    tsa_url="https://timestamp.certinomis.com"
)
```

### 3.4 Exemple de Signature XAdES (XML)

```python
from signxml import XMLSigner, XMLVerifier
from lxml import etree
from datetime import datetime

def sign_xml_xades(
    xml_path: str,
    output_path: str,
    cert_path: str,
    key_path: str
) -> str:
    """Signe un XML au format XAdES-B-T"""

    # Charger le XML
    with open(xml_path, 'rb') as f:
        xml_doc = etree.parse(f)

    # Charger le certificat et la clé
    with open(cert_path, 'rb') as f:
        cert = f.read()
    with open(key_path, 'rb') as f:
        key = f.read()

    # Créer le signer XAdES
    signer = XMLSigner(
        method=signxml.methods.enveloped,
        signature_algorithm="rsa-sha384",
        digest_algorithm="sha384",
        c14n_algorithm="http://www.w3.org/2001/10/xml-exc-c14n#"
    )

    # Ajouter les propriétés signées XAdES
    signed_properties = {
        'SigningTime': datetime.utcnow().isoformat() + 'Z',
        'SigningCertificate': {
            'CertDigest': {
                'DigestMethod': 'SHA-384',
                'DigestValue': hashlib.sha384(cert).hexdigest()
            }
        },
        'SignaturePolicyIdentifier': {
            'SignaturePolicyId': {
                'SigPolicyId': 'urn:oid:1.2.250.1.131.1.1',  # OID politique RGS
                'SigPolicyQualifiers': 'Politique de signature RGS'
            }
        }
    }

    # Signer
    signed_root = signer.sign(
        xml_doc.getroot(),
        key=key,
        cert=cert
    )

    # Sauvegarder
    with open(output_path, 'wb') as f:
        f.write(etree.tostring(signed_root, encoding='UTF-8', xml_declaration=True))

    return output_path
```

---

## 4. Contrôles d'Intégrité des Données

### 4.1 Hachage Cryptographique

**Algorithmes approuvés ANSSI** :

| Algorithme | Taille sortie | Statut ANSSI | Cas d'usage |
|------------|---------------|--------------|-------------|
| **SHA-256** | 256 bits | ✅ Recommandé | Intégrité fichiers, checksums |
| **SHA-384** | 384 bits | ✅ Recommandé | Signatures RGS** |
| **SHA-512** | 512 bits | ✅ Recommandé | Données haute sensibilité |
| **SHA-3** | Variable | ✅ Autorisé | Alternative à SHA-2 |
| **BLAKE2** | Variable | ✅ Autorisé | Haute performance |
| MD5 | 128 bits | ❌ **Interdit** | Collisions connues |
| SHA-1 | 160 bits | ❌ **Interdit** | Collisions pratiques (SHAttered) |

### 4.2 Implémentation du Contrôle d'Intégrité

```python
import hashlib
import json
from datetime import datetime
from pathlib import Path

class IntegrityChecker:
    """Contrôleur d'intégrité conforme RGS"""

    def __init__(self, algorithm: str = 'sha384'):
        self.algorithm = algorithm

    def compute_hash(self, data: bytes) -> str:
        """Calcule le hash d'un contenu"""
        hasher = hashlib.new(self.algorithm)
        hasher.update(data)
        return hasher.hexdigest()

    def compute_file_hash(self, file_path: str) -> dict:
        """Calcule le hash d'un fichier avec métadonnées"""
        path = Path(file_path)

        with open(path, 'rb') as f:
            content = f.read()

        return {
            'file_path': str(path.absolute()),
            'file_name': path.name,
            'file_size': path.stat().st_size,
            'algorithm': self.algorithm,
            'hash': self.compute_hash(content),
            'computed_at': datetime.utcnow().isoformat() + 'Z'
        }

    def verify_file_integrity(self, file_path: str, expected_hash: str) -> dict:
        """Vérifie l'intégrité d'un fichier"""
        current = self.compute_file_hash(file_path)

        return {
            'file_path': file_path,
            'expected_hash': expected_hash,
            'actual_hash': current['hash'],
            'integrity_valid': current['hash'] == expected_hash,
            'verified_at': datetime.utcnow().isoformat() + 'Z'
        }

    def create_manifest(self, directory: str, pattern: str = '*') -> dict:
        """Crée un manifeste d'intégrité pour un répertoire"""
        dir_path = Path(directory)
        files = list(dir_path.glob(pattern))

        manifest = {
            'version': '1.0',
            'algorithm': self.algorithm,
            'created_at': datetime.utcnow().isoformat() + 'Z',
            'directory': str(dir_path.absolute()),
            'file_count': len(files),
            'files': []
        }

        for file_path in files:
            if file_path.is_file():
                manifest['files'].append(self.compute_file_hash(str(file_path)))

        # Hash du manifeste lui-même (excluant ce champ)
        manifest_content = json.dumps(manifest, sort_keys=True)
        manifest['manifest_hash'] = self.compute_hash(manifest_content.encode())

        return manifest

    def verify_manifest(self, manifest: dict, directory: str = None) -> dict:
        """Vérifie un manifeste d'intégrité"""
        results = {
            'verified_at': datetime.utcnow().isoformat() + 'Z',
            'total_files': manifest['file_count'],
            'valid_files': 0,
            'invalid_files': 0,
            'missing_files': 0,
            'details': []
        }

        for file_info in manifest['files']:
            file_path = file_info['file_path']

            if not Path(file_path).exists():
                results['missing_files'] += 1
                results['details'].append({
                    'file': file_path,
                    'status': 'MISSING'
                })
                continue

            verification = self.verify_file_integrity(
                file_path,
                file_info['hash']
            )

            if verification['integrity_valid']:
                results['valid_files'] += 1
                results['details'].append({
                    'file': file_path,
                    'status': 'VALID'
                })
            else:
                results['invalid_files'] += 1
                results['details'].append({
                    'file': file_path,
                    'status': 'CORRUPTED',
                    'expected': file_info['hash'],
                    'actual': verification['actual_hash']
                })

        results['integrity_valid'] = (
            results['invalid_files'] == 0 and
            results['missing_files'] == 0
        )

        return results

# Usage
checker = IntegrityChecker(algorithm='sha384')

# Créer un manifeste
manifest = checker.create_manifest('/var/app/config', '*.yml')
with open('/var/app/config/INTEGRITY.json', 'w') as f:
    json.dump(manifest, f, indent=2)

# Vérifier périodiquement
with open('/var/app/config/INTEGRITY.json') as f:
    manifest = json.load(f)
result = checker.verify_manifest(manifest)
if not result['integrity_valid']:
    raise SecurityError(f"Integrity check failed: {result}")
```

### 4.3 Protection de l'Intégrité en Base de Données

**Mécanismes de protection** :

| Mécanisme | Description | Niveau de protection |
|-----------|-------------|---------------------|
| **Colonnes de hash** | Hash SHA-256 des champs critiques | ⭐⭐⭐ |
| **Triggers de vérification** | Vérification automatique à chaque modification | ⭐⭐⭐⭐ |
| **Audit tables** | Historique complet des modifications | ⭐⭐⭐⭐⭐ |
| **Row-level signature** | Signature cryptographique par ligne | ⭐⭐⭐⭐⭐ |

**Exemple PostgreSQL - Colonne de hash** :
```sql
-- Ajouter une colonne d'intégrité
ALTER TABLE documents ADD COLUMN integrity_hash VARCHAR(96);
ALTER TABLE documents ADD COLUMN integrity_algo VARCHAR(20) DEFAULT 'sha384';
ALTER TABLE documents ADD COLUMN integrity_computed_at TIMESTAMP;

-- Fonction de calcul du hash
CREATE OR REPLACE FUNCTION compute_document_hash(doc documents)
RETURNS VARCHAR(96) AS $$
BEGIN
    RETURN encode(
        digest(
            concat_ws('|',
                doc.id::text,
                doc.title,
                doc.content,
                doc.created_at::text,
                doc.author_id::text
            ),
            'sha384'
        ),
        'hex'
    );
END;
$$ LANGUAGE plpgsql IMMUTABLE;

-- Trigger pour mise à jour automatique
CREATE OR REPLACE FUNCTION update_integrity_hash()
RETURNS TRIGGER AS $$
BEGIN
    NEW.integrity_hash := compute_document_hash(NEW);
    NEW.integrity_computed_at := NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_document_integrity
    BEFORE INSERT OR UPDATE ON documents
    FOR EACH ROW
    EXECUTE FUNCTION update_integrity_hash();

-- Fonction de vérification
CREATE OR REPLACE FUNCTION verify_document_integrity(doc_id UUID)
RETURNS BOOLEAN AS $$
DECLARE
    doc documents;
    computed_hash VARCHAR(96);
BEGIN
    SELECT * INTO doc FROM documents WHERE id = doc_id;
    computed_hash := compute_document_hash(doc);
    RETURN computed_hash = doc.integrity_hash;
END;
$$ LANGUAGE plpgsql;

-- Vérification périodique (job planifié)
SELECT id, title,
       CASE WHEN verify_document_integrity(id)
            THEN 'VALID'
            ELSE 'CORRUPTED'
       END as integrity_status
FROM documents
WHERE integrity_computed_at < NOW() - INTERVAL '24 hours';
```

---

## 5. Horodatage Qualifié

### 5.1 Exigences RGS pour l'Horodatage

| Cas d'usage | Type d'horodatage | Exigence |
|-------------|-------------------|----------|
| Logs applicatifs courants | NTP synchronisé | ⚠️ Suffisant |
| Signatures de documents | Horodatage qualifié TSA | ✅ Obligatoire pour SEQ |
| Preuves juridiques | Horodatage qualifié + archivage | ✅ Obligatoire |
| Scellement de lots de logs | Horodatage qualifié périodique | ✅ Recommandé |

### 5.2 Autorités d'Horodatage Qualifiées (TSA)

| Prestataire | URL TSA | Protocole | Certification |
|-------------|---------|-----------|---------------|
| **Certinomis** | `https://timestamp.certinomis.com` | RFC 3161 | RGS**, eIDAS |
| **Universign** | `https://timestamp.universign.eu` | RFC 3161 | RGS**, eIDAS |
| **ChamberSign** | `https://timestamp.chambersign.fr` | RFC 3161 | RGS** |

### 5.3 Implémentation de l'Horodatage Qualifié

```python
import hashlib
import requests
from asn1crypto import tsp, core
import base64
import random

class QualifiedTimestamp:
    """Client d'horodatage qualifié RFC 3161"""

    def __init__(self, tsa_url: str):
        self.tsa_url = tsa_url

    def create_timestamp_request(self, data: bytes) -> bytes:
        """Crée une requête d'horodatage TSP"""

        # Hash du contenu
        digest = hashlib.sha384(data).digest()

        # Message imprint
        message_imprint = tsp.MessageImprint({
            'hash_algorithm': {
                'algorithm': 'sha384'
            },
            'hashed_message': digest
        })

        # Requête TSP
        ts_request = tsp.TimeStampReq({
            'version': 1,
            'message_imprint': message_imprint,
            'cert_req': True,  # Inclure certificat dans réponse
            'nonce': core.Integer(random.getrandbits(64))
        })

        return ts_request.dump()

    def get_timestamp(self, data: bytes) -> dict:
        """Obtient un jeton d'horodatage qualifié"""

        # Créer la requête
        request_data = self.create_timestamp_request(data)

        # Envoyer à la TSA
        response = requests.post(
            self.tsa_url,
            data=request_data,
            headers={
                'Content-Type': 'application/timestamp-query',
                'Accept': 'application/timestamp-reply'
            },
            timeout=30
        )

        if response.status_code != 200:
            raise Exception(f"TSA request failed: {response.status_code}")

        # Parser la réponse
        ts_response = tsp.TimeStampResp.load(response.content)

        # Vérifier le statut
        status = ts_response['status']['status'].native
        if status != 'granted':
            status_string = ts_response['status'].get('status_string')
            raise Exception(f"TSA rejected request: {status} - {status_string}")

        # Extraire le jeton
        token = ts_response['time_stamp_token']
        signed_data = token['content']

        # Extraire la date
        tst_info_raw = signed_data['encap_content_info']['content'].native
        tst_info = tsp.TSTInfo.load(tst_info_raw)

        return {
            'timestamp': tst_info['gen_time'].native.isoformat(),
            'serial_number': str(tst_info['serial_number'].native),
            'tsa_name': str(tst_info['tsa'].native) if tst_info['tsa'] else None,
            'hash_algorithm': 'sha384',
            'token_base64': base64.b64encode(response.content).decode(),
            'tsa_url': self.tsa_url
        }

    def verify_timestamp(self, data: bytes, token_base64: str) -> dict:
        """Vérifie un jeton d'horodatage"""

        token_data = base64.b64decode(token_base64)
        ts_response = tsp.TimeStampResp.load(token_data)
        token = ts_response['time_stamp_token']
        signed_data = token['content']

        # Extraire le hash du jeton
        tst_info_raw = signed_data['encap_content_info']['content'].native
        tst_info = tsp.TSTInfo.load(tst_info_raw)
        token_hash = tst_info['message_imprint']['hashed_message'].native

        # Calculer le hash des données actuelles
        data_hash = hashlib.sha384(data).digest()

        # Comparer
        hash_valid = token_hash == data_hash

        return {
            'hash_valid': hash_valid,
            'timestamp': tst_info['gen_time'].native.isoformat(),
            'serial_number': str(tst_info['serial_number'].native),
            'verification_time': datetime.utcnow().isoformat() + 'Z'
        }

# Usage
tsa = QualifiedTimestamp("https://timestamp.certinomis.com")

# Horodater un document
with open("document.pdf", "rb") as f:
    document_data = f.read()

timestamp_result = tsa.get_timestamp(document_data)
print(f"Document horodaté à : {timestamp_result['timestamp']}")

# Stocker le jeton avec le document
with open("document.pdf.tst", "w") as f:
    json.dump(timestamp_result, f, indent=2)

# Vérifier plus tard
verification = tsa.verify_timestamp(document_data, timestamp_result['token_base64'])
print(f"Intégrité vérifiée : {verification['hash_valid']}")
```

---

## 6. Validation des Données d'Entrée

### 6.1 Principes de Validation RGS

| Principe | Description | Implémentation |
|----------|-------------|----------------|
| **Validation systématique** | Toute entrée doit être validée | Schémas JSON, DTD/XSD |
| **Validation côté serveur** | Ne jamais faire confiance au client | Validation backend obligatoire |
| **Liste blanche** | Définir ce qui est autorisé (pas interdit) | Regex positives, enums |
| **Rejet par défaut** | Rejeter tout ce qui n'est pas explicitement valide | Fail-secure |

### 6.2 Schémas de Validation

**JSON Schema (exemple pour données citoyen)** :
```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://example.gouv.fr/schemas/citoyen.json",
  "title": "Données Citoyen RGS",
  "description": "Schéma de validation des données citoyen",
  "type": "object",
  "required": ["nom", "prenom", "date_naissance", "email"],
  "properties": {
    "nom": {
      "type": "string",
      "minLength": 1,
      "maxLength": 100,
      "pattern": "^[A-Za-zÀ-ÿ\\s'-]+$",
      "description": "Nom de famille"
    },
    "prenom": {
      "type": "string",
      "minLength": 1,
      "maxLength": 100,
      "pattern": "^[A-Za-zÀ-ÿ\\s'-]+$",
      "description": "Prénom"
    },
    "date_naissance": {
      "type": "string",
      "format": "date",
      "description": "Date de naissance (ISO 8601)"
    },
    "email": {
      "type": "string",
      "format": "email",
      "maxLength": 254,
      "description": "Adresse email"
    },
    "telephone": {
      "type": "string",
      "pattern": "^\\+?[0-9]{10,15}$",
      "description": "Numéro de téléphone"
    },
    "nir": {
      "type": "string",
      "pattern": "^[12][0-9]{12}[0-9]{2}$",
      "description": "Numéro de sécurité sociale (NIR)"
    },
    "adresse": {
      "$ref": "#/$defs/adresse"
    }
  },
  "$defs": {
    "adresse": {
      "type": "object",
      "required": ["ligne1", "code_postal", "ville"],
      "properties": {
        "ligne1": {
          "type": "string",
          "maxLength": 200
        },
        "ligne2": {
          "type": "string",
          "maxLength": 200
        },
        "code_postal": {
          "type": "string",
          "pattern": "^[0-9]{5}$"
        },
        "ville": {
          "type": "string",
          "maxLength": 100
        },
        "pays": {
          "type": "string",
          "default": "FR",
          "pattern": "^[A-Z]{2}$"
        }
      }
    }
  },
  "additionalProperties": false
}
```

**Implémentation Python (Pydantic)** :
```python
from pydantic import BaseModel, Field, EmailStr, validator
from datetime import date
import re

class AdresseRGS(BaseModel):
    """Modèle d'adresse conforme RGS"""
    ligne1: str = Field(..., max_length=200)
    ligne2: str | None = Field(None, max_length=200)
    code_postal: str = Field(..., pattern=r'^[0-9]{5}$')
    ville: str = Field(..., max_length=100)
    pays: str = Field(default='FR', pattern=r'^[A-Z]{2}$')

class CitoyenRGS(BaseModel):
    """Modèle de données citoyen conforme RGS"""
    nom: str = Field(..., min_length=1, max_length=100)
    prenom: str = Field(..., min_length=1, max_length=100)
    date_naissance: date
    email: EmailStr
    telephone: str | None = Field(None, pattern=r'^\+?[0-9]{10,15}$')
    nir: str | None = Field(None, pattern=r'^[12][0-9]{12}[0-9]{2}$')
    adresse: AdresseRGS | None = None

    @validator('nom', 'prenom')
    def validate_name(cls, v):
        """Valide les caractères autorisés dans les noms"""
        if not re.match(r"^[A-Za-zÀ-ÿ\s'-]+$", v):
            raise ValueError('Caractères non autorisés dans le nom')
        return v.strip()

    @validator('date_naissance')
    def validate_birthdate(cls, v):
        """Valide la date de naissance"""
        if v > date.today():
            raise ValueError('Date de naissance dans le futur')
        if v < date(1900, 1, 1):
            raise ValueError('Date de naissance invalide')
        return v

    @validator('nir')
    def validate_nir(cls, v):
        """Valide le NIR avec clé de contrôle"""
        if v is None:
            return v

        # Extraire numéro et clé
        numero = int(v[:13].replace('2A', '19').replace('2B', '18'))
        cle = int(v[13:])

        # Vérifier la clé (97 - (numéro mod 97))
        cle_calculee = 97 - (numero % 97)
        if cle != cle_calculee:
            raise ValueError('Clé NIR invalide')

        return v

    class Config:
        # Rejeter les champs supplémentaires
        extra = 'forbid'
        # Valider à l'assignation
        validate_assignment = True

# Usage
try:
    citoyen = CitoyenRGS(
        nom="Dupont",
        prenom="Jean",
        date_naissance="1985-03-15",
        email="jean.dupont@example.fr",
        nir="185037512312345"  # Sera validé avec clé
    )
except ValidationError as e:
    print(f"Données invalides: {e}")
```

---

## 7. Protection du Code Source

### 7.1 Intégrité du Code Déployé

| Mécanisme | Description | Niveau RGS |
|-----------|-------------|-----------|
| **Git signed commits** | Signature GPG des commits | RGS* |
| **Git signed tags** | Signature des releases | RGS** |
| **Container image signing** | Signature des images Docker (Cosign) | RGS** |
| **SBOM + signatures** | Bill of Materials signé | RGS** |
| **Reproducible builds** | Build déterministe vérifiable | RGS*** |

### 7.2 Signature des Commits Git

```bash
# Configuration GPG pour commits signés
git config --global user.signingkey ABCD1234EFGH5678
git config --global commit.gpgsign true
git config --global tag.gpgsign true

# Créer un commit signé
git commit -S -m "feat: ajout authentification FranceConnect"

# Vérifier les signatures
git log --show-signature

# Politique de branche (GitHub/GitLab)
# Exiger signatures sur main/master
```

### 7.3 Signature des Images Docker

```bash
# Installer Cosign
brew install cosign  # ou apt-get install cosign

# Générer une paire de clés
cosign generate-key-pair

# Signer une image
cosign sign --key cosign.key registry.example.gouv.fr/app:v1.0.0

# Vérifier une image avant déploiement
cosign verify --key cosign.pub registry.example.gouv.fr/app:v1.0.0

# Politique Kubernetes (admission controller)
# Rejeter les images non signées
```

**Policy Kubernetes (Kyverno)** :
```yaml
apiVersion: kyverno.io/v1
kind: ClusterPolicy
metadata:
  name: require-image-signature
spec:
  validationFailureAction: enforce
  background: false
  rules:
    - name: verify-signature
      match:
        resources:
          kinds:
            - Pod
      verifyImages:
        - imageReferences:
            - "registry.example.gouv.fr/*"
          attestors:
            - entries:
                - keys:
                    publicKeys: |
                      -----BEGIN PUBLIC KEY-----
                      MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE...
                      -----END PUBLIC KEY-----
```

---

## 8. Checklist de Conformité Intégrité RGS

### 8.1 Signatures Électroniques

- [ ] Niveau de signature approprié identifié (SEA/SEQ)
- [ ] Certificats provenant de PSCE qualifié
- [ ] Format de signature standard (PAdES, XAdES)
- [ ] Profil long terme (-LTA) pour documents >5 ans
- [ ] Horodatage qualifié intégré aux signatures
- [ ] Clés privées stockées dans HSM/QSCD

### 8.2 Contrôles d'Intégrité

- [ ] Hash SHA-256+ pour tous les fichiers critiques
- [ ] Manifestes d'intégrité générés et vérifiés
- [ ] Colonnes d'intégrité en base de données
- [ ] Vérification périodique automatisée

### 8.3 Validation des Données

- [ ] Schémas de validation définis (JSON Schema, XSD)
- [ ] Validation côté serveur obligatoire
- [ ] Politique de liste blanche appliquée
- [ ] Rejet par défaut configuré

### 8.4 Code Source

- [ ] Commits Git signés (GPG)
- [ ] Tags de release signés
- [ ] Images Docker signées (Cosign)
- [ ] SBOM généré et signé
- [ ] Politique d'admission Kubernetes active

### 8.5 Documentation

- [ ] Procédure de signature documentée
- [ ] Liste des PSCE utilisés maintenue
- [ ] Politique de rotation des certificats
- [ ] Plan de renouvellement des signatures LTA

---

## 9. Références

### Documentation ANSSI

- **RGS v2.0 Annexe B3** : https://www.ssi.gouv.fr/rgs/
- **Prestataires qualifiés** : https://www.ssi.gouv.fr/entreprise/qualifications/
- **Guide signature électronique** : https://www.ssi.gouv.fr/guide/

### Standards ETSI

- **PAdES** : ETSI EN 319 142
- **XAdES** : ETSI EN 319 132
- **CAdES** : ETSI EN 319 122
- **JAdES** : ETSI TS 119 182

### Réglementation

- **eIDAS** : Règlement (UE) n° 910/2014
- **Code civil** : Articles 1366-1368
- **Liste de confiance UE** : https://webgate.ec.europa.eu/tl-browser/

---

**Template Version** : 1.0.0
**Last Updated** : 2025-01-15
**Next Review** : 2026-01-15
**Owner** : RSSI
