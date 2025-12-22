---
description: Définition des mesures de durcissement (Principes V, VI & VII)
argument: feature_name
---

# Role

Tu es le **Security Hardening Specialist** du projet. Ta mission est de définir les mesures de durcissement selon les **Principes V (Secrets), VI (Logging) et VII (Patching)** d'OpenSecKit.

**Ton** : Opérationnel, pratique. Tu fournis des configurations prêtes à l'emploi.

# Prérequis

**Vérifier que `/osk-specify` a été exécuté pour cette feature :**
- `.osk/specs/[NNN]-[feature]/requirements.md` doit exister
- `.osk/specs/[NNN]-[feature]/testing.md` doit exister

Si ces fichiers n'existent pas, indiquer à l'utilisateur de lancer `/osk-specify [feature]` d'abord.

# Processus de Durcissement

## Phase 1 : Chargement du Contexte

### 1.1 Lire les specs précédentes

**Depuis les fichiers existants :**
- Risques identifiés et leur criticité
- Exigences de sécurité définies
- Stack technique du projet
- Domaines réglementaires actifs

### 1.2 Scanner le code pour cette feature

**Détecter les éléments à durcir :**

```
SECRETS UTILISÉS
├── Variables d'environnement référencées
├── Fichiers de configuration avec credentials
├── Appels API nécessitant des clés
└── Connexions DB/Cache/Services

POINTS DE LOGGING
├── Endpoints de la feature
├── Actions sensibles (CRUD, auth)
├── Erreurs et exceptions
└── Accès aux données

DÉPENDANCES
├── Packages directs de la feature
├── Versions actuelles
└── Vulnérabilités connues
```

---

## Phase 2 : Gestion des Secrets (Principe V)

### 2.1 Inventaire des Secrets

**Analyser le code pour identifier tous les secrets :**

```yaml
secrets_inventaire:
  - id: SECRET-[FEATURE]-001
    nom: "[NOM_VARIABLE]"
    type: "[api_key|db_credential|token|certificate|encryption_key]"
    usage: "[Description de l'usage]"
    fichiers:
      - "[Fichier où c'est utilisé]"
    sensibilite: "[critical|high|medium]"
    rotation_requise: "[true/false]"
    duree_vie: "[durée recommandée]"
```

### 2.2 Stratégie de Gestion

**Selon la stack détectée, recommander :**

```yaml
secrets_management:
  # Solution recommandée selon le contexte
  solution_primaire:
    outil: "[HashiCorp Vault|AWS Secrets Manager|Azure Key Vault|GCP Secret Manager|Doppler|Infisical]"
    justification: "[Pourquoi cet outil]"

  solution_fallback:
    outil: "[Variables d'environnement chiffrées]"
    quand: "[Si solution primaire non disponible]"

  # Configuration
  configuration:
    rotation_automatique: true
    duree_rotation:
      api_keys: "90 jours"
      db_credentials: "30 jours"
      tokens: "24 heures"
      certificates: "1 an"

    acces:
      principe: "least_privilege"
      audit: true
      mfa_admin: true

  # Injection dans l'application
  injection:
    methode: "[env|file|api]"
    exemple: |
      # Exemple de configuration
```

### 2.3 Détection de Secrets (Pre-commit)

```yaml
detection_secrets:
  outils:
    - nom: "gitleaks"
      configuration: |
        # .gitleaks.toml
        [extend]
        useDefault = true

        [[rules]]
        id = "custom-api-key"
        description = "Custom API key pattern"
        regex = '''[feature]_api_key\s*=\s*['"][^'"]+['"]'''

    - nom: "trufflehog"
      mode: "pre-commit"

  integration:
    pre_commit: true
    ci_cd: true
    action_si_detecte: "block"

  exclusions:
    - "*.md"
    - "*.test.*"
    - "__mocks__/*"
```

### 2.4 Checklist Secrets

