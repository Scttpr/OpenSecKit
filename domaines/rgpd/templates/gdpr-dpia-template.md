---
title: "Modèle d'analyse d'impact relative à la protection des données (AIPD) RGPD"
template_version: "1.0.0"
domain: "gdpr"
constitutional_principle: "I - Threat Modeling, II - Risk Analysis"
ssdlc_phase: "planning"
last_updated: "2025-01-15"
reviewers:
  - dpo
  - security-champion-team
description: "Modèle d'analyse d'impact relative à la protection des données conforme au RGPD. Obligatoire pour les traitements à risque élevé selon l'Article 35. Identifie et atténue les risques liés à la vie privée pour les personnes concernées."
tags:
  - rgpd
  - aipd
  - vie-privee
  - protection-donnees
  - evaluation-risques
difficulty: "intermediate"
estimated_time: "4-8 heures"
prerequisites:
  - "Activités de traitement définies"
  - "Flux de données documentés"
  - "Base juridique identifiée"
related_templates:
  - "../../templates/threat-modeling/stride-threat-model-template-planning.md"
  - "../../templates/risk-analysis/risk-scoring-template-planning.md"
compliance_frameworks:
  - "RGPD Article 35"
  - "Lignes directrices CEPD sur AIPD"
---

# Analyse d'impact relative à la protection des données (AIPD) RGPD

## Résumé exécutif

