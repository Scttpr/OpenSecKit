---
title: "Modèle de notification de violation de données RGPD (Articles 33-34)"
template_version: "1.0.0"
domain: "gdpr"
constitutional_principle: "VI"
regulatory_references:
  - "RGPD Article 33 (Notification d'une violation de données à caractère personnel à l'autorité de contrôle)"
  - "RGPD Article 34 (Communication d'une violation de données à caractère personnel à la personne concernée)"
  - "RGPD Article 4(12) (Définition de violation de données à caractère personnel)"
ssdlc_phase: "maintenance"
difficulty: "intermediate"
estimated_time: "4-8 heures (sous pression d'incident)"
last_updated: "2025-01-19"
reviewers:
  - name: "Délégué à la Protection des Données"
    role: "Conformité RGPD"
  - name: "RSSI / Responsable Réponse aux Incidents"
    role: "Gestion des Incidents de Sécurité"
---

# Modèle de notification de violation de données RGPD

# (Articles 33-34 : Notification sous 72 heures)

## Objectif

Ce modèle vous guide à travers le **processus obligatoire de notification de violation RGPD** lorsque des données personnelles sont compromises. Le RGPD exige une notification à :

1. **L'autorité de contrôle** (Autorité de protection des données) dans les **72 heures** (Article 33).
2. **Les personnes concernées** (individus affectés) **dans les meilleurs délais** si le risque est élevé (Article 34).

**Délai critique** : Le non-respect du délai de 72 heures peut entraîner des amendes allant jusqu'à **10 M€ ou 2 % du chiffre d'affaires annuel mondial** (le montant le plus élevé étant retenu). Le RGPD autorise la notification "échelonnée" (en plusieurs fois). Il vaut mieux notifier une information partielle à 72h que rien du tout, ou que de fausses informations complètes.

## Quand utiliser ce modèle

### Quand la notification est OBLIGATOIRE (Article 33)

Une violation doit être notifiée si elle est **susceptible d'engendrer un risque** pour les droits et libertés des personnes :

- ✅ **Violation de confidentialité** : Accès/divulgation non autorisé de données personnelles (ex : exfiltration par un pirate, consultation abusive par un employé).
- ✅ **Violation d'intégrité** : Altération non autorisée de données personnelles (ex : modification malveillante, chiffrement par ransomware).
- ✅ **Violation de disponibilité** : Perte d'accès aux données personnelles (ex : ransomware, suppression accidentelle, panne de service > 24h).

### Quand la notification n'est PAS requise (Exceptions)

