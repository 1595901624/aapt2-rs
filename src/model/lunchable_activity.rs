use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LaunchableActivity {
    pub name: String,
    pub label: String,
    pub icon: String,
}