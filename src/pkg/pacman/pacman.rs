use crate::{util::command_line::CommandLine, pkg::{pkgm::Pkgm, pkg::Pkg, cli::pkg_params_install_remove::PkgParamsInstallRemove}};

use super::cli::pacman_params::{PacmanParamsSync, PacmanParamsShared};

pub struct Pacman {}
impl Pacman {
    pub fn sync(pkg_params: &PkgParamsInstallRemove, shared_params: &PacmanParamsShared, sync_params: &PacmanParamsSync) {
        let mut command = vec!["pacman", "-S"];
        if shared_params.noconfirm {
            command.push("--noconfirm");
        }        
        if sync_params.sysupgrade {
            command.push("-u");
        }
        if sync_params.refresh {
            command.push("-y");
        }
        let package = pkg_params.package.to_string();
        if !package.is_empty() {
            command.push(&package);

        }

        let command = CommandLine::create("sudo", command);

        if !package.is_empty() {
            Pkg::install(Pkgm::PACMAN, &command, &pkg_params);
        } else if sync_params.sysupgrade {
            Pkg::update(Pkgm::PACMAN, &command)
        } else {
            log::error!("Package must be set for sync");
            std::process::exit(1);
        }

    }

    pub fn remove(pkg_params: &PkgParamsInstallRemove, shared_params: &PacmanParamsShared) {
        let package = pkg_params.package.to_string();

        let mut command = vec!["pacman", "-R", &package];
        if shared_params.noconfirm {
            command.push("--noconfirm");
        }

        let remove_command = CommandLine::create("sudo", command);
        Pkg::remove(Pkgm::PACMAN, &remove_command, pkg_params);
    }
}