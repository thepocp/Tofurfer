use crate::css::models::StyleRule;
use std::collections::HashMap;

pub struct CascadeContext {
    rules: Vec<StyleRule>,
}

impl CascadeContext {
    pub fn new(rules: Vec<StyleRule>) -> Self {
        CascadeContext { rules }
    }

    pub fn compute_styles(&self, tag: &str) -> HashMap<String, String> {
        let mut computed_styles = HashMap::new();

        for rule in &self.rules {
            if rule.selector == *tag {
                for (key, value) in &rule.properties {
                    computed_styles.insert(key.clone(), value.clone());
                }
            }
        }

        computed_styles
    }
}
