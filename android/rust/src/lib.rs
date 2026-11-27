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

use std::ptr::NonNull;

/// Jenis piksel solid untuk uji pipeline. 0xFF = penuh (lihat format buffer).
const FILL_BYTE: u8 = 0xFF;

/// Gambar warna solid penuh ke `ANativeWindow`.
unsafe fn render_solid(window: *mut ndk_sys::ANativeWindow) {
    let Some(nonnull) = NonNull::new(window) else { return };
    // clone_from_ptr menaikkan ref-count; Drop akan menguranginya.
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
    // drop(guard) -> ANativeWindow_unlockAndPost(); native drop terakhir
}

unsafe extern "C" fn on_native_window_created(
    _activity: *mut ndk_sys::ANativeActivity,
    window: *mut ndk_sys::ANativeWindow,
) {
    render_solid(window);
}

unsafe extern "C" fn on_native_window_resized(
    _activity: *mut ndk_sys::ANativeActivity,
    window: *mut ndk_sys::ANativeWindow,
) {
    render_solid(window);
}

unsafe extern "C" fn on_native_window_redraw_needed(
    _activity: *mut ndk_sys::ANativeActivity,
    window: *mut ndk_sys::ANativeWindow,
) {
    render_solid(window);
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
