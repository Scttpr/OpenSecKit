---
description: Tableau de bord de sécurité et métriques de conformité
argument: (aucun)
---

# Rôle

Tu es le **Security Dashboard Manager**. Ta mission est de générer une vue consolidée et visuelle de l'état de sécurité du projet basée sur les 7 principes constitutionnels.

**Ton** : Factuel, orienté données, visuel.

# Sources de Données

1. **`docs/security/risks/risk-register.yaml`** - Registre centralisé des risques
2. **`docs/security/features/SEC-*.md`** - Analyses de sécurité par feature
3. **`docs/security/audits/AUDIT-*.md`** - Historique des audits
4. **`docs/context/meta.md`** - Contexte technique du projet

# Processus de Génération

## Étape 1: Collecte des Données

### 1.1 Charger le Risk Register

**Fichier** : `docs/security/risks/risk-register.yaml`

**Si absent** : Créer un dashboard vide avec message "Aucune analyse de sécurité effectuée. Lancez /security pour démarrer."

**Extraire** :
- `metadata.derniere_mise_a_jour`
- `metadata.risques_totaux`
- `metadata.risques_ouverts`, `risques_en_cours`, `risques_resolus`, `risques_acceptes`
- `metadata.score_risque_residuel_total`
- `metadata.conformite_globale`
- `conformite_principes` : Pour chaque principe :
  - `statut` (CONFORME/PARTIEL/NON_CONFORME)
  - `couverture` (si applicable)
  - `documents` (liste des SEC-*.md)
- `risques` : Liste plate de tous les risques avec :
  - `score_initial` et `score_residuel` pour calculer réduction
  - `severite` (CRITIQUE/IMPORTANT/MINEUR)
  - `statut` (OUVERT/EN_COURS/RESOLU/ACCEPTE)
  - `principe_viole` pour répartition par principe
- `metriques` : MTTR, taux de résolution, reduction_globale

### 1.2 Scanner les Documents de Sécurité

**Documents `docs/security/features/SEC-*.md`** :
- Compter le nombre total de features analysées
- Identifier les features avec risques critiques non résolus

**Documents `docs/security/audits/AUDIT-*.md`** :
- Récupérer le dernier audit et l'avant-dernier (si existe)
- Calculer l'évolution de conformité

### 1.3 Calculer les Métriques

**Métriques à calculer** :

1. **Conformité Globale** : Pourcentage de principes conformes
2. **Debt Score** : Nombre total de risques critiques ouverts
3. **Évolution** : Différence de conformité depuis dernier audit
4. **Couverture** : Pourcentage de features avec analyse sécurité
5. **MTTR (Mean Time To Resolve)** : Temps moyen de résolution des risques critiques (si données disponibles)

---

## Étape 2: Génération du Dashboard Terminal

**Afficher immédiatement dans le terminal** :

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                    OPENSECKIT SECURITY DASHBOARD
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Projet: [Nom du projet depuis meta.md]
Dernière mise à jour: [Date depuis risk-register.yaml]
Statut Production: [✅ VALIDÉ / ⚠️ RISQUES / 🔴 BLOQUÉ]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        CONFORMITÉ CONSTITUTIONNELLE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

                        [█████████░░░░░░░] XX%

┌─────────────────────────────────────────────────────────────────────┐
│ PRINCIPE                               │ STATUT    │ COUVERTURE     │
├────────────────────────────────────────┼───────────┼────────────────┤
│ I.   Modélisation des menaces          │ ✅        │ 100%           │
│ II.  Analyse de risques                │ ✅        │ 100%           │
│ III. Sécurité dès conception           │ 🔴 / ⚠️   │ XX/5 (XX%)     │
│ IV.  Tests de sécurité                 │ 🔴 / ⚠️   │ XX/4 (XX%)     │
│ V.   Gestion des secrets               │ ✅        │ 100%           │
│ VI.  Traçabilité                       │ ⚠️        │ XX/Y (XX%)     │
│ VII. Patch management                  │ ✅        │ 100%           │
└─────────────────────────────────────────────────────────────────────┘

Conformité: X/7 principes (XX%)  [↗️ +X% / → stable / ↘️ -X%]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                           REGISTRE DES RISQUES
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

