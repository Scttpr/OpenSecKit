---
title: "Checklist d'Audit de Conformité RGS"
template_version: "1.0.0"
domain: "government-rgs"
purpose: "osk-audit-rgs"
constitutional_principles:
  - "I - Threat Modeling"
  - "II - Risk Analysis"
  - "III - Security by Design"
  - "IV - Security Testing"
  - "V - Secrets Management"
  - "VI - Audit Logging"
  - "VII - Patch Management"
regulatory_references:
  - "RGS v2.0 - Toutes annexes"
  - "ANSSI - Guide d'homologation"
ssdlc_phase: "All"
---

# Checklist d'Audit de Conformité RGS

## Objectif

Cette checklist permet d'auditer la conformité RGS d'un système d'information en croisant :
- Les **exigences réglementaires** du RGS v2.0
- Les **documents de sécurité** générés par OSK (`docs/security/features/`, `docs/security/risks/`)
- Le **contexte projet** (`skeleton.yaml` / `.osk/rgs-context.yaml`)

Elle est utilisée par la commande **`osk-audit rgs`** pour générer un rapport de conformité.

---

## Structure de l'Audit

```
AUDIT RGS
├── 1. Prérequis (contexte projet)
├── 2. Fonction 1 : Authentification (Annexe B2)
├── 3. Fonction 2 : Intégrité (Annexe B3)
├── 4. Fonction 3 : Confidentialité (Annexe B4)
├── 5. Fonction 4 : Traçabilité (Annexe B5)
├── 6. Analyse de risques (EBIOS RM)
├── 7. Homologation (Articles 8-11)
├── 8. MCS - Maintien en Condition de Sécurité (Article 13)
└── 9. Synthèse et score global
```

---

## 1. Prérequis - Contexte Projet

### 1.1 Fichiers de contexte requis

| Fichier | Obligatoire | Source | Vérification |
|---------|-------------|--------|--------------|
| `.osk/rgs-context.yaml` | ✅ | Manuel (copie de `skeleton.yaml`) | Fichier existe et parsable |
| `docs/context/meta.md` | ✅ | Auto-généré par `/osk-security` | Fichier existe |
| `docs/security/risks/risk-register.yaml` | ✅ | Auto-généré par `/osk-security` | Fichier existe |

**Critères de validation** :

```yaml
# Vérification automatique
prerequis:
  - id: PRE-001
    description: "Fichier de contexte RGS présent"
    verification: "file_exists(.osk/rgs-context.yaml)"
    criticite: BLOQUANT
    source_osk: null
    source_manuelle: "skeleton.yaml"

  - id: PRE-002
    description: "Niveau RGS défini"
    verification: "rgs_context.classification.niveau_rgs in ['RGS*', 'RGS**', 'RGS***']"
    criticite: BLOQUANT
    source_osk: null
    source_manuelle: "rgs-context.yaml#classification.niveau_rgs"

  - id: PRE-003
    description: "Autorité d'homologation identifiée"
    verification: "rgs_context.organisation.autorite_homologation.nom != '[À COMPLÉTER]'"
    criticite: BLOQUANT
    source_osk: null
    source_manuelle: "rgs-context.yaml#organisation.autorite_homologation"

  - id: PRE-004
    description: "RSSI identifié"
    verification: "rgs_context.organisation.rssi.nom != '[À COMPLÉTER]'"
    criticite: IMPORTANT
    source_osk: null
    source_manuelle: "rgs-context.yaml#organisation.rssi"

  - id: PRE-005
    description: "Méta-données projet générées"
    verification: "file_exists(docs/context/meta.md)"
    criticite: BLOQUANT
    source_osk: "/osk-security (première exécution)"
    source_manuelle: null

  - id: PRE-006
    description: "Registre des risques initialisé"
    verification: "file_exists(docs/security/risks/risk-register.yaml)"
    criticite: BLOQUANT
    source_osk: "/osk-security"
    source_manuelle: null
```

---

## 2. Fonction 1 : Authentification (Annexe B2)

### 2.1 Exigences par niveau RGS

| Exigence | RGS* | RGS** | RGS*** | Référence |
|----------|------|-------|--------|-----------|
| Authentification simple (login/password) | ✅ | ❌ | ❌ | B2 §3.1 |
| Authentification forte (MFA) | ⚠️ Recommandé | ✅ | ✅ | B2 §3.2 |
| Certificat RGS ou FranceConnect | ❌ | ✅ | ✅ | B2 §3.3 |
| Certificat qualifié eIDAS | ❌ | ⚠️ Recommandé | ✅ | B2 §3.4 |

### 2.2 Checklist d'audit

