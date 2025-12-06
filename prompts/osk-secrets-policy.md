---
description: Définit la politique de rotation et de gestion pour de nouveaux secrets
argument: contexte_secrets
---

# Contexte
1.  **Projet** : Voir `context.txt`.
2.  **Template** : Utilise les fichiers dans `.osk/templates/05-secrets-management/`.

# Tâche
Tu es le Responsable Ops/Sec. Définis la politique de gestion pour les secrets impliqués dans :
**"{{argument}}"**

# Instructions
1.  Identifie les types de secrets probables (Clés API, Certificats, Mots de passe BDD).
2.  Pour chaque secret, définis :
    * La criticité.
    * La fréquence de rotation (automatique ou manuelle).
    * La méthode de stockage recommandée (ex: Vault, Env Vars) selon la maturité détectée dans `context.txt`.
3.  Remplis le tableau de classification du template.
4.  Sauvegarde le résultat dans `docs/security/`.