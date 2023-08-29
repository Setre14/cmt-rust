use serde::{Serialize, Deserialize};

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
