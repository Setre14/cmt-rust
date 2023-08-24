use std::collections::BTreeSet;

use serde::{Serialize, Deserialize};

use crate::util::exec::Exec;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemEnvConfig {
    #[serde(default = "SystemEnvConfig::get_default_env_config")]
    pub main_config: String,
    #[serde(default = "SystemEnvConfig::get_default_env_configs")]
    pub configs: BTreeSet<String>,
    #[serde(default)]
    pub template_values: String,
}

impl Default for SystemEnvConfig {
    fn default() -> Self { 
        SystemEnvConfig {
            main_config: Self::get_default_env_config(),
            configs: Self::get_default_env_configs(),
            template_values: Self::get_default_template_values(),
        }
    }
}

impl SystemEnvConfig {
    pub fn get_default_template_values() -> String {
        format!("values-{}.json", Exec::get_hostname())
    }

    pub fn get_default_env_config() -> String {
        return format!("env-{}", Exec::get_hostname());
    }

    pub fn get_default_env_configs() -> BTreeSet<String> {
        let mut configs: BTreeSet<String> = BTreeSet::new();
        configs.insert(Self::get_default_env_config());
        configs
    }
}
