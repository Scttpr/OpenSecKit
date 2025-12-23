# Domaine RGS

Référentiel Général de Sécurité pour l'administration française.

## Commande

```bash
/osk-rgs           # Configuration et génération dossier homologation
/osk-rgs renew     # Ré-homologation
```

## Workflow V3

```
┌─────────────────────────────────────────────────────────────┐
│  BROUILLONS PAR FEATURE                                     │
│                                                             │
│  /osk-analyze [feature]                                     │
│       │                                                     │
│       └──▶ .osk/specs/NNN-feature/rgs/ebios.md             │
│            (Analyse EBIOS brouillon par feature)            │
└─────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│  CONSOLIDATION RGS                                          │
│                                                             │
│  /osk-rgs                                                   │
│       │                                                     │
│       ├──▶ Wizard configuration (DICP, niveau, org)        │
│       ├──▶ Consolidation EBIOS → EBIOS-RM-*.md             │
│       └──▶ Génération dossier homologation                  │
└─────────────────────────────────────────────────────────────┘
```

## Documents Générés

| Fichier | Usage |
|---------|-------|
| `EBIOS-RM-[SYSTEME].md` | Analyse de risques (5 ateliers) |
| `DOSSIER-HOMOLOGATION-[SYSTEME].md` | Pour commission d'homologation |
| `MCS-[SYSTEME].md` | Maintien en Condition de Sécurité |
| `RENOUVELLEMENT-*.md` | Rapport de ré-homologation |
| `AUDIT-*.md` | Rapports d'audit |

Tous les fichiers sont générés dans `docs/security/rgs/`.

## Niveaux RGS

| Niveau | Description | Usage |
|--------|-------------|-------|
| `*` (standard) | Niveau de base | Services grand public |
| `**` (renforcé) | Niveau élevé | Données sensibles, services critiques |
| `***` | Niveau très élevé | Secret défense |

## Configuration

Les données RGS sont stockées dans `.osk/config.toml` :

```toml
[domains.rgs]
enabled = true
niveau = "standard"           # standard | renforce | *
systeme = "MonSI"
autorite_homologation = "Directeur SI"

# DICP - Besoins de sécurité
[domains.rgs.dicp]
disponibilite = 3             # 1-4
integrite = 4
confidentialite = 3
preuve = 2

# Fonctions de sécurité
[domains.rgs.fonctions]
authentification = "franceconnect"   # franceconnect | certificat | mfa
signature = "qualifiee"              # simple | avancee | qualifiee
chiffrement = "aes256"
horodatage = "qualifie"

# Homologation
[domains.rgs.homologation]
date_decision = "2025-01-15"
duree = "3 ans"
perimetre = "Application de téléprocédure"
```

## Données Partagées

Ces sections de config sont partagées avec `/osk-rgpd` :

```toml
[domains.organisation]
nom = "Ministère X"
siret = "123 456 789 00000"
rssi = "Jean Dupont"
rssi_email = "rssi@gouv.fr"

[[domains.suppliers]]         # Supply Chain
nom = "OVH"
type = "hebergement"
localisation = "France"
qualification_rgs = true
```

## EBIOS Risk Manager

La méthode EBIOS RM structure l'analyse en 5 ateliers :

```
┌─────────────────────────────────────────────────────────────┐
│  ATELIER 1 : Socle de Sécurité                              │
│  • Missions et valeurs métier                               │
│  • Biens supports (techniques)                              │
│  • Besoins DICP                                             │
└─────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│  ATELIER 2 : Sources de Risques                             │
│  • Identification des acteurs malveillants                  │
│  • Objectifs visés                                          │
└─────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│  ATELIER 3 : Scénarios Stratégiques                         │
│  • Chemins d'attaque haut niveau                            │
│  • Cartographie des menaces                                 │
└─────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│  ATELIER 4 : Scénarios Opérationnels                        │
│  • Modes opératoires détaillés                              │
│  • Vraisemblance et gravité                                 │
└─────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│  ATELIER 5 : Traitement du Risque                           │
│  • Mesures de sécurité                                      │
│  • Risque résiduel accepté                                  │
└─────────────────────────────────────────────────────────────┘
```

## Checklist Homologation

```
PRE-HOMOLOGATION
────────────────
[ ] Niveau RGS déterminé (*, **, ***)
[ ] Autorité d'homologation désignée
[ ] RSSI identifié
[ ] Périmètre défini

EBIOS RM
────────
[ ] Atelier 1 : Socle de sécurité validé
[ ] Atelier 2 : Sources de risques identifiées
[ ] Atelier 3 : Scénarios stratégiques documentés
[ ] Atelier 4 : Scénarios opérationnels détaillés
[ ] Atelier 5 : Plan de traitement défini

DOSSIER
───────
[ ] Expression des besoins de sécurité
[ ] Analyse de risques EBIOS RM
[ ] Spécifications de sécurité
[ ] Plan d'action sécurité
[ ] Attestation formelle

DECISION
────────
[ ] Revue par autorité d'homologation
[ ] Risque résiduel accepté
[ ] Décision signée
```

## Exigences par Fonction

### Authentification

| Niveau | Exigence |
|--------|----------|
| * | MFA ou FranceConnect |
| ** | Certificat RGS ou FranceConnect+ |
| *** | Certificat RGS qualifié |

### Chiffrement (ANSSI)

```
Algorithmes approuvés :
  ✅ AES-128, AES-192, AES-256 (GCM)
  ✅ RSA ≥ 2048 bits (recommandé 4096)
  ✅ ECDSA (P-256, P-384, P-521)
  ✅ SHA-256, SHA-384, SHA-512

Algorithmes interdits :
  ❌ DES, 3DES
  ❌ RSA < 2048 bits
  ❌ MD5, SHA-1
```

### Journalisation

| Exigence | Valeur |
|----------|--------|
| Rétention minimum | 3 ans |
| Format | Structuré (JSON) |
| Horodatage | RGS qualifié |
| Intégrité | Logs immuables |

## FranceConnect

Intégration recommandée pour l'authentification :

```
Niveaux eIDAS :
  • Faible      : email + mot de passe
  • Substantiel : 2FA
  • Élevé       : certificat qualifié
```

Fournisseurs d'identité :
- Impots.gouv.fr (DGFiP)
- Ameli.fr (Assurance Maladie)
- La Poste (Identité Numérique)

## Détection Automatique

`/osk-configure` détecte les patterns RGS dans le code :

```
Patterns détectés → Domaine RGS activé automatiquement :
  • "gouv.fr", "service-public"
  • "franceconnect", "fc_callback"
  • "siret", "siren"
  • "administration", "RGS", "ANSSI"
```

## Templates

Les templates RGS sont dans `domaines/gouvernement-rgs/templates/` :

| Template | Usage |
|----------|-------|
| `ebios-rm-template.md` | Méthodologie EBIOS RM |
| `dossier-homologation-template.md` | Format dossier ANSSI |
| `grille-dicp-template.md` | Évaluation besoins DICP |
| `mcs-template.md` | Maintien Condition Sécurité |

> `/osk-rgs` utilise ces templates automatiquement.

## Références

- [ANSSI - RGS v2.0](https://cyber.gouv.fr/le-referentiel-general-de-securite-rgs)
- [ANSSI - EBIOS RM](https://cyber.gouv.fr/la-methode-ebios-risk-manager)
- [FranceConnect - Documentation](https://partenaires.franceconnect.gouv.fr/)
- [LSTI - Prestataires qualifiés](https://www.lsti-certification.fr/)
