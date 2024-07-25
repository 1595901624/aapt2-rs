use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
#[warn(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Density {
    LDPI = 120,
    MDPI = 160,
    HDPI = 240,
    XHDPI = 320,
    XXHDPI = 480,
    XXXHDPI = 640,

    None65534 = 65534,
    #[warn(non_camel_case_types)]
    None65535 = 65535,
}

impl From<i32> for Density {
    fn from(value: i32) -> Self {
        match value {
            120 => Density::LDPI,
            160 => Density::MDPI,
            240 => Density::HDPI,
            320 => Density::XHDPI,
            480 => Density::XXHDPI,
            640 => Density::XXXHDPI,
            65534 => Density::None65534,
            65535 => Density::None65535,
            _ => Density::XHDPI
        }
    }
}

impl From<String> for Density {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ldpi" => Density::LDPI,
            "mdpi" => Density::MDPI,
            "hdpi" => Density::HDPI,
            "xhdpi" => Density::XHDPI,
            "xxhdpi" => Density::XXHDPI,
            "xxxhdpi" => Density::XXXHDPI,
            "none_65534" => Density::None65534,
            "none_65535" => Density::None65535,
            "120" => Density::LDPI,
            "160" => Density::MDPI,
            "240" => Density::HDPI,
            "320" => Density::XHDPI,
            "480" => Density::XXHDPI,
            "640" => Density::XXXHDPI,
            "65534" => Density::None65534,
            "65535" => Density::None65535,
            _ => Density::XHDPI
        }
    }
}

impl Display for Density {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}