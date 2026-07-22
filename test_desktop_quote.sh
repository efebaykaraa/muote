#!/bin/bash
# Test script for desktop-quote
# This creates a sample quote cache and runs the desktop-quote binary

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Desktop Quote Test Script ===${NC}\n"

# Get the config directory
CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/muote"
CACHE_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/muote"

echo -e "${YELLOW}Creating test directories...${NC}"
mkdir -p "$CONFIG_DIR"
mkdir -p "$CACHE_DIR"

# Create a sample config if it doesn't exist
CONFIG_FILE="$CONFIG_DIR/config.json"
if [ ! -f "$CONFIG_FILE" ]; then
    echo -e "${YELLOW}Creating sample config at $CONFIG_FILE${NC}"
    cat > "$CONFIG_FILE" << 'EOF'
{
  "appearance": {
    "font": "Sans",
    "font_size": 24,
    "text_color": "#ffffff",
    "bg_enabled": true,
    "bg_color": "#000000cc",
    "bg_style": "rounded",
    "stroke_enabled": true,
    "stroke_color": "#000000",
    "stroke_width": 2.0,
    "shadow_enabled": false,
    "shadow_color": "#000000",
    "shadow_offset": 2.0,
    "quote_x": 100,
    "quote_y": 100,
    "quote_max_width": 800,
    "quote_max_height": 400,
    "author_x": 150,
    "author_y": 520
  }
}
EOF
else
    echo -e "${GREEN}Config already exists at $CONFIG_FILE${NC}"
fi

# Create a sample quote cache
CACHE_FILE="$CACHE_DIR/quote.json"
echo -e "${YELLOW}Creating sample quote at $CACHE_FILE${NC}"
cat > "$CACHE_FILE" << 'EOF'
{
  "quote": "The philosophers have only interpreted the world, in various ways. The point, however, is to change it.",
  "author": "Karl Marx"
}
EOF

echo -e "${GREEN}✓ Test files created${NC}\n"

# Build the binary if needed
if [ ! -f "target/debug/desktop-quote" ]; then
    echo -e "${YELLOW}Building desktop-quote...${NC}"
    cargo build --bin desktop-quote
    echo -e "${GREEN}✓ Build complete${NC}\n"
fi

# Show the config and cache locations
echo -e "${BLUE}Configuration:${NC}"
echo -e "  Config: $CONFIG_FILE"
echo -e "  Cache:  $CACHE_FILE\n"

# Run the desktop-quote binary
echo -e "${BLUE}Starting desktop-quote...${NC}"
echo -e "${YELLOW}Press Ctrl+C to stop${NC}\n"

./target/debug/desktop-quote