```yaml
authentification:
  # --- Exigences communes ---
  - id: AUTH-001
    description: "Politique de mots de passe conforme"
    niveau_requis: ["RGS*", "RGS**", "RGS***"]
    criteres:
      - "Longueur minimum 12 caractères"
      - "Complexité (majuscule, minuscule, chiffre, spécial)"
      - "Historique des 5 derniers mots de passe"
      - "Expiration configurable (recommandé 90 jours)"
    verification_auto: "meta.md#patterns_securite.authentification contains 'password_policy'"
    source_osk: "docs/context/meta.md"
    source_manuelle: "rgs-context.yaml#fonctions_securite.authentification.politique_mdp"
    criticite: IMPORTANT

  - id: AUTH-002
    description: "Verrouillage de compte après échecs"
    niveau_requis: ["RGS*", "RGS**", "RGS***"]
    criteres:
      - "Verrouillage après 5 tentatives maximum"
      - "Durée de verrouillage minimum 30 minutes"
      - "Journalisation des verrouillages"
    verification_auto: "SEC-*.md contains 'verrouillage' OR 'account_lockout'"
    source_osk: "docs/security/features/SEC-*.md"
    source_manuelle: "rgs-context.yaml#fonctions_securite.authentification.verrouillage"
    criticite: IMPORTANT

  - id: AUTH-003
    description: "Gestion des sessions sécurisée"
    niveau_requis: ["RGS*", "RGS**", "RGS***"]
    criteres:
      - "Timeout d'inactivité (15 min pour RGS**)"
      - "Timeout absolu (4h pour RGS**)"
      - "Invalidation à la déconnexion"
      - "Token de session non prédictible"
    verification_auto: "SEC-*.md contains 'session' AND 'timeout'"
    source_osk: "docs/security/features/SEC-*.md (Principe VI)"
    source_manuelle: null
    criticite: IMPORTANT

  # --- Exigences RGS** et RGS*** ---
  - id: AUTH-004
    description: "Authentification forte (MFA) activée"
    niveau_requis: ["RGS**", "RGS***"]
    criteres:
      - "Second facteur obligatoire (TOTP, SMS, clé physique)"
      - "Résistance au phishing (FIDO2 pour RGS***)"
    verification_auto: "rgs_context.fonctions_securite.authentification.mfa_actif == true"
    source_osk: null
    source_manuelle: "rgs-context.yaml#fonctions_securite.authentification.mfa_actif"
    criticite: BLOQUANT

  - id: AUTH-005
    description: "FranceConnect intégré ou équivalent RGS"
    niveau_requis: ["RGS**", "RGS***"]
    criteres:
      - "Intégration FranceConnect OU certificat RGS"
      - "Niveau eIDAS Substantial minimum"
      - "Validation du nonce et state (anti-replay, anti-CSRF)"
    verification_auto: "SEC-*.md contains 'FranceConnect' OR 'certificat RGS'"
    source_osk: "docs/security/features/SEC-*.md"
    source_manuelle: "rgs-context.yaml#fonctions_securite.authentification"
    template_reference: "rgs-franceconnect-auth-requirements.md"
    criticite: BLOQUANT

  # --- Exigences RGS*** uniquement ---
  - id: AUTH-006
    description: "Certificat qualifié eIDAS"
    niveau_requis: ["RGS***"]
    criteres:
      - "Certificat délivré par PSCE qualifié"
      - "Niveau eIDAS High"
      - "QSCD (dispositif de création qualifié)"
    verification_auto: "rgs_context.fonctions_securite.authentification.niveau_eidas == 'High'"
    source_osk: null
    source_manuelle: "rgs-context.yaml"
    criticite: BLOQUANT
```

### 2.3 Mapping vers documents OSK

| Exigence | Document OSK | Section | Champ à vérifier |
|----------|--------------|---------|------------------|
| AUTH-001 | `meta.md` | Patterns de sécurité | `authentification` |
| AUTH-002 | `SEC-*.md` | Principe III | Checklist moindre privilège |
| AUTH-003 | `SEC-*.md` | Principe VI | Gestion des sessions |
| AUTH-004 | `rgs-context.yaml` | Fonctions sécurité | `mfa_actif` |
| AUTH-005 | `SEC-auth-*.md` | Analyse feature auth | FranceConnect |

---

## 3. Fonction 2 : Intégrité (Annexe B3)

### 3.1 Exigences par niveau RGS

