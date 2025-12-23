# OpenSecKit V3 - Structure des Fichiers

> Ce document décrit l'organisation des fichiers générés par OpenSecKit.
> Il est essentiel pour comprendre où trouver chaque type d'information.

---

## Principe Fondamental

OpenSecKit utilise **deux espaces de stockage distincts** avec des rôles différents :

| Espace | Rôle | Versionné ? | Partageable ? |
|--------|------|-------------|---------------|
| `.osk/` | Fichiers de **travail internes** | Optionnel | Non (interne équipe) |
| `docs/security/` | Documentation **finale officielle** | Oui | Oui (audits, clients) |

---

## Vue d'Ensemble

```
projet/
│
├── .osk/                           ← ESPACE DE TRAVAIL INTERNE
│   ├── config.toml                 ← Configuration centrale (source de vérité)
│   ├── memory/                     ← Mémoire contextuelle du projet
│   │   ├── context.md              ← Faits techniques détectés
│   │   └── constitution.md         ← Principes et exigences adaptés
│   ├── specs/                      ← Spécifications par feature (brouillons)
│   │   └── NNN-feature-name/
│   │       ├── threats.md          ← Analyse STRIDE
│   │       ├── risks.md            ← Risques identifiés
│   │       ├── requirements.md     ← Exigences de sécurité
│   │       ├── testing.md          ← Stratégie de tests
│   │       ├── hardening.md        ← Mesures de durcissement
│   │       ├── plan.md             ← Plan d'implémentation
│   │       ├── rgpd/               ← Brouillons RGPD (consolidés par /osk-rgpd)
│   │       │   └── dpia.md         ← DPIA de la feature
│   │       └── rgs/                ← Brouillons RGS (consolidés par /osk-rgs)
│   │           └── ebios.md        ← Analyse EBIOS de la feature
│   ├── templates/                  ← Templates téléchargés
│   └── prompts/                    ← Prompts personnalisés
│
├── docs/security/                  ← DOCUMENTATION FINALE OFFICIELLE
│   ├── risks/
│   │   └── risk-register.yaml      ← Registre central des risques
│   ├── rgpd/                       ← Documents RGPD finaux
│   │   ├── registre-traitements.md ← Registre Art. 30
│   │   ├── dpia-global.md          ← DPIA consolidé
│   │   ├── procedure-violation.md  ← Procédure Art. 33-34
│   │   ├── politique-conservation.md
│   │   └── AUDIT-YYYY-MM-DD.md     ← Rapports d'audit
│   ├── rgs/                        ← Documents RGS finaux
│   │   ├── EBIOS-RM-[SYSTEME].md   ← Analyse de risques EBIOS
│   │   ├── DOSSIER-HOMOLOGATION-[SYSTEME].md
│   │   ├── MCS-[SYSTEME].md        ← Maintien en Condition de Sécurité
│   │   └── RENOUVELLEMENT-*.md     ← Rapports de renouvellement
│   └── continuity/                 ← Plans de continuité
│       ├── PCA-*.md                ← Plan de Continuité d'Activité
│       └── PRA-*.md                ← Plan de Reprise d'Activité
│
└── .claude/commands/               ← Slash commands Claude Code
    └── osk-*.md                    ← Commandes installées
```

---

## Détail des Espaces

### `.osk/` - Espace de Travail Interne

Cet espace contient les fichiers de travail utilisés par les commandes OSK.
Ces fichiers peuvent être incomplets, en cours d'édition, ou temporaires.

#### `.osk/config.toml` - Configuration Centrale

**C'est la source de vérité unique du projet.**

```toml
# Informations projet
[project]
name = "MonProjet"
description = "..."

# Stack technique détectée
[stack]
detected = ["nodejs", "typescript", "postgresql"]

# Domaines réglementaires
[domains]
active = ["rgpd", "rgs"]

[domains.organisation]      # Partagé RGPD + RGS
nom = "..."
siret = "..."

[[domains.suppliers]]       # Partagé RGPD Art. 28 + RGS Supply Chain
nom = "OVH"
type = "hebergement"
dpa_signe = true

[domains.rgpd]              # Configuration RGPD
enabled = true
niveau = "sensible"

[domains.rgs]               # Configuration RGS
enabled = true
niveau = "renforce"

# Pondération des 7 principes
[principles]
threat_modeling = "high"
# ...
```

**Alimenté par** : `osk init`, `/osk-configure`, `/osk-rgpd`, `/osk-rgs`

#### `.osk/memory/` - Mémoire Contextuelle

| Fichier | Contenu | Généré par |
|---------|---------|------------|
| `context.md` | Faits techniques : stack, données détectées, patterns existants | `/osk-configure` |
| `constitution.md` | Principes pondérés, exigences par domaine, règles projet | `/osk-configure` |

**Usage** : Lu par toutes les commandes pour adapter leurs analyses au contexte.

#### `.osk/specs/NNN-feature/` - Spécifications par Feature

