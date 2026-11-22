use std::collections::HashMap;
pub type BoW = HashMap<String, u32>;

pub fn tokenize(s: &str) -> String {
    s.chars()
        .filter(|x| x.is_alphanumeric())
        .collect::<String>()
        .to_uppercase()
}