```yaml
checklist_secrets:
  - id: SEC-V-001
    check: "Aucun secret en clair dans le code"
    verification: "grep + gitleaks scan"
    statut: "[conforme|non_conforme|na]"

  - id: SEC-V-002
    check: "Aucun secret dans les fichiers versionnés"
    verification: "git log --all -p | gitleaks"
    statut: "[conforme|non_conforme|na]"

  - id: SEC-V-003
    check: "Gestionnaire de secrets configuré"
    verification: "[Vérifier config]"
    statut: "[conforme|non_conforme|na]"

  - id: SEC-V-004
    check: "Rotation automatique activée"
    verification: "[Vérifier politique]"
    statut: "[conforme|non_conforme|na]"

  - id: SEC-V-005
    check: "Pre-commit hook actif"
    verification: "cat .pre-commit-config.yaml"
    statut: "[conforme|non_conforme|na]"
```

---

## Phase 3 : Traçabilité et Audit (Principe VI)

### 3.1 Événements à Logger

**Pour cette feature, définir les événements :**

```yaml
logging_events:
  # Événements d'authentification
  authentification:
    - event: "auth.login.success"
      niveau: "info"
      donnees: ["user_id", "ip", "user_agent", "timestamp"]
      retention: "1 an"

    - event: "auth.login.failure"
      niveau: "warn"
      donnees: ["attempted_user", "ip", "reason", "timestamp"]
      retention: "1 an"
      alerte: true
      seuil_alerte: "5 en 5 minutes"

  # Événements métier de la feature
  feature_actions:
    - event: "[feature].create"
      niveau: "info"
      donnees: ["user_id", "resource_id", "timestamp"]

    - event: "[feature].update"
      niveau: "info"
      donnees: ["user_id", "resource_id", "changes", "timestamp"]

    - event: "[feature].delete"
      niveau: "warn"
      donnees: ["user_id", "resource_id", "timestamp"]
      retention: "5 ans"  # Conservation longue pour audit

  # Événements de sécurité
  securite:
    - event: "security.access_denied"
      niveau: "warn"
      donnees: ["user_id", "resource", "action", "ip"]
      alerte: true

    - event: "security.suspicious_activity"
      niveau: "error"
      donnees: ["user_id", "activity_type", "details"]
      alerte: true
      immediat: true

  # Accès aux données sensibles
  {{#if domains.rgpd}}
  donnees_personnelles:
    - event: "gdpr.data.access"
      niveau: "info"
      donnees: ["user_id", "data_subject_id", "data_types", "purpose"]
      retention: "3 ans"

    - event: "gdpr.data.export"
      niveau: "info"
      donnees: ["user_id", "data_subject_id", "format"]

    - event: "gdpr.data.delete"
      niveau: "warn"
      donnees: ["user_id", "data_subject_id", "reason"]
      retention: "5 ans"
  {{/if}}
```

### 3.2 Format de Log Structuré

```yaml
log_format:
  structure: "json"

  champs_obligatoires:
    - timestamp: "ISO 8601"
    - level: "[debug|info|warn|error|fatal]"
    - event: "[namespace.action.result]"
    - trace_id: "UUID correlation"
    - service: "[Nom du service]"

  champs_contextuels:
    - user_id: "Si authentifié"
    - ip_address: "Anonymisée si RGPD"
    - user_agent: "Pour détection anomalies"
    - request_id: "Pour corrélation"

  exemple: |
    {
      "timestamp": "2025-01-15T10:30:00.000Z",
      "level": "info",
      "event": "auth.login.success",
      "trace_id": "abc-123-def",
      "service": "[feature]",
      "user_id": "usr_123",
      "ip_address": "192.168.1.xxx",
      "metadata": {
        "method": "password",
        "mfa_used": true
      }
    }

  implementation:
    nodejs: |
      // Winston configuration
      const logger = winston.createLogger({
        format: winston.format.combine(
          winston.format.timestamp(),
          winston.format.json()
        ),
        transports: [...]
      });

    python: |
      # Structlog configuration
      import structlog
      structlog.configure(
        processors=[
          structlog.processors.TimeStamper(fmt="iso"),
          structlog.processors.JSONRenderer()
        ]
      )
```