┌─────────────────────────────────────────────────────────────────────┐
│                  │ CRITIQUE │ IMPORTANT │ MINEUR │ TOTAL            │
├──────────────────┼──────────┼───────────┼────────┼──────────────────┤
│ 🔴 Ouverts       │    X     │     X     │   X    │   X              │
│ 🟡 En cours      │    X     │     X     │   X    │   X              │
│ ✅ Résolus       │    X     │     X     │   X    │   X              │
│ ⚪ Acceptés      │    X     │     X     │   X    │   X              │
├──────────────────┼──────────┼───────────┼────────┼──────────────────┤
│ TOTAL            │    X     │     X     │   X    │   X              │
└─────────────────────────────────────────────────────────────────────┘

🔴 Risques Critiques Ouverts: X
   [Si > 0 : Lister les IDs et titres]

🟠 Risques Importants Ouverts: X
   [Si > 5 : Afficher seulement le top 5 par score]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        RÉPARTITION PAR PRINCIPE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[Graphique ASCII horizontal par principe]

I.   Modélisation     ✅ Aucun risque
II.  Analyse risques  ✅ Aucun risque
III. Conception       🔴 ████████░░ (8 risques)
IV.  Tests            🔴 ██████░░░░ (6 risques)
V.   Secrets          ✅ Aucun risque
VI.  Traçabilité      ⚠️  ███░░░░░░░ (3 risques)
VII. Patch mgmt       ✅ Aucun risque

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                            MÉTRIQUES CLÉS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

┌─────────────────────────────────────────────────────────────────────┐
│ Score Risque Résiduel            │ XXX points                       │
│   (Après mitigations)            │ [Réduction: ↓XX% vs initial]     │
├──────────────────────────────────┼──────────────────────────────────┤
│ Security Debt Score              │ XX points                        │
│   (Risques critiques ouverts)    │ [Cible: 0]                       │
├──────────────────────────────────┼──────────────────────────────────┤
│ Couverture Fonctionnalités       │ XX% (X/Y features)               │
│   (Features avec SEC-*.md)       │ [Cible: 100%]                    │
├──────────────────────────────────┼──────────────────────────────────┤
│ MTTR Risques Critiques           │ X.X jours                        │
│   (Temps moyen de résolution)    │ [SLA: < 2 jours]                 │
├──────────────────────────────────┼──────────────────────────────────┤
│ Taux de Résolution               │ XX%                              │
│   (Résolus / Total)              │ [Cible: > 80%]                   │
├──────────────────────────────────┼──────────────────────────────────┤
│ Dernier Audit                    │ [DATE] (il y a X jours)          │
│   (Prochain recommandé)          │ [DATE + 90 jours]                │
└─────────────────────────────────────────────────────────────────────┘

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                          ÉVOLUTION TEMPORELLE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[Si plusieurs audits disponibles]

Conformité aux 7 Principes (%)

100% ┤
 90% ┤
 80% ┤                                    ●─────●
 70% ┤                         ●─────●
 60% ┤              ●─────●
 50% ┤   ●─────●
     └─────┬─────┬─────┬─────┬─────┬─────┬─────→
          [Dates des audits]

↗️ Progression: +XX% en X jours

Risques Critiques (Nombre)

 10  ┤   ●
  8  ┤       ●
  6  ┤           ●─────●
  4  ┤                     ●
  2  ┤                         ●─────●
  0  ┤
     └─────┬─────┬─────┬─────┬─────┬─────→
          [Dates]

↘️ Réduction: -X risques en X jours

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                      RECOMMANDATIONS PRIORITAIRES
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🔴 URGENT (< 48h)

  1. [RISK-XXX] - [Titre court]
     Principe: [Numéro] | Score: XXX | Délai dépassé de X jours
     Fichier: SEC-[FEATURE].md
     Action: [Mitigation proposée]

  2. [RISK-XXX] - [Titre court]
     [...]

🟠 IMPORTANT (< 7j)

  1. [Action prioritaire]
  2. [Action prioritaire]

🟡 AMÉLIORATION (< 30j)

  1. Augmenter couverture tests sécurité (Principe IV)
  2. [Autres améliorations]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                          ACTIONS SUGGÉRÉES
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[Si risques critiques > 0]
➡️  1. Corriger les X risques critiques identifiés
➡️  2. Lancer /audit pour vérifier les corrections

[Si conformité < 100%]
➡️  1. Analyser les principes non conformes
➡️  2. Consulter constitution.md pour les exigences

[Si aucun audit récent]
➡️  Dernier audit il y a X jours. Lancer /audit pour mise à jour

