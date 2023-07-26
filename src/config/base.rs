use std::path::PathBuf;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::BufReader;

use serde::{Serialize, de};

pub trait ConfigReader {
    fn get_conf_name(&self) -> String;
    fn get_conf_dir(&self) -> PathBuf;
    
    fn get_conf_file(&self) -> PathBuf {   
        let mut conf_buf = self.get_conf_dir();
        conf_buf.push(self.get_conf_name());
    
        return conf_buf;
    }
}

pub fn get_conf<T: de::DeserializeOwned + ConfigReader>(default: T) -> T 
{
    let conf_buf = default.get_conf_file();
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

pub fn save_conf<T: Serialize + ConfigReader>(conf: &T) {
    let conf_file = conf.get_conf_file();

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(conf_file.clone()).expect("Could not open conf file");

    log::info!("{}", conf_file.into_os_string().into_string().unwrap());

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