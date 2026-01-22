# Framework RGS

Conformité au Référentiel Général de Sécurité (RGS v2.0) de l'ANSSI.

## Commande

```bash
/osk-comply rgs              # Évaluation de conformité
/osk-comply rgs --update     # Mise à jour évaluation existante
/osk-comply rgs --export md  # Export dossier d'homologation
```

## Périmètre

Le RGS s'applique aux :
- Systèmes d'information des administrations françaises
- Services publics numériques
- Opérateurs d'importance vitale (OIV)
- Opérateurs de services essentiels (OSE)

## Niveaux de Sécurité

| Niveau | Description | Score Minimum |
|--------|-------------|---------------|
| `RGS*` | Standard - Services courants | 70% |
| `RGS**` | Renforcé - Données sensibles | 85% |
| `RGS***` | Élevé - Infrastructure critique | 95% |

## Annexes RGS v2.0

### Annexes A - Certificats électroniques

| Annexe | Titre |
|--------|-------|
| A1 | Règles relatives aux fonctions de sécurité basées sur les certificats électroniques |
| A2 | Politique de Certification Type « certificats électroniques de personne » |
| A3 | Politique de Certification Type « services applicatifs » |
| A4 | Profils de certificats, CRL, OCSP et algorithmes cryptographiques |
| A5 | Politique d'Horodatage Type |

### Annexes B - Mécanismes cryptographiques

| Annexe | Titre |
|--------|-------|
| B1 | Choix et dimensionnement des mécanismes cryptographiques |
| B2 | Gestion des clés utilisées dans des mécanismes cryptographiques |
| B3 | Mécanismes d'authentification |

### Annexe C - Audit

| Annexe | Titre |
|--------|-------|
| C | Référentiel d'exigences applicables aux prestataires d'audit de la SSI |

## Modèle DICP

Le RGS utilise le modèle DICP (et non CIA) :

| Critère | Description | Échelle |
|---------|-------------|---------|
| **D** - Disponibilité | Accessibilité du service | 0-4 |
| **I** - Intégrité | Protection contre altération | 0-4 |
| **C** - Confidentialité | Protection contre divulgation | 0-4 |
| **P** - Preuve | Traçabilité et non-répudiation | 0-4 |

## Fichiers Générés

```
.osk/comply/rgs/
├── assessment-rgs.yaml          # Évaluation machine-readable
├── assessment-rgs.md            # Résumé lisible
├── homologation-checklist.md    # Checklist pré-homologation
└── system-perimeter.md          # Périmètre du système
```

## Homologation

L'homologation est la décision formelle autorisant l'exploitation du SI.

### Les 9 Étapes

1. Définir le besoin
2. Identifier les besoins DICP
3. Analyser les risques (EBIOS RM)
4. Identifier les mesures de sécurité
5. Constituer le dossier de sécurité
6. Réaliser les audits
7. Préparer la décision
8. Prononcer la décision
9. Maintenir la sécurité (MCS)

**Durée de validité** : 3 ans maximum (renouvelable)

## Certifications Requises

### Pour `RGS**`/`RGS***`

| Service | Certification Requise |
|---------|----------------------|
| Cloud | SecNumCloud |
| Hébergement santé | HDS |
| Certificats | ANSSI qualified |

### Fournisseurs SecNumCloud

Consultez la liste officielle sur [cyber.gouv.fr/secnumcloud](https://cyber.gouv.fr/secnumcloud).

## Base de Connaissances

### Documents officiels RGS

| Fichier | Contenu |
|---------|---------|
| `knowledge/rgs-v2-document-principal.md` | Document principal RGS v2.0 |
| `knowledge/rgs-v2-annexe-a1-certificats-electroniques.md` | Annexe A1 - Certificats électroniques |
| `knowledge/rgs-v2-annexe-a2-politique-certification-personne.md` | Annexe A2 - PC Type personne |
| `knowledge/rgs-v2-annexe-a3-politique-certification-services-applicatifs.md` | Annexe A3 - PC Type services |
| `knowledge/rgs-v2-annexe-a4-profils-certificats-algorithmes.md` | Annexe A4 - Profils et algorithmes |
| `knowledge/rgs-v2-annexe-a5-politique-horodatage.md` | Annexe A5 - Politique d'horodatage |
| `knowledge/rgs-v2-annexe-b1-mecanismes-cryptographiques.md` | Annexe B1 - Mécanismes crypto |
| `knowledge/rgs-v2-annexe-b2-gestion-cles.md` | Annexe B2 - Gestion des clés |
| `knowledge/rgs-v2-annexe-b3-authentification.md` | Annexe B3 - Authentification |
| `knowledge/rgs-v2-annexe-c-audit-prestataires.md` | Annexe C - Audit prestataires |

### Guides ANSSI

| Fichier | Contenu |
|---------|---------|
| `knowledge/guide-homologation-securite.md` | Guide d'homologation en 9 étapes |
| `knowledge/guide-hygiene-informatique.md` | Guide d'hygiène informatique (42 mesures) |
| `knowledge/ebios-risk-manager.md` | Méthodologie EBIOS Risk Manager |

## Références

- [ANSSI - RGS v2.0](https://cyber.gouv.fr/le-referentiel-general-de-securite-rgs)
- [ANSSI - Homologation en 9 étapes](https://cyber.gouv.fr/publications/lhomologation-de-securite-en-9-etapes-simples)
- [ANSSI - EBIOS Risk Manager](https://cyber.gouv.fr/la-methode-ebios-risk-manager)
- [ANSSI - SecNumCloud](https://cyber.gouv.fr/secnumcloud)
