use std::collections::BTreeSet;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemEnvConfig {
    #[serde(default)]
    pub template_values: String,
    #[serde(default = "SystemEnvConfig::get_default_env_config")]
    pub configs: BTreeSet<String>,
}

impl Default for SystemEnvConfig {
    fn default() -> Self { 
        SystemEnvConfig {
            template_values: "values.json".to_string(),
            configs: SystemEnvConfig::get_default_env_config()
        }
    }
}

impl SystemEnvConfig {
    pub fn get_default_env_config() -> BTreeSet<String> {
        let mut configs: BTreeSet<String> = BTreeSet::new();
        configs.insert("env".to_string());
        configs
    }
}
