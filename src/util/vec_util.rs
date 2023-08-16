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
        let expected = vec!["ls".to_string(), "-l".to_string(), "-a".to_string()];

        assert_eq!(VecUtil::to_vec_string(["ls", "-l", "-a"].as_ref()), expected);
    }
}
