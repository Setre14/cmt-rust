use crate::util::exec;
use crate::config::pkgm;

use clap::{Subcommand};

#[derive(Subcommand)]
pub enum Command {
    #[command(name = "-S")]
    Sync {
        package: String,
    },

    #[command(name = "-R")]
    Remove {
        package: String,
    },

    #[command(name = "-Syu")]
    Update {}

}

pub struct Pacman {
    
}

impl Pacman {
    pub fn handle_command(command: &Command) {
        match command {
            Command::Sync {package} => {
                Self::sync(package);
            },
            Command::Remove {package} => {
                Self::remove(package);
            },
            Command::Update {} => {
                Self::update();
            }
        }
    }

    pub fn sync(package: &String) 
    {
        let mut pkgm_conf = pkgm::get_conf(pkgm::Pkgm::PACMAN);

        let command = ["pacman", "-S", package];    
        let result = exec::status("sudo", command);
    
        if result {
            pkgm_conf.add_package(package)
        }
    }

    pub fn remove(package: &String) 
    {
        let mut pkgm_conf = pkgm::get_conf(pkgm::Pkgm::PACMAN);

        let command = ["pacman", "-R", package];
        let result = exec::status("sudo", command);
        
        if result {
            pkgm_conf.remove_package(package)
        }
    }

    pub fn update() {
        let _result = exec::status("sudo", ["pacman", "-Syu"]);
    }
}
