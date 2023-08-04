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

    pub fn add_to_list(list: &mut Vec<String>, item: &String) -> bool {
        let contains = list.contains(item);
        if !contains {
            list.push(item.clone().to_string());
        }
        list.sort();

        return contains;
    }
    
    pub fn remove_from_list(list: &mut Vec<String>, item: &String) -> bool {
        let contains = list.contains(item);
        if contains {
            let index = list.iter().position(|x| *x == item.to_string()).unwrap();
            list.remove(index);
        }
        list.sort();

        return contains;
    }

    pub fn merge_vec(a: &mut Vec<String>, b: &Vec<String>) {
        for item in b {
            if !a.contains(&item) {
                a.push(item.clone().to_string());
            }
        } 
    }
}

    