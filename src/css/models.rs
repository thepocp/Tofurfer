use std::collections::HashMap;

#[derive(Debug)]
pub struct StyleRule {
    pub selector: String,
    pub properties: HashMap<String, String>,
}
