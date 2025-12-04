---
title: "Modèle d'avis de confidentialité RGPD (Articles 13-14)"
template_version: "1.0.0"
domain: "gdpr"
constitutional_principles:
  - "III - Security by Design"
  - "VI - Audit Logging"
regulatory_references:
  - "RGPD Article 13 (Informations à fournir lorsque des données à caractère personnel sont collectées auprès de la personne concernée)"
  - "RGPD Article 14 (Informations à fournir lorsque les données à caractère personnel n'ont pas été collectées auprès de la personne concernée)"
  - "RGPD Article 12 (Transparence des informations et des communications et modalités de l'exercice des droits de la personne concernée)"
ssdlc_phase: "design"
difficulty: "beginner"
estimated_time: "2-4 hours"
last_updated: "2025-01-19"
reviewers:
  - name: "Data Protection Officer"
    role: "GDPR Compliance"
  - name: "Legal Counsel"
    role: "Regulatory Review"
---

# Modèle d'avis de confidentialité RGPD
# (Privacy Notice / Data Protection Notice)

## Objectif

Ce modèle vous aide à créer un **avis de confidentialité conforme au RGPD** (aussi appelé "politique de confidentialité" ou "notice de protection des données") qui informe les personnes concernées de la manière dont leurs données personnelles sont collectées, utilisées et protégées.

**Exigence légale** : Les avis de confidentialité sont **obligatoires** selon les Articles 13-14 du RGPD et doivent être fournis :
- Au moment de la collecte des données (Article 13).
- Dans un délai d'un mois si les données proviennent d'autres sources (Article 14).

## Quand utiliser ce modèle

- [ ] **Obligatoire** pour tous les systèmes traitant des données personnelles de résidents de l'UE.
- [ ] Requis lors de la création de compte, soumission de formulaire, consentement cookies.
- [ ] Requis pour les applications mobiles (avis in-app + app store).
- [ ] Requis pour les sites web (lien en pied de page accessible, clair et visible).
- [ ] Mise à jour requise lorsque les finalités de traitement changent.

## Exigences clés du RGPD

| Article | Exigence | Conséquence si manquant |
|---------|----------|-------------------------|
| **Art. 12** | Langage clair et simple, facilement accessible | Jusqu'à 10 M€ ou 2 % du CA mondial |
| **Art. 13** | Information au moment de la collecte (14 éléments obligatoires) | Jusqu'à 20 M€ ou 4 % du CA mondial |
| **Art. 14** | Information si données de tiers (15 éléments + source) | Jusqu'à 20 M€ ou 4 % du CA mondial |

**Langage simple** : Les avis de confidentialité doivent être compréhensibles par les utilisateurs moyens (éviter le jargon juridique, utiliser des phrases simples, définir les termes techniques).

---

## MODÈLE D'AVIS DE CONFIDENTIALITÉ

*Remplacer les [espaces réservés entre crochets] par vos informations spécifiques. Supprimer les sections non applicables à votre traitement.*

---

# Avis de confidentialité / Notice de protection des données
# Privacy Notice / Data Protection Notice

**Dernière mise à jour** : [Date, ex : "19 janvier 2025"]

**Date d'effet** : [Date de prise d'effet de cet avis]

---

## 1. Qui nous sommes (Identification du responsable de traitement)

