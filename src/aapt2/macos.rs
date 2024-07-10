use std::env;
use std::path::PathBuf;

/// exe 路径
/// windows: OUT_DIR/assets/macos/aapt2
pub(crate) fn get_exe_path() -> PathBuf {
    let out_dir = env::var("OUT_DIR").unwrap();
    // macos: OUT_DIR/assets/macos/aapt2
    return PathBuf::from(out_dir).join("assets").join("macos").join("aapt2");
}


#[test]
fn get_exe_path_test() {
    let path = get_exe_path();
    println!("path => {:?}", path);
}