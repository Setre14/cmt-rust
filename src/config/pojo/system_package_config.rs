use std::collections::BTreeSet;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemPackageConfig {
    #[serde(default = "SystemPackageConfig::get_default_package_config")]
    pub configs: BTreeSet<String>,
}

impl Default for SystemPackageConfig {
    fn default() -> Self { 
        SystemPackageConfig {
            configs: SystemPackageConfig::get_default_package_config()
        }
    }
}

impl SystemPackageConfig {
    pub fn get_default_package_config() -> BTreeSet<String> {
        let mut configs: BTreeSet<String> = BTreeSet::new();
        configs.insert("package".to_string());
        configs
    }
}
