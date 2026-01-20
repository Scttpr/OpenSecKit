---
title: "Bibliothèque de vulnérabilités standards (Mappées STRIDE)"
template_version: "1.1.0"
constitutional_principle: "I"
ssdlc_phase: "planning"
last_updated: "2025-11-29"
description: "Catalogue des vulnérabilités (faiblesses) courantes qui exposent le système aux menaces STRIDE. À utiliser pour identifier rapidement les manques de sécurité standards."
tags:
  - vulnerability-library
  - stride
  - common-weaknesses
usage: "Vérifier si ces vulnérabilités sont présentes dans l'architecture pour prévenir les menaces associées."
---

# Bibliothèque de vulnérabilités standards

## Concept
Ce document établit le lien entre les **Menaces** (l'objectif de l'attaquant, classé par STRIDE) et les **Vulnérabilités** (les faiblesses du système qui rendent l'attaque possible).

---

## S - Spoofing (Menace : Usurpation d'identité)
*L'attaquant prétend être une autre personne ou entité.*

| ID | Vulnérabilité (La faiblesse) | Scénario de menace (L'attaque) | Contre-mesures |
|:---|:---|:---|:---|
| **VULN-S01** | **Authentification faible** | Un attaquant devine ou force le mot de passe, ou accède à une API publique sans contrôle. | • Implémenter OAuth2 / OIDC<br>• Exiger l'authentification sur tous les endpoints |
| **VULN-S02** | **Gestion de session défaillante** | Un attaquant vole un cookie ou un jeton (via XSS ou MITM) et l'utilise pour usurper l'identité de l'utilisateur. | • Cookies : `Secure`, `HttpOnly`, `SameSite`<br>• JWT : Expiration courte + rotation |
| **VULN-S03** | **Absence d'authentification multi-facteurs (MFA)** | Un attaquant utilise des identifiants volés (phishing) pour accéder à un compte critique. | • Imposer le MFA pour les accès administrateur et les données sensibles |

---

## T - Tampering (Menace : Altération de données)
*L'attaquant modifie des données sur le disque, en mémoire ou sur le réseau.*

| ID | Vulnérabilité (La faiblesse) | Scénario de menace (L'attaque) | Contre-mesures |
|:---|:---|:---|:---|
| **VULN-T01** | **Transport non chiffré (HTTP)** | Un attaquant sur le réseau (Man-in-the-Middle) intercepte et modifie les paquets à la volée. | • HTTPS (TLS 1.2+) partout<br>• HSTS activé |
| **VULN-T02** | **Absence de protection CSRF** | Un attaquant force le navigateur de la victime à envoyer une requête de modification non désirée. | • Jetons anti-CSRF<br>• Vérification `Origin`/`Referer`<br>• Cookies `SameSite=Strict` |
| **VULN-T03** | **Absence de signature de données** | Un attaquant modifie les paramètres (prix, ID) côté client avant de les envoyer au serveur. | • Signer les données sensibles (HMAC)<br>• Recalculer les valeurs critiques côté serveur |

---

## R - Repudiation (Menace : Répudiation)
*L'attaquant nie avoir effectué une action, ou un administrateur ne peut pas prouver qui a fait quoi.*

| ID | Vulnérabilité (La faiblesse) | Scénario de menace (L'attaque) | Contre-mesures |
|:---|:---|:---|:---|
| **VULN-R01** | **Journalisation insuffisante ou absente** | Impossible de tracer l'origine d'une action malveillante après un incident. | • Journaliser : Qui, Quoi, Quand, Où, Résultat<br>• Centraliser les journaux (logs) |
| **VULN-R02** | **Stockage de journaux non sécurisé** | Un attaquant efface les journaux après son intrusion pour supprimer les preuves. | • Stockage WORM (Write Once Read Many)<br>• Envoi en temps réel vers un SIEM externe |

---

## I - Information Disclosure (Menace : Divulgation d'information)
*L'attaquant accède à des données confidentielles.*

| ID | Vulnérabilité (La faiblesse) | Scénario de menace (L'attaque) | Contre-mesures |
|:---|:---|:---|:---|
| **VULN-I01** | **Secrets codés en dur** | Un attaquant scanne le dépôt Git et trouve des clés API pour accéder aux systèmes. | • Scanner avec Gitleaks<br>• Utiliser un coffre-fort (Vault) |
| **VULN-I02** | **Gestion d'erreurs verbeuse** | Un attaquant provoque une erreur pour lire la trace de la pile (stack trace) et comprendre l'architecture. | • Messages d'erreurs génériques pour l'utilisateur<br>• Journaux détaillés côté serveur uniquement |
| **VULN-I03** | **Journaux trop verbeux (fuite de données)** | Un attaquant accède aux fichiers de journaux et y lit des mots de passe ou données personnelles enregistrés par erreur. | • Masquage (redaction) des données sensibles dans les journaux |

---

## D - Denial of Service (Menace : Déni de service)
*L'attaquant rend le système indisponible pour les utilisateurs légitimes.*

| ID | Vulnérabilité (La faiblesse) | Scénario de menace (L'attaque) | Contre-mesures |
|:---|:---|:---|:---|
| **VULN-D01** | **Absence de limitation de débit (rate limiting)** | Un attaquant sature l'API avec des millions de requêtes automatisées. | • Limitation par IP ou par utilisateur<br>• Timeouts stricts |
| **VULN-D02** | **Absence de limites de ressources** | Un attaquant envoie un fichier géant ou une requête complexe (ReDoS) pour faire planter le serveur. | • Limiter la taille des uploads<br>• Pagination obligatoire<br>• Valider la complexité des expressions régulières |

---

## E - Elevation of Privilege (Menace : Élévation de privilèges)
*L'attaquant obtient des droits supérieurs à ceux prévus.*

| ID | Vulnérabilité (La faiblesse) | Scénario de menace (L'attaque) | Contre-mesures |
|:---|:---|:---|:---|
| **VULN-E01** | **Composants vulnérables (CVE)** | Un attaquant exploite une faille connue dans une librairie non mise à jour. | • Analyse SCA (Dependabot/Snyk)<br>• Application régulière des correctifs |
| **VULN-E02** | **Contrôle d'accès défaillant (IDOR)** | Un attaquant change l'ID dans l'URL (`/user/123`) pour accéder aux données d'un autre utilisateur. | • Vérification stricte de la propriété (`owner_id`) côté serveur |
| **VULN-E03** | **Absence d'assainissement des entrées (Injection)** | Un attaquant injecte du code (SQL/Script) via un formulaire pour exécuter des commandes administrateur. | • Requêtes préparées (SQL)<br>• Échappement contextuel (XSS) |