- ❌ **Risque improbable** : Ex : violation de données chiffrées dont les clés ne sont pas compromises (le chiffrement rend les données inintelligibles).
- ❌ **Mesures éliminant le risque** : Ex : violation contenue avant tout accès non autorisé.
- ❌ **Données anonymes** : Données véritablement anonymisées (le RGPD ne s'applique pas).

**Question clé** : "Cette violation pourrait-elle entraîner un préjudice physique, matériel ou moral pour les personnes ?" (ex : vol d'identité, perte financière, discrimination, atteinte à la réputation, perte de confidentialité).

Si **oui** → Notifier dans les **72 heures**.
Si **non** mais violation avérée → Documenter en interne (notification non requise, mais conserver les registres pendant 3 ans).

---

## Chronologie de notification de violation

```
┌─────────────────────────────────────────────────────────────────┐
│             CHRONOLOGIE DE NOTIFICATION DE VIOLATION             │
└─────────────────────────────────────────────────────────────────┘

Heure 0 : PRISE DE CONSCIENCE DE LA VIOLATION
├── L'équipe sécurité détecte la violation (alerte SIEM, signalement utilisateur, etc.)
├── Commandant d'incident désigné
└── Démarrage du chronomètre (compte à rebours de 72 heures commence)

Heure 0-4 : ÉVALUATION INITIALE
├── Classifier la gravité de la violation (confidentialité/intégrité/disponibilité)
├── Estimer le périmètre (nombre de personnes, catégories de données)
├── Déterminer si la notification est requise (évaluation du risque)
└── Notifier le DPO et la direction

Heure 4-24 : INVESTIGATION & CONFINEMENT
├── Analyse forensique (vecteur d'attaque, données accédées, étendue)
├── Mesures de confinement (isoler les systèmes, révoquer les identifiants)
├── Préservation des preuves (logs, dumps mémoire, captures d'écran)
└── Commencer la rédaction de la notification

Heure 24-48 : PRÉPARATION DE LA NOTIFICATION
├── Compléter la Partie 1 de la notification (détails initiaux)
├── Revue et approbation du DPO
├── Revue du conseil juridique (si nécessaire)
└── Préparer la soumission

Heure 48-72 : SOUMISSION À L'AUTORITÉ
├── Soumettre la notification via le portail de l'autorité (ex : formulaire en ligne CNIL)
├── Email de confirmation de l'autorité (conserver pour les dossiers)
└── Si >72h, fournir une justification du retard

Heure 72+ : NOTIFICATION AUX PERSONNES CONCERNÉES (si risque élevé)
├── Évaluer si la violation présente un RISQUE ÉLEVÉ pour les personnes
├── Si oui : Notifier les personnes concernées DANS LES MEILLEURS DÉLAIS
├── Méthode : Email, notification in-app, annonce publique (si effort disproportionné)
└── Documenter toutes les notifications envoyées

En continu : SUIVI & REMÉDIATION
├── Répondre aux demandes de l'autorité (peut demander des informations supplémentaires)
├── Mettre en œuvre les mesures correctives
├── Informer les personnes concernées de la progression de la remédiation
└── Retour d'expérience post-incident (leçons apprises)
```

---

---

## PARTIE 1 : Notification à l'autorité de contrôle (Article 33)

**Délai** : **72 heures** après avoir **pris connaissance** de la violation.

**Prise de connaissance** : Le moment où vous avez un degré raisonnable de certitude qu'une violation de données personnelles s'est produite (pas quand la violation s'est produite, mais quand vous l'avez découverte).

### Étape 1.1 : Déterminer votre autorité de contrôle

**Autorité de contrôle chef de file** (Article 56) : Si vous avez des établissements dans plusieurs pays de l'UE, notifiez l'autorité de votre **établissement principal** (où les décisions concernant le traitement sont prises).

**Exemples** :

| Pays | Autorité de contrôle | Portail en ligne | Email | Téléphone |
|---------|----------------------|---------------|-------|-------|
| **France** | CNIL (Commission Nationale de l'Informatique et des Libertés) | <https://notifications.cnil.fr/> | <violations-rgpd@cnil.fr> | +33 1 53 73 22 22 |
| **Irlande** | DPC (Data Protection Commission) | <https://forms.dataprotection.ie/contact> | <info@dataprotection.ie> | +353 761 104 800 |

**Trouver votre autorité** : <https://edpb.europa.eu/about-edpb/about-edpb/members_en>

### Étape 1.2 : Formulaire de notification de violation (Article 33.3)

*Utilisez le formulaire officiel de votre autorité de contrôle (la plupart ont des portails en ligne). Ci-dessous figurent les informations que vous devez fournir.*

---

#### Section A : Identification du responsable de traitement

**Responsable de traitement** (votre organisation) :

- **Nom** : [Nom de l'entité juridique, ex : "ACME SaaS Inc."]
- **Numéro d'immatriculation** : [Numéro SIRET/SIREN]
- **Adresse** : [Adresse légale complète]
- **Email** : [Email de contact]
- **Téléphone** : [Numéro de téléphone]
- **Site web** : [URL]

**Délégué à la protection des données** (si désigné) :

- **Nom** : [Nom du DPO ou "N/A"]
- **Email** : [Email du DPO]
- **Téléphone** : [Téléphone du DPO]

**Personne de contact pour cette violation** :

- **Nom** : [Responsable de la réponse aux incidents]
- **Rôle** : [ex : "RSSI", "Responsable sécurité"]
- **Email** : [Email]
- **Téléphone** : [Téléphone 24/7]

---

#### Section B : Détails de la violation (Article 33.3.a-b)

**1. Date et heure de la violation** :

- **Violation survenue** : [Date/heure de survenance de la violation, si connue. Ex : "2025-01-15 14:30 UTC" ou "Inconnue - estimée entre le 2025-01-10 et le 2025-01-15"]
- **Violation découverte** : [Date/heure de prise de connaissance. Ex : "2025-01-16 09:00 UTC"]
- **Mode de découverte** : [ex : "Alerte SIEM pour accès base de données inhabituel", "Signalement utilisateur email phishing", "Divulgation chercheur sécurité externe"]

**2. Type de violation** (cocher tout ce qui s'applique) :

- ☐ **Violation de confidentialité** : Divulgation non autorisée ou accidentelle de données personnelles.
- ☐ **Violation d'intégrité** : Altération non autorisée ou accidentelle de données personnelles.
- ☐ **Violation de disponibilité** : Perte d'accès accidentelle ou non autorisée aux données personnelles.

**3. Description de la violation** :

*Fournir un récit clair et concis (2-5 paragraphes) :*

> **Exemple** :
> "Le 15 janvier 2025, vers 14h30 UTC, un tiers non autorisé a obtenu l'accès à notre base de données clients par le biais d'une vulnérabilité d'injection SQL dans la fonctionnalité de recherche d'utilisateurs. L'attaquant a exfiltré un fichier de sauvegarde de base de données contenant les données personnelles d'environ 50 000 clients européens.
>
> La violation a été découverte le 16 janvier 2025 à 09h00 UTC lorsque notre système de gestion des informations et des événements de sécurité (SIEM) a signalé des requêtes de base de données anormales (instructions SELECT sur la table clients avec un volume inhabituel). Suite à l'investigation, notre équipe de sécurité a identifié le vecteur d'attaque par injection SQL et confirmé l'exfiltration de données via l'analyse des logs de base de données et des logs de trafic réseau.
>
> L'attaquant a accédé à la base de données entre 14h30 UTC et 16h45 UTC le 15 janvier 2025 (2,25 heures). Nous avons confirmé qu'un fichier de sauvegarde de 1,2 Go a été téléchargé vers une adresse IP externe (185.220.101.XX, nœud de sortie Tor). La vulnérabilité a été corrigée à 10h30 UTC le 16 janvier 2025, et le serveur de base de données affecté a été isolé du réseau."

**Votre description de la violation** :
[Décrivez la violation clairement : ce qui s'est passé, quand, comment elle a été découverte, vecteur d'attaque si connu, durée de l'accès non autorisé]

**4. Responsable(s) de traitement affecté(s)** :

- ☐ **Nous sommes le responsable de traitement** (traitant les données pour notre propre compte).
- ☐ **Nous sommes un sous-traitant** (traitant les données pour le compte d'un autre responsable).
  - **Nom du responsable** : [Nom du responsable pour lequel nous traitons]
  - **Contact du responsable** : [Email/téléphone]
  - **Notification au responsable** : [Date/heure de notification au responsable]

---

#### Section C : Catégories de données et personnes concernées (Article 33.3.c)

**1. Catégories de données personnelles affectées** :

*Cocher toutes les catégories de données compromises :*

**Données d'identification** :

- ☐ Nom complet
- ☐ Date de naissance
- ☐ Numéro national d'identification (Numéro de Sécurité Sociale, NIR, etc.)
- ☐ Numéro de passeport/carte d'identité
- ☐ Numéro de permis de conduire
- ☐ Données biométriques (empreintes digitales, reconnaissance faciale)

**Coordonnées** :

- ☐ Adresse email
- ☐ Numéro de téléphone
- ☐ Adresse postale
- ☐ Adresse IP

**Données financières** :

- ☐ Numéro de carte bancaire (16 chiffres complets)
- ☐ Numéro de compte bancaire (IBAN)
- ☐ Historique de paiements
- ☐ Adresse de facturation

**Identifiants de connexion** :

- ☐ Mots de passe (texte clair) ← **CRITIQUE**
- ☐ Mots de passe (hachés) - Algorithme : [ex : "bcrypt facteur de travail 12"]
- ☐ Questions/réponses de sécurité
- ☐ Jetons MFA

**Catégories particulières** (Article 9 - risque élevé) :

- ☐ Origine raciale ou ethnique
- ☐ Opinions politiques
- ☐ Convictions religieuses ou philosophiques
- ☐ Appartenance syndicale
- ☐ Données génétiques
- ☐ Données de santé (dossiers médicaux, prescriptions)
- ☐ Vie sexuelle ou orientation sexuelle

**Autres données** :

- ☐ Données de localisation (GPS, historique de géolocalisation)
- ☐ Historique de navigation
- ☐ Historique d'achats
- ☐ Profils de réseaux sociaux
- ☐ Données d'emploi
- ☐ Données d'éducation
- ☐ Autre : [Préciser]

**2. Nombre de personnes concernées affectées** :

- **Nombre approximatif** : [ex : "50 000 personnes" ou "Inconnu - estimé entre 10 000 et 100 000"]
- **Répartition géographique** :
  - Résidents UE : [Nombre ou %]
  - France : [Nombre]
  - Allemagne : [Nombre]
  - Autres pays UE : [Liste]
  - Hors UE (si pertinent) : [Nombre]

**3. Catégories de personnes concernées** :

- ☐ Clients/utilisateurs
- ☐ Employés
- ☐ Candidats à l'emploi
- ☐ Fournisseurs/partenaires
- ☐ Enfants (moins de 16 ans)
- ☐ Personnes vulnérables (ex : patients, mineurs, personnes âgées)

---

#### Section D : Conséquences probables (Article 33.3.d)

**Préjudice potentiel pour les personnes concernées** :

*Évaluer l'impact probable sur les personnes (pas sur votre organisation) :*

| Catégorie de risque | Probabilité | Gravité | Explication |
|---------------|------------|----------|-------------|
| **Vol d'identité** | ☐ Faible ☐ Moyenne ☐ Élevée | ☐ Faible ☐ Moyenne ☐ Élevée | [ex : "Probabilité élevée - noms complets, dates de naissance et adresses exposés, suffisant pour fraude identitaire"] |
| **Perte financière** | ☐ Faible ☐ Moyenne ☐ Élevée | ☐ Faible ☐ Moyenne ☐ Élevée | [ex : "Probabilité moyenne - numéros de cartes partiels (4 derniers chiffres) exposés, risque de fraude limité"] |
| **Discrimination** | ☐ Faible ☐ Moyenne ☐ Élevée | ☐ Faible ☐ Moyenne ☐ Élevée | [ex : "Probabilité faible - aucune catégorie particulière de données (santé, origine ethnique) exposée"] |
| **Atteinte à la réputation** | ☐ Faible ☐ Moyenne ☐ Élevée | ☐ Faible ☐ Moyenne ☐ Élevée | [ex : "Probabilité élevée - adresses email et noms exposés, risque de phishing/spam ciblé"] |
| **Sécurité physique** | ☐ Faible ☐ Moyenne ☐ Élevée | ☐ Faible ☐ Moyenne ☐ Élevée | [ex : "Probabilité faible - aucune donnée de localisation ou information sensible pouvant mettre en danger"] |
| **Préjudice psychologique** | ☐ Faible ☐ Moyenne ☐ Élevée | ☐ Faible ☐ Moyenne ☐ Élevée | [ex : "Probabilité moyenne - anxiété suite à notification violation, peur du vol d'identité"] |
| **Perte de confidentialité** | ☐ Faible ☐ Moyenne ☐ Élevée | ☐ Faible ☐ Moyenne ☐ Élevée | [ex : "Gravité élevée - données de santé exposées, violation significative de la vie privée"] |

**Évaluation globale du risque** :

- ☐ **Risque faible** : Préjudice improbable (notification à l'autorité non requise, mais documenter en interne).
- ☐ **Risque moyen** : Préjudice possible (notification à l'autorité requise, notification personnes concernées évaluée cas par cas).
- ☐ **Risque élevé** : Préjudice significatif probable (notification à l'autorité ET aux personnes concernées requise).

**Justification** :
[2-3 phrases expliquant votre évaluation globale du risque]

**Exemple** :
> "Cette violation présente un **risque élevé** pour les personnes concernées. Les données exposées incluent les noms complets, adresses email, numéros de téléphone et mots de passe hachés (bcrypt). Bien que les mots de passe soient hachés, il existe un risque d'attaques par bourrage d'identifiants si les utilisateurs réutilisent leurs mots de passe. La combinaison d'informations de contact et de détails de compte crée un risque élevé d'attaques de phishing ciblées et de potentielles prises de contrôle de comptes. Le risque de vol d'identité est modéré en raison des données d'identification limitées (pas de numéros d'identité nationaux ou d'informations financières exposés)."

---

#### Section E : Mesures prises (Article 33.3.e)

**1. Mesures de confinement immédiates** (dans les 24 heures) :

*Qu'avez-vous fait pour arrêter la violation et prévenir d'autres préjudices ?*

- [ ] **Vulnérabilité corrigée** : [ex : "Vulnérabilité d'injection SQL corrigée à 10h30 UTC le 2025-01-16, déployée en production"]
- [ ] **Systèmes affectés isolés** : [ex : "Serveur de base de données déconnecté d'internet à 09h30 UTC, accessible uniquement via VPN"]
- [ ] **Identifiants compromis révoqués** : [ex : "Tous les mots de passe admin base de données réinitialisés, clés API rotées"]
- [ ] **Accès non autorisé bloqué** : [ex : "Règles pare-feu mises à jour pour bloquer les plages IP de l'attaquant (nœuds de sortie Tor)"]
- [ ] **Preuves forensiques préservées** : [ex : "Logs de base de données, captures de trafic réseau, dumps mémoire préservés pour analyse"]

**2. Mesures d'atténuation pour réduire le préjudice aux personnes concernées** :

*Que faites-vous pour réduire l'impact sur les personnes affectées ?*

- [ ] **Réinitialisation de mot de passe requise** : [ex : "Tous les utilisateurs affectés doivent réinitialiser leur mot de passe à la prochaine connexion"]
- [ ] **Application MFA** : [ex : "Authentification multifacteur activée par défaut pour tous les utilisateurs (2025-01-17)"]
- [ ] **Surveillance de crédit offerte** : [ex : "12 mois de surveillance de crédit gratuite offerts aux utilisateurs affectés via Experian"]
- [ ] **Surveillance renforcée** : [ex : "Surveillance d'activité de compte augmentée, alertes pour connexions suspectes (voyage impossible, nouveaux appareils)"]
- [ ] **Ligne d'assistance client** : [ex : "Email dédié support violation (breach-support@acme.com) et ligne téléphonique (+33 1 23 45 67 89) disponibles 24/7"]

**3. Mesures de remédiation à long terme** :

*Que mettez-vous en place pour prévenir de futures violations ?*

| Mesure | Description | Échéance | Responsable |
|---------|-------------|----------|-------|
| **SAST dans CI/CD** | Déployer l'analyse statique (SonarQube) pour détecter les patterns d'injection SQL en revue de code | 2025-01-30 | Équipe développement |
| **Règles WAF** | Mettre à jour le Web Application Firewall (ModSecurity) avec règles de détection d'injection SQL | 2025-01-20 | Équipe sécurité |
| **Tests d'intrusion** | Effectuer un test d'intrusion complet de l'application pour identifier des vulnérabilités similaires | 2025-02-15 | Prestataire externe (Synacktiv) |
| **Formation sécurité** | Formation obligatoire au codage sécurisé pour tous les développeurs (OWASP Top 10) | 2025-02-28 | RH + Sécurité |
| **Chiffrement base de données** | Implémenter le chiffrement transparent des données (TDE) pour toutes les bases de données (AES-256) | 2025-03-31 | Équipe infrastructure |

---

#### Section F : Éléments transfrontaliers (Si applicable)

**1. Cette violation affecte-t-elle des personnes concernées dans plusieurs pays de l'UE ?**

- ☐ Non (un seul pays).
- ☐ Oui (plusieurs pays de l'UE).

**Si oui** :

- **Pays principalement affecté** : [ex : "France (30 000 personnes)"]
- **Autres pays affectés** : [ex : "Allemagne (10 000), Pays-Bas (5 000), Espagne (5 000)"]
- **Avez-vous notifié les autres autorités concernées ?** : ☐ Oui ☐ Non ☐ En cours
  - Si oui, lister les autorités notifiées : [ex : "BfDI (Allemagne) le 2025-01-16, AP (Pays-Bas) le 2025-01-16"]

**2. Cette violation implique-t-elle des transferts internationaux de données (hors UE/EEE) ?**

- ☐ Non.
- ☐ Oui.

**Si oui** :

- **Données transférées vers** : [Pays]
- **Base juridique du transfert** : [ex : "Clauses contractuelles types", "Cadre de protection des données UE-US"]
- **Garanties en place** : [ex : "Chiffrement en transit, DPA sous-traitant"]

---

#### Section G : Notification retardée (Si > 72 heures)

**Si notification après 72 heures, fournir les raisons du retard** :

- ☐ **Investigation en cours** : [ex : "Nous avons eu besoin de 96 heures pour déterminer l'étendue complète de la violation et confirmer les catégories de données affectées"]
- ☐ **Analyse forensique** : [ex : "Les experts forensiques externes ont nécessité du temps supplémentaire pour analyser les logs et identifier le vecteur d'attaque"]
- ☐ **Incident complexe** : [ex : "Attaque en plusieurs étapes nécessitant la corrélation de données de plusieurs systèmes"]
- ☐ **Autre** : [Préciser]

**Justification** (2-3 phrases) :
[Expliquer pourquoi il n'a pas été possible de notifier dans les 72 heures]

**Exemple** :
> "Nous notifions 96 heures après la prise de connaissance en raison de la complexité de la violation. L'attaque impliquait plusieurs points d'entrée (injection SQL et phishing), et nous avons requis une analyse forensique supplémentaire pour déterminer l'étendue complète des données accédées. Des experts en cybersécurité externes ont été engagés le 16 janvier 2025 pour mener une investigation complète, qui s'est achevée le 18 janvier 2025. Nous avons priorisé l'exactitude de la notification sur la rapidité pour garantir que nous fournissions des informations complètes à l'autorité."

---

#### Section H : Informations additionnelles

**Notification échelonnée** (Article 33.4) :

Si vous n'êtes pas en mesure de fournir toutes les informations en une fois, vous pouvez soumettre une **notification échelonnée** :

- ☐ **Ceci est une notification initiale** (plus de détails à suivre).
  - **Date de suivi prévue** : [Date]
  - **Informations manquantes** : [ex : "Décompte final des personnes affectées (investigation en cours)"]

- ☐ **Ceci est une notification complète** (toutes les informations fournies).

**Documentation à l'appui** :

- [ ] Rapport d'analyse forensique
- [ ] Logs de base de données (expurgés)
- [ ] Analyse de trafic réseau
- [ ] Capture d'écran des alertes SIEM
- [ ] Rapport de scan de vulnérabilités
- [ ] Plan de remédiation

---

### Étape 1.3 : Soumettre la notification

**Méthode de soumission** (varie selon l'autorité) :

- **Portail en ligne** : [ex : "Portail de notification de la CNIL" ← **Préféré**
- **Email** : [ex : "violations-rgpd@cnil.fr"]
- **Courrier postal** : [Seulement si en ligne/email indisponible]

**Confirmation** :

- [ ] Notification soumise
- [ ] Email de confirmation reçu de l'autorité
- [ ] Numéro de référence de confirmation : [ex : "CNIL-2025-01234"]
- [ ] Horodatage de soumission : [Date/heure]

**Conservation des registres** (Article 33.5) :

- Documenter la violation, incluant :
  - Faits de la violation
  - Effets de la violation
  - Mesures correctives prises
- Conserver les registres pendant **au moins 3 ans**
- L'autorité peut demander les registres lors d'un audit/investigation

---

## PARTIE 2 : Notification aux personnes concernées (Article 34)

**Délai** : **Dans les meilleurs délais** (typiquement sous 24-72 heures après notification à l'autorité).

**Quand requis** : Seulement si la violation est **susceptible d'engendrer un RISQUE ÉLEVÉ** pour les droits et libertés des personnes.

### Étape 2.1 : Évaluer si la notification aux personnes concernées est requise

**Indicateurs de risque élevé** :

- ✅ **Catégories particulières** de données exposées (santé, origine ethnique, religion, etc.).
- ✅ **Données financières** exposées (cartes bancaires, comptes bancaires).
- ✅ **Violation à grande échelle** (dizaines de milliers de personnes).
- ✅ **Personnes vulnérables** affectées (enfants, personnes âgées, patients).
- ✅ **Risque de vol d'identité** (combinaison nom + date naissance + adresse + numéro ID).
- ✅ **Absence d'atténuation efficace** (données non chiffrées, pas de protections secondaires).

**Décision** :

- ☐ **Notification aux personnes concernées REQUISE** (risque élevé).
- ☐ **Notification aux personnes concernées NON REQUISE** (risque moyen/faible, ou exception s'applique).

**Exceptions** (Article 34.3) - Notification NON requise si :

1. **Protection technique appropriée** : [ex : "Les données étaient chiffrées en AES-256, et les clés de chiffrement n'ont PAS été compromises"]
   - ☐ S'applique
   - Justification : [Expliquer la mise en œuvre du chiffrement]

2. **Mesures ultérieures éliminent le risque élevé** : [ex : "Tous les comptes compromis immédiatement désactivés, réinitialisation de mot de passe forcée, MFA activée"]
   - ☐ S'applique
   - Justification : [Expliquer les mesures prises]

3. **Effort disproportionné** : [ex : "250 000 personnes affectées, la notification individuelle nécessiterait des ressources excessives. Annonce publique utilisée à la place."]
   - ☐ S'applique
   - Alternative : [ex : "Annonce publique sur site web + communiqué de presse + réseaux sociaux"]

**Approbation du DPO** : Avez-vous consulté votre DPO sur cette décision ?

- [ ] Oui - Le DPO a approuvé l'approche de notification
- [ ] Non - Pas de DPO désigné
- [ ] En cours

---

### Étape 2.2 : Rédiger la notification aux personnes concernées (Article 34.2)

**Exigences de communication** :

- **Langage clair et simple** (éviter le jargon technique, juridique).
- **Format facilement accessible** (email, in-app, SMS, courrier postal).
- **Conseils actionnables** (ce que les personnes doivent faire).

**Modèle d'email aux personnes concernées** :

---

**Objet** : Avis de sécurité important - Violation de données affectant votre compte

**Corps de l'email** :

Cher(e) [Nom],

Nous vous écrivons pour vous informer d'un incident de sécurité qui a pu affecter vos données personnelles.

**Ce qui s'est passé**

Le [Date], nous avons découvert qu'un tiers non autorisé a obtenu l'accès à nos systèmes par le biais d'une vulnérabilité de sécurité. En conséquence, certaines de vos informations personnelles ont pu être accédées.

**Quelles données ont été affectées**

Les catégories suivantes de vos données personnelles ont pu être accédées :

- [Lister les catégories de données, ex : "Nom, adresse email, numéro de téléphone, mot de passe haché"]
- [Préciser si catégories particulières : "Cela inclut des données de santé / données financières"]

**Ce que nous faisons**

Nous avons pris des mesures immédiates pour traiter cet incident :

- [Action 1 : ex : "Nous avons corrigé la vulnérabilité de sécurité le [Date]"]
- [Action 2 : ex : "Nous avons réinitialisé tous les mots de passe utilisateurs et requis un changement de mot de passe"]
- [Action 3 : ex : "Nous avons engagé des experts externes en cybersécurité pour investiguer"]
- [Action 4 : ex : "Nous avons signalé cet incident à [l'Autorité de protection des données] le [Date]"]

**Ce que vous devez faire**

Pour protéger votre compte, nous recommandons les actions suivantes :

1. **Réinitialisez immédiatement votre mot de passe** (si pas déjà fait automatiquement)
   - Utilisez un mot de passe fort et unique (12+ caractères, mélange de lettres/chiffres/symboles).
   - NE réutilisez PAS de mots de passe d'autres comptes.

2. **Activez l'authentification multifacteur (MFA)**
   - Allez dans Paramètres du compte → Sécurité → Activer MFA.
   - Cela ajoute une couche de sécurité supplémentaire à votre compte.

3. **Surveillez votre compte pour toute activité suspecte**
   - Vérifiez votre journal d'activité de compte : [Lien]
   - Si vous remarquez un accès non autorisé, contactez-nous immédiatement.

4. **Soyez prudent avec les emails de phishing**
   - Nous ne vous demanderons JAMAIS de fournir votre mot de passe par email.
   - Vérifiez les adresses email de l'expéditeur avant de cliquer sur des liens.

5. **[Si données financières exposées] Surveillez vos relevés bancaires/carte de crédit**
   - Surveillez les transactions non autorisées.
   - Envisagez de placer une alerte fraude auprès des bureaux de crédit.

**Support additionnel**

Nous prenons cet incident très au sérieux et vous présentons nos sincères excuses pour toute préoccupation que cela pourrait causer.

Si vous avez des questions ou besoin d'assistance :

- **Email** : [breach-support@acme.com]
- **Téléphone** : [+33 1 23 45 67 89] (ligne de support 24/7)
- **FAQ** : [Lien vers page FAQ violation]

**[Si surveillance de crédit offerte]**
Nous offrons 12 mois de surveillance de crédit gratuite et de protection contre le vol d'identité. Pour vous inscrire, visitez : [Lien]

**Vos droits**

En vertu de la loi sur la protection des données, vous avez le droit de :

- Demander une copie de vos données personnelles ([Lien vers formulaire de demande d'accès]).
- Déposer une plainte auprès de [l'Autorité de protection des données] : [Site web/téléphone].

Pour plus d'informations sur cet incident, veuillez visiter : [Lien vers page d'information dédiée à la violation]

Cordialement,

[Nom]
[Titre, ex : "Responsable de la sécurité des systèmes d'information"]
[Nom de l'entreprise]

---

**Checklist de notification** (Article 34.2) :

- [ ] **Nature de la violation** décrite en langage clair
- [ ] **Point de contact** fourni (DPO ou équipe sécurité)
- [ ] **Conséquences probables** expliquées
- [ ] **Mesures prises** pour traiter la violation listées
- [ ] **Actions recommandées** pour les personnes concernées fournies

---

### Étape 2.3 : Envoyer les notifications

**Méthode de notification** :

- ☐ **Email** (préféré pour la plupart des cas)
  - [ ] Email envoyé à : [Nombre de personnes]
  - [ ] Date/heure d'envoi : [Horodatage]
  - [ ] Suivi de livraison : [Utiliser un service de livraison d'emails avec suivi]

- ☐ **Notification in-app** (pour applications mobiles, web)
  - [ ] Bannière de notification affichée à la connexion
  - [ ] Dialogue modal avec accusé de réception requis

- ☐ **SMS** (si numéro de téléphone disponible et approprié)
  - [ ] SMS envoyé à : [Nombre de personnes]

- ☐ **Courrier postal** (si pas d'email/téléphone, ou personnes à haut risque)
  - [ ] Lettres envoyées à : [Nombre de personnes]
  - [ ] Date d'envoi : [Date]

- ☐ **Annonce publique** (si effort disproportionné pour contacter individuellement)
  - [ ] Bannière site web : [URL]
  - [ ] Communiqué de presse : [Date, médias]
  - [ ] Réseaux sociaux : [Plateformes, liens de posts]
  - [ ] Notification dans journal national : [Nom, date]

**Timing** :

- **Envoyé** : [Date/heure]
- **Justification du retard** (si > 72h après découverte) : [Raison]

**Documentation** :

- [ ] Copie de l'email/lettre de notification sauvegardée
- [ ] Liste des destinataires (pseudonymisée pour la confidentialité)
- [ ] Confirmation de livraison (rapports de livraison email, reçus postaux)
- [ ] Réponses suivies (tickets de support, appels téléphoniques)

---

## PARTIE 3 : Documentation interne

### Registre de violation (Article 33.5)

**Toutes les violations doivent être documentées en interne**, même si non notifiées à l'autorité.

**ID de violation** : [ex : "BREACH-2025-001"]

**Ticket d'incident interne** : [ex : "INC-20250116-001"]

**Commandant d'incident** : [Nom, rôle]

**Chronologie** :

| Événement | Date/Heure | Notes |
|-------|-----------|-------|
| Violation survenue (estimée) | [Horodatage] | [Comment déterminé] |
| Violation découverte | [Horodatage] | [Méthode de découverte] |
| DPO notifié | [Horodatage] | [Méthode de notification] |
| Autorité notifiée | [Horodatage] | [Réf. de confirmation #] |
| Personnes concernées notifiées | [Horodatage] | [Méthode, # notifiées] |

**Analyse de cause racine** :

- **Vecteur d'attaque** : [ex : "Vulnérabilité d'injection SQL dans endpoint de recherche utilisateur (CVE-2025-XXXX)"]
- **Vulnérabilité introduite** : [ex : "Code legacy de 2020, concaténation de chaînes au lieu de requêtes préparées"]
- **Pourquoi non détectée plus tôt** : [ex : "SAST non configuré pour la base de code legacy, revue de code manuelle a manqué le problème"]
- **Facteurs contributeurs** : [ex : "Absence de Web Application Firewall (WAF), pas de limitation de débit sur endpoint de recherche"]

**Leçons apprises** :

- [ ] **Améliorations techniques** : [ex : "Déployer WAF, implémenter SAST dans CI/CD pour tout le code"]
- [ ] **Améliorations de processus** : [ex : "Audits de sécurité trimestriels pour code legacy, formation obligatoire codage sécurisé"]
- [ ] **Améliorations de détection** : [ex : "Règles SIEM améliorées pour patterns d'injection SQL, seuils d'alerte ajustés"]

**Réunion de retour d'expérience post-incident** :

- **Date** : [Date]
- **Participants** : [RSSI, DPO, Responsable développement, Équipe réponse aux incidents]
- **Actions à mener** : [Liste avec responsables et échéances]

---

## PARTIE 4 : Actions de suivi

### Suivi par l'autorité de contrôle

**L'autorité peut demander des informations supplémentaires** :

- [ ] **Rapport forensique** : Analyse technique détaillée de la violation.
- [ ] **Liste des personnes affectées** : Si l'autorité doit vérifier le périmètre (pseudonymisée).
- [ ] **Preuves de remédiation** : Preuve que les vulnérabilités ont été corrigées.
- [ ] **Mises à jour de politiques** : Politiques et procédures de sécurité mises à jour.

**Réponse de l'autorité** :

- **Date de réception** : [Date]
- **Questions/demandes de l'autorité** : [Résumé]
- **Date de notre réponse** : [Date]
- **Statut du dossier** : ☐ Ouvert ☐ Fermé ☐ En investigation

**Actions possibles de l'autorité** :

- ☐ **Aucune action** (violation traitée de manière appropriée).
- ☐ **Avertissement** (améliorer les mesures de sécurité).
- ☐ **Injonction corrective** (mesures spécifiques requises).
- ☐ **Investigation** (violations potentielles du RGPD).
- ☐ **Amende** (jusqu'à 10 M€ ou 2 % du CA pour défaut de notification, jusqu'à 20 M€ ou 4 % pour autres violations RGPD).

### Suivi des personnes concernées

**Métriques de support** :

- **Emails reçus** : [Nombre]
- **Appels téléphoniques reçus** : [Nombre]
- **Temps de réponse moyen** : [ex : "4 heures"]
- **Résolus** : [Nombre / %]
- **Escaladés** : [Nombre / %]

**Questions fréquentes** :

1. [Question 1, ex : "Mes informations de carte bancaire sont-elles en sécurité ?"]
   - **Réponse** : [Votre réponse]
2. [Question 2, ex : "Dois-je fermer mon compte ?"]
   - **Réponse** : [Votre réponse]

**Page FAQ** : [Lien vers page FAQ dédiée à la violation sur le site web]

---

## EXEMPLE : Violation plateforme E-Commerce

**Scénario** : Violation par injection SQL exposant 50 000 enregistrements clients

**Détails de la violation** :

- **Date** : 2025-01-15 14:30 UTC (survenue), 2025-01-16 09:00 UTC (découverte)
- **Vecteur d'attaque** : Injection SQL dans la recherche de produits
- **Données affectées** : Nom, email, téléphone, mot de passe haché (bcrypt), adresse de facturation (pas de cartes bancaires - tokenisées par Stripe)
- **Personnes affectées** : 50 000 clients UE (30 000 France, 10 000 Allemagne, 5 000 Pays-Bas, 5 000 autre UE)

**Évaluation du risque** : **Risque élevé**

- Probabilité élevée d'attaques de phishing (email + nom exposés).
- Probabilité moyenne de bourrage d'identifiants (mots de passe hachés, mais risque de réutilisation).
- Probabilité faible de fraude financière (pas de données de paiement exposées).

**Actions prises** :

1. **Confinement** (Heure 0-4) :
   - Vulnérabilité corrigée (requêtes préparées implémentées).
   - Serveur de base de données isolé.
   - IP de l'attaquant bloquée (nœuds de sortie Tor).
2. **Notification** (Heure 24-72) :
   - Autorité notifiée : CNIL (France) à l'heure 48 (sous 72h).
   - Autres autorités notifiées : BfDI (Allemagne), AP (Pays-Bas) à l'Heure 50.
   - Personnes concernées notifiées : Email envoyé aux 50 000 à l'Heure 60.
3. **Remédiation** (Semaine 1-4) :
   - Réinitialisation forcée de mot de passe pour tous les utilisateurs.
   - MFA activée par défaut.
   - WAF déployé (ModSecurity).
   - SAST intégré dans CI/CD (SonarQube).
   - Test d'intrusion planifié (prestataire externe).

**Résultat** :

- **Réponse de l'autorité** : La CNIL a examiné la notification, aucune action supplémentaire requise (réponse jugée appropriée).
- **Impact utilisateurs** : 12 % des utilisateurs ont contacté le support, 95 % résolus sous 48h, 0 incident de fraude signalé.
- **Coût** : 120 000 € (forensique : 30k €, remédiation : 50k €, surveillance de crédit : 40k €).

---

## Checklist de validation

**Avant de soumettre la notification de violation** :

### Notification à l'autorité (Article 33)

- [ ] **Délai** : Notification soumise sous 72 heures (ou retard justifié).
- [ ] **Section A** : Identification du responsable de traitement complète (DPO si désigné).
- [ ] **Section B** : Détails de la violation (date, heure, type, description).
- [ ] **Section C** : Catégories de données et nombre de personnes affectées.
- [ ] **Section D** : Conséquences probables évaluées (risque pour les personnes).
- [ ] **Section E** : Mesures prises (confinement, atténuation, remédiation).
- [ ] **Section F** : Éléments transfrontaliers (si applicable).
- [ ] **Section G** : Justification du retard (si > 72h).
- [ ] **Confirmation** : Email de confirmation de l'autorité reçu, numéro de référence enregistré.

### Notification aux personnes concernées (Article 34)

- [ ] **Évaluation du risque élevé** : Déterminé si risque élevé pour les personnes (si oui, notifier).
- [ ] **Exceptions examinées** : Vérifié si exceptions s'appliquent (chiffrement, mesures éliminent le risque, effort disproportionné).
- [ ] **Consultation du DPO** : Le DPO a approuvé la décision de notification.
- [ ] **Langage clair** : Notification rédigée en langage simple (testé avec lecteur non technique).
- [ ] **Informations requises** : Nature, point de contact, conséquences, mesures, recommandations incluses.
- [ ] **Notification envoyée** : Email/SMS/postal/annonce publique envoyé(e) dans les meilleurs délais.
- [ ] **Support prêt** : Email/ligne téléphonique de support dédié(e) opérationnel(le) et prêt(e).

### Documentation interne

- [ ] **Registre de violation** : Enregistrement complet documenté (Article 33.5).
- [ ] **Analyse de cause racine** : Vecteur d'attaque identifié, facteurs contributeurs analysés.
- [ ] **Leçons apprises** : Retour d'expérience post-incident complété, actions à mener assignées.
- [ ] **Preuves préservées** : Preuves forensiques (logs, captures d'écran, rapports) conservées pendant 3+ ans.

---

**Version du modèle** : 1.0.0
**Dernière mise à jour** : 2025-01-19
**Mainteneur** : SSDLC Toolkit - Domaine RGPD

Pour toute question ou contribution, voir [Directives de contribution](../../../CONTRIBUTING.md).
