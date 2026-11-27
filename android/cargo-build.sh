#!/usr/bin/env bash
#
# cargo-build.sh — kompilasi cdylib Rust untuk Android (arm64-v8a) dan salin
# hasilnya ke app/src/main/jniLibs/arm64-v8a/ agar Gradle dapat memaketkannya ke APK.
#
# Prerequisites:
#   - rustup target aarch64-linux-android terpasang
#   - ANDROID_NDK_HOME mengarah ke NDK
#   - aarch64-linux-android-clang tersedia di $PATH (biasanya dari
#     $ANDROID_NDK_HOME/toolchains/llvm/prebuilt/<host>/bin)
#
# Contoh penggunaan:
#   cd /root/veloren/android
#   ANDROID_NDK_HOME=/opt/android-ndk ./cargo-build.sh
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
RUST_DIR="$ROOT_DIR/android/rust"
APK_JNI_DIR="$ROOT_DIR/android/app/src/main/jniLibs/arm64-v8a"

# GitHub Action android-actions/setup-android sets ANDROID_NDK_ROOT;
# local/manual use typically sets ANDROID_NDK_HOME. Accept either.
if [ -z "${ANDROID_NDK_HOME:-}" ]; then
    export ANDROID_NDK_HOME="${ANDROID_NDK_ROOT:-}"
fi
if [ -z "${ANDROID_NDK_HOME:-}" ]; then
    echo "ERROR: ANDROID_NDK_HOME (atau ANDROID_NDK_ROOT) belum diatur." >&2
    exit 1
fi

# Pastikan linker clang dari NDK ada di PATH agar .cargo/config.toml dapat
# menemukannya (biasanya difilter melalui CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER).
NDK_CLANG_DIR="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt"
if [ -d "$NDK_CLANG_DIR" ]; then
    HOST_DIR="$(ls -1 "$NDK_CLANG_DIR" | head -n1)"
    export PATH="$NDK_CLANG_DIR/$HOST_DIR/bin:$PATH"
fi

echo "[cargo-build] membersihkan jniLibas lama..."
rm -rf "$APK_JNI_DIR"
mkdir -p "$APK_JNI_DIR"

echo "[cargo-build] kompilasi veloren_android (cdylib) untuk aarch64-linux-android..."
cargo build \
    --manifest-path "$RUST_DIR/Cargo.toml" \
    --target aarch64-linux-android \
    --release

SO_FILE="$RUST_DIR/target/aarch64-linux-android/release/libveloren_android.so"
if [ ! -f "$SO_FILE" ]; then
    echo "ERROR: $SO_FILE tidak ditemukan setelah build." >&2
    exit 1
fi

cp "$SO_FILE" "$APK_JNI_DIR/libveloren_android.so"
echo "[cargo-build] selesai: $APK_JNI_DIR/libveloren_android.so"
