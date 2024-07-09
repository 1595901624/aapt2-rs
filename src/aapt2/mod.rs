use std::path::PathBuf;

#[cfg(target_os = "windows")]
mod win;

#[cfg(target_os = "linux")]
mod linux;
mod aapt2;

#[cfg(target_os = "windows")]
fn get_exe_path() -> PathBuf {
    return win::get_exe_path();
}

#[cfg(target_os = "linux")]
fn get_exe_path() -> PathBuf {
    return linux::get_exe_path();
}

// /// dump_badging
// fn dump_badging() {
//
// }