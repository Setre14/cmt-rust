use serde::{Serialize, Deserialize};
use std::fmt;
use std::path::PathBuf;
use std::fs;

use crate::base;
use crate::config::app;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PkgmConfig {
    #[serde(default)]
    pub pkgm: String,
    #[serde(default)]
    pub repos: Vec<String>,
    #[serde(default)]
    pub remotes: Vec<String>,
    #[serde(default)]
    pub packages: Vec<String>,
}


impl base::ConfigReader for PkgmConfig {
    fn get_conf_name(&self) -> String {
        return self.pkgm.to_string() + ".json";
    }

    fn get_conf_dir(&self) -> PathBuf {
        let app_conf = app::get_conf();

        let mut conf_dir = PathBuf::new();
        conf_dir.push(app_conf.git_config_dir);
        conf_dir.push("pkg");
        let _ = fs::create_dir_all(conf_dir.clone());
    
        return conf_dir;
    }
}

impl PkgmConfig {
    pub fn add_package(&mut self, package: &String) {
        base::add_to_list(&mut self.packages, package);
        base::save_conf(self);
    }

    pub fn remove_package(&mut self, package: &String) {
        base::remove_from_list(&mut self.packages, package);
        base::save_conf(self);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Pkgm {
    PACMAN,
    DNF
}

impl Default for Pkgm {
    fn default() -> Self { Pkgm::PACMAN }
}

impl fmt::Display for Pkgm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Pkgm::PACMAN => write!(f, "pacman"),
            Pkgm::DNF => write!(f, "dnf"),
        }
    }
}

#[allow(dead_code)]
pub fn get_conf(pkgm: Pkgm) -> PkgmConfig {
    return base::get_conf(PkgmConfig { pkgm: pkgm.to_string(), ..Default::default() });
}
