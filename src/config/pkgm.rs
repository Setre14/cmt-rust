use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs;
use std::string::ToString;
use strum_macros::Display;
use std::str::FromStr;
use strum_macros::EnumString;
use strum::{EnumIter, IntoEnumIterator};

use crate::config::app;
use crate::config::config_reader;
use crate::config::config_reader::ConfigReader;
use crate::config::config_track::ConfigTrack;
use crate::config::config_util::ConfigUtil;
use crate::config::string_accessable::StringAccessable;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PkgmConfig {
    #[serde(default)]
    pub track: String,
    #[serde(default)]
    pub pkgm: String,
    #[serde(default)]
    pub repos: Vec<String>,
    #[serde(default)]
    pub remotes: Vec<String>,
    #[serde(default)]
    pub packages: Vec<String>,
}

impl StringAccessable for PkgmConfig {
    fn get_string(&self, field_string: &str) -> Result<&String, String> {
        match field_string {
            "pkgm" => Ok(&self.pkgm),
            _ => Err(format!("invalid field name to get '{}'", field_string))
        }
    }

    fn get_vec(&self, field_string: &str) -> Result<&Vec<String>, String> {
        match field_string {
            "repos" => Ok(&self.repos),
            "remotes" => Ok(&self.remotes),
            "packages" => Ok(&self.packages),
            _ => Err(format!("invalid field name to get '{}'", field_string))
        }
    }

    fn get_u8(&self, field_string: &str) -> Result<&u8, String> {
        match field_string {
            _ => Err(format!("invalid field name to get '{}'", field_string))
        }
    }
}

impl ConfigReader for PkgmConfig {
    // fn get_conf_name(&self) -> String {
    //     return self.pkgm.to_string().to_lowercase();
    // }

    fn get_conf_dir(&self) -> PathBuf {
        let app_conf = app::get_conf();

        let mut conf_dir = PathBuf::new();
        conf_dir.push(app_conf.git_config_dir);
        conf_dir.push("pkg");
        conf_dir.push(self.pkgm.to_string().to_lowercase());
        let _ = fs::create_dir_all(conf_dir.clone());
    
        return conf_dir;
    }

    fn get_track(&self) -> ConfigTrack {
        return ConfigTrack::from_str(&self.track).unwrap();
    }

    fn set_track(&mut self, track: &ConfigTrack) {
        self.track = track.to_string();
    }

    fn merge<T: StringAccessable + Clone + std::fmt::Debug>(&mut self, other: T) {
        self.pkgm = other.get_string("pkgm").unwrap().to_string();
        log::debug!("Merge self: {:?}", self.clone());
        log::debug!("Merge other: {:?}", other.clone());

        let repos = other.get_vec("repos").unwrap();
        merge_vec(&mut self.repos, repos);

        let remotes = other.get_vec("remotes").unwrap();
        merge_vec(&mut self.remotes, remotes);
        
        let packages = other.get_vec("packages").unwrap();
        merge_vec(&mut self.packages, packages);

        log::debug!("Merge result: {:?}", self.clone());
    }
}

impl PkgmConfig {
    pub fn add_package(&mut self, package: &String) {
        ConfigUtil::add_to_list(&mut self.packages, package);
        config_reader::save_conf(self);
    }

    pub fn remove_package(&mut self, package: &String) {
        ConfigUtil::remove_from_list(&mut self.packages, package);
        config_reader::save_conf(self);
    }
}

#[derive(Serialize, Deserialize, Debug, Display, EnumString, EnumIter)]
pub enum Pkgm {
    PACMAN,
    DNF
}

impl Default for Pkgm {
    fn default() -> Self { Pkgm::PACMAN }
}

// impl fmt::Display for Pkgm {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Pkgm::PACMAN => write!(f, "pacman"),
//             Pkgm::DNF => write!(f, "dnf"),
//         }
//     }
// }

pub fn get_conf(pkgm: &Pkgm, track: &ConfigTrack) -> PkgmConfig {
    return config_reader::get_conf(&track, &mut PkgmConfig { pkgm: pkgm.to_string(), ..Default::default() });
}

#[allow(dead_code)]
pub fn get_combined_conf(pkgm: &Pkgm) -> PkgmConfig {
    return config_reader::get_combined_conf(&mut PkgmConfig { pkgm: pkgm.to_string(), ..Default::default() });
}

fn merge_vec(a: &mut Vec<String>, b: &Vec<String>) {
    for item in b {
        if !a.contains(&item) {
            a.push(item.clone().to_string());
        }
    } 
}

pub fn cleanup() {
    
    for pkgm in Pkgm::iter() {
        let global_conf = get_conf(&pkgm, &ConfigTrack::GLOBAL);
        let mut system_conf = get_conf(&pkgm, &ConfigTrack::SYSTEM);
        // for repo in global_conf.repos {
        //     system_conf.remove_repos(&repo);
        // }
    
        // for remote in global_conf.remotes {
        //     system_conf.remove_remote(&remote);
        // }
    
        for package in global_conf.packages {
            system_conf.remove_package(&package);
        }
    }
}