use std::collections::BTreeSet;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct YayConfig {
    #[serde(default)]
    pub packages: BTreeSet<String>,
}
