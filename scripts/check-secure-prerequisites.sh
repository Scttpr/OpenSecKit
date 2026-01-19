#!/bin/bash
# check-secure-prerequisites.sh
# Validates prerequisites for /osk-secure commands
# Usage: check-secure-prerequisites.sh <command> [feature_id]

set -e

COMMAND="${1:-}"
FEATURE_ID="${2:-}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Helper functions
check_file() {
    local file="$1"
    local desc="$2"
    if [[ -f "$file" ]]; then
        echo -e "${GREEN}✓${NC} $desc exists: $file"
        return 0
    else
        echo -e "${RED}✗${NC} $desc missing: $file"
        return 1
    fi
}

check_dir() {
    local dir="$1"
    local desc="$2"
    if [[ -d "$dir" ]]; then
        echo -e "${GREEN}✓${NC} $desc exists: $dir"
        return 0
    else
        echo -e "${RED}✗${NC} $desc missing: $dir"
        return 1
    fi
}

# Resolve feature spec directory
resolve_spec_dir() {
    local feature="$1"

    # Try specs/<feature>/
    if [[ -d "specs/$feature" ]]; then
        echo "specs/$feature"
        return 0
    fi

    # Try specs/<NNN>-<feature>/
    local match=$(find specs -maxdepth 1 -type d -name "*-$feature" 2>/dev/null | head -1)
    if [[ -n "$match" ]]; then
        echo "$match"
        return 0
    fi

    # Default to specs/<feature>/
    echo "specs/$feature"
}

# Usage
usage() {
    echo "Usage: $0 <command> [feature_id]"
    echo ""
    echo "Commands:"
    echo "  specify   - Check prerequisites for /osk-secure specify"
    echo "  plan      - Check prerequisites for /osk-secure plan"
    echo "  tasks     - Check prerequisites for /osk-secure tasks"
    echo "  implement - Check prerequisites for /osk-secure implement"
    echo "  clarify   - Check prerequisites for /osk-secure clarify"
    echo ""
    echo "Examples:"
    echo "  $0 specify payment-flow"
    echo "  $0 plan 042-payment-flow"
    echo "  $0 tasks"
    exit 1
}

