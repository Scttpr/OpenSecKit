# Modèles de modélisation des menaces

## Aperçu

Ce répertoire contient des modèles pour le **Principe Constitutionnel I : modélisation des menaces** - l'identification systématique des menaces de sécurité potentielles pour aider les équipes à concevoir des systèmes sécurisés dès le départ.

## Pourquoi la modélisation des menaces ?

La modélisation des menaces vous aide à :
- **Identifier les menaces tôt** dans le cycle de développement (phases de planification/conception)
- **Prioriser les efforts de sécurité** en fonction du risque
- **Concevoir des contre-mesures efficaces** avant l'implémentation
- **Communiquer les risques de sécurité** aux parties prenantes
- **Se conformer** aux cadres de sécurité (OWASP SAMM, NIST SSDF, ISO 27001)

## Modèles disponibles

| Modèle | Phase | Difficulté | Temps | Description |
|--------|-------|------------|-------|-------------|
| [modele-menaces-stride-planification.md](stride-threat-model-template-planning.md) | Planification | Intermédiaire | 2-4 heures | Modélisation des menaces basée sur STRIDE (Spoofing, Tampering, Repudiation, Information Disclosure, Denial of Service, Elevation of Privilege) |
| [modele-arbre-attaque-planification.md](attack-tree-template-planning.md) | Planification | Intermédiaire | 1-2 heures | Analyse hiérarchique des chemins d'attaque pour atteindre des objectifs spécifiques |
| [modele-diagramme-flux-donnees-conception.md](data-flow-diagram-template-design.md) | Conception | Débutant | 1-2 heures | Représentation visuelle du mouvement des données à travers les systèmes, fondation essentielle pour STRIDE |

## Quand utiliser ces modèles

### Phase de planification
- ✅ Utiliser le **Modèle de Menaces STRIDE** quand vous avez une architecture de haut niveau
- ✅ Utiliser l'**Arbre d'Attaque** pour analyser des scénarios d'attaque spécifiques de haute valeur
- ✅ Commencer avec le **Diagramme de Flux de Données** si vous devez d'abord cartographier les interactions système

### Phase de conception
- ✅ Créer/raffiner le **Diagramme de Flux de Données** à mesure que l'architecture se solidifie
- ✅ Mettre à jour le **Modèle de Menaces STRIDE** avec les éléments de conception détaillés
- ✅ Développer les **Arbres d'Attaque** pour les actifs critiques nouvellement identifiés

### Workflow recommandé

```
1. Créer un Diagramme de Flux de Données (DFD)
   ↓
2. Identifier les limites de confiance et flux de données
   ↓
3. Exécuter l'analyse STRIDE sur chaque composant
   ↓
4. Pour les actifs de haute valeur, créer des Arbres d'Attaque
   ↓
5. Documenter les menaces dans le registre des risques
   ↓
6. Concevoir les contre-mesures (voir Principe III - exigences de sécurité)
```

## Démarrage rapide

### Première modélisation des menaces ?

**Commencez ici** : [modele-diagramme-flux-donnees-conception.md](data-flow-diagram-template-design.md)

1. **Cartographier votre système** : Créer un Diagramme de Flux de Données
2. **Identifier les menaces** : Utiliser le modèle STRIDE sur chaque composant
3. **Prioriser** : Utiliser la Notation des Risques (voir [../02-risk-analysis/](../02-risk-analysis/))
4. **Atténuer** : Documenter les contre-mesures

### Vous avez déjà une architecture ?

**Commencez ici** : [modele-menaces-stride-planification.md](stride-threat-model-template-planning.md)

1. **Rassembler la documentation d'architecture** : Diagrammes système, liste des composants, flux de données
2. **Exécuter STRIDE** : Analyser systématiquement chaque composant
3. **Approfondir les actifs critiques** : Utiliser les Arbres d'Attaque pour les zones à haut risque
4. **Documenter et suivre** : Lier au registre des risques et aux exigences de sécurité

## Choisir le bon modèle