| Exigence | RGS* | RGS** | RGS*** | Référence |
|----------|------|-------|--------|-----------|
| Hash SHA-256+ pour fichiers | ✅ | ✅ | ✅ | B3 §2.1 |
| Signature électronique avancée (SEA) | ⚠️ | ✅ | ✅ | B3 §3.1 |
| Signature électronique qualifiée (SEQ) | ❌ | ⚠️ | ✅ | B3 §3.2 |
| Horodatage qualifié | ❌ | ⚠️ | ✅ | B3 §4.1 |

### 3.2 Checklist d'audit

```yaml
integrite:
  - id: INT-001
    description: "Algorithmes de hash conformes ANSSI"
    niveau_requis: ["RGS*", "RGS**", "RGS***"]
    criteres:
      - "SHA-256 minimum (SHA-384 recommandé)"
      - "MD5 et SHA-1 interdits"
    verification_auto: "meta.md NOT contains 'MD5' AND NOT contains 'SHA-1'"
    source_osk: "docs/context/meta.md"
    source_manuelle: null
    template_reference: "rgs-anssi-cryptography-requirements.md"
    criticite: IMPORTANT

  - id: INT-002
    description: "Validation des données d'entrée"
    niveau_requis: ["RGS*", "RGS**", "RGS***"]
    criteres:
      - "Schémas de validation définis (JSON Schema, XSD)"
      - "Validation côté serveur obligatoire"
      - "Liste blanche (pas liste noire)"
    verification_auto: "SEC-*.md contains 'validation' AND 'schéma'"
    source_osk: "docs/security/features/SEC-*.md (Principe III)"
    source_manuelle: null
    template_reference: "rgs-integrity-requirements.md"
    criticite: IMPORTANT

  - id: INT-003
    description: "Contrôle d'intégrité des fichiers critiques"
    niveau_requis: ["RGS**", "RGS***"]
    criteres:
      - "Manifeste d'intégrité pour fichiers de config"
      - "Vérification au démarrage"
      - "Alerting en cas de modification"
    verification_auto: "SEC-*.md contains 'intégrité' OR 'checksum' OR 'hash'"
    source_osk: "docs/security/features/SEC-*.md"
    source_manuelle: "rgs-context.yaml#fonctions_securite.integrite"
    criticite: IMPORTANT

  - id: INT-004
    description: "Signature des documents officiels"
    niveau_requis: ["RGS**", "RGS***"]
    criteres:
      - "Format PAdES, XAdES ou CAdES"
      - "Profil -B-T minimum (avec horodatage)"
      - "Certificat de PSCE qualifié"
    verification_auto: "rgs_context.fonctions_securite.integrite.signature_documents != null"
    source_osk: null
    source_manuelle: "rgs-context.yaml#fonctions_securite.integrite"
    template_reference: "rgs-integrity-requirements.md"
    criticite: IMPORTANT

  - id: INT-005
    description: "Horodatage qualifié pour preuves juridiques"
    niveau_requis: ["RGS***"]
    criteres:
      - "TSA qualifiée RGS (Certinomis, Universign, etc.)"
      - "RFC 3161 conforme"
      - "Archivage des jetons"
    verification_auto: "rgs_context.fonctions_securite.integrite.horodatage_qualifie == true"
    source_osk: null
    source_manuelle: "rgs-context.yaml#fonctions_securite.integrite"
    template_reference: "rgs-integrity-requirements.md"
    criticite: BLOQUANT

  - id: INT-006
    description: "Signature du code et des images Docker"
    niveau_requis: ["RGS**", "RGS***"]
    criteres:
      - "Commits Git signés (GPG)"
      - "Tags de release signés"
      - "Images Docker signées (Cosign, Notary)"
    verification_auto: "git log --show-signature returns signatures"
    source_osk: null
    source_manuelle: "Vérification manuelle Git"
    template_reference: "rgs-integrity-requirements.md"
    criticite: RECOMMANDE
```

---

## 4. Fonction 3 : Confidentialité (Annexe B4)

### 4.1 Exigences par niveau RGS

| Exigence | RGS* | RGS** | RGS*** | Référence |
|----------|------|-------|--------|-----------|
| TLS 1.2+ | ✅ | ✅ | ✅ | B4 §2.1 |
| TLS 1.3 | ⚠️ | ✅ | ✅ | B4 §2.1 |
| AES-128 | ✅ | ⚠️ | ❌ | B4 §3.1 |
| AES-256 | ⚠️ | ✅ | ✅ | B4 §3.1 |
| Gestion des clés (KMS/HSM) | ⚠️ | ✅ | ✅ | B4 §4 |

### 4.2 Checklist d'audit

