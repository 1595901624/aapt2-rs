use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Application {
    pub label: String,
    pub icon: String,
}