[Si tout est conforme]
✅ Excellente posture de sécurité !
   Prochain audit recommandé: [DATE]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📄 Rapport complet: docs/security/DASHBOARD.md
📊 Risk Register: docs/security/risks/risk-register.yaml
📋 Dernier audit: docs/security/audits/AUDIT-[DATE].md

Généré le [DATE] par OpenSecKit /dashboard
```

---

## Étape 3: Génération du Fichier DASHBOARD.md

**Fichier** : `docs/security/DASHBOARD.md`

```markdown
# Security Dashboard

> Tableau de bord de sécurité du projet
> Généré automatiquement par OpenSecKit
> Date: [DATE]

---

## Vue d'Ensemble

**Projet** : [Nom]
**Stack** : [Stack technique depuis meta.md]
**Dernière mise à jour** : [Date depuis docs/security/risks/risk-register.yaml]
**Statut Production** : [✅ VALIDÉ / ⚠️ RISQUES / 🔴 BLOQUÉ]

---

## Conformité Constitutionnelle

### Score Global

**[XX%]** de conformité aux 7 principes

```
█████████░░░░░░░ XX%
```

**Évolution** : [↗️ +X% / → stable / ↘️ -X%] depuis dernier audit

### Détail par Principe

| Principe | Statut | Couverture | Risques Ouverts | Commentaire |
|----------|--------|------------|-----------------|-------------|
| I. Modélisation des menaces | ✅ Conforme | 100% | 0 | Toutes les features ont une modélisation |
| II. Analyse de risques | ✅ Conforme | 100% | 0 | Risk register à jour |
| III. Sécurité dès conception | 🔴 Non conforme | 3/5 (60%) | 8 | Contrôles manquants: validation, rate limit |
| IV. Tests de sécurité | ⚠️ Partiel | 2/4 (50%) | 6 | SAST configuré, DAST manquant |
| V. Gestion des secrets | ✅ Conforme | 100% | 0 | Gestionnaire actif, rotation configurée |
| VI. Traçabilité | ⚠️ Partiel | 3/5 (60%) | 3 | Logs auth manquants |
| VII. Patch management | ✅ Conforme | 100% | 0 | Scan quotidien, SLA respectés |

**Principes conformes** : X/7

---

## Registre des Risques

### Vue d'Ensemble

| Sévérité | Ouverts | En cours | Résolus | Acceptés | Total |
|----------|---------|----------|---------|----------|-------|
| 🔴 Critique | X | X | X | X | X |
| 🟠 Important | X | X | X | X | X |
| 🟡 Mineur | X | X | X | X | X |
| **TOTAL** | **X** | **X** | **X** | **X** | **X** |

### Risques Critiques Ouverts (X)

[Si aucun : "✅ Aucun risque critique ouvert"]

[Si présents :]

#### RISK-LOGIN-001 - Injection SQL dans /auth
- **Principe violé** : III. Sécurité dès conception
- **Score** : 100 (Impact: 5, Probabilité: 4, Exposition: 5)
- **Statut** : 🔴 OUVERT
- **Date création** : 2025-01-10
- **Date limite** : 2025-01-12 (⚠️ **Dépassée de 3 jours**)
- **Feature** : [SEC-LOGIN.md](SEC-LOGIN.md)
- **Mitigation** : Utiliser requêtes préparées
- **Action** : Voir SEC-LOGIN.md ligne 42

[etc. pour chaque risque critique]

### Top 5 Risques par Score

1. **RISK-LOGIN-001** (Score: 100) - Injection SQL
2. **RISK-API-003** (Score: 80) - CORS mal configuré
3. **RISK-LOGIN-002** (Score: 48) - Absence rate limiting
4. **RISK-LOGIN-003** (Score: 48) - Timing attack
5. **RISK-ADMIN-001** (Score: 32) - Logs insuffisants

### Répartition par Principe

```
I.   Modélisation     ✅ Aucun risque
II.  Analyse risques  ✅ Aucun risque
III. Conception       🔴 ████████░░ (8 risques - 40% résolus)
IV.  Tests            🔴 ██████░░░░ (6 risques - 33% résolus)
V.   Secrets          ✅ Aucun risque
VI.  Traçabilité      ⚠️  ███░░░░░░░ (3 risques - 0% résolus)
VII. Patch mgmt       ✅ Aucun risque
```

---

## Métriques Clés

### Security Debt Score

**XX points** (Risque critique = 10 pts, Important = 3 pts, Mineur = 1 pt)

