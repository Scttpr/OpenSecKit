---
title: "Modèle de registre des activités de traitement RGPD (Article 30)"
template_version: "1.1.0"
domain: "gdpr"
constitutional_principle: "VI"
regulatory_references:
  - "RGPD Article 30 (Registre des activités de traitement)"
  - "RGPD Article 5.2 (Principe de responsabilité)"
ssdlc_phase: "all"
difficulty: "beginner"
estimated_time: "4-8 heures (création initiale), 1-2 heures (mises à jour trimestrielles)"
last_updated: "2025-01-19"
reviewers:
  - name: "Délégué à la protection des données"
    role: "Conformité RGPD"
  - name: "Conseil juridique"
    role: "Revue réglementaire"
---

# Modèle de registre des activités de traitement RGPD
# (Article 30 : Registre des traitements)

## Objectif

Ce modèle vous aide à créer et maintenir un **Registre des activités de traitement** (RoPA), un **document de conformité RGPD obligatoire** qui inventorie toutes les opérations de traitement de données personnelles.

**Exigence légale** : L'Article 30 requiert :
- Aux **responsables de traitement** de maintenir un registre de toutes les activités de traitement sous leur responsabilité.
- Aux **sous-traitants** de maintenir un registre de toutes les activités de traitement effectuées pour le compte des responsables.

---

## REGISTRE DES ACTIVITÉS DE TRAITEMENT

---

## A. REGISTRES RESPONSABLE DE TRAITEMENT (Article 30.1)

**Utiliser cette section si vous décidez "comment et pourquoi" traiter les données (ex : SaaS B2C, E-commerce, Employeur).**

### 1. IDENTIFICATION DU RESPONSABLE DE TRAITEMENT

**Responsable de traitement** :
- **Nom légal** : [AG: organization.legal_name]
- **Adresse** : [AG: organization.address]
- **Numéro d'enregistrement** : [AG: organization.registration_number]
- **Email de contact** : [AG: organization.contact_email]
- **Téléphone** : [AG: organization.phone]
- **Site web** : [AG: organization.website]

**Délégué à la protection des données (DPO)** :
- **Nom** : [AG: governance.dpo.name]
- **Email** : [AG: governance.dpo.email]

**Représentant dans l'UE** (si responsable hors UE) :
- **Nom** : [AG: organization.eu_representative.name]
- **Email** : [AG: organization.eu_representative.email]

---

### 2. ACTIVITÉS DE TRAITEMENT DÉTAILLÉES

#### Activité : [AG: activity.name]

**ID de l'activité** : [AG: activity.id]
**Finalité** : [AG: activity.purpose]
**Département responsable** : [À compléter]

**1. Catégories de personnes concernées** :
- [ ] Clients / Utilisateurs
- [ ] Employés
- [ ] Candidats
- [ ] Visiteurs du site web
- [ ] Autre : [AG: activity.data_subjects]

**2. Catégories de données personnelles** :
*Exemples détectés :* [AG: activity.data_categories]

**3. Base juridique (Article 6)** :
- [ ] Consentement
- [ ] Contrat (Exécution)
- [ ] Obligation légale
- [ ] Intérêt légitime
**Base retenue** : [AG: activity.legal_basis]

**4. Destinataires des données (Sous-traitants)** :
| Nom du destinataire | Rôle | Localisation | Garantie Transfert |
|---------------------|------|--------------|--------------------|
| [AG: processor.name] | [AG: processor.role] | [AG: processor.location] | [AG: processor.transfer_mechanism] |

**5. Durée de conservation** :
**Règle appliquée** : [AG: retention_policies.default ou spécifique]

---

### 3. MESURES DE SÉCURITÉ (Article 32)

**Mesures techniques** :
- **Chiffrement au repos** :
  - Statut : [AG: security_measures.encryption_at_rest.enabled ? "✅ Activé" : "❌ Non activé"]
  - Standard : [AG: security_measures.encryption_at_rest.standard]
  - Périmètre : [AG: security_measures.encryption_at_rest.scope]

- **Chiffrement en transit** :
  - Statut : [AG: security_measures.encryption_in_transit.enabled ? "✅ Activé" : "❌ Non activé"]
  - Standard : [AG: security_measures.encryption_in_transit.standard]

- **Contrôle d'accès** :
  - MFA (Admin) : [AG: security_measures.access_control.mfa_required ? "✅ Requis" : "❌ Non requis"]
  - SSO : [AG: security_measures.access_control.sso_enabled ? "✅ Activé" : "❌ Désactivé"]

- **Sauvegardes (Disponibilité)** :
  - Fréquence : [AG: security_measures.backups.frequency]
  - Rétention : [AG: security_measures.backups.retention_days] jours

**Certifications** :
- [AG: security_measures.certifications]

---

## B. REGISTRES SOUS-TRAITANT (Article 30.2)

**Utiliser cette section si vous traitez des données pour le compte d'un autre (ex : Hébergeur, API Backend B2B, Service Paie).**

### 1. IDENTIFICATION DU SOUS-TRAITANT

**Sous-traitant** :
- **Nom légal** : [AG: organization.legal_name]
- **Adresse** : [AG: organization.address]
- **Contact DPO** : [AG: governance.dpo.email]

### 2. CLIENTS (RESPONSABLES DE TRAITEMENT)

| Nom du Responsable (Client) | Contact | DPA signé ? | Nature du traitement |
|-----------------------------|---------|-------------|----------------------|
| [Nom Client 1] | [Email] | [ ] Oui | [Description] |
| [Nom Client 2] | [Email] | [ ] Oui | [Description] |

### 3. CATÉGORIES DE TRAITEMENT

**Description des services fournis** :
> [AG: organization.service_description ou description manuelle]

**Mesures de sécurité** :
*(Voir la section Sécurité ci-dessus, identique pour le sous-traitant)*
- Chiffrement : [AG: security_measures.encryption_at_rest.standard]
- Backups : [AG: security_measures.backups.frequency]

---

## MAINTENANCE DU REGISTRE

### Propriété et responsabilité

**Propriétaire du registre** : [AG: governance.dpo.name]
**Dernière mise à jour** : [AG: CURRENT_DATE]

**Fréquence de revue** : Trimestrielle

### Accès autorité de contrôle

Ces registres sont maintenus sous format électronique et sont exportables pour l'autorité de contrôle ([AG: governance.supervisory_authority.name]) sur simple demande.