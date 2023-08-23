use std::{path::PathBuf, collections::BTreeSet};

use serde::{Serialize, Deserialize};

use crate::{config::pojo::base_config::BaseConfig, util::confy_util::ConfyUtil, pkg::pkgm::Pkgm};

use super::{pkg::{dnf_config::DnfConfig, pacman_config::PacmanConfig, yay_config::YayConfig}, system_config::SystemConfig};



#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PkgConfig {
    #[serde(skip_serializing, default)]
    pub file_name: String,
    #[serde(default)]
    pub dnf_config: DnfConfig,
    #[serde(default)]
    pub pacman_config: PacmanConfig,
    #[serde(default)]
    pub yay_config: YayConfig,
}

impl BaseConfig for PkgConfig {
    fn get_default_config_file_name() -> String {
        "pkg/pkg".to_string()

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

                if system_config.package_config.configs.len() > 1 {
                    log::error!("More then one pkg config is set, specify where it should be added with --env-config");
                    std::process::exit(1);
                }

                system_config.package_config.configs.first().unwrap().clone()
            },
        };

        PkgConfig::get_config(Some(format!("pkg/{}", config_name)))
    }

    pub fn get_packages(&self, pkgm: &Pkgm) -> BTreeSet<String> {
        match pkgm {
            Pkgm::DNF => self.dnf_config.packages.clone(),
            Pkgm::PACMAN => self.pacman_config.packages.clone(),
            Pkgm::YAY => self.yay_config.packages.clone(),
        }
    }

    pub fn set_packages(&mut self, pkgm: &Pkgm, packages: &BTreeSet<String>) {
        match pkgm {
            Pkgm::DNF => self.dnf_config.packages = packages.clone(),
            Pkgm::PACMAN => self.pacman_config.packages = packages.clone(),
            Pkgm::YAY => self.yay_config.packages = packages.clone(),
        }
    }
}