#!/bin/bash
# =============================================================================
# OpenSecKit - Link Checker avec résumé
# =============================================================================
# Vérifie tous les liens dans les fichiers Markdown et affiche un tableau
# récapitulatif des liens morts à la fin.
#
# Usage:
#   ./scripts/check-links.sh           # Tous les fichiers .md trackés par git
#   ./scripts/check-links.sh file.md   # Un fichier spécifique
#   ./scripts/check-links.sh dir/      # Tous les .md dans un répertoire
# =============================================================================

set -o pipefail

# Couleurs
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color
BOLD='\033[1m'

# Configuration
CONFIG_FILE=".github/link-check-config.json"
TEMP_DIR=$(mktemp -d)
DEAD_LINKS_FILE="$TEMP_DIR/dead_links.txt"
SUMMARY_FILE="$TEMP_DIR/summary.txt"

# Compteurs
TOTAL_FILES=0
FILES_WITH_ERRORS=0
TOTAL_DEAD_LINKS=0

# Nettoyage à la sortie
cleanup() {
    rm -rf "$TEMP_DIR"
}
trap cleanup EXIT

# Initialiser les fichiers temporaires
> "$DEAD_LINKS_FILE"
> "$SUMMARY_FILE"

# Fonction pour vérifier un fichier
check_file() {
    local file="$1"
    local output
    local exit_code

    ((TOTAL_FILES++))

    echo -e "${BLUE}📄 Checking:${NC} $file"

    # Exécuter markdown-link-check et capturer la sortie
    if [[ -f "$CONFIG_FILE" ]]; then
        output=$(npx markdown-link-check --config "$CONFIG_FILE" "$file" 2>&1)
        exit_code=$?
    else
        output=$(npx markdown-link-check "$file" 2>&1)
        exit_code=$?
    fi

    # Parser la sortie pour extraire les liens morts
    while IFS= read -r line; do
        # Détecter les lignes avec des erreurs (✖ ou [✖])
        if echo "$line" | grep -qE '^\s*\[?[✖✗×]'; then
            ((TOTAL_DEAD_LINKS++))

            # Extraire l'URL et le status
            # Format typique: "  [✖] https://example.com → Status: 404"
            url=$(echo "$line" | sed -E 's/.*\[?[✖✗×]\]?\s*//; s/\s*→.*//; s/\s*->.*//')
            status=$(echo "$line" | grep -oE '(Status:|→)\s*[0-9]+' | grep -oE '[0-9]+' || echo "ERR")

            # Ajouter au fichier des liens morts
            echo "$file|$url|$status" >> "$DEAD_LINKS_FILE"
        fi
    done <<< "$output"

    # Afficher le résultat pour ce fichier
    if [[ $exit_code -ne 0 ]]; then
        ((FILES_WITH_ERRORS++))
        echo -e "${RED}   ✗ Erreurs trouvées${NC}"
    else
        echo -e "${GREEN}   ✓ OK${NC}"
    fi
}

# Fonction pour afficher le tableau récapitulatif
print_summary() {
    echo ""
    echo -e "${BOLD}═══════════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${BOLD}                           RÉSUMÉ DES LIENS MORTS                          ${NC}"
    echo -e "${BOLD}═══════════════════════════════════════════════════════════════════════════${NC}"
    echo ""

    # Statistiques globales
    echo -e "${BOLD}📊 Statistiques:${NC}"
    echo -e "   Fichiers vérifiés  : $TOTAL_FILES"
    echo -e "   Fichiers en erreur : $FILES_WITH_ERRORS"
    echo -e "   Liens morts total  : $TOTAL_DEAD_LINKS"
    echo ""

    if [[ $TOTAL_DEAD_LINKS -eq 0 ]]; then
        echo -e "${GREEN}${BOLD}✅ Aucun lien mort trouvé !${NC}"
        return 0
    fi

    # Tableau des liens morts
    echo -e "${BOLD}📋 Détail des liens morts:${NC}"
    echo ""

    # Header du tableau
    printf "${BOLD}%-50s │ %-60s │ %s${NC}\n" "Fichier" "URL" "Status"
    printf "%-50s─┼─%-60s─┼─%s\n" "$(printf '─%.0s' {1..50})" "$(printf '─%.0s' {1..60})" "$(printf '─%.0s' {1..6})"

    # Contenu du tableau
    while IFS='|' read -r file url status; do
        # Tronquer le fichier et l'URL si trop longs
        file_display="${file:0:48}"
        [[ ${#file} -gt 48 ]] && file_display="${file_display}.."

        url_display="${url:0:58}"
        [[ ${#url} -gt 58 ]] && url_display="${url_display}.."

        # Colorer le status
        if [[ "$status" == "404" ]]; then
            status_color="${RED}$status${NC}"
        elif [[ "$status" =~ ^5 ]]; then
            status_color="${YELLOW}$status${NC}"
        else
            status_color="${RED}$status${NC}"
        fi

        printf "%-50s │ %-60s │ %b\n" "$file_display" "$url_display" "$status_color"
    done < "$DEAD_LINKS_FILE"

    echo ""

    # Grouper par fichier pour un résumé plus compact
    echo -e "${BOLD}📁 Résumé par fichier:${NC}"
    echo ""

    # Compter les liens morts par fichier
    awk -F'|' '{count[$1]++} END {for (f in count) print count[f], f}' "$DEAD_LINKS_FILE" | sort -rn | while read -r count file; do
        echo -e "   ${RED}$count${NC} lien(s) mort(s) dans ${BLUE}$file${NC}"
    done

    echo ""
    echo -e "${BOLD}═══════════════════════════════════════════════════════════════════════════${NC}"

    return 1
}

# Main
echo -e "${BOLD}"
echo "╔═══════════════════════════════════════════════════════════════════════════╗"
echo "║                    OpenSecKit - Link Checker                               ║"
echo "╚═══════════════════════════════════════════════════════════════════════════╝"
echo -e "${NC}"

# Déterminer les fichiers à vérifier
if [[ $# -eq 0 ]]; then
    # Tous les fichiers .md trackés par git
    echo -e "${BLUE}📂 Recherche des fichiers Markdown...${NC}"
    files=$(git ls-files '*.md' 2>/dev/null || find . -name "*.md" -not -path "./node_modules/*")
elif [[ -d "$1" ]]; then
    # Répertoire spécifié
    echo -e "${BLUE}📂 Recherche dans $1...${NC}"
    files=$(find "$1" -name "*.md" -not -path "*/node_modules/*")
else
    # Fichier(s) spécifique(s)
    files="$@"
fi

file_count=$(echo "$files" | grep -c . || echo 0)
echo -e "${BLUE}📝 $file_count fichier(s) à vérifier${NC}"
echo ""

# Vérifier chaque fichier
for file in $files; do
    if [[ -f "$file" ]]; then
        check_file "$file"
    fi
done

# Afficher le résumé
print_summary
exit_code=$?

exit $exit_code
