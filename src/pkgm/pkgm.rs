use crate::util::exec;
use crate::util::command_line::CommandLine;
use crate::config::pkgm_config;
use crate::config::package_manager::PackageManager;
use crate::git_config;
use crate::config::config_track;

use crate::pkgm::pkgm_command::PkgmCommand;

pub trait Pkgm {
    fn handle_command(command: &PkgmCommand) {
        match command {
            PkgmCommand::Install {package, global} => {
                Self::install(package, global);
            },
            PkgmCommand::Remove {package, global} => {
                Self::remove(package, global);
            },
            PkgmCommand::Upgrade {} => {
                Self::upgrade();
            }
        }
    }

    fn get_package_manager() -> PackageManager;

    fn install_command(package: &String) -> CommandLine;
    fn install(package: &String, global: &bool) 
    {
        let mut pkgm_conf = pkgm_config::get_conf(&Self::get_package_manager(), &config_track::bool_to_track(global));

        log::info!("pkgm_conf: {:?}", pkgm_conf.clone());

        let result = exec::status(&Self::install_command(package));
    
        if result {
            pkgm_conf.add_package(package);
            git_config::update(&Some(format!("Add package '{}'", package)));
        }
    }

    fn remove_command(package: &String) -> CommandLine;
    fn remove(package: &String, global: &bool) 
    {
        let mut pkgm_conf = pkgm_config::get_conf(&Self::get_package_manager(), &config_track::bool_to_track(global));

        let result = exec::status(&Self::remove_command(package));
        
        if result {
            pkgm_conf.remove_package(package);
            git_config::update(&Some(format!("Remove package '{}'", package)));
        }
    }

    fn upgrade_command() -> CommandLine;
    fn upgrade() {
        let _result = exec::status(&Self::upgrade_command());
    }
}