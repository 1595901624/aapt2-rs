use std::path::PathBuf;

#[cfg(target_os = "windows")]
pub mod win;

#[cfg(target_os = "linux")]
mod linux;
pub mod aapt2;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub fn get_exe_path() -> PathBuf {
    return win::get_exe_path();
}

#[cfg(target_os = "linux")]
pub fn get_exe_path() -> PathBuf {
    return linux::get_exe_path();
}

#[cfg(target_os = "macos")]
pub fn get_exe_path() -> PathBuf {
    return macos::get_exe_path();
}

// /// dump_badging
// fn dump_badging() {
//
// }