//! Jembatan native minimal untuk launcher Android Veloren.
//!
//! FILE INI ADALAH STUB. Diekspor sebagai `libveloren_android.so` dan dipanggil
//! oleh `NativeActivity` melalui `ANativeActivity_onCreate`.
//!
//! Integrasi sebenarnya dengan voxygen (loop event winit pada Android, grafis
//! wgpu, audio kira/cpal, tokio, direktori userdata Android, input sentuh) BELUM
//! diimplementasikan. Ganti isi `ANativeActivity_onCreate` di bawah dengan
//! inisialisasi game yang benar.

#![allow(non_snake_case)]
#![allow(dead_code)]

use std::ffi::CString;
use std::ptr;

/// Representasi minimum dari ANativeActivity (cukup untuk tanda tangan fungsi;
/// definisi lengkap ada di header NDK `native_activity.h`).
#[repr(C)]
pub struct ANativeActivity {
    _unused: [u8; 0],
}

/// Tanda tangan `ANativeActivity_onCreate` yang diharapkan NativeActivity.
///
/// `savedInstanceState` boleh nol jika aktivitas baru dibuat.
#[no_mangle]
pub extern "C" fn ANativeActivity_onCreate(
    _activity: *mut ANativeActivity,
    _savedInstanceState: *mut std::ffi::c_void,
    _savedInstanceStateSize: usize,
) {
    let _tag = CString::new("VelorenAndroid").expect("CString gagal dibuat");

    // TODO: ganti stub ini dengan inisialisasi voxygen sebenarnya:
    //   1. Ambil ANativeWindow / AInputQueue dari activity (via ANativeActivity callbacks).
    //   2. Buat winit EventLoop untuk Android dan Window.
    //   3. Panggil logika run() voxygen dengan settings yang cocok Android.
    //   4. Atur direktori userdata ke konteks internal Android.
    let _ = ptr::null::<()>();
}