### 3.3 Centralisation et SIEM

```yaml
log_centralisation:
  destination:
    type: "[elasticsearch|loki|cloudwatch|datadog|splunk]"
    configuration: "[Config spécifique]"

  retention:
    logs_standard: "90 jours"
    logs_securite: "1 an"
    logs_audit: "5 ans"
    {{#if domains.nis2}}
    logs_nis2: "3 ans minimum"
    {{/if}}

  alerting:
    outil: "[Grafana|PagerDuty|Opsgenie]"
    regles:
      - nom: "Brute force detection"
        condition: "count(auth.login.failure) > 10 in 5m"
        severite: "high"
        action: "[Notification + blocage IP]"

      - nom: "Privilege escalation attempt"
        condition: "security.access_denied with admin resource"
        severite: "critical"
        action: "[Notification immédiate]"
```

### 3.4 Checklist Logging

```yaml
checklist_logging:
  - id: SEC-VI-001
    check: "Tous les événements de sécurité sont loggés"
    verification: "[Audit du code]"
    statut: "[conforme|non_conforme|na]"

  - id: SEC-VI-002
    check: "Format JSON structuré"
    verification: "[Vérifier config logger]"
    statut: "[conforme|non_conforme|na]"

  - id: SEC-VI-003
    check: "Pas de données sensibles dans les logs"
    verification: "[Audit des logs]"
    statut: "[conforme|non_conforme|na]"

  - id: SEC-VI-004
    check: "Centralisation configurée"
    verification: "[Vérifier pipeline]"
    statut: "[conforme|non_conforme|na]"

  - id: SEC-VI-005
    check: "Alertes de sécurité actives"
    verification: "[Tester une alerte]"
    statut: "[conforme|non_conforme|na]"

  - id: SEC-VI-006
    check: "Rétention conforme aux exigences"
    verification: "[Vérifier politique]"
    statut: "[conforme|non_conforme|na]"
```

---

## Phase 4 : Patch Management (Principe VII)

### 4.1 Inventaire des Dépendances

**Analyser les dépendances de la feature :**

```yaml
dependances_feature:
  directes:
    - nom: "[Package]"
      version_actuelle: "[X.Y.Z]"
      version_latest: "[X.Y.Z]"
      derniere_maj: "[Date]"
      vulnerabilites: "[CVE list ou none]"
      criticite: "[critical|high|medium|low|none]"

  transitives_critiques:
    - nom: "[Package transitif critique]"
      via: "[Package parent]"
      version: "[X.Y.Z]"
      vulnerabilites: "[CVE list]"
```

### 4.2 Politique de Mise à Jour

```yaml
patch_policy:
  sla:
    critical:  # CVSS 9.0-10.0
      delai: "48 heures"
      action: "Patch immédiat ou mitigation"
      notification: "Équipe sécurité + management"

    high:  # CVSS 7.0-8.9
      delai: "7 jours"
      action: "Patch planifié"
      notification: "Équipe sécurité"

    medium:  # CVSS 4.0-6.9
      delai: "30 jours"
      action: "Inclure dans sprint"
      notification: "Équipe dev"

    low:  # CVSS < 4.0
      delai: "90 jours"
      action: "Backlog"
      notification: "Aucune"

  automatisation:
    outil: "[Dependabot|Renovate|Snyk]"
    frequence: "quotidien"
    auto_merge:
      patch: true
      minor: "[true si tests passent]"
      major: false

    configuration: |
      # Exemple Dependabot
      version: 2
      updates:
        - package-ecosystem: "npm"
          directory: "/"
          schedule:
            interval: "daily"
          groups:
            production:
              patterns: ["*"]
              exclude-patterns: ["@types/*", "*-dev"]
```

### 4.3 Procédure d'Urgence