### Utiliser le modèle de menaces STRIDE quand :
- ✅ Vous avez besoin d'une identification complète des menaces
- ✅ Vous travaillez sur un nouveau système ou une fonctionnalité majeure
- ✅ La conformité exige une modélisation formelle des menaces
- ✅ Vous voulez une couverture systématique des catégories de menaces

### Utiliser l'arbre d'attaque quand :
- ✅ Vous devez comprendre les chemins d'attaque pour des objectifs spécifiques
- ✅ Vous effectuez une planification de test d'intrusion / red team
- ✅ Vous voulez communiquer la complexité d'attaque aux parties prenantes
- ✅ Vous devez prioriser les défenses basées sur la probabilité d'attaque

### Utiliser le diagramme de flux de données quand :
- ✅ Vous devez visualiser les interactions système
- ✅ Vous préparez une analyse STRIDE
- ✅ Vous voulez identifier les limites de confiance
- ✅ Vous devez documenter le mouvement des données pour la conformité (RGPD, HIPAA)

## Intégration avec les autres principes

La modélisation des menaces alimente :

- **Principe II (analyse de risques)** : Les menaces identifiées ici sont notées dans le [registre des risques](../02-risk-analysis/risk-register-template-all.md)
- **Principe III (exigences de sécurité)** : Les contre-mesures deviennent des [exigences de sécurité](../03-security-requirements/)
- **Principe IV (tests de sécurité)** : Les menaces informent les [Plans de Tests](../04-security-testing/)
- **Principe VI (Journalisation d'Audit)** : Les flux de données critiques nécessitent des [Journaux d'Audit](../06-audit-logging/)

## Exemples

Voir l'exemple concret :
- [_example-ecommerce-stride.md](_example-ecommerce-stride.md) - Modèle de menaces STRIDE complet pour une application e-commerce

## Mapping de conformité

Ces modèles aident à satisfaire les exigences de :

- **OWASP SAMM** : Pratique de sécurité - Évaluation des Menaces
- **NIST SSDF** : Pratique PW.1.2 - Identifier les vulnérabilités potentielles
- **ISO 27001** : Annexe A.14.1.2 - Sécurisation des services d'application sur réseaux publics
- **PCI-DSS** : Exigence 6.3.1 - Suppression des comptes d'application personnalisés avant activation
- **HIPAA** : § 164.308(a)(1)(ii)(A) - analyse de risques

## Outils et ressources

### Outils recommandés
- [Microsoft Threat Modeling Tool](https://aka.ms/threatmodelingtool) - Gratuit, supporte STRIDE et DFD
- [OWASP Threat Dragon](https://owasp.org/www-project-threat-dragon/) - Outil de modélisation des menaces open-source
- [IriusRisk](https://www.iriusrisk.com/) - Plateforme commerciale de modélisation des menaces

### Ressources externes
- [OWASP Threat Modeling](https://owasp.org/www-community/Threat_Modeling)
- [Microsoft SDL Threat Modeling](https://www.microsoft.com/en-us/securityengineering/sdl/threatmodeling)
- [Livre de Threat Modeling d'Adam Shostack](https://shostack.org/books/threat-modeling-book)
- [NIST SP 800-154 - Guide to Data-Centric System Threat Modeling](https://csrc.nist.gov/publications/detail/sp/800-154/draft)

## Contribuer

Vous avez des améliorations pour ces modèles ? Voir les [directives de contribution](../../.github/CONTRIBUTING.md).

Contributions courantes :
- Méthodologies de modélisation des menaces supplémentaires (PASTA, LINDDUN)
- Catalogues de menaces spécifiques aux domaines
- Exemples spécifiques aux secteurs
- Intégration avec les outils de modélisation des menaces

---

**Besoin d'Aide ?** Ouvrir une [Discussion GitHub](https://github.com/YourOrg/OpenSecKit/discussions) avec le tag `01-threat-modeling`.

**Prochaines étapes** : Après la modélisation des menaces, procéder à l'[analyse de risques](../02-risk-analysis/) pour noter et prioriser les menaces identifiées.
