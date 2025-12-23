# RGPD

Règlement Général sur la Protection des Données (UE 2016/679).

## Vue d'ensemble

Le RGPD encadre le traitement des données personnelles dans l'Union Européenne.

## Articles Couverts

| Article | Description | Commande OSK |
|---------|-------------|--------------|
| Art. 25 | Privacy by Design | `/osk-specify` |
| Art. 30 | Registre des traitements | `/osk-rgpd` |
| Art. 32 | Sécurité du traitement | `/osk-harden` |
| Art. 33 | Notification de violation | `/osk-rgpd` |
| Art. 35 | DPIA | `/osk-rgpd` |

## Commandes

### Configuration

```bash
>>> /osk-rgpd
```

Génère :

- `docs/security/rgpd/registre-traitements.md`
- `docs/security/rgpd/dpia-global.md`
- `docs/security/rgpd/procedure-violation.md`
- `docs/security/rgpd/politique-conservation.md`

### Audit

```bash
>>> /osk-rgpd audit
```

Génère un rapport d'audit de conformité.

## Registre des Traitements (Art. 30)

Le registre documente chaque traitement de données personnelles :

```yaml
traitements:
  - id: "T001"
    nom: "Gestion des utilisateurs"
    finalite: "Authentification et gestion des comptes"
    base_legale: "Exécution du contrat"
    categories_donnees:
      - "email"
      - "nom"
      - "mot de passe (hashé)"
    destinataires:
      - "Équipe support interne"
    transferts_hors_ue: false
    duree_conservation: "3 ans après dernière connexion"
    mesures_securite:
      - "Chiffrement AES-256"
      - "Accès restreint RBAC"
```

## DPIA (Art. 35)

L'Analyse d'Impact sur la Protection des Données est requise pour :

- Traitement à grande échelle de données sensibles
- Profilage avec effets juridiques
- Surveillance systématique d'un lieu public

## Procédure de Violation (Art. 33)

En cas de violation de données :

1. **Notification CNIL** - Dans les 72h
2. **Notification personnes** - Si risque élevé
3. **Documentation** - Registre des violations

## Principes Fondamentaux

| Principe | Description |
|----------|-------------|
| Licéité | Base légale valide |
| Limitation des finalités | Traitement pour finalité définie |
| Minimisation | Données strictement nécessaires |
| Exactitude | Données à jour |
| Limitation conservation | Durée définie |
| Intégrité et confidentialité | Sécurité appropriée |

## Voir aussi

- [Commande /osk-rgpd](../commands/osk-rgpd.md)
- [CNIL - RGPD](https://www.cnil.fr/fr/rgpd-de-quoi-parle-t-on)
