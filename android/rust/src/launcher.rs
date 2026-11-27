//! State machine launcher untuk Veloren Android.
//!
//! Logika yang diminta:
//!   OpenNormal -> CheckUpdate -> (UpdateAvailable? DownloadAssets -> Download100%)
//!     -> VerifyingAssets -> LoadAssets -> RunVoxygen
//!
//! Catatan jujur: `check_update`, `download_assets`, `load_assets`, `run_voxygen`
//! adalah **stub/TODO**. Tidak ada server update, tidak ada `voxygen`, tidak ada
//! rendering game. `verify_assets` adalah NYATA (jumlah + tidak ada penunjuk LFS).

/// Fase launcher. Gaya "Soul Knight prequel": buka -> cek update -> download
/// aset -> setelah 100% baru load aset -> masuk game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LauncherState {
    /// Buka normal (splash).
    OpenNormal,
    /// Cek update baru (query manifest server).
    CheckUpdate,
    /// Tidak ada update; aset lokal dipakai.
    UpToDate,
    /// Ada update; akan download aset baru.
    UpdateAvailable,
    /// Sedang download aset (progress 0..100).
    DownloadAssets,
    /// Download selesai 100%.
    Download100,
    /// Verifikasi aset (jumlah + bukan penunjuk LFS + checksum TODO).
    VerifyingAssets,
    /// Load aset (AAssetManager / writable + userdata + voxygen asset-root).
    LoadAssets,
    /// Masuk game `voxygen`.
    RunVoxygen,
    /// Error; mungkin retry / offline.
    Error,
}

/// Progress download aset.
#[derive(Debug, Clone, Copy, Default)]
pub struct DownloadProgress {
    pub downloaded: u64,
    pub total: u64,
}

impl DownloadProgress {
    /// 0..100. total=0 -> 0.
    pub fn percent(&self) -> u32 {
        if self.total == 0 {
            0
        } else {
            ((self.downloaded.min(self.total) as u128 * 100) / self.total as u128) as u32
        }
    }
    /// Benar-benar 100% (downloaded >= total && total > 0).
    pub fn is_100_percent(&self) -> bool {
        self.total > 0 && self.downloaded >= self.total
    }
}

/// Hasil verifikasi aset. `checksum_ok` TODO (perlu manifest checksum).
#[derive(Debug, Clone, Copy, Default)]
pub struct AssetVerification {
    pub source_count: usize,
    pub dest_count: usize,
    pub no_lfs_pointer: bool,
    pub checksum_ok: bool, // TODO: verifikasi SHA-256 dari manifest
}

impl AssetVerification {
    /// Parity `prepare-assets.sh` + (nanti) checksum.
    pub fn packaging_100_percent(&self) -> bool {
        self.source_count == self.dest_count && self.no_lfs_pointer
    }
    pub fn fully_100_percent(&self) -> bool {
        self.packaging_100_percent() && self.checksum_ok
    }
}

/// Hasil cek update (stub).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateCheck {
    UpToDate,
    UpdateAvailable,
    Unknown,
}

pub struct Launcher {
    pub state: LauncherState,
    pub progress: DownloadProgress,
    pub verification: AssetVerification,
}

impl Launcher {
    pub const fn new() -> Self {
        Self {
            state: LauncherState::OpenNormal,
            // tidak pakai Default::default() (non-const) di const fn
            progress: DownloadProgress { downloaded: 0, total: 0 },
            verification: AssetVerification {
                source_count: 0,
                dest_count: 0,
                no_lfs_pointer: false,
                checksum_ok: false,
            },
        }
    }

    /// Tentukan transisi berikutnya. `update` hasil `check_update`.
    pub fn advance(&mut self, update: UpdateCheck, verification: AssetVerification) -> LauncherState {
        self.state = match self.state {
            LauncherState::OpenNormal => LauncherState::CheckUpdate,
            LauncherState::CheckUpdate => match update {
                UpdateCheck::UpToDate | UpdateCheck::Unknown => LauncherState::UpToDate,
                UpdateCheck::UpdateAvailable => LauncherState::UpdateAvailable,
            },
            LauncherState::UpToDate => LauncherState::LoadAssets,
            LauncherState::UpdateAvailable => LauncherState::DownloadAssets,
            LauncherState::DownloadAssets => {
                // baru lanjut kalau download 100%
                if self.progress.is_100_percent() {
                    LauncherState::Download100
                } else {
                    LauncherState::DownloadAssets
                }
            },
            LauncherState::Download100 => LauncherState::VerifyingAssets,
            LauncherState::VerifyingAssets => {
                self.verification = verification;
                // "setelah download 100% aset baru terload" + packaging/verify 100%
                if verification.packaging_100_percent() {
                    LauncherState::LoadAssets
                } else {
                    LauncherState::Error
                }
            },
            LauncherState::LoadAssets => LauncherState::RunVoxygen,
            LauncherState::RunVoxygen => LauncherState::RunVoxygen,
            LauncherState::Error => LauncherState::Error,
        };
        self.state
    }
}

// --- operasi nyata vs stub --------------------------------------------------

/// STUB. Cek update baru. TODO: `reqwest` GET manifest server + versi + checksum + URL paket.
pub fn check_update() -> UpdateCheck {
    UpdateCheck::Unknown
}

/// STUB. Download aset. TODO: download paket, tulis ke disk, progress callback, retry.
pub fn download_assets(_progress_out: &mut DownloadProgress) {}

/// NYATA. Verifikasi aset di `dest` terhadap `source_count` + scan penunjuk LFS.
/// TODO tambahan: checksum SHA-256 per file vs manifest.
pub fn verify_assets(dest_path: &str, source_count: usize) -> AssetVerification {
    let dest_count = count_files(dest_path);
    AssetVerification {
        source_count,
        dest_count,
        no_lfs_pointer: !has_lfs_pointer(dest_path),
        checksum_ok: false, // TODO
    }
}

fn count_files(dir: &str) -> usize {
    let mut n = 0usize;
    if let Ok(rd) = std::fs::read_dir(dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                n += count_files(p.to_str().unwrap_or(""));
            } else {
                n += 1;
            }
        }
    }
    n
}

/// Scan rekursif penunjuk Git LFS `version https://git-lfs.github.com/spec/v1`.
fn has_lfs_pointer(dir: &str) -> bool {
    const MARK: &[u8] = b"version https://git-lfs.github.com/spec/v1";
    let mut stack = vec![dir.to_string()];
    while let Some(d) = stack.pop() {
        if let Ok(rd) = std::fs::read_dir(&d) {
            for e in rd.flatten() {
                let p = e.path();
                if p.is_dir() {
                    stack.push(p.to_string_lossy().into_owned());
                } else if let Ok(bytes) = std::fs::read(&p) {
                    if bytes.windows(MARK.len()).any(|w| w == MARK) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// STUB. Load aset. TODO: `AAssetManager`/copy-to-writable + `userdata` + voxygen asset-root.
pub fn load_assets() {}

/// STUB. Masuk game `voxygen`. TODO: winit Android + wgpu + kira/cpal + tokio + `voxygen::run`.
pub fn run_voxygen() {}

impl Default for Launcher {
    fn default() -> Self {
        Self::new()
    }
}
