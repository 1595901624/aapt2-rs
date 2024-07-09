use std::env;
use std::path::PathBuf;

/// exe 路径
/// windows: OUT_DIR/assets/linux/aapt2
pub(crate) fn get_exe_path() -> PathBuf {
    let out_dir = env::var("OUT_DIR").unwrap();
    // linux: OUT_DIR/assets/linux/aapt2
    return PathBuf::from(out_dir).join("assets").join("linux").join("aapt");
}

#[cfg(test)]
mod tests {
    use crate::aapt2::win::get_exe_path;

    #[test]
    fn get_exe_path_test() {
        let path = get_exe_path();
        println!("path => {:?}", path);
    }
}