```yaml
confidentialite:
  - id: CONF-001
    description: "TLS correctement configuré"
    niveau_requis: ["RGS*", "RGS**", "RGS***"]
    criteres:
      - "TLS 1.2 minimum (TLS 1.3 pour RGS**+)"
      - "TLS 1.0 et 1.1 désactivés"
      - "Cipher suites ECDHE uniquement (PFS)"
      - "HSTS activé"
    verification_auto: "meta.md contains 'TLS' AND (contains '1.2' OR contains '1.3')"
    source_osk: "docs/context/meta.md"
    source_manuelle: "rgs-context.yaml#fonctions_securite.confidentialite.chiffrement_transit"
    template_reference: "rgs-anssi-cryptography-requirements.md"
    criticite: BLOQUANT

  - id: CONF-002
    description: "Chiffrement des données au repos"
    niveau_requis: ["RGS*", "RGS**", "RGS***"]
    criteres:
      - "AES-256-GCM pour données sensibles (RGS**+)"
      - "Chiffrement BDD (TDE ou champ)"
      - "Chiffrement disque (LUKS/BitLocker)"
    verification_auto: "SEC-*.md contains 'chiffrement' OR 'AES'"
    source_osk: "docs/security/features/SEC-*.md (Principe V)"
    source_manuelle: "rgs-context.yaml#fonctions_securite.confidentialite"
    template_reference: "rgs-anssi-cryptography-requirements.md"
    criticite: IMPORTANT

  - id: CONF-003
    description: "Gestion des clés cryptographiques"
    niveau_requis: ["RGS**", "RGS***"]
    criteres:
      - "Clés stockées dans KMS/HSM (pas en clair)"
      - "Rotation des clés automatisée"
      - "Séparation KEK/DEK (envelope encryption)"
    verification_auto: "rgs_context.fonctions_securite.confidentialite.gestion_cles != null"
    source_osk: null
    source_manuelle: "rgs-context.yaml#fonctions_securite.confidentialite.gestion_cles"
    template_reference: "rgs-anssi-cryptography-requirements.md"
    criticite: BLOQUANT

  - id: CONF-004
    description: "Aucun secret dans le code source"
    niveau_requis: ["RGS*", "RGS**", "RGS***"]
    criteres:
      - "Scan gitleaks/trufflehog en CI"
      - "Pre-commit hook actif"
      - "Secrets dans variables d'environnement ou vault"
    verification_auto: "SEC-*.md (Principe V) contains 'gitleaks' OR 'trufflehog' OR 'vault'"
    source_osk: "docs/security/features/SEC-*.md (Principe V)"
    source_manuelle: null
    criticite: BLOQUANT

  - id: CONF-005
    description: "Certificat TLS valide et conforme"
    niveau_requis: ["RGS*", "RGS**", "RGS***"]
    criteres:
      - "Délivré par CA reconnue (Let's Encrypt pour RGS*, PSCE pour RGS**+)"
      - "RSA-4096 ou ECDSA P-384"
      - "Validité < 13 mois"
      - "OCSP Stapling activé"
    verification_auto: "rgs_context.fonctions_securite.confidentialite.certificat_tls != null"
    source_osk: null
    source_manuelle: "rgs-context.yaml#fonctions_securite.confidentialite.certificat_tls"
    criticite: IMPORTANT

  - id: CONF-006
    description: "Préparation post-quantique"
    niveau_requis: ["RGS***"]
    criteres:
      - "Crypto-agilité (algorithmes paramétrables)"
      - "Plan de migration PQC documenté"
      - "Données >10 ans : chiffrement hybride"
    verification_auto: null
    source_osk: null
    source_manuelle: "Documentation architecture"
    template_reference: "rgs-anssi-cryptography-requirements.md §7"
    criticite: RECOMMANDE
```

---

## 5. Fonction 4 : Traçabilité (Annexe B5)

### 5.1 Exigences par niveau RGS

| Exigence | RGS* | RGS** | RGS*** | Référence |
|----------|------|-------|--------|-----------|
| Logs d'authentification | ✅ | ✅ | ✅ | B5 §2.1 |
| Logs d'accès aux données | ⚠️ | ✅ | ✅ | B5 §2.2 |
| Rétention 3 ans | ✅ | ✅ | ✅ | B5 §3 |
| SIEM centralisé | ⚠️ | ✅ | ✅ | B5 §4 |
| SOC 24/7 | ❌ | ⚠️ | ✅ | B5 §5 |

### 5.2 Checklist d'audit

