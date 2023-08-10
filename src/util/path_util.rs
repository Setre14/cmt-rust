use std::path::PathBuf;

pub struct PathUtil {}

impl PathUtil {
    pub fn to_string(path: &PathBuf) -> String {
        return path.clone().into_os_string().into_string().unwrap();
    }
}