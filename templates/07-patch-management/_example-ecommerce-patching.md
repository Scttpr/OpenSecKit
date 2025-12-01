---
title: "Exemple : Patch management pour une plateforme e-commerce"
template_version: "1.0.0"
constitutional_principle: "VII - Gestion proactive des correctifs"
project: "ShopSecure - Plateforme e-commerce"
ssdlc_phase: "all"
tags: ["example", "ecommerce", "patch-management", "dependencies", "vulnerabilities"]
---

# Exemple : Patch management pour ShopSecure

**Projet** : ShopSecure - Plateforme e-commerce B2C
**Outils** : Détection automatique de vulnérabilités, scans de sécurité
**SLA** : CRITICAL 48h, HIGH 7j, MEDIUM 30j, LOW 90j

---

## 1. Registre des vulnérabilités

| CVE | Package | CVSS | EPSS | Expo | Data | Biz | Total | Priorité | Deadline | Statut |
|-----|---------|------|------|------|------|-----|-------|----------|----------|--------|
| CVE-2024-001 | express | 9.8 | 10 | 10 | 10 | 10 | **50** | P0 | 2024-12-02 | ✅ Corrigé |
| CVE-2024-002 | lodash | 7.4 | 7 | 10 | 7 | 7 | **38** | P1 | 2024-12-08 | ⏳ En cours |
| CVE-2024-003 | moment | 5.5 | 1 | 4 | 4 | 4 | **17** | P3 | 2025-01-15 | 📋 Backlog |

---

## 2. Cas réel : CVE-2024-001 (CRITICAL)

### Détection (Jour 0 - 08:00)

**Alerte de sécurité** :
```
🚨 CRITICAL vulnerability detected

Package: express
Version: 4.17.1
CVE: CVE-2024-001
CVSS: 9.8 (Critical)
Description: Remote Code Execution via HTTP request parsing
Exploit available: Yes (PoC published)
Affected: shopsecure-api production

Recommended action: Upgrade to express@4.18.5 immediately
```

---

### Évaluation (Jour 0 - 08:15)

**Calcul du score de priorité** :
```
CVSS (9.8)           : 10 points (Critical)
EPSS (92%)           : 10 points (Exploit public + attaques actives)
Exposition           : 10 points (API publique Internet)
Sensibilité données  : 10 points (Paiements + PII)
Criticité business   : 10 points (API principale)
─────────────────────────────────────────────────
TOTAL                : 50 points → P0 - URGENT
```

**Décision** : Activer procédure d'urgence

---

### Mobilisation (Jour 0 - 08:30)

**War Room créé** :
```bash
# Slack notification
@channel 🚨 ALERTE SÉCURITÉ CRITIQUE

CVE-2024-001 détecté dans express@4.17.1
Impact: RCE sur API production
SLA: 48h
War Room: https://meet.google.com/emergency-12-02

Équipe requise:
- Security Champion ✅
- DevOps Lead ✅
- Tech Lead ✅
- Product Manager ✅
```

---

### Mitigation temporaire (Jour 0 - 09:00)

**Actions** :
- **Règle WAF déployée** : Configuration pour bloquer les patterns d'exploitation connus
- **Validation** : Tests pour confirmer que l'exploit est bien bloqué
- **Résultat** : Tentatives d'exploitation bloquées avec code 403 ✅

---

### Tests (Jour 0 - 10:00)

**Actions** :
1. **Créer branche hotfix** : `hotfix/CVE-2024-001`
2. **Mettre à jour la dépendance** : express 4.17.1 → 4.18.5
3. **Vérifier la correction** : Scanner de sécurité confirme 0 vulnérabilité critique ✅
4. **Tests de non-régression** (suite réduite - 30min max) :
   - ✅ API endpoints: 127/127 passed
   - ✅ Authentication: 45/45 passed
   - ✅ Payments: 23/23 passed
5. **Commit et push** : Changements versionnés avec message explicite

---

### Déploiement staging (Jour 0 - 11:00)

**Actions** :
1. **Déployer en staging** : Version `hotfix-cve-2024-001`
2. **Smoke tests** :
   - Endpoint health : 200 OK ✅
   - API products : Fonctionnel ✅
3. **Vérifier vulnérabilité corrigée** :
   - Scan de vulnérabilité : NOT VULNERABLE ✅
4. **Performance test** :
   - Tests de charge : Pas de régression > 10% ✅

---

### Déploiement production (Jour 0 - 14:00)

**Actions** :
1. **Backup pré-déploiement** :
   - Configuration de l'application sauvegardée
   - Base de données sauvegardée
2. **Annoncer maintenance** :
   - Status page mise à jour : "Emergency security patch - No downtime expected"
3. **Déploiement Blue/Green** :
   - Déployer nouvelle version (green) en parallèle
   - Attendre que green soit prêt (5 min)
   - Smoke test green : 200 OK ✅
4. **Basculer le trafic** :
   - 100% du trafic vers green instantanément
5. **Monitoring renforcé** (30 min) :
   - CPU < 80% ✅
   - Memory < 80% ✅
   - Aucune erreur dans les logs ✅

---

### Vérification post-patch (Jour 0 - 15:00)

**Vérifications** :
1. **Service opérationnel** :
   - Endpoint health : 200 OK ✅
