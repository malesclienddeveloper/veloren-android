# Veloren — Native Android Launcher (via GitHub Actions)

> **Status: kerangka awal. GitHub Actions membangun APK otomatis saat push.**
> Veloren tidak mendukung Android secara resmi (hanya Windows/macOS/Linux).
> Berkas-berkas ini adalah dasar proyek launcher Android (Gradle + `NativeActivity` +
> plumbing Rust untuk target `aarch64-linux-android`). Build sebenarnya dilakukan
> oleh **GitHub Actions**, bukan di lokal.

## Alur kerja (GitHub 100%)

1. Kerja pada repositori **GitLab** (`git@gitlab.com:veloren/veloren.git`), karena itu adalah sumber kebenaran.
2. Push perubahan ke GitLab.
3. GitHub Action `mirror.yml` menyalin repositori ke GitHub setiap jam (sudah ada).
4. GitHub Action `android-build.yml` terpicu otomatis, mengkompilasi:
   - library native Rust `libveloren_android.so` (target `aarch64-linux-android`),
   - APK debug melalui Gradle.
5. Unduh APK hasil build dari tab **Actions → artifacts** di GitHub.

Tidak perlu memasang Android NDK, Rust toolchain Android, atau Gradle di komputer lokal.

## Struktur

```
android/
  README.md            # ini
  cargo-build.sh       # kompilasi Rust cdylib (digunakan oleh CI)
  rust/
    Cargo.toml         # cdylib veloren_android (stub ANativeActivity_onCreate)
    src/lib.rs         # jembatan native (TODO: koneksi ke voxygen)
  app/
    build.gradle
    src/main/AndroidManifest.xml
    src/main/java/net/veloren/veloren/VelorenActivity.java
    src/main/jniLibs/arm64-v8a/.gitkeep   # .so ditempelkan oleh CI
    src/main/assets/.gitkeep               # aset game ditempelkan oleh CI
.github/workflows/android-build.yml  # CI: build APK otomatis
```

## Masih harus dilakukan (porting sebenarnya)

- `rust/src/lib.rs` saat ini adalah **stub** yang hanya mengekspor
  `ANativeActivity_onCreate` dan mencatat log. Integrasi nyata dengan
  `voxygen` (loop event winit pada Android, grafis `wgpu`, audio `kira`/cpal,
  tokio, direktori userdata Android, input sentuh/kontroler) harus
  diimplementasikan.
- `voxygen/src/main.rs` mengurai argumen CLI (`clap`) dan membuat jendela sendiri;
  pada Android tidak ada CLI, sehingga perlu pintu masuk Android terpisah.

## Catatan

Tidak ada berkas yang mengubah sumber `voxygen` atau crate desktop lain, sehingga
build desktop yang ada tidak terpengaruh. Perubahan pada root repo hanya
konfigurasi linker Android di `.cargo/config.toml` dan pengecualian
`android/rust` dari workspace Cargo.

## Push & build (ringkas)

```bash
# di cloned repo GitLab
git add android/ .github/workflows/android-build.yml .cargo/config.toml Cargo.toml
git commit -m "feat: add native Android launcher skeleton + GitHub Actions build"
git push origin main   # GitLab → mirror otomatis ke GitHub → GitHub Actions build APK
```

Lihat hasil build di: `https://github.com/<user>/veloren/actions` (tab Actions → workflow "Build Android APK" → artifact `veloren-android-apk`).
