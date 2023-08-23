use std::path::Path;

pub struct PathUtil {}

impl PathUtil {
    pub fn to_string(path: &Path) -> String {
        path.to_str().unwrap().to_string()
    }
}