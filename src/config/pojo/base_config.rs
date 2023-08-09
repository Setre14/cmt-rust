use serde::{de, Serialize};
use dirs::config_dir;
use std::fs;
use std::path::PathBuf;
use confy;

pub trait BaseConfig<T=Self> 
where T:  Serialize + de::DeserializeOwned+ Clone + std::fmt::Debug
{
    fn get_config_file_name() -> String;
    fn get_default() -> T;

    fn get_config_dir() -> PathBuf {
        let mut settings_dir = config_dir().expect("Could not get config dir");
        settings_dir.push("cmt-rust");
        let _ = fs::create_dir_all(settings_dir.clone());

        return settings_dir;
    }

    fn get_config_file() -> PathBuf {
        let mut settings_dir = Self::get_config_dir();
        settings_dir.push(Self::get_config_file_name());
    
        return settings_dir;
    }

    fn get_config() -> T
    where T: Default
    {
        let app_name = env!("CARGO_PKG_NAME");
        let cfg: T = confy::load(app_name, "local").unwrap();

        return cfg;
    }

    fn save_config(&self) where Self: Serialize {
        let app_name = env!("CARGO_PKG_NAME");
        let _ = confy::store(app_name, "local", self).unwrap();
    }

}