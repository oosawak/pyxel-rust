#!/bin/bash
# pyxel-rust CI/CD Pipeline
# Build, test, and notify Discord

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SCRIPTS_DIR="$PROJECT_DIR/scripts"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║         pyxel-rust CI/CD Pipeline                         ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
echo

# Parse arguments
SKIP_TESTS=false
RELEASE_BUILD=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --skip-tests)
            SKIP_TESTS=true
            ;;
        --release)
            RELEASE_BUILD=true
            ;;
    esac
    shift
done

STEP=0
TOTAL=4
if [ "$SKIP_TESTS" = false ]; then TOTAL=$((TOTAL+1)); fi

# 1. Format check
STEP=$((STEP+1))
echo -e "${BLUE}[$STEP/$TOTAL]${NC} ${YELLOW}Checking code formatting...${NC}"
cd "$PROJECT_DIR"
if cargo fmt -- --check 2>&1 | head -20; then
    echo -e "${GREEN}✓${NC} Code formatting OK"
else
    echo -e "${RED}✗${NC} Code needs formatting. Run: cargo fmt"
    "$SCRIPTS_DIR/notify-discord.sh" "❌ Format check failed" 16711680
    exit 1
fi
echo

# 2. Clippy linting
STEP=$((STEP+1))
echo -e "${BLUE}[$STEP/$TOTAL]${NC} ${YELLOW}Running Clippy linter...${NC}"
if cargo clippy --all-targets -- -D warnings 2>&1 | tail -10; then
    echo -e "${GREEN}✓${NC} Clippy checks passed"
else
    echo -e "${RED}✗${NC} Clippy warnings found"
    "$SCRIPTS_DIR/notify-discord.sh" "❌ Clippy check failed" 16711680
    exit 1
fi
echo

# 3. Build
STEP=$((STEP+1))
echo -e "${BLUE}[$STEP/$TOTAL]${NC} ${YELLOW}Building project...${NC}"
if cargo build 2>&1 | tail -10; then
    echo -e "${GREEN}✓${NC} Build successful"
else
    echo -e "${RED}✗${NC} Build failed"
    "$SCRIPTS_DIR/notify-discord.sh" "❌ Build failed" 16711680
    exit 1
fi
echo

# 4. Tests
if [ "$SKIP_TESTS" = false ]; then
    STEP=$((STEP+1))
    echo -e "${BLUE}[$STEP/$TOTAL]${NC} ${YELLOW}Running tests...${NC}"
    if cargo test --lib 2>&1 | tail -15; then
        echo -e "${GREEN}✓${NC} Tests passed"
    else
        echo -e "${RED}✗${NC} Tests failed"
        "$SCRIPTS_DIR/notify-discord.sh" "❌ Tests failed" 16711680
        exit 1
    fi
    echo
fi

# 5. Type checking
STEP=$((STEP+1))
echo -e "${BLUE}[$STEP/$TOTAL]${NC} ${YELLOW}Type checking...${NC}"
if cargo check --all-targets 2>&1 | tail -5; then
    echo -e "${GREEN}✓${NC} Type checking passed"
else
    echo -e "${RED}✗${NC} Type errors found"
    "$SCRIPTS_DIR/notify-discord.sh" "❌ Type checking failed" 16711680
    exit 1
fi
echo

# Final notification
echo -e "${GREEN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║              ✓ All checks passed!                         ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════════╝${NC}"
echo

COMMIT=$(cd "$PROJECT_DIR" && git rev-parse --short HEAD 2>/dev/null || echo "unknown")
"$SCRIPTS_DIR/notify-discord.sh" "✅ CI passed - $COMMIT" 65280

echo "Ready for merge! 🚀"
