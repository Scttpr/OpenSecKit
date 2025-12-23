#!/bin/bash
# =============================================================================
# OpenSecKit - Test Local
# =============================================================================
# Teste le CLI et les prompts sur un projet cobaye avant de pousser.
#
# Usage:
#   ./scripts/test-local.sh           # Crée un projet test temporaire
#   ./scripts/test-local.sh /path     # Teste sur un projet existant
#   ./scripts/test-local.sh --clean   # Nettoie les projets test précédents
#
# Le script copie automatiquement les ressources LOCALES (non-publiées)
# pour tester les modifications avant de pousser.
# =============================================================================

set -e

# Couleurs
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m'

# Répertoire racine d'OpenSecKit
OSK_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CLI_BIN="$OSK_ROOT/cli/target/debug/osk"

echo -e "${BOLD}"
echo "╔═══════════════════════════════════════════════════════════════════════════╗"
echo "║                    OpenSecKit - Test Local                                 ║"
echo "╚═══════════════════════════════════════════════════════════════════════════╝"
echo -e "${NC}"

# Option --clean
if [[ "$1" == "--clean" ]]; then
    echo -e "${YELLOW}🧹 Nettoyage des projets test...${NC}"
    rm -rf /tmp/osk-test-*
    echo -e "${GREEN}✅ Nettoyé${NC}"
    exit 0
fi

# =============================================================================
# Étape 1 : Build du CLI
# =============================================================================
echo -e "${BLUE}[1/4] 🔨 Compilation du CLI...${NC}"

cd "$OSK_ROOT/cli"
if ! cargo build; then
    echo -e "${RED}❌ Erreur de compilation${NC}"
    exit 1
fi
echo -e "${GREEN}   ✓ CLI compilé${NC}"

# =============================================================================
# Étape 2 : Préparation du projet test
# =============================================================================
echo -e "${BLUE}[2/4] 📦 Préparation du projet test...${NC}"

if [[ -n "$1" && -d "$1" ]]; then
    TEST_DIR="$1"
    echo -e "   Utilisation du projet existant: ${BOLD}$TEST_DIR${NC}"
    EXISTING_PROJECT=true
else
    TEST_DIR=$(mktemp -d /tmp/osk-test-XXXXXX)
    echo -e "   Nouveau projet test: ${BOLD}$TEST_DIR${NC}"
    EXISTING_PROJECT=false

    cd "$TEST_DIR"
    git init -q

    # Créer une structure réaliste
    cat > package.json << 'EOF'
{
  "name": "test-webapp",
  "version": "1.0.0",
  "dependencies": {
    "express": "^4.18.0",
    "jsonwebtoken": "^9.0.0",
    "bcrypt": "^5.1.0",
    "pg": "^8.11.0"
  }
}
EOF

    mkdir -p src/models src/routes src/middleware

    cat > src/models/user.js << 'EOF'
// User model with personal data (RGPD relevant)
const User = {
  id: 'uuid',
  email: 'string',
  password_hash: 'string',
  first_name: 'string',
  last_name: 'string',
  phone: 'string',
  created_at: 'datetime',
  last_login: 'datetime',
  ip_address: 'string'
};

module.exports = User;
EOF

    cat > src/routes/auth.js << 'EOF'
const express = require('express');
const jwt = require('jsonwebtoken');
const bcrypt = require('bcrypt');

const router = express.Router();

// POST /login
router.post('/login', async (req, res) => {
  const { email, password } = req.body;
  // TODO: Add rate limiting
  // TODO: Add input validation
  const user = await findUserByEmail(email);
  if (user && await bcrypt.compare(password, user.password_hash)) {
    const token = jwt.sign({ userId: user.id }, process.env.JWT_SECRET);
    res.json({ token });
  } else {
    res.status(401).json({ error: 'Invalid credentials' });
  }
});

module.exports = router;
EOF

    cat > src/middleware/auth.js << 'EOF'
const jwt = require('jsonwebtoken');

module.exports = (req, res, next) => {
  const token = req.headers.authorization?.split(' ')[1];
  if (!token) {
    return res.status(401).json({ error: 'No token provided' });
  }
  try {
    req.user = jwt.verify(token, process.env.JWT_SECRET);
    next();
  } catch (err) {
    res.status(401).json({ error: 'Invalid token' });
  }
};
EOF

    cat > .env.example << 'EOF'
DATABASE_URL=postgresql://user:password@localhost:5432/mydb
JWT_SECRET=change-me-in-production
EOF

    echo -e "${GREEN}   ✓ Projet test créé avec stack Node.js/Express/PostgreSQL${NC}"
fi

# =============================================================================
# Étape 3 : Exécution de osk init
# =============================================================================
echo -e "${BLUE}[3/4] 🚀 Exécution de osk init...${NC}"

cd "$TEST_DIR"

# Supprimer l'ancienne config si on force le test
if [[ "$EXISTING_PROJECT" == false ]]; then
    rm -rf .osk .claude
