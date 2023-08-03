use crate::config::package_manager::PackageManager;
use crate::pkgm::pkgm::Pkgm;
use crate::util::command_line::CommandLine;

pub struct Dnf {
    
}

impl Pkgm for Dnf {
    fn get_package_manager() -> PackageManager {
        return PackageManager::DNF;
    }

    fn install_command(package: &String) -> CommandLine {
        return CommandLine::create("sudo", ["dnf", "install", package].to_vec())
    }
    fn remove_command(package: &String) -> CommandLine {
        return CommandLine::create("sudo", ["dnf", "remove", package].to_vec())
    }
    fn upgrade_command() -> CommandLine {
        return CommandLine::create("sudo", ["dnf", "upgrade"].to_vec())
    }
}
