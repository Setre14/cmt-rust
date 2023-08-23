use std::collections::BTreeSet;

use serde::{Serialize, Deserialize};

use crate::util::exec::Exec;

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
            template_values: Self::get_default_template_values(),
            configs: Self::get_default_env_config()
        }
    }
}

impl SystemEnvConfig {
    pub fn get_default_template_values() -> String {
        format!("values-{}.json", Exec::get_hostname())
    }

    pub fn get_default_env_config() -> BTreeSet<String> {
        let mut configs: BTreeSet<String> = BTreeSet::new();
        configs.insert(format!("env-{}", Exec::get_hostname()));
        configs
    }
}
