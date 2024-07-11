use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::model::application::Application;
use crate::model::density::Density;
use crate::model::locale::Locale;
use crate::model::lunchable_activity::LaunchableActivity;
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
/// application: label='AI Chat' icon='res/mipmap-mdpi-v4/ic_launcher.webp'
/// launchable-activity: name='com.energysh.aichat.mvvm.ui.activity.SplashActivity'  label='' icon=''
/// uses-permission: name='com.android.vending.CHECK_LICENSE'
/// feature-group: label=''
///   uses-feature-not-required: name='android.hardware.camera'
///   uses-feature: name='android.hardware.faketouch'
///   uses-implied-feature: name='android.hardware.faketouch' reason='default feature for all apps'
///   uses-feature: name='android.hardware.microphone'
///   uses-implied-feature: name='android.hardware.microphone' reason='requested android.permission.RECORD_AUDIO permission'
///   uses-feature: name='android.hardware.screen.portrait'
///   uses-implied-feature: name='android.hardware.screen.portrait' reason='one or more activities have specified a portrait orientation'
///   uses-feature: name='android.hardware.wifi'
///   uses-implied-feature: name='android.hardware.wifi' reason='requested android.permission.ACCESS_WIFI_STATE permission'
/// provides-component:'app-widget'
/// main
/// other-activities
/// other-receivers
/// other-services
/// supports-screens: 'small' 'normal' 'large' 'xlarge'
/// supports-any-density: 'true'
/// locales: '--_--' 'af' 'am' 'ar' 'as' 'az' 'be' 'bg' 'bn' 'bs' 'ca' 'cs' 'da' 'de' 'el' 'en' 'en-AU' 'en-CA' 'en-GB' 'en-IN' 'en-XC' 'es' 'es-419' 'es-US' 'et' 'eu' 'fa' 'fi' 'fr' 'fr-CA' 'gl' 'gu' 'hi' 'hr' 'hu' 'hy' 'id' 'in' 'is' 'it' 'iw' 'ja' 'ka' 'kk' 'km' 'kn' 'ko' 'ky' 'lo' 'lt' 'lv' 'mk' 'ml' 'mn' 'mr' 'ms' 'my' 'nb' 'ne' 'nl' 'no' 'or' 'pa' 'pl' 'pt' 'pt-BR' 'pt-PT' 'ro' 'ru' 'si' 'sk' 'sl' 'sq' 'sr' 'sr-Latn' 'sv' 'sw' 'ta' 'te' 'th' 'tl' 'tr' 'tw' 'uk' 'ur' 'uz' 'vi' 'zh' 'zh-CN' 'zh-HK' 'zh-MO' 'zh-TW' 'zu'
/// densities: '160' '240' '320' '480' '640' '65534'
/// native-code: 'arm64-v8a'
#[derive(Debug, Deserialize, Serialize)]
pub struct Manifest {
    pub package: Package,
    pub sdk_version: String,
    pub target_sdk_version: String,
    pub uses_permissions: Vec<String>,
    pub application_labels: HashMap<Locale, String>,
    pub application_icons: HashMap<Density, String>,
    pub application: Application,
    pub launchable_activity: LaunchableActivity,
    pub native_code: String,
}
