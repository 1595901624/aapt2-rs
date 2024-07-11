use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct Application {
    pub(crate) label: String,
    pub(crate) icon: String,
}