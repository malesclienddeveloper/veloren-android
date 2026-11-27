#!/usr/bin/env bash
#
# prepare-assets.sh — salin aset game Veloren ke dalam APK Android.
# Jalankan SEBELUM ./gradlew assembleDebug. Butuh git-lfs agar aset biner bukan penunjuk LFS.
#   android/prepare-assets.sh            # pakai <repo>/assets
#   android/prepare-assets.sh /path/assets
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
SRC_ASSETS="${1:-$REPO_ROOT/assets}"
DEST="$SCRIPT_DIR/app/src/main/assets/veloren-assets"

if [ ! -d "$SRC_ASSETS" ]; then
    echo "ERROR: '$SRC_ASSETS' tidak ada. Jalankan di root repositori Veloren lengkap (bukan repo android-only)." >&2
    exit 1
fi

rm -rf "$DEST"
mkdir -p "$DEST"
cp -a "$SRC_ASSETS/." "$DEST/"
echo "[prepare-assets] $SRC_ASSETS -> $DEST"
