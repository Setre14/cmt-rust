use std::collections::BTreeSet;

use serde::{Serialize, Deserialize};

use crate::util::exec::Exec;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkConfig {
    #[serde(default = "LinkConfig::get_default_main_config")]
    pub main_config: String,
    #[serde(default = "LinkConfig::get_default_configs")]
    pub configs: BTreeSet<String>,
}

impl Default for LinkConfig {
    fn default() -> Self { 
        LinkConfig {
            main_config: Self::get_default_main_config(),
            configs: Self::get_default_configs(),
        }
    }
}

impl LinkConfig {
    pub fn get_default_main_config() -> String {
        return format!("config-{}", Exec::get_hostname());
    }

    pub fn get_default_configs() -> BTreeSet<String> {
        let mut configs: BTreeSet<String> = BTreeSet::new();
        configs.insert(Self::get_default_main_config());
        configs
    }
}