use std::collections::{HashMap, HashSet};
use std::io;
use std::path::PathBuf;
use std::process::Command;

use regex::Regex;

use crate::aapt2::get_exe_path;
use crate::model::application::Application;
use crate::model::density::Density;
use crate::model::locale::Locale;
use crate::model::lunchable_activity::LaunchableActivity;
use crate::model::manifest::Manifest;
use crate::model::package::Package;

pub struct AAPT2 {
    /// aapt2 path
    exe_path: PathBuf,
}

impl AAPT2 {
    pub(crate) fn new() -> Self {
        return Self {
            exe_path: get_exe_path()
        };
    }

    pub fn from(exe_path: PathBuf) -> Self {
        return Self {
            exe_path
        };
    }

    /// Prints the version of aapt.
    pub fn version(&self) -> io::Result<String> {
        let mut command = Command::new(self.exe_path.as_os_str());
        let mut status = command.arg("version");
        if cfg!(target_os = "windows") {
            use std::os::windows::process::CommandExt;
            status = status.creation_flags(0x08000000);
        }
        let status = status.output()?;

        return if status.status.success() {
            Ok(String::from_utf8_lossy(&status.stderr).trim().to_string())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, String::from_utf8_lossy(&status.stderr)))
        };
    }

    /// aapt2 dump badging
    /// Print information extracted from the manifest of the APK.
    pub fn dump_badging(&self, app_path: PathBuf) -> io::Result<Manifest> {
        let mut command = Command::new(self.exe_path.as_os_str());
        let mut status = command.arg("dump")
            .arg("badging")
            .arg(app_path.as_os_str());

        if cfg!(target_os = "windows") {
            use std::os::windows::process::CommandExt;
            status = status.creation_flags(0x08000000);
        }
        let status = status.output()?;

        return if status.status.success() {
            let manifest = self.parse_manifest(String::from_utf8_lossy(&status.stdout).to_string())?;
            Ok(manifest)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, String::from_utf8_lossy(&status.stderr)))
        };
    }

    /// aapt2 dump packagename
    pub fn dump_packagename(&self, app_path: PathBuf) -> io::Result<String> {
        let mut command = Command::new(self.exe_path.as_os_str());
        let mut status = command.arg("dump")
            .arg("packagename")
            .arg(app_path.as_os_str());

        if cfg!(target_os = "windows") {
            use std::os::windows::process::CommandExt;
            status = status.creation_flags(0x08000000);
        }
        let status = status.output()?;

        return if status.status.success() {
            Ok(String::from_utf8_lossy(&status.stdout).trim().to_string())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, String::from_utf8_lossy(&status.stderr)))
        };
    }

    /// aapt2 parse manifest
    fn parse_manifest(&self, input: String) -> io::Result<Manifest> {
        let mut name = String::new();
        let mut version_code = String::new();
        let mut version_name = String::new();
        let mut platform_build_version_name = String::new();
        let mut platform_build_version_code = String::new();
        let mut compile_sdk_version = String::new();
        let mut compile_sdk_version_codename = String::new();
        let mut sdk_version = String::new();
        let mut target_sdk_version = String::new();
        let mut uses_permissions = HashSet::new();
        let mut application_labels = HashMap::new();
        let mut application_icons = HashMap::new();
        let mut application = Application::default();
        let mut launchable_activity = LaunchableActivity::default();
        // let mut additional_permissions = Vec::new();
        // let mut feature_group_label = String::new();
        // let mut features = Vec::new();
        let mut native_code = String::new();

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
                uses_permissions.insert(permission);
            } else if line.starts_with("application-icon-") {
                let parts: Vec<&str> = line.split(':').collect();
                let density = parts[0].split('-').nth(2).unwrap().to_string();
                let icon = parts[1].trim_matches('\'').to_string();
                application_icons.insert(Density::from(density), icon);
            } else if line.starts_with("application:") {
                // for part in line.split_whitespace().skip(1) {
                //     let (key, value) = part.split_at(part.find('=').unwrap());
                //     let value = value.trim_matches('=').trim_matches('\'');
                //     match key {
                //         "label" => application.label = value.to_string(),
                //         "icon" => application.icon = value.to_string(),
                //         _ => {}
                //     }
                // }
                let re = Regex::new(r"(\w+)='([^']*)'").unwrap();
                for cap in re.captures_iter(line) {
                    // map.insert(cap[1].to_string(), cap[2].to_string());
                    match cap[1].to_string().as_str() {
                        "label" => application.label = cap[2].to_string(),
                        "icon" => application.icon = cap[2].to_string(),
                        _ => {}
                    }
                }
            } else if line.starts_with("launchable-activity:") {
                for part in line.split_whitespace().skip(1) {
                    let (key, value) = part.split_at(part.find('=').unwrap());
                    let value = value.trim_matches('=').trim_matches('\'');
                    match key {
                        "name" => launchable_activity.name = value.to_string(),
                        "label" => launchable_activity.label = value.to_string(),
                        "icon" => launchable_activity.icon = value.to_string(),
                        _ => {}
                    }
                }
                // } else if line.starts_with("uses-implied-feature:") {
                //     let parts: Vec<&str> = line.split("name='").collect();
                //     let feature_name = parts[1].split('\'').next().unwrap().to_string();
                //     let reason = parts[1].split("reason='").nth(1).unwrap().trim_matches('\'').to_string();
                //     if let Some(feature) = features.iter_mut().find(|f| f.name == feature_name) {
                //         feature.implied_reason = Some(reason);
                //     } else {
                //         features.push(Feature { name: feature_name, required: true, implied_reason: Some(reason) });
                //     }
                // } else if line.starts_with("feature-group:") {
                //     feature_group_label = line.split(':').nth(1).unwrap().trim_matches('\'').to_string();
                // } else if line.starts_with("uses-feature-not-required:") {
                //     let feature_name = line.split("name='").nth(1).unwrap().trim_matches('\'').to_string();
                //     features.push(Feature { name: feature_name, required: false, implied_reason: None });
                // } else if line.starts_with("uses-feature:") {
                //     let feature_name = line.split("name='").nth(1).unwrap().trim_matches('\"').to_string();
                //     features.push(Feature { name: feature_name, required: true, implied_reason: None });
            } else if line.starts_with("native-code:") {
                native_code = line.split(':').nth(1).unwrap().trim().to_string();
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

        // convert to vector
        let uses_permissions = uses_permissions.into_iter().collect::<Vec<String>>();

        let manifest = Manifest {
            package,
            sdk_version,
            target_sdk_version,
            uses_permissions,
            application_labels,
            application_icons,
            application,
            launchable_activity,
            native_code,
        };

        return Ok(manifest);
    }
}

#[test]
fn aapt2_test() {
    let aapt2 = AAPT2::new();
    let manifest = aapt2.dump_badging(PathBuf::from("C:\\Users\\luhao\\Desktop\\mm\\77.apk")).expect("");
    println!("manifest => {:#?}", manifest);
}

#[test]
fn aapt2_dump_packagename_test() {
    let aapt2 = AAPT2::new();
    let package_name = aapt2.dump_packagename(PathBuf::from("C:\\Users\\luhao\\Desktop\\mm\\77.apk")).expect("");
    println!("package_name => {:#?}", package_name);
}

#[test]
fn aapt2_dump_version_test() {
    let aapt2 = AAPT2::new();
    let version = aapt2.version().expect("");
    println!("version => {:#?}", version);
}