2. **Vulnérabilité corrigée** :
   - Scan de sécurité : NOT VULNERABLE ✅
3. **DAST scan rapide** :
   - Résultat : 0 HIGH/CRITICAL ✅
4. **Logs propres** :
   - Aucune erreur critique détectée ✅
5. **Métriques normales** :
   - Error rate: 0.2% (< 1% ✅)
   - Latency P95: 250ms (< 500ms ✅)
   - CPU: 45% (< 80% ✅)
   - Memory: 60% (< 80% ✅)

---

### Communication (Jour 0 - 16:00)

**Email interne** :
```
De: security@shopsecure.com
À: all@shopsecure.com
Objet: [RÉSOLU] Patch critique CVE-2024-001 appliqué

Bonjour,

Une vulnérabilité critique (CVE-2024-001, CVSS 9.8) a été détectée
dans la bibliothèque express.

✅ Actions prises:
- 08:00: Détection et évaluation
- 09:00: Mitigation WAF appliquée
- 14:00: Patch appliqué en production
- 15:00: Vérification complète OK

✅ Résultat:
- Downtime: 0 minute
- Vulnérabilité corrigée et vérifiée
- Aucune exploitation détectée
- Aucun impact utilisateur

Temps de résolution: 7 heures (SLA: 48h)

Détails: https://wiki.shopsecure.com/incidents/cve-2024-001

Merci à l'équipe pour la réactivité.

-- Security Team
```

---

## 3. Cas réel : CVE-2024-002 (HIGH)

### Vulnérabilité lodash (Priorité P1)

**Détails** :
```
Package: lodash
CVE: CVE-2024-002
CVSS: 7.4 (High)
Description: Prototype Pollution
EPSS: 65% (Exploit public disponible)
Path: express > body-parser > lodash@4.17.20
Fixed in: lodash@4.17.21
```

**Score priorité** : 38 points → **P1 - Haute** (SLA: 7 jours)

---

### Correction (Semaine en cours)

**Chronologie** :

**Jour 1** : Créer ticket
- Titre: [P1] Patch CVE-2024-002 (lodash prototype pollution)
- Assigné: DevOps team
- Deadline: 2024-12-08

**Jour 2** : Tentative de mise à jour
- Tentative de mise à jour simple : ❌ Échec (dépendance transitive)
- Solution alternative : Forcer la version corrigée via configuration
- Vérification : ✅ CVE-2024-002 corrigée

**Jours 3-4** : Tests complets
- Suite de tests complète : ✅ Tous les tests passent

**Jour 5** : Déploiement staging

**Jour 6** : Déploiement production (fenêtre de maintenance planifiée)

**Jour 7** : Vérification post-patch complète

---

## 4. Automatisation de la détection

### Configuration des scans automatiques

**Paramètres** :
- **Fréquence** : Quotidienne (03:00)
- **Scope** : Dépendances applicatives et infrastructure
- **Seuil d'alerte** : Toutes les criticités
- **Revue** : Équipe sécurité notifiée
- **Labels** : `dependencies`, `security`

**Actions automatiques** :
- Détection quotidienne des nouvelles vulnérabilités
- Création automatique de tickets pour les vulnérabilités détectées
- Groupement des mises à jour mineures et patches
- Auto-merge des patches non-critiques après validation des tests

**Bénéfices** :
- Réduction du délai de détection (< 24h)
- Traitement systématique des mises à jour
- Moins de charge manuelle sur l'équipe

---

## 5. Métriques de patch management

### Dashboard (Mois de Novembre 2024)

| Métrique | Cible | Actuel | Statut |
|----------|-------|--------|--------|
| **Vulnérabilités CRITICAL** | 0 | 0 | ✅ |
| **Vulnérabilités HIGH** | < 5 | 2 | ✅ |
| **MTTR (Critical)** | < 48h | 7h | ✅ |
| **MTTR (High)** | < 7j | 4j | ✅ |
| **Taux respect SLA** | > 95% | 100% | ✅ |
| **Dépendances à jour** | > 80% | 92% | ✅ |

---

### Rapport mensuel

```markdown
# Patch Management Report - Novembre 2024

## Résumé
- Vulnérabilités détectées: 15
- Vulnérabilités corrigées: 15
- Taux de respect SLA: 100%
- MTTR moyen: 2.5 jours

## Détail par criticité
| Criticité | Détectées | Corrigées | Moyenne correction |
|-----------|-----------|-----------|-------------------|
| CRITICAL  | 1         | 1         | 7h                |
| HIGH      | 5         | 5         | 4j                |
| MEDIUM    | 6         | 6         | 12j               |
| LOW       | 3         | 3         | 25j               |

## Incidents notables
1. CVE-2024-001 (express RCE) - Procédure d'urgence activée, corrigé en 7h
2. CVE-2024-002 (lodash) - Override npm utilisé avec succès

## Actions d'amélioration
1. ✅ Automatisation Dependabot configurée
2. ✅ WAF rules templates créés pour réponse rapide
3. 📋 Formation équipe sur procédure d'urgence (prévu Dec)
```

---

**Conclusion** : Tous les principes constitutionnels sont maintenant implémentés avec succès sur ShopSecure.
