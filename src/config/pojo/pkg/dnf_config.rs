use std::{collections::BTreeSet, path::PathBuf};

use serde::{Serialize, Deserialize};

use crate::{util::confy_util::ConfyUtil, config::pojo::base_config::BaseConfig};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DnfConfig {
    #[serde(skip_serializing, default)]
    pub file_name: String,
    #[serde(default)]
    pub repos: BTreeSet<String>,
    #[serde(default)]
    pub packages: BTreeSet<String>,
}

impl BaseConfig for DnfConfig {
    fn get_default_config_file_name() -> String {
        "pkg/dnf/dnf".to_string()

    }

    fn get_config_file_name(&self) -> String {
        self.file_name.clone()
    }

    fn set_config_file_name(&mut self, file_name: &str) {
        self.file_name = file_name.to_string();
    }

    fn get_dir() -> PathBuf {
        let mut path = ConfyUtil::get_git_configuration_dir();
        path.push("pkg");
        path.push("dnf");

        path.clone()
    }
}