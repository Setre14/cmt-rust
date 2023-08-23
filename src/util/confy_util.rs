use std::path::PathBuf;

pub struct ConfyUtil {}

impl ConfyUtil {
    pub fn get_app_name() -> String {
        env!("CARGO_PKG_NAME").to_string()
    }

    pub fn get_root_configuration_dir() -> PathBuf {
        Self::get_configuration_dir("test")
    }

    pub fn get_git_config_dir() -> String {
        "git-config".to_string()
    }

    pub fn get_git_configuration_dir() -> PathBuf {
        let mut path = Self::get_root_configuration_dir();
        path.push(Self::get_git_config_dir().as_str());

        path
    }

    // pub fn get_git_config_file_path(file_name: &str) -> String {
    //     let mut path = PathBuf::from(Self::get_git_config_dir());
    //     path.push(file_name);

    //     PathUtil::to_string(&path)
    // }

    pub fn get_configuration_dir(config_name: &str) -> PathBuf {
        let mut path = confy::get_configuration_file_path(&Self::get_app_name(), config_name).unwrap();
        path.pop();

        path
    }

    pub fn get_config_file_ending() -> String {
        ".toml".to_string()
    }
}
