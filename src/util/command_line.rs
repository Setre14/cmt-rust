use crate::util::vec_util::VecUtil;

#[derive(Debug, Default, Clone)]
pub struct CommandLine {
    pub command: String,
    pub args: Vec<String>
}

impl CommandLine {
    pub fn create(command: &str, args: Vec<&str>) -> CommandLine {
        return CommandLine {
            command: String::from(command), 
            args: VecUtil::to_vec_string(&args)
        }
    }
}