```yaml
tracabilite:
  - id: TRACE-001
    description: "Événements d'authentification journalisés"
    niveau_requis: ["RGS*", "RGS**", "RGS***"]
    criteres:
      - "AUTH-001 à AUTH-008 implémentés"
      - "Succès et échecs tracés"
      - "IP, timestamp, user_id, user_agent"
    verification_auto: "SEC-*.md (Principe VI) contains 'authentification' AND 'log'"
    source_osk: "docs/security/features/SEC-*.md (Principe VI)"
    source_manuelle: null
    template_reference: "rgs-tracability-requirements.md"
    criticite: BLOQUANT

  - id: TRACE-002
    description: "Événements d'autorisation journalisés"
    niveau_requis: ["RGS*", "RGS**", "RGS***"]
    criteres:
      - "Accès autorisés et refusés tracés"
      - "Élévations de privilèges tracées"
    verification_auto: "SEC-*.md (Principe VI) contains 'autorisation' OR 'RBAC'"
    source_osk: "docs/security/features/SEC-*.md (Principe VI)"
    source_manuelle: null
    template_reference: "rgs-tracability-requirements.md"
    criticite: IMPORTANT

  - id: TRACE-003
    description: "Accès aux données personnelles tracés"
    niveau_requis: ["RGS**", "RGS***"]
    criteres:
      - "DATA-001 à DATA-005 implémentés"
      - "Champs accédés identifiés"
      - "Finalité documentée"
    verification_auto: "SEC-*.md contains 'données personnelles' AND 'log'"
    source_osk: "docs/security/features/SEC-*.md"
    source_manuelle: null
    template_reference: "rgs-tracability-requirements.md"
    criticite: BLOQUANT

  - id: TRACE-004
    description: "Format de log JSON structuré"
    niveau_requis: ["RGS**", "RGS***"]
    criteres:
      - "Format JSON avec champs obligatoires"
      - "timestamp, event.id, actor.user_id, context.trace_id"
      - "Données sensibles masquées"
    verification_auto: "rgs_context.fonctions_securite.tracabilite.format_logs == 'JSON structuré'"
    source_osk: null
    source_manuelle: "rgs-context.yaml#fonctions_securite.tracabilite"
    template_reference: "rgs-tracability-requirements.md §3"
    criticite: IMPORTANT

  - id: TRACE-005
    description: "Rétention minimum 3 ans"
    niveau_requis: ["RGS*", "RGS**", "RGS***"]
    criteres:
      - "Politique ILM configurée (hot/warm/cold)"
      - "Rétention ≥ 1095 jours"
      - "Backup des logs"
    verification_auto: "rgs_context.fonctions_securite.tracabilite.retention_jours >= 1095"
    source_osk: null
    source_manuelle: "rgs-context.yaml#fonctions_securite.tracabilite.retention_jours"
    template_reference: "rgs-tracability-requirements.md §4"
    criticite: BLOQUANT

  - id: TRACE-006
    description: "SIEM centralisé déployé"
    niveau_requis: ["RGS**", "RGS***"]
    criteres:
      - "Collecte centralisée (ELK, Splunk, DataDog)"
      - "Dashboards de sécurité"
      - "Règles d'alerte configurées"
    verification_auto: "rgs_context.fonctions_securite.tracabilite.siem != null"
    source_osk: null
    source_manuelle: "rgs-context.yaml#fonctions_securite.tracabilite.siem"
    template_reference: "rgs-tracability-requirements.md §4"
    criticite: IMPORTANT

  - id: TRACE-007
    description: "Protection des logs (intégrité)"
    niveau_requis: ["RGS**", "RGS***"]
    criteres:
      - "WORM storage ou chaînage cryptographique"
      - "Aucune possibilité de suppression"
      - "Accès aux logs journalisé (meta-logging)"
    verification_auto: "rgs_context.fonctions_securite.tracabilite.protection_logs != null"
    source_osk: null
    source_manuelle: "rgs-context.yaml#fonctions_securite.tracabilite.protection_logs"
    template_reference: "rgs-tracability-requirements.md §6"
    criticite: IMPORTANT

  - id: TRACE-008
    description: "Alertes de sécurité configurées"
    niveau_requis: ["RGS**", "RGS***"]
    criteres:
      - "ALERT-001 à ALERT-008 configurées"
      - "Escalade définie (SOC → RSSI)"
      - "Tests mensuels des alertes"
    verification_auto: null
    source_osk: null
    source_manuelle: "Documentation SIEM"
    template_reference: "rgs-tracability-requirements.md §7"
    criticite: IMPORTANT
```

---

## 6. Analyse de Risques (EBIOS RM)

### 6.1 Checklist d'audit

