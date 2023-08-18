use crate::{config::{pojo::{base_config, pkg_config::PkgConfig, system_config::SystemConfig}, config::Config}, util::{exec::Exec, command_line::CommandLine}};

use super::cli::dnf_params_install_remove::DnfParamsInstallRemove;


pub struct Dnf {}
impl Dnf {
    pub fn install(params: &DnfParamsInstallRemove) {
        Config::auto_pull();
        let mut pkg_config = PkgConfig::get_pkg_config(&params.pkg_config);
        let mut dnf_config = pkg_config.dnf_config;

        dnf_config.packages.insert(params.package.clone());

        let mut command = vec!["dnf", "install", &params.package];
        if params.assumeyes {
            command.push("-y");
        }

        let install_command = CommandLine::create("sudo", command);
        let _ = Exec::status(&install_command, None);
    
        pkg_config.dnf_config = dnf_config.clone();
        base_config::save_config(&pkg_config);

        Config::auto_commit_push(Some(format!("Add dnf package: '{}'", &params.package)));
    }

    pub fn remove(params: &DnfParamsInstallRemove) {
        Config::auto_pull();
        let mut pkg_config: PkgConfig = PkgConfig::get_pkg_config(&params.pkg_config);
        let mut dnf_config = pkg_config.dnf_config;

        dnf_config.packages.remove(&params.package);
        pkg_config.dnf_config = dnf_config.clone();
        base_config::save_config(&pkg_config);

        let pkg_configs = SystemConfig::get_system_config().package_config.configs;

        let mut remove = true;
        for pkg_conf in pkg_configs {
            let config = PkgConfig::get_pkg_config(&Some(pkg_conf.clone()));
            let packages = config.dnf_config.packages;

            if packages.contains(&params.package) {
                log::info!("'{}' is still references in pkg config '{}' - will not be delted from system", &params.package, &pkg_conf);
                remove = false;
                break;
            }
        }

        if remove {
            let mut command = vec!["dnf", "remove", &params.package];
            if params.assumeyes {
                command.push("-y");
            }

            
            let remove_command = CommandLine::create("sudo", command);
            let _ = Exec::status(&remove_command, None);
        }


        Config::auto_commit_push(Some(format!("Remove dnf package: '{}'", &params.package)));
    }

    pub fn update() {
        let remove_command = CommandLine::create("sudo", ["dnf", "update"].to_vec());
        let _ = Exec::status(&remove_command, None);
    }
}