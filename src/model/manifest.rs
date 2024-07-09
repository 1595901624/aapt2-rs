use std::iter::Map;
use crate::model::package::Package;

///
/// sdkVersion:'23'
/// targetSdkVersion:'33'
/// uses-permission: name='android.permission.INTERNET'
/// application-label:'AI Chat'
/// application-icon-160:'res/mipmap-mdpi-v4/ic_launcher.webp'
/// application-icon-240:'res/mipmap-hdpi-v4/ic_launcher.webp'
/// application-icon-320:'res/mipmap-xhdpi-v4/ic_launcher.webp'
/// application-icon-480:'res/mipmap-xxhdpi-v4/ic_launcher.webp'
/// application-icon-640:'res/mipmap-xxxhdpi-v4/ic_launcher.webp'
/// application-icon-65534:'res/mipmap-mdpi-v4/ic_launcher.webp'
pub(crate) struct Manifest {
    pub(crate) package: Package,
    pub(crate) application_label: Map<String, String>,
}
