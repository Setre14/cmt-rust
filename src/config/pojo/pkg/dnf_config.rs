use std::collections::BTreeSet;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DnfConfig {
    #[serde(default)]
    pub packages: BTreeSet<String>,
}
