# Extensions spécifiques par domaine

## Aperçu

Ce répertoire contient des extensions spécifiques par domaine qui adaptent les modèles SSDLC de base pour des industries, des réglementations et des cadres de conformité particuliers.

## Domaines disponibles

| Domaine | Description | Réglementations clés | Public cible |
|---------|-------------|----------------------|------------------|
| [gouvernement-rgs/](gouvernement-rgs/) | **Gouvernement français (RGS)** | Référentiel général de sécurité (RGS), directives ANSSI | Administration publique française, OIV (Opérateurs d'Importance Vitale) |
| [nis2/](nis2/) | **Directive NIS2** | Directive UE 2022/2555, cybersécurité pour les infrastructures critiques | Entités essentielles (EE) et importantes (EI) dans l'UE (18 secteurs) |
| [rgpd/](rgpd/) | **RGPD (Protection des données)** | Règlement UE 2016/679 | Toute organisation traitant des données personnelles de résidents de l'UE |

## Guide de sélection rapide

### Par région géographique

**France (Secteur public)** :
- Principal : [gouvernement-rgs/](gouvernement-rgs/) (obligatoire pour les administrations)
- Secondaire : [rgpd/](rgpd/) (si traitement de données personnelles)
- Secondaire : [nis2/](nis2/) (si infrastructure critique ou OSE/EE)

**Union Européenne** :
- [rgpd/](rgpd/) (obligatoire pour le traitement de données personnelles)
- [nis2/](nis2/) (obligatoire pour les entités essentielles/importantes)

**France (Secteur privé - Infrastructure critique)** :
- Principal : [nis2/](nis2/) (si dans le périmètre des secteurs NIS2)
- Secondaire : [gouvernement-rgs/](gouvernement-rgs/) (principes RGS recommandés)
- Secondaire : [rgpd/](rgpd/) (si traitement de données personnelles)

### Par secteur d'activité

**Administration publique (France)** :
- ✅ [gouvernement-rgs/](gouvernement-rgs/) - Obligatoire
- ✅ [rgpd/](rgpd/) - Si traitement de données personnelles
- ✅ [nis2/](nis2/) - Si entité d'administration publique sous NIS2

**Énergie, Transport, Santé (UE)** :
- ✅ [nis2/](nis2/) - Obligatoire (entités essentielles)
- ✅ [rgpd/](rgpd/) - Si traitement de données personnelles

**Services numériques, Fournisseurs Cloud (UE)** :
- ✅ [nis2/](nis2/) - Obligatoire (entités essentielles)
- ✅ [rgpd/](rgpd/) - Obligatoire (traitement de données personnelles)

**E-commerce, SaaS (marché UE)** :
- ✅ [rgpd/](rgpd/) - Obligatoire
- ⚠️ [nis2/](nis2/) - Si fourniture d'infrastructure/services numériques à grande échelle

**Fabrication, Production alimentaire (UE)** :
- ⚠️ [nis2/](nis2/) - Si "entité importante" (vérifier la liste des secteurs)
- ✅ [rgpd/](rgpd/) - Si traitement de données employés/clients

## Matrice de conformité par domaine

| Exigence | RGS (France) | NIS2 (UE) | RGPD (UE) |
|----------|--------------|-----------|-----------|
| **Modélisation des menaces** | ✅ EBIOS Risk Manager | ✅ Analyse de risques (Art. 21.2.a) | ✅ AIPD pour risque élevé (Art. 35) |
| **Analyse de risques** | ✅ Dossier d'homologation | ✅ Mesures de gestion des risques (Art. 21.2) | ✅ Approche par les risques (Art. 32) |
| **Authentification** | ✅ FranceConnect, certificats RGS | ✅ MFA requis (Art. 21.2.h) | ✅ Authentification appropriée (Art. 32) |
| **Chiffrement** | ✅ Algorithmes approuvés ANSSI | ✅ Cryptographie (Art. 21.2.f) | ✅ Pseudonymisation/chiffrement (Art. 32.1.a) |
| **Journalisation d'audit** | ✅ Rétention 3 ans | ✅ Gestion des incidents (Art. 21.2.b) | ✅ Registre des traitements (Art. 30) |
| **Signalement d'incidents** | ✅ À l'ANSSI | ✅ 24h/72h au CSIRT/ANC (Art. 23) | ✅ 72h à l'APD (CNIL) (Art. 33) |
| **Gestion des correctifs** | ✅ MCS (Maintien en condition de sécurité) | ✅ Divulgation des vulnérabilités (Art. 21.3) | ✅ Rétablissement de la disponibilité (Art. 32.1.c) |
| **Chaîne d'approvisionnement** | ✅ Prestataires qualifiés RGS | ✅ Sécurité de la chaîne (Art. 21.2.d) | ✅ DPA avec sous-traitants (Art. 28) |
| **Tests** | ✅ Audits de conformité RGS | ✅ Tests/audits (Art. 21.2.f) | ✅ Tests réguliers (Art. 32.1.d) |
| **Sanctions** | Administratives | 10 M€ ou 2 % CA (essentielles) | 20 M€ ou 4 % CA (max) |

## Combinaison de plusieurs domaines

De nombreuses organisations doivent se conformer à **plusieurs réglementations**. Voici comment combiner les domaines :

### Secteur public français + Données personnelles

**Appliquer** :
1. [gouvernement-rgs/](gouvernement-rgs/) - Cadre principal (RGS obligatoire)
2. [rgpd/](rgpd/) - Ajouter les exigences spécifiques RGPD (AIPD, droits des personnes concernées)

**Intégration** :
- Utiliser EBIOS RM (RGS) comme cadre de gestion des risques global, compléter avec l'AIPD RGPD.
- FranceConnect répond à la fois aux exigences d'authentification RGS et de sécurité RGPD.
- La rétention de 3 ans des journaux RGS dépasse le minimum souvent requis par le RGPD.
- Ajouter les droits des personnes concernées RGPD (accès, effacement, portabilité) aux exigences RGS.

### Infrastructure critique UE + Données personnelles

**Appliquer** :
1. [nis2/](nis2/) - Cadre principal (NIS2 obligatoire pour les entités essentielles)
2. [rgpd/](rgpd/) - Ajouter les exigences spécifiques RGPD

**Intégration** :
- La gestion des risques NIS2 (Art. 21.2) satisfait l'approche par les risques du RGPD (Art. 32).
- Signalement d'incidents NIS2 (24h/72h) + notification de violation RGPD (72h) : processus séparés mais complémentaires.
- Sécurité de la chaîne d'approvisionnement NIS2 + Accords de traitement des données (DPA) RGPD.
- Les deux règlements exigent des tests et évaluations réguliers.

### Secteur public français + Infrastructure critique UE + Données personnelles

**Appliquer** (les trois) :
1. [gouvernement-rgs/](gouvernement-rgs/) - RGS obligatoire pour l'administration française
2. [nis2/](nis2/) - Si l'entité est dans le périmètre NIS2 (ex : santé, énergie)
3. [rgpd/](rgpd/) - Pour le traitement de données personnelles

**Intégration** :
- EBIOS RM (RGS) + analyse de risques NIS2 + AIPD RGPD → Évaluation de risques intégrée unique.
- FranceConnect (RGS) + MFA (NIS2) + sécurité appropriée (RGPD) → Système d'authentification unique.
- Signalement d'incidents : ANSSI (RGS) + CSIRT National (NIS2) + CNIL (RGPD) - peut nécessiter plusieurs notifications parallèles.
- Le standard le plus élevé l'emporte : chiffrement RGS***, chaîne d'approvisionnement NIS2, droits RGPD.

## Comment utiliser les extensions de domaine

### Étape 1 : Déterminer les domaines applicables

Demandez-vous :
- ✅ Êtes-vous une entité publique française ? → [gouvernement-rgs/](gouvernement-rgs/)
- ✅ Êtes-vous une entité essentielle (EE) ou importante (EI) sous NIS2 ? → [nis2/](nis2/)
- ✅ Traitez-vous des données personnelles de résidents de l'UE ? → [rgpd/](rgpd/)

### Étape 2 : Consulter le README du domaine

Lire le README spécifique au domaine pour :
- Le contexte réglementaire et la base légale ;
- La correspondance (mapping) avec les principes constitutionnels SSDLC ;
- Les checklists de conformité ;
- Les exigences spécifiques au domaine.

### Étape 3 : Utiliser les modèles du domaine

Chaque domaine fournit :
- **Des modèles étendus** adaptant les modèles SSDLC de base ;
- **Des checklists de conformité** spécifiques à la réglementation ;
- **Des exemples** et des guides.

### Étape 4 : Intégrer avec les modèles de base

Les extensions de domaine **complètent** (ne remplacent pas) les modèles de base. Il est important de distinguer le niveau stratégique du niveau opérationnel :

**Exemple de workflow** :
1. **Modélisation des menaces (Opérationnel)** : Utiliser le [modèle STRIDE de base](../templates/01-threat-modeling/stride-threat-model-template-planning.md) pour identifier les menaces techniques sur les composants logiciels.
2. **Ajouter RGS (Stratégique)** : Utiliser [EBIOS RM](gouvernement-rgs/templates/) comme socle d'analyse de risques global (obligatoire pour l'homologation RGS). Les scénarios opérationnels d'EBIOS peuvent être alimentés par l'analyse STRIDE.
3. **Ajouter RGPD** : Inclure l'[AIPD](rgpd/templates/) si traitement de données personnelles à risque élevé.
4. **Résultat** : Une stratégie de risques cohérente liant conformité réglementaire et sécurité technique.

### Étape 5 : Maintenir la conformité

- Révisions régulières (RGS : ré-homologation tous les 3-5 ans).
- Surveillance continue (NIS2 : MCS, RGPD : tests réguliers).
- Signalement d'incidents (suivre le calendrier de chaque domaine).
- Mises à jour de la documentation (maintenir les preuves de conformité).

## Contribution

Une extension de domaine est manquante ? Consultez les [Directives de contribution](../.github/CONTRIBUTING.md).

**Domaines suggérés** :
- Spécifiques à la santé (HDS pour la France, HIPAA pour les US)
- Finance (DORA, PCI-DSS, SOX)
- Gouvernement US (FedRAMP, NIST 800-53)
- Industrie 4.0 / ICS (IEC 62443)

---

**Besoin d'aide pour sélectionner les domaines ?**

Ouvrez une [GitHub Discussion](https://github.com/YourOrg/OpenSecKit/discussions) avec :
- Votre secteur/industrie ;
- Votre localisation géographique ;
- Les types de données traitées ;
- Les exigences de conformité existantes.

Les *Security Champions* peuvent vous aider à déterminer les domaines applicables.

**Prochaines étapes** : Naviguez vers votre/vos domaine(s) applicable(s) et consultez le README pour des instructions détaillées.