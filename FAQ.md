# Foire aux questions (FAQ)

Réponses aux questions les plus fréquentes sur OpenSecKit.

---

## 🎯 Démarrage

### Par où commencer si je n'ai jamais fait de sécurité applicative ?

**Réponse courte** : Suivez le [QUICK-START.md](QUICK-START.md) (10 minutes).

**Réponse détaillée** :
1. Lisez le préambule de la [constitution.md](constitution.md) (5 min)
2. Suivez le guide quick start avec votre projet (15 min)
3. Appliquez les 3 actions immédiates (sécuriser DB, installer gitleaks, planifier MFA)
4. Revenez aux autres templates progressivement

**Vous n'avez pas besoin d'être expert en sécurité.** Les templates sont conçus pour guider les débutants.

---

### Dois-je remplir TOUS les templates ?

**Non.** Adaptez à votre contexte :

**Minimum vital** :
- ✅ Modélisation des menaces (template STRIDE)
- ✅ Analyse de risques (scoring)
- ✅ Détection de secrets (gitleaks)

**Recommandé** :
- ✅ Exigences de sécurité (OWASP ASVS checklist)
- ✅ SAST dans CI/CD

**Si applicable** :
- Domaines spécifiques (RGPD si données UE, RGS si secteur public français)
- Templates avancés (fuzzing, threat modeling continu)

**Règle** : Commencez petit, itérez. Mieux vaut 3 templates bien remplis que 15 à moitié.

---

### Combien de temps faut-il pour sécuriser un projet avec OpenSecKit ?

**Version rapide** (quick start) : 10-15 minutes
- Top 3 des menaces identifiées
- 1 vulnérabilité critique corrigée
- Détection de secrets installée

**Version complète** (conformité totale aux 7 principes) : 1-2 jours
- Modélisation menaces complète
- Tous les risques scorés
- SAST/DAST configurés
- Gestionnaire de secrets intégré
- Logging d'audit implémenté
- Pipeline de patch management automatisé

**Amortissement** : Investissement initial important, mais gain massif à long terme (moins de vulnérabilités, audits simplifiés, conformité).

---

### OpenSecKit fonctionne avec quel langage/framework ?

**Tous.** OpenSecKit est **agnostique du langage**.

Les templates sont des documents Markdown à remplir, pas du code. Ils s'appliquent à :
- Node.js, Python, Java, Go, Ruby, PHP, .NET, etc.
- React, Vue, Angular, Django, Rails, Spring, Express, etc.
- Applications web, APIs, microservices, serverless, mobile backends

**Exemples de code** dans les templates couvrent les langages populaires (Node.js, Python, Go).

---

## 📋 Utilisation des templates

### Où stocker les fichiers remplis ?

**Recommandation** : Dans votre projet, sous `docs/security/` :

```
mon-projet/
├── docs/
│   └── security/
│       ├── threat-model.md
│       ├── risk-analysis.md
│       ├── security-requirements.md
│       └── audit-logs-spec.md
├── src/
└── tests/
```

**Alternative** : Wiki du projet, Confluence, Notion (mais moins bien versionné).

