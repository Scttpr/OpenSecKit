# Référentiel Général de sécurité (RGS) - Domain Extension

## Vue d'ensemble

Cette extension du toolkit SSDLC couvre les exigences du **Référentiel Général de sécurité (RGS)**, le cadre de sécurité obligatoire pour les systèmes d'information de l'administration française et ses partenaires.

**Applicable à**:
- Administrations publiques françaises (État, collectivités territoriales)
- Opérateurs d'importance vitale (OIV)
- Prestataires de services numériques pour l'administration
- Systèmes traitant des données sensibles de l'État

## Contexte réglementaire

### Base légale

- **Ordonnance n° 2005-1516** du 8 décembre 2005
- **Décret n° 2010-112** du 2 février 2010
- **Arrêté du 13 juin 2014** portant référentiel général de sécurité (version 2.0)
- **ANSSI** (Agence Nationale de la sécurité des Systèmes d'Information) - autorité de certification

### Niveaux de sécurité RGS

| Niveau | Description | Exemples d'usage |
|--------|-------------|------------------|
| **RGS*** | Niveau standard | Services numériques grand public (démarches administratives courantes) |
| **RGS** | Niveau renforcé | Données sensibles, services critiques (impôts, santé) |
| **RGS***| Niveau élevé | Données très sensibles, secret défense |

## Principes constitutionnels RGS

Le RGS s'articule autour de 4 fonctions de sécurité :

### 1. Identification et Authentification
- Authentification forte (certificats RGS, FranceConnect)
- Gestion des identités numériques
- Traçabilité des accès

### 2. Intégrité
- Signature électronique qualifiée RGS
- Horodatage qualifié
- Protection contre l'altération des données

### 3. Confidentialité
- Chiffrement conforme RGS (algorithmes homologués ANSSI)
- Gestion des clés cryptographiques
- Protection des données sensibles

### 4. Traçabilité
- Journalisation des événements de sécurité
- Conservation sécurisée des journaux (3 ans minimum)
- Horodatage des événements

## Mapping avec les 7 principes constitutionnels SSDLC

| Principe SSDLC | Exigences RGS | Référence RGS |
|----------------|---------------|---------------|
| **I - Threat Modeling** | Analyse de risques obligatoire (méthode EBIOS Risk Manager) | Article 5 |
| **II - Risk Analysis** | Homologation de sécurité avec dossier d'homologation | Articles 8-11 |
| **III - Security by Design** | Règles et recommandations de sécurité par fonction | Annexe B1 |
| **IV - Security Testing** | Tests de sécurité et audits de conformité | Article 12 |
| **V - Secrets Management** | Gestion des clés cryptographiques (IGC/PKI conforme RGS) | Annexe B3 |
| **VI - Audit Logging** | Traçabilité des événements (conservation 3 ans) | Annexe B1 §3.4 |
| **VII - Patch Management** | Maintien en condition de sécurité (MCS) | Article 13 |

## Templates spécifiques RGS

### Templates disponibles

> **Note** : Les templates spécifiques RGS suivants sont prévus pour une version future. En attendant, utilisez les templates génériques de la section "Templates de base compatibles RGS" ci-dessous.

Templates prévus :
- Dossier d'homologation de sécurité RGS - Articles 8-11
- Analyse de risques selon méthode EBIOS RM - Guide ANSSI EBIOS RM
- Exigences d'authentification RGS (FranceConnect, certificats) - Annexe B1 §3.1
- Algorithmes et mécanismes cryptographiques homologués ANSSI - Annexe B3

### Templates de base compatibles RGS

Les templates core du toolkit peuvent être utilisés avec les adaptations RGS suivantes :

- **STRIDE Threat Model** → Compléter avec **EBIOS Risk Manager** (méthode obligatoire)
- **Risk Scoring** → Utiliser **grille de gravité RGS** (négligeable, limitée, importante, critique)
- **Authentication Requirements** → Ajouter **FranceConnect** et **certificats RGS**
- **Encryption Requirements** → Utiliser **uniquement algorithmes homologués ANSSI**
- **Audit Logging** → Conservation **minimum 3 ans** (vs 90 jours standard)

## Exigences cryptographiques ANSSI

### Algorithmes approuvés (liste blanche)

**Chiffrement symétrique**:
- ✅ AES-128, AES-192, AES-256 (modes GCM, CCM)
- ❌ DES, 3DES (deprecated)

**Chiffrement asymétrique**:
- ✅ RSA ≥ 2048 bits (recommandé 4096 bits)
- ✅ ECDSA (courbes P-256, P-384, P-521)
- ❌ RSA < 2048 bits

**Fonction de hachage**:
- ✅ SHA-256, SHA-384, SHA-512
- ❌ MD5, SHA-1 (deprecated)

**Signature électronique**:
- ✅ RSA-PSS ≥ 2048 bits
- ✅ ECDSA (courbes ANSSI)

### Prestataires de confiance qualifiés

- **ANTS** (Agence Nationale des Titres Sécurisés)
- **Certinomis** (La Poste)
- **ChamberSign** (CCI France)
- **Universign**

Liste complète : https://www.lsti-certification.fr/

## Processus d'homologation RGS

### Étapes obligatoires

```
1. Étude de sécurité
   ├── Analyse de risques (EBIOS RM)
   ├── Identification des mesures de sécurité
   └── Validation par l'autorité d'homologation

2. Dossier d'homologation
   ├── Expression des besoins de sécurité
   ├── Analyse de risques
   ├── Spécifications de sécurité
   ├── Plan d'action
   └── Attestation formelle de sécurité

3. Décision d'homologation
   ├── Revue par autorité d'homologation
   ├── Validation du risque résiduel accepté
   └── Signature de la décision d'homologation

4. Maintien en condition de sécurité (MCS)
   ├── Surveillance continue
   ├── Gestion des incidents
   ├── Mises à jour de sécurité
   └── Ré-homologation (tous les 3-5 ans)
```

### Rôles et responsabilités

| Rôle | Responsabilités |
|------|-----------------|
| **Autorité d'homologation** | Valide et signe la décision d'homologation, accepte le risque résiduel |
| **Responsable de la sécurité des systèmes d'information (RSSI)** | Pilote l'étude de sécurité, constitue le dossier d'homologation |
| **Maîtrise d'ouvrage (MOA)** | Définit les besoins de sécurité, valide les mesures |
| **Maîtrise d'œuvre (MOE)** | Met en œuvre les mesures de sécurité |
| **Exploitant** | Assure le MCS (maintien en condition de sécurité) |

## FranceConnect - Authentification mutualisée

### Qu'est-ce que FranceConnect ?

Système d'authentification mutualisé permettant aux usagers d'accéder aux services en ligne de l'administration avec leurs identifiants de fournisseurs d'identité partenaires.

### Fournisseurs d'identité FranceConnect

- **Impots.gouv.fr** (DGFiP)
- **Ameli.fr** (Assurance Maladie)
- **LaPoste** (Identité Numérique La Poste)
- **MobileConnect et Moi** (Orange)
- **MSA** (Mutualité Sociale Agricole)

### Intégration FranceConnect

**Protocole**: OpenID Connect (OIDC)

**Niveaux de garantie**:
- **eIDAS Faible** : Authentification simple (email + mot de passe)
- **eIDAS Substantiel** : Authentification forte (2FA)
- **eIDAS Élevé** : Authentification très forte (certificat qualifié)

**Cas d'usage**:
```javascript
// Redirection vers FranceConnect
const franceConnectUrl = 'https://fcp.integ01.dev-franceconnect.fr/api/v1/authorize';
const params = new URLSearchParams({
  client_id: 'YOUR_CLIENT_ID',
  redirect_uri: 'https://your-app.gouv.fr/callback',
  response_type: 'code',
  scope: 'openid profile email',
  state: generateSecureState(),
  nonce: generateSecureNonce(),
  acr_values: 'eidas1' // Niveau eIDAS
});

window.location.href = `${franceConnectUrl}?${params}`;
```

## Checklist de conformité RGS

### Fonction 1 : Identification et Authentification

- [ ] Authentification forte mise en œuvre (FranceConnect, certificat RGS, 2FA)
- [ ] Gestion des identités numériques documentée
- [ ] Contrôle d'accès basé sur les rôles (RBAC)
- [ ] Traçabilité des authentifications (logs 3 ans)
- [ ] Politique de mots de passe conforme (12 caractères minimum, complexité)

### Fonction 2 : Intégrité

- [ ] Signature électronique qualifiée RGS pour documents officiels
- [ ] Horodatage qualifié pour événements critiques
- [ ] Contrôles d'intégrité des données (hachage SHA-256+)
- [ ] Protection contre modification non autorisée
- [ ] Journalisation des modifications de données sensibles

### Fonction 3 : Confidentialité

- [ ] Chiffrement des données sensibles au repos (AES-256)
- [ ] Chiffrement des communications (TLS 1.3, algorithmes ANSSI)
- [ ] Gestion des clés cryptographiques (IGC/PKI qualifiée)
- [ ] Destruction sécurisée des données en fin de vie
- [ ] Classification des données selon sensibilité

### Fonction 4 : Traçabilité

- [ ] Journalisation de tous les événements de sécurité
- [ ] Horodatage RGS des événements critiques
- [ ] Conservation des journaux minimum 3 ans
- [ ] Protection en intégrité et confidentialité des journaux
- [ ] Revue régulière des journaux (incidents, anomalies)

### Homologation

- [ ] Analyse de risques EBIOS Risk Manager réalisée
- [ ] Dossier d'homologation constitué (complet)
- [ ] Mesures de sécurité définies et mises en œuvre
- [ ] Risque résiduel documenté et accepté
- [ ] Décision d'homologation signée par autorité compétente
- [ ] Plan de maintien en condition de sécurité (MCS)

## Ressources officielles

### documentation ANSSI

- **Guide RGS v2.0**: https://cyber.gouv.fr/le-referentiel-general-de-securite-rgs
- **EBIOS Risk Manager**: https://cyber.gouv.fr/la-methode-ebios-risk-manager
- **Guides de bonnes pratiques**: https://cyber.gouv.fr/publications

### FranceConnect

- **documentation technique**: https://partenaires.franceconnect.gouv.fr/
- **Kit d'intégration**: https://github.com/france-connect/
- **Fournisseurs d'identité**: https://franceconnect.gouv.fr/partenaires

### Prestataires qualifiés

- **Liste des PSCE qualifiés**: https://www.lsti-certification.fr/
- **Annuaire eIDAS**: https://webgate.ec.europa.eu/tl-browser/

## Support et accompagnement

### ANSSI

- **Contact**: cert-fr.cossi@ssi.gouv.fr
- **Téléphone**: +33 1 71 75 84 68
- **Accompagnement**: Guides, formations, audits

### DINUM (Direction Interministérielle du Numérique)

- **FranceConnect Support**: support.partenaires@franceconnect.gouv.fr
- **documentation**: https://partenaires.franceconnect.gouv.fr/

---

## Exemple de workflow RGS complet

```
Projet : Nouvelle application de téléprocédure administrative

Phase 0 : Cadrage (Avant-projet)
├── Identifier le niveau de sécurité requis (RGS*, RGS**, RGS***)
├── Désigner l'autorité d'homologation
└── Constituer l'équipe projet (MOA, MOE, RSSI)

Phase 1 : Analyse de risques (Planning)
├── EBIOS Risk Manager (obligatoire)
│   ├── Socle de sécurité (besoins fondamentaux)
│   ├── Sources de risques (acteurs malveillants)
│   ├── Scénarios stratégiques (objectifs visés)
│   ├── Scénarios opérationnels (modes opératoires)
│   └── Traitement du risque (mesures de sécurité)
└── documentation : ebios-risk-manager.md

Phase 2 : Spécifications de sécurité (Design)
├── Exigences RGS par fonction (auth, intégrité, confidentialité, traçabilité)
├── Choix des solutions (FranceConnect, certificats RGS, chiffrement ANSSI)
└── documentation : rgs-authentication-requirements.md, rgs-cryptography-requirements.md

Phase 3 : implémentation
├── Intégration FranceConnect
├── Mise en œuvre chiffrement (AES-256-GCM)
├── Journalisation conforme (3 ans)
└── Tests de sécurité (pentests, audits)

Phase 4 : Dossier d'homologation
├── Rédaction du dossier (template rgs-homologation-dossier-template.md)
├── Validation RSSI
├── Revue par autorité d'homologation
└── Signature de la décision d'homologation

Phase 5 : Mise en production
├── déploiement sécurisé
├── Formation des exploitants
└── Mise en place du MCS (maintien en condition de sécurité)

Phase 6 : Maintien en condition de sécurité (Maintenance)
├── Surveillance continue (SIEM)
├── Gestion des incidents
├── Mises à jour de sécurité (Principle VII - Patch Management)
├── Revue annuelle de sécurité
└── Ré-homologation tous les 3-5 ans
```

---

**Note importante**: Le RGS est un référentiel évolutif. Toujours consulter la dernière version officielle sur le site de l'ANSSI.

**Prochaines étapes**: Utilisez les templates génériques d'OpenSecKit (Principes I-VII) pour démarrer votre mise en conformité RGS. Des templates spécifiques RGS seront ajoutés prochainement.
