# Domaine RGPD – Conformité opérationnelle

Ce module fournit un kit complet pour mettre en œuvre la conformité RGPD (Règlement (UE) 2016/679) au sein de vos projets logiciels.

L'objectif est de passer d'une conformité « théorique » à une conformité **opérationnelle**, alignée avec la réalité technique de votre code.

## ⚡️ Modes d'utilisation

Choisissez l'approche qui correspond à votre maturité et à vos outils :

### 🤖 Option A : Workflow automatisé (Agent IA)
*Idéal pour : Équipes Tech, CI/CD, Projets complexes.*

1.  **Extraction** : Un agent IA analyse votre code pour préremplir le contexte.
2.  **Centralisation** : Vous validez un fichier unique (`gdpr-context.yaml`).
3.  **Génération** : Les documents sont générés automatiquement depuis ce contexte.

### ✍️ Option B : Workflow manuel (Classique)
*Idéal pour : Petits projets, DPO sans accès au code, Démarrage rapide.*

1.  **Sélection** : Choisissez le modèle pertinent dans le catalogue ci-dessous.
2.  **Édition** : Recherchez les textes entre crochets `[...]` et remplacez-les par vos informations.
3.  **Validation** : Relisez et exportez le document final.

---

## 🛠️ Le système de configuration (Pour l'option A)

### 1. La source de vérité : `gdpr-context.yaml`

Si vous choisissez l'automatisation, ce fichier (à placer à la racine, ex : `.github/compliance/`) centralise toutes les données variables. Il permet de mettre à jour cinq documents en ne modifiant qu'une seule ligne.

### 2. Comment générer le contexte (Prompt système)

Fournissez ce prompt à votre assistant IA (Claude, GPT-4o, etc.) avec votre code et le fichier `gdpr-context.skeleton.yaml` :

```text
Rôle : Expert en audit de conformité technique.
Tâche : Générer le fichier 'gdpr-context.yaml' via l'analyse du code.
Instructions :
1. IDENTITÉ : Scanne README/LICENSE pour le nom légal et les contacts.
2. INFRA : Scanne Terraform/Docker pour le chiffrement et les backups.
3. SOUS-TRAITANTS : Scanne package.json/go.mod pour les services tiers (AWS, Stripe...).
4. DONNÉES : Scanne les schémas BDD pour les champs personnels (PII).
5. FORMAT : Remplis le template squelette. Utilise <HUMAN_INPUT_REQUIRED> si l'information est inconnue.
```

## 📂 Catalogue des modèles

Ces modèles sont conçus pour être juridiquement robustes tout en restant compréhensibles pour des équipes techniques.

| Fichier | Titre | Usage / Obligation RGPD |
| :--- | :--- | :--- |
| [`gdpr-privacy-notice-template.md`](templates/gdpr-privacy-notice-template.md) | **Avis de confidentialité** | **Public**. Informe les utilisateurs (Art. 13-14). À publier sur le site/app. |
| [`gdpr-records-of-processing.md`](templates/gdpr-records-of-processing.md) | **Registre des traitements** | **Interne**. Inventaire obligatoire des flux de données (Art. 30). À présenter à la CNIL sur demande. |
| [`gdpr-data-processing-agreement.md`](templates/gdpr-data-processing-agreement.md) | **DPA (Accord de traitement)** | **Contractuel**. À signer avec chaque sous-traitant ou client B2B (Art. 28). |
| [`gdpr-breach-notification-template.md`](templates/gdpr-breach-notification-template.md) | **Notification de violation** | **Urgence**. Procédure et modèle pour la réponse aux incidents sous 72 h (Art. 33-34). |
| [`gdpr-dpia-template.md`](templates/gdpr-dpia-template.md) | **AIPD (Analyse d'impact)** | **Risque**. Obligatoire pour les traitements à haut risque (Art. 35). |

---

## 🔗 Correspondance avec OpenSecKit

La conformité RGPD s'appuie sur les principes de sécurité constitutionnels du framework :

| Principe SSDLC | Exigence RGPD | Implémentation |
| :--- | :--- | :--- |
| **I - Threat Modeling** | Privacy by Design (Art. 25) | Intégré dans l'AIPD |
| **II - Risk Analysis** | Approche par les risques | Matrice de risques dans l'AIPD |
| **III - Security by Design** | Mesures techniques (Art. 32) | Annexe B du DPA et Registre |
| **V - Secrets Management** | Chiffrement / Pseudonymisation | Vérification technique |
| **VI - Audit Logging** | Responsabilité (Accountability) | Registre (RoPA) et logs d'incidents |

---

## 🚦 Premiers pas

**Pour l'option automatisée :**
1.  Copiez `gdpr-context.skeleton.yaml` dans votre projet.
2.  Lancez le prompt système sur votre code.
3.  Validez le YAML généré.
4.  Injectez les variables dans les modèles.

**Pour l'option manuelle :**
1.  Ouvrez le fichier [`gdpr-records-of-processing.md`](templates/gdpr-records-of-processing.md) pour commencer votre inventaire.
2.  Recherchez `[...` dans votre éditeur pour trouver les champs à remplir.
3.  Supprimez les sections non pertinentes (ex : supprimez la partie « Sous-traitant » si vous êtes « Responsable »).