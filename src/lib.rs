mod aapt2;
mod model;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::path::PathBuf;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
        let out_dir = env::var("OUT_DIR").unwrap();
        // windows: OUT_DIR/assets/win/aapt2-x86_64-pc-windows-msvc.exe
        // linux: OUT_DIR/assets/linux/aapt2
        let aapt2_path = PathBuf::from(out_dir).join("assets").join("win").join("aapt2-x86_64-pc-windows-msvc.exe");
        println!("aapt2_path => {:?}", aapt2_path);
    }
}
