use crate::config::package_manager::PackageManager;
use crate::pkgm::pkgm::Pkgm;
use crate::util::command_line::CommandLine;

pub struct Pacman {
    
}

impl Pkgm for Pacman {
    fn get_package_manager() -> PackageManager {
        return PackageManager::PACMAN;
    }

    fn install_command(package: &String) -> CommandLine {
        return CommandLine::create("sudo", ["pacman", "-S", package].to_vec())
    }
    fn remove_command(package: &String) -> CommandLine {
        return CommandLine::create("sudo", ["pacman", "-R", package].to_vec())
    }
    fn upgrade_command() -> CommandLine {
        return CommandLine::create("sudo", ["pacman", "-Syu"].to_vec())
    }
}
