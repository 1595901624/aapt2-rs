#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused)]

use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

/// application-label-af:'AI Chat'
/// application-label-am:'AI Chat'
/// application-label-ar:'AI Chat'
/// application-label-as:'AI Chat'
/// application-label-az:'AI Chat'
/// application-label-be:'AI Chat'
/// application-label-bg:'AI Chat'
/// application-label-bn:'AI Chat'
/// application-label-bs:'AI Chat'
/// application-label-ca:'AI Chat'
/// application-label-cs:'AI Chat'
/// application-label-da:'AI Chat'
/// application-label-de:'AI Chat'
/// application-label-el:'AI Chat'
/// application-label-en:'AI Chat'
/// application-label-en-AU:'AI Chat'
/// application-label-en-CA:'AI Chat'
/// application-label-en-GB:'AI Chat'
/// application-label-en-IN:'AI Chat'
/// application-label-en-XC:'AI Chat'
/// application-label-es:'AI Chat'
/// application-label-es-419:'AI Chat'
/// application-label-es-US:'AI Chat'
/// application-label-et:'AI Chat'
/// application-label-eu:'AI Chat'
/// application-label-fa:'AI Chat'
/// application-label-fi:'AI Chat'
/// application-label-fr:'AI Chat'
/// application-label-fr-CA:'AI Chat'
/// application-label-gl:'AI Chat'
/// application-label-gu:'AI Chat'
/// application-label-hi:'AI Chat'
/// application-label-hr:'AI Chat'
/// application-label-hu:'AI Chat'
/// application-label-hy:'AI Chat'
/// application-label-id:'AI Chat'
/// application-label-in:'AI Chat'
/// application-label-is:'AI Chat'
/// application-label-it:'AI Chat'
/// application-label-iw:'AI Chat'
/// application-label-ja:'AI Chat'
/// application-label-ka:'AI Chat'
/// application-label-kk:'AI Chat'
/// application-label-km:'AI Chat'
/// application-label-kn:'AI Chat'
/// application-label-ko:'AI Chat'
/// application-label-ky:'AI Chat'
/// application-label-lo:'AI Chat'
/// application-label-lt:'AI Chat'
/// application-label-lv:'AI Chat'
/// application-label-mk:'AI Chat'
/// application-label-ml:'AI Chat'
/// application-label-mn:'AI Chat'
/// application-label-mr:'AI Chat'
/// application-label-ms:'AI Chat'
/// application-label-my:'AI Chat'
/// application-label-nb:'AI Chat'
/// application-label-ne:'AI Chat'
/// application-label-nl:'AI Chat'
/// application-label-no:'AI Chat'
/// application-label-or:'AI Chat'
/// application-label-pa:'AI Chat'
/// application-label-pl:'AI Chat'
/// application-label-pt:'AI Chat'
/// application-label-pt-BR:'AI Chat'
/// application-label-pt-PT:'AI Chat'
/// application-label-ro:'AI Chat'
/// application-label-ru:'AI Chat'
/// application-label-si:'AI Chat'
/// application-label-sk:'AI Chat'
/// application-label-sl:'AI Chat'
/// application-label-sq:'AI Chat'
/// application-label-sr:'AI Chat'
/// application-label-sr-Latn:'AI Chat'
/// application-label-sv:'AI Chat'
/// application-label-sw:'AI Chat'
/// application-label-ta:'AI Chat'
/// application-label-te:'AI Chat'
/// application-label-th:'AI Chat'
/// application-label-tl:'AI Chat'
/// application-label-tr:'AI Chat'
/// application-label-tw:'AI Chat'
/// application-label-uk:'AI Chat'
/// application-label-ur:'AI Chat'
/// application-label-uz:'AI Chat'
/// application-label-vi:'AI Chat'
/// application-label-zh:'AI Chat'
/// application-label-zh-CN:'AI Chat'
/// application-label-zh-HK:'AI Chat'
/// application-label-zh-MO:'AI Chat'
/// application-label-zh-TW:'AI Chat'
/// application-label-zu:'AI Chat'
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) enum Locale {
    DEFAULT,
    AF,
    AM,
    AR,
    AS,
    AZ,
    BE,
    BG,
    BN,
    BS,
    CA,
    CS,
    DA,
    DE,
    EL,
    EN,
    EN_AU,
    EN_CA,
    EN_GB,
    EN_IN,
    EN_XC,
    ES,
    ES_419,
    ES_US,
    ET,
    EU,
    FA,
    FI,
    FR,
    FR_CA,
    GL,
    GU,
    HI,
    HR,
    HU,
    HY,
    ID,
    IN,
    IS,
    IT,
    IW,
    JA,
    KA,
    KK,
    KM,
    KN,
    KO,
    KY,
    LO,
    LT,
    LV,
    MK,
    ML,
    MN,
    MR,
    MS,
    MY,
    NB,
    NE,
    NL,
    NO,
    OR,
    PA,
    PL,
    PT,
    PT_BR,
    PT_PT,
    RO,
    RU,
    SI,
    SK,
    SL,
    SQ,
    SR,
    SR_LATN,
    SV,
    SW,
    TA,
    TE,
    TH,
    TL,
    TR,
    TW,
    UK,
    UR,
    UZ,
    VI,
    ZH,
    ZH_CN,
    ZH_HK,
    ZH_MO,
    ZH_TW,
    ZU,
}

