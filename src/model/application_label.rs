use std::collections::HashMap;
use crate::model::locale::Locale;

pub(crate) struct ApplicationLabel {
    pub(crate) map: HashMap<String, String>,
}

impl ApplicationLabel {
    pub(crate) fn new() -> Self {
        return Self {
            map: HashMap::new(),
        };
    }

    pub(crate) fn get_application_label(&self, locale: Locale) -> String {
        // if self.map.get(&locale.to_string()).is_none() {
        //         // }
        //         // return self.map.get(&locale.to_string()).unwrap().clone();
        return String::new();
    }

    fn get_map_key(&self) {}
}