```yaml
analyse_risques:
  - id: RISK-001
    description: "Analyse EBIOS RM réalisée"
    niveau_requis: ["RGS*", "RGS**", "RGS***"]
    criteres:
      - "5 ateliers EBIOS RM documentés"
      - "Sources de risques identifiées"
      - "Scénarios stratégiques et opérationnels"
    verification_auto: "file_exists(docs/security/rgs/*ebios*) OR risks/risk-register.yaml not empty"
    source_osk: "docs/security/risks/risk-register.yaml"
    source_manuelle: "Document EBIOS RM externe"
    template_reference: "ebios-risk-manager-template.md"
    criticite: BLOQUANT

  - id: RISK-002
    description: "Registre des risques à jour"
    niveau_requis: ["RGS*", "RGS**", "RGS***"]
    criteres:
      - "Tous les risques identifiés et scorés"
      - "Mitigations définies"
      - "Risques résiduels calculés"
    verification_auto: "risk-register.yaml contains risques AND risques.length > 0"
    source_osk: "docs/security/risks/risk-register.yaml"
    source_manuelle: null
    criticite: BLOQUANT

  - id: RISK-003
    description: "Risques résiduels acceptables"
    niveau_requis: ["RGS*", "RGS**", "RGS***"]
    criteres:
      - "Aucun risque résiduel CRITIQUE"
      - "Risques résiduels IMPORTANTS justifiés"
      - "Validation par autorité d'homologation"
    verification_auto: "risk-register.yaml.risques.filter(r => r.severite == 'CRITIQUE' AND r.statut == 'OUVERT').length == 0"
    source_osk: "docs/security/risks/risk-register.yaml"
    source_manuelle: "Décision d'homologation"
    criticite: BLOQUANT

  - id: RISK-004
    description: "Couverture des 7 principes constitutionnels"
    niveau_requis: ["RGS*", "RGS**", "RGS***"]
    criteres:
      - "Au moins un document SEC-*.md par principe"
      - "Conformité ≥ 6/7 principes"
    verification_auto: "count(docs/security/features/SEC-*.md) >= 1 AND risk-register.yaml.conformite_globale >= 85%"
    source_osk: "docs/security/risks/risk-register.yaml#conformite_principes"
    source_manuelle: null
    criticite: IMPORTANT
```

---

## 7. Homologation (Articles 8-11)

### 7.1 Checklist d'audit

```yaml
homologation:
  - id: HOMO-001
    description: "Dossier d'homologation constitué"
    niveau_requis: ["RGS*", "RGS**", "RGS***"]
    criteres:
      - "Toutes les sections obligatoires remplies"
      - "Périmètre clairement défini"
      - "Analyse de risques intégrée"
    verification_auto: null
    source_osk: "Généré par osk-audit rgs"
    source_manuelle: "rgs-homologation-dossier-template.md"
    template_reference: "rgs-homologation-dossier-template.md"
    criticite: BLOQUANT

  - id: HOMO-002
    description: "Autorité d'homologation désignée"
    niveau_requis: ["RGS*", "RGS**", "RGS***"]
    criteres:
      - "Nom et fonction documentés"
      - "Pouvoir de signature confirmé"
    verification_auto: "rgs_context.organisation.autorite_homologation.nom != '[À COMPLÉTER]'"
    source_osk: null
    source_manuelle: "rgs-context.yaml#organisation.autorite_homologation"
    criticite: BLOQUANT

  - id: HOMO-003
    description: "Commission d'homologation prévue"
    niveau_requis: ["RGS**", "RGS***"]
    criteres:
      - "Date de commission planifiée"
      - "Participants identifiés (RSSI, MOA, MOE)"
    verification_auto: "rgs_context.homologation.dates.commission_homologation != '[À COMPLÉTER]'"
    source_osk: null
    source_manuelle: "rgs-context.yaml#homologation.dates"
    criticite: IMPORTANT

  - id: HOMO-004
    description: "Audit de pénétration réalisé"
    niveau_requis: ["RGS**", "RGS***"]
    criteres:
      - "Pentest par prestataire qualifié ANSSI"
      - "Rapport de moins d'un an"
      - "Vulnérabilités critiques corrigées"
    verification_auto: "rgs_context.mcs.pentest.dernier_audit != '[À COMPLÉTER]'"
    source_osk: null
    source_manuelle: "rgs-context.yaml#mcs.pentest + Rapport externe"
    criticite: BLOQUANT

  - id: HOMO-005
    description: "Plan MCS défini"
    niveau_requis: ["RGS*", "RGS**", "RGS***"]
    criteres:
      - "Procédures de revue documentées"
      - "Fréquence des audits définie"
      - "Gestion des vulnérabilités"
    verification_auto: "rgs_context.mcs != null"
    source_osk: null
    source_manuelle: "rgs-context.yaml#mcs"
    template_reference: "rgs-mcs-maintenance.md"
    criticite: IMPORTANT
```

---

## 8. MCS - Maintien en Condition de Sécurité (Article 13)

