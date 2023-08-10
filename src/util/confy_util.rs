use std::path::PathBuf;

pub struct ConfyUtil {}

impl ConfyUtil {
    pub fn get_app_name() -> String {
        env!("CARGO_PKG_NAME").to_string()
    }

    pub fn get_configuration_dir(config_name: &str) -> PathBuf {
        let mut path = confy::get_configuration_file_path(&Self::get_app_name(), config_name).unwrap();
        path.pop();

        path
    }
}
