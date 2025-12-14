```
 ███████╗ █████╗  ██████╗
 ██╔════╝██╔══██╗██╔═══██╗
 █████╗  ███████║██║   ██║
 ██╔══╝  ██╔══██║██║▄▄ ██║
 ██║     ██║  ██║╚██████╔╝
 ╚═╝     ╚═╝  ╚═╝ ╚══▀▀═╝
```

```
╔═══════════════════════════════════════════════════════════════════════════╗
║                                                                           ║
║  FAQ - Questions fréquentes                                               ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝
```

---

## ▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂
## DÉMARRAGE

**Par où commencer ?**

Suivez QUICK-START.md (10 minutes).

**Dois-je remplir TOUS les templates ?**

Non. Minimum vital : Modélisation menaces + Analyse risques + Détection secrets.

**Combien de temps ?**

Version rapide : 10-15 min. Version complète : 1-2 jours.

**Quel langage ?**

Tous. OpenSecKit est agnostique du langage.

---

## ▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂
## CLAUDE CODE

**Quels agents IA supportés ?**

Claude Code uniquement pour l'exécution automatisée. Autres LLMs via copier-coller manuel.

**Comment ça marche ?**

```
osk init → Installe slash commands dans .claude/commands/ → /audit dans Claude Code
```

**Mon code est envoyé où ?**

Nulle part. Tout reste local. Claude Code utilise votre session.

---

## ▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂
## CONFORMITÉ

**Comment utiliser les templates RGPD/NIS2/RGS ?**

```bash
osk init              # Installe les slash commands
claude
>>> /domain rgpd
>>> /domain nis2
>>> /domain rgs
```

Puis suivez les templates dans `domaines/{secteur}/`.

**Puis-je combiner plusieurs domaines ?**

Oui. Les templates sont compatibles entre eux.

---

## ▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂
## OUTILS

**Quels outils recommandés ?**

```
Détection secrets    gitleaks, truffleHog
SAST                 Semgrep, SonarQube
DAST                 OWASP ZAP, Burp Suite
SCA                  Dependabot, Snyk
```

**Intégration CI/CD ?**

Oui, via `osk ingest`. Voir cli/README.md section CI/CD.

---

## ▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂
## DÉPANNAGE

**'osk' n'est pas reconnu**

```bash
cd OpenSecKit/cli
cargo install --path .
osk --version
```

**Config introuvable**

```bash
osk init
```

**Templates obsolètes**

```bash
osk init --force
```

**Secret trouvé dans l'historique git**

```
[1] Révoquer le secret immédiatement
[2] Nettoyer l'historique (git-filter-repo, BFG)
[3] Force push (coordonner avec l'équipe)
[4] Installer gitleaks pre-commit hook
```

---

## ▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂
## CONTRIBUTION

**Puis-je contribuer ?**

Oui. Nouveaux templates, exemples, traductions, corrections.

Voir CONTRIBUTING.md.

**Comment proposer un template ?**

```
[1] Fork le repo
[2] Créer template dans templates/{principe}/
[3] Ajouter exemple _example-*.md
[4] Tester avec projet réel
[5] Pull request
```

---

## ▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂
## SUPPORT

```
[*] Issues      : https://github.com/Scttpr/OpenSecKit/issues
[*] Discussions : https://github.com/Scttpr/OpenSecKit/discussions
```

---

```
╔═══════════════════════════════════════════════════════════════════════════╗
║                                                                           ║
║  OpenSecKit FAQ v2.0.0                                                    ║
║  https://github.com/Scttpr/OpenSecKit                                    ║
║                                                                           ║
║  "Security as Code, AI-Ready"                                            ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝
```
