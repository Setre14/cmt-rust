pub struct VecUtil {}

impl VecUtil {
    pub fn to_vec_string(list: &[&str]) -> Vec<String> {
        return list.iter().map(|&s|s.into()).collect();
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use pretty_assertions::assert_eq;
    
    #[test]
    fn test_to_vec_string() {
        let mut expected = Vec::new();
        expected.push("ls".to_string());
        expected.push("-l".to_string());
        expected.push("-a".to_string());

        assert_eq!(VecUtil::to_vec_string(&["ls", "-l", "-a"].to_vec()), expected);
    }
}
