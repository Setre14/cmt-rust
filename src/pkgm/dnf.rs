use crate::util::exec;
use crate::config::pkgm_config;
use crate::config::package_manager::PackageManager;
use crate::git_config;
use crate::config::config_track;

use clap::{Subcommand};

#[derive(Subcommand)]
pub enum Command {
    /// Install a package
    Install {
        /// Package to install
        package: String,

        /// Install package globally
        #[arg(short, long)]
        global: bool,
    },

    /// Remove a package
    Remove {
        /// Package to remove
        package: String,

        /// Remove package globally
        #[arg(short, long)]
        global: bool,
    },

    /// Update all packages
    Upgrade {}

}

pub struct Dnf {
    
}

impl Dnf {
    pub fn handle_command(command: &Command) {
        match command {
            Command::Install {package, global} => {
                Self::install(package, global);
            },
            Command::Remove {package, global} => {
                Self::remove(package, global);
            },
            Command::Upgrade {} => {
                Self::upgrade();
            }
        }
    }

    pub fn install(package: &String, global: &bool) 
    {
        let mut pkgm_conf = pkgm_config::get_conf(&PackageManager::DNF, &config_track::bool_to_track(global));

        log::info!("pkgm_conf: {:?}", pkgm_conf.clone());

        let command = ["dnf", "install", package];    
        let result = exec::status("sudo", command);
    
        if result {
            pkgm_conf.add_package(package);
            git_config::update(&Some(format!("Add package '{}'", package)));
        }
    }

    pub fn remove(package: &String, global: &bool) 
    {
        let mut pkgm_conf = pkgm_config::get_conf(&PackageManager::DNF, &config_track::bool_to_track(global));

        let command = ["dnf", "install", package];
        let result = exec::status("sudo", command);
        
        if result {
            pkgm_conf.remove_package(package);
            git_config::update(&Some(format!("Remove package '{}'", package)));
        }
    }

    pub fn upgrade() {
        let _result = exec::status("sudo", ["dnf", "upgrade"]);
    }
}
