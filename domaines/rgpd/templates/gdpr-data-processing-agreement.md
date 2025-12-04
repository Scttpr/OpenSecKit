---
title: "Modèle d'accord de traitement des données RGPD (Article 28)"
template_version: "1.1.0"
domain: "gdpr"
constitutional_principles:
  - "III - Security by Design"
  - "V - Secrets Management"
regulatory_references:
  - "RGPD Article 28 (Obligations du sous-traitant)"
  - "RGPD Article 32 (Sécurité du traitement)"
  - "RGPD Article 33 (Notification d'une violation de données à caractère personnel à l'autorité de contrôle)"
ssdlc_phase: "design"
difficulty: "intermediate"
estimated_time: "2-4 heures"
last_updated: "2025-01-19"
reviewers:
  - name: "Délégué à la protection des données"
    role: "Conformité RGPD"
  - name: "Conseil juridique"
    role: "Revue de contrat"
---

# Modèle d'accord de traitement des données RGPD
# (Article 28 : DPA Responsable - Sous-traitant)

## Objectif

Ce modèle fournit un **Accord de traitement des données (DPA)** qui doit être mis en place chaque fois qu'un **sous-traitant** traite des données personnelles pour le compte d'un **responsable du traitement**.

---

## ACCORD DE TRAITEMENT DES DONNÉES

Cet Accord de traitement des données (le « **DPA** ») fait partie intégrante du [Nom de l'accord principal, ex : "Contrat de service SaaS"] (l'« **Accord** ») entre :

**LE RESPONSABLE DU TRAITEMENT** (le « **Responsable** ») :
- **Dénomination sociale** : [AG: organization.legal_name (si Responsable) ou Nom Client]
- **Adresse** : [AG: organization.address (si Responsable) ou Adresse Client]
- **Numéro d'immatriculation** : [AG: organization.registration_number (si Resp.)]
- **Email de contact** : [AG: organization.contact_email (si Resp.)]
- **Délégué à la protection des données** : [AG: governance.dpo.email (si Resp.)]

**et**

**LE SOUS-TRAITANT** (le « **Sous-traitant** ») :
- **Dénomination sociale** : [AG: organization.legal_name (si Sous-traitant) ou Nom Fournisseur]
- **Adresse** : [AG: organization.address (si Sous-traitant)]
- **Numéro d'immatriculation** : [AG: organization.registration_number (si S-T)]
- **Email de contact** : [AG: organization.contact_email (si S-T)]
- **Délégué à la protection des données** : [AG: governance.dpo.email (si S-T)]

**Date d'effet** : [AG: CURRENT_DATE]

---

## 1. DÉFINITIONS ET INTERPRÉTATION

### 1.1 Définitions

Dans le présent DPA, les termes suivants ont les significations définies ci-dessous :

- **« Lois sur la protection des données »** : Toutes les lois applicables relatives à la protection des données et à la vie privée, incluant le RGPD (Règlement (UE) 2016/679).
- **« Données personnelles »**, **« Traitement »**, **« Responsable »**, **« Sous-traitant »**, **« Violation »** : Tels que définis à l'article 4 du RGPD.

*(Le reste des définitions standard s'applique)*

---

## 2. PÉRIMÈTRE ET DÉTAILS DU TRAITEMENT

### 2.1 Objet et durée

**Description des services** :
> [AG: organization.service_description (si Sous-traitant) ou Description Manuelle]

**Durée** : Durée de l'Accord principal.

### 2.2 Nature et finalité

Le Sous-traitant est autorisé à traiter les données pour :
- Fourniture du service et support technique.
- Sécurité et intégrité du service.
- Conformité légale.

### 2.3 Catégories de Données

- [ ] Données d'identification (Nom, Email, IP)
- [ ] Données professionnelles
- [ ] Données techniques (Logs, Cookies)
- [ ] Données financières
- [ ] Autre : [AG: data_processing.other_categories]

---

## 3. OBLIGATIONS DU SOUS-TRAITANT

### 3.1 Instructions et Confidentialité
Le Sous-traitant traite les données uniquement sur instruction du Responsable et garantit la confidentialité de son personnel.

### 3.2 Sécurité (Article 32)
Le Sous-traitant met en œuvre les mesures techniques et organisationnelles décrites à l'**Annexe B**.

### 3.3 Sous-traitants ultérieurs
Le Responsable autorise le Sous-traitant à engager des sous-traitants ultérieurs (liste disponible sur demande ou site web). Tout changement sera notifié avec un préavis de 30 jours.

### 3.4 Assistance et Droits des personnes
Le Sous-traitant assiste le Responsable pour répondre aux demandes (Droit d'accès, rectification, effacement) et pour la réalisation d'AIPD si nécessaire.

### 3.5 Notification de violation
Le Sous-traitant notifie le Responsable de toute violation de données personnelles dans les meilleurs délais (max **48 heures**).

---

## 4. TRANSFERTS INTERNATIONAUX

**Lieux de traitement principaux** : [AG: organization.address (Pays)]

Si les données sont transférées hors de l'EEE :
- [ ] Décision d'adéquation.
- [ ] Clauses Contractuelles Types (CCT) de l'UE (Voir Annexe A).
- [ ] Cadre de protection des données (ex: DPF UE-US).

---

## 5. SIGNATURES

**POUR LE RESPONSABLE** :
Nom : ___________________________
Date : [AG: CURRENT_DATE]
Signature : _____________________

**POUR LE SOUS-TRAITANT** :
Nom : ___________________________
Date : [AG: CURRENT_DATE]
Signature : _____________________

---

## ANNEXE A : CLAUSES CONTRACTUELLES TYPES (Si applicable)

*(Intégrer ici les CCT 2021/914 Module 2 si transfert hors UE sans adéquation)*

---

## ANNEXE B : MESURES DE SÉCURITÉ TECHNIQUES ET ORGANISATIONNELLES

Le Sous-traitant met en œuvre les mesures suivantes :

### 1. Chiffrement et Anonymisation
- **Au repos** : [AG: security_measures.encryption_at_rest.enabled ? "✅ Activé" : "❌ Non activé"]
  - Standard : [AG: security_measures.encryption_at_rest.details] (ex: AES-256)
- **En transit** : [AG: security_measures.encryption_in_transit.enabled ? "✅ Activé" : "❌ Non activé"]
  - Standard : [AG: security_measures.encryption_in_transit.details] (ex: TLS 1.2+)

### 2. Capacité à assurer la confidentialité, l'intégrité et la disponibilité
- **Contrôle d'accès** : [AG: security_measures.access_control.mfa_enforced ? "Authentification Multi-Facteurs (MFA)" : "Authentification standard"]
- **Sauvegardes** :
  - Fréquence : [AG: security_measures.backups.frequency]
  - Tests de restauration effectués régulièrement.

### 3. Procédure de test et d'évaluation (Audit)
- Scans de vulnérabilités réguliers.
- Audits de sécurité (ex: Tests d'intrusion) annuels.
- Certifications : [AG: security_measures.certifications]

### 4. Mesures organisationnelles
- Politique de sécurité de l'information documentée.
- Formation du personnel à la protection des données.
- Gestion des incidents et procédure de notification.