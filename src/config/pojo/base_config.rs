use serde::{de, Serialize};
use dirs::config_dir;
use std::fs;
use std::path::PathBuf;
use confy;

use crate::util::confy_util::ConfyUtil;

pub trait BaseConfig<T=Self> 
where T:  Serialize + de::DeserializeOwned+ Clone + std::fmt::Debug
{
    fn get_config_file_name() -> String;
    fn get_default() -> T;

    fn get_config_dir() -> PathBuf {
        let settings_dir = ConfyUtil::get_configuration_dir("config");
        let _ = fs::create_dir_all(settings_dir.clone());

        settings_dir
    }

    fn get_config_file() -> PathBuf {
        let mut settings_dir = Self::get_config_dir();
        settings_dir.push(Self::get_config_file_name());
    
        settings_dir
    }

    fn get_config() -> T
    where T: Default
    {
        let app_name = env!("CARGO_PKG_NAME");
        let cfg: T = confy::load(app_name, "local").unwrap();

        cfg
    }

    fn save_config(&self) where Self: Serialize {
        let app_name = env!("CARGO_PKG_NAME");
        confy::store(app_name, "local", self).unwrap();
    }

}