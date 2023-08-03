pub struct VecUtil {}

impl VecUtil {
    pub fn to_vec_string(list: &Vec<&str>) -> Vec<String> {
        return list.iter().map(|&s|s.into()).collect();
    }
}