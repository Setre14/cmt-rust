use crate::{util::command_line::CommandLine, pkg::{pkg::Pkg, pkgm::Pkgm}};

use super::cli::dnf_params_install_remove::DnfParamsInstallRemove;


pub struct Dnf {}
impl Dnf {
    pub fn install(params: &DnfParamsInstallRemove) {
        if params.pkg_params.package.is_none() {
            log::error!("Package must be set for install");
            std::process::exit(1);
        }
        let package = params.pkg_params.package.clone().unwrap();

        let mut command = vec!["dnf", "install", &package];
        if params.assumeyes {
            command.push("-y");
        }
        let install_command = CommandLine::create("sudo", command);

        Pkg::install(Pkgm::DNF, &install_command, &params.pkg_params);
    }

    pub fn remove(params: &DnfParamsInstallRemove) {
        if params.pkg_params.package.is_none() {
            log::error!("Package must be set for remove");
            std::process::exit(1);
        }
        let package = params.pkg_params.package.clone().unwrap();

        let mut command = vec!["dnf", "remove", &package];
        if params.assumeyes {
            command.push("-y");
        }

        let remove_command = CommandLine::create("sudo", command);


        Pkg::remove(Pkgm::DNF, &remove_command, &params.pkg_params);
    }

    pub fn update() {
        let update_command = CommandLine::create("sudo", ["dnf", "update"].to_vec());
        Pkg::update(Pkgm::DNF, &update_command);
    }
}