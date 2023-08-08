use serde::{de, Serialize};
use dirs::config_dir;
use std::fs;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::io::BufReader;

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
    {
        let conf_buf = Self::get_config_file();
        let config_file = conf_buf.as_path();

        let mut config = Self::get_default();

        if config_file.exists() {
            log::debug!("Load config_file: {:?}", config_file.clone());

            let file = File::open(config_file).expect("Could not read app conf");
            let reader = BufReader::new(file);
            config = serde_json::from_reader(reader).expect("Could not descerialize app conf");
        } else {
            log::debug!("config_file {:?}, does not exits. Use default", config_file.clone());
        }
        
        log::debug!("Config settings: {:#?}", config.clone());

        return config;
    }

    fn save_config(&self) where Self: Serialize {
        let config_file = Self::get_config_file();

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(config_file.clone()).expect("Could not open conf file");
    
        log::info!("Save config_file: {}", config_file.into_os_string().into_string().unwrap());
    
        serde_json::to_writer_pretty(&file, self).expect("Failed to write settings");
    }

}