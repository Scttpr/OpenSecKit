# Domaines Réglementaires

Les domaines adaptent OpenSecKit aux exigences réglementaires spécifiques.

## Domaines Disponibles

| Domaine | Commande OSK | Description |
|---------|--------------|-------------|
| [RGPD](rgpd/) | `/osk-rgpd` | Protection des données personnelles (UE) |
| [RGS](gouvernement-rgs/) | `/osk-rgs` | Référentiel Général de Sécurité (France) |
| [NIS2](nis2/) | *(en cours)* | Directive cybersécurité UE |

## Sélection Rapide

### Par Contexte

| Vous êtes... | Domaines applicables |
|--------------|---------------------|
| Administration publique française | **RGS** + RGPD |
| OIV / Infrastructure critique France | **RGS** + NIS2 + RGPD |
| Entreprise traitant données personnelles UE | **RGPD** |
| Entité essentielle/importante NIS2 | **NIS2** + RGPD |

### Par Priorité

```
1. RGPD     → Obligatoire si données personnelles UE
2. RGS      → Obligatoire si administration française
3. NIS2     → Obligatoire si entité essentielle/importante
```

## Intégration V3

Les domaines s'intègrent au workflow principal :

```
┌─────────────────────────────────────────────────────────────┐
│  WORKFLOW PAR FEATURE                                       │
│                                                             │
│  /osk-analyze ──▶ Génère brouillons DPIA et EBIOS          │
│                   dans .osk/specs/NNN-feature/              │
└─────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│  CONSOLIDATION PAR DOMAINE                                  │
│                                                             │
│  /osk-rgpd ──▶ Consolide les DPIA → dpia-global.md         │
│            ──▶ Génère registre, procédures                  │
│                                                             │
│  /osk-rgs  ──▶ Consolide les EBIOS → EBIOS-RM-*.md         │
│            ──▶ Génère dossier homologation                  │
└─────────────────────────────────────────────────────────────┘
```

## Fichiers Générés

### RGPD (`/osk-rgpd`)

| Fichier | Description |
|---------|-------------|
| `docs/security/rgpd/registre-traitements.md` | Registre Art. 30 |
| `docs/security/rgpd/dpia-global.md` | DPIA consolidé (Art. 35) |
| `docs/security/rgpd/procedure-violation.md` | Notification Art. 33-34 |
| `docs/security/rgpd/AUDIT-*.md` | Rapports d'audit |

### RGS (`/osk-rgs`)

| Fichier | Description |
|---------|-------------|
| `docs/security/rgs/EBIOS-RM-*.md` | Analyse de risques EBIOS |
| `docs/security/rgs/DOSSIER-HOMOLOGATION-*.md` | Pour commission |
| `docs/security/rgs/MCS-*.md` | Maintien Condition Sécurité |

## Configuration

Les domaines partagent des données via `.osk/config.toml` :

```toml
[domains]
active = ["rgpd", "rgs"]

[domains.organisation]    # Partagé RGPD + RGS
nom = "Mon Organisation"
siret = "123 456 789 00000"

[[domains.suppliers]]     # Art. 28 + Supply Chain
nom = "OVH"
type = "hebergement"
localisation = "France"
dpa_signe = true

[domains.rgpd]
dpo_nom = "Jean Dupont"
dpo_email = "dpo@example.com"

[domains.rgs]
niveau = "standard"       # standard | renforce | *
systeme = "MonSI"
```

## Combinaison de Domaines

### Administration + Données Personnelles

```bash
# 1. Analyser les features (génère brouillons DPIA + EBIOS)
/osk-analyze authentication

# 2. Consolider RGPD
/osk-rgpd

# 3. Consolider RGS
/osk-rgs

# 4. Auditer
/osk-rgpd audit
/osk-rgs renew
```

### Exigences Croisées

| Exigence | RGS | NIS2 | RGPD |
|----------|-----|------|------|
| Analyse de risques | EBIOS RM | Art. 21.2.a | DPIA (Art. 35) |
| Notification incident | ANSSI | CSIRT 24h/72h | CNIL 72h |
| Chiffrement | ANSSI-approved | Art. 21.2.f | Art. 32.1.a |
| Tests | Audit RGS | Art. 21.2.f | Art. 32.1.d |
| Sanctions | Administratives | 10M ou 2% CA | 20M ou 4% CA |

## Détection Automatique

`/osk-configure` détecte automatiquement les domaines applicables :

```
Patterns détectés :
  ✅ RGPD : "user", "email", "password" trouvés
  ✅ RGS  : "gouv.fr", "franceconnect" trouvés
  ⚠️ NIS2 : vérifier manuellement le périmètre
```

---

**Prochaines étapes** : Consultez le README du domaine applicable pour les détails.
