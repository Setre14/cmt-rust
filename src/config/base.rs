use dirs::config_dir;
use std::path::PathBuf;
use std::fs;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::BufReader;

use serde::{Serialize, de};

pub trait ConfigReader {
    fn get_conf_name(&self) -> String;
}


pub fn get_conf_dir() -> PathBuf {
    let mut conf_dir = config_dir().expect("Could not get config dir");
    conf_dir.push("cmt-rust");
    let _ = fs::create_dir_all(conf_dir.clone());

    return conf_dir;
}

pub fn get_conf_file(name: String) -> PathBuf {    
    let mut conf_buf = get_conf_dir();
    conf_buf.push(name);

    return conf_buf;
}

#[allow(dead_code)]
pub fn get_conf<T: de::DeserializeOwned + ConfigReader>(default: T) -> T 
{
    let conf_buf = get_conf_file(default.get_conf_name());
    let conf_file = conf_buf.as_path();

    if conf_file.exists() {
    
        let file = File::open(conf_file).expect("Could not read app conf");
        
        let reader = BufReader::new(file);
    
        let config: T = serde_json::from_reader(reader).expect("Could not descerialize app conf");

        return config;

    } else {
        return default;
    }
}

#[allow(dead_code)]
pub fn save_conf<T: Serialize + ConfigReader>(conf: &T) {
    let conf_file = get_conf_file(conf.get_conf_name());

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(conf_file).expect("Could not open conf file");

    serde_json::to_writer_pretty(&file, conf).expect("Failed to write conf");
}