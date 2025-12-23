# /osk-incident

Gestion des incidents de securite et reponse a crise.

## Synopsis

```bash
>>> /osk-incident "description de l'incident"
```

## Description

`/osk-incident` active le mode **Incident Commander** pour gerer une crise de securite en temps reel. Cette commande guide l'equipe a travers les etapes critiques de reponse a incident et prepare le terrain pour l'analyse post-mortem.

## Prerequis

- `.osk/memory/context.md` - Contexte projet

## Fichiers generes

| Fichier | Description |
|---------|-------------|
| `docs/security/incidents/INC-[DATE]-001.md` | Journal d'incident |
| `docs/security/risks/risk-register.yaml` | Mise a jour du registre |

## Processus

### 1. Qualification

- Type d'incident (Secret/Faille/Donnees)
- Evaluation de la gravite

### 2. Identification du Principe Viole

Determine quel principe constitutionnel a ete compromis :

| Principe | Cause possible |
|----------|----------------|
| I | Menace non anticipee |
| II | Risque non identifie |
| III | Controle absent |
| IV | Vulnerabilite non detectee |
| V | Secret expose |
| VI | Incident non detecte a temps |
| VII | Vulnerabilite connue non corrigee |

### 3. Actions Immediates

**Si fuite de secret :**

- Revocation du secret compromis
- Rotation et deploiement nouveau secret
- Nettoyage historique git (BFG/Filter-repo)

**Si exploitation/bug :**

- Isolation du service / Mode maintenance
- Identification de la ligne vulnerable
- Hotfix immediat

**Si donnees personnelles :**

- Demarrage compteur 72h RGPD
- Notification DPO

### 4. Documentation

- Journal de bord avec timeline
- Communication de crise (Slack/Teams, DPO)
- Enregistrement au Risk Register

## Exemple

```bash
>>> /osk-incident "Cle API Stripe exposee dans un commit public"
```

Genere :

```
docs/security/incidents/INC-2025-01-15-001.md
```

Avec :

- Checklist de revocation/rotation
- Messages de communication pre-rediges
- Entree dans le risk-register

## Cloture et Post-Mortem

Une fois l'incident resolu :

1. Marquer le statut comme **RESOLU**
2. Mettre a jour `risk-register.yaml`
3. Lancer `/osk-dashboard` pour verifier la conformite
4. Documenter les lecons apprises

## Voir aussi

- [Risk Register](../concepts/risk-register.md) - Gestion des risques
- [/osk-dashboard](osk-dashboard.md) - Vue consolidee
- [/osk-pca-pra](osk-pca-pra.md) - Plans de continuite
