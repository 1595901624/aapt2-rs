use std::io;
use std::path::PathBuf;
use std::process::Command;
use crate::aapt2::get_exe_path;
use crate::model::manifest::Manifest;

struct AAPT2 {
    /// aapt2 path
    exe_path: PathBuf,
}

impl AAPT2 {
    fn new() -> Self {
        return Self {
            exe_path: get_exe_path()
        };
    }

    /// aapt2 dump badging
    /// Print information extracted from the manifest of the APK.
    pub(crate) fn dump_badging(&self, app_path: PathBuf) -> io::Result<()> {
        let status = Command::new(self.exe_path.as_os_str())
            .arg("dump")
            .arg("badging")
            .arg(app_path.as_os_str())
            .output()
            .expect("Failed to execute command");

        println!("status => {:#?}", status);
        println!("status.stdout => {:#?}", String::from_utf8_lossy(&status.stdout));
        println!("status.stderr => {:#?}", String::from_utf8_lossy(&status.stderr));

        return if status.status.success() {
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, String::from_utf8_lossy(&status.stderr)))
        };
    }

    /// aapt2 parse manifest
    pub(crate) fn parse_manifest(&self) -> io::Result<Manifest> {

        return Ok(Manifest::new());
    }
}

#[test]
fn aapt2_test() {
    let aapt2 = AAPT2::new();
    aapt2.dump_badging(PathBuf::from("C:\\Users\\luhao\\Desktop\\mm\\AIChat.apk"));
}