use crate::util::vec_util::VecUtil;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CommandLine {
    pub command: String,
    pub args: Vec<String>
}

impl CommandLine {
    pub fn create(command: &str, args: Vec<&str>) -> CommandLine {
        CommandLine {
            command: String::from(command), 
            args: VecUtil::to_vec_string(&args)
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use pretty_assertions::assert_eq;
    
    use crate::util::vec_util::VecUtil;

    #[test]
    fn test_create() {
        let expected = CommandLine {
            command: String::from("ls"),
            args: VecUtil::to_vec_string(["-l", "-a"].as_ref())
        };

        let actual = CommandLine::create("ls", ["-l", "-a"].to_vec());

        assert_eq!(actual, expected);
    }
}