### 8.1 Checklist d'audit

```yaml
mcs:
  - id: MCS-001
    description: "Revues de sécurité périodiques planifiées"
    niveau_requis: ["RGS*", "RGS**", "RGS***"]
    criteres:
      - "Fréquence définie (trimestrielle pour RGS**)"
      - "Responsable identifié"
      - "Check-list de revue documentée"
    verification_auto: "rgs_context.mcs.revue_securite.frequence != null"
    source_osk: null
    source_manuelle: "rgs-context.yaml#mcs.revue_securite"
    template_reference: "rgs-mcs-maintenance.md"
    criticite: IMPORTANT

  - id: MCS-002
    description: "Gestion des vulnérabilités active"
    niveau_requis: ["RGS*", "RGS**", "RGS***"]
    criteres:
      - "Scan de vulnérabilités régulier (hebdomadaire)"
      - "SLA de correction définis"
      - "Suivi des CVE"
    verification_auto: "SEC-*.md (Principe VII) contains 'SCA' OR 'vulnérabilités'"
    source_osk: "docs/security/features/SEC-*.md (Principe VII)"
    source_manuelle: "rgs-context.yaml#mcs.vulnerabilites"
    criticite: IMPORTANT

  - id: MCS-003
    description: "SLA de correction des vulnérabilités"
    niveau_requis: ["RGS**", "RGS***"]
    criteres:
      - "Critique (CVSS 9-10) : 48h"
      - "Haute (CVSS 7-8.9) : 7 jours"
      - "Moyenne (CVSS 4-6.9) : 30 jours"
    verification_auto: "rgs_context.mcs.vulnerabilites.sla_critique_heures <= 48"
    source_osk: null
    source_manuelle: "rgs-context.yaml#mcs.vulnerabilites"
    criticite: IMPORTANT

  - id: MCS-004
    description: "Procédure de gestion des incidents"
    niveau_requis: ["RGS*", "RGS**", "RGS***"]
    criteres:
      - "Contact d'astreinte défini"
      - "Procédure de notification (ANSSI si OIV/NIS2)"
      - "Délai notification RGPD (72h)"
    verification_auto: "rgs_context.mcs.incidents.contact_astreinte != '[À COMPLÉTER]'"
    source_osk: null
    source_manuelle: "rgs-context.yaml#mcs.incidents"
    template_reference: "rgs-mcs-maintenance.md"
    criticite: IMPORTANT

  - id: MCS-005
    description: "Sensibilisation des utilisateurs"
    niveau_requis: ["RGS*", "RGS**", "RGS***"]
    criteres:
      - "Formation sécurité annuelle"
      - "Campagnes anti-phishing"
      - "Taux de completion > 80%"
    verification_auto: "rgs_context.mcs.sensibilisation.taux_completion != '[À COMPLÉTER]'"
    source_osk: null
    source_manuelle: "rgs-context.yaml#mcs.sensibilisation"
    criticite: RECOMMANDE

  - id: MCS-006
    description: "Ré-homologation planifiée"
    niveau_requis: ["RGS*", "RGS**", "RGS***"]
    criteres:
      - "Durée de validité définie (3 ou 5 ans)"
      - "Date de ré-homologation planifiée"
      - "Rappel 6 mois avant expiration"
    verification_auto: "rgs_context.homologation.duree_validite_annees in [3, 5]"
    source_osk: null
    source_manuelle: "rgs-context.yaml#homologation"
    template_reference: "rgs-mcs-maintenance.md"
    criticite: IMPORTANT
```

---

## 9. Synthèse et Calcul du Score

### 9.1 Pondération des exigences

| Criticité | Poids | Impact sur score |
|-----------|-------|-----------------|
| **BLOQUANT** | 10 | 0% si non conforme |
| **IMPORTANT** | 5 | Réduit le score proportionnellement |
| **RECOMMANDE** | 2 | Impact mineur |

### 9.2 Calcul du score de conformité

