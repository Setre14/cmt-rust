use crate::{util::command_line::CommandLine, pkg::{pkgm::Pkgm, pkg::Pkg, cli::pkg_params_install_remove::PkgParamsInstallRemove}};

use super::cli::yay_params::{YayParamsSync, YayParamsShared};

pub struct Yay {}
impl Yay {
    pub fn sync(pkg_params: &PkgParamsInstallRemove, shared_params: &YayParamsShared, sync_params: &YayParamsSync) {
        let mut command = vec!["-S"];
        if shared_params.noconfirm {
            command.push("--noconfirm");
        }        
        if sync_params.sysupgrade {
            command.push("-u");
        }
        if sync_params.refresh {
            command.push("-y");
        }
        let package = pkg_params.package.clone().unwrap_or("".to_string());
        if !package.is_empty() {
            command.push(&package);

        }

        let command = CommandLine::create("yay", command);

        if !package.is_empty() {
            Pkg::install(Pkgm::YAY, &command, &pkg_params);
        } else if sync_params.sysupgrade {
            Pkg::update(Pkgm::YAY, &command)
        } else {
            log::error!("Package must be set for sync");
            std::process::exit(1);
        }

    }

    pub fn remove(pkg_params: &PkgParamsInstallRemove, shared_params: &YayParamsShared) {
        if pkg_params.package.is_none() {
            log::error!("Package must be set for remove");
            std::process::exit(1);
        }

        let package = pkg_params.package.clone().unwrap();

        let mut command = vec!["-R", &package];
        if shared_params.noconfirm {
            command.push("--noconfirm");
        }

        let remove_command = CommandLine::create("yay", command);
        Pkg::remove(Pkgm::YAY, &remove_command, pkg_params);
    }
}