**Important** : Versionnez ces documents dans git (sauf s'ils contiennent des infos sensibles).

---

### Que faire après avoir rempli un template ?

1. **Valider** : Faire relire par un collègue ou security champion
2. **Implémenter** : Transformer les contre-mesures identifiées en code/config
3. **Tracker** : Créer des tickets pour chaque contre-mesure à implémenter
4. **Auditer** : Vérifier périodiquement que le template est à jour

**Exemple** : Après `threat-model.md` rempli →
- Créer ticket "Implémenter MFA" (contre-mesure T1)
- Créer ticket "Ajouter requêtes préparées" (contre-mesure T2)
- Planifier revue du modèle dans 3 mois

---

### Comment savoir si mon template est bien rempli ?

**Checklist générale** :

- [ ] Toutes les sections obligatoires remplies (marquées "Required")
- [ ] Pas de `[À compléter]` ou `[TBD]` restants
- [ ] Exemples adaptés à mon contexte (pas de copier-coller générique)
- [ ] Revu par au moins 1 autre personne
- [ ] Contre-mesures identifiées sont actionnables (pas vagues)

**Checklist spécifique par template** :

**Modélisation des menaces** :
- [ ] Au moins 3 assets identifiés
- [ ] Au moins 5 menaces documentées (couvrant STRIDE)
- [ ] Chaque menace a une contre-mesure concrète

**Analyse de risques** :
- [ ] Chaque risque a un score calculé (Criticité × Probabilité × Exposition)
- [ ] Risques > 15/25 validés par la direction
- [ ] Plan d'action pour top 5 des risques

---

### Puis-je modifier les templates pour mon projet ?

**Oui, absolument.** Les templates sont des points de départ, pas des règles rigides.

**Adaptations courantes** :
- Ajouter des colonnes spécifiques à votre contexte
- Supprimer des sections non applicables
- Ajouter des références à vos outils internes
- Traduire en anglais

**Mais gardez** :
- La structure de base (facilite les revues)
- Les sections obligatoires
- Le lien avec le principe constitutionnel

**Astuce** : Créez une copie locale du template avant de l'adapter :
```bash
cp templates/01-threat-modeling/stride.md mon-projet/templates/stride-custom.md
# Modifier stride-custom.md
```

---

## 🔍 Validation et conformité

### Comment valider que je respecte la constitution ?

**Validation manuelle** (recommandée pour débuter) :

Utilisez la checklist de validation constitutionnelle (voir [constitution.md](constitution.md#validation-constitutionnelle)) :

- [ ] **Principe I** : Fichier `threat-model.md` existe et complet
- [ ] **Principe II** : Risques scorés, top risques validés
- [ ] **Principe III** : Exigences de sécurité documentées (OWASP ASVS)
- [ ] **Principe IV** : Pipeline CI/CD inclut SAST/DAST/SCA
- [ ] **Principe V** : 0 secret dans le code (gitleaks passe)
- [ ] **Principe VI** : Logs de sécurité implémentés
- [ ] **Principe VII** : Dependabot/Snyk configuré avec SLA

**Validation automatisée** (avancé) :

Créer un script qui vérifie :
```bash
#!/bin/bash
# constitution-check.sh

# Principe I : Modélisation des menaces
[ -f docs/security/threat-model.md ] || exit 1

# Principe V : Détection de secrets
gitleaks detect --source . --no-git || exit 1

# Principe VII : Scan de dépendances
npm audit --audit-level=high || exit 1
```

---

### Quelle est la différence entre OpenSecKit et un audit de sécurité ?

**OpenSecKit** :
- **Préventif** : Appliqué **pendant** le développement
- **Auto-service** : Équipe de développement autonome
- **Continu** : Templates mis à jour régulièrement
- **Gratuit** : Open source

**Audit de sécurité** :
- **Réactif** : Effectué **après** le développement
- **Externe** : Auditeur indépendant
- **Ponctuel** : Une fois par an généralement
- **Coûteux** : Plusieurs milliers d'euros

**Complémentarité** : OpenSecKit prépare votre code pour l'audit. Un projet suivant OpenSecKit aura beaucoup moins de findings lors de l'audit externe.

---

## 🛠️ Outils et intégration

### Dois-je acheter des outils payants ?

**Non, pour commencer.** OpenSecKit fonctionne 100% avec des outils open source gratuits :

**Gratuit et suffisant** :
- **Modélisation menaces** : Microsoft Threat Modeling Tool (gratuit), OWASP Threat Dragon
- **SAST** : Semgrep (gratuit), SonarQube Community
- **Détection secrets** : gitleaks (gratuit), detect-secrets
- **SCA** : Dependabot (gratuit GitHub), npm audit (gratuit)

**Payant (optionnel, pour scale)** :
- **SAST avancé** : Snyk, Checkmarx
- **DAST** : Burp Suite Pro
- **Secrets management** : HashiCorp Vault Enterprise, AWS Secrets Manager

**Règle** : Commencez gratuit, payez quand vous scalez (>50 développeurs, compliance stricte).

---

### Comment intégrer OpenSecKit dans mon workflow existant ?

**Agile/Scrum** :

- **Sprint planning** : Ajouter une story "Security review" pour chaque feature
- **Definition of Done** : Inclure "Constitution check passed"
- **Sprint retrospective** : Revoir les incidents de sécurité

**GitFlow** :

- **Pre-merge** : Hook gitleaks bloque les secrets
- **Pull request** : Template inclut "Security checklist"
- **Avant release** : Validation constitutionnelle

**CI/CD** :

Ajouter des stages sécurité :
```yaml
stages:
  - build
  - test
  - security  # ← Nouveau stage
  - deploy

security:
  script:
    - gitleaks detect
    - npm audit
    - semgrep --config auto
```

---

## 🌍 Domaines spécifiques

### Dois-je utiliser les templates des domaines (RGPD, NIS2, RGS) ?

**Utilisez si** :
- Vous traitez des données personnelles d'utilisateurs UE → RGPD
- Vous êtes un opérateur d'infrastructures critiques UE → NIS2
- Vous développez pour le secteur public français → RGS

**Sinon** : Les templates de base (`templates/`) suffisent.

**Comment utiliser** :
1. Commencez par les templates de base
2. Complétez avec les exigences du domaine spécifique
3. Exemple : `threat-model.md` (base) + `rgpd/dpia-template.md` (RGPD)

---

## 🚨 Incidents et urgences

### J'ai accidentellement commité un secret, que faire ?

**Action immédiate** (dans l'heure) :

1. **Révoquer le secret** côté provider :
   - Clé API GitHub → Régénérer dans Settings
   - Token Stripe → Révoquer dans Dashboard
   - Password → Changer immédiatement

2. **Supprimer de l'historique git** :
   ```bash
   # Utiliser BFG Repo-Cleaner
   java -jar bfg.jar --delete-files .env
   git reflog expire --expire=now --all
   git gc --prune=now --aggressive
   git push --force
   ```

3. **Alerter l'équipe** : Tous doivent re-cloner le repo

4. **Investiguer** : Vérifier les logs d'accès au secret (a-t-il été utilisé ?)

**Prévention future** :
- Installer [secrets-detection-setup.md](templates/05-secrets-management/secrets-detection-setup.md)
- Configurer gitleaks en pre-commit hook

**Détails** : Voir template `templates/05-secrets-management/secrets-detection-setup.md`

---

### Une vulnérabilité critique a été trouvée dans une dépendance, comment réagir ?

**SLA selon la constitution** : 48h pour correction d'une vulnérabilité critique.

**Processus** :

1. **Vérifier l'impact** (30 min) :
   ```bash
   npm audit  # Voir le CVSS score
   ```
   Si CVSS ≥ 9.0 → Critique, action immédiate

2. **Tester la mise à jour** (1h) :
   ```bash
   npm update <package>
   npm test  # Vérifier non-régression
   ```

3. **Déployer** (30 min) :
   - En staging d'abord
   - Puis production (procédure d'urgence si nécessaire)

4. **Documenter** :
   - Mettre à jour `docs/security/patch-log.md`
   - Informer la direction si impact client

**Si pas de patch disponible** : Workaround (désactiver la fonctionnalité, firewall rules) en attendant.

---

## 🤝 Contribution et communauté

### Puis-je contribuer mes propres templates ?

**Oui !** OpenSecKit est open source (licence MIT).

**Comment contribuer** :

1. Forker le repo
2. Créer un nouveau template dans le bon dossier
3. Suivre la structure existante (frontmatter YAML + sections)
4. Ajouter un exemple (`_example-*.md`)
5. Mettre à jour le README du dossier
6. Soumettre une Pull Request

**Templates les plus demandés** :
- Fuzzing guide (principe IV)
- Incident response plan
- Security code review checklist
- Cloud security (AWS/Azure/GCP specific)

**Voir** : [.github/CONTRIBUTING.md](.github/CONTRIBUTING.md)

---

### Où poser des questions non couvertes par la FAQ ?

**GitHub Discussions** : [https://github.com/Scttpr/OpenSecKit/discussions](https://github.com/Scttpr/OpenSecKit/discussions)

**Channels** :
- `#general` : Questions générales
- `#templates` : Questions sur un template spécifique
- `#show-and-tell` : Partager votre utilisation d'OpenSecKit

**Temps de réponse** : Généralement < 24h (communauté + mainteneurs)

---

## 📚 Ressources additionnelles

### Où apprendre les bases de la sécurité applicative ?

**Gratuit** :
- [OWASP Top 10](https://owasp.org/www-project-top-ten/) - Les 10 vulnérabilités les plus critiques
- [PortSwigger Web Security Academy](https://portswigger.net/web-security) - Labs pratiques gratuits
- [OWASP Cheat Sheet Series](https://cheatsheetseries.owasp.org/) - Guides pratiques par sujet

**Payant (mais excellent)** :
- [Security Journey](https://www.securityjourney.com/) - Plateforme de formation
- [PentesterLab](https://pentesterlab.com/) - Exercices pratiques

**Livre recommandé** :
- "The Web Application Hacker's Handbook" (Stuttard & Pinto) - Bible de la sécurité web

---

**Cette FAQ ne répond pas à votre question ?**

👉 [Ouvrir une Discussion GitHub](https://github.com/Scttpr/OpenSecKit/discussions/new?category=q-a)

*FAQ mise à jour le 2025-11-29*
