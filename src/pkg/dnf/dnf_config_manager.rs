use crate::{util::{command_line::CommandLine, exec::Exec}, pkg::pkg::Pkg, config::{pojo::{pkg_config::PkgConfig, base_config}, config::Config}};

use super::cli::dnf_params_config_manager::DnfParamsConfigManager;

pub struct DnfConfigManager {}
impl DnfConfigManager {
    pub fn handle_command(params: &DnfParamsConfigManager, config: &Option<String>) {
        match &params.add_repo {
            Some(repo) => Self::add_repo(&repo, config),
            None => { return }
        }
    }

    fn add_repo(repo: &str, config: &Option<String>) {
        let command = vec!["dnf", "config-manager", "--add-repo", repo];
        let command = CommandLine::create("sudo", command);

        let pkg_config = PkgConfig::get_pkg_config(config);
        let mut dnf_config = pkg_config.get_dnf_config();

        let mut repos = dnf_config.repos;

        repos.insert(repo.to_string());

        let result = Exec::status(&command, None);
    
        if result {
            dnf_config.repos = repos;
            base_config::save_config(&dnf_config);
    
            Config::auto_commit_push(Some(format!("Add dnf repo: '{}'", repo)));
        }
    }

}
