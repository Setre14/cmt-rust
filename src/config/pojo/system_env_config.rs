use std::collections::BTreeSet;

use serde::{Serialize, Deserialize};

use crate::util::exec::Exec;

use super::link_config::LinkConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemEnvConfig {
    #[serde(default, flatten)]
    pub link_config: LinkConfig,
    #[serde(default)]
    pub template_values: String,
}

impl Default for SystemEnvConfig {
    fn default() -> Self { 
        SystemEnvConfig {
            link_config: LinkConfig { ..Default::default() },
            template_values: Self::get_default_template_values(),
        }
    }
}

impl SystemEnvConfig {
    pub fn get_default_template_values() -> String {
        format!("values-{}.json", Exec::get_hostname())
    }
}