**Nom du projet** : [ex : Plateforme d'analyse du comportement client]

**Date** : 2025-01-15

**AIPD requise ?** : ☑ Oui ☐ Non

**Niveau de risque global** : ☐ Faible ☑ Moyen ☐ Élevé

**Recommandation** : ☑ Procéder avec atténuations ☐ Nécessite consultation avec l'autorité de contrôle ☐ Ne pas procéder

---

## 1. Filtrage : L'AIPD est-elle obligatoire ?

### RGPD Article 35.3 - Critères d'AIPD obligatoire

Une AIPD est **obligatoire** lorsque le traitement est susceptible d'engendrer un **risque élevé** pour les personnes concernées, notamment :

#### Critère 1 : Évaluation systématique et approfondie (Profilage)

☑ **Applicable**

Le traitement inclut une prise de décision automatisée avec des effets juridiques ou similaires significatifs :
- **Exemple** : Notation de crédit automatisée affectant l'approbation de prêt.
- **Ce projet** : Modèles d'apprentissage automatique (*Machine Learning*) prédisant le comportement d'achat client, utilisés pour le marketing ciblé (pas d'effets juridiques mais impact commercial significatif).

#### Critère 2 : Traitement à grande échelle de catégories particulières de données

☐ **Non applicable**

Traitement de catégories particulières de données (santé, biométrique, génétique) ou condamnations pénales à grande échelle :
- **Ce projet** : Aucune catégorie particulière de données traitée (seulement données comportementales/transactionnelles).

#### Critère 3 : Surveillance systématique de zones accessibles au public

☐ **Non applicable**

Surveillance systématique de zones accessibles au public à grande échelle :
- **Ce projet** : Pas de vidéosurveillance ou de surveillance publique.

### Critères additionnels du CEPD (au moins 2 déclenchent une AIPD)

| Critère | Applicable ? | Justification |
|---------|-------------|---------------|
| Évaluation ou scoring | ☑ Oui | Scoring client basé sur le comportement d'achat |
| Prise de décision automatisée avec effet juridique/significatif | ☐ Non | Recommandations marketing, pas d'effets juridiques |
| Surveillance systématique | ☐ Non | Pas de surveillance continue |
| Données sensibles | ☐ Non | Pas de catégories particulières |
| Données traitées à grande échelle | ☑ Oui | Plus de 500 000 profils clients |
| Croisement ou combinaison d'ensembles de données | ☑ Oui | Combinaison historique d'achats, comportement web, démographie |
| Données concernant des personnes vulnérables | ☐ Non | Clients adultes, pas de groupes vulnérables |
| Utilisation innovante ou technologie | ☑ Oui | Apprentissage automatique pour prédiction comportementale |
| Transfert hors UE | ☐ Non | Tout le traitement a lieu dans l'UE |

**Décompte** : **4 critères sur 9** remplis → **AIPD obligatoire**

---

## 2. Description des opérations de traitement

### 2.1 Nature du traitement

**Nom du projet** : Plateforme d'analyse du comportement client

**Finalité** : Prédire le comportement d'achat des clients pour optimiser les campagnes marketing et améliorer l'expérience client.

**Base juridique** (RGPD Article 6) :
- ☐ Consentement (Art. 6.1.a)
- ☐ Contrat (Art. 6.1.b)
- ☑ **Intérêt légitime** (Art. 6.1.f) - Amélioration du service client et de l'efficacité marketing
- ☐ Obligation légale (Art. 6.1.c)
- ☐ Intérêts vitaux (Art. 6.1.d)
- ☐ Mission d'intérêt public (Art. 6.1.e)

**Évaluation de l'intérêt légitime (LIA)** :
- **Finalité** : Marketing personnalisé pour améliorer l'expérience client et l'efficacité commerciale.
- **Nécessité** : Alternatives moins intrusives considérées (analytics agrégées) mais insuffisantes pour la personnalisation.
- **Test de mise en balance** : Les personnes concernées s'attendent raisonnablement à des offres personnalisées ; risques atténués par pseudonymisation et droit d'opposition (*opt-out*).

---

### 2.2 Portée du traitement

**Catégories de données collectées** :
| Type de données | Exemples | Sensibilité | Source |
|-----------------|----------|-------------|--------|
| Données d'identification | Nom, email, ID client | Moyenne | Enregistrement client |
| Données démographiques | Âge, sexe, localisation (ville) | Faible | Profil client |
| Données comportementales | Historique d'achats, comportement de navigation, abandon de panier | Moyenne | Suivi site web, transactions |
| Préférences | Intérêts produits, préférences de communication | Faible | Paramètres utilisateur |
| Données appareil | Adresse IP (pseudonymisée), type de navigateur | Faible | Logs serveur web |

**Données de catégories particulières** : ☐ Oui ☑ **Non**

**Volume de données** :
- Nombre de personnes concernées : 500 000 clients (résidents UE).
- Enregistrements par personne : ~1 000 événements comportementaux/an.
- Ensemble de données total : 500 millions d'événements.

---

### 2.3 Contexte du traitement

**Personnes concernées** :
- **Principales** : Clients adultes (18+) de la plateforme e-commerce.
- **Géographique** : Résidents UE (France, Allemagne, Belgique marchés principaux).
- **Groupes vulnérables** : Aucun.

**Conservation des données** :
- **Données comportementales** : 2 ans (fenêtre glissante pour entraînement modèle ML).
- **Profils clients** : Durée de la relation client + 1 an.
- **Analytics anonymisées** : Indéfiniment (après anonymisation complète).

**Partage de données** :
- **Interne** : Équipe marketing, équipe science des données (*Data Science*) (IDs pseudonymisés uniquement).
- **Sous-traitants externes** :
  - AWS (hébergement cloud - région Irlande).
  - SendGrid (email marketing - centres de données UE).
  - Google Analytics (web analytics - addendum traitement données UE).
- **Tiers** : Aucun (pas de vente de données).

**Transferts de données** :
- ☑ Tout le traitement dans l'UE/EEE.
- ☐ Transferts vers pays tiers : N/A.

---

## 3. Évaluation de la nécessité et de la proportionnalité

### 3.1 Le traitement est-il nécessaire ?

| Question | Réponse | Justification |
|----------|---------|---------------|
| Le traitement atteint-il la finalité déclarée ? | ☑ Oui | La prédiction comportementale nécessite des données comportementales historiques. |
| Existe-t-il des alternatives moins intrusives ? | ☐ Pas d'alternatives efficaces | Les analytics agrégées ne peuvent fournir de recommandations personnalisées. |
| Les données sont-elles minimisées ? | ☑ Oui | Seules les catégories nécessaires à la prédiction sont collectées (pas de données santé, financières ou politiques). |
| La période de conservation est-elle justifiée ? | ☑ Oui | Fenêtre de 2 ans équilibre précision du modèle et vie privée (plus court = moins précis, plus long = excessif). |

**Conclusion sur la nécessité** : Le traitement est nécessaire pour l'intérêt légitime (marketing personnalisé) sans alternatives moins intrusives.

---

### 3.2 Évaluation de la proportionnalité

**Test de proportionnalité** :
1. **Intérêt légitime mis en balance avec les droits des personnes concernées** :
   - Intérêt : Expérience client améliorée, recommandations produits pertinentes.
   - Droits : Vie privée, minimisation des données.
   - **Équilibre** : Modéré - atténué par pseudonymisation, *opt-out* et transparence.

2. **Minimisation des données** :
   - [x] Seules les catégories de données nécessaires sont collectées.
   - [x] Pseudonymisation appliquée lorsque possible (IDs clients au lieu de noms dans analytics).
   - [x] Pas de données de catégories particulières.

3. **Transparence** :
   - [x] Politique de confidentialité explique clairement le suivi comportemental et le profilage.
   - [x] Mécanisme d'*opt-out* facile (centre de préférences).
   - [x] Droit d'opposition au profilage (RGPD Art. 21).

**Conclusion sur la proportionnalité** : Le traitement est proportionné avec les mesures d'atténuation en place.

---

## 4. Évaluation des risques pour les personnes concernées

### 4.1 Risques identifiés pour les personnes concernées

#### Risque 1 : Accès non autorisé aux profils comportementaux

**Description** : Un attaquant obtient l'accès à la base de données d'analytics comportementale, exposant les habitudes d'achat et préférences de 500 000 clients.

**Préjudice potentiel pour les personnes concernées** :
- **Type de préjudice** : Violation de la vie privée, *phishing* ciblé, risque d'usurpation d'identité.
- **Gravité** : ☐ Faible ☑ Moyenne ☐ Élevée
- **Vraisemblance** : ☐ Faible ☑ Moyenne ☐ Élevée

**Score de risque** : Gravité Moyenne × Vraisemblance Moyenne = **Risque Moyen**

**Mesures de protection existantes** :
- [x] Chiffrement de la base de données au repos (AES-256).
- [x] TLS 1.3 pour les données en transit.
- [x] Contrôle d'accès (RBAC - équipe science des données uniquement).
- [ ] Pseudonymisation des données (IDs clients, pas de noms) - partiel.
- [x] Audits de sécurité réguliers.

**Lacunes** :
- ❌ Pas de pseudonymisation complète (adresses email toujours dans la DB analytics pour envoi de campagnes).
- ❌ Pas de masquage des données pour les environnements hors production.

---

#### Risque 2 : Discrimination via profilage

**Description** : Le modèle ML crée par inadvertance des segments clients discriminatoires (ex : exclusion de certaines démographies des offres premium).

**Préjudice potentiel pour les personnes concernées** :
- **Type de préjudice** : Discrimination, traitement inéquitable, désavantage économique.
- **Gravité** : ☑ Moyenne ☐ Élevée ☐ Faible
- **Vraisemblance** : ☑ Faible ☐ Moyenne ☐ Élevée (le monitoring proactif des biais réduit la vraisemblance).

**Score de risque** : Gravité Moyenne × Vraisemblance Faible = **Risque Faible-Moyen**

**Mesures de protection existantes** :
- [x] Pas de caractéristiques protégées utilisées dans le modèle (pas d'âge, sexe, ethnicité).
- [x] Tests d'équité du modèle (audits de biais trimestriels).
- [ ] Revue humaine des prédictions du modèle - non implémenté.
- [x] Droit d'opposition au profilage (RGPD Art. 21).

**Lacunes** :
- ❌ Pas de détection automatisée de biais dans le pipeline ML.

---

#### Risque 3 : Violation de données lors du transfert vers les sous-traitants

**Description** : Données interceptées lors du transfert vers SendGrid (sous-traitant email) ou AWS (hébergeur cloud).

**Préjudice potentiel pour les personnes concernées** :
- **Type de préjudice** : Violation de la vie privée, spam, *phishing* ciblé.
- **Gravité** : ☐ Faible ☑ Moyenne ☐ Élevée
- **Vraisemblance** : ☑ Faible ☐ Moyenne ☐ Élevée (chiffrement TLS atténue).

**Score de risque** : Gravité Moyenne × Vraisemblance Faible = **Risque Faible**

**Mesures de protection existantes** :
- [x] Chiffrement TLS 1.3 pour tous les transferts.
- [x] Contrats de sous-traitance (DPA) avec SendGrid, AWS.
- [x] Traitement des données UE uniquement (pas de transferts vers pays tiers).
- [x] Audits de sécurité des sous-traitants (SOC 2, ISO 27001).

**Lacunes** :
- Aucune identifiée.

---

#### Risque 4 : Manque de transparence / Fatigue du consentement

**Description** : Les personnes concernées ne comprennent pas pleinement le suivi comportemental, ou n'exercent pas leurs droits d'*opt-out* en raison de la complexité.

**Préjudice potentiel pour les personnes concernées** :
- **Type de préjudice** : Manque de contrôle, violation de la vie privée, érosion de la confiance.
- **Gravité** : ☑ Faible ☐ Moyenne ☐ Élevée
- **Vraisemblance** : ☐ Faible ☑ Moyenne ☐ Élevée (politiques de confidentialité complexes courantes).

**Score de risque** : Gravité Faible × Vraisemblance Moyenne = **Risque Faible**

**Mesures de protection existantes** :
- [x] Politique de confidentialité conforme au RGPD.
- [x] Centre de préférences (*opt-out* du suivi comportemental).
- [ ] Politique de confidentialité claire et simple - version actuelle juridique/complexe.
- [x] Droit d'accès, de rectification, d'effacement des données.

**Lacunes** :
- ❌ Politique de confidentialité trop complexe (nécessite simplification).
- ❌ Pas de notifications "juste-à-temps" (explications contextuelles).

---

### 4.2 Matrice de synthèse des risques

| ID Risque | Description du risque | Gravité | Vraisemblance | Niveau de risque | Risque résiduel (après atténuations) |
|-----------|----------------------|---------|---------------|------------------|-------------------------------------|
| R1 | Accès non autorisé aux profils comportementaux | Moyenne | Moyenne | **Moyen** | Faible (après pseudonymisation complète) |
| R2 | Discrimination via profilage | Moyenne | Faible | Faible-Moyen | Faible (après monitoring des biais) |
| R3 | Violation de données lors du transfert | Moyenne | Faible | **Faible** | Faible (maintenu) |
| R4 | Manque de transparence | Faible | Moyenne | **Faible** | Faible (après simplification politique) |

**Niveau de risque global** : **Moyen** (principalement dû à R1).

**Seuil de consultation de l'Autorité** : Risque résiduel élevé après atténuations → Non déclenché (risque résiduel est Faible).

---

## 5. Mesures d'atténuation

### 5.1 Atténuations pour Risque 1 (Accès non autorisé)

| Mesure | Type | Responsable | Calendrier | Efficacité |
|--------|------|-------------|------------|------------|
| **Pseudonymisation complète** de la DB analytics (table de correspondance email/nom séparée) | Technique | Data Engineering | T1 2025 | Élevée - réduit l'exposition en cas de violation |
| **Masquage des données** pour environnements hors production (dev, staging) | Technique | DevOps | T1 2025 | Moyenne - prévient l'exposition en dev |
| **Contrôle d'accès renforcé** (MFA pour accès DB, moindre privilège) | Technique | Sécurité | T1 2025 | Élevée - réduit les accès non autorisés |
| **Tests d'intrusion réguliers** (annuels) | Organisationnel | CISO | Annuel | Moyenne - identifie les vulnérabilités |

**Risque résiduel après atténuations** : **Faible**

---

### 5.2 Atténuations pour Risque 2 (Discrimination)

| Mesure | Type | Responsable | Calendrier | Efficacité |
|--------|------|-------------|------------|------------|
| **Détection automatisée de biais** dans le pipeline ML (bibliothèque Fairlearn) | Technique | Science des données | T1 2025 | Élevée - détection proactive |
| **Revue humaine** des décisions modèle à fort impact (exclusions premium) | Organisationnel | Marketing + DS | Immédiat | Moyenne - détecte les cas limites |
| **Métriques de diversité** dans le tableau de bord de suivi modèle | Organisationnel | Science des données | T2 2025 | Moyenne - visibilité sur l'équité |
| **Audits d'équité trimestriels** par consultant externe | Organisationnel | CISO | Trimestriel | Élevée - revue indépendante |

**Risque résiduel après atténuations** : **Faible**

---

### 5.3 Atténuations pour Risque 3 (Violation de données lors du transfert)

**Pas d'atténuations supplémentaires requises** - les contrôles existants (TLS 1.3, DPA, sous-traitants SOC 2) sont suffisants.

**Risque résiduel** : **Faible** (maintenu)

---

### 5.4 Atténuations pour Risque 4 (Manque de transparence)

| Mesure | Type | Responsable | Calendrier | Efficacité |
|--------|------|-------------|------------|------------|
| **Politique de confidentialité simplifiée** (langage clair, approche en couches) | Organisationnel | Juridique + UX | T1 2025 | Élevée - améliore la compréhension |
| **Notifications juste-à-temps** (pop-ups contextuelles expliquant le suivi) | Technique | Engineering | T2 2025 | Moyenne - transparence opportune |
| **Amélioration centre de préférences** (tableau de bord visuel) | Technique | Engineering | T2 2025 | Moyenne - *opt-out* facilité |
| **Revue annuelle politique de confidentialité** (tests de lisibilité) | Organisationnel | DPO | Annuel | Moyenne - amélioration continue |

**Risque résiduel après atténuations** : **Faible**

---

## 6. Consultation

### 6.1 Consultation interne

**Délégué à la protection des données (DPO) consulté** : ☑ Oui ☐ Non

**Nom du DPO** : [Marie Dupont]
**Date de consultation** : 2025-01-10
**Retour du DPO** :
- Soutient les conclusions de l'AIPD.
- Recommande de prioriser la pseudonymisation complète (T1 2025).
- Suggère d'ajouter la politique de conservation des données à la politique de confidentialité (actuellement vague).

**Approbation du DPO** : ☑ Approuvé ☐ Approuvé sous conditions ☐ Non approuvé

---

### 6.2 Consultation des personnes concernées

**Les personnes concernées ont-elles été consultées ?** : ☑ Oui ☐ Non ☐ Non applicable

**Méthode** : Enquête en ligne auprès de 1 000 clients (janvier 2025).

**Retours clés** :
- 72 % soutiennent les recommandations personnalisées (échange de valeur compris).
- 18 % préoccupés par le partage de données avec des tiers (traité dans la politique de confidentialité).
- 54 % trouvent la politique de confidentialité actuelle trop longue/complexe (atténuation : simplification).

**Changements effectués** : Simplification de la politique de confidentialité ajoutée au plan d'atténuation.

---

### 6.3 Consultation de l'autorité de contrôle

**Une consultation préalable de l'Autorité est-elle requise ?** (RGPD Art. 36)

☐ **Oui** - Risque résiduel élevé après atténuations ne peut être adéquatement traité.
☑ **Non** - Risque résiduel est Faible après atténuations ; pas de consultation préalable nécessaire.

**Justification** : Tous les risques identifiés atténués au niveau Faible ; le traitement peut procéder sans pré-approbation de l'Autorité.

---

## 7. Validation et revue

### 7.1 Approbation de l'AIPD

**Préparé par** : [Équipe Protection des données - Alice Martin]
**Révisé par** : [DPO - Marie Dupont]
**Approuvé par** : [CISO - Jacques Bernard]

**Date** : 2025-01-15

**Décision d'approbation** :
☑ **Approuvé** - Le traitement peut procéder avec les atténuations identifiées.
☐ Approuvé sous conditions - [Préciser les conditions]
☐ Non approuvé - Le traitement présente un risque inacceptable.

---

### 7.2 Suivi et revue

**Déclencheurs de revue de l'AIPD** :
- [ ] Changements majeurs aux opérations de traitement (ex : nouvelles catégories de données, finalités).
- [ ] Nouvelles technologies introduites (ex : algorithmes ML différents).
- [ ] Changements dans le paysage des risques (ex : nouveaux acteurs de menace, violations de données dans le secteur).
- [ ] Mises à jour des orientations de l'autorité de contrôle.
- [ ] **Revue programmée** : Annuelle (prochaine revue : janvier 2026).

**Suivi continu** :
- Réévaluation trimestrielle des risques (équipe science des données + DPO).
- Revue annuelle des résultats de tests d'intrusion.
- Revue trimestrielle des résultats d'audits d'équité.
- Suivi des incidents de violation de données (mise à jour immédiate de l'AIPD si violation survient).

---

## 8. Annexes

### Annexe A : Diagramme de flux de données

```
Client → Site Web → Événement Comportemental (clic, achat, ajout panier)
                ↓
        Service Analytics (AWS UE)
                ↓
        ID Client Pseudonymisé + Détails Événement
                ↓
        Entraînement Modèle ML (toutes les 2 semaines)
                ↓
        Affectation Segment Client
                ↓
        Déclenchement Campagne Marketing
                ↓
        SendGrid (Email) → Client
```

### Annexe B : Documentation de la base juridique

**Évaluation de l'intérêt légitime (LIA)** :
- Finalité : Marketing personnalisé.
- Nécessité : Impossible d'atteindre la personnalisation sans données comportementales.
- Test de mise en balance : Attentes client (offres personnalisées) vs préoccupations vie privée (atténuées).
- Conclusion : Intérêt légitime valide selon Art. 6.1(f).

### Annexe C : Contrats de sous-traitance

- [x] Contrat de sous-traitance AWS (signé 2024-12-01).
- [x] Contrat de sous-traitance SendGrid (signé 2024-11-15).
- [x] Addendum traitement données Google Analytics (accepté 2024-10-01).

### Annexe D : Extrait de la politique de confidentialité

[Lien vers politique de confidentialité complète : https://example.com/privacy]

**Section pertinente** : "Nous utilisons votre historique d'achats et votre comportement de navigation pour recommander des produits susceptibles de vous intéresser. Vous pouvez refuser le suivi comportemental dans votre Centre de préférences. Nous traitons ces données sur la base de notre intérêt légitime à améliorer votre expérience client."

---

## Références

- **RGPD Article 35** : Analyse d'impact relative à la protection des données.
- **EDPB Guidelines 3/2019** : Traitement de données à caractère personnel via dispositifs vidéo.
- **EDPB WP248** : Lignes directrices sur l'AIPD (mise à jour).
- **ICO DPIA Template** : https://ico.org.uk/for-organisations/guide-to-data-protection/guide-to-the-general-data-protection-regulation-gdpr/accountability-and-governance/data-protection-impact-assessments/
- **CNIL DPIA Guidance** (Français) : https://www.cnil.fr/fr/RGPD-analyse-impact-protection-donnees-aipd

---

**Version AIPD** : 1.0 | **Date** : 2025-01-15 | **Prochaine revue** : 2026-01-15 | **Statut** : Approuvé