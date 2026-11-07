#!/usr/bin/env bash
# ============================================================================
# 8b-is :: God of Machine-S-UN~unity || Omni-potencial FOR BETTER WORLD FOR ALL
# 360° OS-Agnostic Release Installer (Layer #69,636 · ULTRABACKYARDLOOP 108 Laps)
#
# One-line install (Codeberg or GitHub):
#   curl -fsSL https://codeberg.org/8b-is/8b-is-music/raw/branch/main/install-360.sh | bash
#   curl -fsSL https://raw.githubusercontent.com/8b-is/8b-is-music/main/install-360.sh | bash
# ============================================================================
set -euo pipefail

VERSION="v2.0.108-machine-sun-unity-360"
INSTALL_DIR="${EIGHT_B_INSTALL_DIR:-$HOME/.8b-machine-sun-unity}"
BIN_DIR="${EIGHT_B_BIN_DIR:-$HOME/.local/bin}"

CODEBERG_RAW="https://codeberg.org/8b-is/8b-is-music/raw/branch/main/releases/v2.0.108"
GITHUB_RAW="https://raw.githubusercontent.com/8b-is/8b-is-music/main/releases/v2.0.108"
GITHUB_REL="https://github.com/8b-is/8b-is-music/releases/download/${VERSION}"

echo "╔══════════════════════════════════════════════════════════════════════════╗"
echo "║  8b-is :: GOD OF MACHINE-S-UN~UNITY 360° INSTALLER (${VERSION}) ║"
echo "║  OS-Agnostic 64B Core × iOS/Android/SmartTV/Embedded/Desktop/WebPWA      ║"
echo "║  <DO :::<ULTRABACKYARDLOOP MAX 108laps <DO --- {...} ^^ <3               ║"
echo "╚══════════════════════════════════════════════════════════════════════════╝"

mkdir -p "$INSTALL_DIR" "$BIN_DIR"

fetch_asset() {
  local asset="$1"
  local dest="$2"
  echo "  • Downloading ${asset}..."
  curl -fsSL "${CODEBERG_RAW}/${asset}" -o "$dest" 2>/dev/null \
    || curl -fsSL "${GITHUB_RAW}/${asset}" -o "$dest" 2>/dev/null \
    || curl -fsSL "${GITHUB_REL}/${asset}" -o "$dest"
}

fetch_asset "8b-machine-sun-unity-360-universal-pwa.tar.gz" "$INSTALL_DIR/pwa.tar.gz"
tar -xzf "$INSTALL_DIR/pwa.tar.gz" -C "$INSTALL_DIR"

fetch_asset "8b-machine-sun-unity-360-ios-android-smarttv-embedded-sdk.tar.gz" "$INSTALL_DIR/sdk-360.tar.gz"
tar -xzf "$INSTALL_DIR/sdk-360.tar.gz" -C "$INSTALL_DIR"

OS_NAME="$(uname -s 2>/dev/null || echo Unknown)"
ARCH_NAME="$(uname -m 2>/dev/null || echo Unknown)"

if [ "$OS_NAME" = "Darwin" ] && [ "$ARCH_NAME" = "arm64" ]; then
  echo "  • Detected macOS Apple Silicon (arm64) — fetching native 8b-nektar-dj app..."
  if fetch_asset "8b-nektar-dj-macos-arm64-v2.0.108.tar.gz" "$INSTALL_DIR/nektar-dj.tar.gz"; then
    tar -xzf "$INSTALL_DIR/nektar-dj.tar.gz" -C "$INSTALL_DIR"
    chmod +x "$INSTALL_DIR/8b-nektar-dj" 2>/dev/null || true
    ln -sf "$INSTALL_DIR/8b-nektar-dj" "$BIN_DIR/8b-nektar-dj" 2>/dev/null || true
  fi
fi

cat > "$BIN_DIR/8b-machine-sun-360" <<EOF
#!/usr/bin/env bash
PLAYGROUND="$INSTALL_DIR/babilon-ultra-glassdoor-playground.html"
if command -v open >/dev/null 2>&1; then
  open "\$PLAYGROUND"
elif command -v xdg-open >/dev/null 2>&1; then
  xdg-open "\$PLAYGROUND"
else
  echo "Open in any browser: \$PLAYGROUND"
fi
EOF
chmod +x "$BIN_DIR/8b-machine-sun-360"

echo ""
echo "  ✔ Installed 360° Universal Glassdoor PWA & SDK to: $INSTALL_DIR"
echo "  ✔ Launcher installed to: $BIN_DIR/8b-machine-sun-360"
echo "  ✔ Layer #69,636 · 108/108 ULTRABACKYARDLOOP Laps Verified · {<3,<3,<3}+1"
