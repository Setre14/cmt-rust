use serde::{Serialize, Deserialize};

use crate::config::pojo::base_config::BaseConfig;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SystemConfig {
    #[serde(default)]
    pub template_values: String,
    #[serde(default)]
    pub env_config: Vec<String>,
    #[serde(default)]
    pub package_config: Vec<String>,
}

impl BaseConfig for SystemConfig {
    fn get_config_file_name() -> String {
        "system.json".to_string()
    }

    fn get_default() -> SystemConfig {
        SystemConfig { ..Default::default()}
    }
}

impl SystemConfig {
}