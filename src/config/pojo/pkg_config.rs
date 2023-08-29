use std::{path::PathBuf, collections::BTreeSet};

use serde::{Serialize, Deserialize};

use crate::{config::pojo::base_config::BaseConfig, util::confy_util::ConfyUtil, pkg::pkgm::Pkgm};

use super::{pkg::{dnf_config::DnfConfig, pacman_config::PacmanConfig, yay_config::YayConfig}, system_config::SystemConfig, link_config::LinkConfig, base_config};



#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PkgConfig {
    #[serde(skip_serializing, default)]
    pub file_name: String,
    #[serde(default)]
    pub dnf_config: LinkConfig,
    #[serde(default)]
    pub pacman_config: LinkConfig,
    #[serde(default)]
    pub yay_config: LinkConfig,
}

impl BaseConfig for PkgConfig {
    fn get_default_config_file_name() -> String {
        "pkg".to_string()

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

        path.clone()
    }
}

impl PkgConfig {
    pub fn get_pkg_config(config: &Option<String>) -> PkgConfig {
        let config_name = match config {
            Some(conf) => conf.clone(),
            None => {
                let system_config = SystemConfig::get_system_config();

                system_config.package_config.link_config.main_config
            },
        };

        PkgConfig::get_config(Some(config_name))
    }

    pub fn get_dnf_config(&self) -> DnfConfig {
        DnfConfig::get_config(Some(self.dnf_config.main_config.clone()))
    }
    pub fn get_pacman_config(&self) -> PacmanConfig {
        PacmanConfig::get_config(Some(self.pacman_config.main_config.clone()))
    }
    pub fn get_yay_config(&self) -> YayConfig {
        YayConfig::get_config(Some(self.yay_config.main_config.clone()))
    }

    pub fn get_packages(&self, pkgm: &Pkgm) -> BTreeSet<String> {
        match pkgm {
            Pkgm::DNF => {
                let config = self.get_dnf_config();
                config.packages
            },
            Pkgm::PACMAN =>  {
                let config = self.get_pacman_config();
                config.packages
            },
            Pkgm::YAY => {
                let config = self.get_yay_config();
                config.packages
            }
        }
    }

    pub fn set_packages(&mut self, pkgm: &Pkgm, packages: &BTreeSet<String>) {
        match pkgm {
            Pkgm::DNF => {
                let mut config = self.get_dnf_config();
                config.packages = packages.clone();
                base_config::save_config(&config)
            },
            Pkgm::PACMAN => {
                let mut config = self.get_pacman_config();
                config.packages = packages.clone();
                base_config::save_config(&config)
            },
            Pkgm::YAY => {
                let mut config = self.get_yay_config();
                config.packages = packages.clone();
                base_config::save_config(&config)
            },
        }
    }
}