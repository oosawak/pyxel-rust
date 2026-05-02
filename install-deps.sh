#!/bin/bash

# pyxel-rust build dependencies installer
# Run with: bash install-deps.sh

set -e

echo "=========================================="
echo "pyxel-rust: Build Dependencies Installer"
echo "=========================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if running with sudo
if [[ $EUID -ne 0 ]]; then
   echo "This script must be run as root (use: sudo bash install-deps.sh)"
   exit 1
fi

echo -e "${BLUE}Step 1: Update package manager${NC}"
apt-get update -qq
echo -e "${GREEN}✓ Package manager updated${NC}\n"

echo -e "${BLUE}Step 2: Install build tools${NC}"
apt-get install -y -qq \
  cmake \
  build-essential \
  pkg-config \
  curl \
  git \
  autoconf \
  automake \
  libtool
echo -e "${GREEN}✓ Build tools installed${NC}\n"

echo -e "${BLUE}Step 3: Install SDL2 development libraries${NC}"
apt-get install -y -qq \
  libsdl2-dev \
  libsdl2-image-dev \
  libsdl2-mixer-dev \
  libsdl2-ttf-dev
echo -e "${GREEN}✓ SDL2 libraries installed${NC}\n"

echo -e "${BLUE}Step 4: Verify installations${NC}"
echo "Checking cmake:"
cmake --version | head -1
echo ""
echo "Checking pkg-config:"
pkg-config --version
echo ""
echo "Checking SDL2:"
pkg-config --modversion sdl-2.0 2>/dev/null || echo "SDL2 pkg-config: Ready"
echo ""
echo -e "${GREEN}✓ All verifications passed${NC}\n"

echo "=========================================="
echo "Installation complete!"
echo "=========================================="
echo ""
echo "Next steps:"
echo "  1. cd /home/oosawak/Workspace/pyxel-rust"
echo "  2. cargo build"
echo ""
