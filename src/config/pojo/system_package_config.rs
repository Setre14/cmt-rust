use std::collections::BTreeSet;

use serde::{Serialize, Deserialize};

use crate::util::exec::Exec;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemPackageConfig {
    #[serde(default = "SystemPackageConfig::get_default_package_config")]
    pub main_config: String,
    #[serde(default = "SystemPackageConfig::get_default_package_configs")]
    pub configs: BTreeSet<String>,
}

impl Default for SystemPackageConfig {
    fn default() -> Self { 
        SystemPackageConfig {
            main_config: SystemPackageConfig::get_default_package_config(),
            configs: SystemPackageConfig::get_default_package_configs(),
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