Chaque feature analysée a son propre dossier avec un numéro auto-incrémenté.

| Fichier/Dossier | Contenu | Généré par |
|-----------------|---------|------------|
| `threats.md` | Analyse STRIDE, DFD, attack trees | `/osk-analyze` |
| `risks.md` | Risques scorés, contrôles requis | `/osk-analyze` |
| `requirements.md` | Exigences de sécurité (MUST/SHOULD/MAY) | `/osk-specify` |
| `testing.md` | Stratégie SAST/DAST/SCA | `/osk-specify` |
| `hardening.md` | Secrets, logging, patching | `/osk-harden` |
| `plan.md` | Plan d'implémentation consolidé | `/osk-plan` |
| `rgpd/dpia.md` | DPIA brouillon de la feature | `/osk-analyze` |
| `rgs/ebios.md` | Analyse EBIOS brouillon de la feature | `/osk-analyze` |

**Important** : Ces fichiers sont des **brouillons de travail**.
Les documents finaux sont consolidés dans `docs/security/` :
- Les `rgpd/dpia.md` → consolidés dans `docs/security/rgpd/dpia-global.md`
- Les `rgs/ebios.md` → consolidés dans `docs/security/rgs/EBIOS-RM-[SYSTEME].md`

---

### `docs/security/` - Documentation Finale

Cet espace contient la documentation **officielle et publiable**.
Ces fichiers sont prêts à être :
- Versionnés dans Git
- Partagés avec des auditeurs
- Présentés en commission d'homologation
- Fournis aux clients/partenaires

#### `docs/security/risks/` - Registre des Risques

| Fichier | Contenu | Alimenté par |
|---------|---------|--------------|
| `risk-register.yaml` | Registre central consolidé de tous les risques | `/osk-analyze`, `/osk-baseline` |

**Structure du registre** :
```yaml
metadata:
  projet: "MonProjet"
  derniere_maj: "2025-01-15"

stats:
  total: 25
  critiques: 3
  importants: 8
  mineurs: 14

risques:
  - id: "RISK-AUTH-001"
    titre: "Injection SQL"
    severite: "CRITIQUE"
    feature: "authentication"
    # ...
```

#### `docs/security/rgpd/` - Documents RGPD

| Fichier | Article RGPD | Généré par |
|---------|--------------|------------|
| `registre-traitements.md` | Art. 30 | `/osk-rgpd` |
| `dpia-global.md` | Art. 35 | `/osk-rgpd` (consolide les DPIA des specs) |
| `procedure-violation.md` | Art. 33-34 | `/osk-rgpd` |
| `politique-conservation.md` | Art. 5 | `/osk-rgpd` |
| `mentions-legales.md` | Art. 13-14 | `/osk-rgpd` |
| `AUDIT-YYYY-MM-DD.md` | - | `/osk-rgpd audit` |

#### `docs/security/rgs/` - Documents RGS

| Fichier | Usage | Généré par |
|---------|-------|------------|
| `EBIOS-RM-[SYSTEME].md` | Analyse de risques (5 ateliers) | `/osk-rgs` (consolide les risques des specs) |
| `DOSSIER-HOMOLOGATION-[SYSTEME].md` | Pour commission d'homologation | `/osk-rgs` |
| `MCS-[SYSTEME].md` | Maintien en Condition de Sécurité | `/osk-rgs` |
| `RENOUVELLEMENT-*.md` | Rapport de ré-homologation | `/osk-rgs renew` |

---

## Flux de Données

```
                    TRAVAIL INTERNE                    DOCUMENTATION FINALE
                    ───────────────                    ────────────────────

                    .osk/specs/NNN-feature/
                    ┌─────────────────────┐
/osk-analyze   ───▶ │ threats.md          │
                    │ risks.md            │ ───────┐
                    └─────────────────────┘        │
                                                   │
/osk-specify   ───▶ ┌─────────────────────┐        │
                    │ requirements.md     │        │
                    │ testing.md          │        │
                    └─────────────────────┘        │
                                                   │
/osk-harden    ───▶ ┌─────────────────────┐        │
                    │ hardening.md        │        │
                    └─────────────────────┘        │
                                                   │
                                                   ▼
                                        docs/security/risks/
                                        ┌─────────────────────┐
                                        │ risk-register.yaml  │ ◀── Consolidation
                                        └─────────────────────┘     automatique
                                                   │
                    ┌──────────────────────────────┼──────────────────────────────┐
                    │                              │                              │
                    ▼                              ▼                              ▼
          docs/security/rgpd/          docs/security/rgs/           docs/security/
          ┌──────────────────┐         ┌──────────────────┐         continuity/
/osk-rgpd │ registre.md      │ /osk-rgs│ EBIOS-RM.md      │         ┌────────────┐
     ───▶ │ dpia-global.md   │    ───▶ │ DOSSIER-HOMOL.md │         │ PCA.md     │
          │ procedure.md     │         │ MCS.md           │         │ PRA.md     │
          └──────────────────┘         └──────────────────┘         └────────────┘
```

