use std::{path::PathBuf, collections::BTreeSet, fs};

use serde::{de, Serialize};
use confy;

use crate::{util::{confy_util::ConfyUtil, path_util::PathUtil}, config::pojo::local_config::LocalConfig};

pub trait BaseConfig<T=Self> 
where T:  Serialize + de::DeserializeOwned + Clone + std::fmt::Debug
{
    fn get_default_config_file_name() -> String;
    fn get_config_file_name(&self) -> String;
    fn set_config_file_name(&mut self, file_name: &str);

    fn get_dir() -> PathBuf;

    fn get_config(file_name: Option<String>) -> T
    where T: Default + std::fmt::Debug + BaseConfig
    {
        let app_name = ConfyUtil::get_app_name();
        let default_file_name = Self::get_default_config_file_name();
        let mut config_name = file_name.unwrap_or(default_file_name);

        if config_name != LocalConfig::get_default_config_file_name() {
            let mut path = PathBuf::from(ConfyUtil::get_git_config_dir());
            path.push(config_name);

            config_name = PathUtil::to_string(&path);
        }

        log::debug!("Load config: {:?}", &config_name);
        let mut cfg: T = confy::load(&app_name, config_name.as_str()).unwrap();
        cfg.set_config_file_name(config_name.as_str());
        log::debug!("Loaded config: {:?}", &cfg);


        cfg
    }

    fn get_configs() -> BTreeSet<String> {
        let mut configs: BTreeSet<String> = BTreeSet::new();

        let paths = fs::read_dir(Self::get_dir()).unwrap();

        for p in paths {
            let  path = p.unwrap().path();
            if !path.is_file() {
                continue;
            }

            let file = path.file_name().unwrap().to_str().unwrap();

            if !file.ends_with(&ConfyUtil::get_config_file_ending()) {
                continue;
            }

            let file_name = file.replace(&ConfyUtil::get_config_file_ending(), "");

            configs.insert(file_name);
        }

        configs
    }
}

pub fn save_config<T>(cfg: &T) where T: BaseConfig + Serialize + de::DeserializeOwned + Clone + std::fmt::Debug {
    let app_name = ConfyUtil::get_app_name();
    let config_name = cfg.get_config_file_name();
    log::debug!("Save config: {:?}, {:?}", config_name.clone(), cfg.clone());
    
    confy::store(&app_name, config_name.as_str(), cfg).unwrap();
}