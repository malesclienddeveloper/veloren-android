#!/usr/bin/env bash
#
# prepare-assets.sh — salin aset game Veloren ke dalam APK Android + VERIFIKASI 100%.
# Jalankan SEBELUM ./gradlew assembleDebug. Butuh git-lfs agar aset biner bukan penunjuk LFS.
#   android/prepare-assets.sh            # pakai <repo>/assets
#   android/prepare-assets.sh /path/assets
#
# Gagal (exit !=0) kalau ada file yang tidak tersalin atau penunjuk LFS.
# (Tidak menuntut "tidak ada file kosong": upstream boleh punya file 0-byte.)
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

# --- verifikasi 100% (tanpa pipeline `head`, hindari SIGPIPE+pipefail) ------
SRC_COUNT="$(find "$SRC_ASSETS" -type f | wc -l | tr -d ' ')"
DST_COUNT="$(find "$DEST" -type f | wc -l | tr -d ' ')"
echo "[prepare-assets] source=$SRC_COUNT files -> dest=$DST_COUNT files"
if [ "$SRC_COUNT" != "$DST_COUNT" ]; then
    echo "ERROR: jumlah file beda! source=$SRC_COUNT dest=$DST_COUNT" >&2
    exit 1
fi

# penunjuk Git LFS tidak boleh tersisa (aset biner harus asli, jalankan `git lfs pull`)
LFS_PTR="$(mktemp)"
grep -RIl --exclude-dir=.git "version https://git-lfs.github.com/spec/v1" "$DEST" > "$LFS_PTR" || true
if [ -s "$LFS_PTR" ]; then
    echo "ERROR: ditemukan penunjuk Git LFS di $DEST. Jalankan 'git lfs pull' lalu ulangi." >&2
    rm -f "$LFS_PTR"
    exit 1
fi
rm -f "$LFS_PTR"

echo "[prepare-assets] OK 100%: $DST_COUNT file diketemukan, tanpa penunjuk LFS -> $DEST"