```python
def calculer_score_rgs(resultats_audit: dict, niveau_rgs: str) -> dict:
    """
    Calcule le score de conformité RGS

    Args:
        resultats_audit: Résultats de l'audit par exigence
        niveau_rgs: 'RGS*', 'RGS**', ou 'RGS***'

    Returns:
        Score global et détaillé par fonction
    """

    poids = {'BLOQUANT': 10, 'IMPORTANT': 5, 'RECOMMANDE': 2}

    scores = {
        'authentification': {'conforme': 0, 'total': 0},
        'integrite': {'conforme': 0, 'total': 0},
        'confidentialite': {'conforme': 0, 'total': 0},
        'tracabilite': {'conforme': 0, 'total': 0},
        'analyse_risques': {'conforme': 0, 'total': 0},
        'homologation': {'conforme': 0, 'total': 0},
        'mcs': {'conforme': 0, 'total': 0}
    }

    bloquants_non_conformes = []

    for exigence_id, resultat in resultats_audit.items():
        # Filtrer selon niveau RGS
        if niveau_rgs not in resultat['niveau_requis']:
            continue

        categorie = exigence_id.split('-')[0].lower()
        if categorie == 'auth':
            categorie = 'authentification'
        elif categorie == 'int':
            categorie = 'integrite'
        elif categorie == 'conf':
            categorie = 'confidentialite'
        elif categorie == 'trace':
            categorie = 'tracabilite'
        elif categorie == 'risk':
            categorie = 'analyse_risques'
        elif categorie == 'homo':
            categorie = 'homologation'

        poids_exigence = poids[resultat['criticite']]
        scores[categorie]['total'] += poids_exigence

        if resultat['conforme']:
            scores[categorie]['conforme'] += poids_exigence
        elif resultat['criticite'] == 'BLOQUANT':
            bloquants_non_conformes.append(exigence_id)

    # Calcul des scores par fonction
    for cat in scores:
        if scores[cat]['total'] > 0:
            scores[cat]['pourcentage'] = round(
                100 * scores[cat]['conforme'] / scores[cat]['total']
            )
        else:
            scores[cat]['pourcentage'] = 100

    # Score global
    total_conforme = sum(s['conforme'] for s in scores.values())
    total_possible = sum(s['total'] for s in scores.values())
    score_global = round(100 * total_conforme / total_possible) if total_possible > 0 else 0

    # Statut final
    if bloquants_non_conformes:
        statut = 'NON_CONFORME'
    elif score_global >= 90:
        statut = 'CONFORME'
    elif score_global >= 70:
        statut = 'PARTIEL'
    else:
        statut = 'NON_CONFORME'

    return {
        'niveau_rgs': niveau_rgs,
        'score_global': score_global,
        'statut': statut,
        'bloquants_non_conformes': bloquants_non_conformes,
        'scores_par_fonction': scores
    }
```

### 9.3 Format du rapport d'audit

```markdown
# Rapport d'Audit de Conformité RGS

**Système** : [Nom du système]
**Niveau RGS cible** : RGS**
**Date d'audit** : 2025-01-15
**Auditeur** : OpenSecKit `/osk-audit rgs`

---

## Score Global : 85% - PARTIEL

### Statut par Fonction RGS

| Fonction | Score | Statut | Bloquants |
|----------|-------|--------|-----------|
| 1. Authentification (B2) | 90% | ✅ | 0 |
| 2. Intégrité (B3) | 80% | ⚠️ | 0 |
| 3. Confidentialité (B4) | 95% | ✅ | 0 |
| 4. Traçabilité (B5) | 75% | ⚠️ | 0 |
| Analyse de risques | 100% | ✅ | 0 |
| Homologation | 70% | ⚠️ | 1 |
| MCS | 80% | ⚠️ | 0 |

---

## Exigences Bloquantes Non Conformes

### HOMO-004 : Audit de pénétration réalisé
- **Statut** : ❌ NON CONFORME
- **Criticité** : BLOQUANT
- **Constat** : Aucun rapport de pentest disponible
- **Action requise** : Planifier un pentest avec prestataire qualifié ANSSI
- **Échéance recommandée** : Avant commission d'homologation

---

## Exigences Non Conformes (Non Bloquantes)

### TRACE-006 : SIEM centralisé déployé
- **Statut** : ⚠️ PARTIEL
- **Criticité** : IMPORTANT
- **Constat** : SIEM présent mais dashboards non configurés
- **Action requise** : Configurer dashboards de sécurité

---

## Prochaines Étapes

1. ⚠️ **[BLOQUANT]** Réaliser l'audit de pénétration
2. Configurer les dashboards SIEM
3. Compléter le dossier d'homologation
4. Planifier la commission d'homologation

---

## Documents Générés

- `docs/security/audits/AUDIT-RGS-2025-01-15.md` (ce rapport)
- `docs/security/rgs/DOSSIER-HOMOLOGATION-DRAFT.md` (pré-dossier homologation)
```

---

## 10. Références

- **RGS v2.0** : https://cyber.gouv.fr/le-referentiel-general-de-securite-rgs
- **Guide d'homologation ANSSI** : https://cyber.gouv.fr/lhomologation-de-securite
- **Templates OSK** : `domaines/gouvernement-rgs/templates/`

---

**Template Version** : 1.0.0
**Last Updated** : 2025-01-15
**Owner** : RSSI
