# Domaines Réglementaires

OpenSecKit supporte plusieurs domaines réglementaires pour adapter la sécurité au contexte légal.

## Domaines Disponibles

| Domaine | Description | Commande |
|---------|-------------|----------|
| [RGPD](rgpd.md) | Protection des données personnelles | `/osk-rgpd` |
| [RGS](rgs.md) | Référentiel Général de Sécurité (France) | `/osk-rgs` |
| [NIS2](nis2.md) | Directive cybersécurité UE | *En cours* |

## Détection Automatique

`/osk-configure` détecte automatiquement les domaines applicables selon :

### RGPD

Patterns détectés :

- `user`, `email`, `password`, `address`, `phone`
- `date_of_birth`, `ip_address`, `health`, `religion`
- `biometric`, `RGPD`, `GDPR`, `données personnelles`

### RGS

Patterns détectés :

- `gouv.fr`, `service-public`, `franceconnect`
- `siret`, `siren`, `administration`, `RGS`, `ANSSI`

### NIS2

Patterns détectés :

- `energy`, `transport`, `banking`, `health`, `water`
- `digital`, `NIS2`, `OIV`, `OSE`, `entité essentielle`

## Configuration Manuelle

```toml
# .osk/config.toml
[domains]
active = ["rgpd", "rgs"]

[domains.rgpd]
enabled = true
niveau = "standard"

[domains.rgs]
enabled = true
niveau = "standard"
```

## Niveaux

Chaque domaine peut avoir différents niveaux d'exigence :

| Domaine | Niveaux |
|---------|---------|
| RGPD | standard |
| RGS | standard, renforcé, * |
| NIS2 | essentiel, important |
