pub trait StringAccessable {
    fn get_string(&self, field_string: &str) -> Result<&String, String>;
    fn get_vec(&self, field_string: &str) -> Result<&Vec<String>, String>;
    fn get_u8(&self, field_string: &str) -> Result<&u8, String>;
}