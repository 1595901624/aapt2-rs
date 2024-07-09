use serde::{Deserialize, Serialize};

/// package:
/// name='com.facebook.videodownload.videodownloaderforfacebook'
/// versionCode='75'
/// versionName='3.0.6.1'
/// platformBuildVersionName='13'
/// platformBuildVersionCode='33'
/// compileSdkVersion='33'
/// compileSdkVersionCodename='13'
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Package {
    pub(crate) name: String,
    #[serde(rename = "versionName")]
    pub(crate) version_name: String,
    #[serde(rename = "versionCode")]
    pub(crate) version_code: u32,
    #[serde(rename = "platformBuildVersionName")]
    pub(crate) platform_build_version_name: String,
    #[serde(rename = "platformBuildVersionCode")]
    pub(crate) platform_build_version_code: u32,
    #[serde(rename = "compileSdkVersion")]
    pub(crate) compile_sdk_version: String,
    #[serde(rename = "compileSdkVersionCodename")]
    pub(crate) compile_sdk_version_codename: String,
}