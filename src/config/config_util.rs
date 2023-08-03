use std::process::{Command, Stdio};

pub struct ConfigUtil {}

impl ConfigUtil {
    pub fn get_hostname() -> String {
        let output = Command::new("hostname")
            .stdout(Stdio::piped())
            .output()
            .unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap().replace("\n", "");
    
        return stdout;
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
}

    