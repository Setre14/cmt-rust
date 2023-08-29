use std::collections::BTreeSet;

use serde::{Serialize, Deserialize};

use crate::util::exec::Exec;

use super::link_config::LinkConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemPackageConfig {
    #[serde(default, flatten)]
    pub link_config: LinkConfig,
}

impl Default for SystemPackageConfig {
    fn default() -> Self { 
        SystemPackageConfig {
            link_config: LinkConfig { ..Default::default() },
        }
    }
}

impl SystemPackageConfig {
    pub fn get_default_package_config() -> String {
        return format!("package-{}", Exec::get_hostname())
    }

    pub fn get_default_package_configs() -> BTreeSet<String> {
        let mut configs: BTreeSet<String> = BTreeSet::new();
        configs.insert(Self::get_default_package_config());
        configs
    }
}
