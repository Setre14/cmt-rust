use serde::{Serialize, Deserialize};
use std::fmt;

use crate::base;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PkgConfig {
    pkgm: String,
    repos: Vec<String>,
    remotes: Vec<String>,
    packages: Vec<String>,
}


impl base::ConfigReader for PkgConfig {
    fn get_conf_name(&self) -> String {
        return self.pkgm.to_string() + ".json";
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
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
pub fn get_conf(pkgm: Pkgm) -> PkgConfig {
    return base::get_conf(PkgConfig { pkgm: pkgm.to_string(), ..Default::default() });
}
