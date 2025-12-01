# Constitution - Principes de sécurité logicielle

---

## Préambule

Cette constitution définit les principes non négociables pour le développement sécurisé de logiciels. Elle s'adresse à toutes les parties prenantes d'un projet logiciel : direction, équipes de développement, opérations, produit, auditeurs de sécurité et conformité.

**Objectif** : Intégrer la sécurité dès la conception et tout au long du cycle de vie du développement logiciel.

**Pourquoi une constitution ?** Parce que la sécurité n'est pas optionnelle. Ces sept principes forment le socle minimum pour développer des logiciels résistants aux menaces actuelles.

---

## Les sept principes fondamentaux

### I. Modélisation des menaces 

**Principe** : Toute nouvelle fonctionnalité doit faire l'objet d'une modélisation des menaces **avant** le début du développement.

**Pourquoi ?** La modélisation proactive des menaces réduit de 70% les vulnérabilités découvertes en production. Elle permet d'anticiper les risques avant qu'ils ne deviennent des failles exploitables.

**Comment l'appliquer :**

1. **Identifier les actifs critiques** : données sensibles, secrets, flux métier
2. **Documenter les vecteurs d'attaque** : utiliser STRIDE, Kill Chain ou MITRE ATT&CK
3. **Évaluer les impacts** : confidentialité, intégrité, disponibilité
4. **Définir les contre-mesures** : contrôles de sécurité requis
5. **Documenter** : créer un document de modélisation des menaces pour chaque fonctionnalité majeure

**Méthodologies recommandées :**
- **STRIDE** : Spoofing, Tampering, Repudiation, Information disclosure, Denial of service, Elevation of privilege
- **Arbres d'attaque** : visualisation hiérarchique des chemins d'attaque
- **Diagrammes de flux de données (DFD)** : cartographie des flux d'information

**Qui est concerné :**
- **Direction** : approuve les risques résiduels acceptables
- **Développeurs** : implémentent les contre-mesures identifiées
- **Auditeurs** : valident la couverture des menaces

---

### II. Analyse de risques continue

**Principe** : Chaque modification de code doit être évaluée pour son impact sur la posture de sécurité globale.

**Pourquoi ?** L'analyse de risques transforme l'intuition en données mesurables, permettant des décisions éclairées sur les arbitrages sécurité/fonctionnalité.

**Comment l'appliquer :**

1. **Scorer chaque risque** : Criticité × Probabilité × Exposition (échelle 1-5, score max 25)
2. **Maintenir un registre des risques** : document vivant recensant tous les risques identifiés
3. **Définir des seuils de validation** :
   - Risques critiques (score > 15/25) → validation Direction
   - Risques élevés (score 10-15/25) → validation équipe sécurité
   - Risques moyens/faibles → documentation et suivi
4. **Réévaluer périodiquement** : le paysage de menaces évolue, les risques aussi

**Matrice de scoring :**
```
Score de risque = Criticité × Probabilité × Exposition

Criticité (impact) :
1 = Mineur, 2 = Faible, 3 = Moyen, 4 = Élevé, 5 = Critique

Probabilité (fréquence d'exploitation) :
1 = Très improbable, 2 = Improbable, 3 = Possible, 4 = Probable, 5 = Très probable

Exposition (surface d'attaque) :
1 = Très limitée, 2 = Limitée, 3 = Moyenne, 4 = Étendue, 5 = Très étendue
```

**Qui est concerné :**
- **Direction** : prend les décisions d'acceptation de risque
- **Développeurs** : comprennent le contexte de risque de chaque modification
- **Ops** : priorisent les patchs selon les scores de risque
- **Conformité** : tracent les décisions pour les audits réglementaires

---

### III. Sécurité dès la conception

**Principe** : Les contrôles de sécurité sont des exigences fonctionnelles de première classe, au même titre que la logique métier.

**Pourquoi ?** Corriger une vulnérabilité en production coûte 30× plus cher qu'en phase de conception (NIST). La sécurité ajoutée après coup est toujours moins efficace et plus coûteuse.

**Comment l'appliquer :**

**Principes de conception sécurisée :**
- **Moindre privilège** : chaque composant/utilisateur n'a que les droits strictement nécessaires
- **Défense en profondeur** : plusieurs couches de contrôles (ne pas compter sur un seul mécanisme)
- **Échec sécurisé (fail secure)** : en cas d'erreur, refuser l'action par défaut
- **Validation systématique** : ne jamais faire confiance aux entrées (utilisateur, API, fichiers)
- **Chiffrement par défaut** : données sensibles chiffrées au repos et en transit
- **Séparation des environnements** : dev/staging/prod isolés avec cloisonnement réseau

