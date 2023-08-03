use std::path::PathBuf;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::BufReader;
use strum::IntoEnumIterator;
use std::string::ToString;

use std::process::{Command, Stdio};

use serde::{Serialize, de};

use crate::config::config_track::ConfigTrack;


pub trait StringAccessable {
    fn get_string(&self, field_string: &str) -> Result<&String, String>;
    fn get_vec(&self, field_string: &str) -> Result<&Vec<String>, String>;
    fn get_u8(&self, field_string: &str) -> Result<&u8, String>;
}

pub trait ConfigReader {
    // fn get_conf_name(&self) -> String;
    fn get_conf_dir(&self) -> PathBuf;
    
    fn get_conf_file(&self, track: &ConfigTrack) -> PathBuf {   
        let mut conf_buf = self.get_conf_dir();
        conf_buf.push(self.get_conf_dir());

        let filename = match track {
            ConfigTrack::GLOBAL => "global".to_string(),
            ConfigTrack::SYSTEM => get_hostname(),
        };

        conf_buf.push(format!("{}.json", filename));
    
        return conf_buf;
    }

    fn get_track(&self) -> ConfigTrack;
    fn set_track(&mut self, track: &ConfigTrack);

    fn merge<T: StringAccessable + Clone + std::fmt::Debug>(&mut self, other: T);
}

pub fn get_conf<T>(track: &ConfigTrack, default: T) -> T
where 
    T: Serialize + de::DeserializeOwned + ConfigReader,
{
    let conf_buf = default.get_conf_file(track);
    let conf_file = conf_buf.as_path();

    let mut config: T;

    if conf_file.exists() {
        log::debug!("Load conf_file: {:?}", conf_file.clone());

        let file = File::open(conf_file).expect("Could not read app conf");
        let reader = BufReader::new(file);
        config = serde_json::from_reader(reader).expect("Could not descerialize app conf");
    } else {
        log::debug!("conf_file {:?}, does not exits. Use default", conf_file.clone());
        config = default;
    }
    config.set_track(track);
    
    return config;
}

pub fn get_combined_conf<T>(default: T) -> T 
where 
    T: Serialize + de::DeserializeOwned + ConfigReader + std::fmt::Debug + Clone + StringAccessable,
{
    let mut combined_conf = default.clone();

    for track in ConfigTrack::iter() {
        let conf = get_conf(&track, default.clone());
        combined_conf.merge(conf.clone());
    }

    log::debug!("combined_conf: {:?}", combined_conf.clone());
    
    return combined_conf;
}

pub fn save_conf<T: Serialize + ConfigReader>(conf: &T) {
    let conf_file = conf.get_conf_file(&conf.get_track());

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(conf_file.clone()).expect("Could not open conf file");

    log::info!("Save conf file: {}", conf_file.into_os_string().into_string().unwrap());

    serde_json::to_writer_pretty(&file, conf).expect("Failed to write conf");
}

pub fn add_to_list(list: &mut Vec<String>, item: &String) {
    if !list.contains(item) {
        list.push(item.clone().to_string());
    }
    list.sort();
}

pub fn remove_from_list(list: &mut Vec<String>, item: &String) {
    if list.contains(item) {
        let index = list.iter().position(|x| *x == item.to_string()).unwrap();
        list.remove(index);
    }
    list.sort();
}

fn get_hostname() -> String {
    let output = Command::new("hostname")
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap().replace("\n", "");

    return stdout;
}
