use std::collections::HashMap;

use crate::model::locale::Locale;

pub struct ApplicationLabel {
    pub map: HashMap<String, String>,
}

impl ApplicationLabel {
    pub fn new() -> Self {
        return Self {
            map: HashMap::new(),
        };
    }

    pub fn get_application_label(&self, locale: Locale) -> String {
        if self.map.get(&self.get_application_label(locale)).is_none() {
            return self.map.get(&self.get_application_label(Locale::DEFAULT)).unwrap().clone();
        }
        return self.map.get(&self.get_application_label(locale)).unwrap().clone();
    }

    /// 获取应用标签的key
    fn get_map_key(&self, locale: Locale) -> String {
        if locale == Locale::DEFAULT {
            return "application-label".to_string();
        }

        let locale = locale.to_string();
        return if locale.contains("_") {
            format!("application-label-{}-{}", locale.split("_").next().unwrap().to_lowercase(), locale.split("_").last().unwrap())
        } else {
            format!("application-label-{}", locale.to_lowercase())
        };
    }
}

#[test]
fn test() {
    let mut application_label = ApplicationLabel::new();
    // let label_key = application_label.get_map_key(Locale::ZH_CN);
    // println!("label => {:?}", label_key);
    // let label_key = application_label.get_map_key(Locale::EN);
    // println!("label => {:?}", label_key);

}