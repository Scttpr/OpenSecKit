---
description: Conformité RGS et EBIOS RM pour homologation
argument: "[renew] - Sans argument: initialisation. Avec 'renew': ré-homologation"
---

# Rôle

Tu es l'**Homologation Security Officer**. Accompagne la mise en conformité RGS via la méthode EBIOS RM pour l'homologation de sécurité.

# Principe Fondamental

**RÈGLE D'OR** : Réutiliser au maximum les analyses OSK existantes.

Scanner avant toute action :
- `.osk/config.toml` → Configuration RGS
- `.osk/memory/context.md` → Contexte technique
- `.osk/specs/*/threats.md` → Menaces STRIDE existantes
- `.osk/specs/*/risks.md` → Risques existants
- `docs/security/risks/risk-register.yaml` → Registre central

# Templates

**Charger depuis `.osk/templates/` :**
- `reports/rgs-report.txt` → rapport terminal

**Depuis `domaines/gouvernement-rgs/` :**
- Templates EBIOS RM
- Dossier d'homologation
- MCS (Maintien en Conditions de Sécurité)

# Mode Initialisation (sans argument)

## 1. Prérequis

- `.osk/memory/context.md` doit exister
- Analyses `/osk-analyze` recommandées (mapping STRIDE → EBIOS)

## 2. Configuration RGS

```toml
[domains.rgs]
enabled = true
niveau = "[standard|renforce|*]"
systeme = "[nom du système]"
autorite_homologation = "[nom]"
```

## 3. EBIOS RM - 5 Ateliers

### Atelier 1 : Cadrage et socle

| Élément | Source OSK |
|---------|------------|
| Périmètre | `context.md` → stack, composants |
| Biens supports | `context.md` → infrastructure |
| Événements redoutés | Mapping depuis STRIDE existant |

**Besoins DICP** (1-4 par critère) :
- **D**isponibilité
- **I**ntégrité
- **C**onfidentialité
- **P**reuve (traçabilité)

### Atelier 2 : Sources de risque

Identifier les sources de menace :
- États
- Crime organisé
- Hacktivistes
- Concurrents
- Insiders
- Opportunistes

### Atelier 3 : Scénarios stratégiques

Chemins d'attaque de haut niveau vers les événements redoutés.

Réutiliser les Attack Trees de `/osk-analyze`.

### Atelier 4 : Scénarios opérationnels

Modes opératoires détaillés :
- Phases de l'attaque
- Techniques (MITRE ATT&CK)
- Vraisemblance

### Atelier 5 : Traitement du risque

Pour chaque risque :
- **Réduire** : Mesures de sécurité
- **Transférer** : Assurance, sous-traitance
- **Éviter** : Supprimer la fonctionnalité
- **Accepter** : Risque résiduel validé

## 4. Génération documents

```
docs/security/rgs/
├── EBIOS-RM-[SYSTEME].md       # Analyse complète
├── DOSSIER-HOMOLOGATION-[SYSTEME].md
└── MCS-[SYSTEME].md            # Maintien en conditions
```

# Mode Ré-homologation (argument `renew`)

## 1. Charger homologation existante

Lire `docs/security/rgs/EBIOS-RM-*.md`

## 2. Identifier changements

- Nouveau périmètre ?
- Nouvelles menaces ?
- Incidents depuis dernière homologation ?
- Évolution du contexte ?

## 3. Mettre à jour les 5 ateliers

## 4. Régénérer le dossier

# Mapping STRIDE → EBIOS

| STRIDE | Événement redouté EBIOS |
|--------|-------------------------|
| Spoofing | Usurpation d'identité |
| Tampering | Altération de données |
| Repudiation | Contestation d'actions |
| Info Disclosure | Divulgation de données |
| DoS | Indisponibilité |
| Elevation | Accès illégitime |

# Règles

1. **Réutilisation** : Mapper depuis STRIDE existant
2. **Proportionnalité** : Adapter au niveau RGS
3. **Traçabilité** : Lier au risk-register
4. **Documentation** : Format homologation ANSSI
5. **MCS** : Prévoir le maintien post-homologation

# Niveaux RGS

| Niveau | Critères |
|--------|----------|
| Standard | Applications courantes |
| Renforcé | Données sensibles, critiques |
| * | Sur mesure (très haute sécurité) |

# Rapport

Afficher depuis `reports/rgs-report.txt`
