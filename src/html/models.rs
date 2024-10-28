use std::collections::HashMap;

#[derive(Debug)]
pub enum Node {
    Element {
        tag: String,
        children: Vec<Node>,
        attributes: HashMap<String, String>,
    },
    Text(String),
}
