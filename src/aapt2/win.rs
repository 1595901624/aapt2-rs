use std::env;
use std::path::PathBuf;

/// exe 路径
/// windows: OUT_DIR/assets/win/aapt2.exe
pub(crate) fn get_exe_path() -> PathBuf {
    let out_dir = env::var("OUT_DIR").unwrap();
    // linux: OUT_DIR/assets/linux/aapt2
    return PathBuf::from(out_dir).join("assets").join("win").join("aapt2.exe");
}


#[test]
fn get_exe_path_test() {
    let path = get_exe_path();
    println!("path => {:?}", path);
}