**Checklist de conception :**
- [ ] Authentification robuste (multi-facteur pour les accès critiques)
- [ ] Autorisation granulaire (contrôle d'accès basé sur les rôles - RBAC)
- [ ] Validation et échappement de toutes les entrées
- [ ] Chiffrement des données sensibles (AES-256, TLS 1.3)
- [ ] Gestion sécurisée des sessions (tokens signés, expiration, rotation)
- [ ] Protection contre les attaques OWASP Top 10

**Qui est concerné :**
- **Développeurs** : implémentent les contrôles
- **Architectes** : conçoivent l'architecture sécurisée
- **Équipes produit** : expriment les exigences de sécurité en langage métier
- **Ops** : configurent l'infrastructure de manière sécurisée

---

### IV. Tests de sécurité

**Principe** : Les tests de sécurité sont non négociables et bloquants pour le déploiement.

**Pourquoi ?** L'automatisation des tests de sécurité détecte 85% des vulnérabilités communes (OWASP) sans intervention humaine.

**Comment l'appliquer :**

**Types de tests :**

1. **SAST (Static Application Security Testing)** :
   - Analyse du code source avant chaque merge
   - Intégration CI/CD (build échoue si vulnérabilité critique)
   - Outils : SonarQube, Semgrep, CodeQL

2. **DAST (Dynamic Application Security Testing)** :
   - Scan des endpoints/interfaces en environnement staging
   - Détection des vulnérabilités runtime (injection, XSS, CSRF)
   - Outils : OWASP ZAP, Burp Suite

3. **SCA (Software Composition Analysis)** :
   - Détection de vulnérabilités dans les dépendances tierces
   - Scan quotidien automatique
   - Outils : Dependabot, Snyk, OWASP Dependency-Check

4. **Tests de non-régression sécurité** :
   - Suite de tests validant que les failles connues ne réapparaissent pas
   - Exécutés à chaque build

**Seuils de qualité :**
- **Couverture minimum** : 80% des flux critiques identifiés dans le modèle de menaces
- **Seuil de blocage** : aucune vulnérabilité critique ou haute non résolue autorisée en production

**Qui est concerné :**
- **Développeurs** : écrivent et corrigent les tests
- **Ops** : intègrent les tests dans les pipelines CI/CD
- **Auditeurs** : valident la couverture des tests

---

### V. Gestion rigoureuse des secrets

**Principe** : Aucun secret (clé API, mot de passe, certificat, token) ne doit être commité dans le code source ou les fichiers de configuration versionnés.

**Pourquoi ?** 73% des fuites de données impliquent des secrets exposés (GitHub Security Report 2024). La prévention technique élimine le risque humain.

**Comment l'appliquer :**

**Règles absolues :**
- ❌ **Jamais** de secrets en clair dans le code
- ❌ **Jamais** de secrets dans les fichiers de configuration versionnés (`.env`, `config.yml`)
- ❌ **Jamais** de secrets dans les variables d'environnement publiques
- ✅ **Toujours** utiliser un gestionnaire de secrets dédié

**Infrastructure requise :**
- **Gestionnaire de secrets** : HashiCorp Vault, Azure Key Vault, AWS Secrets Manager, ou équivalent
- **Rotation automatique** : secrets critiques renouvelés automatiquement (max 90 jours)
- **Scan pre-commit** : détection automatique des secrets avant commit (gitleaks, trufflehog)
- **Audit d'accès** : logs traçant qui a accédé à quels secrets et quand
- **Révocation rapide** : processus documenté pour révoquer un secret compromis

**Qui est concerné :**
- **Développeurs** : utilisent le gestionnaire de secrets dans leur code
- **Ops** : gèrent les rotations et les accès
- **Auditeurs** : vérifient les logs d'accès
- **Conformité** : prouvent l'absence d'exposition de secrets

---

### VI. Traçabilité et auditabilité complète

**Principe** : Toutes les décisions de sécurité, tous les accès aux données sensibles et toutes les modifications de configuration doivent être tracées et horodatées de manière immuable.

**Pourquoi ?** La détection rapide d'une compromission réduit le coût moyen de 40% (IBM Cost of Data Breach Report). La traçabilité est aussi une exigence réglementaire (ISO 27001, SOC 2, RGPD).

**Comment l'appliquer :**

**Infrastructure de logging :**
- **Format structuré** : logs en JSON avec corrélation entre événements (trace IDs)
- **Stockage sécurisé** : write-only, WORM (Write Once Read Many) ou SIEM centralisé
- **Rétention conforme** : minimum 1 an pour les accès (RGPD), ajuster selon réglementations

**Événements critiques à tracer :**
- ✅ Authentification et autorisation (succès **et** échecs)
- ✅ Modifications de données sensibles (qui, quoi, quand, depuis où)
- ✅ Changements de configuration sécurité
- ✅ Déploiements en production
- ✅ Accès aux secrets
- ✅ Actions administratives

**Alerting en temps réel :**
- Tentatives d'authentification échouées répétées
- Accès hors horaires habituels
- Modifications massives de données
- Élévation de privilèges inhabituelle

**Format de log recommandé :**
```json
{
  "timestamp": "2025-11-29T14:32:01.123Z",
  "trace_id": "abc-123-def",
  "event_type": "authentication_failure",
  "user_id": "user@example.com",
  "ip_address": "192.168.1.100",
  "resource": "/api/admin/users",
  "severity": "warning",
  "metadata": {
    "attempt_count": 3,
    "reason": "invalid_password"
  }
}
```

**Qui est concerné :**
- **Développeurs** : implémentent les logs dans le code
- **Ops** : gèrent le SIEM et les alertes
- **Auditeurs** : utilisent les logs pour les investigations
- **Conformité** : fournissent les preuves réglementaires

---

### VII. Mise à jour et patch management proactif

**Principe** : Les vulnérabilités connues des dépendances et des systèmes doivent être corrigées dans des délais stricts.

**Pourquoi ?** 60% des intrusions exploitent des vulnérabilités connues pour lesquelles un patch existe depuis plus d'un an (Verizon DBIR). La réactivité est la meilleure défense.

**Comment l'appliquer :**

**SLA de correction non négociables :**
- **Vulnérabilités critiques (CVSS 9.0-10.0)** : correction sous **48h maximum**
- **Vulnérabilités hautes (CVSS 7.0-8.9)** : correction sous **7 jours maximum**
- **Vulnérabilités moyennes (CVSS 4.0-6.9)** : correction sous **30 jours maximum**
- **Vulnérabilités faibles (CVSS 0.1-3.9)** : correction planifiée dans le backlog

**Processus :**
1. **Scan quotidien** : automatique des dépendances (npm audit, pip-audit, Dependabot)
2. **Priorisation** : selon CVSS et exploitabilité dans le contexte
3. **Test** : non-régression avant déploiement du patch
4. **Déploiement** : selon SLA et procédure d'urgence si nécessaire
5. **Communication** : transparente aux parties prenantes

**Processus d'urgence pour 0-days :**
- Équipe dédiée mobilisable H24
- Procédure de patch documentée et testée régulièrement
- Communication de crise préparée

**Qui est concerné :**
- **Développeurs** : appliquent les patchs
- **Ops** : déploient en urgence si nécessaire
- **Direction** : alloue les ressources pour les patches urgents
- **Conformité** : trace le respect des SLA

---

## Gouvernance de la sécurité

### Rôles et responsabilités

**Direction :**
- Valide les risques résiduels de niveau critique
- Alloue le budget pour la sécurité (outils, formation, audits)
- Arbitre en cas de conflit sécurité/délai

**Security champions :**
- Au moins 1 par équipe, formé à la modélisation des menaces et OWASP Top 10
- Effectue les revues de code avec focus sécurité
- Évangélise les bonnes pratiques au sein de l'équipe

**Équipes opérations :**
- Durcissent les infrastructures (OS, réseau, containers)
- Assurent le monitoring continu et la réponse aux incidents
- Gèrent les secrets et certificats

**Équipes produit :**
- Expriment les exigences de sécurité en langage métier
- Priorisent les correctifs de sécurité dans la roadmap

**Auditeurs et conformité :**
- Effectuent des audits trimestriels
- Valident la traçabilité
- Accompagnent sur les exigences réglementaires

### Indicateurs clés de performance (KPI)

- **MTTD (Mean Time To Detect)** : < 24h pour détecter une intrusion
- **MTTR (Mean Time To Resolve)** : < 48h pour résoudre une vulnérabilité critique
- **Couverture des menaces** : 100% des menaces identifiées ont des contre-mesures
- **Conformité des commits** : 100% des commits passent le scan pre-commit
- **Maturité SSDLC** : évaluation annuelle (BSIMM, SAMM) avec progression visible

---

## Cycle de vie sécurisé

### Intégration dans le processus de développement

**1. Spécification**
- Capturer les exigences de sécurité
- Documenter les cas d'usage malveillants (abuse cases)

**2. Planification**
- Modélisation des menaces (principe I)
- Analyse de risques (principe II)
- Validation constitutionnelle

**3. Conception**
- Architecture sécurisée avec diagrammes de flux de données
- Identification des points de contrôle (firewalls, WAF, validation, chiffrement)
- Revue de design avec security champion

**4. Implémentation**
- Code review avec checklist OWASP
- SAST intégré en CI/CD (build bloqué si vulnérabilité critique)
- Pre-commit hooks (détection secrets + linting sécurité)

**5. Tests**
- Tests de sécurité unitaires (injection, XSS, CSRF, etc.)
- DAST sur environnement staging
- Fuzzing pour endpoints critiques
- Validation des contrôles d'accès

**6. Déploiement**
- Validation finale : aucune vulnérabilité critique/haute non résolue
- Déploiement progressif avec possibilité de rollback
- Monitoring renforcé post-déploiement (24h de surveillance active)

**7. Maintenance**
- Scan quotidien des dépendances
- Application des patchs selon SLA
- Revue trimestrielle du modèle de menaces

### Validation constitutionnelle (porte de qualité)

Avant chaque mise en production, vérifier que :

- ✅ **Modélisation des menaces** : document complet (actifs, menaces, contre-mesures)
- ✅ **Analyse de risques** : scoring effectué, risques > 10/25 validés par les parties prenantes
- ✅ **Sécurité dès la conception** : contrôles de sécurité documentés et implémentés
- ✅ **Tests de sécurité** : SAST/DAST/SCA exécutés avec succès
- ✅ **Gestion des secrets** : aucun secret dans le code, gestionnaire configuré
- ✅ **Traçabilité** : logs de sécurité implémentés pour les opérations sensibles
- ✅ **Dépendances** : scan SCA effectué, pas de vulnérabilités critiques

**Responsables** : Security champion + chef de projet

**En cas d'échec** : blocage jusqu'à résolution des non-conformités

---

## Gouvernance de la constitution

### Amendements

1. **Proposition** : tout membre de l'équipe peut proposer un amendement
2. **Discussion** : période de commentaires de 14 jours minimum
3. **Approbation** : requiert validation unanime des security champions + direction + conformité
4. **Documentation** : mise à jour avec incrément de version (semantic versioning)
5. **Communication** : annonce à toutes les parties prenantes + formation si nécessaire
6. **Migration** : plan de migration des projets existants si applicable

### Versioning

- **MAJOR (X.0.0)** : suppression ou redéfinition incompatible d'un principe
- **MINOR (X.Y.0)** : ajout d'un nouveau principe ou extension significative
- **PATCH (X.Y.Z)** : clarifications, corrections, exemples

### Revue de conformité

- **Fréquence** : audit trimestriel
- **Périmètre** : échantillon représentatif (minimum 20% du code produit)
- **Métriques** : taux de conformité aux 7 principes + KPI sécurité
- **Non-conformités** : escalade direction + plan de remédiation
- **Amélioration continue** : résultats partagés en rétrospective + roadmap

---

## Ressources et références

### Frameworks de référence

- **OWASP SAMM** : auto-évaluation de maturité SSDLC
- **NIST SSDF** : pratiques recommandées par le NIST
- **MITRE ATT&CK** : catalogue de techniques d'attaque
- **STRIDE** : méthodologie de modélisation des menaces (Microsoft)

### Standards et normes

- **ISO 27001** : management de la sécurité de l'information
- **SOC 2 Type II** : contrôles de sécurité opérationnels
- **RGPD** : protection des données personnelles (UE)
- **PCI-DSS** : sécurité des données de cartes de paiement

### Outils recommandés

**Modélisation des menaces :**
- OWASP Threat Dragon
- Microsoft Threat Modeling Tool
- pytm

**Tests de sécurité :**
- **SAST** : SonarQube, Semgrep, CodeQL
- **DAST** : OWASP ZAP, Burp Suite
- **SCA** : Dependabot, Snyk, OWASP Dependency-Check

**Gestion des secrets :**
- HashiCorp Vault
- Azure Key Vault
- AWS Secrets Manager

**Détection de secrets :**
- gitleaks
- trufflehog
- detect-secrets

**Logging et monitoring :**
- ELK Stack
- Splunk
- Datadog Security Monitoring

### Formation recommandée

Formation annuelle pour tous les développeurs :
- OWASP Top 10 (8h)
- Atelier pratique de modélisation des menaces (4h)
- Certification security champion (recommandée)

---

## Conclusion

**Cette constitution forme le contrat social de sécurité de votre équipe.**

La sécurité n'est pas une fonctionnalité qu'on ajoute à la fin. C'est une propriété fondamentale du logiciel, au même titre que sa correction fonctionnelle. Ces sept principes établissent le minimum acceptable pour tout développement logiciel professionnel.

**Adopter cette constitution, c'est :**
- Protéger vos utilisateurs
- Protéger votre organisation
- Réduire drastiquement vos coûts de remédiation
- Démontrer votre professionnalisme
- Respecter vos obligations réglementaires

**La violation de ces principes n'est pas une option.**