fi

# Exécuter osk init en mode non-interactif
"$CLI_BIN" init --default 2>&1 | while read -r line; do
    echo "   $line"
done

echo -e "${GREEN}   ✓ osk init terminé${NC}"

# Copie des ressources locales (non-publiées) par-dessus
echo -e "   📂 Copie des ressources locales..."
cp -r "$OSK_ROOT/prompts/"*.md .osk/prompts/ 2>/dev/null || true
cp -r "$OSK_ROOT/templates/"* .osk/templates/ 2>/dev/null || true
cp -r "$OSK_ROOT/domaines/"* .osk/domaines/ 2>/dev/null || true

# Synchronise les slash commands depuis les prompts locaux
cp .osk/prompts/*.md .claude/commands/ 2>/dev/null || true
echo -e "${GREEN}   ✓ Ressources locales installées${NC}"

# =============================================================================
# Étape 4 : Vérification
# =============================================================================
echo -e "${BLUE}[4/4] ✅ Vérification de l'installation...${NC}"

ERRORS=0

# Config
if [[ -f .osk/config.toml ]]; then
    echo -e "   ${GREEN}✓${NC} .osk/config.toml"
else
    echo -e "   ${RED}✗${NC} .osk/config.toml manquant"
    ((ERRORS++))
fi

# Registry
if [[ -f .osk/registry.toml ]]; then
    echo -e "   ${GREEN}✓${NC} .osk/registry.toml"
else
    echo -e "   ${RED}✗${NC} .osk/registry.toml manquant"
    ((ERRORS++))
fi

# Prompts
PROMPT_COUNT=$(ls .osk/prompts/*.md 2>/dev/null | wc -l)
if [[ $PROMPT_COUNT -ge 8 ]]; then
    echo -e "   ${GREEN}✓${NC} .osk/prompts/ ($PROMPT_COUNT fichiers)"
else
    echo -e "   ${RED}✗${NC} .osk/prompts/ ($PROMPT_COUNT fichiers, attendu >= 8)"
    ((ERRORS++))
fi

# Slash commands
CMD_COUNT=$(ls .claude/commands/*.md 2>/dev/null | wc -l)
if [[ $CMD_COUNT -ge 8 ]]; then
    echo -e "   ${GREEN}✓${NC} .claude/commands/ ($CMD_COUNT slash commands)"
else
    echo -e "   ${RED}✗${NC} .claude/commands/ ($CMD_COUNT fichiers, attendu >= 8)"
    ((ERRORS++))
fi

# Templates
TEMPLATE_COUNT=$(find .osk/templates -name "*.md" 2>/dev/null | wc -l)
if [[ $TEMPLATE_COUNT -ge 20 ]]; then
    echo -e "   ${GREEN}✓${NC} .osk/templates/ ($TEMPLATE_COUNT templates)"
else
    echo -e "   ${YELLOW}⚠${NC} .osk/templates/ ($TEMPLATE_COUNT templates)"
fi

# Domaines
DOMAINE_COUNT=$(find .osk/domaines -name "*.md" 2>/dev/null | wc -l)
if [[ $DOMAINE_COUNT -ge 10 ]]; then
    echo -e "   ${GREEN}✓${NC} .osk/domaines/ ($DOMAINE_COUNT fichiers)"
else
    echo -e "   ${YELLOW}⚠${NC} .osk/domaines/ ($DOMAINE_COUNT fichiers)"
fi

# Stack détectée
if grep -q "nodejs\|node\|javascript" .osk/config.toml 2>/dev/null; then
    echo -e "   ${GREEN}✓${NC} Stack Node.js détectée"
else
    echo -e "   ${YELLOW}⚠${NC} Stack Node.js non détectée"
fi

# =============================================================================
# Résumé
# =============================================================================
echo ""
echo -e "${BOLD}═══════════════════════════════════════════════════════════════════════════${NC}"

if [[ $ERRORS -eq 0 ]]; then
    echo -e "${GREEN}${BOLD}✅ Test réussi !${NC}"
else
    echo -e "${RED}${BOLD}❌ $ERRORS erreur(s) détectée(s)${NC}"
fi

echo ""
echo -e "${BOLD}📂 Projet test :${NC} $TEST_DIR"
echo ""
echo -e "${BOLD}🧪 Pour tester les slash commands :${NC}"
echo "   cd $TEST_DIR"
echo "   claude"
echo ""
echo -e "${BOLD}   Commandes à tester :${NC}"
echo "   /osk-configure          # Analyse le code, détecte RGPD"
echo "   /osk-baseline           # État des lieux sécurité"
echo "   /osk-analyze auth       # Analyse feature authentification"
echo ""
echo -e "${BOLD}🧹 Nettoyage :${NC}"
echo "   ./scripts/test-local.sh --clean"
echo ""
echo -e "${BOLD}═══════════════════════════════════════════════════════════════════════════${NC}"

exit $ERRORS
