use std::collections::HashMap;
use std::io;
use std::panic::Location;
use std::path::PathBuf;
use std::process::Command;

use crate::aapt2::get_exe_path;
use crate::model::locale::Locale;
use crate::model::manifest::Manifest;
use crate::model::package::Package;

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
    pub(crate) fn dump_badging(&self, app_path: PathBuf) -> io::Result<Manifest> {
        let status = Command::new(self.exe_path.as_os_str())
            .arg("dump")
            .arg("badging")
            .arg(app_path.as_os_str())
            .output()
            .expect("Failed to execute command");

        // println!("status => {:#?}", status);
        // println!("status.stdout => {:#?}", String::from_utf8_lossy(&status.stdout));
        // println!("status.stderr => {:#?}", String::from_utf8_lossy(&status.stderr));

        return if status.status.success() {
            let manifest = self.parse_manifest(String::from_utf8_lossy(&status.stdout).to_string())?;
            Ok(manifest)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, String::from_utf8_lossy(&status.stderr)))
        };
    }

    /// aapt2 parse manifest
    pub(crate) fn parse_manifest(&self, input: String) -> io::Result<Manifest> {
        let mut name = String::new();
        let mut version_code = String::new();
        let mut version_name = String::new();
        let mut platform_build_version_name = String::new();
        let mut platform_build_version_code = String::new();
        let mut compile_sdk_version = String::new();
        let mut compile_sdk_version_codename = String::new();
        let mut sdk_version = String::new();
        let mut target_sdk_version = String::new();
        let mut uses_permissions = Vec::new();
        let mut application_labels = HashMap::new();
        let mut application_icons = HashMap::new();
        // let mut application_label = String::new();
        // let mut application_icon = String::new();
        // let mut launchable_activity_name = String::new();
        // let mut launchable_activity_label = String::new();
        // let mut launchable_activity_icon = String::new();
        // let mut additional_permissions = Vec::new();
        // let mut feature_group_label = String::new();
        // let mut features = Vec::new();
        // let mut native_code = String::new();

        for line in input.lines() {
            let line = line.trim();
            if line.starts_with("package:") {
                for part in line.split_whitespace().skip(1) {
                    let (key, value) = part.split_at(part.find('=').unwrap());
                    let value = value.trim_matches('=').trim_matches('\'');
                    match key {
                        "name" => name = value.to_string(),
                        "versionCode" => version_code = value.to_string(),
                        "versionName" => version_name = value.to_string(),
                        "platformBuildVersionName" => platform_build_version_name = value.to_string(),
                        "platformBuildVersionCode" => platform_build_version_code = value.to_string(),
                        "compileSdkVersion" => compile_sdk_version = value.to_string(),
                        "compileSdkVersionCodename" => compile_sdk_version_codename = value.to_string(),
                        _ => {}
                    }
                }
            } else if line.starts_with("application-label:") {
                let label = line.split(':').nth(1).unwrap().trim_matches('\'').to_string();
                application_labels.insert(Locale::DEFAULT, label);
            } else if line.starts_with("application-label-") {
                let parts: Vec<&str> = line.split(':').collect();
                let lang = parts[0].split('-').nth(2).unwrap().to_string();
                let label = parts[1].trim_matches('\'').to_string();
                application_labels.insert(Locale::from(lang), label);
            } else if line.starts_with("sdkVersion:") {
                sdk_version = line.split(':').nth(1).unwrap().trim_matches('\'').to_string();
            } else if line.starts_with("targetSdkVersion:") {
                target_sdk_version = line.split(':').nth(1).unwrap().trim_matches('\'').to_string();
            } else if line.starts_with("uses-permission:") {
                let permission = line.split("name='").nth(1).unwrap().trim_matches('\'').to_string();
                uses_permissions.push(permission);
            } else {}
        }

        let package = Package {
            name,
            version_name,
            version_code: version_code.parse::<u32>().unwrap(),
            platform_build_version_name,
            platform_build_version_code: platform_build_version_code.parse::<u32>().unwrap(),
            compile_sdk_version,
            compile_sdk_version_codename,
        };

        let manifest = Manifest {
            package,
            sdk_version,
            target_sdk_version,
            uses_permissions,
            application_labels,
        };

        return Ok(manifest);
    }
}

#[test]
fn aapt2_test() {
    let aapt2 = AAPT2::new();
    let manifest = aapt2.dump_badging(PathBuf::from("C:\\Users\\luhao\\Desktop\\mm\\AIChat.apk")).expect("");
    println!("manifest => {:#?}", manifest);
}