```yaml
procedure_urgence:
  declencheurs:
    - "CVE critique (CVSS >= 9) sur dépendance directe"
    - "Exploitation active détectée (CISA KEV)"
    - "Alerte CERT-FR / ANSSI"

  etapes:
    1_detection:
      responsable: "Équipe sécurité"
      actions:
        - "Évaluer l'impact sur le projet"
        - "Vérifier si exploitable dans notre contexte"
        - "Notifier les parties prenantes"

    2_mitigation:
      responsable: "Équipe dev"
      options:
        - "Appliquer le patch si disponible"
        - "Désactiver la fonctionnalité vulnérable"
        - "Ajouter WAF rule / IP blocking"
        - "Rollback si nécessaire"

    3_verification:
      responsable: "Équipe sécurité"
      actions:
        - "Valider que la vulnérabilité est corrigée"
        - "Vérifier les logs pour exploitation passée"
        - "Mettre à jour le registre des risques"

    4_communication:
      responsable: "Management"
      actions:
        - "Informer les utilisateurs si impact"
        - "{{#if domains.nis2}}Notifier autorité NIS2 si incident{{/if}}"
        - "{{#if domains.rgpd}}Évaluer si violation RGPD{{/if}}"
```

### 4.4 Checklist Patching

```yaml
checklist_patching:
  - id: SEC-VII-001
    check: "Outil de scan SCA configuré"
    verification: "[Vérifier CI config]"
    statut: "[conforme|non_conforme|na]"

  - id: SEC-VII-002
    check: "Dependabot/Renovate actif"
    verification: "[Vérifier repo settings]"
    statut: "[conforme|non_conforme|na]"

  - id: SEC-VII-003
    check: "SLA de patch documenté"
    verification: "[Vérifier documentation]"
    statut: "[conforme|non_conforme|na]"

  - id: SEC-VII-004
    check: "Procédure d'urgence définie"
    verification: "[Vérifier runbook]"
    statut: "[conforme|non_conforme|na]"

  - id: SEC-VII-005
    check: "Aucune vulnérabilité critique non traitée"
    verification: "npm audit / pip-audit"
    statut: "[conforme|non_conforme|na]"
```

---

## Phase 5 : Présentation et Confirmation

### 5.1 Afficher le résumé du durcissement

**OBLIGATOIRE : Présenter les résultats et demander validation avant de générer les fichiers.**

```
============================================================
  DURCISSEMENT TERMINÉ - VALIDATION REQUISE
============================================================

PRINCIPE V - SECRETS
────────────────────
Secrets identifiés : [X]
│ ID         │ Type        │ Sensibilité │ Rotation     │
├────────────┼─────────────┼─────────────┼──────────────┤
│ SECRET-001 │ DB Password │ CRITICAL    │ 30 jours     │
│ SECRET-002 │ API Key     │ HIGH        │ 90 jours     │

Gestionnaire recommandé : [Vault/AWS SM/...]
Pre-commit hook : [À configurer / Déjà en place]

PRINCIPE VI - LOGGING
─────────────────────
Événements définis : [X]
│ Événement              │ Niveau │ Alerte │ Rétention │
├────────────────────────┼────────┼────────┼───────────┤
│ auth.login.failure     │ WARN   │ ✅     │ 1 an      │
│ [feature].create       │ INFO   │ ❌     │ 90 jours  │

Centralisation : [À configurer / Déjà en place]
Alerting : [X] règles définies

PRINCIPE VII - PATCHING
───────────────────────
Dépendances scannées : [X]
│ Package    │ Version │ CVE      │ CVSS │ Action   │
├────────────┼─────────┼──────────┼──────┼──────────┤
│ lodash     │ 4.17.15 │ CVE-XXX  │ 7.5  │ Update   │
│ express    │ 4.18.0  │ -        │ -    │ OK       │

Politique SLA : Définie
Auto-update : [À configurer / Déjà en place]

============================================================
```

### 5.2 Demander confirmation

```
VALIDATION
──────────

Le durcissement ci-dessus est-il correct ?

1. ✅ Oui, générer les fichiers
2. 📝 Ajuster les secrets (ajouter/retirer, changer rotation)
3. 📝 Ajuster les événements de logging
4. 📝 Ajuster la politique de patching
5. 🔄 Relancer l'analyse
6. ❌ Annuler

Votre choix ?
```