---

## Commandes et Fichiers Générés

| Commande | Lit depuis | Écrit dans `.osk/` | Écrit dans `docs/security/` |
|----------|------------|--------------------|-----------------------------|
| `osk init` | - | `config.toml` (base) | - |
| `/osk-configure` | Code source | `config.toml`, `memory/*` | - |
| `/osk-baseline` | Code existant | - | `risks/risk-register.yaml` |
| `/osk-analyze` | `memory/*`, code | `specs/NNN/threats.md`, `risks.md` | `risks/risk-register.yaml` (append) |
| `/osk-specify` | `specs/NNN/*` | `specs/NNN/requirements.md`, `testing.md` | - |
| `/osk-harden` | `specs/NNN/*` | `specs/NNN/hardening.md` | - |
| `/osk-plan` | `specs/NNN/*` | `specs/NNN/plan.md` | - |
| `/osk-rgpd` | `config.toml`, `specs/*/` | `config.toml` (enrichi) | `rgpd/*` |
| `/osk-rgpd audit` | `config.toml`, `rgpd/*` | - | `rgpd/AUDIT-*.md` |
| `/osk-rgs` | `config.toml`, `specs/*/` | `config.toml` (enrichi) | `rgs/*` |
| `/osk-rgs renew` | `config.toml`, `rgs/*` | `config.toml` | `rgs/RENOUVELLEMENT-*.md` |
| `/osk-dashboard` | `risk-register.yaml`, `config.toml` | - | `DASHBOARD.md` |
| `/osk-pca-pra` | `config.toml`, `memory/*` | - | `continuity/PCA-*.md`, `PRA-*.md` |

---

## Bonnes Pratiques

### Que versionner dans Git ?

```gitignore
# .gitignore recommandé

# NE PAS ignorer - Documentation officielle
# docs/security/  ← À versionner !

# Optionnel - Fichiers de travail
# .osk/          ← Peut être versionné ou ignoré selon les préférences
```

**Recommandation** :
- `docs/security/` : **Toujours versionner** (documentation officielle)
- `.osk/config.toml` : **Versionner** (configuration partagée)
- `.osk/memory/` : **Versionner** (contexte du projet)
- `.osk/specs/` : **Au choix** (brouillons, peuvent être regénérés)

### Quand utiliser quoi ?

| Besoin | Fichier à consulter |
|--------|---------------------|
| Voir la config du projet | `.osk/config.toml` |
| Comprendre le contexte technique | `.osk/memory/context.md` |
| Voir les exigences d'une feature | `.osk/specs/NNN-feature/requirements.md` |
| Présenter les risques à un auditeur | `docs/security/risks/risk-register.yaml` |
| Fournir le registre RGPD au DPO | `docs/security/rgpd/registre-traitements.md` |
| Présenter en commission RGS | `docs/security/rgs/DOSSIER-HOMOLOGATION-*.md` |

---

## Migration depuis V2

Si vous avez des fichiers de l'ancienne structure :

| Ancien fichier | Nouveau emplacement |
|----------------|---------------------|
| `.osk/rgs-context.yaml` | `.osk/config.toml` [domains.rgs] |
| `docs/security/features/SEC-*.md` | `.osk/specs/NNN-*/` (brouillons) + `docs/security/risks/risk-register.yaml` (consolidé) |

---

## Questions Fréquentes

**Q: Pourquoi deux espaces séparés ?**
A: Pour distinguer le travail en cours (`.osk/`) de la documentation officielle (`docs/security/`). Les brouillons peuvent être incomplets, les documents finaux sont validés et partageables.

**Q: Les fichiers `.osk/specs/` sont-ils inutiles ?**
A: Non, ils servent de brouillons détaillés par feature. Les commandes `/osk-rgpd` et `/osk-rgs` les consolident en documents finaux.

**Q: Puis-je éditer manuellement les fichiers ?**
A: Oui, tous les fichiers sont en texte (TOML, Markdown, YAML). Les commandes OSK détectent les modifications manuelles.

**Q: Comment régénérer un document final ?**
A: Relancez la commande correspondante (`/osk-rgpd`, `/osk-rgs`). Elle consolidera les dernières données.

**Q: Où sont stockés les rapports d'audit ?**
A: Les audits vont dans le dossier du domaine concerné, pas dans un dossier générique :
- Audit RGPD → `docs/security/rgpd/AUDIT-YYYY-MM-DD.md`
- Audit RGS → `docs/security/rgs/AUDIT-YYYY-MM-DD.md`

**Q: Y a-t-il un dossier pour les incidents ?**
A: Non, il n'y a pas de dossier `incidents/` générique. Les procédures de gestion d'incidents sont documentées :
- RGPD : `docs/security/rgpd/procedure-violation.md` (Art. 33-34)
- RGS : Dans le MCS (`docs/security/rgs/MCS-[SYSTEME].md`)
