use crate::util::exec;
use crate::config::pkgm_config;
use crate::config::package_manager::PackageManager;
use crate::git_config;
use crate::config::config_track;

use clap::{Subcommand};

#[derive(Subcommand)]
pub enum Command {
    /// Install a package
    #[command(name = "-S")]
    Sync {
        /// Package to install
        package: String,

        /// Install package globally
        #[arg(short, long)]
        global: bool,
    },

    /// Remove a package
    #[command(name = "-R")]
    Remove {
        /// Package to remove
        package: String,

        /// Install package globally
        #[arg(short, long)]
        global: bool,
    },

    /// Update all packages
    #[command(name = "-Syu")]
    Update {}

}

pub struct Pacman {
    
}

impl Pacman {
    pub fn handle_command(command: &Command) {
        match command {
            Command::Sync {package, global} => {
                Self::sync(package, global);
            },
            Command::Remove {package, global} => {
                Self::remove(package, global);
            },
            Command::Update {} => {
                Self::update();
            }
        }
    }

    pub fn sync(package: &String, global: &bool) 
    {
        let mut pkgm_conf = pkgm_config::get_conf(&PackageManager::PACMAN, &config_track::bool_to_track(global));

        let command = ["pacman", "-S", package];    
        let result = exec::status("sudo", command);
    
        if result {
            pkgm_conf.add_package(package);
            git_config::update(&Some(format!("Add package '{}'", package)));
        }
    }

    pub fn remove(package: &String, global: &bool) 
    {
        let mut pkgm_conf = pkgm_config::get_conf(&PackageManager::PACMAN, &config_track::bool_to_track(global));

        let command = ["pacman", "-R", package];
        let result = exec::status("sudo", command);
        
        if result {
            pkgm_conf.remove_package(package);
            git_config::update(&Some(format!("Remove package '{}'", package)));
        }
    }

    pub fn update() {
        let _result = exec::status("sudo", ["pacman", "-Syu"]);
    }
}
