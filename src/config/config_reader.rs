use std::path::PathBuf;
use serde::{Serialize, de};
use strum::IntoEnumIterator;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::BufReader;
use std::string::ToString;

use crate::config::config_track::ConfigTrack;
use crate::config::config_util::ConfigUtil;
use crate::config::string_accessable::StringAccessable;

pub trait ConfigReader {
    fn get_conf_dir(&self) -> PathBuf;
    
    fn get_conf_file(&self) -> PathBuf {   
        let mut conf_buf = self.get_conf_dir();
        conf_buf.push(self.get_conf_dir());

        let filename = match self.get_track() {
            ConfigTrack::GLOBAL => "global".to_string(),
            ConfigTrack::SYSTEM => ConfigUtil::get_hostname(),
        };

        conf_buf.push(format!("{}.json", filename));
    
        return conf_buf;
    }

    fn get_track(&self) -> ConfigTrack;
    fn set_track(&mut self, track: &ConfigTrack);

    fn merge<T: StringAccessable + Clone + std::fmt::Debug>(&mut self, other: T);
}

pub fn get_conf<T>(track: &ConfigTrack, default: &mut T) -> T
where 
    T: Serialize + de::DeserializeOwned + ConfigReader + Clone,
{
    default.set_track(track);
    let conf_buf = default.get_conf_file();
    let conf_file = conf_buf.as_path();

    let mut config = default.clone();

    if conf_file.exists() {
        log::debug!("Load conf_file: {:?}", conf_file.clone());

        let file = File::open(conf_file).expect("Could not read app conf");
        let reader = BufReader::new(file);
        config = serde_json::from_reader(reader).expect("Could not descerialize app conf");
    } else {
        log::debug!("conf_file {:?}, does not exits. Use default", conf_file.clone());
        // config = default;
    }
    config.set_track(track);
    
    return config;
}

pub fn get_combined_conf<T>(default: &mut T) -> T 
where 
    T: Serialize + de::DeserializeOwned + ConfigReader + std::fmt::Debug + Clone + StringAccessable,
{
    let mut combined_conf = default.clone();

    for track in ConfigTrack::iter() {
        combined_conf.merge(get_conf(&track, default));
    }

    log::debug!("combined_conf: {:?}", combined_conf.clone());
    
    return combined_conf;
}

pub fn save_conf<T: Serialize + ConfigReader>(conf: &T) {
    let conf_file = conf.get_conf_file();

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(conf_file.clone()).expect("Could not open conf file");

    log::info!("Save conf file: {}", conf_file.into_os_string().into_string().unwrap());

    serde_json::to_writer_pretty(&file, conf).expect("Failed to write conf");
}