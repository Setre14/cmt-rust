use crate::util::exec;
use crate::config::pkgm;
use crate::git_config;

use clap::{Subcommand};

#[derive(Subcommand)]
pub enum Command {
    /// Install a package
    Install {
        /// Package to install
        package: String,
    },

    /// Remove a package
    Remove {
        /// Package to remove
        package: String,
    },

    /// Update all packages
    Upgrade {}

}

pub struct Dnf {
    
}

impl Dnf {
    pub fn handle_command(command: &Command) {
        match command {
            Command::Install {package} => {
                Self::install(package);
            },
            Command::Remove {package} => {
                Self::remove(package);
            },
            Command::Upgrade {} => {
                Self::upgrade();
            }
        }
    }

    pub fn install(package: &String) 
    {
        let mut pkgm_conf = pkgm::get_conf(pkgm::Pkgm::DNF);

        let command = ["dnf", "install", package];    
        let result = exec::status("sudo", command);
    
        if result {
            pkgm_conf.add_package(package);
            git_config::update(&Some(format!("Add package '{}'", package)));
        }
    }

    pub fn remove(package: &String) 
    {
        let mut pkgm_conf = pkgm::get_conf(pkgm::Pkgm::DNF);

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