**Responsable de traitement** :
- **Nom** : [Nom de l'entité juridique, ex : "ACME SaaS Inc."]
- **Adresse** : [Adresse légale complète, ex : "123 rue de l'Innovation, 75001 Paris, France"]
- **Email de contact** : [Email, ex : "privacy@acme.com"]
- **Téléphone** : [Numéro avec indicatif pays, ex : "+33 1 23 45 67 89"]
- **Site web** : [URL, ex : "https://www.acme.com"]

**Délégué à la protection des données (DPO)** *(obligatoire si traitement de catégories particulières ou surveillance à grande échelle)* :
- **Nom** : [Nom du DPO ou "N/A" si non requis]
- **Email** : [Email DPO, ex : "dpo@acme.com"]
- **Téléphone** : [Numéro du DPO]

**Représentant dans l'UE** *(obligatoire si responsable hors UE)* :
- **Nom** : [Nom du représentant UE ou "N/A"]
- **Adresse** : [Adresse UE]
- **Email** : [Email représentant]

---

## 2. Quelles données personnelles nous collectons

Nous collectons les catégories suivantes de données personnelles :

### 2.1 Données que vous fournissez directement

| Catégorie de données | Exemples | Finalité |
|----------------------|----------|----------|
| **Données d'identification** | Prénom, nom, nom d'utilisateur | Création de compte, personnalisation |
| **Informations de contact** | Adresse email, numéro de téléphone, adresse postale | Communication, livraison du service |
| **Identifiants de compte** | Mot de passe (haché), tokens MFA | Authentification, sécurité du compte |
| **Informations de paiement** | Numéro carte bancaire (tokenisé), adresse de facturation | Traitement des paiements, facturation |
| **Données de profil** | Date de naissance, photo de profil, bio | Personnalisation du service |
| **Données de contenu** | Messages, documents uploadés, commentaires | Fonctionnalité du service |

### 2.2 Données que nous collectons automatiquement

| Catégorie de données | Exemples | Finalité |
|----------------------|----------|----------|
| **Données techniques** | Adresse IP, type de navigateur, ID appareil, système d'exploitation | Sécurité, prévention fraude, analytics |
| **Données d'usage** | Pages visitées, fonctionnalités utilisées, temps passé, parcours de clics | Amélioration du service, analytics |
| **Données de localisation** | Pays, ville (depuis géolocalisation IP) | Localisation, conformité |
| **Cookies et suivi** | Cookies de session, cookies analytics, cookies publicitaires | Voir Politique cookies (lien ci-dessous) |

### 2.3 Données que nous recevons de tiers

*(Supprimer cette section si non applicable)*

| Source | Catégories de données | Finalité |
|--------|----------------------|----------|
| **Connexion sociale (Google, Facebook, LinkedIn)** | Nom, email, photo de profil | Création de compte, authentification |
| **Processeurs de paiement (Stripe, PayPal)** | Statut paiement, ID transaction | Confirmation de paiement |
| **Courtiers en données / Partenaires** | [Préciser catégories de données] | [Préciser finalité] |

**Sources tierces** : [Lister sources spécifiques, ex : "Nous recevons des données de LinkedIn lorsque vous utilisez 'Se connecter avec LinkedIn'"]

### 2.4 Catégories particulières de données personnelles

*(Supprimer si non applicable - celles-ci nécessitent un consentement explicite ou une autre base juridique)*

Nous traitons les **catégories particulières** suivantes de données personnelles (Article 9) :

- ☐ **Origine raciale ou ethnique** : [Finalité, base juridique]
- ☐ **Opinions politiques** : [Finalité, base juridique]
- ☐ **Convictions religieuses ou philosophiques** : [Finalité, base juridique]
- ☐ **Appartenance syndicale** : [Finalité, base juridique]
- ☐ **Données génétiques** : [Finalité, base juridique]
- ☐ **Données biométriques** (ex : reconnaissance faciale) : [Finalité, base juridique]
- ☐ **Données de santé** : [Finalité, base juridique]
- ☐ **Vie sexuelle ou orientation sexuelle** : [Finalité, base juridique]

**Base juridique pour catégories particulières** : [ex : "Consentement explicite (Article 9.2.a)", "Nécessaire pour soins de santé (Article 9.2.h)"]

---

## 3. Pourquoi nous traitons vos données (Finalités et base juridique)

Nous traitons vos données personnelles pour les finalités suivantes :

| Finalité | Base juridique (RGPD Article 6) | Catégories de données |
|----------|----------------------------------|----------------------|
| **Création et gestion de compte** | Exécution du contrat (Art. 6.1.b) | Identification, contact, identifiants |
| **Fourniture du service** (fonctionnalité principale) | Exécution du contrat (Art. 6.1.b) | Toutes données nécessaires au service |
| **Traitement des paiements** | Exécution du contrat (Art. 6.1.b) | Informations paiement, adresse facturation |
| **Support client** | Exécution du contrat (Art. 6.1.b) | Informations contact, tickets support |
| **Sécurité et prévention de la fraude** | Intérêt légitime (Art. 6.1.f) | Données techniques, adresse IP, schémas d'usage |
| **Analytics et amélioration du service** | Intérêt légitime (Art. 6.1.f) | Données d'usage, données techniques |
| **Communications marketing** (newsletters, offres) | Consentement (Art. 6.1.a) | Informations de contact |
| **Publicité personnalisée** | Consentement (Art. 6.1.a) | Données d'usage, cookies, données de profil |
| **Obligations légales** (fiscal, comptable, lois conservation données) | Obligation légale (Art. 6.1.c) | Toutes données requises par la loi |

**Évaluation de l'intérêt légitime** :

Lorsque nous nous appuyons sur l'**intérêt légitime**, nous avons évalué que notre intérêt ne l'emporte pas sur vos droits et libertés. Vous avez le droit de vous opposer au traitement basé sur l'intérêt légitime (voir Section 8).

**Exemple d'intérêt légitime** :
- **Sécurité et prévention de la fraude** : Nous avons un intérêt légitime à protéger nos systèmes et utilisateurs contre les cyberattaques, la fraude et l'abus. Ce traitement est nécessaire et proportionné.
- **Analytics** : Nous avons un intérêt légitime à comprendre comment les utilisateurs interagissent avec notre service pour améliorer les fonctionnalités et l'expérience utilisateur.

**Retrait du consentement** :

Lorsque nous nous appuyons sur le **consentement**, vous pouvez retirer votre consentement à tout moment en :
- [Méthode 1 : ex : "Cliquant sur 'Se désabonner' dans les emails marketing"]
- [Méthode 2 : ex : "Visitant vos paramètres de compte → Confidentialité → Gestion du consentement"]
- [Méthode 3 : ex : "Envoyant un email à privacy@acme.com"]

---

## 4. Avec qui nous partageons vos données (Destinataires)

Nous partageons vos données personnelles avec les catégories suivantes de destinataires :

### 4.1 Fournisseurs de services (Sous-traitants)

Nous utilisons des fournisseurs de services tiers de confiance pour soutenir nos opérations. Ces sous-traitants agissent selon nos instructions et sont liés par des **Contrats de sous-traitance** (Article 28).

| Fournisseur de services | Service | Données partagées | Localisation |
|-------------------------|---------|-------------------|--------------|
| **AWS (Amazon Web Services)** | Hébergement cloud | Toutes données (chiffrées au repos) | UE (Francfort, Irlande) |
| **Stripe** | Traitement paiements | Informations paiement, adresse facturation | UE + US (cadre d'adéquation) |
| **SendGrid** | Livraison email | Adresse email, nom | US (Clauses Contractuelles Types) |
| **Google Analytics** | Analytics | Adresse IP (anonymisée), données d'usage | US (Clauses Contractuelles Types) |
| **Zendesk** | Support client | Informations contact, tickets support | US (Clauses Contractuelles Types) |

**Contrats de sous-traitance** : Tous les sous-traitants ont signé des DPA garantissant la conformité RGPD.

### 4.2 Autorités légales et réglementaires

Nous pouvons partager des données avec :
- **Forces de l'ordre** (si légalement requis, ex : ordre judiciaire, enquête criminelle)
- **Autorités fiscales** (facturation, conformité TVA)
- **Autorités de contrôle** (ex : CNIL en France, si notification de violation requise)

### 4.3 Transferts d'entreprise

En cas de fusion, acquisition ou vente d'actifs, vos données peuvent être transférées au nouveau propriétaire. Vous serez notifié par email 30 jours avant le transfert.

### 4.4 Tiers (avec votre consentement)

- **Partenaires publicitaires** : [ex : "Facebook Ads, Google Ads"] - uniquement si vous consentez à la publicité personnalisée
- **Médias sociaux** : [ex : "Boutons partage pour Twitter, LinkedIn"] - uniquement lorsque vous partagez activement du contenu

---

## 5. Transferts internationaux de données (Hors UE/EEE)

Certains de nos fournisseurs de services sont situés **en dehors de l'Espace Économique Européen (EEE)**. Lorsque nous transférons des données internationalement, nous garantissons une protection adéquate :

### 5.1 Transferts vers les pays "adéquats"

Les données peuvent être transférées vers des pays bénéficiant d'une **décision d'adéquation UE** (Article 45) :
- ✅ **Royaume-Uni** (décision d'adéquation depuis juin 2021)
- ✅ **Suisse** (décision d'adéquation)
- ✅ **Canada** (organisations commerciales)
- ✅ **Japon** (décision d'adéquation)

### 5.2 Transferts vers les États-Unis

**Cadre de protection des données UE-US** (DPF) :
- Nous transférons des données vers des fournisseurs américains certifiés selon le **Cadre de protection des données UE-US** (remplacement du *Privacy Shield*).
- **Fournisseurs certifiés** : [ex : "Google LLC (certification DPF), AWS (certification DPF)"]
- **Vérification** : Vous pouvez vérifier les certifications sur https://www.dataprivacyframework.gov/list

**Clauses contractuelles types** (CCT) :
- Pour les fournisseurs US non couverts par le DPF, nous utilisons les **Clauses contractuelles types UE** (CCT 2021) approuvées par la Commission Européenne.
- **Évaluation d'impact du transfert** : Nous avons évalué que les fournisseurs US mettent en œuvre des garanties appropriées (chiffrement, contrôles d'accès) pour protéger vos données.

### 5.3 Transferts vers d'autres pays

Pour les transferts vers des pays sans adéquation :
- ✅ **Clauses contractuelles types** (CCT) avec les sous-traitants
- ✅ **Règles d'entreprise contraignantes** (BCR) pour transferts intra-groupe (si applicable)
- ✅ **Consentement** pour transferts spécifiques (vous serez explicitement sollicité)

**Vos droits** : Vous pouvez demander une copie des garanties que nous utilisons pour les transferts internationaux en envoyant un email à [privacy@acme.com].

---

## 6. Combien de temps nous conservons vos données (Période de conservation)

Nous conservons vos données personnelles uniquement le temps nécessaire aux finalités listées ci-dessus :

| Catégorie de données | Période de conservation | Base juridique |
|----------------------|------------------------|----------------|
| **Données de compte** (utilisateurs actifs) | Durée du compte + 30 jours | Exécution du contrat |
| **Données de compte** (comptes supprimés) | 30 jours après demande suppression (puis définitivement supprimées) | RGPD Article 17 (droit à l'effacement) |
| **Enregistrements paiements** | 7 ans après dernière transaction | Obligation légale (loi fiscale) |
| **Tickets support** | 3 ans après clôture ticket | Intérêt légitime (amélioration service) |
| **Consentement marketing** | Jusqu'au retrait consentement + 30 jours | Consentement (RGPD Article 6.1.a) |
| **Logs d'audit** | 3 ans | Obligation légale (RGPD Article 30, exigences sécurité) |
| **Analytics anonymisées** | Indéfiniment (pas de données personnelles) | Non applicable (données anonymes) |

**Processus de suppression** :
- Les données subissent une **suppression logique** (*soft-delete* : marquées pour suppression, inaccessibles aux utilisateurs) sous 30 jours.
- Les données subissent une **suppression définitive** (*hard-delete* : effacées de tous systèmes, y compris sauvegardes) sous 90 jours.

**Exception - Blocages légaux** : Nous pouvons conserver les données plus longtemps si requis par la loi (ex : gel judiciaire, enquête réglementaire).

---

## 7. Comment nous protégeons vos données (Mesures de sécurité)

Nous mettons en œuvre des **mesures techniques et organisationnelles appropriées** (Article 32) pour protéger vos données :

### 7.1 Mesures techniques

- ✅ **Chiffrement au repos** : Chiffrement AES-256 pour toutes bases de données et stockage fichiers
- ✅ **Chiffrement en transit** : TLS 1.3 pour toutes communications (HTTPS)
- ✅ **Pseudonymisation** : IDs utilisateurs pseudonymisés dans analytics et logs
- ✅ **Contrôle d'accès** : Contrôle d'accès basé sur les rôles (RBAC), principe du moindre privilège
- ✅ **Authentification multi-facteurs** : MFA requis pour comptes admin
- ✅ **Sauvegardes régulières** : Sauvegardes chiffrées quotidiennes avec rétention 30 jours
- ✅ **Scan de vulnérabilités** : Scans automatisés hebdomadaires (Nessus, OWASP ZAP)
- ✅ **Tests d'intrusion** : Audits de sécurité tiers annuels

### 7.2 Mesures organisationnelles

- ✅ **Formation du personnel** : Formation annuelle RGPD et sensibilisation sécurité (100% employés)
- ✅ **Contrôles d'accès** : Seul le personnel autorisé peut accéder aux données personnelles
- ✅ **Accords de confidentialité** : Tous employés et contractants signent des NDA
- ✅ **Plan de réponse aux incidents** : Réponse violation de données sous 72 heures (RGPD Article 33)
- ✅ **Analyses d'impact sur la protection des données** : AIPD pour traitement à haut risque (Article 35)

**Notification de violation de données** :
- Si nous subissons une violation de données présentant un risque pour vos droits et libertés, nous vous notifierons **sans délai indu** (typiquement sous 72 heures).
- La notification comprendra : nature de la violation, catégories de données affectées, point de contact, conséquences probables, mesures prises pour traiter la violation.

---

## 8. Vos droits (Droits des personnes concernées RGPD)

Vous disposez des droits suivants selon le RGPD :

### 8.1 Droit d'accès (Article 15)

**Quoi** : Demander une copie de vos données personnelles.
**Comment** : Email [privacy@acme.com] avec objet "Demande d'accès aux données".
**Délai de réponse** : 1 mois (gratuit pour la première demande).

**Nous fournirons** :
- Confirmation du traitement.
- Copie de vos données personnelles (format JSON ou CSV).
- Informations sur finalités, destinataires, période de conservation.

### 8.2 Droit de rectification (Article 16)

**Quoi** : Corriger les données inexactes ou incomplètes.
**Comment** :
- Option 1 : Mettre à jour directement dans paramètres compte (instantané).
- Option 2 : Email [privacy@acme.com] avec corrections.
**Délai de réponse** : 1 mois.

### 8.3 Droit à l'effacement ("Droit à l'oubli") (Article 17)

**Quoi** : Demander la suppression de vos données personnelles.
**Comment** : Email [privacy@acme.com] avec objet "Supprimer mes données".
**Délai de réponse** : 1 mois (données supprimées sous 30 jours, définitivement effacées sous 90 jours).

**Quand nous pouvons supprimer** :
- ✅ Données plus nécessaires pour la finalité initiale.
- ✅ Vous retirez votre consentement (pour traitement basé sur consentement).
- ✅ Vous vous opposez au traitement (base intérêt légitime).
- ✅ Données traitées illégalement.

**Quand nous **ne pouvons pas** supprimer** (exceptions) :
- ❌ Obligation légale de conservation (ex : conservation 7 ans registres fiscaux).
- ❌ Réclamations légales (ex : litige en cours).
- ❌ Exercice de la liberté d'expression.

### 8.4 Droit à la portabilité des données (Article 20)

**Quoi** : Recevoir vos données dans un format lisible par machine (JSON, CSV, XML).
**Comment** : Email [privacy@acme.com] avec objet "Demande de portabilité des données".
**Délai de réponse** : 1 mois.
**Portée** : Uniquement les données que vous avez fournies (pas les données déduites ou dérivées).

**Nous fournirons** :
- Export de données structurées (format JSON).
- Possibilité de transmettre à un autre responsable de traitement (si techniquement faisable).

### 8.5 Droit d'opposition (Article 21)

**Quoi** : S'opposer au traitement basé sur l'intérêt légitime ou pour le marketing.

**Marketing direct** : Vous avez un **droit absolu** de vous opposer au marketing direct à tout moment.
- **Comment** : Cliquer "Se désabonner" dans les emails ou email [privacy@acme.com].
- **Délai de réponse** : Immédiat (vous ne recevrez plus de marketing).

**Traitement intérêt légitime** : Vous pouvez vous opposer si vous avez des motifs liés à votre situation particulière.
- **Comment** : Email [privacy@acme.com] expliquant votre opposition.
- **Délai de réponse** : 1 mois (nous évaluerons et répondrons).

### 8.6 Droit à la limitation du traitement (Article 18)

**Quoi** : Demander la suspension temporaire du traitement dans des situations spécifiques :
- Vous contestez l'exactitude des données (pendant vérification).
- Le traitement est illégal, mais vous préférez la limitation à l'effacement.
- Nous n'avons plus besoin des données, mais vous en avez besoin pour réclamations légales.

**Comment** : Email [privacy@acme.com] avec justification.
**Délai de réponse** : 1 mois.

### 8.7 Droits relatifs à la prise de décision automatisée (Article 22)

**Décisions automatisées** : [Choisir un]

- ☐ **Nous N'utilisons PAS de prise de décision automatisée** avec effets juridiques ou significatifs.
- ☐ **Nous utilisons la prise de décision automatisée** pour : [ex : "Scoring crédit, détection fraude, tarification personnalisée"]

**Si nous utilisons des décisions automatisées** :
- ✅ Vous avez le droit à une **intervention humaine**.
- ✅ Vous avez le droit d'**exprimer votre point de vue**.
- ✅ Vous avez le droit de **contester la décision**.

**Comment exercer** : Email [privacy@acme.com] demandant une revue manuelle.

### 8.8 Comment exercer vos droits

**Méthodes de contact** :
- **Email** : [privacy@acme.com] (préféré)
- **Courrier postal** : [Adresse complète]
- **In-App** : Paramètres Compte → Confidentialité → Droits aux Données

**Vérification d'identité** :
- Pour protéger vos données, nous vérifierons votre identité avant de traiter les demandes (ex : confirmer adresse email, répondre questions sécurité).

**Délai de réponse** : 1 mois (extensible à 3 mois pour demandes complexes - nous vous notifierons sous 1 mois).

**Frais** : Gratuit pour première demande, frais administratifs raisonnables pour demandes manifestement infondées ou excessives.

---

## 9. Cookies et technologies de suivi

**Politique cookies** : [Lien vers politique cookies complète, ex : "https://www.acme.com/cookie-policy"]

**Cookies que nous utilisons** :

| Type de cookie | Finalité | Consentement requis | Rétention |
|----------------|----------|---------------------|-----------|
| **Strictement nécessaires** | Authentification, sécurité, gestion de session | Non (essentiel pour le service) | Session ou 1 an |
| **Performance** | Analytics, suivi erreurs (Google Analytics) | Oui | 2 ans |
| **Fonctionnels** | Préférence langue, paramètres UI | Oui | 1 an |
| **Publicité** | Publicités personnalisées, reciblage | Oui | 1 an |

**Gérer les cookies** :
- **Bandeau cookies** : Personnaliser préférences lors de la première visite.
- **Paramètres cookies** : [Lien, ex : "Paramètres Compte → Confidentialité → Cookies"]
- **Paramètres navigateur** : Désactiver cookies dans le navigateur (peut affecter les fonctionnalités).

**Cookies tiers** :
- **Google Analytics** : [Lien opt-out : https://tools.google.com/dlpage/gaoptout]
- **Facebook Pixel** : [Lien opt-out : https://www.facebook.com/settings?tab=ads]

---

## 10. Confidentialité des enfants

**Restriction d'âge** : Notre service **n'est pas destiné aux enfants de moins de 16 ans** (ou âge local de consentement, ex : 15 ans en France).

**Consentement parental** :
- Nous ne collectons pas sciemment de données d'enfants sans consentement parental.
- Si nous apprenons avoir collecté des données d'un enfant sans consentement, nous les supprimerons sous 30 jours.
- **Parents** : Si vous pensez que votre enfant a fourni des données sans consentement, contactez [privacy@acme.com].

---

## 11. Modifications de cet avis de confidentialité

**Mises à jour** : Nous pouvons mettre à jour cet avis de confidentialité pour refléter :
- Changements dans nos pratiques de traitement de données
- Nouvelles exigences légales
- Améliorations du service

**Notification** :
- **Changements matériels** : Nous vous notifierons 30 jours à l'avance par email ou notification in-app.
- **Changements non matériels** (ex : clarifications) : Avis mis à jour publié avec nouvelle date "Dernière Mise à Jour".

**Revue** : Nous recommandons de revoir cet avis périodiquement pour les mises à jour.

**Historique des versions** : [Lien vers changelog, ex : "https://www.acme.com/privacy-history"]

---

## 12. Réclamations et autorité de contrôle

**Délégué à la protection des données** : Si vous avez des préoccupations concernant nos pratiques de données, contactez d'abord notre DPO :
- **Email** : [dpo@acme.com]
- **Téléphone** : [Numéro DPO]

**Autorité de contrôle** : Vous avez le droit de déposer plainte auprès de votre autorité locale de protection des données :

**France - CNIL** (Commission Nationale de l'Informatique et des Libertés) :
- **Site web** : https://www.cnil.fr/
- **Adresse** : 3 Place de Fontenoy, TSA 80715, 75334 Paris Cedex 07, France
- **Téléphone** : +33 1 53 73 22 22

**Autres pays UE** : Trouvez votre autorité de protection des données sur https://edpb.europa.eu/about-edpb/about-edpb/members_en

---

## 13. Nous contacter

**Questions, demandes ou réclamations** :

**Responsable de traitement** :
- **Nom** : [Nom entité juridique]
- **Email** : [privacy@acme.com]
- **Téléphone** : [Numéro avec indicatif pays]
- **Adresse** : [Adresse légale complète]

**Délégué à la protection des données** :
- **Nom** : [Nom DPO]
- **Email** : [dpo@acme.com]
- **Téléphone** : [Numéro DPO]

**Délai de réponse** : Nous visons à répondre à toutes les demandes sous 5 jours ouvrables (exigence légale : 1 mois pour demandes formelles de personne concernée).

---

## EXEMPLE : Avis de confidentialité plateforme SaaS

**Système** : ACME Project Management SaaS
**Utilisateurs** : 500 000 entreprises (5M utilisateurs individuels)
**Données** : Email, nom, adresse IP, analytics d'usage, données projet (documents, tâches)
**Base juridique** : Exécution du contrat (livraison service), Intérêt légitime (analytics), Consentement (marketing)

**Points clés** :
- **Délégué protection données** : dpo@acme.com
- **Conservation** : Données de compte conservées pour durée du compte + 30 jours après suppression
- **Transferts internationaux** : AWS UE (Francfort), Stripe (DPF UE-US), SendGrid (CCT)
- **Droits personnes concernées** : Email privacy@acme.com, délai réponse 1 mois
- **Cookies** : Strictement nécessaires (pas de consentement), Analytics (consentement requis via bandeau)
- **Enfants** : Pas destiné aux utilisateurs de moins de 16 ans
- **Autorité de contrôle** : CNIL (France), ICO (UK), BfDI (Allemagne)

---

## Liste de validation

**Avant de publier cet avis de confidentialité** :

- [ ] **Section 1** : Informations contact responsable traitement et DPO complètes
- [ ] **Section 2** : Toutes catégories de données listées (identification, techniques, sensibles)
- [ ] **Section 3** : Base juridique spécifiée pour chaque finalité (consentement, contrat, intérêt légitime, obligation légale)
- [ ] **Section 4** : Tous sous-traitants listés avec DPA en place
- [ ] **Section 5** : Transferts internationaux documentés avec garanties (CCT, adéquation, DPF)
- [ ] **Section 6** : Périodes conservation spécifiées pour toutes catégories de données
- [ ] **Section 7** : Mesures sécurité décrites (chiffrement, contrôle accès, audits)
- [ ] **Section 8** : Les 7 droits personnes concernées expliqués avec instructions d'exercice
- [ ] **Section 9** : Politique cookies liée ou intégrée
- [ ] **Section 10** : Confidentialité enfants traitée (restriction d'âge)
- [ ] **Section 11** : Processus notification changements défini
- [ ] **Section 12** : Informations contact autorité de contrôle fournies
- [ ] **Langage simple** : Revu par membre équipe non-juridique pour clarté
- [ ] **Accessibilité** : Avis confidentialité accessible depuis footer sur toutes pages
- [ ] **Revue juridique** : Approuvé par DPO et conseil juridique
- [ ] **Traductions** : Traduit dans langues de tous marchés cibles (si applicable)

---

**Version du modèle** : 1.0.0
**Dernière mise à jour** : 2025-01-19
**Mainteneur** : SSDLC Toolkit - Domaine RGPD

For questions or contributions, see [Contributing Guidelines](../../../.github/CONTRIBUTING.md).