# Main command handling
case "$COMMAND" in
    specify)
        echo "Checking prerequisites for /osk-secure specify..."
        echo ""

        # Requires: System model
        errors=0

        echo "Required: System model"
        check_dir ".osk/system-model" "System model directory" || ((errors++))

        if [[ -d ".osk/system-model" ]]; then
            check_file ".osk/system-model/index.yaml" "System model index" || ((errors++))
            check_file ".osk/system-model/architecture.yaml" "Architecture model" || ((errors++))
            check_file ".osk/system-model/data.yaml" "Data model" || ((errors++))
        fi

        echo ""
        echo "Required: Threat libraries"
        check_dir "knowledge/libraries/threats" "Threat libraries directory" || ((errors++))

        if [[ -d "knowledge/libraries/threats" ]]; then
            threat_count=$(find knowledge/libraries/threats -name "*.yaml" 2>/dev/null | wc -l)
            if [[ $threat_count -gt 0 ]]; then
                echo -e "${GREEN}✓${NC} Found $threat_count threat library files"
            else
                echo -e "${RED}✗${NC} No threat library files found"
                ((errors++))
            fi
        fi

        echo ""
        if [[ $errors -eq 0 ]]; then
            echo -e "${GREEN}All prerequisites met for /osk-secure specify${NC}"
            echo ""
            echo "Output files will be created:"
            if [[ -n "$FEATURE_ID" ]]; then
                spec_dir=$(resolve_spec_dir "$FEATURE_ID")
                echo "  - $spec_dir/security-spec.md"
                echo "  - $spec_dir/risks.yaml"
            else
                echo "  - specs/<feature>/security-spec.md"
                echo "  - specs/<feature>/risks.yaml"
            fi
            exit 0
        else
            echo -e "${RED}$errors prerequisite(s) missing${NC}"
            echo ""
            echo "Run /osk-discover init to create system model"
            exit 1
        fi
        ;;

    plan)
        echo "Checking prerequisites for /osk-secure plan..."
        echo ""

        errors=0

        if [[ -z "$FEATURE_ID" ]]; then
            echo -e "${YELLOW}Warning: No feature_id specified, checking general prerequisites${NC}"
            echo ""
        fi

        # Requires: security-spec.md
        echo "Required: Security specification"
        if [[ -n "$FEATURE_ID" ]]; then
            spec_dir=$(resolve_spec_dir "$FEATURE_ID")
            check_file "$spec_dir/security-spec.md" "Security specification" || ((errors++))
        else
            echo -e "${YELLOW}⚠${NC} Specify feature_id to check security-spec.md"
        fi

        echo ""
        if [[ $errors -eq 0 ]]; then
            echo -e "${GREEN}All prerequisites met for /osk-secure plan${NC}"
            echo ""
            echo "Output files will be created:"
            if [[ -n "$FEATURE_ID" ]]; then
                echo "  - $spec_dir/security-plan.md"
            else
                echo "  - specs/<feature>/security-plan.md"
            fi
            exit 0
        else
            echo -e "${RED}$errors prerequisite(s) missing${NC}"
            echo ""
            echo "Run /osk-secure specify first to create security-spec.md"
            exit 1
        fi
        ;;

    tasks)
        echo "Checking prerequisites for /osk-secure tasks..."
        echo ""

        errors=0

        if [[ -z "$FEATURE_ID" ]]; then
            echo -e "${YELLOW}Warning: No feature_id specified, checking general prerequisites${NC}"
            echo ""
        fi

        # Requires: security-plan.md
        echo "Required: Security plan"
        if [[ -n "$FEATURE_ID" ]]; then
            spec_dir=$(resolve_spec_dir "$FEATURE_ID")
            check_file "$spec_dir/security-plan.md" "Security plan" || ((errors++))
        else
            echo -e "${YELLOW}⚠${NC} Specify feature_id to check security-plan.md"
        fi

        echo ""
        if [[ $errors -eq 0 ]]; then
            echo -e "${GREEN}All prerequisites met for /osk-secure tasks${NC}"
            echo ""
            echo "Output files will be created/updated:"
            if [[ -n "$FEATURE_ID" ]]; then
                echo "  - $spec_dir/security-tasks.md (or security section in tasks.md)"
            else
                echo "  - specs/<feature>/security-tasks.md"
            fi
            exit 0
        else
            echo -e "${RED}$errors prerequisite(s) missing${NC}"
            echo ""
            echo "Run /osk-secure plan first to create security-plan.md"
            exit 1
        fi
        ;;

    implement)
        echo "Checking prerequisites for /osk-secure implement..."
        echo ""

        errors=0

        if [[ -z "$FEATURE_ID" ]]; then
            echo -e "${YELLOW}Warning: No feature_id specified, checking general prerequisites${NC}"
            echo ""
        fi

        # Requires: security-tasks.md and risks.yaml
        echo "Required: Security tasks and risk register"
        if [[ -n "$FEATURE_ID" ]]; then
            spec_dir=$(resolve_spec_dir "$FEATURE_ID")

            # Check for tasks (either security-tasks.md or tasks.md with security section)
            if [[ -f "$spec_dir/security-tasks.md" ]]; then
                echo -e "${GREEN}✓${NC} Security tasks exist: $spec_dir/security-tasks.md"
            elif [[ -f "$spec_dir/tasks.md" ]]; then
                echo -e "${GREEN}✓${NC} Tasks file exists: $spec_dir/tasks.md (verify security section)"
            else
                echo -e "${RED}✗${NC} No tasks file found in $spec_dir"
                ((errors++))
            fi

            check_file "$spec_dir/risks.yaml" "Risk register" || ((errors++))
        else
            echo -e "${YELLOW}⚠${NC} Specify feature_id to check security-tasks.md and risks.yaml"
        fi

        echo ""
        if [[ $errors -eq 0 ]]; then
            echo -e "${GREEN}All prerequisites met for /osk-secure implement${NC}"
            echo ""
            echo "Output files will be updated:"
            if [[ -n "$FEATURE_ID" ]]; then
                echo "  - $spec_dir/risks.yaml (mitigation status updates)"
            else
                echo "  - specs/<feature>/risks.yaml"
            fi
            exit 0
        else
            echo -e "${RED}$errors prerequisite(s) missing${NC}"
            echo ""
            echo "Run /osk-secure tasks first to create tasks"
            exit 1
        fi
        ;;

    clarify)
        echo "Checking prerequisites for /osk-secure clarify..."
        echo ""

        errors=0

        if [[ -z "$FEATURE_ID" ]]; then
            echo -e "${YELLOW}Warning: No feature_id specified, checking general prerequisites${NC}"
            echo ""
        fi

        # Requires: security-spec.md
        echo "Required: Security specification"
        if [[ -n "$FEATURE_ID" ]]; then
            spec_dir=$(resolve_spec_dir "$FEATURE_ID")
            check_file "$spec_dir/security-spec.md" "Security specification" || ((errors++))

            # Check for clarification markers
            if [[ -f "$spec_dir/security-spec.md" ]]; then
                markers=$(grep -c "\[NEEDS CLARIFICATION\]" "$spec_dir/security-spec.md" 2>/dev/null || echo "0")
                if [[ $markers -gt 0 ]]; then
                    echo -e "${YELLOW}⚠${NC} Found $markers [NEEDS CLARIFICATION] markers to resolve"
                else
                    echo -e "${GREEN}✓${NC} No [NEEDS CLARIFICATION] markers found"
                fi
            fi
        else
            echo -e "${YELLOW}⚠${NC} Specify feature_id to check security-spec.md"
        fi

        echo ""
        if [[ $errors -eq 0 ]]; then
            echo -e "${GREEN}All prerequisites met for /osk-secure clarify${NC}"
            echo ""
            echo "Output files will be updated:"
            if [[ -n "$FEATURE_ID" ]]; then
                echo "  - $spec_dir/security-spec.md (clarifications added)"
            else
                echo "  - specs/<feature>/security-spec.md"
            fi
            exit 0
        else
            echo -e "${RED}$errors prerequisite(s) missing${NC}"
            echo ""
            echo "Run /osk-secure specify first to create security-spec.md"
            exit 1
        fi
        ;;

    "")
        usage
        ;;

    *)
        echo "Unknown command: $COMMAND"
        echo ""
        usage
        ;;
esac