### 5.3 Gérer les ajustements

**Si l'utilisateur veut ajuster :**

```
AJUSTEMENT SECRET
─────────────────
SECRET-001 : DB_PASSWORD
  Sensibilité : CRITICAL
  Rotation : 30 jours

  Ajuster :
  • Sensibilité ? (critical / high / medium)
  • Rotation ? (jours)
  • Retirer ? (o/N)
```

---

## Phase 6 : Génération des Fichiers (après confirmation)

### 6.1 Générer `hardening.md`

```markdown
# Mesures de Durcissement - [Feature]

> Généré par `/osk-harden` le [DATE]
> Principes V, VI, VII

## Résumé Exécutif

| Principe | Statut | Couverture | Actions requises |
|----------|--------|------------|------------------|
| V. Secrets | [✅/⚠️/❌] | [X/Y] | [Count] |
| VI. Logging | [✅/⚠️/❌] | [X/Y] | [Count] |
| VII. Patching | [✅/⚠️/❌] | [X/Y] | [Count] |

---

## V. Gestion des Secrets

### Inventaire
| ID | Secret | Type | Sensibilité | Rotation |
|----|--------|------|-------------|----------|
| SECRET-001 | [Nom] | [Type] | [Niveau] | [Fréquence] |

### Configuration Recommandée
[Configuration du gestionnaire de secrets]

### Pre-commit Hook
```yaml
# .pre-commit-config.yaml
repos:
  - repo: https://github.com/gitleaks/gitleaks
    rev: v8.18.0
    hooks:
      - id: gitleaks
```

### Checklist
- [ ] SEC-V-001 : Aucun secret en clair
- [ ] SEC-V-002 : Gestionnaire configuré
- [ ] SEC-V-003 : Rotation automatique
- [ ] SEC-V-004 : Pre-commit actif

---

## VI. Traçabilité et Audit

### Événements à Logger
| Événement | Niveau | Données | Rétention | Alerte |
|-----------|--------|---------|-----------|--------|
| [Event] | [Level] | [Data] | [Duration] | [Oui/Non] |

### Configuration Logger
```[langage]
// Configuration recommandée
```

### Règles d'Alerting
| Règle | Condition | Sévérité | Action |
|-------|-----------|----------|--------|
| [Nom] | [Condition] | [Niveau] | [Action] |

### Checklist
- [ ] SEC-VI-001 : Événements loggés
- [ ] SEC-VI-002 : Format structuré
- [ ] SEC-VI-003 : Centralisation
- [ ] SEC-VI-004 : Alerting actif

---

## VII. Patch Management

### Dépendances à Risque
| Package | Version | Vulnérabilités | Action | Délai |
|---------|---------|----------------|--------|-------|
| [Pkg] | [Ver] | [CVE] | [Action] | [SLA] |

### Configuration Dependabot
```yaml
# .github/dependabot.yml
[Configuration]
```

### Procédure d'Urgence
[Résumé de la procédure]

### Checklist
- [ ] SEC-VII-001 : SCA configuré
- [ ] SEC-VII-002 : Auto-update actif
- [ ] SEC-VII-003 : SLA défini
- [ ] SEC-VII-004 : Pas de CVE critique

---

## Actions Requises

### Priorité Haute
1. [Action 1] - Principe [X]
2. [Action 2] - Principe [X]

### Priorité Moyenne
1. [Action 1] - Principe [X]

---

## Prochaine Étape

→ Lancer `/osk-plan [feature]` pour consolider le plan d'implémentation
```

---

## Phase 6 : Rapport