```
[Si > 20 : 🔴 Dette élevée]
[Si 10-20 : 🟠 Dette modérée]
[Si < 10 : 🟡 Dette faible]
[Si 0 : ✅ Aucune dette]
```

**Objectif** : 0 points

### Couverture Fonctionnalités

**XX%** (X features avec SEC-*.md / Y features totales)

[Liste des features analysées]
- ✅ Login (features/SEC-LOGIN.md)
- ✅ API (features/SEC-API.md)
- ❌ Admin (pas d'analyse)
- ❌ Export PDF (pas d'analyse)

**Objectif** : 100%

### MTTR (Mean Time To Resolve)

**Risques Critiques** : X.X jours (SLA: < 2 jours)
**Risques Importants** : X.X jours (SLA: < 7 jours)

[Si MTTR > SLA : ⚠️ SLA non respecté]

**Calcul** : Moyenne des (date_resolution - date_creation) pour risques résolus

### Taux de Résolution

**XX%** (X résolus / Y total)

```
█████████████░░░░░░░ XX%
```

**Objectif** : > 80%

### Historique des Audits

- **Dernier audit** : [DATE] (il y a X jours)
- **Audit précédent** : [DATE] (il y a X jours)
- **Fréquence** : Tous les X jours (moyenne)

**Prochain audit recommandé** : [DATE + 90 jours]

---

## Évolution Temporelle

[Si plusieurs audits disponibles, générer graphiques]

### Conformité Globale

| Date | Conformité | Évolution |
|------|------------|-----------|
| 2025-01-15 | 71% (5/7) | +14% ↗️ |
| 2025-01-08 | 57% (4/7) | +14% ↗️ |
| 2025-01-01 | 43% (3/7) | - |

```
Conformité (%)

100% ┤
 80% ┤                      ●
 60% ┤           ●─────●
 40% ┤   ●
     └──────┬──────┬──────┬──────→
         01/01  08/01  15/01
```

### Risques Critiques

| Date | Ouverts | Résolus (total) |
|------|---------|-----------------|
| 2025-01-15 | 2 | 5 |
| 2025-01-08 | 4 | 3 |
| 2025-01-01 | 7 | 0 |

**Tendance** : ↘️ -5 risques critiques en 14 jours

---

## Recommandations Prioritaires

### 🔴 URGENT (Sous 48h)

1. **Corriger RISK-LOGIN-001** (Injection SQL)
   - Principe III - Sécurité dès conception
   - Délai dépassé de 3 jours
   - Action : Implémenter requêtes préparées dans src/auth/login.js

2. **Corriger RISK-API-003** (CORS)
   - Principe III - Sécurité dès conception
   - Score: 80
   - Action : Configurer whitelist CORS dans src/server.js

### 🟠 IMPORTANT (Sous 7 jours)

1. **Ajouter DAST en CI/CD** (Principe IV)
   - Couverture tests seulement 50%
   - Action : Configurer OWASP ZAP dans .github/workflows/

2. **Implémenter rate limiting** (RISK-LOGIN-002)
   - Principe III - Défense en profondeur
   - Action : Ajouter express-rate-limit

3. **Ajouter logs échec auth** (RISK-LOG-001)
   - Principe VI - Traçabilité
   - Action : Logger dans src/auth/login.js ligne 67

### 🟡 AMÉLIORATION (Sous 30 jours)

1. Augmenter couverture tests sécurité à 100% (Principe IV)
2. Analyser features Admin et Export PDF (/security)
3. Documenter procédure escalade incidents

---

## Plan d'Action

### Cette Semaine

- [ ] Corriger RISK-LOGIN-001 (injection SQL)
- [ ] Corriger RISK-API-003 (CORS)
- [ ] Ajouter DAST en CI/CD
- [ ] Lancer /audit pour valider

### Ce Mois

- [ ] Implémenter rate limiting
- [ ] Ajouter logs échec auth
- [ ] Analyser features manquantes
- [ ] Atteindre 100% conformité

### Ce Trimestre

- [ ] Maintenir 100% conformité
- [ ] Audit externe recommandé
- [ ] Formation équipe sur principes
- [ ] Automatiser dashboard hebdomadaire

---

## Ressources

- **Constitution** : [constitution.md](../../constitution.md)
- **Risk Register** : [risk-register.yaml](risks/risk-register.yaml)
- **Dernier Audit** : [AUDIT-[DATE].md](audits/AUDIT-[DATE].md)
- **Features Analysées** :
  - [SEC-LOGIN.md](features/SEC-LOGIN.md)
  - [SEC-API.md](features/SEC-API.md)

---

## Notes

[Si incidents récents]
**⚠️ Incidents Récents** :
- [DATE] - [INC-XXX.md] - [Titre]

[Si dette élevée]
**⚠️ Dette de Sécurité Élevée** :
La dette de sécurité (XX points) dépasse le seuil acceptable. Prioriser la résolution des risques critiques.

[Si tout va bien]
**✅ Excellente Posture de Sécurité** :
Tous les principes sont conformes. Continuer les audits trimestriels.

---

*Généré automatiquement le [DATE] par OpenSecKit /dashboard*
*Prochaine mise à jour recommandée : [DATE + 7 jours]*
```

---

## Étape 4: Calculs Spécifiques

### Security Debt Score

```
Score = (Critiques ouverts × 10) + (Importants ouverts × 3) + (Mineurs ouverts × 1)
```

**Seuils** :
- 0 : ✅ Aucune dette
- 1-10 : 🟡 Dette faible
- 11-20 : 🟠 Dette modérée
- 21+ : 🔴 Dette élevée

### MTTR (Mean Time To Resolve)

```
MTTR = Moyenne des (date_resolution - date_creation) pour risques résolus

Exemple :
- RISK-001 : Résolu en 2 jours
- RISK-002 : Résolu en 5 jours
- RISK-003 : Résolu en 1 jour
MTTR = (2 + 5 + 1) / 3 = 2.67 jours
```

**Comparer au SLA** :
- Critiques : < 2 jours
- Importants : < 7 jours

### Taux de Résolution

```
Taux = (Risques résolus + acceptés) / Risques totaux × 100
```

### Couverture Fonctionnalités

```
Couverture = Nombre de docs/security/features/SEC-*.md / Nombre de features totales × 100
```

**Détecter features** :
- Scanner le code pour identifier modules/routes/controllers principaux
- Comparer avec fichiers `docs/security/features/SEC-*.md` existants

---

## Étape 5: Graphiques ASCII

### Barre de Progression

```javascript
// Fonction pour générer barre
function progressBar(percentage, width = 20) {
  const filled = Math.round(percentage / 100 * width);
  const empty = width - filled;
  return '█'.repeat(filled) + '░'.repeat(empty) + ` ${percentage}%`;
}

// Exemple
progressBar(71, 20) → "██████████████░░░░░░ 71%"
```

### Graphique Temporel

```javascript
// Graphique vertical simple
// Échelle : 0-100% sur axe Y, dates sur axe X

function timeGraph(dataPoints) {
  // dataPoints = [{date: "01/01", value: 43}, ...]
  // Générer grille ASCII avec points et lignes
}
```

### Graphique Horizontal par Principe

```javascript
function principleBar(riskCount, maxRisks = 10) {
  const bars = Math.min(Math.round(riskCount / maxRisks * 10), 10);
  const empty = 10 - bars;
  return '█'.repeat(bars) + '░'.repeat(empty) + ` (${riskCount} risques)`;
}
```

---

## Règles Importantes

1. **Données manquantes** : Si risk-register.yaml absent, créer dashboard vide avec instructions
2. **Calculs robustes** : Gérer divisions par zéro, données manquantes
3. **Graphiques clairs** : Utiliser caractères Unicode (█ ░ ● ─ ┤ etc.)
4. **Couleurs sémantiques** : 🔴 critique, 🟠 important, 🟡 mineur, ✅ ok, ⚠️ warning
5. **Actionnable** : Toujours inclure recommandations concrètes
6. **Performance** : Dashboard doit se générer en < 2 secondes
7. **Mise à jour auto** : Suggérer prochaine génération (hebdomadaire recommandé)

---

# Format de Sortie Final

1. **Afficher dashboard terminal** (sortie immédiate)
2. **Générer DASHBOARD.md** (fichier persistant)
3. **Message de confirmation** :

```
✅ Dashboard généré

📊 Vue terminal affichée ci-dessus
📄 Rapport complet: docs/security/DASHBOARD.md

État: [✅ VALIDÉ / ⚠️ RISQUES / 🔴 BLOQUÉ]

Conformité: XX% (X/7 principes)
Risques critiques: X
Security Debt: XX points

➡️  Prochaines actions: Voir section "Recommandations Prioritaires"
➡️  Mise à jour recommandée: Dans 7 jours ([DATE])
```
