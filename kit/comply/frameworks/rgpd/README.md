# Domaine RGPD

Conformité au Règlement Général sur la Protection des Données (UE 2016/679).

## Commande

```bash
/osk-rgpd           # Configuration et génération documents
/osk-rgpd audit     # Audit de conformité
```

## Workflow V3

```
┌─────────────────────────────────────────────────────────────┐
│  BROUILLONS PAR FEATURE                                     │
│                                                             │
│  /osk-analyze [feature]                                     │
│       │                                                     │
│       └──▶ .osk/specs/NNN-feature/rgpd/dpia.md             │
│            (DPIA brouillon par feature)                     │
└─────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│  CONSOLIDATION RGPD                                         │
│                                                             │
│  /osk-rgpd                                                  │
│       │                                                     │
│       ├──▶ Extraction données depuis .osk/config.toml      │
│       ├──▶ Consolidation DPIA → dpia-global.md             │
│       └──▶ Génération documents finaux                      │
└─────────────────────────────────────────────────────────────┘
```

## Documents Générés

| Fichier | Article RGPD | Description |
|---------|--------------|-------------|
| `registre-traitements.md` | Art. 30 | Inventaire des traitements |
| `dpia-global.md` | Art. 35 | Analyse d'impact consolidée |
| `procedure-violation.md` | Art. 33-34 | Notification en 72h |
| `politique-conservation.md` | Art. 5 | Durées de conservation |
| `mentions-legales.md` | Art. 13-14 | Transparence utilisateurs |
| `AUDIT-*.md` | - | Rapports d'audit |

Tous les fichiers sont générés dans `docs/security/rgpd/`.

## Configuration

Les données RGPD sont stockées dans `.osk/config.toml` :

```toml
[domains.rgpd]
enabled = true
niveau = "sensible"           # standard | sensible | special

# DPO
dpo_nom = "Jean Dupont"
dpo_email = "dpo@example.com"
dpo_telephone = "+33 1 23 45 67 89"

# Conservation
durees_conservation = [
  { type = "logs_connexion", duree = "1 an" },
  { type = "donnees_client", duree = "3 ans après fin contrat" }
]

# Traitements principaux
[[domains.rgpd.traitements]]
nom = "Gestion des utilisateurs"
finalite = "Authentification et gestion des comptes"
base_legale = "contrat"
donnees = ["nom", "email", "mot_de_passe_hash"]
destinataires = ["Service technique"]
```

## Données Partagées

Ces sections de config sont partagées avec `/osk-rgs` :

```toml
[domains.organisation]
nom = "Mon Organisation"
siret = "123 456 789 00000"
adresse = "1 rue Example, 75001 Paris"

[[domains.suppliers]]         # Art. 28 - Sous-traitants
nom = "OVH"
type = "hebergement"
localisation = "France"
dpa_signe = true
contact = "dpo@ovh.com"
```

## Articles Clés

### Art. 30 - Registre des Traitements

**Obligatoire pour :**
- Organisations > 250 employés
- Traitements non occasionnels
- Données sensibles ou relatives à des condamnations

### Art. 35 - DPIA

**Obligatoire si :**
- Évaluation systématique (profilage)
- Traitement grande échelle de données sensibles
- Surveillance systématique de zones publiques

### Art. 33-34 - Violation de Données

| Délai | Action |
|-------|--------|
| 72h | Notification CNIL (si risque) |
| Sans délai | Notification personnes (si risque élevé) |

## Checklist Conformité

```
PRE-PRODUCTION
─────────────
[ ] Registre des traitements complet
[ ] Base légale identifiée par traitement
[ ] DPO désigné (si requis)
[ ] Mentions légales publiées
[ ] DPIA réalisée (si risque élevé)

CONTRATS
────────
[ ] DPA signés avec sous-traitants
[ ] Clauses RGPD dans CGV/CGU
[ ] Transferts hors UE encadrés

TECHNIQUE
─────────
[ ] Chiffrement données sensibles
[ ] Logs d'accès aux données personnelles
[ ] Procédure de réponse aux demandes (accès, effacement)
[ ] Procédure de notification violation
```

## Détection Automatique

`/osk-configure` détecte les patterns RGPD dans le code :

```
Patterns détectés → Domaine RGPD activé automatiquement :
  • "user", "email", "password"
  • "date_of_birth", "address", "phone"
  • "ip_address", "health", "religion"
  • "biometric", "RGPD", "GDPR"
```

## Templates

Les templates RGPD sont dans `frameworks/rgpd/templates/` :

| Template | Usage |
|----------|-------|
| `registre-traitements-template.md` | Format registre Art. 30 |
| `dpia-template.md` | Méthodologie DPIA CNIL |
| `procedure-violation-template.md` | Notification Art. 33-34 |
| `dpa-template.md` | Accord sous-traitance Art. 28 |

> `/osk-rgpd` utilise ces templates automatiquement.

## Références

- [CNIL - Guide RGPD](https://www.cnil.fr/fr/rgpd-de-quoi-parle-t-on)
- [CNIL - Méthodologie DPIA](https://www.cnil.fr/fr/ce-quil-faut-savoir-sur-lanalyse-dimpact-relative-la-protection-des-donnees-dpia)
- [EUR-Lex - Règlement 2016/679](https://eur-lex.europa.eu/legal-content/FR/TXT/?uri=CELEX%3A32016R0679)
