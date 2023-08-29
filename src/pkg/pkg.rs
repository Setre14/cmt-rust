use crate::{config::{pojo::{base_config, pkg_config::PkgConfig, system_config::SystemConfig}, config::Config}, util::{exec::Exec, command_line::CommandLine}};

use super::{pkgm::Pkgm, cli::pkg_params_install_remove::PkgParamsInstallRemove};



pub struct Pkg {}
impl Pkg {
    pub fn install(pkgm: Pkgm, command_line: &CommandLine, params: &PkgParamsInstallRemove) {
        Config::auto_pull();
        let package = params.package.clone().unwrap();

        let mut pkg_config = PkgConfig::get_pkg_config(&params.pkg_config);
        let mut packages = pkg_config.get_packages(&pkgm);

        packages.insert(package.clone());

        let result = Exec::status(&command_line, None);
    
        if result {
            pkg_config.set_packages(&pkgm, &packages);
            base_config::save_config(&pkg_config);
    
            Config::auto_commit_push(Some(format!("Add {:?} package: '{}'", &pkgm, &package)));
        }
    }

    pub fn remove(pkgm: Pkgm, command_line: &CommandLine, params: &PkgParamsInstallRemove) {
        Config::auto_pull();
        let package = params.package.clone().unwrap();

        let mut pkg_config: PkgConfig = PkgConfig::get_pkg_config(&params.pkg_config);
        let mut packages = pkg_config.get_packages(&pkgm);

        packages.remove(&package);
        pkg_config.set_packages(&pkgm, &packages);
        base_config::save_config(&pkg_config);

        let pkg_configs = SystemConfig::get_system_config().package_config.link_config.configs;

        let mut remove = true;
        for pkg_conf in pkg_configs {
            let config = PkgConfig::get_pkg_config(&Some(pkg_conf.clone()));
            let packages = config.get_packages(&pkgm);

            if packages.contains(&package) {
                log::info!("'{}' is still references in pkg config '{}' - will not be delted from system", &package, &pkg_conf);
                remove = false;
                break;
            }
        }

        if remove {
            let _ = Exec::status(&command_line, None);
        }

        Config::auto_commit_push(Some(format!("Remove {:?} package: '{}'", &pkgm, &package)));
    }

    pub fn update(_pkgm: Pkgm, command_line: &CommandLine) {
        let _ = Exec::status(&command_line, None);
    }
}