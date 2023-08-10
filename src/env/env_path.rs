use std::path::PathBuf;
use std::path::{self, Path};
use std::fs;

#[derive(Debug, PartialEq)]
pub struct EnvPath {
    path: String
}

impl EnvPath {
    pub fn from_local(path: &str) -> EnvPath {
        let mut env_path = fs::canonicalize(path).unwrap().into_os_string().into_string().unwrap();

        let user_home = dirs::home_dir().unwrap().into_os_string().into_string().unwrap();

        env_path = env_path.replace(&user_home, "user-home");

        if env_path.starts_with("/") {
            env_path = (&env_path[1..]).to_string();
        }

        EnvPath {
            path: env_path.to_string()
        }
    }

    pub fn get_env_path(&self) -> PathBuf {
        

        PathBuf::from(&self.path)
    }
}