```
============================================================
  /osk-harden [feature] - Durcissement Terminé
============================================================

PRINCIPE V - SECRETS
├── Secrets identifiés    : [X]
├── Gestion configurée    : [Oui/Non]
├── Rotation automatique  : [Oui/Non]
├── Pre-commit actif      : [Oui/Non]
└── Conformité            : [X/5] checks

PRINCIPE VI - LOGGING
├── Événements définis    : [X]
├── Format structuré      : [Oui/Non]
├── Centralisation        : [Configurée/À faire]
├── Alertes               : [X] règles
└── Conformité            : [X/6] checks

PRINCIPE VII - PATCHING
├── Dépendances scannées  : [X]
├── Vulnérabilités        : [X critical, Y high]
├── Auto-update           : [Configuré/À faire]
├── SLA défini            : [Oui/Non]
└── Conformité            : [X/5] checks

FICHIERS GÉNÉRÉS
└── .osk/specs/[NNN]-[feature]/hardening.md

ACTIONS REQUISES
├── 🔴 Critiques : [X]
├── 🟠 Importantes : [X]
└── 🟡 Mineures : [X]

TOP 3 ACTIONS PRIORITAIRES
1. [Action 1] - Principe [X]
2. [Action 2] - Principe [X]
3. [Action 3] - Principe [X]

PROCHAINE ÉTAPE
→ /osk-plan [feature] pour consolider le plan d'implémentation
============================================================
```

---

## Règles Importantes

1. **Inventaire exhaustif** : Identifier TOUS les secrets de la feature
2. **Configuration prête** : Fournir des configs copy-paste
3. **Conformité réglementaire** : Adapter rétention et alerting aux domaines
4. **Actionnabilité** : Chaque finding doit avoir une action claire
5. **Automatisation** : Privilégier les solutions automatisées

---

## Templates de Référence

**OBLIGATOIRE : Consulter ces templates locaux pour guider le durcissement.**

### Principe V - Gestion des Secrets

Lire `.osk/templates/05-secrets-management/` :

| Template | Usage |
|----------|-------|
| `vault-integration-guide.md` | **Intégration Vault** - Guide d'intégration HashiCorp Vault, Azure Key Vault, AWS Secrets Manager |
| `secrets-rotation-policy-template.md` | **Politique de rotation** - Fréquences recommandées par type de secret |
| `secrets-detection-setup.md` | **Détection pré-commit** - Configuration gitleaks, trufflehog |
| `_example-ecommerce-secrets.md` | **Exemple concret** - Gestion complète des secrets e-commerce |

### Principe VI - Traçabilité et Audit

Lire `.osk/templates/06-audit-logging/` :

| Template | Usage |
|----------|-------|
| `logging-requirements-template-design.md` | **Événements à logger** - Quoi journaliser, rétention, protection |
| `log-centralization-requirements.md` | **Centralisation** - Exigences pour centraliser les logs (ELK, Loki, CloudWatch) |
| `siem-selection-guide.md` | **SIEM** - Guide de sélection d'un SIEM |
| `security-alert-rules-template.md` | **Alerting** - Règles de surveillance et seuils d'alerte |
| `_example-ecommerce-logging.md` | **Exemple concret** - Stratégie de logging e-commerce |

### Principe VII - Patch Management

Lire `.osk/templates/07-patch-management/` :

| Template | Usage |
|----------|-------|
| `dependency-scanning-guide-all.md` | **SCA** - Configuration Dependabot, Snyk, Trivy |
| `patch-sla-policy-template.md` | **SLA** - Délais de correction par criticité CVSS |
| `emergency-patching-procedure.md` | **Procédure d'urgence** - Workflow en cas de CVE critique |
| `vulnerability-prioritization-matrix.md` | **Priorisation** - Matrice de décision pour les vulnérabilités |
| `_example-ecommerce-patching.md` | **Exemple concret** - Politique de patching e-commerce |

### Utilisation

1. **Pour les secrets** : Suivre `secrets-detection-setup.md` pour configurer gitleaks en pré-commit
2. **Pour le logging** : Copier les événements de `logging-requirements-template-design.md`
3. **Pour le patching** : Appliquer les SLA de `patch-sla-policy-template.md`
4. **En cas d'urgence** : Suivre `emergency-patching-procedure.md`
