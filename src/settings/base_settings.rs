use serde::{de, Serialize};
use dirs::config_dir;
use std::fs;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::io::BufReader;

pub trait BaseSettings<T=Self> 
where T:  Serialize + de::DeserializeOwned+ Clone + std::fmt::Debug
{
    fn get_settings_file_name() -> String;
    fn get_default() -> T;

    fn get_settings_dir() -> PathBuf {
        let mut settings_dir = config_dir().expect("Could not get config dir");
        settings_dir.push("cmt-rust");
        let _ = fs::create_dir_all(settings_dir.clone());

        return settings_dir;
    }

    fn get_settings_file() -> PathBuf {
        let mut settings_dir = Self::get_settings_dir();
        settings_dir.push(Self::get_settings_file_name());
    
        return settings_dir;
    }

    fn get_settings() -> T
    {
        let conf_buf = Self::get_settings_file();
        let settings_file = conf_buf.as_path();

        let mut config = Self::get_default();

        if settings_file.exists() {
            log::debug!("Load settings_file: {:?}", settings_file.clone());

            let file = File::open(settings_file).expect("Could not read app conf");
            let reader = BufReader::new(file);
            config = serde_json::from_reader(reader).expect("Could not descerialize app conf");
        } else {
            log::debug!("settings_file {:?}, does not exits. Use default", settings_file.clone());
        }
        
        log::debug!("Config settings: {:#?}", config.clone());

        return config;
    }

    fn save_settings(&self) where Self: Serialize {
        let settings_file = Self::get_settings_file();

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(settings_file.clone()).expect("Could not open conf file");
    
        log::info!("Save settings_file: {}", settings_file.into_os_string().into_string().unwrap());
    
        serde_json::to_writer_pretty(&file, self).expect("Failed to write settings");
    }

}