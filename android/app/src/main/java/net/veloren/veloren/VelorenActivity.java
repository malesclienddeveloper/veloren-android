package net.veloren.veloren;

import android.app.NativeActivity;

/**
 * Titik masuk aktivitas Android native.
 *
 * {@link NativeActivity} memuat library native yang dinyatakan melalui metadata
 * {@code android.app.lib_name} di {@code AndroidManifest.xml} (di sini
 * {@code veloren_android}). Library itu harus mengekspor fungsi
 * {@code ANativeActivity_onCreate}.
 *
 * Saat ini Rust di {@code android/rust/src/lib.rs} masih merupakan stub. Integrasi
 * penuh dengan loop event dan grafis voxygen (winit pada Android) harus
 * diimplementasikan di sana.
 */
public class VelorenActivity extends NativeActivity {
    static {
        System.loadLibrary("veloren_android");
    }
}
