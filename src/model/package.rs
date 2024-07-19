use serde::{Deserialize, Serialize};

/// package:
/// name='com.facebook.videodownload.videodownloaderforfacebook'
/// versionCode='75'
/// versionName='3.0.6.1'
/// platformBuildVersionName='13'
/// platformBuildVersionCode='33'
/// compileSdkVersion='33'
/// compileSdkVersionCodename='13'
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Package {
    pub name: String,
    #[serde(rename = "versionName")]
    pub version_name: String,
    #[serde(rename = "versionCode")]
    pub version_code: u32,
    #[serde(rename = "platformBuildVersionName")]
    pub platform_build_version_name: String,
    #[serde(rename = "platformBuildVersionCode")]
    pub platform_build_version_code: u32,
    #[serde(rename = "compileSdkVersion")]
    pub compile_sdk_version: String,
    #[serde(rename = "compileSdkVersionCodename")]
    pub compile_sdk_version_codename: String,
}