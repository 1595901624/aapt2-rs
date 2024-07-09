use std::{env, fs};
use std::path::{Path, PathBuf};

use walkdir::WalkDir;
fn main() {
    println!("cargo:warning====start build====");
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("assets");

    let src_path = PathBuf::from("assets");
    println!("cargo:warning====copy_dir_all===={:?}", src_path);
    let result = copy_dir_all(&src_path, &dest_path);

    println!("cargo:warning==result => {:#?}", result);
}

fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    for entry in WalkDir::new(src) {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = dst.join(src_path.strip_prefix(src)?);
        if src_path.is_dir() {
            fs::create_dir_all(&dest_path)?;
        } else {
            fs::copy(&src_path, &dest_path)?;
        }
    }
    Ok(())
}