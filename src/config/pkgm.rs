use serde::{Serialize, Deserialize};
use std::fmt;
use std::path::PathBuf;
use std::fs;

use crate::base;
use crate::config::app;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PkgmConfig {
    pub pkgm: String,
    pub repos: Vec<String>,
    pub remotes: Vec<String>,
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
        if !self.packages.contains(package) {
            self.packages.push(package.clone().to_string());
        }
        self.packages.sort();

        base::save_conf(self);
    }

    pub fn remove_package(&mut self, package: &String) {
        if self.packages.contains(package) {
            let index = self.packages.iter().position(|x| *x == package.to_string()).unwrap();
            self.packages.remove(index);
        }
        self.packages.sort();
    
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