impl From<String> for Locale {
    fn from(value: String) -> Self {
        match value.trim().to_uppercase().as_str() {
            "DEFAULT" => Locale::DEFAULT,
            "AF" => Locale::AF,
            "AM" => Locale::AM,
            "AR" => Locale::AR,
            "AS" => Locale::AS,
            "AZ" => Locale::AZ,
            "BE" => Locale::BE,
            "BG" => Locale::BG,
            "BN" => Locale::BN,
            "BS" => Locale::BS,
            "CA" => Locale::CA,
            "CS" => Locale::CS,
            "DA" => Locale::DA,
            "DE" => Locale::DE,
            "EL" => Locale::EL,
            "EN" => Locale::EN,
            "EN_AU" => Locale::EN_AU,
            "EN_CA" => Locale::EN_CA,
            "EN_GB" => Locale::EN_GB,
            "EN_IN" => Locale::EN_IN,
            "EN_XC" => Locale::EN_XC,
            "ES" => Locale::ES,
            "ES_419" => Locale::ES_419,
            "ES_US" => Locale::ES_US,
            "ET" => Locale::ET,
            "EU" => Locale::EU,
            "FA" => Locale::FA,
            "FI" => Locale::FI,
            "FR" => Locale::FR,
            "FR_CA" => Locale::FR_CA,
            "GL" => Locale::GL,
            "GU" => Locale::GU,
            "HI" => Locale::HI,
            "HR" => Locale::HR,
            "HU" => Locale::HU,
            "HY" => Locale::HY,
            "ID" => Locale::ID,
            "IN" => Locale::IN,
            "IS" => Locale::IS,
            "IT" => Locale::IT,
            "IW" => Locale::IW,
            "JA" => Locale::JA,
            "KA" => Locale::KA,
            "KK" => Locale::KK,
            "KM" => Locale::KM,
            "KN" => Locale::KN,
            "KO" => Locale::KO,
            "KY" => Locale::KY,
            "LO" => Locale::LO,
            "LT" => Locale::LT,
            "LV" => Locale::LV,
            "MK" => Locale::MK,
            "ML" => Locale::ML,
            "MN" => Locale::MN,
            "MR" => Locale::MR,
            "MS" => Locale::MS,
            "MY" => Locale::MY,
            "NB" => Locale::NB,
            "NE" => Locale::NE,
            "NL" => Locale::NL,
            "NO" => Locale::NO,
            "OR" => Locale::OR,
            "PA" => Locale::PA,
            "PL" => Locale::PL,
            "PT" => Locale::PT,
            "PT_BR" => Locale::PT_BR,
            "PT_PT" => Locale::PT_PT,
            "RO" => Locale::RO,
            "RU" => Locale::RU,
            "SI" => Locale::SI,
            "SK" => Locale::SK,
            "SL" => Locale::SL,
            "SQ" => Locale::SQ,
            "SR" => Locale::SR,
            "SR_LATN" => Locale::SR_LATN,
            "SV" => Locale::SV,
            "SW" => Locale::SW,
            "TA" => Locale::TA,
            "TE" => Locale::TE,
            "TH" => Locale::TH,
            "TL" => Locale::TL,
            "TR" => Locale::TR,
            "TW" => Locale::TW,
            "UK" => Locale::UK,
            "UR" => Locale::UR,
            "UZ" => Locale::UZ,
            "VI" => Locale::VI,
            "ZH" => Locale::ZH,
            "ZH_CN" => Locale::ZH_CN,
            "ZH_HK" => Locale::ZH_HK,
            "ZH_MO" => Locale::ZH_MO,
            "ZH_TW" => Locale::ZH_TW,
            "ZU" => Locale::ZU,
            _ => Locale::DEFAULT,
        }
    }
}

impl Display for Locale {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[test]
fn test() {
    println!("{}", Locale::AF.to_string());
}