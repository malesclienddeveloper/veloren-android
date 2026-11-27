//! Minimal native Android renderer untuk launcher Veloren.
//!
//! `ANativeActivity_onCreate` mendaftarkan callback `onNativeWindowCreated` /
//! `onNativeWindowResized` / `onNativeWindowRedrawNeeded`, lalu menggambar
//! **solid-color** ke `ANativeWindow`. Ini membuktikan pipeline native Rust ->
//! NativeActivity -> rendering hidup. Belum Veloren.
//!
//! Integrasi sebenarnya (winit + wgpu + voxygen + aset + input sentuh) menjadi
//! langkah berikutnya; lihat README.

#![allow(non_snake_case)]

pub mod launcher;

use std::mem::MaybeUninit;
use std::ptr::NonNull;

/// Jenis piksel solid untuk uji pipeline. 0xFF = penuh (lihat format buffer).
const FILL_BYTE: u8 = 0xFF;

/// Instance launcher global.
static LAUNCHER: std::sync::Mutex<crate::launcher::Launcher> =
    std::sync::Mutex::new(crate::launcher::Launcher::new());

/// Gambar warna solid penuh ke `ANativeWindow`.
unsafe fn render_solid(window: *mut ndk_sys::ANativeWindow) {
    // Solid white penuh (0xFF). Dipertahankan untuk fallback.
    let Some(nonnull) = NonNull::new(window) else { return };
    let native = ndk::native_window::NativeWindow::clone_from_ptr(nonnull);
    let mut guard = match native.lock(None) {
        Ok(g) => g,
        Err(_) => return,
    };
    if let Some(bytes) = guard.bytes() {
        for b in bytes.iter_mut() {
            b.write(FILL_BYTE);
        }
    }
}

/// Renderer launcher minimal (Soul Knight style sementara):
/// background NON-PUTIH + progress bar atas + aksen warna per `LauncherState`.
/// Belum `voxygen`, tidak bisa melihat hasil di sini.
unsafe fn render_launcher(window: *mut ndk_sys::ANativeWindow, launcher: &crate::launcher::Launcher) {
    let Some(nonnull) = NonNull::new(window) else { return };
    let native = ndk::native_window::NativeWindow::clone_from_ptr(nonnull);
    let mut guard = match native.lock(None) {
        Ok(g) => g,
        Err(_) => return,
    };

    let width = guard.width();
    let height = guard.height();
    let stride = guard.stride();
    let bpp = guard.format().bytes_per_pixel().unwrap_or(4);
    if width == 0 || height == 0 || stride == 0 || bpp == 0 {
        return;
    }

    // peta RGBA piksel (asumsi urutan RGBA; format Android biasa RGBA8888/RGBX8888)
    let bg = state_bg(launcher.state); // background per state, non-putih
    let accent = state_accent(launcher.state); // aksen bar
    let bar_h = (height / 24).clamp(8, 64);
    let pct = launcher.progress.percent().min(100) as usize;
    let bar_w = (width * pct) / 100;

    if let Some(bytes) = guard.bytes() {
        // isi background
        for b in bytes.iter_mut() {
            b.write(0);
        }
        let stride_b = stride * bpp;
        // helper tulis piksel (x,y,warna)
        let mut put = |buf: &mut [MaybeUninit<u8>], x: usize, y: usize, c: [u8; 4]| {
            let idx = y * stride_b + x * bpp;
            if idx + bpp <= buf.len() {
                buf[idx].write(c[0]);
                if bpp > 1 { buf[idx + 1].write(c[1]); }
                if bpp > 2 { buf[idx + 2].write(c[2]); }
                if bpp > 3 { buf[idx + 3].write(c[3]); }
            }
        };
        // background penuh (per-pixel untuk hormati stride)
        for y in 0..height {
            for x in 0..width {
                put(bytes, x, y, bg);
            }
        }
        // track bar (garis atas) latar gelap
        for y in 0..bar_h {
            for x in 0..width {
                put(bytes, x, y, [24, 24, 32, 255]);
            }
        }
        // progress bar
        for y in 0..bar_h {
            for x in 0..bar_w {
                put(bytes, x, y, accent);
            }
        }
    }
    // drop(guard) -> ANativeWindow_unlockAndPost()
}

fn state_bg(s: crate::launcher::LauncherState) -> [u8; 4] {
    use crate::launcher::LauncherState as S;
    match s {
        S::OpenNormal => [20, 24, 48, 255],
        S::CheckUpdate => [24, 28, 56, 255],
        S::UpToDate => [16, 40, 32, 255],
        S::UpdateAvailable => [48, 36, 16, 255],
        S::DownloadAssets => [16, 32, 56, 255],
        S::Download100 => [16, 48, 40, 255],
        S::VerifyingAssets => [32, 32, 48, 255],
        S::LoadAssets => [16, 40, 48, 255],
        S::RunVoxygen => [16, 24, 24, 255],
        S::Error => [56, 16, 24, 255],
    }
}

fn state_accent(s: crate::launcher::LauncherState) -> [u8; 4] {
    use crate::launcher::LauncherState as S;
    match s {
        S::UpdateAvailable => [240, 180, 60, 255],
        S::DownloadAssets => [80, 170, 255, 255],
        S::Download100 => [90, 220, 160, 255],
        S::VerifyingAssets => [200, 200, 220, 255],
        S::Error => [240, 80, 90, 255],
        _ => [120, 200, 255, 255],
    }
}

unsafe extern "C" fn on_native_window_created(
    _activity: *mut ndk_sys::ANativeActivity,
    window: *mut ndk_sys::ANativeWindow,
) {
    render_with_launcher(window);
}

unsafe extern "C" fn on_native_window_resized(
    _activity: *mut ndk_sys::ANativeActivity,
    window: *mut ndk_sys::ANativeWindow,
) {
    render_with_launcher(window);
}

unsafe extern "C" fn on_native_window_redraw_needed(
    _activity: *mut ndk_sys::ANativeActivity,
    window: *mut ndk_sys::ANativeWindow,
) {
    render_with_launcher(window);
}

/// Render launcher; kalau lock gagal, fallback render_solid.
unsafe fn render_with_launcher(window: *mut ndk_sys::ANativeWindow) {
    match LAUNCHER.lock() {
        Ok(launcher) => render_launcher(window, &launcher),
        Err(_) => render_solid(window),
    }
}

/// Tanda tangan `ANativeActivity_onCreate` yang diharapkan `NativeActivity`.
#[no_mangle]
pub extern "C" fn ANativeActivity_onCreate(
    activity: *mut ndk_sys::ANativeActivity,
    _savedInstanceState: *mut std::ffi::c_void,
    _savedInstanceStateSize: usize,
) {
    if activity.is_null() {
        return;
    }
    unsafe {
        if let Some(activity_ref) = activity.as_mut() {
            if let Some(callbacks) = activity_ref.callbacks.as_mut() {
                callbacks.onNativeWindowCreated = Some(on_native_window_created);
                callbacks.onNativeWindowResized = Some(on_native_window_resized);
                callbacks.onNativeWindowRedrawNeeded = Some(on_native_window_redraw_needed);
            }
        }
    }
}
