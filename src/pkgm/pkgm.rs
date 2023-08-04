use crate::util::exec::Exec;
use crate::util::command_line::CommandLine;
use crate::config::pkgm_config;
use crate::config::package_manager::PackageManager;
use crate::git_config;
use crate::config::config_track::ConfigTrack;

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
        git_config::auto_pull();
        let mut global_pkgm_conf = pkgm_config::get_conf(&Self::get_package_manager(), &ConfigTrack::GLOBAL);
        let mut system_pkgm_conf = pkgm_config::get_conf(&Self::get_package_manager(), &ConfigTrack::SYSTEM);

        let mut added = false;
        if !global {
            if !global_pkgm_conf.packages.contains(package) {
                added = system_pkgm_conf.add_package(package);
            }
        } else {
            added = global_pkgm_conf.add_package(package);
        }

        let result = Exec::status(&Self::install_command(package));
    
        if result {
            git_config::auto_update(&Some(format!("Add package '{}'", package)));
        } else {
            if added {
                if *global {
                    global_pkgm_conf.remove_package(package);
                } else {
                    system_pkgm_conf.remove_package(package);
                }
            }
        }
    }

    fn remove_command(package: &String) -> CommandLine;
    fn remove(package: &String, global: &bool) 
    {
        git_config::auto_pull();
        let mut global_pkgm_conf = pkgm_config::get_conf(&Self::get_package_manager(), &ConfigTrack::GLOBAL);
        let mut system_pkgm_conf = pkgm_config::get_conf(&Self::get_package_manager(), &ConfigTrack::SYSTEM);

        let mut _removed = false;
        if !global {
            if !global_pkgm_conf.packages.contains(package) {
                _removed = Exec::status(&Self::remove_command(package));
            }
        } else {
            _removed = Exec::status(&Self::remove_command(package));
        }
        
        if *global {
            global_pkgm_conf.remove_package(package);
        } else {
            system_pkgm_conf.remove_package(package);
        }
        git_config::auto_update(&Some(format!("Remove package '{}'", package)));
    }

    fn upgrade_command() -> CommandLine;
    fn upgrade() {
        let _result = Exec::status(&Self::upgrade_command());
    }
}
